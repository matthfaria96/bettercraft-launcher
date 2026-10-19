//! BetterCraft: everything this fork adds to the upstream `storytold/craft-launcher`.
//!
//! - `icons`: the original programs' icons in place of the Craft ones, inside the launcher.
//! - `machine_icons`: the same icons on the installed `.app` bundles (macOS Finder, Dock, Launchpad).
//! - `compact`: a denser, quieter grid of apps.
//!
//! Upstream files only carry one-line hooks into this module (`lib.rs`, `icons.rs`, `views.rs`), so
//! the fork keeps merging cleanly with upstream. Choices live in egui's persisted memory, so the
//! upstream `Settings` struct is never touched.

use std::sync::atomic::{AtomicBool, Ordering};

use egui::Ui;

use crate::LauncherUi;
use crate::widgets::{self, Kind};

pub(crate) mod compact;
pub(crate) mod icons;
mod machine_icons;

/// An on/off choice that survives restarts.
pub(crate) struct Flag {
    key: &'static str,
    value: AtomicBool,
    loaded: AtomicBool,
    default: bool,
}

impl Flag {
    const fn new(key: &'static str, default: bool) -> Self {
        Self { key, value: AtomicBool::new(default), loaded: AtomicBool::new(false), default }
    }

    pub(crate) fn get(&self) -> bool {
        self.value.load(Ordering::Relaxed)
    }

    fn load_once(&self, ctx: &egui::Context) {
        if self.loaded.swap(true, Ordering::Relaxed) {
            return;
        }
        let saved = ctx.data_mut(|d| d.get_persisted::<bool>(egui::Id::new(self.key))).unwrap_or(self.default);
        self.value.store(saved, Ordering::Relaxed);
    }

    fn set(&self, ctx: &egui::Context, on: bool) {
        self.value.store(on, Ordering::Relaxed);
        ctx.data_mut(|d| d.insert_persisted(egui::Id::new(self.key), on));
        ctx.request_repaint();
    }

    /// A toolbar button, lit while on. Returns true when clicked.
    fn button(&self, ui: &mut Ui, label: &str, tip: &str) -> bool {
        let on = self.get();
        let clicked = widgets::button(ui, label, if on { Kind::Primary } else { Kind::Secondary }, 0.0).on_hover_text(tip).clicked();
        if clicked {
            self.set(ui.ctx(), !on);
        }
        clicked
    }
}

pub(crate) static ORIGINAL_ICONS: Flag = Flag::new("bettercraft-original-icons", false);
pub(crate) static COMPACT: Flag = Flag::new("bettercraft-compact", !cfg!(feature = "upstream-look"));

/// The fork's toolbar buttons, drawn after upstream's refresh button. Also keeps the installed
/// apps' icons in step with the choice.
pub(crate) fn toolbar(launcher: &LauncherUi, ui: &mut Ui) {
    ORIGINAL_ICONS.load_once(ui.ctx());
    COMPACT.load_once(ui.ctx());
    let icons_clicked = ORIGINAL_ICONS.button(ui, "Original Icons", "Use the original programs' icons, here and on the installed apps");
    COMPACT.button(ui, "Compact", "A denser list with less detail");
    machine_icons::sync(launcher, icons_clicked);
}
