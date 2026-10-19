//! The Dock icon of an installed Craft App, which the Finder's custom icon doesn't reach:
//!
//! - while the app starts, the Dock shows the bundle's own `.icns` (`CFBundleIconFile`): it is
//!   replaced by the original program's, and the Craft one kept beside it to put back;
//! - once it runs, eframe sets the Dock icon to its built-in default (the egui hexagon, a PNG
//!   embedded in the executable) because the Craft Apps don't give one. Those PNG bytes are
//!   replaced in the executable by the original program's icon, a PNG of at most the same size,
//!   padded with zeros (PNG decoders stop at `IEND`), and swapped back to undo;
//! - a Craft App that gives eframe its own icon embeds it as a PNG too: any embedded PNG that looks
//!   like the app's own `.icns` (compared small) is replaced the same way, and kept in
//!   `Resources/bettercraft-runtime-N.png` to put back.
//!
//! Both change sealed files: the caller signs the bundle again afterwards.

use std::path::{Path, PathBuf};
use std::process::Command;

/// eframe 0.36's default app icon, as embedded in every eframe executable.
const EFRAME_DEFAULT: &[u8] = include_bytes!("../../../../assets/better-icons/dock/eframe-default.png");
/// Where the bundle's own `.icns` is kept while the original program's is in its place.
const BACKUP: &str = "bettercraft-original.icns";
/// Where the app's own embedded icons are kept: `bettercraft-runtime-0.png`, `-1`…
const RUNTIME_PREFIX: &str = "bettercraft-runtime-";
/// Below this [`difference`] an embedded picture is taken for the app's own icon. Measured: the
/// app's icon scores 0.9–1.4 against a reference, everything else in the Craft Apps 29 and up.
const ALIKE: f32 = 10.0;

/// The original program's icon at 256 px, for embedded icons with room for it.
fn large_png(id: &str) -> Option<&'static [u8]> {
    Some(match id {
        "photocraft" => include_bytes!("../../../../assets/better-icons/photocraft.png"),
        "vectorcraft" => include_bytes!("../../../../assets/better-icons/vectorcraft.png"),
        "filmcraft" => include_bytes!("../../../../assets/better-icons/filmcraft.png"),
        "lightcraft" => include_bytes!("../../../../assets/better-icons/lightcraft.png"),
        "pdfcraft" => include_bytes!("../../../../assets/better-icons/pdfcraft.png"),
        "effectcraft" => include_bytes!("../../../../assets/better-icons/effectcraft.png"),
        "designcraft" => include_bytes!("../../../../assets/better-icons/designcraft.png"),
        "wordcraft" => include_bytes!("../../../../assets/better-icons/wordcraft.png"),
        "deckcraft" => include_bytes!("../../../../assets/better-icons/deckcraft.png"),
        "gridcraft" => include_bytes!("../../../../assets/better-icons/gridcraft.png"),
        "cadcraft" => include_bytes!("../../../../assets/better-icons/cadcraft.png"),
        "soundcraft" => include_bytes!("../../../../assets/better-icons/soundcraft.png"),
        "figcraft" => include_bytes!("../../../../assets/better-icons/figcraft.png"),
        _ => return None,
    })
}

/// The original program's icon at 96 px, for small embedded icons.
fn small_png(id: &str) -> Option<&'static [u8]> {
    Some(match id {
        "photocraft" => include_bytes!("../../../../assets/better-icons/small/photocraft-96.png"),
        "vectorcraft" => include_bytes!("../../../../assets/better-icons/small/vectorcraft-96.png"),
        "filmcraft" => include_bytes!("../../../../assets/better-icons/small/filmcraft-96.png"),
        "lightcraft" => include_bytes!("../../../../assets/better-icons/small/lightcraft-96.png"),
        "pdfcraft" => include_bytes!("../../../../assets/better-icons/small/pdfcraft-96.png"),
        "effectcraft" => include_bytes!("../../../../assets/better-icons/small/effectcraft-96.png"),
        "designcraft" => include_bytes!("../../../../assets/better-icons/small/designcraft-96.png"),
        "wordcraft" => include_bytes!("../../../../assets/better-icons/small/wordcraft-96.png"),
        "deckcraft" => include_bytes!("../../../../assets/better-icons/small/deckcraft-96.png"),
        "gridcraft" => include_bytes!("../../../../assets/better-icons/small/gridcraft-96.png"),
        "cadcraft" => include_bytes!("../../../../assets/better-icons/small/cadcraft-96.png"),
        "soundcraft" => include_bytes!("../../../../assets/better-icons/small/soundcraft-96.png"),
        "figcraft" => include_bytes!("../../../../assets/better-icons/small/figcraft-96.png"),
        _ => return None,
    })
}

