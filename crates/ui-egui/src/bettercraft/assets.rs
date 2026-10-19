//! The original programs' icons, in two styles: plain, or with the yellow "Craft" side badge
//! (`assets/better-icons-craft/`, made by `scripts/make-badged-icons.sh`). Each comes as the
//! `.icns` for the bundle, a 256 px PNG (the launcher's rows and roomy embedded icons), a PNG that
//! fits eframe's default icon (the Dock), and 96 / 64 px PNGs for small embedded icons.

/// Bumped whenever the pictures change: installed apps carrying an older revision are re-dressed.
pub(crate) const REVISION: u32 = 2;

/// How the original programs' icons look: as they are, or with the "Craft" badge.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum IconStyle {
    #[default]
    Plain,
    Craft,
}

impl IconStyle {
    pub(crate) const ALL: [Self; 2] = [Self::Plain, Self::Craft];

    pub(crate) fn key(self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::Craft => "craft",
        }
    }

    pub(crate) fn from_key(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|s| s.key() == key.trim())
    }
}

/// The 256 px icon as an egui image (one copy of the bytes, shared with [`png256`]).
pub(crate) fn image(style: IconStyle, id: &str) -> Option<egui::ImageSource<'static>> {
    let uri = match (style, id) {
        (IconStyle::Plain, "photocraft") => "bytes://bettercraft/better-icons/photocraft.png",
        (IconStyle::Plain, "vectorcraft") => "bytes://bettercraft/better-icons/vectorcraft.png",
        (IconStyle::Plain, "filmcraft") => "bytes://bettercraft/better-icons/filmcraft.png",
        (IconStyle::Plain, "lightcraft") => "bytes://bettercraft/better-icons/lightcraft.png",
        (IconStyle::Plain, "pdfcraft") => "bytes://bettercraft/better-icons/pdfcraft.png",
        (IconStyle::Plain, "effectcraft") => "bytes://bettercraft/better-icons/effectcraft.png",
        (IconStyle::Plain, "designcraft") => "bytes://bettercraft/better-icons/designcraft.png",
        (IconStyle::Plain, "wordcraft") => "bytes://bettercraft/better-icons/wordcraft.png",
        (IconStyle::Plain, "deckcraft") => "bytes://bettercraft/better-icons/deckcraft.png",
        (IconStyle::Plain, "gridcraft") => "bytes://bettercraft/better-icons/gridcraft.png",
        (IconStyle::Plain, "cadcraft") => "bytes://bettercraft/better-icons/cadcraft.png",
        (IconStyle::Plain, "soundcraft") => "bytes://bettercraft/better-icons/soundcraft.png",
        (IconStyle::Plain, "figcraft") => "bytes://bettercraft/better-icons/figcraft.png",
        (IconStyle::Craft, "photocraft") => "bytes://bettercraft/better-icons-craft/photocraft.png",
        (IconStyle::Craft, "vectorcraft") => "bytes://bettercraft/better-icons-craft/vectorcraft.png",
        (IconStyle::Craft, "filmcraft") => "bytes://bettercraft/better-icons-craft/filmcraft.png",
        (IconStyle::Craft, "lightcraft") => "bytes://bettercraft/better-icons-craft/lightcraft.png",
        (IconStyle::Craft, "pdfcraft") => "bytes://bettercraft/better-icons-craft/pdfcraft.png",
        (IconStyle::Craft, "effectcraft") => "bytes://bettercraft/better-icons-craft/effectcraft.png",
        (IconStyle::Craft, "designcraft") => "bytes://bettercraft/better-icons-craft/designcraft.png",
        (IconStyle::Craft, "wordcraft") => "bytes://bettercraft/better-icons-craft/wordcraft.png",
        (IconStyle::Craft, "deckcraft") => "bytes://bettercraft/better-icons-craft/deckcraft.png",
        (IconStyle::Craft, "gridcraft") => "bytes://bettercraft/better-icons-craft/gridcraft.png",
        (IconStyle::Craft, "cadcraft") => "bytes://bettercraft/better-icons-craft/cadcraft.png",
        (IconStyle::Craft, "soundcraft") => "bytes://bettercraft/better-icons-craft/soundcraft.png",
        (IconStyle::Craft, "figcraft") => "bytes://bettercraft/better-icons-craft/figcraft.png",
        _ => return None,
    };
    Some(egui::ImageSource::Bytes { uri: uri.into(), bytes: egui::load::Bytes::Static(png256(style, id)?) })
}

