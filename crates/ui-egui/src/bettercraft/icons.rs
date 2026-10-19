//! The original programs' names, and their icons in the chosen style, for the Craft Apps: the
//! icons are drawn beside each Craft icon in the rows; `machine_icons` puts both on installed apps.

use egui::ImageSource;

/// The original program's name for a Craft App id, as Finder, Launchpad and the Dock show it.
pub fn name(id: &str) -> Option<&'static str> {
    Some(match id {
        "photocraft" => "Adobe Photoshop",
        "vectorcraft" => "Adobe Illustrator",
        "filmcraft" => "Adobe Premiere Pro",
        "lightcraft" => "Adobe Lightroom",
        "pdfcraft" => "Adobe Acrobat",
        "effectcraft" => "Adobe After Effects",
        "designcraft" => "Adobe InDesign",
        "wordcraft" => "Microsoft Word",
        "deckcraft" => "Microsoft PowerPoint",
        "gridcraft" => "Microsoft Excel",
        "cadcraft" => "AutoCAD",
        "soundcraft" => "Adobe Audition",
        "figcraft" => "Figma",
        _ => return None,
    })
}

/// The original program's icon for a Craft App id, in the style chosen in Settings.
pub fn app(id: &str) -> Option<ImageSource<'static>> {
    super::assets::image(super::icon_set(), id)
}

#[cfg(test)]
mod tests {
    use super::name;

    #[test]
    fn every_craft_app_has_an_original_name() {
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
            assert!(name(id).is_some(), "{id}");
        }
        assert!(name("nope").is_none());
    }
}
