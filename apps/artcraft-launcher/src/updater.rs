//! BetterCraft: the launcher's own updates through Sparkle 2 (macOS).
//!
//! Sparkle is an Objective-C framework and this crate has no unsafe code, so it runs in a small
//! helper inside the bundle, `Contents/Helpers/BetterCraft Updater.app` (`packaging/macos/updater`).
//! It reads the feed and the key from the launcher's Info.plist and replaces the launcher's bundle.
//! The launcher only starts it: once at launch (daily checks while the launcher runs) and from the
//! tray's "Check for Launcher Updates…". Builds without the helper (other systems, `cargo run`)
//! skip all of this.

use std::path::PathBuf;
use std::process::{Command, Stdio};

/// The helper's executable, when this launcher runs from an app bundle that has one.
fn helper() -> Option<PathBuf> {
    if !cfg!(target_os = "macos") {
        return None;
    }
    let exe = std::env::current_exe().ok()?;
    let contents = exe.parent()?.parent()?;
    let path = contents.join("Helpers/BetterCraft Updater.app/Contents/MacOS/BetterCraft Updater");
    path.is_file().then_some(path)
}

/// True when the tray should offer "Check for Launcher Updates…".
pub fn available() -> bool {
    helper().is_some()
}

fn spawn(check: bool) {
    let Some(path) = helper() else { return };
    let mut command = Command::new(&path);
    command.arg("--parent").arg(std::process::id().to_string());
    if check {
        command.arg("--check");
    }
    command.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
    match command.spawn() {
        // Reaped on its own thread: the helper lives as long as the launcher.
        Ok(mut child) => {
            std::thread::spawn(move || {
                let _ = child.wait();
            });
        }
        Err(e) => log::warn!("couldn't start the updater {}: {e}", path.display()),
    }
}

/// Starts the helper's scheduled checks. Once, at launch.
pub fn start() {
    spawn(false);
}

/// "Check for Launcher Updates…": the helper always shows a result.
pub fn check_now() {
    spawn(true);
}
