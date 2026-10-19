//! The original programs' icons for the Craft Apps, shown in the launcher while
//! [`super::ORIGINAL_ICONS`] is on.

use egui::ImageSource;

/// Whether the original icons are shown instead of the Craft ones.
pub fn enabled() -> bool {
    super::ORIGINAL_ICONS.get()
}

/// The original program's icon for a Craft App id.
pub fn app(id: &str) -> Option<ImageSource<'static>> {
    Some(match id {
        "photocraft" => egui::include_image!("../../../../assets/better-icons/photocraft.png"),
        "vectorcraft" => egui::include_image!("../../../../assets/better-icons/vectorcraft.png"),
        "filmcraft" => egui::include_image!("../../../../assets/better-icons/filmcraft.png"),
        "lightcraft" => egui::include_image!("../../../../assets/better-icons/lightcraft.png"),
        "pdfcraft" => egui::include_image!("../../../../assets/better-icons/pdfcraft.png"),
        "effectcraft" => egui::include_image!("../../../../assets/better-icons/effectcraft.png"),
        "designcraft" => egui::include_image!("../../../../assets/better-icons/designcraft.png"),
        "wordcraft" => egui::include_image!("../../../../assets/better-icons/wordcraft.png"),
        "deckcraft" => egui::include_image!("../../../../assets/better-icons/deckcraft.png"),
        "gridcraft" => egui::include_image!("../../../../assets/better-icons/gridcraft.png"),
        "cadcraft" => egui::include_image!("../../../../assets/better-icons/cadcraft.png"),
        "soundcraft" => egui::include_image!("../../../../assets/better-icons/soundcraft.png"),
        "figcraft" => egui::include_image!("../../../../assets/better-icons/figcraft.png"),
        _ => return None,
    })
}

/// The original program's `.icns` for a Craft App id, for the installed app bundles.
#[cfg(target_os = "macos")]
pub fn icns(id: &str) -> Option<&'static [u8]> {
    Some(match id {
        "photocraft" => include_bytes!("../../../../assets/better-icons/icns/photocraft.icns"),
        "vectorcraft" => include_bytes!("../../../../assets/better-icons/icns/vectorcraft.icns"),
        "filmcraft" => include_bytes!("../../../../assets/better-icons/icns/filmcraft.icns"),
        "lightcraft" => include_bytes!("../../../../assets/better-icons/icns/lightcraft.icns"),
        "pdfcraft" => include_bytes!("../../../../assets/better-icons/icns/pdfcraft.icns"),
        "effectcraft" => include_bytes!("../../../../assets/better-icons/icns/effectcraft.icns"),
        "designcraft" => include_bytes!("../../../../assets/better-icons/icns/designcraft.icns"),
        "wordcraft" => include_bytes!("../../../../assets/better-icons/icns/wordcraft.icns"),
        "deckcraft" => include_bytes!("../../../../assets/better-icons/icns/deckcraft.icns"),
        "gridcraft" => include_bytes!("../../../../assets/better-icons/icns/gridcraft.icns"),
        "cadcraft" => include_bytes!("../../../../assets/better-icons/icns/cadcraft.icns"),
        "soundcraft" => include_bytes!("../../../../assets/better-icons/icns/soundcraft.icns"),
        "figcraft" => include_bytes!("../../../../assets/better-icons/icns/figcraft.icns"),
        _ => return None,
    })
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
