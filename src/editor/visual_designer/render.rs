use egui;
use crate::editor::visual_designer::{GuideSystem, GridSettings, ResizeHandle, ComponentSelection};

/// Rendering logic for Visual Designer
impl GuideSystem {
    pub fn draw_guides(&self, ui: &mut egui::Ui, canvas_rect: egui::Rect) {
        let painter = ui.painter();
        for &y in &self.horizontal_guides {
            painter.line_segment(
                [egui::pos2(canvas_rect.min.x, y), egui::pos2(canvas_rect.max.x, y)],
                egui::Stroke::new(1.0, self.guide_color)
            );
        }
        for &x in &self.vertical_guides {
            painter.line_segment(
                [egui::pos2(x, canvas_rect.min.y), egui::pos2(x, canvas_rect.max.y)],
                egui::Stroke::new(1.0, self.guide_color)
            );
        }
    }

    pub fn draw_rulers(&self, ui: &mut egui::Ui, canvas_rect: egui::Rect) {
        let painter = ui.painter();
        let ruler_size = 20.0;
        let h_ruler_rect = egui::Rect::from_min_size(
            egui::pos2(canvas_rect.min.x, canvas_rect.min.y - ruler_size),
            egui::vec2(canvas_rect.width(), ruler_size)
        );
        painter.rect_filled(h_ruler_rect, 0.0, self.ruler_color);
        let v_ruler_rect = egui::Rect::from_min_size(
            egui::pos2(canvas_rect.min.x - ruler_size, canvas_rect.min.y),
            egui::vec2(ruler_size, canvas_rect.height())
        );
        painter.rect_filled(v_ruler_rect, 0.0, self.ruler_color);
        Self::draw_ruler_marks(ui, h_ruler_rect, v_ruler_rect);
    }

    fn draw_ruler_marks(ui: &mut egui::Ui, h_ruler: egui::Rect, v_ruler: egui::Rect) {
        let painter = ui.painter();
        let mark_interval = 50.0;
        let mut x = h_ruler.min.x;
        let mut pixel_count = 0;
        while x <= h_ruler.max.x {
            let mark_height = if pixel_count % 100 == 0 { 8.0 } else { 4.0 };
            painter.line_segment(
                [egui::pos2(x, h_ruler.max.y - mark_height), egui::pos2(x, h_ruler.max.y)],
                egui::Stroke::new(1.0, egui::Color32::WHITE)
            );
            if pixel_count % 100 == 0 {
                painter.text(
                    egui::pos2(x + 2.0, h_ruler.max.y - 12.0),
                    egui::Align2::LEFT_BOTTOM,
                    pixel_count.to_string(),
                    egui::FontId::monospace(8.0),
                    egui::Color32::WHITE
                );
            }
            x += mark_interval;
            pixel_count += mark_interval as i32;
        }
        let mut y = v_ruler.min.y;
        pixel_count = 0;
        while y <= v_ruler.max.y {
            let mark_width = if pixel_count % 100 == 0 { 8.0 } else { 4.0 };
            painter.line_segment(
                [egui::pos2(v_ruler.max.x - mark_width, y), egui::pos2(v_ruler.max.x, y)],
                egui::Stroke::new(1.0, egui::Color32::WHITE)
            );
            if pixel_count % 100 == 0 {
                painter.text(
                    egui::pos2(v_ruler.max.x - 2.0, y - 2.0),
                    egui::Align2::RIGHT_BOTTOM,
                    pixel_count.to_string(),
                    egui::FontId::monospace(8.0),
                    egui::Color32::WHITE
                );
            }
            y += mark_interval;
            pixel_count += mark_interval as i32;
        }
    }
}

impl GridSettings {
    pub fn draw_grid(&self, ui: &mut egui::Ui, canvas_rect: egui::Rect) {
        let painter = ui.painter();
        let grid_size = self.size.max(1.0);
        let max_lines = 1000;
        let h_lines = ((canvas_rect.width() / grid_size) as i32).min(max_lines);
        let v_lines = ((canvas_rect.height() / grid_size) as i32).min(max_lines);
        for i in 0..=h_lines {
            let x = canvas_rect.min.x + (i as f32 * grid_size);
            if x > canvas_rect.max.x { break; }
            let is_major = i % (self.major_lines as i32) == 0;
            let color = if is_major {
                self.color.gamma_multiply(1.8)
            } else {
                self.color
            };
            painter.line_segment(
                [egui::pos2(x, canvas_rect.min.y), egui::pos2(x, canvas_rect.max.y)],
                egui::Stroke::new(if is_major { 1.2 } else { 0.6 }, color)
            );
        }
        for i in 0..=v_lines {
            let y = canvas_rect.min.y + (i as f32 * grid_size);
            if y > canvas_rect.max.y { break; }
            let is_major = i % (self.major_lines as i32) == 0;
            let color = if is_major {
                self.color.gamma_multiply(1.8)
            } else {
                self.color
            };
            painter.line_segment(
                [egui::pos2(canvas_rect.min.x, y), egui::pos2(canvas_rect.max.x, y)],
                egui::Stroke::new(if is_major { 1.2 } else { 0.6 }, color)
            );
        }
        if canvas_rect.contains(egui::pos2(canvas_rect.min.x, canvas_rect.min.y)) {
            painter.circle_filled(
                egui::pos2(canvas_rect.min.x, canvas_rect.min.y),
                3.0,
                egui::Color32::from_rgb(255, 100, 100)
            );
        }
    }
}

