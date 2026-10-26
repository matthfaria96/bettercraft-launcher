//! The original programs' icons, and optionally names, on the Craft Apps the launcher installed
//! as AppImages (Linux only).
//!
//! The launcher already writes, and owns, each AppImage's freedesktop files: the menu entry
//! `<data>/applications/ai.storyteller.<app>.desktop` (marked `X-ArtCraft-Launcher=true`) and its
//! icon `<data>/icons/hicolor/128x128/apps/ai.storyteller.<app>.png`. This module rewrites those
//! two files: the icon is replaced with the BetterCraft/Study icon (resized to 128 px) and, for
//! the Study set, the entry's `Name=` becomes the original program's name. The state lives in the
//! entry itself (an `X-BetterCraft-Style` key), so an update that rewrites the entry drops the
//! dressing and the next pass dresses the new copy — exactly like the in-bundle marker on macOS.

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use crate::LauncherUi;

use super::MacMode;
use super::assets::IconSet;
use super::machine_icons::{AppState, Status};

const RECHECK: Duration = Duration::from_secs(5);

/// The key the launcher writes into its own entries, marking them as its own.
const OWNED_KEY: &str = "X-ArtCraft-Launcher=true";
/// The key recording which icon set (and revision) the entry currently carries.
const STYLE_KEY: &str = "X-BetterCraft-Style";