/// The original program's icon at 64 px, the last resort.
fn tiny_png(id: &str) -> Option<&'static [u8]> {
    Some(match id {
        "photocraft" => include_bytes!("../../../../assets/better-icons/small/photocraft-64.png"),
        "vectorcraft" => include_bytes!("../../../../assets/better-icons/small/vectorcraft-64.png"),
        "filmcraft" => include_bytes!("../../../../assets/better-icons/small/filmcraft-64.png"),
        "lightcraft" => include_bytes!("../../../../assets/better-icons/small/lightcraft-64.png"),
        "pdfcraft" => include_bytes!("../../../../assets/better-icons/small/pdfcraft-64.png"),
        "effectcraft" => include_bytes!("../../../../assets/better-icons/small/effectcraft-64.png"),
        "designcraft" => include_bytes!("../../../../assets/better-icons/small/designcraft-64.png"),
        "wordcraft" => include_bytes!("../../../../assets/better-icons/small/wordcraft-64.png"),
        "deckcraft" => include_bytes!("../../../../assets/better-icons/small/deckcraft-64.png"),
        "gridcraft" => include_bytes!("../../../../assets/better-icons/small/gridcraft-64.png"),
        "cadcraft" => include_bytes!("../../../../assets/better-icons/small/cadcraft-64.png"),
        "soundcraft" => include_bytes!("../../../../assets/better-icons/small/soundcraft-64.png"),
        "figcraft" => include_bytes!("../../../../assets/better-icons/small/figcraft-64.png"),
        _ => return None,
    })
}

/// The best of the original program's icons that fits in `len` bytes, padded with zeros to `len`.
fn ours_for(id: &str, len: usize) -> Option<Vec<u8>> {
    [large_png(id), dock_png(id), small_png(id), tiny_png(id)].into_iter().flatten().find_map(|png| padded(png, len))
}

/// The original program's icon as a PNG no larger than eframe's default, for the executable.
fn dock_png(id: &str) -> Option<&'static [u8]> {
    Some(match id {
        "photocraft" => include_bytes!("../../../../assets/better-icons/dock/photocraft.png"),
        "vectorcraft" => include_bytes!("../../../../assets/better-icons/dock/vectorcraft.png"),
        "filmcraft" => include_bytes!("../../../../assets/better-icons/dock/filmcraft.png"),
        "lightcraft" => include_bytes!("../../../../assets/better-icons/dock/lightcraft.png"),
        "pdfcraft" => include_bytes!("../../../../assets/better-icons/dock/pdfcraft.png"),
        "effectcraft" => include_bytes!("../../../../assets/better-icons/dock/effectcraft.png"),
        "designcraft" => include_bytes!("../../../../assets/better-icons/dock/designcraft.png"),
        "wordcraft" => include_bytes!("../../../../assets/better-icons/dock/wordcraft.png"),
        "deckcraft" => include_bytes!("../../../../assets/better-icons/dock/deckcraft.png"),
        "gridcraft" => include_bytes!("../../../../assets/better-icons/dock/gridcraft.png"),
        "cadcraft" => include_bytes!("../../../../assets/better-icons/dock/cadcraft.png"),
        "soundcraft" => include_bytes!("../../../../assets/better-icons/dock/soundcraft.png"),
        "figcraft" => include_bytes!("../../../../assets/better-icons/dock/figcraft.png"),
        _ => return None,
    })
}

/// `png` padded with zeros to `len`; `None` if it doesn't fit.
fn padded(png: &[u8], len: usize) -> Option<Vec<u8>> {
    let extra = len.checked_sub(png.len())?;
    let mut bytes = png.to_vec();
    bytes.extend(std::iter::repeat_n(0u8, extra));
    Some(bytes)
}

