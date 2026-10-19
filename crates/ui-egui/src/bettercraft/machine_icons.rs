//! The original programs' icons on the installed Craft App bundles, so Finder, Launchpad and the
//! Dock show them too (macOS only).
//!
//! macOS keeps a custom icon in an `Icon\r` file at the bundle's root. The bundles are kept in step
//! with the switch both ways: on, a bundle without the icon gets it (an update replaces the whole
//! `.app`, dropping it); off, a bundle with it is cleared back to the Craft icon. The check runs at
//! once when the switch is clicked and every few seconds after.

use crate::LauncherUi;

/// What the last pass found, for the switch's label and tooltip.
#[derive(Clone, Debug, Default)]
pub(super) struct Status {
    pub busy: bool,
    /// Installed apps that carry the original icon.
    pub swapped: usize,
    /// Installed `.app` bundles.
    pub total: usize,
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

    use super::Status;
    use crate::LauncherUi;

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
        let on = super::super::ORIGINAL_ICONS.get();
        let apps: Vec<(String, PathBuf)> = launcher
            .entries
            .iter()
            .filter_map(|e| {
                let location = &e.installed.as_ref()?.location;
                location.ends_with(".app").then(|| (e.app.id.to_string(), PathBuf::from(location)))
            })
            .collect();
        if BUSY.swap(true, Ordering::AcqRel) {
            return;
        }
        std::thread::spawn(move || {
            let mut error = None;
            for (id, bundle) in &apps {
                if let Err(e) = apply(id, bundle, on) {
                    log::warn!("BetterCraft: couldn't change the icon of {}: {e}", bundle.display());
                    error = Some(format!("{}: {e}", bundle.display()));
                }
            }
            let swapped = apps.iter().filter(|(_, b)| has_custom_icon(b)).count();
            if let Ok(mut s) = STATUS.lock() {
                *s = Some(Status { busy: false, swapped, total: apps.len(), error });
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
        // A new modification date makes the Dock and Launchpad look again too.
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
