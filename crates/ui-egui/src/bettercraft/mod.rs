//! BetterCraft: everything this fork adds to the upstream `storytold/craft-launcher`.
//!
//! - `icons`: the original programs' icons, shown beside each Craft icon in the compact rows.
//! - `machine_icons`: those icons, and optionally names, on the installed `.app` bundles (macOS),
//!   with `dock_icon` for the icon the Dock shows while an app starts and runs.
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
use assets::IconStyle;

pub(crate) mod assets;
pub(crate) mod compact;
#[cfg(target_os = "macos")]
mod dock_icon;
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

/// The BetterCraft build: everywhere but upstream's own UI tests, which check upstream's look and
/// behaviour. Turns on the compact rows and turns off the noise (app pages, announcements, social
/// links, grid/table switch) and upstream's launcher self-update.
pub(crate) fn fork() -> bool {
    !cfg!(feature = "upstream-look")
}

/// What the installed apps look like on this Mac: as the Craft clones they are, or as the
/// original programs (icon and name together).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MacMode {
    Clone,
    Original,
}

impl MacMode {
    pub(crate) fn icons(self) -> bool {
        self == Self::Original
    }

    pub(crate) fn names(self) -> bool {
        self == Self::Original
    }
}

const MODE_KEY: &str = "bettercraft-mac-mode";
/// The on/off file of the version with icons only; read when there is no mode yet.
const OLD_ICONS_KEY: &str = "bettercraft-original-icons";
const STYLE_KEY: &str = "bettercraft-icon-style";
static ORIGINAL: AtomicBool = AtomicBool::new(false);
static CRAFT_BADGE: AtomicBool = AtomicBool::new(false);
static LOADED: AtomicBool = AtomicBool::new(false);
/// Set when the style changes in Settings: the next toolbar pass re-dresses the apps at once.
static RESTYLED: AtomicBool = AtomicBool::new(false);

pub(crate) fn mac_mode() -> MacMode {
    if ORIGINAL.load(Ordering::Relaxed) { MacMode::Original } else { MacMode::Clone }
}

pub(crate) fn icon_style() -> IconStyle {
    if CRAFT_BADGE.load(Ordering::Relaxed) { IconStyle::Craft } else { IconStyle::Plain }
}

fn load_choices() {
    if LOADED.swap(true, Ordering::Relaxed) {
        return;
    }
    // "1" (icons) and "2" (icons and names) were the modes before Original covered both.
    let original = match store::read_text(MODE_KEY).as_deref().map(str::trim) {
        Some(n) => n == "1" || n == "2",
        None => store::read(OLD_ICONS_KEY) == Some(true),
    };
    ORIGINAL.store(original, Ordering::Relaxed);
    let style = store::read_text(STYLE_KEY).as_deref().and_then(IconStyle::from_key).unwrap_or_default();
    CRAFT_BADGE.store(style == IconStyle::Craft, Ordering::Relaxed);
}

fn set_mac_mode(mode: MacMode) {
    ORIGINAL.store(mode == MacMode::Original, Ordering::Relaxed);
    store::write_text(MODE_KEY, if mode == MacMode::Original { "2" } else { "0" });
}

fn set_icon_style(style: IconStyle) {
    CRAFT_BADGE.store(style == IconStyle::Craft, Ordering::Relaxed);
    store::write_text(STYLE_KEY, style.key());
    RESTYLED.store(true, Ordering::Relaxed);
}

/// English, or Portuguese when the launcher is in Portuguese (the fork's strings are not in
/// upstream's catalogs).
fn say(en: &'static str, pt: &'static str) -> &'static str {
    if crate::i18n::current().code().starts_with("pt") { pt } else { en }
}

/// The fork's toolbar control, drawn after upstream's refresh button. Also keeps the installed
/// apps in step with the choice.
pub(crate) fn toolbar(launcher: &LauncherUi, ui: &mut Ui) {
    load_choices();
    let changed = mac_mode_picker(ui) | RESTYLED.swap(false, Ordering::Relaxed);
    machine_icons::sync(launcher, changed);
    if machine_icons::status().busy {
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(250));
    }
}

/// "On this Mac  [ Clone | Original ]": what the installed apps look like in Finder, Launchpad
/// and the Dock. The caption shows "…" while the apps are being changed, and its tooltip how many
/// are done. Returns true when the choice changed.
fn mac_mode_picker(ui: &mut Ui) -> bool {
    let t = crate::theme::Tokens::get(ui.ctx());
    let status = machine_icons::status();
    let caption = if status.busy { say("On this Mac …", "No Mac …") } else { say("On this Mac", "No Mac") };
    let mut tip = say(
        "How the installed apps look in Finder, Launchpad and the Dock: as the Craft clones, or with the original programs' icons and names.",
        "Como os apps instalados aparecem no Finder, no Launchpad e no Dock: como os clones Craft, ou com o ícone e o nome dos programas originais.",
    )
    .to_string();
    if status.total > 0 {
        tip.push_str(&format!("\n{} {}/{}", say("Original:", "Original:"), status.icons.min(status.names), status.total));
    }
    if let Some(error) = &status.error {
        tip.push_str(&format!("\n{} {error}", say("Last error:", "Último erro:")));
    }
    widgets::hud(ui, caption, t.muted).on_hover_text(&tip);
    let mut mode = mac_mode();
    let cell = |label, tip| widgets::Cell { label, tip, count: None };
    let options = [
        (MacMode::Clone, cell(say("Clone", "Clone"), say("Craft icons and names", "Ícones e nomes Craft"))),
        (MacMode::Original, cell(say("Original", "Original"), say("The original programs' icons and names", "Ícone e nome dos programas originais"))),
    ];
    let changed = widgets::cells(ui, &mut mode, &options);
    if changed {
        set_mac_mode(mode);
        ui.ctx().request_repaint();
    }
    changed
}

/// The Settings panel for the original icons' style: plain, or with the yellow "Craft" badge.
pub(crate) fn settings_panel(ui: &mut Ui) {
    load_choices();
    crate::sheet::panel(ui, say("Original icons", "Ícones originais"), |ui| {
        let mut style = icon_style();
        let cell = |label, tip| widgets::Cell { label, tip, count: None };
        let options = [
            (
                IconStyle::Plain,
                cell(say("Plain", "Simples"), say("The original programs' icons as they are", "Os ícones dos programas originais como são")),
            ),
            (
                IconStyle::Craft,
                cell(say("Craft badge", "Selo Craft"), say("With a yellow \"Craft\" badge on the side", "Com um selo amarelo \"Craft\" na lateral")),
            ),
        ];
        ui.horizontal(|ui| {
            if widgets::cells(ui, &mut style, &options) {
                set_icon_style(style);
            }
        });
        if let Some(preview) = icons::app("photocraft") {
            ui.add_space(6.0);
            ui.add(egui::Image::new(preview).fit_to_exact_size(egui::vec2(56.0, 56.0)));
        }
        let t = crate::theme::Tokens::get(ui.ctx());
        ui.label(
            egui::RichText::new(say(
                "Used in this list and, in Original mode, on the installed apps.",
                "Usado nesta lista e, no modo Original, nos apps instalados.",
            ))
            .size(12.0)
            .color(t.muted),
        );
    });
}
