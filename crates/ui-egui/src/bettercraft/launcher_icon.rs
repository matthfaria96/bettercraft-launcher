//! The launcher's own icon follows the icon set: the official BetterCraft logo for Normal (the
//! bundle's `.icns`, put there by `scripts/install-mac.sh`), the older "A" logo for Study. In Study
//! the launcher's bundle gets the old logo as its Finder icon (`machine_icons`), and the running
//! Dock icon is set to it; back in Normal both return to the official logo.

use std::sync::Mutex;

use egui::ImageSource;

use super::assets::IconSet;

const NORMAL_PNG: &[u8] = include_bytes!("../../../../assets/bettercraft/logo-normal-256.png");
const STUDY_PNG: &[u8] = include_bytes!("../../../../assets/bettercraft/logo-study-256.png");
/// The Study logo for the bundle's Finder icon.
#[cfg(target_os = "macos")]
pub(super) const STUDY_ICNS: &[u8] = include_bytes!("../../../../assets/bettercraft/logo-study.icns");

/// The set whose logo the Dock shows, once this run has set it (`None`: the bundle's own).
static DOCK: Mutex<Option<IconSet>> = Mutex::new(None);

/// The launcher's logo for `set`, for the Settings header.
pub(crate) fn logo(set: IconSet) -> ImageSource<'static> {
    let (uri, bytes) = match set {
        IconSet::Craft => ("bytes://bettercraft/logo-normal.png", NORMAL_PNG),
        IconSet::Study => ("bytes://bettercraft/logo-study.png", STUDY_PNG),
    };
    ImageSource::Bytes { uri: uri.into(), bytes: egui::load::Bytes::Static(bytes) }
}

/// Keeps the running Dock icon on the logo of the current set. Nothing is sent while Normal is
/// all this run has seen: the bundle's own `.icns` already is the official logo, drawn with the
/// system's icon treatment.
pub(super) fn sync_dock(ctx: &egui::Context, set: IconSet) {
    let Ok(mut shown) = DOCK.lock() else { return };
    let wanted = match (*shown, set) {
        (None, IconSet::Craft) => return,
        (current, set) if current == Some(set) => return,
        (_, set) => set,
    };
    let png = if wanted == IconSet::Study { STUDY_PNG } else { NORMAL_PNG };
    let Ok(picture) = image::load_from_memory(png) else { return };
    let picture = picture.to_rgba8();
    let icon = egui::IconData { width: picture.width(), height: picture.height(), rgba: picture.into_raw() };
    ctx.send_viewport_cmd(egui::ViewportCommand::Icon(Some(std::sync::Arc::new(icon))));
    *shown = Some(wanted);
}

/// The `.app` bundle this launcher runs from, if it runs from one.
#[cfg(target_os = "macos")]
pub(super) fn own_bundle() -> Option<std::path::PathBuf> {
    let exe = std::env::current_exe().ok()?;
    exe.ancestors().find(|p| p.extension().is_some_and(|e| e == "app")).map(std::path::Path::to_path_buf)
}

#[cfg(test)]
mod tests {
    use super::{IconSet, NORMAL_PNG, STUDY_PNG, logo};

    #[test]
    fn both_logos_decode() {
        for png in [NORMAL_PNG, STUDY_PNG] {
            assert_eq!(image::load_from_memory(png).map(|i| i.width()).ok(), Some(256));
        }
        assert!(matches!(logo(IconSet::Study), egui::ImageSource::Bytes { .. }));
    }
}
