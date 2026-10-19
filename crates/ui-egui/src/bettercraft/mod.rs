//! BetterCraft: everything this fork adds to the upstream `storytold/craft-launcher`.
//!
//! - `icons`: the original programs' icons, shown beside each Craft icon in the compact rows.
//! - `machine_icons`: those icons, and optionally names, on the installed `.app` bundles (macOS).
//! - `compact`: a denser, quieter grid of apps (always on).
//!
//! Upstream files only carry one-line hooks into this module (`lib.rs`, `views.rs`), so the fork
//! keeps merging cleanly with upstream. Choices are written to a small file the moment they change
//! (egui's memory only saves every 30 s or on a clean quit), so the upstream `Settings` struct is
//! never touched.

use std::sync::atomic::{AtomicBool, Ordering};

use egui::Ui;

use crate::LauncherUi;
use crate::widgets;

pub(crate) mod compact;
pub(crate) mod icons;
mod machine_icons;

/// Saved choices as one small text file per key, in the user's data folder (desktop only).
mod store {
    #[cfg(not(target_arch = "wasm32"))]
    fn path(key: &str) -> Option<std::path::PathBuf> {
        let home = std::path::PathBuf::from(std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE"))?);
        let base = if cfg!(target_os = "macos") {
            home.join("Library/Application Support/BetterCraft Launcher")
        } else {
            home.join(".config/bettercraft-launcher")
        };
        Some(base.join(key))
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn read(key: &str) -> Option<bool> {
        let text = std::fs::read_to_string(path(key)?).ok()?;
        match text.trim() {
            "1" => Some(true),
            "0" => Some(false),
            _ => None,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn read_text(key: &str) -> Option<String> {
        std::fs::read_to_string(path(key)?).ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn write_text(key: &str, text: &str) {
        let Some(path) = path(key) else { return };
        let saved = path.parent().map_or(Ok(()), std::fs::create_dir_all).and_then(|()| std::fs::write(&path, text));
        if let Err(e) = saved {
            log::warn!("BetterCraft: couldn't save {key}: {e}");
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub(super) fn read_text(_key: &str) -> Option<String> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    pub(super) fn write_text(_key: &str, _text: &str) {}

    #[cfg(target_arch = "wasm32")]
    pub(super) fn read(_key: &str) -> Option<bool> {
        None
    }
}

/// Compact rows everywhere but upstream's own UI tests, which check upstream's cards.
pub(crate) fn compact() -> bool {
    !cfg!(feature = "upstream-look")
}

/// What the installed apps get on this Mac.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MacMode {
    /// The Craft icons and names, as installed.
    Off,
    /// The original programs' icons.
    Icons,
    /// The original programs' icons and names.
    IconsAndNames,
}

impl MacMode {
    pub(crate) fn icons(self) -> bool {
        self != Self::Off
    }

    pub(crate) fn names(self) -> bool {
        self == Self::IconsAndNames
    }
}

const MODE_KEY: &str = "bettercraft-mac-mode";
/// The on/off file of the version with icons only; read when there is no mode yet.
const OLD_ICONS_KEY: &str = "bettercraft-original-icons";
static MODE: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
static MODE_LOADED: AtomicBool = AtomicBool::new(false);

pub(crate) fn mac_mode() -> MacMode {
    match MODE.load(Ordering::Relaxed) {
        1 => MacMode::Icons,
        2 => MacMode::IconsAndNames,
        _ => MacMode::Off,
    }
}

fn set_mac_mode(mode: MacMode) {
    let n = match mode {
        MacMode::Off => 0,
        MacMode::Icons => 1,
        MacMode::IconsAndNames => 2,
    };
    MODE.store(n, Ordering::Relaxed);
    store::write_text(MODE_KEY, &n.to_string());
}

fn load_mac_mode() {
    if MODE_LOADED.swap(true, Ordering::Relaxed) {
        return;
    }
    let n = match store::read_text(MODE_KEY).as_deref().map(str::trim) {
        Some("1") => 1,
        Some("2") => 2,
        Some(_) => 0,
        None => u8::from(store::read(OLD_ICONS_KEY) == Some(true)),
    };
    MODE.store(n, Ordering::Relaxed);
}

/// English, or Portuguese when the launcher is in Portuguese (the fork's strings are not in
/// upstream's catalogs).
fn say(en: &'static str, pt: &'static str) -> &'static str {
    if crate::i18n::current().code().starts_with("pt") { pt } else { en }
}

/// The fork's toolbar control, drawn after upstream's refresh button. Also keeps the installed
/// apps in step with the choice.
pub(crate) fn toolbar(launcher: &LauncherUi, ui: &mut Ui) {
    load_mac_mode();
    let changed = mac_mode_picker(ui);
    machine_icons::sync(launcher, changed);
    if machine_icons::status().busy {
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(250));
    }
}

/// "Mac icons  [ Off | Icons | + Names ]": what the installed apps look like in Finder,
/// Launchpad and the Dock. The caption shows "…" while the apps are being changed, and its
/// tooltip how many are done. Returns true when the choice changed.
fn mac_mode_picker(ui: &mut Ui) -> bool {
    let t = crate::theme::Tokens::get(ui.ctx());
    let status = machine_icons::status();
    let caption = if status.busy { say("Mac icons …", "Ícones Mac …") } else { say("Mac icons", "Ícones Mac") };
    let mut tip =
        say("How the installed apps look in Finder, Launchpad and the Dock.", "Como os apps instalados aparecem no Finder, no Launchpad e no Dock.")
            .to_string();
    if status.total > 0 {
        tip.push_str(&format!(
            "\n{} {}/{} · {} {}/{}",
            say("Original icons:", "Ícones originais:"),
            status.icons,
            status.total,
            say("names:", "nomes:"),
            status.names,
            status.total
        ));
    }
    if let Some(error) = &status.error {
        tip.push_str(&format!("\n{} {error}", say("Last error:", "Último erro:")));
    }
    widgets::hud(ui, caption, t.muted).on_hover_text(&tip);
    let mut mode = mac_mode();
    let cell = |label, tip| widgets::Cell { label, tip, count: None };
    let options = [
        (MacMode::Off, cell(say("Off", "Off"), say("Craft icons and names", "Ícones e nomes Craft"))),
        (MacMode::Icons, cell(say("Icons", "Ícones"), say("Original programs' icons", "Ícones dos programas originais"))),
        (
            MacMode::IconsAndNames,
            cell(say("+ Names", "+ Nomes"), say("Original programs' icons and names", "Ícones e nomes dos programas originais")),
        ),
    ];
    let changed = widgets::cells(ui, &mut mode, &options);
    if changed {
        set_mac_mode(mode);
        ui.ctx().request_repaint();
    }
    changed
}