pub(super) fn is_patched(bundle: &Path) -> bool {
    bundle.join("Contents/Resources").join(BACKUP).exists()
}

/// Puts the original program's Dock icons into the bundle (`on`) or the Craft ones back.
pub(super) fn set(id: &str, bundle: &Path, on: bool) -> Result<(), String> {
    let missing = || format!("no Dock icon for {id}");
    let resources = bundle.join("Contents/Resources");
    let icns = resources.join(icon_file(bundle)?);
    let backup = resources.join(BACKUP);
    let exe = bundle.join("Contents/MacOS").join(plist_value(bundle, "CFBundleExecutable")?);
    let eframe_ours = ours_for(id, EFRAME_DEFAULT.len()).ok_or_else(missing)?;
    if on {
        let original = super::icons::icns(id).ok_or_else(|| format!("no original icon for {id}"))?;
        if !backup.exists() {
            std::fs::copy(&icns, &backup).map_err(|e| format!("couldn't keep the Craft icon ({e})"))?;
        }
        std::fs::write(&icns, original).map_err(|e| format!("couldn't write the icon ({e})"))?;
        replace_in_file(&exe, EFRAME_DEFAULT, &eframe_ours)?;
        replace_own_icons(id, &exe, &backup, &resources)
    } else {
        for kept in runtime_backups(&resources) {
            let craft = std::fs::read(&kept).map_err(|e| e.to_string())?;
            let ours = ours_for(id, craft.len()).ok_or_else(missing)?;
            replace_in_file(&exe, &ours, &craft)?;
            std::fs::remove_file(&kept).map_err(|e| e.to_string())?;
        }
        replace_in_file(&exe, &eframe_ours, EFRAME_DEFAULT)?;
        if backup.exists() {
            std::fs::rename(&backup, &icns).map_err(|e| format!("couldn't restore the Craft icon ({e})"))?;
        }
        Ok(())
    }
}

/// The app's own icons embedded in `exe` (alike its `.icns`, kept at `craft_icns`, or the Craft
/// icon the launcher shows, which some versions use instead) are kept in `resources` and replaced
/// by the original program's.
fn replace_own_icons(id: &str, exe: &Path, craft_icns: &Path, resources: &Path) -> Result<(), String> {
    let mut references = vec![icns_picture(craft_icns)?];
    if let Some(egui::ImageSource::Bytes { bytes, .. }) = crate::icons::app(id)
        && let Ok(picture) = image::load_from_memory(bytes.as_ref())
    {
        references.push(picture);
    }
    let bytes = std::fs::read(exe).map_err(|e| format!("couldn't read {} ({e})", exe.display()))?;
    let mut own: Vec<&[u8]> = Vec::new();
    for (at, len) in embedded_pngs(&bytes) {
        let Some(png) = bytes.get(at..at + len) else { continue };
        if own.contains(&png) {
            continue;
        }
        let Ok(picture) = image::load_from_memory(png) else { continue };
        if picture.width() == picture.height() && picture.width() >= 64 && references.iter().any(|r| difference(&picture, r) < ALIKE) {
            own.push(png);
        }
    }
    let mut n = runtime_backups(resources).len();
    for craft in own {
        let Some(ours) = ours_for(id, craft.len()) else { continue };
        std::fs::write(resources.join(format!("{RUNTIME_PREFIX}{n}.png")), craft).map_err(|e| e.to_string())?;
        n += 1;
        replace_in_file(exe, craft, &ours)?;
    }
    Ok(())
}

fn runtime_backups(resources: &Path) -> Vec<PathBuf> {
    let Ok(dir) = std::fs::read_dir(resources) else { return Vec::new() };
    let mut kept: Vec<PathBuf> =
        dir.flatten().map(|e| e.path()).filter(|p| p.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.starts_with(RUNTIME_PREFIX))).collect();
    kept.sort();
    kept
}

/// An `.icns` as a picture, through `sips` (the `image` crate doesn't read `.icns`).
fn icns_picture(icns: &Path) -> Result<image::DynamicImage, String> {
    let png = std::env::temp_dir().join(format!("bettercraft-ref-{}.png", std::process::id()));
    let out = Command::new("/usr/bin/sips")
        .args(["-s", "format", "png", "-Z", "128"])
        .arg(icns)
        .arg("--out")
        .arg(&png)
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(format!("sips couldn't read {}", icns.display()));
    }
    let picture = image::open(&png).map_err(|e| e.to_string());
    let _ = std::fs::remove_file(&png);
    picture
}