/// The 256 px PNG.
pub(crate) fn png256(style: IconStyle, id: &str) -> Option<&'static [u8]> {
    Some(match (style, id) {
        (IconStyle::Plain, "photocraft") => include_bytes!("../../../../assets/better-icons/photocraft.png"),
        (IconStyle::Plain, "vectorcraft") => include_bytes!("../../../../assets/better-icons/vectorcraft.png"),
        (IconStyle::Plain, "filmcraft") => include_bytes!("../../../../assets/better-icons/filmcraft.png"),
        (IconStyle::Plain, "lightcraft") => include_bytes!("../../../../assets/better-icons/lightcraft.png"),
        (IconStyle::Plain, "pdfcraft") => include_bytes!("../../../../assets/better-icons/pdfcraft.png"),
        (IconStyle::Plain, "effectcraft") => include_bytes!("../../../../assets/better-icons/effectcraft.png"),
        (IconStyle::Plain, "designcraft") => include_bytes!("../../../../assets/better-icons/designcraft.png"),
        (IconStyle::Plain, "wordcraft") => include_bytes!("../../../../assets/better-icons/wordcraft.png"),
        (IconStyle::Plain, "deckcraft") => include_bytes!("../../../../assets/better-icons/deckcraft.png"),
        (IconStyle::Plain, "gridcraft") => include_bytes!("../../../../assets/better-icons/gridcraft.png"),
        (IconStyle::Plain, "cadcraft") => include_bytes!("../../../../assets/better-icons/cadcraft.png"),
        (IconStyle::Plain, "soundcraft") => include_bytes!("../../../../assets/better-icons/soundcraft.png"),
        (IconStyle::Plain, "figcraft") => include_bytes!("../../../../assets/better-icons/figcraft.png"),
        (IconStyle::Craft, "photocraft") => include_bytes!("../../../../assets/better-icons-craft/photocraft.png"),
        (IconStyle::Craft, "vectorcraft") => include_bytes!("../../../../assets/better-icons-craft/vectorcraft.png"),
        (IconStyle::Craft, "filmcraft") => include_bytes!("../../../../assets/better-icons-craft/filmcraft.png"),
        (IconStyle::Craft, "lightcraft") => include_bytes!("../../../../assets/better-icons-craft/lightcraft.png"),
        (IconStyle::Craft, "pdfcraft") => include_bytes!("../../../../assets/better-icons-craft/pdfcraft.png"),
        (IconStyle::Craft, "effectcraft") => include_bytes!("../../../../assets/better-icons-craft/effectcraft.png"),
        (IconStyle::Craft, "designcraft") => include_bytes!("../../../../assets/better-icons-craft/designcraft.png"),
        (IconStyle::Craft, "wordcraft") => include_bytes!("../../../../assets/better-icons-craft/wordcraft.png"),
        (IconStyle::Craft, "deckcraft") => include_bytes!("../../../../assets/better-icons-craft/deckcraft.png"),
        (IconStyle::Craft, "gridcraft") => include_bytes!("../../../../assets/better-icons-craft/gridcraft.png"),
        (IconStyle::Craft, "cadcraft") => include_bytes!("../../../../assets/better-icons-craft/cadcraft.png"),
        (IconStyle::Craft, "soundcraft") => include_bytes!("../../../../assets/better-icons-craft/soundcraft.png"),
        (IconStyle::Craft, "figcraft") => include_bytes!("../../../../assets/better-icons-craft/figcraft.png"),
        _ => return None,
    })
}

