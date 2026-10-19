//! The two icon sets the launcher can give the installed apps:
//!
//! - [`IconSet::Craft`], the official look: the BetterCraft icons for each Craft App
//!   (`assets/craft-icons/`, from `scripts/make-craft-icons.sh`); the apps keep their names.
//! - [`IconSet::Study`], unofficial, for study and testing: the original programs' icons with the
//!   CRAFT corner ribbon (`assets/study-icons/`, from `scripts/make-study-icons.sh`), and their names.
//!
//! Each comes as the `.icns` for the bundle, a 256 px PNG (the launcher's rows and roomy embedded
//! icons), a PNG that fits eframe's default icon (the Dock), and 96 / 64 px PNGs.

/// Bumped whenever the pictures change: installed apps carrying an older revision are re-dressed.
pub(crate) const REVISION: u32 = 3;

/// Which icons (and names) the installed apps get.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum IconSet {
    /// The official BetterCraft icons; names stay.
    #[default]
    Craft,
    /// The original programs' icons with the CRAFT ribbon, and their names.
    Study,
}

impl IconSet {
    #[cfg(test)]
    pub(crate) const ALL: [Self; 2] = [Self::Craft, Self::Study];

    pub(crate) fn key(self) -> &'static str {
        match self {
            Self::Craft => "normal",
            Self::Study => "study",
        }
    }

    /// Also reads the markers of earlier versions, which only had the original programs' icons
    /// ("plain", or "craft" for the ribboned ones).
    pub(crate) fn from_key(key: &str) -> Option<Self> {
        match key.trim() {
            "normal" => Some(Self::Craft),
            "study" | "craft" | "plain" => Some(Self::Study),
            _ => None,
        }
    }

    /// Whether this set also gives the apps the original programs' names.
    pub(crate) fn names(self) -> bool {
        self == Self::Study
    }
}

/// The 256 px icon as an egui image (one copy of the bytes, shared with [`png256`]).
pub(crate) fn image(set: IconSet, id: &str) -> Option<egui::ImageSource<'static>> {
    let uri = match (set, id) {
        (IconSet::Craft, "photocraft") => "bytes://bettercraft/craft-icons/photocraft.png",
        (IconSet::Craft, "vectorcraft") => "bytes://bettercraft/craft-icons/vectorcraft.png",
        (IconSet::Craft, "filmcraft") => "bytes://bettercraft/craft-icons/filmcraft.png",
        (IconSet::Craft, "lightcraft") => "bytes://bettercraft/craft-icons/lightcraft.png",
        (IconSet::Craft, "pdfcraft") => "bytes://bettercraft/craft-icons/pdfcraft.png",
        (IconSet::Craft, "effectcraft") => "bytes://bettercraft/craft-icons/effectcraft.png",
        (IconSet::Craft, "designcraft") => "bytes://bettercraft/craft-icons/designcraft.png",
        (IconSet::Craft, "wordcraft") => "bytes://bettercraft/craft-icons/wordcraft.png",
        (IconSet::Craft, "deckcraft") => "bytes://bettercraft/craft-icons/deckcraft.png",
        (IconSet::Craft, "gridcraft") => "bytes://bettercraft/craft-icons/gridcraft.png",
        (IconSet::Craft, "cadcraft") => "bytes://bettercraft/craft-icons/cadcraft.png",
        (IconSet::Craft, "soundcraft") => "bytes://bettercraft/craft-icons/soundcraft.png",
        (IconSet::Craft, "figcraft") => "bytes://bettercraft/craft-icons/figcraft.png",
        (IconSet::Study, "photocraft") => "bytes://bettercraft/study-icons/photocraft.png",
        (IconSet::Study, "vectorcraft") => "bytes://bettercraft/study-icons/vectorcraft.png",
        (IconSet::Study, "filmcraft") => "bytes://bettercraft/study-icons/filmcraft.png",
        (IconSet::Study, "lightcraft") => "bytes://bettercraft/study-icons/lightcraft.png",
        (IconSet::Study, "pdfcraft") => "bytes://bettercraft/study-icons/pdfcraft.png",
        (IconSet::Study, "effectcraft") => "bytes://bettercraft/study-icons/effectcraft.png",
        (IconSet::Study, "designcraft") => "bytes://bettercraft/study-icons/designcraft.png",
        (IconSet::Study, "wordcraft") => "bytes://bettercraft/study-icons/wordcraft.png",
        (IconSet::Study, "deckcraft") => "bytes://bettercraft/study-icons/deckcraft.png",
        (IconSet::Study, "gridcraft") => "bytes://bettercraft/study-icons/gridcraft.png",
        (IconSet::Study, "cadcraft") => "bytes://bettercraft/study-icons/cadcraft.png",
        (IconSet::Study, "soundcraft") => "bytes://bettercraft/study-icons/soundcraft.png",
        (IconSet::Study, "figcraft") => "bytes://bettercraft/study-icons/figcraft.png",
        _ => return None,
    };
    Some(egui::ImageSource::Bytes { uri: uri.into(), bytes: egui::load::Bytes::Static(png256(set, id)?) })
}

