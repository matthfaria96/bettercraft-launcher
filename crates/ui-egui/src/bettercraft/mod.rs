//! BetterCraft: everything this fork adds to the upstream `storytold/craft-launcher`.
//!
//! - `icons`: the original programs' icons, shown beside each Craft icon in the compact rows.
//! - `machine_icons`: those icons on the installed `.app` bundles (macOS Finder, Dock, Launchpad).
//! - `compact`: a denser, quieter grid of apps.
//!
//! Upstream files only carry one-line hooks into this module (`lib.rs`, `views.rs`), so the fork
//! keeps merging cleanly with upstream. Choices are written to a small file the moment they change
//! (egui's memory only saves every 30 s or on a clean quit), so the upstream `Settings` struct is
//! never touched.

use std::sync::atomic::{AtomicBool, Ordering};

use egui::Ui;

use crate::LauncherUi;
use crate::widgets::{self, Kind};

pub(crate) mod compact;
pub(crate) mod icons;
mod machine_icons;

/// An on/off choice that survives restarts.
pub(crate) struct Flag {
    key: &'static str,
    value: AtomicBool,
    loaded: AtomicBool,
    default: bool,
}

impl Flag {
    const fn new(key: &'static str, default: bool) -> Self {
        Self { key, value: AtomicBool::new(default), loaded: AtomicBool::new(false), default }
    }

    pub(crate) fn get(&self) -> bool {
        self.value.load(Ordering::Relaxed)
    }

    fn load_once(&self, ctx: &egui::Context) {
        if self.loaded.swap(true, Ordering::Relaxed) {
            return;
        }
        let saved = store::read(self.key).or_else(|| ctx.data_mut(|d| d.get_persisted::<bool>(egui::Id::new(self.key))));
        self.value.store(saved.unwrap_or(self.default), Ordering::Relaxed);
    }

    fn set(&self, ctx: &egui::Context, on: bool) {
        self.value.store(on, Ordering::Relaxed);
        ctx.data_mut(|d| d.insert_persisted(egui::Id::new(self.key), on));
        store::write(self.key, on);
        ctx.request_repaint();
    }
}

/// Saved choices as one file per key, `1` or `0`, in the user's data folder (desktop only).
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
    pub(super) fn write(key: &str, on: bool) {
        let Some(path) = path(key) else { return };
        let saved = path.parent().map_or(Ok(()), std::fs::create_dir_all).and_then(|()| std::fs::write(&path, if on { "1" } else { "0" }));
        if let Err(e) = saved {
            log::warn!("BetterCraft: couldn't save {key}: {e}");
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub(super) fn read(_key: &str) -> Option<bool> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    pub(super) fn write(_key: &str, _on: bool) {}
}

pub(crate) static ORIGINAL_ICONS: Flag = Flag::new("bettercraft-original-icons", false);
pub(crate) static COMPACT: Flag = Flag::new("bettercraft-compact", !cfg!(feature = "upstream-look"));

/// English, or Portuguese when the launcher is in Portuguese (the fork's strings are not in
/// upstream's catalogs).
fn say(en: &'static str, pt: &'static str) -> &'static str {
    if crate::i18n::current().code().starts_with("pt") { pt } else { en }
}

/// The fork's toolbar switch, drawn after upstream's refresh button. Also keeps the installed
/// apps' icons in step with the choice.
pub(crate) fn toolbar(launcher: &LauncherUi, ui: &mut Ui) {
    ORIGINAL_ICONS.load_once(ui.ctx());
    let toggled = mac_icons_switch(ui);
    machine_icons::sync(launcher, toggled);
    if machine_icons::status().busy {
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(250));
    }
}

/// The switch for the installed apps' icons: says on or off, shows progress while it works, and
/// how many apps carry the original icons. Returns true when clicked.
fn mac_icons_switch(ui: &mut Ui) -> bool {
    let on = ORIGINAL_ICONS.get();
    let status = machine_icons::status();
    let state = match (status.busy, on) {
        (true, _) => "…",
        (false, true) => "ON",
        (false, false) => "OFF",
    };
    let label = format!("{} ● {state}", say("Mac icons", "Ícones Mac"));
    let mut tip = if on {
        say(
            "On: the installed apps use the original programs' icons. Click to restore the Craft icons.",
            "Ligado: os apps instalados usam os ícones dos programas originais. Clique para voltar aos ícones Craft.",
        )
        .to_string()
    } else {
        say(
            "Off: the installed apps use the Craft icons. Click to give them the original programs' icons.",
            "Desligado: os apps instalados usam os ícones Craft. Clique para trocar pelos ícones dos programas originais.",
        )
        .to_string()
    };
    if status.total > 0 {
        tip.push_str(&format!("\n{} {}/{}", say("Original icons on:", "Com ícone original:"), status.swapped, status.total));
    }
    if let Some(error) = &status.error {
        tip.push_str(&format!("\n{} {error}", say("Last error:", "Último erro:")));
    }
    let clicked = widgets::button(ui, &label, if on { Kind::Primary } else { Kind::Secondary }, 0.0).on_hover_text(tip).clicked();
    if clicked {
        ORIGINAL_ICONS.set(ui.ctx(), !on);
    }
    clicked
}
