//! The original programs' icons, and optionally names, on the installed Craft App bundles, so
//! Finder, Launchpad and the Dock show them (macOS only).
//!
//! Names: the bundle keeps its file name (the launcher finds and updates apps by it). macOS shows
//! a localized display name instead when `LSHasLocalizedDisplayName` is set and the bundle has
//! `InfoPlist.strings` with `CFBundleDisplayName`. Adding them changes sealed files, so the bundle
//! is then signed again (ad hoc, keeping its entitlements); the menu bar keeps the executable's
//! name. A custom icon is extra data `codesign` refuses, so it is cleared before signing and put
//! back after.
//!
//! macOS keeps a custom icon in an `Icon\r` file at the bundle's root. The bundles are kept in step
//! with the switch both ways: on, a bundle without the icon gets it (an update replaces the whole
//! `.app`, dropping it); off, a bundle with it is cleared back to the Craft icon. The check runs at
//! once when the switch is clicked and every few seconds after.

use crate::LauncherUi;

/// What one installed app carries on this machine.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct AppState {
    /// The original program's icon where the shell shows it.
    pub icons: bool,
    /// The original program's name.
    pub names: bool,
}

/// What the last pass found, for the switch's label and tooltip.
#[derive(Clone, Debug, Default)]
pub(crate) struct Status {
    pub busy: bool,
    /// Installed apps that carry the original icon.
    pub icons: usize,
    /// Installed apps the pass looked at.
    pub total: usize,
    /// Each installed app's id and what it carries, for the badge on its row.
    pub apps: Vec<(String, AppState)>,
    /// Apps whose icon or name the last pass actually changed (0 on a pass that found everything
    /// in place, so the UI can tell a real change from a routine re-check).
    pub changed: usize,
    pub error: Option<String>,
}

#[cfg(target_os = "macos")]
pub(super) fn sync(launcher: &LauncherUi, toggled: bool) {
    imp::sync(launcher, toggled);
}

#[cfg(windows)]
pub(super) fn sync(launcher: &LauncherUi, toggled: bool) {
    super::windows_icons::sync(launcher, toggled);
}

#[cfg(target_os = "linux")]
pub(super) fn sync(launcher: &LauncherUi, toggled: bool) {
    super::linux_icons::sync(launcher, toggled);
}

#[cfg(not(any(target_os = "macos", windows, target_os = "linux")))]
pub(super) fn sync(_launcher: &LauncherUi, _toggled: bool) {}

#[cfg(target_os = "macos")]
pub(super) fn status() -> Status {
    imp::status()
}

#[cfg(windows)]
pub(super) fn status() -> Status {
    super::windows_icons::status()
}

#[cfg(target_os = "linux")]
pub(super) fn status() -> Status {
    super::linux_icons::status()
}

/// What the installed app `id` carries, once a pass has looked at it.
pub(crate) fn app_state(id: &str) -> Option<AppState> {
    status().apps.into_iter().find(|(app, _)| app == id).map(|(_, state)| state)
}

#[cfg(not(any(target_os = "macos", windows, target_os = "linux")))]
pub(super) fn status() -> Status {
    Status::default()
}

#[cfg(target_os = "macos")]
mod imp {
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::{Duration, Instant};

    use super::{AppState, Status};
    use crate::LauncherUi;
    use crate::bettercraft::MacMode;
    use crate::bettercraft::assets::IconSet;

    const RECHECK: Duration = Duration::from_secs(5);

    static BUSY: AtomicBool = AtomicBool::new(false);
    static LAST: Mutex<Option<Instant>> = Mutex::new(None);
    static STATUS: Mutex<Option<Status>> = Mutex::new(None);

    /// Sets (with an `.icns` path) or clears (with an empty one) a file's custom icon, then tells
    /// Finder the file changed so it redraws.
    const SET_ICON: &str = "ObjC.import('AppKit');\
        function run(argv) {\
          const ws = $.NSWorkspace.sharedWorkspace;\
          const image = argv[0] ? $.NSImage.alloc.initWithContentsOfFile(argv[0]) : undefined;\
          const ok = ws.setIconForFileOptions(image, argv[1], 0);\
          ws.noteFileSystemChanged(argv[1]);\
          return ok ? 'ok' : 'failed';\
        }";

