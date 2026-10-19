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

/// What one installed app carries on this Mac.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct AppState {
    /// The original program's icon everywhere: Finder, the bundle and the Dock.
    pub icons: bool,
    /// The original program's name.
    pub names: bool,
}

/// What the last pass found, for the switch's label and tooltip.
#[derive(Clone, Debug, Default)]
pub(super) struct Status {
    pub busy: bool,
    /// Installed apps that carry the original icon.
    pub icons: usize,
    /// Installed apps that carry the original name.
    pub names: usize,
    /// Installed `.app` bundles.
    pub total: usize,
    /// Each installed app's id and what it carries, for the badge on its row.
    pub apps: Vec<(String, AppState)>,
    pub error: Option<String>,
}

#[cfg(target_os = "macos")]
pub(super) fn sync(launcher: &LauncherUi, toggled: bool) {
    imp::sync(launcher, toggled);
}

#[cfg(target_os = "macos")]
pub(super) fn status() -> Status {
    imp::status()
}

#[cfg(not(target_os = "macos"))]
pub(super) fn sync(_launcher: &LauncherUi, _toggled: bool) {}

/// What the installed app `id` carries, once a pass has looked at it.
pub(crate) fn app_state(id: &str) -> Option<AppState> {
    status().apps.into_iter().find(|(app, _)| app == id).map(|(_, state)| state)
}

#[cfg(not(target_os = "macos"))]
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
    use crate::bettercraft::assets::IconStyle;

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

    pub(super) fn sync(launcher: &LauncherUi, toggled: bool) {
        if !toggled && !due() {
            return;
        }
        let mode = super::super::mac_mode();
        let style = super::super::icon_style();
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
        std::thread::spawn(move || {
            let mut error = None;
            let mut dock_changed = false;
            for (id, bundle) in &apps {
                match apply(id, bundle, mode, style) {
                    Ok(changed) => dock_changed |= changed,
                    Err(e) => {
                        log::warn!("BetterCraft: couldn't change the icon of {}: {e}", bundle.display());
                        error = Some(format!("{}: {e}", bundle.display()));
                    }
                }
            }
            // The Dock caches icons: restarting it (a second's blink) makes it read them again.
            // A Craft App that is open keeps its old Dock icon until it is opened again.
            if dock_changed {
                let _ = Command::new("/usr/bin/killall").arg("Dock").status();
            }
            let states: Vec<(String, AppState)> = apps
                .iter()
                .map(|(id, b)| {
                    let icons = has_custom_icon(b) && crate::bettercraft::dock_icon::is_patched(b);
                    (id.clone(), AppState { icons, names: has_original_name(b) })
                })
                .collect();
            let icons = states.iter().filter(|(_, s)| s.icons).count();
            let names = states.iter().filter(|(_, s)| s.names).count();
            if let Ok(mut s) = STATUS.lock() {
                *s = Some(Status { busy: false, icons, names, total: apps.len(), error, apps: states });
            }
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

    fn has_custom_icon(bundle: &Path) -> bool {
        bundle.join("Icon\r").exists()
    }

    /// Which icon style the bundle carries, written next to the icons it got.
    fn style_marker(bundle: &Path) -> PathBuf {
        bundle.join("Contents/Resources/bettercraft-style")
    }

    /// Brings one bundle in line with `mode` and `style`. Returns true when its Dock icon changed.
    fn apply(id: &str, bundle: &Path, mode: MacMode, style: IconStyle) -> Result<bool, String> {
        use crate::bettercraft::dock_icon;
        if !bundle.is_dir() {
            return Ok(false);
        }
        let patched = dock_icon::is_patched(bundle);
        let carried = std::fs::read_to_string(style_marker(bundle)).ok().and_then(|k| IconStyle::from_key(&k)).unwrap_or_default();
        let restyle = mode.icons() && patched && carried != style;
        let names = mode.names() != has_original_name(bundle);
        let dock = mode.icons() != patched || restyle;
        if names || dock {
            // codesign refuses a bundle with a custom icon: clear it, sign, then put it back.
            if has_custom_icon(bundle) {
                set_icon(style, id, bundle, false)?;
            }
            let _ = std::fs::remove_file(bundle.join("Icon\r"));
            run("/usr/bin/xattr", &["-cr"], bundle)?;
            if names {
                set_name(id, bundle, mode.names())?;
            }
            if dock {
                if patched {
                    // Out with the style it carries, so its padded icons are found again.
                    dock_icon::set(carried, id, bundle, false)?;
                    let _ = std::fs::remove_file(style_marker(bundle));
                }
                if mode.icons() {
                    dock_icon::set(style, id, bundle, true)?;
                    std::fs::write(style_marker(bundle), style.key()).map_err(|e| e.to_string())?;
                }
            }
            run("/usr/bin/codesign", &["--force", "--deep", "--preserve-metadata=entitlements,flags", "--sign", "-"], bundle)?;
            run("/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister", &["-f"], bundle)?;
        }
        if mode.icons() != has_custom_icon(bundle) {
            set_icon(style, id, bundle, mode.icons())?;
        }
        Ok(dock)
    }

    fn set_icon(style: IconStyle, id: &str, bundle: &Path, on: bool) -> Result<(), String> {
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

    /// The bundled `.icns` for `id`, written to a temporary folder for AppKit to read.
    fn write_icns(style: IconStyle, id: &str) -> Result<PathBuf, String> {
        let bytes = crate::bettercraft::assets::icns(style, id).ok_or_else(|| format!("no original icon for {id}"))?;
        let dir = std::env::temp_dir().join("bettercraft-icons");
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let path = dir.join(format!("{id}-{}.icns", style.key()));
        std::fs::write(&path, bytes).map_err(|e| e.to_string())?;
        Ok(path)
    }
}