/// The `.icns` for the app bundle.
#[cfg(target_os = "macos")]
pub(crate) fn icns(style: IconStyle, id: &str) -> Option<&'static [u8]> {
    Some(match (style, id) {
        (IconStyle::Plain, "photocraft") => include_bytes!("../../../../assets/better-icons/icns/photocraft.icns"),
        (IconStyle::Plain, "vectorcraft") => include_bytes!("../../../../assets/better-icons/icns/vectorcraft.icns"),
        (IconStyle::Plain, "filmcraft") => include_bytes!("../../../../assets/better-icons/icns/filmcraft.icns"),
        (IconStyle::Plain, "lightcraft") => include_bytes!("../../../../assets/better-icons/icns/lightcraft.icns"),
        (IconStyle::Plain, "pdfcraft") => include_bytes!("../../../../assets/better-icons/icns/pdfcraft.icns"),
        (IconStyle::Plain, "effectcraft") => include_bytes!("../../../../assets/better-icons/icns/effectcraft.icns"),
        (IconStyle::Plain, "designcraft") => include_bytes!("../../../../assets/better-icons/icns/designcraft.icns"),
        (IconStyle::Plain, "wordcraft") => include_bytes!("../../../../assets/better-icons/icns/wordcraft.icns"),
        (IconStyle::Plain, "deckcraft") => include_bytes!("../../../../assets/better-icons/icns/deckcraft.icns"),
        (IconStyle::Plain, "gridcraft") => include_bytes!("../../../../assets/better-icons/icns/gridcraft.icns"),
        (IconStyle::Plain, "cadcraft") => include_bytes!("../../../../assets/better-icons/icns/cadcraft.icns"),
        (IconStyle::Plain, "soundcraft") => include_bytes!("../../../../assets/better-icons/icns/soundcraft.icns"),
        (IconStyle::Plain, "figcraft") => include_bytes!("../../../../assets/better-icons/icns/figcraft.icns"),
        (IconStyle::Craft, "photocraft") => include_bytes!("../../../../assets/better-icons-craft/icns/photocraft.icns"),
        (IconStyle::Craft, "vectorcraft") => include_bytes!("../../../../assets/better-icons-craft/icns/vectorcraft.icns"),
        (IconStyle::Craft, "filmcraft") => include_bytes!("../../../../assets/better-icons-craft/icns/filmcraft.icns"),
        (IconStyle::Craft, "lightcraft") => include_bytes!("../../../../assets/better-icons-craft/icns/lightcraft.icns"),
        (IconStyle::Craft, "pdfcraft") => include_bytes!("../../../../assets/better-icons-craft/icns/pdfcraft.icns"),
        (IconStyle::Craft, "effectcraft") => include_bytes!("../../../../assets/better-icons-craft/icns/effectcraft.icns"),
        (IconStyle::Craft, "designcraft") => include_bytes!("../../../../assets/better-icons-craft/icns/designcraft.icns"),
        (IconStyle::Craft, "wordcraft") => include_bytes!("../../../../assets/better-icons-craft/icns/wordcraft.icns"),
        (IconStyle::Craft, "deckcraft") => include_bytes!("../../../../assets/better-icons-craft/icns/deckcraft.icns"),
        (IconStyle::Craft, "gridcraft") => include_bytes!("../../../../assets/better-icons-craft/icns/gridcraft.icns"),
        (IconStyle::Craft, "cadcraft") => include_bytes!("../../../../assets/better-icons-craft/icns/cadcraft.icns"),
        (IconStyle::Craft, "soundcraft") => include_bytes!("../../../../assets/better-icons-craft/icns/soundcraft.icns"),
        (IconStyle::Craft, "figcraft") => include_bytes!("../../../../assets/better-icons-craft/icns/figcraft.icns"),
        _ => return None,
    })
}