/// Every PNG embedded in `bytes`: its offset and length, found by walking its chunks from the
/// signature to `IEND` (a search for `IEND` alone can run into the next PNG).
fn embedded_pngs(bytes: &[u8]) -> Vec<(usize, usize)> {
    const SIG: &[u8] = b"\x89PNG\r\n\x1a\n";
    let mut found = Vec::new();
    let mut at = 0;
    while let Some(start) = bytes.get(at..).and_then(|rest| rest.windows(SIG.len()).position(|w| w == SIG)).map(|p| p + at) {
        if let Some(len) = png_len(bytes.get(start..).unwrap_or_default()) {
            found.push((start, len));
        }
        at = start + SIG.len();
    }
    found
}

/// The length of the PNG at the start of `bytes`, if its chunks hold together up to `IEND`.
fn png_len(bytes: &[u8]) -> Option<usize> {
    let mut at = 8;
    loop {
        let len = u32::from_be_bytes(bytes.get(at..at + 4)?.try_into().ok()?) as usize;
        let kind = bytes.get(at + 4..at + 8)?;
        if !kind.iter().all(u8::is_ascii_alphabetic) {
            return None;
        }
        // length, type, data, CRC
        at = at.checked_add(12)?.checked_add(len)?;
        if at > bytes.len() {
            return None;
        }
        if kind == b"IEND" {
            return Some(at);
        }
    }
}

/// How unlike two pictures are, 0 (same) to 255: their mean colour difference at 32 × 32, over
/// a black background.
fn difference(a: &image::DynamicImage, b: &image::DynamicImage) -> f32 {
    let small = |i: &image::DynamicImage| image::imageops::resize(&i.to_rgba8(), 32, 32, image::imageops::FilterType::Triangle);
    let (a, b) = (small(a), small(b));
    let mut total = 0.0;
    for (p, q) in a.pixels().zip(b.pixels()) {
        for c in 0..3 {
            let pa = f32::from(p.0[c]) * f32::from(p.0[3]) / 255.0;
            let qa = f32::from(q.0[c]) * f32::from(q.0[3]) / 255.0;
            total += (pa - qa).abs();
        }
    }
    total / (32.0 * 32.0 * 3.0)
}

/// `CFBundleIconFile`, with the `.icns` it may leave out.
fn icon_file(bundle: &Path) -> Result<String, String> {
    let name = plist_value(bundle, "CFBundleIconFile")?;
    Ok(if name.ends_with(".icns") { name } else { format!("{name}.icns") })
}

fn plist_value(bundle: &Path, key: &str) -> Result<String, String> {
    let out = Command::new("/usr/libexec/PlistBuddy")
        .args(["-c", &format!("Print :{key}")])
        .arg(bundle.join("Contents/Info.plist"))
        .output()
        .map_err(|e| e.to_string())?;
    let value = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if !out.status.success() || value.is_empty() || value.contains('/') {
        return Err(format!("the app's Info.plist has no usable {key}"));
    }
    Ok(value)
}

/// Replaces every `from` in the file with `to` (same length), writing a new file in its place so a
/// running copy keeps the old one. Nothing to replace is not an error.
fn replace_in_file(path: &Path, from: &[u8], to: &[u8]) -> Result<(), String> {
    let bytes = std::fs::read(path).map_err(|e| format!("couldn't read {} ({e})", path.display()))?;
    let Some(patched) = replace_all(&bytes, from, to) else { return Ok(()) };
    let temp: PathBuf = path.with_extension("bettercraft-tmp");
    std::fs::write(&temp, patched).map_err(|e| format!("couldn't write {} ({e})", temp.display()))?;
    let permissions = std::fs::metadata(path).map_err(|e| e.to_string())?.permissions();
    std::fs::set_permissions(&temp, permissions).map_err(|e| e.to_string())?;
    std::fs::rename(&temp, path).map_err(|e| format!("couldn't replace {} ({e})", path.display()))
}