    pub(super) fn status() -> Status {
        let mut status = STATUS.lock().ok().and_then(|s| s.clone()).unwrap_or_default();
        status.busy = BUSY.load(Ordering::Acquire);
        status
    }

    /// Publishes the pass's running status so the toolbar can show live progress.
    fn publish(busy: bool, apps: &[(String, AppState)], total: usize, changed: usize, error: Option<String>) {
        let icons = apps.iter().filter(|(_, s)| s.icons).count();
        if let Ok(mut s) = STATUS.lock() {
            *s = Some(Status { busy, icons, total, changed, error, apps: apps.to_vec() });
        }
    }

    pub(super) fn sync(launcher: &LauncherUi, toggled: bool) {
        if !toggled && !due() {
            return;
        }
        let mode = super::super::mac_mode();
        let style = super::super::icon_set();
        let set = style;
        if crate::bettercraft::showcase() {
            // Screenshots: show the badges the choice would give, change nothing on disk.
            let state = AppState { icons: mode.icons(), names: mode.icons() && style.names() };
            let apps: Vec<(String, AppState)> =
                launcher.entries.iter().filter(|e| e.installed.is_some()).map(|e| (e.app.id.to_string(), state)).collect();
            let icons = if state.icons { apps.len() } else { 0 };
            if let Ok(mut s) = STATUS.lock() {
                *s = Some(Status { busy: false, icons, total: apps.len(), changed: 0, error: None, apps });
            }
            return;
        }
        let apps: Vec<(String, PathBuf)> = launcher
            .entries
            .iter()
            .filter_map(|e| {
                // Never touch an app while the launcher downloads, installs or removes it: the
                // update replaces the bundle, and the next pass dresses the new one.
                if e.job.is_some() {
                    return None;
                }
                let location = &e.installed.as_ref()?.location;
                location.ends_with(".app").then(|| (e.app.id.to_string(), PathBuf::from(location)))
            })
            .collect();
        if BUSY.swap(true, Ordering::AcqRel) {
            return;
        }
        // Show the pass starting at once, before the thread's first publish.
        publish(true, &[], apps.len(), 0, None);
        std::thread::spawn(move || {
            let mut error = None;
            let mut changed = 0usize;
            let mut states: Vec<(String, AppState)> = Vec::new();
            for (id, bundle) in &apps {
                match apply(id, bundle, mode, style) {
                    Ok(did) => changed += usize::from(did),
                    Err(e) => {
                        log::warn!("BetterCraft: couldn't change the icon of {}: {e}", bundle.display());
                        error = Some(format!("{}: {e}", bundle.display()));
                    }
                }
                let icons = has_custom_icon(bundle) && crate::bettercraft::dock_icon::is_patched(bundle);
                states.push((id.clone(), AppState { icons, names: has_original_name(bundle) }));
                publish(true, &states, apps.len(), changed, error.clone());
            }
            // The launcher's own bundle: the old logo as its Finder icon in Study, none in Normal.
            if let Err(e) = dress_launcher(set == IconSet::Study) {
                log::warn!("BetterCraft: couldn't change the launcher's icon: {e}");
            }
            // Clear the icon caches and restart the Dock (which also draws Launchpad) so Finder,
            // Launchpad and the Dock re-read the icons now instead of keeping the old pictures.
            // A user-triggered pass (switch change, install, "Check again") always refreshes; a
            // routine re-check only when it changed something. A Craft App that is open keeps its
            // old Dock icon until it is opened again.
            if toggled || changed > 0 {
                refresh_display();
            }
            publish(false, &states, apps.len(), changed, error);
            BUSY.store(false, Ordering::Release);
        });
    }

    fn due() -> bool {
        let Ok(mut last) = LAST.lock() else { return false };
        let now = Instant::now();
        if last.is_some_and(|t| now.duration_since(t) < RECHECK) {
            return false;
        }
        *last = Some(now);
        true
    }