/// The 256 px PNG.
pub(crate) fn png256(set: IconSet, id: &str) -> Option<&'static [u8]> {
    Some(match (set, id) {
        (IconSet::Craft, "photocraft") => include_bytes!("../../../../assets/craft-icons/photocraft.png"),
        (IconSet::Craft, "vectorcraft") => include_bytes!("../../../../assets/craft-icons/vectorcraft.png"),
        (IconSet::Craft, "filmcraft") => include_bytes!("../../../../assets/craft-icons/filmcraft.png"),
        (IconSet::Craft, "lightcraft") => include_bytes!("../../../../assets/craft-icons/lightcraft.png"),
        (IconSet::Craft, "pdfcraft") => include_bytes!("../../../../assets/craft-icons/pdfcraft.png"),
        (IconSet::Craft, "effectcraft") => include_bytes!("../../../../assets/craft-icons/effectcraft.png"),
        (IconSet::Craft, "designcraft") => include_bytes!("../../../../assets/craft-icons/designcraft.png"),
        (IconSet::Craft, "wordcraft") => include_bytes!("../../../../assets/craft-icons/wordcraft.png"),
        (IconSet::Craft, "deckcraft") => include_bytes!("../../../../assets/craft-icons/deckcraft.png"),
        (IconSet::Craft, "gridcraft") => include_bytes!("../../../../assets/craft-icons/gridcraft.png"),
        (IconSet::Craft, "cadcraft") => include_bytes!("../../../../assets/craft-icons/cadcraft.png"),
        (IconSet::Craft, "soundcraft") => include_bytes!("../../../../assets/craft-icons/soundcraft.png"),
        (IconSet::Craft, "figcraft") => include_bytes!("../../../../assets/craft-icons/figcraft.png"),
        (IconSet::Study, "photocraft") => include_bytes!("../../../../assets/study-icons/photocraft.png"),
        (IconSet::Study, "vectorcraft") => include_bytes!("../../../../assets/study-icons/vectorcraft.png"),
        (IconSet::Study, "filmcraft") => include_bytes!("../../../../assets/study-icons/filmcraft.png"),
        (IconSet::Study, "lightcraft") => include_bytes!("../../../../assets/study-icons/lightcraft.png"),
        (IconSet::Study, "pdfcraft") => include_bytes!("../../../../assets/study-icons/pdfcraft.png"),
        (IconSet::Study, "effectcraft") => include_bytes!("../../../../assets/study-icons/effectcraft.png"),
        (IconSet::Study, "designcraft") => include_bytes!("../../../../assets/study-icons/designcraft.png"),
        (IconSet::Study, "wordcraft") => include_bytes!("../../../../assets/study-icons/wordcraft.png"),
        (IconSet::Study, "deckcraft") => include_bytes!("../../../../assets/study-icons/deckcraft.png"),
        (IconSet::Study, "gridcraft") => include_bytes!("../../../../assets/study-icons/gridcraft.png"),
        (IconSet::Study, "cadcraft") => include_bytes!("../../../../assets/study-icons/cadcraft.png"),
        (IconSet::Study, "soundcraft") => include_bytes!("../../../../assets/study-icons/soundcraft.png"),
        (IconSet::Study, "figcraft") => include_bytes!("../../../../assets/study-icons/figcraft.png"),
        _ => return None,
    })
}

