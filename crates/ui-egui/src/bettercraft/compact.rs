//! The compact grid: one slim row per app (icon, name, one status line, the main button), in place
//! of upstream's tall cards with their version strip, GitHub link, description and date.
//!
//! Hooked from the top of upstream's `grid_block`; returns false (draw upstream's cards) while
//! [`super::COMPACT`] is off.

use artcraft_launcher_core::status::AppStatus;
use egui::{CornerRadius, FontId, Rect, RichText, Sense, Ui, vec2};

use crate::LauncherUi;
use crate::i18n::{current, fmt, tr_ctx};
use crate::model::{self, AppEntry};
use crate::theme::{self, Tokens, rgb};
use crate::widgets;
use artcraft_launcher_core::details::AppDetails;

const ROW_MIN_W: f32 = 460.0;
const ROW_H: f32 = 64.0;
const PAD: f32 = 14.0;
const ICON: f32 = 40.0;
const ACTIONS_W: f32 = 190.0;

/// Draws `ids` as compact rows when compact mode is on; false means upstream should draw them.
pub(crate) fn grid_block(launcher: &mut LauncherUi, ui: &mut Ui, ids: &[&'static str]) -> bool {
    if !super::COMPACT.get() {
        return false;
    }
    if ids.is_empty() {
        return true;
    }
    let width = ui.available_width();
    let cols = ((width / ROW_MIN_W).floor() as usize).clamp(1, 3);
    // Neighbours share their 1 px hairline.
    let cell_w = (width + (cols as f32 - 1.0)) / cols as f32;
    let rows = ids.len().div_ceil(cols);
    let (area, _) = ui.allocate_exact_size(vec2(width, rows as f32 * (ROW_H - 1.0) + 1.0), Sense::hover());
    for (i, id) in ids.iter().enumerate() {
        let (row, col) = (i / cols, i % cols);
        let min = area.min + vec2(col as f32 * (cell_w - 1.0), row as f32 * (ROW_H - 1.0));
        row_ui(launcher, ui, id, Rect::from_min_size(min, vec2(cell_w, ROW_H)));
    }
    true
}

fn row_ui(launcher: &mut LauncherUi, ui: &mut Ui, id: &str, rect: Rect) {
    let Some(entry) = launcher.entry(id).cloned() else { return };
    let t = Tokens::get(ui.ctx());
    let app = entry.app;
    let status = entry.status(launcher.platform, launcher.caps.manage_apps);
    let installed = status.is_installed();
    let hovered = ui.rect_contains_pointer(rect);

    if AppDetails::by_id(id).is_some() {
        let open = ui.interact(rect, ui.id().with(("compact-open", id)), Sense::click());
        open.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, fmt(tl!("{name} details"), &[("name", app.name)])));
        if open.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
            launcher.detail = Some(id.to_string());
        }
    }
    let fill = if hovered { t.bg.lerp_to_gamma(rgb(app.color), 0.04) } else { t.bg };
    ui.painter().rect(rect, CornerRadius::ZERO, fill, t.hairline(), egui::StrokeKind::Inside);

    let inner = rect.shrink2(vec2(PAD, 0.0));
    let icon_rect = Rect::from_center_size(egui::pos2(inner.left() + ICON / 2.0, inner.center().y), vec2(ICON, ICON));
    if let Some(icon) = crate::icons::app_image(ui.ctx(), app.id, installed || hovered) {
        icon.paint_at(ui, icon_rect);
    }

    let gap = ui.spacing().item_spacing.x;
    let actions_w = ACTIONS_W + gap + theme::CONTROL_H;
    let actions = Rect::from_min_max(egui::pos2(inner.right() - actions_w, inner.top()), inner.max);
    let text = Rect::from_min_max(egui::pos2(icon_rect.right() + 12.0, inner.top()), egui::pos2(actions.left() - 8.0, inner.bottom()));

    let mut text_ui = ui.new_child(egui::UiBuilder::new().max_rect(text).layout(egui::Layout::top_down(egui::Align::Min)));
    text_ui.set_clip_rect(text.intersect(ui.clip_rect()));
    text_ui.add_space((ROW_H - 36.0) / 2.0);
    text_ui.spacing_mut().item_spacing.y = 2.0;
    text_ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;
        ui.add(egui::Label::new(RichText::new(app.name).font(FontId::new(15.0, theme::semibold())).color(t.ink_strong)).selectable(false));
        let tagline = tr_ctx(current(), app.id, app.tagline);
        ui.add(egui::Label::new(RichText::new(tagline).size(12.0).color(t.faint)).selectable(false).truncate());
    });
    let (line, color) = status_line(&entry, &status, launcher.caps.manage_apps, &t);
    text_ui.add(egui::Label::new(RichText::new(line).size(12.0).color(color)).selectable(false).truncate());

    let mut actions_ui = ui.new_child(
        egui::UiBuilder::new().max_rect(actions).layout(egui::Layout::left_to_right(egui::Align::Center).with_main_align(egui::Align::Max)),
    );
    launcher.actions(&mut actions_ui, &entry, &status, actions_w);
}

/// One quiet line: the version when all is well, the status (in its tone) when something is up.
fn status_line(entry: &AppEntry, status: &AppStatus, manage_apps: bool, t: &Tokens) -> (String, egui::Color32) {
    let version = crate::card_parts::strip_version(entry, status);
    match status {
        AppStatus::UpToDate | AppStatus::Installed if !version.is_empty() => (version, t.muted),
        // The button already says "Update to…": the line only shows from and to.
        AppStatus::UpdateAvailable if !version.is_empty() => (version, t.warn),
        AppStatus::Available if !version.is_empty() => (version, t.faint),
        _ => {
            let (text, tone) = model::entry_status_text(entry, status, manage_apps);
            let text = if version.is_empty() { text } else { format!("{text} · {version}") };
            (text, widgets::tone_color(t, tone))
        }
    }
}