    /// Clears macOS's icon caches, restarts the Dock (which also draws the native Launchpad) and
    /// restarts third-party launchers that keep the icons cached in memory, so Finder, Launchpad,
    /// the Dock and those launchers all re-read the icons now. Best effort: failures are ignored,
    /// the pass itself already succeeded.
    fn refresh_display() {
        if let Some(home) = std::env::var_os("HOME") {
            let caches = Path::new(&home).join("Library/Caches");
            let _ = std::fs::remove_file(caches.join("com.apple.iconservices.store"));
            let _ = std::fs::remove_dir_all(caches.join("com.apple.iconservices"));
        }
        // The native launcher: the Dock process also draws Launchpad.
        let _ = Command::new("/usr/bin/killall").arg("Dock").status();
        // LaunchOS keeps its own icon cache in memory: restart it (only when running) so it
        // re-reads the icons through NSWorkspace.
        let killed = Command::new("/usr/bin/killall").arg("LaunchOS").status().is_ok_and(|s| s.success());
        if killed {
            let _ = Command::new("/usr/bin/open").args(["-a", "LaunchOS"]).status();
        }
    }

    fn has_custom_icon(bundle: &Path) -> bool {
        bundle.join("Icon\r").exists()
    }

    /// Which icon set the bundle carries ("set:revision"), written next to the icons it got.
    fn style_marker(bundle: &Path) -> PathBuf {
        bundle.join("Contents/Resources/bettercraft-style")
    }

    /// Brings one bundle in line with `mode` and `style`. Returns true when its Dock icon changed.
    fn apply(id: &str, bundle: &Path, mode: MacMode, style: IconSet) -> Result<bool, String> {
        use crate::bettercraft::dock_icon;
        if !bundle.is_dir() {
            return Ok(false);
        }
        let patched = dock_icon::is_patched(bundle);
        // The marker reads "style:revision" ("plain" alone before revisions).
        let marker = std::fs::read_to_string(style_marker(bundle)).unwrap_or_default();
        let (key, revision) = marker.trim().split_once(':').unwrap_or((marker.trim(), "1"));
        let carried = IconSet::from_key(key).unwrap_or_default();
        let current = revision == crate::bettercraft::assets::REVISION.to_string();
        let restyle = mode.icons() && patched && (carried != style || !current);
        let want_names = mode.icons() && style.names();
        let names = want_names != has_original_name(bundle);
        let dock = mode.icons() != patched || restyle;
        // Whether anything visible changes this pass (for the UI's feedback and the Dock refresh).
        let mut changed = dock || names;
        if names || dock {
            // codesign refuses a bundle with a custom icon: clear it, sign, then put it back.
            if has_custom_icon(bundle) {
                set_icon(style, id, bundle, false)?;
            }
            let _ = std::fs::remove_file(bundle.join("Icon\r"));
            run("/usr/bin/xattr", &["-cr"], bundle)?;
            if names {
                set_name(id, bundle, want_names)?;
            }
            if dock {
                if patched {
                    // Out with the style it carries, so its padded icons are found again.
                    dock_icon::set(carried, id, bundle, false)?;
                    let _ = std::fs::remove_file(style_marker(bundle));
                }
                if mode.icons() {
                    dock_icon::set(style, id, bundle, true)?;
                    std::fs::write(style_marker(bundle), format!("{}:{}", style.key(), crate::bettercraft::assets::REVISION))
                        .map_err(|e| e.to_string())?;
                }
            }
            run("/usr/bin/codesign", &["--force", "--deep", "--preserve-metadata=entitlements,flags", "--sign", "-"], bundle)?;
            run("/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister", &["-f"], bundle)?;
        }
        if mode.icons() != has_custom_icon(bundle) {
            set_icon(style, id, bundle, mode.icons())?;
            changed = true;
        }
        Ok(changed)
    }

    fn set_icon(style: IconSet, id: &str, bundle: &Path, on: bool) -> Result<(), String> {
        let icon = if on { write_icns(style, id)?.display().to_string() } else { String::new() };
        let out =
            Command::new("/usr/bin/osascript").args(["-l", "JavaScript", "-e", SET_ICON, &icon]).arg(bundle).output().map_err(|e| e.to_string())?;
        let answer = String::from_utf8_lossy(&out.stdout);
        if !out.status.success() || answer.trim() != "ok" {
            return Err(format!("osascript said {:?} {:?}", answer.trim(), String::from_utf8_lossy(&out.stderr).trim()));
        }
        // A new modification date makes the Dock and Launchpad look again too.
        let _ = Command::new("/usr/bin/touch").arg(bundle).status();
        Ok(())
    }

    /// Languages the display name is written for: whichever the Mac prefers, it finds one.
    const LANGS: [&str; 12] = ["Base", "en", "pt", "pt-BR", "pt_BR", "es", "fr", "de", "it", "ja", "zh-Hans", "ko"];
    /// Marks the strings files this launcher wrote, so only those are removed.
    const MARK: &str = "/* BetterCraft */";

