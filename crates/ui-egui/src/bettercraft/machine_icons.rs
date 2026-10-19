//! The original programs' icons on the installed Craft App bundles, so Finder, Launchpad and the
//! Dock show them too (macOS only).
//!
//! macOS keeps a custom icon in an `Icon\r` file at the bundle's root. An update replaces the whole
//! `.app`, dropping it, so while the choice is on the bundles are checked every few seconds and the
//! icon is put back where it is missing. Turning the choice off clears the custom icons.

use crate::LauncherUi;

#[cfg(target_os = "macos")]
pub(super) fn sync(launcher: &LauncherUi, toggled: bool) {
    imp::sync(launcher, toggled);
}

#[cfg(not(target_os = "macos"))]
pub(super) fn sync(_launcher: &LauncherUi, _toggled: bool) {}

#[cfg(target_os = "macos")]
mod imp {
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::{Duration, Instant};

    use crate::LauncherUi;

    const RECHECK: Duration = Duration::from_secs(10);

    static BUSY: AtomicBool = AtomicBool::new(false);
    static LAST: Mutex<Option<Instant>> = Mutex::new(None);

    /// Sets (with an `.icns` path) or clears (with an empty one) a file's custom icon.
    const SET_ICON: &str = "ObjC.import('AppKit');\
        function run(argv) {\
          const image = argv[0] ? $.NSImage.alloc.initWithContentsOfFile(argv[0]) : undefined;\
          return $.NSWorkspace.sharedWorkspace.setIconForFileOptions(image, argv[1], 0) ? 'ok' : 'failed';\
        }";

    pub(super) fn sync(launcher: &LauncherUi, toggled: bool) {
        let on = super::super::ORIGINAL_ICONS.get();
        // Turned off: clear once. Otherwise only while on, every RECHECK.
        if !on && !toggled {
            return;
        }
        if !toggled && !due() {
            return;
        }
        let apps: Vec<(String, PathBuf)> = launcher
            .entries
            .iter()
            .filter_map(|e| {
                let location = &e.installed.as_ref()?.location;
                location.ends_with(".app").then(|| (e.app.id.to_string(), PathBuf::from(location)))
            })
            .collect();
        if apps.is_empty() || BUSY.swap(true, Ordering::AcqRel) {
            return;
        }
        std::thread::spawn(move || {
            for (id, bundle) in apps {
                if let Err(e) = apply(&id, &bundle, on) {
                    log::warn!("BetterCraft: couldn't change the icon of {}: {e}", bundle.display());
                }
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

    fn apply(id: &str, bundle: &Path, on: bool) -> Result<(), String> {
        if !bundle.is_dir() || on == has_custom_icon(bundle) {
            return Ok(());
        }
        let icon = if on { write_icns(id)?.display().to_string() } else { String::new() };
        let out =
            Command::new("/usr/bin/osascript").args(["-l", "JavaScript", "-e", SET_ICON, &icon]).arg(bundle).output().map_err(|e| e.to_string())?;
        let answer = String::from_utf8_lossy(&out.stdout);
        if !out.status.success() || answer.trim() != "ok" {
            return Err(format!("osascript said {:?} {:?}", answer.trim(), String::from_utf8_lossy(&out.stderr).trim()));
        }
        // Nudge Finder and the Dock to redraw the bundle.
        let _ = Command::new("/usr/bin/touch").arg(bundle).status();
        Ok(())
    }

    /// The bundled `.icns` for `id`, written to a temporary folder for AppKit to read.
    fn write_icns(id: &str) -> Result<PathBuf, String> {
        let bytes = super::super::icons::icns(id).ok_or_else(|| format!("no original icon for {id}"))?;
        let dir = std::env::temp_dir().join("bettercraft-icons");
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let path = dir.join(format!("{id}.icns"));
        std::fs::write(&path, bytes).map_err(|e| e.to_string())?;
        Ok(path)
    }
}