/// A PNG no larger than eframe's default icon (12 052 bytes), for the Dock.
#[cfg(target_os = "macos")]
pub(crate) fn dock(style: IconStyle, id: &str) -> Option<&'static [u8]> {
    Some(match (style, id) {
        (IconStyle::Plain, "photocraft") => include_bytes!("../../../../assets/better-icons/dock/photocraft.png"),
        (IconStyle::Plain, "vectorcraft") => include_bytes!("../../../../assets/better-icons/dock/vectorcraft.png"),
        (IconStyle::Plain, "filmcraft") => include_bytes!("../../../../assets/better-icons/dock/filmcraft.png"),
        (IconStyle::Plain, "lightcraft") => include_bytes!("../../../../assets/better-icons/dock/lightcraft.png"),
        (IconStyle::Plain, "pdfcraft") => include_bytes!("../../../../assets/better-icons/dock/pdfcraft.png"),
        (IconStyle::Plain, "effectcraft") => include_bytes!("../../../../assets/better-icons/dock/effectcraft.png"),
        (IconStyle::Plain, "designcraft") => include_bytes!("../../../../assets/better-icons/dock/designcraft.png"),
        (IconStyle::Plain, "wordcraft") => include_bytes!("../../../../assets/better-icons/dock/wordcraft.png"),
        (IconStyle::Plain, "deckcraft") => include_bytes!("../../../../assets/better-icons/dock/deckcraft.png"),
        (IconStyle::Plain, "gridcraft") => include_bytes!("../../../../assets/better-icons/dock/gridcraft.png"),
        (IconStyle::Plain, "cadcraft") => include_bytes!("../../../../assets/better-icons/dock/cadcraft.png"),
        (IconStyle::Plain, "soundcraft") => include_bytes!("../../../../assets/better-icons/dock/soundcraft.png"),
        (IconStyle::Plain, "figcraft") => include_bytes!("../../../../assets/better-icons/dock/figcraft.png"),
        (IconStyle::Craft, "photocraft") => include_bytes!("../../../../assets/better-icons-craft/dock/photocraft.png"),
        (IconStyle::Craft, "vectorcraft") => include_bytes!("../../../../assets/better-icons-craft/dock/vectorcraft.png"),
        (IconStyle::Craft, "filmcraft") => include_bytes!("../../../../assets/better-icons-craft/dock/filmcraft.png"),
        (IconStyle::Craft, "lightcraft") => include_bytes!("../../../../assets/better-icons-craft/dock/lightcraft.png"),
        (IconStyle::Craft, "pdfcraft") => include_bytes!("../../../../assets/better-icons-craft/dock/pdfcraft.png"),
        (IconStyle::Craft, "effectcraft") => include_bytes!("../../../../assets/better-icons-craft/dock/effectcraft.png"),
        (IconStyle::Craft, "designcraft") => include_bytes!("../../../../assets/better-icons-craft/dock/designcraft.png"),
        (IconStyle::Craft, "wordcraft") => include_bytes!("../../../../assets/better-icons-craft/dock/wordcraft.png"),
        (IconStyle::Craft, "deckcraft") => include_bytes!("../../../../assets/better-icons-craft/dock/deckcraft.png"),
        (IconStyle::Craft, "gridcraft") => include_bytes!("../../../../assets/better-icons-craft/dock/gridcraft.png"),
        (IconStyle::Craft, "cadcraft") => include_bytes!("../../../../assets/better-icons-craft/dock/cadcraft.png"),
        (IconStyle::Craft, "soundcraft") => include_bytes!("../../../../assets/better-icons-craft/dock/soundcraft.png"),
        (IconStyle::Craft, "figcraft") => include_bytes!("../../../../assets/better-icons-craft/dock/figcraft.png"),
        _ => return None,
    })
}