/// The `.icns` for the app bundle.
#[cfg(target_os = "macos")]
pub(crate) fn icns(set: IconSet, id: &str) -> Option<&'static [u8]> {
    Some(match (set, id) {
        (IconSet::Craft, "photocraft") => include_bytes!("../../../../assets/craft-icons/icns/photocraft.icns"),
        (IconSet::Craft, "vectorcraft") => include_bytes!("../../../../assets/craft-icons/icns/vectorcraft.icns"),
        (IconSet::Craft, "filmcraft") => include_bytes!("../../../../assets/craft-icons/icns/filmcraft.icns"),
        (IconSet::Craft, "lightcraft") => include_bytes!("../../../../assets/craft-icons/icns/lightcraft.icns"),
        (IconSet::Craft, "pdfcraft") => include_bytes!("../../../../assets/craft-icons/icns/pdfcraft.icns"),
        (IconSet::Craft, "effectcraft") => include_bytes!("../../../../assets/craft-icons/icns/effectcraft.icns"),
        (IconSet::Craft, "designcraft") => include_bytes!("../../../../assets/craft-icons/icns/designcraft.icns"),
        (IconSet::Craft, "wordcraft") => include_bytes!("../../../../assets/craft-icons/icns/wordcraft.icns"),
        (IconSet::Craft, "deckcraft") => include_bytes!("../../../../assets/craft-icons/icns/deckcraft.icns"),
        (IconSet::Craft, "gridcraft") => include_bytes!("../../../../assets/craft-icons/icns/gridcraft.icns"),
        (IconSet::Craft, "cadcraft") => include_bytes!("../../../../assets/craft-icons/icns/cadcraft.icns"),
        (IconSet::Craft, "soundcraft") => include_bytes!("../../../../assets/craft-icons/icns/soundcraft.icns"),
        (IconSet::Craft, "figcraft") => include_bytes!("../../../../assets/craft-icons/icns/figcraft.icns"),
        (IconSet::Study, "photocraft") => include_bytes!("../../../../assets/study-icons/icns/photocraft.icns"),
        (IconSet::Study, "vectorcraft") => include_bytes!("../../../../assets/study-icons/icns/vectorcraft.icns"),
        (IconSet::Study, "filmcraft") => include_bytes!("../../../../assets/study-icons/icns/filmcraft.icns"),
        (IconSet::Study, "lightcraft") => include_bytes!("../../../../assets/study-icons/icns/lightcraft.icns"),
        (IconSet::Study, "pdfcraft") => include_bytes!("../../../../assets/study-icons/icns/pdfcraft.icns"),
        (IconSet::Study, "effectcraft") => include_bytes!("../../../../assets/study-icons/icns/effectcraft.icns"),
        (IconSet::Study, "designcraft") => include_bytes!("../../../../assets/study-icons/icns/designcraft.icns"),
        (IconSet::Study, "wordcraft") => include_bytes!("../../../../assets/study-icons/icns/wordcraft.icns"),
        (IconSet::Study, "deckcraft") => include_bytes!("../../../../assets/study-icons/icns/deckcraft.icns"),
        (IconSet::Study, "gridcraft") => include_bytes!("../../../../assets/study-icons/icns/gridcraft.icns"),
        (IconSet::Study, "cadcraft") => include_bytes!("../../../../assets/study-icons/icns/cadcraft.icns"),
        (IconSet::Study, "soundcraft") => include_bytes!("../../../../assets/study-icons/icns/soundcraft.icns"),
        (IconSet::Study, "figcraft") => include_bytes!("../../../../assets/study-icons/icns/figcraft.icns"),
        _ => return None,
    })
}