static BUSY: AtomicBool = AtomicBool::new(false);
static LAST: Mutex<Option<Instant>> = Mutex::new(None);
static STATUS: Mutex<Option<Status>> = Mutex::new(None);

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
    let mode = super::mac_mode();
    let style = super::icon_set();
    // Only AppImages: the launcher owns their menu entry and icon. A package, Flatpak or tarball
    // is left to its own integration.
    let apps: Vec<(String, String, String)> = launcher
        .entries
        .iter()
        .filter_map(|e| {
            if e.job.is_some() {
                return None;
            }
            let location = e.installed.as_ref()?.location.clone();
            let path = PathBuf::from(location);
            path.extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("AppImage"))
                .then(|| (e.app.id.to_string(), e.app.bundle_id(), e.app.name.to_string()))
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
        for (id, bundle_id, name) in &apps {
            match apply(id, bundle_id, name, mode, style) {
                Ok(did) => changed += usize::from(did),
                Err(e) => {
                    log::warn!("BetterCraft: couldn't change the icons of {name}: {e}");
                    error = Some(format!("{name}: {e}"));
                }
            }
            states.push((id.clone(), state_of(bundle_id)));
            publish(true, &states, apps.len(), changed, error.clone());
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

/// What the installed app `bundle_id` carries, once a pass has looked at it.
fn state_of(bundle_id: &str) -> AppState {
    let Some(desktop) = desktop_path(bundle_id) else { return AppState::default() };
    let Ok(text) = std::fs::read_to_string(&desktop) else { return AppState::default() };
    let Some((key, _)) = style_of(&text) else { return AppState::default() };
    AppState { icons: true, names: key == IconSet::Study.key() }
}

/// Brings one app's entry and icon in line with `mode` and `style`. Returns whether it changed.
fn apply(id: &str, bundle_id: &str, app_name: &str, mode: MacMode, style: IconSet) -> Result<bool, String> {
    let Some(desktop) = desktop_path(bundle_id) else { return Ok(false) };
    if !desktop.exists() {
        return Ok(false);
    }
    let mut text = std::fs::read_to_string(&desktop).map_err(|e| e.to_string())?;
    if !is_ours(&text) {
        return Ok(false);
    }

    let want_custom = mode.icons();
    let want_names = want_custom && style.names();
    let want_style = want_custom.then(|| (style.key().to_string(), super::assets::REVISION));
    let wanted_name = if want_names { super::icons::name(id).unwrap_or(app_name).to_string() } else { app_name.to_string() };

    let style_ok = style_of(&text) == want_style;
    let name_ok = field(&text, "Name") == Some(wanted_name.clone());
    if style_ok && name_ok {
        return Ok(false);
    }

    // Write the icon before the entry: if the icon write fails, the entry's marker is still
    // absent, so the next pass retries both instead of trusting a marker that claims the icon
    // changed.
    if !style_ok {
        write_icon(id, bundle_id, style, want_custom)?;
    }
    if !name_ok {
        text = set_field(&text, "Name", &wanted_name);
    }
    if !style_ok {
        text = match &want_style {
            Some((key, revision)) => set_field(&text, STYLE_KEY, &format!("{key}:{revision}")),
            None => drop_field(&text, STYLE_KEY),
        };
    }
    std::fs::write(&desktop, text).map_err(|e| format!("couldn't write {} ({e})", desktop.display()))?;
    if !style_ok {
        refresh_icon_cache();
    }
    Ok(true)
}

/// Writes the app's icon: the chosen set's (resized to 128 px) when dressing, the app's own
/// 128 px picture when undressing.
fn write_icon(id: &str, bundle_id: &str, style: IconSet, custom: bool) -> Result<(), String> {
    let bytes: Vec<u8> = if custom {
        let png = super::assets::png256(style, id).ok_or_else(|| format!("no icon for {id}"))?;
        icon_128(png).ok_or_else(|| format!("couldn't resize the icon for {id}"))?
    } else {
        original_icon(id).ok_or_else(|| format!("no original icon for {id}"))?
    };
    let Some(path) = icon_path(bundle_id) else { return Ok(()) };
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    std::fs::write(&path, bytes).map_err(|e| format!("couldn't write {} ({e})", path.display()))
}

fn original_icon(id: &str) -> Option<Vec<u8>> {
    match crate::icons::app(id)? {
        egui::ImageSource::Bytes { bytes, .. } => Some(bytes.as_ref().to_vec()),
        _ => None,
    }
}

/// The 256 px picture resized to 128 px and re-encoded as PNG.
fn icon_128(png: &[u8]) -> Option<Vec<u8>> {
    let picture = image::load_from_memory(png).ok()?;
    let resized = image::imageops::resize(&picture, 128, 128, image::imageops::FilterType::Triangle);
    let mut out = Vec::new();
    resized.write_to(&mut std::io::Cursor::new(&mut out), image::ImageFormat::Png).ok()?;
    Some(out)
}

/// Refreshes the icon-theme cache so the desktop picks the new picture up at once (best effort:
/// some desktops don't use `gtk-update-icon-cache`, or don't ship it).
fn refresh_icon_cache() {
    let Some(data) = data_dir() else { return };
    let dir = data.join("icons").join("hicolor");
    if !dir.is_dir() {
        return;
    }
    let _ = std::process::Command::new("gtk-update-icon-cache").arg(&dir).status();
}

/// The freedesktop data folder: `$XDG_DATA_HOME`, else `~/.local/share` (the host's, inside the
/// Flatpak launcher — the same rule the backend uses when it writes the entries).
fn data_dir() -> Option<PathBuf> {
    if Path::new("/.flatpak-info").exists() {
        if let Some(dir) = std::env::var_os("HOST_XDG_DATA_HOME").map(PathBuf::from).filter(|p| p.is_absolute()) {
            return Some(dir);
        }
        return std::env::var_os("HOME").map(PathBuf::from).map(|h| h.join(".local").join("share"));
    }
    if let Some(xdg) = std::env::var_os("XDG_DATA_HOME").filter(|p| !p.is_empty()) {
        return Some(PathBuf::from(xdg));
    }
    std::env::var_os("HOME").map(PathBuf::from).map(|h| h.join(".local").join("share"))
}

fn desktop_path(bundle_id: &str) -> Option<PathBuf> {
    data_dir().map(|d| d.join("applications").join(format!("{bundle_id}.desktop")))
}

fn icon_path(bundle_id: &str) -> Option<PathBuf> {
    data_dir().map(|d| d.join("icons").join("hicolor").join("128x128").join("apps").join(format!("{bundle_id}.png")))
}

/// Whether the launcher wrote this entry.
fn is_ours(text: &str) -> bool {
    text.lines().any(|l| l.trim() == OWNED_KEY)
}

/// The `X-BetterCraft-Style` an entry carries, as `(set key, revision)`.
fn style_of(text: &str) -> Option<(String, u32)> {
    let value = field(text, STYLE_KEY)?;
    let (key, revision) = value.split_once(':')?;
    let revision: u32 = revision.trim().parse().ok()?;
    Some((key.to_string(), revision))
}

/// The value of the `key=` line, if present.
fn field(text: &str, key: &str) -> Option<String> {
    let prefix = format!("{key}=");
    text.lines().find_map(|l| l.strip_prefix(&prefix).map(str::to_string))
}

/// `text` with the `key=` line set to `key=<value>` (appended when absent).
fn set_field(text: &str, key: &str, value: &str) -> String {
    let prefix = format!("{key}=");
    let mut found = false;
    let mut out = String::with_capacity(text.len() + value.len() + 1);
    for line in text.lines() {
        if line.starts_with(&prefix) {
            out.push_str(key);
            out.push('=');
            out.push_str(value);
            out.push('\n');
            found = true;
        } else {
            out.push_str(line);
            out.push('\n');
        }
    }
    if !found {
        out.push_str(key);
        out.push('=');
        out.push_str(value);
        out.push('\n');
    }
    out
}

/// `text` without any `key=` line.
fn drop_field(text: &str, key: &str) -> String {
    let prefix = format!("{key}=");
    let mut out = String::with_capacity(text.len());
    for line in text.lines() {
        if !line.starts_with(&prefix) {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENTRY: &str = "[Desktop Entry]\nType=Application\nName=PhotoCraft\nExec=\"/home/a/photo.AppImage\" %F\nIcon=ai.storyteller.photocraft\nX-ArtCraft-Launcher=true\n";

    #[test]
    fn recognises_only_the_launchers_own_entries() {
        assert!(is_ours(ENTRY));
        assert!(!is_ours("[Desktop Entry]\nName=PhotoCraft\nExec=x\n"));
    }

    #[test]
    fn sets_and_drops_fields() {
        let with_name = set_field(ENTRY, "Name", "Adobe Photoshop");
        assert!(with_name.contains("Name=Adobe Photoshop\n"));
        assert!(!with_name.contains("Name=PhotoCraft\n"));

        let styled = set_field(ENTRY, STYLE_KEY, "study:3");
        assert_eq!(style_of(&styled), Some(("study".to_string(), 3)));
        let dropped = drop_field(&styled, STYLE_KEY);
        assert_eq!(style_of(&dropped), None);
        assert!(dropped.contains("X-ArtCraft-Launcher=true\n"));
    }

    #[test]
    fn field_reads_the_name() {
        assert_eq!(field(ENTRY, "Name").as_deref(), Some("PhotoCraft"));
        assert_eq!(field(ENTRY, "Nope"), None);
    }

    #[test]
    fn resizes_the_256_px_icon_to_128() {
        let png = super::super::assets::png256(IconSet::Craft, "photocraft").unwrap();
        let out = icon_128(png).unwrap();
        let picture = image::load_from_memory(&out).unwrap();
        assert_eq!((picture.width(), picture.height()), (128, 128));
    }
}
