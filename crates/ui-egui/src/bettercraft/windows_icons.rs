//! The original programs' icons, and optionally names, on the installed Craft Apps on Windows.
//!
//! Windows shows an app through its Start Menu, desktop and taskbar shortcuts (`.lnk` files):
//! each carries an icon location, and its displayed name is the file's own name. This module
//! points those shortcuts at the BetterCraft `.ico` (in the chosen style) and, for the Study set,
//! renames them to the original program's name. Everything it changes is recorded in a small
//! marker file, so switching back ("Clone") restores each shortcut exactly, without keeping the
//! pictures around.
//!
//! The `.lnk` binary format is owned by the Windows shell, so the launcher drives the shell's own
//! `WScript.Shell` through PowerShell rather than parsing the file itself. Renames are plain file
//! renames (no re-save), which leaves every other shortcut property untouched. An app is never
//! touched while the launcher downloads, installs or removes it, and a failure on one shortcut is
//! reported, never a crash.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use crate::LauncherUi;

use super::MacMode;
use super::assets::IconSet;
use super::machine_icons::{AppState, Status};

const RECHECK: Duration = Duration::from_secs(5);
/// `CREATE_NO_WINDOW`: the helper PowerShell must not flash a console window.
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

static BUSY: AtomicBool = AtomicBool::new(false);
static LAST: Mutex<Option<Instant>> = Mutex::new(None);
static STATUS: Mutex<Option<Status>> = Mutex::new(None);