/// A PNG no larger than eframe's default icon (12 052 bytes), for the Dock.
#[cfg(target_os = "macos")]
pub(crate) fn dock(set: IconSet, id: &str) -> Option<&'static [u8]> {
    Some(match (set, id) {
        (IconSet::Craft, "photocraft") => include_bytes!("../../../../assets/craft-icons/dock/photocraft.png"),
        (IconSet::Craft, "vectorcraft") => include_bytes!("../../../../assets/craft-icons/dock/vectorcraft.png"),
        (IconSet::Craft, "filmcraft") => include_bytes!("../../../../assets/craft-icons/dock/filmcraft.png"),
        (IconSet::Craft, "lightcraft") => include_bytes!("../../../../assets/craft-icons/dock/lightcraft.png"),
        (IconSet::Craft, "pdfcraft") => include_bytes!("../../../../assets/craft-icons/dock/pdfcraft.png"),
        (IconSet::Craft, "effectcraft") => include_bytes!("../../../../assets/craft-icons/dock/effectcraft.png"),
        (IconSet::Craft, "designcraft") => include_bytes!("../../../../assets/craft-icons/dock/designcraft.png"),
        (IconSet::Craft, "wordcraft") => include_bytes!("../../../../assets/craft-icons/dock/wordcraft.png"),
        (IconSet::Craft, "deckcraft") => include_bytes!("../../../../assets/craft-icons/dock/deckcraft.png"),
        (IconSet::Craft, "gridcraft") => include_bytes!("../../../../assets/craft-icons/dock/gridcraft.png"),
        (IconSet::Craft, "cadcraft") => include_bytes!("../../../../assets/craft-icons/dock/cadcraft.png"),
        (IconSet::Craft, "soundcraft") => include_bytes!("../../../../assets/craft-icons/dock/soundcraft.png"),
        (IconSet::Craft, "figcraft") => include_bytes!("../../../../assets/craft-icons/dock/figcraft.png"),
        (IconSet::Study, "photocraft") => include_bytes!("../../../../assets/study-icons/dock/photocraft.png"),
        (IconSet::Study, "vectorcraft") => include_bytes!("../../../../assets/study-icons/dock/vectorcraft.png"),
        (IconSet::Study, "filmcraft") => include_bytes!("../../../../assets/study-icons/dock/filmcraft.png"),
        (IconSet::Study, "lightcraft") => include_bytes!("../../../../assets/study-icons/dock/lightcraft.png"),
        (IconSet::Study, "pdfcraft") => include_bytes!("../../../../assets/study-icons/dock/pdfcraft.png"),
        (IconSet::Study, "effectcraft") => include_bytes!("../../../../assets/study-icons/dock/effectcraft.png"),
        (IconSet::Study, "designcraft") => include_bytes!("../../../../assets/study-icons/dock/designcraft.png"),
        (IconSet::Study, "wordcraft") => include_bytes!("../../../../assets/study-icons/dock/wordcraft.png"),
        (IconSet::Study, "deckcraft") => include_bytes!("../../../../assets/study-icons/dock/deckcraft.png"),
        (IconSet::Study, "gridcraft") => include_bytes!("../../../../assets/study-icons/dock/gridcraft.png"),
        (IconSet::Study, "cadcraft") => include_bytes!("../../../../assets/study-icons/dock/cadcraft.png"),
        (IconSet::Study, "soundcraft") => include_bytes!("../../../../assets/study-icons/dock/soundcraft.png"),
        (IconSet::Study, "figcraft") => include_bytes!("../../../../assets/study-icons/dock/figcraft.png"),
        _ => return None,
    })
}