    fn strings_file(bundle: &Path, lang: &str) -> PathBuf {
        bundle.join("Contents/Resources").join(format!("{lang}.lproj")).join("InfoPlist.strings")
    }

    fn has_original_name(bundle: &Path) -> bool {
        std::fs::read_to_string(strings_file(bundle, "Base")).is_ok_and(|s| s.starts_with(MARK))
    }

    /// Writes (or removes) the display name files; the caller signs the bundle after.
    fn set_name(id: &str, bundle: &Path, on: bool) -> Result<(), String> {
        let name = super::super::icons::name(id).ok_or_else(|| format!("no original name for {id}"))?;
        let plist = bundle.join("Contents/Info.plist");
        if on {
            let body = format!("{MARK}\n\"CFBundleDisplayName\" = \"{name}\";\n\"CFBundleName\" = \"{name}\";\n");
            for lang in LANGS {
                let file = strings_file(bundle, lang);
                if file.exists() && !std::fs::read_to_string(&file).is_ok_and(|s| s.starts_with(MARK)) {
                    continue; // the app's own translation: leave it
                }
                if let Some(dir) = file.parent() {
                    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
                }
                std::fs::write(&file, &body).map_err(|e| e.to_string())?;
            }
            let set = run("/usr/libexec/PlistBuddy", &["-c", "Set :LSHasLocalizedDisplayName true"], &plist);
            if set.is_err() {
                run("/usr/libexec/PlistBuddy", &["-c", "Add :LSHasLocalizedDisplayName bool true"], &plist)?;
            }
        } else {
            for lang in LANGS {
                let file = strings_file(bundle, lang);
                if std::fs::read_to_string(&file).is_ok_and(|s| s.starts_with(MARK)) {
                    std::fs::remove_file(&file).map_err(|e| e.to_string())?;
                    if let Some(dir) = file.parent() {
                        let _ = std::fs::remove_dir(dir); // only when empty
                    }
                }
            }
            let _ = run("/usr/libexec/PlistBuddy", &["-c", "Delete :LSHasLocalizedDisplayName"], &plist);
        }
        Ok(())
    }

    /// Runs `program args… path`, turning a failure into its message.
    fn run(program: &str, args: &[&str], path: &Path) -> Result<(), String> {
        let out = Command::new(program).args(args).arg(path).output().map_err(|e| format!("{program}: {e}"))?;
        if out.status.success() {
            Ok(())
        } else {
            let name = program.rsplit('/').next().unwrap_or(program);
            Err(format!("{name}: {}", String::from_utf8_lossy(&out.stderr).trim()))
        }
    }

    /// Gives the launcher's bundle the Study logo as its Finder icon, or takes it off.
    fn dress_launcher(study: bool) -> Result<(), String> {
        let Some(bundle) = crate::bettercraft::launcher_icon::own_bundle() else { return Ok(()) };
        if study == has_custom_icon(&bundle) {
            return Ok(());
        }
        let icon = if study {
            let path = std::env::temp_dir().join("bettercraft-icons").join("launcher-study.icns");
            if let Some(dir) = path.parent() {
                std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
            }
            std::fs::write(&path, crate::bettercraft::launcher_icon::STUDY_ICNS).map_err(|e| e.to_string())?;
            path.display().to_string()
        } else {
            String::new()
        };
        let out =
            Command::new("/usr/bin/osascript").args(["-l", "JavaScript", "-e", SET_ICON, &icon]).arg(&bundle).output().map_err(|e| e.to_string())?;
        if !out.status.success() {
            return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
        }
        let _ = Command::new("/usr/bin/touch").arg(&bundle).status();
        Ok(())
    }

    /// The bundled `.icns` for `id`, written to a temporary folder for AppKit to read.
    fn write_icns(style: IconSet, id: &str) -> Result<PathBuf, String> {
        let bytes = crate::bettercraft::assets::icns(style, id).ok_or_else(|| format!("no original icon for {id}"))?;
        let dir = std::env::temp_dir().join("bettercraft-icons");
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let path = dir.join(format!("{id}-{}.icns", style.key()));
        std::fs::write(&path, bytes).map_err(|e| e.to_string())?;
        Ok(path)
    }
}