impl ComponentSelection {
    pub fn draw_selection_indicators(&self, ui: &mut egui::Ui, layout: &crate::editor::visual_designer::LayoutManager) {
        let painter = ui.painter();
        if let Some(selection_rect) = self.selection_rect {
            painter.rect_stroke(
                selection_rect,
                0.0,
                egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(0, 120, 255, 128))
            );
            painter.rect_filled(
                selection_rect,
                0.0,
                egui::Color32::from_rgba_unmultiplied(0, 120, 255, 32)
            );
        }
        if let Some(hover_idx) = self.hover_component {
            if !self.selected.contains(&hover_idx) {
                if let Some(pos) = layout.positions.get(&hover_idx) {
                    let size = layout.sizes.get(&hover_idx).cloned().unwrap_or(egui::vec2(100.0, 30.0));
                    let rect = egui::Rect::from_min_size(*pos, size);
                    painter.rect_stroke(
                        rect.expand(1.0),
                        1.0,
                        egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(0, 120, 255, 128))
                    );
                }
            }
        }
        for &component_idx in &self.selected {
            if let Some(pos) = layout.positions.get(&component_idx) {
                let size = layout.sizes.get(&component_idx).cloned().unwrap_or(egui::vec2(100.0, 30.0));
                let rect = egui::Rect::from_min_size(*pos, size);
                let is_primary = Some(component_idx) == self.primary;
                let selection_color = if is_primary {
                    egui::Color32::from_rgb(0, 120, 255)
                } else {
                    egui::Color32::from_rgb(120, 120, 255)
                };
                painter.rect_stroke(
                    rect.expand(2.0),
                    2.0,
                    egui::Stroke::new(2.0, selection_color)
                );
                if is_primary {
                    Self::draw_resize_handles(&painter, rect);
                }
                // Drag shadow (optional, can be moved to drag logic)
            }
        }
    }

    fn draw_resize_handles(painter: &egui::Painter, rect: egui::Rect) {
        let handle_size = 6.0;
        let handle_color = egui::Color32::from_rgb(0, 120, 255);
        let handles = [
            rect.min,
            egui::pos2(rect.max.x, rect.min.y),
            rect.max,
            egui::pos2(rect.min.x, rect.max.y),
            egui::pos2(rect.center().x, rect.min.y),
            egui::pos2(rect.max.x, rect.center().y),
            egui::pos2(rect.center().x, rect.max.y),
            egui::pos2(rect.min.x, rect.center().y),
        ];
        for handle_pos in handles {
            let handle_rect = egui::Rect::from_center_size(handle_pos, egui::vec2(handle_size, handle_size));
            painter.rect_filled(handle_rect, 1.0, handle_color);
            painter.rect_stroke(handle_rect, 1.0, egui::Stroke::new(1.0, egui::Color32::WHITE));
        }
    }
}
use egui;

/// Visual guides and rulers system
pub struct GuideSystem {
    /// Horizontal guides
    pub horizontal_guides: Vec<f32>,
    /// Vertical guides
    pub vertical_guides: Vec<f32>,
    /// Whether rulers are visible
    pub rulers_visible: bool,
    /// Ruler color
    pub ruler_color: egui::Color32,
    /// Guide color
    pub guide_color: egui::Color32,
}

impl Default for GuideSystem {
    fn default() -> Self {
        Self {
            horizontal_guides: Vec::new(),
            vertical_guides: Vec::new(),
            rulers_visible: true,
            ruler_color: egui::Color32::from_rgb(100, 100, 100),
            guide_color: egui::Color32::from_rgb(0, 150, 255),
        }
    }
}
//! Rendering logic for Visual Designer
//!
//! Handles grid, guides, rulers, and component rendering.

// TODO: Move all rendering-related logic here.

pub struct RenderEngine;

impl RenderEngine {
    pub fn new() -> Self { Self }
    // ...rendering methods...
}