/// The 96 px PNG.
#[cfg(target_os = "macos")]
pub(crate) fn png96(set: IconSet, id: &str) -> Option<&'static [u8]> {
    Some(match (set, id) {
        (IconSet::Craft, "photocraft") => include_bytes!("../../../../assets/craft-icons/small/photocraft-96.png"),
        (IconSet::Craft, "vectorcraft") => include_bytes!("../../../../assets/craft-icons/small/vectorcraft-96.png"),
        (IconSet::Craft, "filmcraft") => include_bytes!("../../../../assets/craft-icons/small/filmcraft-96.png"),
        (IconSet::Craft, "lightcraft") => include_bytes!("../../../../assets/craft-icons/small/lightcraft-96.png"),
        (IconSet::Craft, "pdfcraft") => include_bytes!("../../../../assets/craft-icons/small/pdfcraft-96.png"),
        (IconSet::Craft, "effectcraft") => include_bytes!("../../../../assets/craft-icons/small/effectcraft-96.png"),
        (IconSet::Craft, "designcraft") => include_bytes!("../../../../assets/craft-icons/small/designcraft-96.png"),
        (IconSet::Craft, "wordcraft") => include_bytes!("../../../../assets/craft-icons/small/wordcraft-96.png"),
        (IconSet::Craft, "deckcraft") => include_bytes!("../../../../assets/craft-icons/small/deckcraft-96.png"),
        (IconSet::Craft, "gridcraft") => include_bytes!("../../../../assets/craft-icons/small/gridcraft-96.png"),
        (IconSet::Craft, "cadcraft") => include_bytes!("../../../../assets/craft-icons/small/cadcraft-96.png"),
        (IconSet::Craft, "soundcraft") => include_bytes!("../../../../assets/craft-icons/small/soundcraft-96.png"),
        (IconSet::Craft, "figcraft") => include_bytes!("../../../../assets/craft-icons/small/figcraft-96.png"),
        (IconSet::Study, "photocraft") => include_bytes!("../../../../assets/study-icons/small/photocraft-96.png"),
        (IconSet::Study, "vectorcraft") => include_bytes!("../../../../assets/study-icons/small/vectorcraft-96.png"),
        (IconSet::Study, "filmcraft") => include_bytes!("../../../../assets/study-icons/small/filmcraft-96.png"),
        (IconSet::Study, "lightcraft") => include_bytes!("../../../../assets/study-icons/small/lightcraft-96.png"),
        (IconSet::Study, "pdfcraft") => include_bytes!("../../../../assets/study-icons/small/pdfcraft-96.png"),
        (IconSet::Study, "effectcraft") => include_bytes!("../../../../assets/study-icons/small/effectcraft-96.png"),
        (IconSet::Study, "designcraft") => include_bytes!("../../../../assets/study-icons/small/designcraft-96.png"),
        (IconSet::Study, "wordcraft") => include_bytes!("../../../../assets/study-icons/small/wordcraft-96.png"),
        (IconSet::Study, "deckcraft") => include_bytes!("../../../../assets/study-icons/small/deckcraft-96.png"),
        (IconSet::Study, "gridcraft") => include_bytes!("../../../../assets/study-icons/small/gridcraft-96.png"),
        (IconSet::Study, "cadcraft") => include_bytes!("../../../../assets/study-icons/small/cadcraft-96.png"),
        (IconSet::Study, "soundcraft") => include_bytes!("../../../../assets/study-icons/small/soundcraft-96.png"),
        (IconSet::Study, "figcraft") => include_bytes!("../../../../assets/study-icons/small/figcraft-96.png"),
        _ => return None,
    })
}

