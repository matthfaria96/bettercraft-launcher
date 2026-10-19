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
use assets::IconSet;

pub(crate) mod assets;
pub(crate) mod compact;
#[cfg(target_os = "macos")]
mod dock_icon;
pub(crate) mod icons;
pub(crate) mod launcher_icon;
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

/// Whether the installed apps get the launcher's icons on this Mac: as they come (Clone), or with
/// the icons of the chosen [`IconSet`] (and, for the study set, its names).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MacMode {
    Clone,
    Custom,
}

impl MacMode {
    pub(crate) fn icons(self) -> bool {
        self == Self::Custom
    }
}

const MODE_KEY: &str = "bettercraft-mac-mode";
/// The on/off file of the version with icons only; read when there is no mode yet.
const OLD_ICONS_KEY: &str = "bettercraft-original-icons";
const SET_KEY: &str = "bettercraft-icon-set";
static CUSTOM: AtomicBool = AtomicBool::new(false);
static STUDY: AtomicBool = AtomicBool::new(false);
static LOADED: AtomicBool = AtomicBool::new(false);
/// Set when the icon set changes in Settings: the next toolbar pass re-dresses the apps at once.
static RESET: AtomicBool = AtomicBool::new(false);

pub(crate) fn mac_mode() -> MacMode {
    if CUSTOM.load(Ordering::Relaxed) { MacMode::Custom } else { MacMode::Clone }
}

pub(crate) fn icon_set() -> IconSet {
    if STUDY.load(Ordering::Relaxed) { IconSet::Study } else { IconSet::Craft }
}

fn load_choices() {
    if LOADED.swap(true, Ordering::Relaxed) {
        return;
    }
    let custom = match store::read_text(MODE_KEY).as_deref().map(str::trim) {
        Some(n) => n == "1" || n == "2",
        None => store::read(OLD_ICONS_KEY) == Some(true),
    };
    CUSTOM.store(custom, Ordering::Relaxed);
    let set = store::read_text(SET_KEY).as_deref().and_then(IconSet::from_key).unwrap_or_default();
    STUDY.store(set == IconSet::Study, Ordering::Relaxed);
}

fn set_mac_mode(mode: MacMode) {
    CUSTOM.store(mode == MacMode::Custom, Ordering::Relaxed);
    store::write_text(MODE_KEY, if mode == MacMode::Custom { "2" } else { "0" });
}

fn set_icon_set(set: IconSet) {
    STUDY.store(set == IconSet::Study, Ordering::Relaxed);
    store::write_text(SET_KEY, set.key());
    RESET.store(true, Ordering::Relaxed);
}

/// English, or Portuguese when the launcher is in Portuguese (the fork's strings are not in
/// upstream's catalogs).
fn say(en: &'static str, pt: &'static str) -> &'static str {
    if crate::i18n::current().code().starts_with("pt") { pt } else { en }
}

/// The label of the "custom" choice and badge: NEW for the official set, STUDY for the other.
pub(crate) fn custom_label() -> &'static str {
    match icon_set() {
        IconSet::Craft => say("New", "Novo"),
        IconSet::Study => say("Study", "Estudo"),
    }
}

/// The fork's toolbar control, drawn after upstream's refresh button. Also keeps the installed
/// apps in step with the choice.
pub(crate) fn toolbar(launcher: &LauncherUi, ui: &mut Ui) {
    load_choices();
    let changed = mac_mode_picker(ui) | RESET.swap(false, Ordering::Relaxed);
    machine_icons::sync(launcher, changed);
    launcher_icon::sync_dock(ui.ctx(), icon_set());
    if machine_icons::status().busy {
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(250));
    }
}

/// "Mac icons  [ Clone | New ]": whether the installed apps carry the launcher's icons in
/// Finder, Launchpad and the Dock. "…" while the apps are being changed; the tooltip says how
/// many are done. Returns true when the choice changed.
fn mac_mode_picker(ui: &mut Ui) -> bool {
    let t = crate::theme::Tokens::get(ui.ctx());
    let status = machine_icons::status();
    let caption = if status.busy { say("Mac icons …", "Ícones no Mac …") } else { say("Mac icons", "Ícones no Mac") };
    let mut tip = match icon_set() {
        IconSet::Craft => say(
            "Clone: the apps as they come. New: the BetterCraft icons in Finder, Launchpad and the Dock (names stay).",
            "Clone: os apps como vêm. Novo: os ícones BetterCraft no Finder, no Launchpad e no Dock (os nomes ficam).",
        ),
        IconSet::Study => say(
            "Study mode (unofficial, set in Settings): the original programs' icons with the CRAFT ribbon, and their names.",
            "Modo estudo (não oficial, nas Configurações): ícones dos programas originais com a fita CRAFT, e os nomes deles.",
        ),
    }
    .to_string();
    if status.total > 0 {
        tip.push_str(&format!("\n{} {}/{}", custom_label(), status.icons, status.total));
    }
    if let Some(error) = &status.error {
        tip.push_str(&format!("\n{} {error}", say("Last error:", "Último erro:")));
    }
    widgets::hud(ui, caption, t.muted).on_hover_text(&tip);
    let mut mode = mac_mode();
    let cell = |label, tip| widgets::Cell { label, tip, count: None };
    let options = [
        (MacMode::Clone, cell(say("Clone", "Clone"), say("The apps' own icons", "Os ícones dos próprios apps"))),
        (MacMode::Custom, cell(custom_label(), &tip)),
    ];
    let changed = widgets::cells(ui, &mut mode, &options);
    if changed {
        set_mac_mode(mode);
        ui.ctx().request_repaint();
    }
    changed
}