/// Lists the shortcuts that belong to the app at `BC_TARGET`, one `path<TAB>icon-location` per
/// line. A shortcut belongs when its target is the app's exe, or (for Windows Installer
/// shortcuts, whose target is a small icon-stub exe in `C:\Windows\Installer`) when its working
/// directory is the app's install folder `BC_DIR`.
const LIST_SCRIPT: &str = r#"$ErrorActionPreference = 'SilentlyContinue'
$target = [System.IO.Path]::GetFullPath($env:BC_TARGET)
$folder = ''
if ($env:BC_DIR) { $folder = [System.IO.Path]::GetFullPath($env:BC_DIR).TrimEnd('\') }
$ws = New-Object -ComObject WScript.Shell
$dirs = @(
  ($env:ProgramData + '\Microsoft\Windows\Start Menu\Programs'),
  ($env:APPDATA + '\Microsoft\Windows\Start Menu\Programs'),
  ($env:PUBLIC + '\Desktop'),
  ($env:USERPROFILE + '\Desktop'),
  ($env:APPDATA + '\Microsoft\Internet Explorer\Quick Launch\User Pinned\TaskBar')
)
foreach ($dir in $dirs) {
  if (-not $dir -or -not (Test-Path -LiteralPath $dir)) { continue }
  Get-ChildItem -LiteralPath $dir -Filter '*.lnk' -Recurse -File -ErrorAction SilentlyContinue | ForEach-Object {
    try {
      $s = $ws.CreateShortcut($_.FullName)
      $tp = $s.TargetPath
      $match = $false
      if ($tp -and ([System.IO.Path]::GetFullPath([string]$tp) -ieq $target)) { $match = $true }
      if (-not $match -and $folder) {
        $wd = $s.WorkingDirectory
        if ($wd -and ([System.IO.Path]::GetFullPath([string]$wd).TrimEnd('\') -ieq $folder)) { $match = $true }
      }
      if ($match) {
        Write-Output (([string]$_.FullName) + "`t" + ([string]$s.IconLocation))
      }
    } catch {}
  }
}
"#;

/// Sets the icon location of the shortcut at `BC_LNK` to `BC_ICON` (`",0"` — never an empty
/// string, which `WScript.Shell` refuses — puts the target's own icon back).
const SET_ICON_SCRIPT: &str = r#"$ErrorActionPreference = 'SilentlyContinue'
$ws = New-Object -ComObject WScript.Shell
$s = $ws.CreateShortcut($env:BC_LNK)
$s.IconLocation = [string]$env:BC_ICON
$s.Save()
"#;

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
    let apps: Vec<(String, PathBuf)> = launcher
        .entries
        .iter()
        .filter_map(|e| {
            if e.job.is_some() {
                return None;
            }
            let location = e.installed.as_ref()?.location.clone();
            let exe = PathBuf::from(location);
            exe.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("exe")).then(|| (e.app.id.to_string(), exe))
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
        for (id, exe) in &apps {
            match apply(id, exe, mode, style) {
                Ok(did) => changed += usize::from(did),
                Err(e) => {
                    log::warn!("BetterCraft: couldn't change the shortcuts of {}: {e}", exe.display());
                    let e = if access_denied(&e) {
                        format!("{e} (run BetterCraft Launcher as administrator to change apps installed for all users)")
                    } else {
                        e
                    };
                    error = Some(format!("{}: {e}", exe.display()));
                }
            }
            states.push((id.clone(), state_of(id)));
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

/// Whether `error` is a permission denial (the shortcut lives in an all-users folder).
fn access_denied(error: &str) -> bool {
    let lower = error.to_lowercase();
    lower.contains("denied") || lower.contains("negado") || lower.contains("0x80070005") || lower.contains("0x80030005")
}

/// One shortcut the launcher dressed, and what to put back to undo it.
#[derive(Clone, Debug)]
struct Shortcut {
    /// The shortcut's path after any rename.
    path: PathBuf,
    /// Its file name before the rename (the name to restore).
    name: String,
    /// Its icon location before the change (`",0"` means the target's own icon).
    icon: String,
}

/// What one app currently carries, kept in its marker file.
#[derive(Clone, Debug)]
struct Marker {
    style: String,
    revision: u32,
    shortcuts: Vec<Shortcut>,
}

/// What the installed app `id` carries, once a pass has looked at it.
fn state_of(id: &str) -> AppState {
    let Some(marker) = marker_path(id).and_then(|p| read_marker(&p)) else {
        return AppState::default();
    };
    AppState { icons: true, names: marker.style == IconSet::Study.key() }
}

/// Brings one app's shortcuts in line with `mode` and `style`. Returns whether anything changed.
fn apply(id: &str, exe: &Path, mode: MacMode, style: IconSet) -> Result<bool, String> {
    let Some(marker) = marker_path(id) else { return Ok(false) };
    let current = read_marker(&marker);
    let mut changed = false;
    if mode.icons() {
        if let Some(cur) = &current {
            // An empty record (an app with no shortcuts yet) never counts as dressed: the next
            // pass lists again, so a shortcut that appears later is still caught.
            if !cur.shortcuts.is_empty() && cur.style == style.key() && cur.revision == super::assets::REVISION {
                return Ok(false);
            }
            if let Err(e) = revert(cur) {
                log::warn!("BetterCraft: couldn't undo the previous icons of {id}: {e}");
            }
            changed = true;
        }
        match apply_custom(id, exe, style)? {
            Some(rec) => {
                write_marker(&marker, &rec)?;
                changed = true;
            }
            None => {
                let _ = std::fs::remove_file(&marker);
            }
        }
    } else if let Some(cur) = &current {
        if let Err(e) = revert(cur) {
            log::warn!("BetterCraft: couldn't undo the icons of {id}: {e}");
        }
        let _ = std::fs::remove_file(&marker);
        changed = true;
    }
    Ok(changed)
}

/// Dresses the shortcuts of `exe` in `style`; `None` when the app has no shortcuts to change.
fn apply_custom(id: &str, exe: &Path, style: IconSet) -> Result<Option<Marker>, String> {
    let shortcuts = list_shortcuts(exe)?;
    if shortcuts.is_empty() {
        return Ok(None);
    }
    let icon = icon_path(id, style)?;
    let rename_to = style.names().then(|| super::icons::name(id)).flatten();
    let mut records = Vec::new();
    for (path, current_icon) in shortcuts {
        let original_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
        if current_icon != icon {
            set_icon(&path, &icon)?;
        }
        let final_path = match rename_to {
            Some(name) if valid_name(name) => {
                let wanted = format!("{name}.lnk");
                if path.file_name().and_then(|n| n.to_str()) == Some(wanted.as_str()) { path } else { rename_shortcut(&path, &wanted)? }
            }
            _ => path,
        };
        records.push(Shortcut { path: final_path, name: original_name, icon: current_icon });
    }
    Ok(Some(Marker { style: style.key().to_string(), revision: super::assets::REVISION, shortcuts: records }))
}

/// Puts back every original icon and name the marker recorded.
fn revert(marker: &Marker) -> Result<(), String> {
    let mut first: Option<String> = None;
    for s in &marker.shortcuts {
        if s.path.exists()
            && let Err(e) = set_icon(&s.path, &s.icon)
        {
            first.get_or_insert(e);
        }
        if valid_name(&s.name) {
            let current = s.path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if current != s.name
                && let Some(parent) = s.path.parent()
            {
                let original = parent.join(&s.name);
                if s.path.exists()
                    && !original.exists()
                    && let Err(e) = std::fs::rename(&s.path, &original)
                {
                    first.get_or_insert(format!("couldn't restore {} ({e})", original.display()));
                }
            }
        }
    }
    match first {
        Some(e) => Err(e),
        None => Ok(()),
    }
}

fn list_shortcuts(exe: &Path) -> Result<Vec<(PathBuf, String)>, String> {
    let parent = exe.parent().unwrap_or(Path::new(""));
    let out = powershell(LIST_SCRIPT, &[("BC_TARGET", exe.as_os_str()), ("BC_DIR", parent.as_os_str())])?;
    let mut found = Vec::new();
    for line in out.lines() {
        let line = line.trim_end_matches('\r');
        let Some((path, icon)) = line.split_once('\t') else { continue };
        found.push((PathBuf::from(path), icon.to_string()));
    }
    Ok(found)
}

fn set_icon(lnk: &Path, icon: &str) -> Result<(), String> {
    let location = icon_location(icon);
    powershell(SET_ICON_SCRIPT, &[("BC_LNK", lnk.as_os_str()), ("BC_ICON", OsStr::new(location.as_str()))]).map(|_| ())
}

/// The value to hand `WScript.Shell` for an icon: `,0` (the target's own) for an empty one, and
/// a bare path gets `,0` appended so the shell always reads the icon's first frame.
fn icon_location(icon: &str) -> String {
    if icon.is_empty() || icon == ",0" {
        ",0".to_string()
    } else if icon.contains(',') {
        icon.to_string()
    } else {
        format!("{icon},0")
    }
}

fn powershell(script: &str, env: &[(&str, &OsStr)]) -> Result<String, String> {
    use std::os::windows::process::CommandExt;
    let mut cmd = Command::new("powershell.exe");
    cmd.args(["-NoLogo", "-NoProfile", "-NonInteractive", "-Command", script]);
    cmd.creation_flags(CREATE_NO_WINDOW);
    for &(key, value) in env {
        cmd.env(key, value);
    }
    let out = cmd.output().map_err(|e| format!("couldn't start PowerShell ({e})"))?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        let err = String::from_utf8_lossy(&out.stderr).trim().to_string();
        Err(if err.is_empty() { format!("PowerShell failed ({})", out.status) } else { err })
    }
}

/// The app's data folder, in the roaming profile: `%APPDATA%\BetterCraft Launcher`.
fn data_dir() -> Option<PathBuf> {
    std::env::var_os("APPDATA").map(PathBuf::from).map(|d| d.join("BetterCraft Launcher"))
}

fn marker_path(id: &str) -> Option<PathBuf> {
    data_dir().map(|d| d.join("apps").join(id))
}

fn read_marker(path: &Path) -> Option<Marker> {
    parse_marker(&std::fs::read_to_string(path).ok()?)
}

fn write_marker(path: &Path, marker: &Marker) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    std::fs::write(path, render_marker(marker)).map_err(|e| e.to_string())
}

/// Writes (or refreshes) the `.ico` for `id` and returns its path.
fn icon_path(id: &str, style: IconSet) -> Result<String, String> {
    let Some(ico) = ico_bytes(style, id) else { return Err(format!("no icon for {id}")) };
    let Some(dir) = data_dir().map(|d| d.join("icons")) else { return Err("no application-data folder".into()) };
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join(format!("{id}-{}.ico", style.key()));
    std::fs::write(&path, &ico).map_err(|e| format!("couldn't write {} ({e})", path.display()))?;
    Ok(path.display().to_string())
}

fn ico_bytes(style: IconSet, id: &str) -> Option<Vec<u8>> {
    ico_from_png(super::assets::png256(style, id)?)
}

/// Wraps one square PNG in a single-image ICO (Vista and later read PNG frames in an ICO).
fn ico_from_png(png: &[u8]) -> Option<Vec<u8>> {
    let width = u32::from_be_bytes(png.get(16..20)?.try_into().ok()?);
    let height = u32::from_be_bytes(png.get(20..24)?.try_into().ok()?);
    let mut ico = Vec::with_capacity(22 + png.len());
    ico.extend_from_slice(&0u16.to_le_bytes()); // reserved
    ico.extend_from_slice(&1u16.to_le_bytes()); // type: icon
    ico.extend_from_slice(&1u16.to_le_bytes()); // one image
    ico.push(icon_dim(width));
    ico.push(icon_dim(height));
    ico.push(0); // colour count
    ico.push(0); // reserved
    ico.extend_from_slice(&1u16.to_le_bytes()); // planes
    ico.extend_from_slice(&32u16.to_le_bytes()); // bit count
    ico.extend_from_slice(&(png.len() as u32).to_le_bytes()); // bytes in image
    ico.extend_from_slice(&22u32.to_le_bytes()); // image offset
    ico.extend_from_slice(png);
    Some(ico)
}

/// The byte Windows stores for a dimension in an ICO entry: 0 means 256.
fn icon_dim(size: u32) -> u8 {
    if size >= 256 { 0 } else { size.min(255) as u8 }
}

/// Whether a file name is safe to use as a shortcut's name.
fn valid_name(name: &str) -> bool {
    !name.is_empty() && !name.chars().any(|c| matches!(c, '\\' | '/' | ':' | '*' | '?' | '"' | '<' | '>' | '|'))
}

fn rename_shortcut(path: &Path, new_name: &str) -> Result<PathBuf, String> {
    let parent = path.parent().ok_or_else(|| format!("{} has no parent folder", path.display()))?;
    let new_path = parent.join(new_name);
    if new_path == path {
        return Ok(path.to_path_buf());
    }
    if new_path.exists() {
        return Err(format!("{} already exists; left {} alone", new_path.display(), path.display()));
    }
    std::fs::rename(path, &new_path).map_err(|e| format!("couldn't rename {} ({e})", path.display()))?;
    Ok(new_path)
}

fn parse_marker(text: &str) -> Option<Marker> {
    let mut lines = text.lines();
    let head = lines.next()?;
    let (style, revision) = head.split_once(':')?;
    let revision: u32 = revision.trim().parse().ok()?;
    let mut shortcuts = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let mut parts = line.splitn(3, '\t');
        let path = parts.next()?;
        let name = parts.next()?;
        let icon = parts.next()?;
        shortcuts.push(Shortcut { path: PathBuf::from(path), name: name.to_string(), icon: icon.to_string() });
    }
    Some(Marker { style: style.to_string(), revision, shortcuts })
}

fn render_marker(marker: &Marker) -> String {
    let mut out = String::new();
    out.push_str(&marker.style);
    out.push(':');
    out.push_str(&marker.revision.to_string());
    out.push('\n');
    for s in &marker.shortcuts {
        out.push_str(&s.path.display().to_string());
        out.push('\t');
        out.push_str(&s.name);
        out.push('\t');
        out.push_str(&s.icon);
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_a_png_in_a_single_image_ico() {
        let png = super::super::assets::png256(IconSet::Craft, "photocraft").unwrap();
        let ico = ico_from_png(png).unwrap();
        assert_eq!(&ico[0..2], &0u16.to_le_bytes(), "reserved");
        assert_eq!(&ico[2..4], &1u16.to_le_bytes(), "type icon");
        assert_eq!(&ico[4..6], &1u16.to_le_bytes(), "one image");
        assert_eq!(ico[6], 0, "width 256");
        assert_eq!(ico[7], 0, "height 256");
        assert_eq!(u32::from_le_bytes(ico[14..18].try_into().unwrap()), png.len() as u32, "bytes in image");
        assert_eq!(u32::from_le_bytes(ico[18..22].try_into().unwrap()), 22, "image offset");
        assert_eq!(&ico[22..], png, "png payload");
    }

    #[test]
    fn icon_dimensions_use_zero_for_256() {
        assert_eq!(icon_dim(256), 0);
        assert_eq!(icon_dim(1024), 0);
        assert_eq!(icon_dim(128), 128);
        assert_eq!(icon_dim(16), 16);
    }

    #[test]
    fn marker_round_trips() {
        let marker = Marker {
            style: "study".to_string(),
            revision: 3,
            shortcuts: vec![
                Shortcut { path: PathBuf::from(r"C:\Users\x\Desktop\Adobe Photoshop.lnk"), name: "PhotoCraft.lnk".to_string(), icon: String::new() },
                Shortcut {
                    path: PathBuf::from(r"C:\Users\x\Desktop\PhotoCraft.lnk"),
                    name: "PhotoCraft.lnk".to_string(),
                    icon: r"C:\x\photocraft.exe,0".to_string(),
                },
            ],
        };
        let text = render_marker(&marker);
        let parsed = parse_marker(&text).unwrap();
        assert_eq!(parsed.style, "study");
        assert_eq!(parsed.revision, 3);
        assert_eq!(parsed.shortcuts.len(), 2);
        assert_eq!(parsed.shortcuts[0].name, "PhotoCraft.lnk");
        assert_eq!(parsed.shortcuts[0].icon, "");
        assert_eq!(parsed.shortcuts[1].icon, r"C:\x\photocraft.exe,0");
    }

    #[test]
    fn shortcut_names_must_be_plain_file_names() {
        assert!(valid_name("Adobe Photoshop"));
        assert!(valid_name("AutoCAD"));
        assert!(!valid_name(""));
        assert!(!valid_name("a/b"));
        assert!(!valid_name("a:b"));
        assert!(!valid_name("a*b"));
    }

    #[test]
    fn an_empty_icon_location_means_the_targets_own_icon() {
        assert_eq!(icon_location(""), ",0");
        assert_eq!(icon_location(",0"), ",0");
        assert_eq!(icon_location(r"C:\x\photocraft.exe,0"), r"C:\x\photocraft.exe,0");
        assert_eq!(icon_location(r"C:\x\photocraft.ico"), r"C:\x\photocraft.ico,0");
    }

    #[test]
    fn permission_denials_are_recognised() {
        assert!(access_denied("Access is denied. (os error 5)"));
        assert!(access_denied("Acesso negado. (os error 5)"));
        assert!(access_denied("exception from HRESULT: 0x80070005"));
        assert!(access_denied("STG_E_ACCESSDENIED 0x80030005"));
        assert!(!access_denied("the file already exists"));
        assert!(!access_denied("no icon for photocraft"));
    }
}