/// The 64 px PNG, the last resort for small embedded icons.
#[cfg(target_os = "macos")]
pub(crate) fn png64(set: IconSet, id: &str) -> Option<&'static [u8]> {
    Some(match (set, id) {
        (IconSet::Craft, "photocraft") => include_bytes!("../../../../assets/craft-icons/small/photocraft-64.png"),
        (IconSet::Craft, "vectorcraft") => include_bytes!("../../../../assets/craft-icons/small/vectorcraft-64.png"),
        (IconSet::Craft, "filmcraft") => include_bytes!("../../../../assets/craft-icons/small/filmcraft-64.png"),
        (IconSet::Craft, "lightcraft") => include_bytes!("../../../../assets/craft-icons/small/lightcraft-64.png"),
        (IconSet::Craft, "pdfcraft") => include_bytes!("../../../../assets/craft-icons/small/pdfcraft-64.png"),
        (IconSet::Craft, "effectcraft") => include_bytes!("../../../../assets/craft-icons/small/effectcraft-64.png"),
        (IconSet::Craft, "designcraft") => include_bytes!("../../../../assets/craft-icons/small/designcraft-64.png"),
        (IconSet::Craft, "wordcraft") => include_bytes!("../../../../assets/craft-icons/small/wordcraft-64.png"),
        (IconSet::Craft, "deckcraft") => include_bytes!("../../../../assets/craft-icons/small/deckcraft-64.png"),
        (IconSet::Craft, "gridcraft") => include_bytes!("../../../../assets/craft-icons/small/gridcraft-64.png"),
        (IconSet::Craft, "cadcraft") => include_bytes!("../../../../assets/craft-icons/small/cadcraft-64.png"),
        (IconSet::Craft, "soundcraft") => include_bytes!("../../../../assets/craft-icons/small/soundcraft-64.png"),
        (IconSet::Craft, "figcraft") => include_bytes!("../../../../assets/craft-icons/small/figcraft-64.png"),
        (IconSet::Study, "photocraft") => include_bytes!("../../../../assets/study-icons/small/photocraft-64.png"),
        (IconSet::Study, "vectorcraft") => include_bytes!("../../../../assets/study-icons/small/vectorcraft-64.png"),
        (IconSet::Study, "filmcraft") => include_bytes!("../../../../assets/study-icons/small/filmcraft-64.png"),
        (IconSet::Study, "lightcraft") => include_bytes!("../../../../assets/study-icons/small/lightcraft-64.png"),
        (IconSet::Study, "pdfcraft") => include_bytes!("../../../../assets/study-icons/small/pdfcraft-64.png"),
        (IconSet::Study, "effectcraft") => include_bytes!("../../../../assets/study-icons/small/effectcraft-64.png"),
        (IconSet::Study, "designcraft") => include_bytes!("../../../../assets/study-icons/small/designcraft-64.png"),
        (IconSet::Study, "wordcraft") => include_bytes!("../../../../assets/study-icons/small/wordcraft-64.png"),
        (IconSet::Study, "deckcraft") => include_bytes!("../../../../assets/study-icons/small/deckcraft-64.png"),
        (IconSet::Study, "gridcraft") => include_bytes!("../../../../assets/study-icons/small/gridcraft-64.png"),
        (IconSet::Study, "cadcraft") => include_bytes!("../../../../assets/study-icons/small/cadcraft-64.png"),
        (IconSet::Study, "soundcraft") => include_bytes!("../../../../assets/study-icons/small/soundcraft-64.png"),
        (IconSet::Study, "figcraft") => include_bytes!("../../../../assets/study-icons/small/figcraft-64.png"),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::{IconSet, image, png256};

    #[test]
    fn every_app_has_both_sets() {
        for set in IconSet::ALL {
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
                assert!(png256(set, id).is_some() && image(set, id).is_some(), "{set:?} {id}");
            }
            assert_eq!(IconSet::from_key(set.key()), Some(set));
        }
        assert_eq!(IconSet::from_key("craft"), Some(IconSet::Study));
    }
}