/// The Settings header's picture: the launcher's logo for the current set.
pub(crate) fn settings_logo() -> egui::ImageSource<'static> {
    load_choices();
    launcher_icon::logo(icon_set())
}

/// The Settings panel: the official Normal set or the unofficial Study set, as two cards showing
/// their icons; the chosen one is outlined in the accent colour and ticked.
pub(crate) fn settings_panel(ui: &mut Ui) {
    load_choices();
    crate::sheet::panel(ui, say("Icons on this Mac", "Ícones no Mac"), |ui| {
        let current = icon_set();
        let gap = 10.0;
        let width = ((ui.available_width() - gap) / 2.0).max(140.0);
        let cards = [
            (
                IconSet::Craft,
                say("Normal", "Normal"),
                say("Official", "Oficial"),
                say("BetterCraft icons. The apps keep their names.", "Ícones BetterCraft. Os apps mantêm seus nomes."),
            ),
            (
                IconSet::Study,
                say("Study", "Estudo"),
                say("Unofficial · test", "Não oficial · teste"),
                say(
                    "The original programs' icons with the CRAFT ribbon, and their names.",
                    "Ícones dos programas originais com a fita CRAFT, e os nomes deles.",
                ),
            ),
        ];
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = gap;
            for (set, title, tag, text) in cards {
                if set_card(ui, set, current == set, title, tag, text, width) && current != set {
                    set_icon_set(set);
                }
            }
        });
        let t = crate::theme::Tokens::get(ui.ctx());
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(say(
                "Applied to the installed apps while \"Mac icons\" is on New / Study.",
                "Aplicado aos apps instalados quando \"Ícones no Mac\" está em Novo / Estudo.",
            ))
            .size(12.0)
            .color(t.muted),
        );
    });
}

/// One card of [`settings_panel`]. Returns true when clicked.
fn set_card(ui: &mut Ui, set: IconSet, chosen: bool, title: &str, tag: &str, text: &str, width: f32) -> bool {
    use egui::{CornerRadius, FontId, Rect, Sense, Stroke, StrokeKind, pos2, vec2};
    let t = crate::theme::Tokens::get(ui.ctx());
    let (rect, resp) = ui.allocate_exact_size(vec2(width, 196.0), Sense::click());
    let hovered = resp.hovered();
    let fill = if chosen {
        t.accent.gamma_multiply(0.12)
    } else if hovered {
        t.sunken
    } else {
        t.bg
    };
    let stroke = if chosen { Stroke::new(2.0, t.accent) } else { t.hairline() };
    let p = ui.painter();
    p.rect(rect, CornerRadius::ZERO, fill, stroke, StrokeKind::Inside);
    // Three of the set's icons, side by side.
    let icon = 44.0;
    for (i, id) in ["photocraft", "gridcraft", "wordcraft"].iter().enumerate() {
        if let Some(src) = assets::image(set, id) {
            let at = Rect::from_min_size(pos2(rect.left() + 14.0 + i as f32 * (icon + 6.0), rect.top() + 14.0), vec2(icon, icon));
            egui::Image::new(src).paint_at(ui, at);
        }
    }
    // A tick in a filled circle on the chosen card, an empty ring on the other.
    let c = pos2(rect.right() - 20.0, rect.top() + 20.0);
    if chosen {
        p.circle_filled(c, 10.0, t.accent);
        p.text(c, egui::Align2::CENTER_CENTER, "✓", FontId::proportional(13.0), t.accent_ink);
    } else {
        p.circle_stroke(c, 10.0, Stroke::new(1.5, t.muted));
    }
    let left = rect.left() + 14.0;
    let mut y = rect.top() + 14.0 + icon + 12.0;
    let title_g = p.layout_no_wrap(title.to_string(), FontId::new(17.0, crate::theme::semibold()), t.ink_strong);
    let title_h = title_g.size().y;
    p.galley(pos2(left, y), title_g, t.ink_strong);
    let tag_color = if set == IconSet::Study { t.warn } else { t.good };
    p.text(pos2(left, y + title_h + 2.0), egui::Align2::LEFT_TOP, tag.to_uppercase(), FontId::new(10.0, crate::theme::mono()), tag_color);
    y += title_h + 18.0;
    let text_g = p.layout(text.to_string(), FontId::proportional(12.0), t.muted, rect.width() - 28.0);
    p.galley(pos2(left, y), text_g, t.muted);
    resp.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::RadioButton, true, chosen, title));
    resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked()
}