/// The 96 px PNG.
#[cfg(target_os = "macos")]
pub(crate) fn png96(style: IconStyle, id: &str) -> Option<&'static [u8]> {
    Some(match (style, id) {
        (IconStyle::Plain, "photocraft") => include_bytes!("../../../../assets/better-icons/small/photocraft-96.png"),
        (IconStyle::Plain, "vectorcraft") => include_bytes!("../../../../assets/better-icons/small/vectorcraft-96.png"),
        (IconStyle::Plain, "filmcraft") => include_bytes!("../../../../assets/better-icons/small/filmcraft-96.png"),
        (IconStyle::Plain, "lightcraft") => include_bytes!("../../../../assets/better-icons/small/lightcraft-96.png"),
        (IconStyle::Plain, "pdfcraft") => include_bytes!("../../../../assets/better-icons/small/pdfcraft-96.png"),
        (IconStyle::Plain, "effectcraft") => include_bytes!("../../../../assets/better-icons/small/effectcraft-96.png"),
        (IconStyle::Plain, "designcraft") => include_bytes!("../../../../assets/better-icons/small/designcraft-96.png"),
        (IconStyle::Plain, "wordcraft") => include_bytes!("../../../../assets/better-icons/small/wordcraft-96.png"),
        (IconStyle::Plain, "deckcraft") => include_bytes!("../../../../assets/better-icons/small/deckcraft-96.png"),
        (IconStyle::Plain, "gridcraft") => include_bytes!("../../../../assets/better-icons/small/gridcraft-96.png"),
        (IconStyle::Plain, "cadcraft") => include_bytes!("../../../../assets/better-icons/small/cadcraft-96.png"),
        (IconStyle::Plain, "soundcraft") => include_bytes!("../../../../assets/better-icons/small/soundcraft-96.png"),
        (IconStyle::Plain, "figcraft") => include_bytes!("../../../../assets/better-icons/small/figcraft-96.png"),
        (IconStyle::Craft, "photocraft") => include_bytes!("../../../../assets/better-icons-craft/small/photocraft-96.png"),
        (IconStyle::Craft, "vectorcraft") => include_bytes!("../../../../assets/better-icons-craft/small/vectorcraft-96.png"),
        (IconStyle::Craft, "filmcraft") => include_bytes!("../../../../assets/better-icons-craft/small/filmcraft-96.png"),
        (IconStyle::Craft, "lightcraft") => include_bytes!("../../../../assets/better-icons-craft/small/lightcraft-96.png"),
        (IconStyle::Craft, "pdfcraft") => include_bytes!("../../../../assets/better-icons-craft/small/pdfcraft-96.png"),
        (IconStyle::Craft, "effectcraft") => include_bytes!("../../../../assets/better-icons-craft/small/effectcraft-96.png"),
        (IconStyle::Craft, "designcraft") => include_bytes!("../../../../assets/better-icons-craft/small/designcraft-96.png"),
        (IconStyle::Craft, "wordcraft") => include_bytes!("../../../../assets/better-icons-craft/small/wordcraft-96.png"),
        (IconStyle::Craft, "deckcraft") => include_bytes!("../../../../assets/better-icons-craft/small/deckcraft-96.png"),
        (IconStyle::Craft, "gridcraft") => include_bytes!("../../../../assets/better-icons-craft/small/gridcraft-96.png"),
        (IconStyle::Craft, "cadcraft") => include_bytes!("../../../../assets/better-icons-craft/small/cadcraft-96.png"),
        (IconStyle::Craft, "soundcraft") => include_bytes!("../../../../assets/better-icons-craft/small/soundcraft-96.png"),
        (IconStyle::Craft, "figcraft") => include_bytes!("../../../../assets/better-icons-craft/small/figcraft-96.png"),
        _ => return None,
    })
}

