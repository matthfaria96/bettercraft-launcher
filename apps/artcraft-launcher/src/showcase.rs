//! BetterCraft: a demo state for the witchmymac.com screenshots, on only while
//! `BETTERCRAFT_SHOWCASE` is set (see CLAUDE.md). The backend answers from memory: no network, no
//! look at what's installed, and every command is ignored, so nothing is downloaded, installed,
//! opened or removed. The icon pass only reports what it would do (`bettercraft::showcase`).

use std::time::{SystemTime, UNIX_EPOCH};

use artcraft_launcher_core::protocol::{Backend, Capabilities, Command, Event, JobPhase, LatestRelease, Origin, Source};
use artcraft_launcher_core::release::{Asset, Release};
use artcraft_launcher_core::status::InstallSource;
use artcraft_launcher_core::{Arch, Installed, Os, Platform, SUITE, version};

/// `BETTERCRAFT_SHOWCASE` is set.
pub(crate) fn on() -> bool {
    std::env::var_os("BETTERCRAFT_SHOWCASE").is_some()
}

/// `BETTERCRAFT_SHOWCASE=settings`: open with Settings showing.
pub(crate) fn settings() -> bool {
    std::env::var("BETTERCRAFT_SHOWCASE").is_ok_and(|v| v == "settings")
}

/// `BETTERCRAFT_SHOWCASE_THEME=light|dark`, otherwise the saved choice.
pub(crate) fn theme() -> Option<artcraft_launcher_ui_egui::theme::ThemeChoice> {
    use artcraft_launcher_ui_egui::theme::ThemeChoice;
    match std::env::var("BETTERCRAFT_SHOWCASE_THEME").ok()?.as_str() {
        "light" => Some(ThemeChoice::Light),
        "dark" => Some(ThemeChoice::Dark),
        _ => None,
    }
}

/// `BETTERCRAFT_SHOWCASE_SIZE=<w>x<h>`: the window's size in points (showcase only).
pub(crate) fn size() -> Option<[f32; 2]> {
    if !on() {
        return None;
    }
    let text = std::env::var("BETTERCRAFT_SHOWCASE_SIZE").ok()?;
    let (w, h) = text.split_once('x')?;
    Some([w.trim().parse().ok()?, h.trim().parse().ok()?])
}

/// Latest version of each app, and the version "installed" (`None`: not installed).
const APPS: [(&str, &str, Option<&str>); 13] = [
    ("photocraft", "0.10.0", Some("0.9.2")),
    ("vectorcraft", "0.13.1", Some("0.13.1")),
    ("filmcraft", "0.9.0", None),
    ("lightcraft", "0.9.1", Some("0.9.1")),
    ("pdfcraft", "0.9.0", Some("0.9.0")),
    ("effectcraft", "0.11.1", Some("0.11.1")),
    ("designcraft", "0.9.0", Some("0.9.0")),
    ("wordcraft", "0.8.0", Some("0.8.0")),
    ("deckcraft", "0.8.0", Some("0.8.0")),
    ("gridcraft", "0.8.0", Some("0.7.4")),
    ("cadcraft", "0.8.0", None),
    ("soundcraft", "0.8.0", Some("0.8.0")),
    ("figcraft", "0.7.0", Some("0.7.0")),
];

pub(crate) struct Demo {
    events: Vec<Event>,
}

impl Demo {
    pub(crate) fn new() -> Self {
        let now = unix_now();
        let mut events = Vec::new();
        for (i, (id, latest, installed)) in APPS.iter().enumerate() {
            let at = now - 86_400 * (2 + i as i64);
            events.push(Event::Latest { app: (*id).into(), latest: LatestRelease::Found(release(id, latest, at)), from: Origin::ArtCraftApi });
            let name = SUITE.iter().find(|a| a.id == *id).map_or(*id, |a| a.name);
            let installed = installed.map(|v| Installed {
                version: version::parse(v),
                source: InstallSource::MacApp,
                // Never a real bundle: the showcase icon pass reads no files.
                location: format!("/BetterCraft Showcase/{name}.app"),
                package: None,
            });
            events.push(Event::Detected { app: (*id).into(), installed });
        }
        events.push(Event::Progress { app: "filmcraft".into(), phase: JobPhase::Downloading { done: 61 << 20, total: 142 << 20 } });
        events.push(Event::CheckFinished { at: now - 60, source: Source::Feed, rate_limited_until: None });
        Self { events }
    }
}

impl Backend for Demo {
    fn send(&mut self, _command: Command) {}

    fn poll(&mut self) -> Vec<Event> {
        std::mem::take(&mut self.events)
    }

    fn capabilities(&self) -> Capabilities {
        Capabilities { manage_apps: true, reveal_label: "Show in Finder", self_update: false }
    }

    fn platform(&self) -> Option<Platform> {
        Some(Platform { os: Os::MacOs, arch: if cfg!(target_arch = "aarch64") { Arch::Arm64 } else { Arch::X64 } })
    }

    fn now(&self) -> i64 {
        unix_now()
    }
}

fn unix_now() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| i64::try_from(d.as_secs()).unwrap_or(0))
}

fn release(id: &str, v: &str, at: i64) -> Release {
    let name = format!("{id}-{v}-macos-universal.dmg");
    Release {
        tag: format!("v{v}"),
        version: version::parse(v),
        name: format!("v{v}"),
        published_at: Some(at),
        html_url: format!("https://github.com/storytold/{id}/releases/tag/v{v}"),
        notes: String::new(),
        prerelease: false,
        assets: vec![Asset {
            name: name.clone(),
            size: 140 << 20,
            url: format!("https://github.com/storytold/{id}/releases/download/v{v}/{name}"),
            sha256: None,
        }],
    }
}
