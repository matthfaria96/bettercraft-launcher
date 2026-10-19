//! BetterCraft: a toolbar button that swaps every Craft App's icon for the icon of the program it
//! is a clone of. Kept in this one file (plus a one-line hook in `icons::app`) so the fork merges
//! cleanly with upstream `storytold/craft-launcher`.
//!
//! The choice is saved in egui's persisted memory, so no upstream settings struct changes.

use std::sync::atomic::{AtomicBool, Ordering};

use egui::{ImageSource, Ui};

use crate::widgets::{self, Kind};

static ENABLED: AtomicBool = AtomicBool::new(false);
static LOADED: AtomicBool = AtomicBool::new(false);

fn memory_key() -> egui::Id {
    egui::Id::new("bettercraft-original-icons")
}

/// Whether the original icons are shown instead of the Craft ones.
pub fn enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

/// The original program's icon for a Craft App id.
pub fn app(id: &str) -> Option<ImageSource<'static>> {
    Some(match id {
        "photocraft" => egui::include_image!("../../../assets/better-icons/photocraft.png"),
        "vectorcraft" => egui::include_image!("../../../assets/better-icons/vectorcraft.png"),
        "filmcraft" => egui::include_image!("../../../assets/better-icons/filmcraft.png"),
        "lightcraft" => egui::include_image!("../../../assets/better-icons/lightcraft.png"),
        "pdfcraft" => egui::include_image!("../../../assets/better-icons/pdfcraft.png"),
        "effectcraft" => egui::include_image!("../../../assets/better-icons/effectcraft.png"),
        "designcraft" => egui::include_image!("../../../assets/better-icons/designcraft.png"),
        "wordcraft" => egui::include_image!("../../../assets/better-icons/wordcraft.png"),
        "deckcraft" => egui::include_image!("../../../assets/better-icons/deckcraft.png"),
        "gridcraft" => egui::include_image!("../../../assets/better-icons/gridcraft.png"),
        "cadcraft" => egui::include_image!("../../../assets/better-icons/cadcraft.png"),
        "soundcraft" => egui::include_image!("../../../assets/better-icons/soundcraft.png"),
        "figcraft" => egui::include_image!("../../../assets/better-icons/figcraft.png"),
        _ => return None,
    })
}

/// Reads the saved choice the first time the toolbar is drawn.
fn load_once(ctx: &egui::Context) {
    if LOADED.swap(true, Ordering::Relaxed) {
        return;
    }
    let saved = ctx.data_mut(|d| d.get_persisted::<bool>(memory_key())).unwrap_or(false);
    ENABLED.store(saved, Ordering::Relaxed);
}

/// The toolbar toggle: lit while the original icons are on.
pub fn toggle_button(ui: &mut Ui) {
    load_once(ui.ctx());
    let on = enabled();
    let kind = if on { Kind::Primary } else { Kind::Secondary };
    let clicked = widgets::button(ui, "Original Icons", kind, 0.0).on_hover_text("Swap the Craft icons for the original programs' icons").clicked();
    if clicked {
        ENABLED.store(!on, Ordering::Relaxed);
        ui.ctx().data_mut(|d| d.insert_persisted(memory_key(), !on));
        ui.ctx().request_repaint();
    }
}

#[cfg(test)]
mod tests {
    use super::app;

    #[test]
    fn every_craft_app_has_an_original_icon() {
        for id in [
            "photocraft",
            "vectorcraft",
            "filmcraft",
            "lightcraft",
            "pdfcraft",
            "effectcraft",
            "designcraft",
            "wordcraft",
            "deckcraft",
            "gridcraft",
            "cadcraft",
            "soundcraft",
            "figcraft",
        ] {
            assert!(app(id).is_some(), "{id}");
        }
        assert!(app("nope").is_none());
    }
}