/// The 64 px PNG, the last resort for small embedded icons.
#[cfg(target_os = "macos")]
pub(crate) fn png64(style: IconStyle, id: &str) -> Option<&'static [u8]> {
    Some(match (style, id) {
        (IconStyle::Plain, "photocraft") => include_bytes!("../../../../assets/better-icons/small/photocraft-64.png"),
        (IconStyle::Plain, "vectorcraft") => include_bytes!("../../../../assets/better-icons/small/vectorcraft-64.png"),
        (IconStyle::Plain, "filmcraft") => include_bytes!("../../../../assets/better-icons/small/filmcraft-64.png"),
        (IconStyle::Plain, "lightcraft") => include_bytes!("../../../../assets/better-icons/small/lightcraft-64.png"),
        (IconStyle::Plain, "pdfcraft") => include_bytes!("../../../../assets/better-icons/small/pdfcraft-64.png"),
        (IconStyle::Plain, "effectcraft") => include_bytes!("../../../../assets/better-icons/small/effectcraft-64.png"),
        (IconStyle::Plain, "designcraft") => include_bytes!("../../../../assets/better-icons/small/designcraft-64.png"),
        (IconStyle::Plain, "wordcraft") => include_bytes!("../../../../assets/better-icons/small/wordcraft-64.png"),
        (IconStyle::Plain, "deckcraft") => include_bytes!("../../../../assets/better-icons/small/deckcraft-64.png"),
        (IconStyle::Plain, "gridcraft") => include_bytes!("../../../../assets/better-icons/small/gridcraft-64.png"),
        (IconStyle::Plain, "cadcraft") => include_bytes!("../../../../assets/better-icons/small/cadcraft-64.png"),
        (IconStyle::Plain, "soundcraft") => include_bytes!("../../../../assets/better-icons/small/soundcraft-64.png"),
        (IconStyle::Plain, "figcraft") => include_bytes!("../../../../assets/better-icons/small/figcraft-64.png"),
        (IconStyle::Craft, "photocraft") => include_bytes!("../../../../assets/better-icons-craft/small/photocraft-64.png"),
        (IconStyle::Craft, "vectorcraft") => include_bytes!("../../../../assets/better-icons-craft/small/vectorcraft-64.png"),
        (IconStyle::Craft, "filmcraft") => include_bytes!("../../../../assets/better-icons-craft/small/filmcraft-64.png"),
        (IconStyle::Craft, "lightcraft") => include_bytes!("../../../../assets/better-icons-craft/small/lightcraft-64.png"),
        (IconStyle::Craft, "pdfcraft") => include_bytes!("../../../../assets/better-icons-craft/small/pdfcraft-64.png"),
        (IconStyle::Craft, "effectcraft") => include_bytes!("../../../../assets/better-icons-craft/small/effectcraft-64.png"),
        (IconStyle::Craft, "designcraft") => include_bytes!("../../../../assets/better-icons-craft/small/designcraft-64.png"),
        (IconStyle::Craft, "wordcraft") => include_bytes!("../../../../assets/better-icons-craft/small/wordcraft-64.png"),
        (IconStyle::Craft, "deckcraft") => include_bytes!("../../../../assets/better-icons-craft/small/deckcraft-64.png"),
        (IconStyle::Craft, "gridcraft") => include_bytes!("../../../../assets/better-icons-craft/small/gridcraft-64.png"),
        (IconStyle::Craft, "cadcraft") => include_bytes!("../../../../assets/better-icons-craft/small/cadcraft-64.png"),
        (IconStyle::Craft, "soundcraft") => include_bytes!("../../../../assets/better-icons-craft/small/soundcraft-64.png"),
        (IconStyle::Craft, "figcraft") => include_bytes!("../../../../assets/better-icons-craft/small/figcraft-64.png"),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::{IconStyle, image, png256};

    #[test]
    fn every_app_has_both_styles() {
        for style in IconStyle::ALL {
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
                assert!(png256(style, id).is_some() && image(style, id).is_some(), "{style:?} {id}");
            }
            assert_eq!(IconStyle::from_key(style.key()), Some(style));
        }
    }
}