/// `bytes` with every `from` replaced by `to`; `None` when there is none, or the lengths differ.
fn replace_all(bytes: &[u8], from: &[u8], to: &[u8]) -> Option<Vec<u8>> {
    if from.is_empty() || from.len() != to.len() {
        return None;
    }
    let mut out = Vec::with_capacity(bytes.len());
    let mut rest = bytes;
    let mut found = false;
    while let Some(at) = rest.windows(from.len()).position(|w| w == from) {
        out.extend_from_slice(rest.get(..at)?);
        out.extend_from_slice(to);
        rest = rest.get(at + from.len()..)?;
        found = true;
    }
    out.extend_from_slice(rest);
    found.then_some(out)
}

#[cfg(test)]
mod tests {
    use super::{EFRAME_DEFAULT, dock_png, ours_for, padded, replace_all};

    const IDS: [&str; 13] = [
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
    ];

    #[test]
    fn every_dock_icon_fits_and_still_decodes_with_its_padding() {
        // eframe unwraps the decode of its default icon: a bad PNG here would crash the app.
        for id in IDS {
            let bytes = dock_png(id).and_then(|p| padded(p, EFRAME_DEFAULT.len())).unwrap_or_else(|| panic!("{id} doesn't fit"));
            assert_eq!(bytes.len(), EFRAME_DEFAULT.len(), "{id}");
            let image = image::load_from_memory(&bytes).unwrap_or_else(|e| panic!("{id}: {e}"));
            assert!(image.width() >= 120, "{id} is only {} px", image.width());
            // Room for the 256 px one, as in an app's own 512 px icon.
            let large = ours_for(id, 300_000).unwrap();
            assert_eq!(image::load_from_memory(&large).map(|i| i.width()).ok(), Some(256), "{id}");
        }
    }

    /// Prints how alike each square PNG in the installed Craft Apps is to the app's own icon.
    /// `cargo test -p artcraft-launcher-ui-egui probe_scores -- --ignored --nocapture`
    #[test]
    #[ignore = "reads the apps installed on this Mac"]
    fn probe_scores() {
        for app in ["FilmCraft", "GridCraft", "PdfCraft", "PhotoCraft", "WordCraft"] {
            let bundle = std::path::PathBuf::from(format!("/Applications/{app}.app"));
            let icns = bundle.join("Contents/Resources").join(super::BACKUP);
            let icns = if icns.exists() { icns } else { bundle.join(format!("Contents/Resources/{app}.icns")) };
            let png = std::env::temp_dir().join(format!("probe-{app}.png"));
            std::process::Command::new("sips").args(["-s", "format", "png", "-Z", "128"]).arg(&icns).arg("--out").arg(&png).output().unwrap();
            let reference = image::open(&png).unwrap();
            let id = app.to_lowercase();
            let Some(egui::ImageSource::Bytes { bytes, .. }) = crate::icons::app(&id) else { panic!() };
            let launcher = image::load_from_memory(bytes.as_ref()).unwrap();
            let exe = std::fs::read(bundle.join(format!("Contents/MacOS/{app}"))).unwrap();
            for (at, len) in super::embedded_pngs(&exe) {
                let Ok(img) = image::load_from_memory(&exe[at..at + len]) else { continue };
                if img.width() == img.height() && img.width() >= 64 {
                    println!(
                        "{app} @{at} {}px {len}B icns {:.1} launcher {:.1}",
                        img.width(),
                        super::difference(&img, &reference),
                        super::difference(&img, &launcher)
                    );
                }
            }
        }
    }

    #[test]
    fn replacing_swaps_every_copy_and_back() {
        let ours = padded(dock_png("photocraft").unwrap(), EFRAME_DEFAULT.len()).unwrap();
        let exe = [b"head".as_slice(), EFRAME_DEFAULT, b"mid", EFRAME_DEFAULT, b"tail"].concat();
        let patched = replace_all(&exe, EFRAME_DEFAULT, &ours).unwrap();
        assert_eq!(patched.len(), exe.len());
        assert!(replace_all(&patched, EFRAME_DEFAULT, &ours).is_none());
        assert_eq!(replace_all(&patched, &ours, EFRAME_DEFAULT).unwrap(), exe);
    }
}
