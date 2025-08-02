//! Form Component - Root Container
//!
//! The Form component serves as the root container for all other UI components.
//! It provides a background, layout properties, and acts as the base layer.

use crate::rcl::ui::component::Component;
use egui::{self, Color32, Stroke};
use serde::{Deserialize, Serialize};

/// Form component that serves as the root container
#[derive(Debug, Clone)]
pub struct Form {
    /// Form title/name
    pub title: String,
    /// Background color
    pub background_color: Color32,
    /// Border color
    pub border_color: Color32,
    /// Border width
    pub border_width: f32,
    /// Form size
    pub size: egui::Vec2,
    /// Whether the form has a border
    pub show_border: bool,
    /// Corner radius for rounded corners
    pub corner_radius: f32,
    /// Form padding (internal spacing)
    pub padding: f32,
    /// Whether the form is visible
    pub visible: bool,
    /// Form position (for the designer)
    pub position: egui::Pos2,
}

impl Form {
    /// Create a new form with default properties
    pub fn new(title: String) -> Self {
        Self {
            title,
            background_color: Color32::WHITE,
            border_color: Color32::from_gray(200),
            border_width: 1.0,
            size: egui::Vec2::new(400.0, 300.0),
            show_border: true,
            corner_radius: 4.0,
            padding: 16.0,
            visible: true,
            position: egui::Pos2::ZERO,
        }
    }

    /// Set the background color
    pub fn set_background_color(&mut self, color: Color32) {
        self.background_color = color;
    }

    /// Get the background color
    pub fn background_color(&self) -> Color32 {
        self.background_color
    }

    /// Set the border color
    pub fn set_border_color(&mut self, color: Color32) {
        self.border_color = color;
    }

    /// Get the border color
    pub fn border_color(&self) -> Color32 {
        self.border_color
    }

    /// Set the border width
    pub fn set_border_width(&mut self, width: f32) {
        self.border_width = width;
    }

    /// Get the border width
    pub fn border_width(&self) -> f32 {
        self.border_width
    }

    /// Set whether to show border
    pub fn set_show_border(&mut self, show: bool) {
        self.show_border = show;
    }

    /// Get whether border is shown
    pub fn show_border(&self) -> bool {
        self.show_border
    }

    /// Set the corner radius
    pub fn set_corner_radius(&mut self, radius: f32) {
        self.corner_radius = radius;
    }

    /// Get the corner radius
    pub fn corner_radius(&self) -> f32 {
        self.corner_radius
    }

    /// Set the form size
    pub fn set_size(&mut self, size: egui::Vec2) {
        self.size = size;
    }

    /// Get the form size
    pub fn size(&self) -> egui::Vec2 {
        self.size
    }

    /// Set the form title
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// Get the form title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Set the form padding
    pub fn set_padding(&mut self, padding: f32) {
        self.padding = padding;
    }

    /// Get the form padding
    pub fn padding(&self) -> f32 {
        self.padding
    }

    /// Set visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Get visibility
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Set position
    pub fn set_position(&mut self, position: egui::Pos2) {
        self.position = position;
    }

    /// Get position
    pub fn position(&self) -> egui::Pos2 {
        self.position
    }

    /// Render the form background in a specific rect
    pub fn render_background(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        if !self.visible {
            return;
        }

        let painter = ui.painter();

        // Draw background
        painter.rect_filled(rect, self.corner_radius, self.background_color);

        // Draw border if enabled
        if self.show_border && self.border_width > 0.0 {
            painter.rect_stroke(
                rect,
                self.corner_radius,
                Stroke::new(self.border_width, self.border_color)
            );
        }
    }
}

impl Component for Form {
    fn name(&self) -> &str {
        "Form"
    }

    fn render(&mut self, ui: &mut egui::Ui) {
        if !self.visible {
            return;
        }

        // Get the available rect and render the form background
        let rect = ui.available_rect_before_wrap();
        self.render_background(ui, rect);

        // Add space for the form content
        ui.allocate_space(self.size);
    }

    fn get_property(&self, name: &str) -> Option<String> {
        match name {
            "title" => Some(self.title.clone()),
            "background_color" => Some(format!("{:?}", self.background_color)),
            "border_color" => Some(format!("{:?}", self.border_color)),
            "border_width" => Some(self.border_width.to_string()),
            "show_border" => Some(self.show_border.to_string()),
            "corner_radius" => Some(self.corner_radius.to_string()),
            "width" => Some(self.size.x.to_string()),
            "height" => Some(self.size.y.to_string()),
            "padding" => Some(self.padding.to_string()),
            "visible" => Some(self.visible.to_string()),
            _ => None,
        }
    }

    fn set_property(&mut self, name: &str, value: &str) -> bool {
        match name {
            "title" => {
                self.title = value.to_string();
                true
            }
            "background_color" => {
                // Simple color parsing - in a real implementation, you'd want better parsing
                match value {
                    "white" => self.background_color = Color32::WHITE,
                    "black" => self.background_color = Color32::BLACK,
                    "red" => self.background_color = Color32::RED,
                    "green" => self.background_color = Color32::GREEN,
                    "blue" => self.background_color = Color32::BLUE,
                    "gray" => self.background_color = Color32::GRAY,
                    "light_gray" => self.background_color = Color32::LIGHT_GRAY,
                    _ => return false,
                }
                true
            }
            "border_color" => {
                match value {
                    "white" => self.border_color = Color32::WHITE,
                    "black" => self.border_color = Color32::BLACK,
                    "red" => self.border_color = Color32::RED,
                    "green" => self.border_color = Color32::GREEN,
                    "blue" => self.border_color = Color32::BLUE,
                    "gray" => self.border_color = Color32::GRAY,
                    "light_gray" => self.border_color = Color32::LIGHT_GRAY,
                    _ => return false,
                }
                true
            }
            "border_width" => {
                if let Ok(width) = value.parse::<f32>() {
                    self.border_width = width.max(0.0);
                    true
                } else {
                    false
                }
            }
            "show_border" => {
                if let Ok(show) = value.parse::<bool>() {
                    self.show_border = show;
                    true
                } else {
                    false
                }
            }
            "corner_radius" => {
                if let Ok(radius) = value.parse::<f32>() {
                    self.corner_radius = radius.max(0.0);
                    true
                } else {
                    false
                }
            }
            "width" => {
                if let Ok(width) = value.parse::<f32>() {
                    self.size.x = width.max(100.0);
                    true
                } else {
                    false
                }
            }
            "height" => {
                if let Ok(height) = value.parse::<f32>() {
                    self.size.y = height.max(100.0);
                    true
                } else {
                    false
                }
            }
            "padding" => {
                if let Ok(padding) = value.parse::<f32>() {
                    self.padding = padding.max(0.0);
                    true
                } else {
                    false
                }
            }
            "visible" => {
                if let Ok(visible) = value.parse::<bool>() {
                    self.visible = visible;
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn get_property_names(&self) -> Vec<String> {
        vec![
            "title".to_string(),
            "background_color".to_string(),
            "border_color".to_string(),
            "border_width".to_string(),
            "show_border".to_string(),
            "corner_radius".to_string(),
            "width".to_string(),
            "height".to_string(),
            "padding".to_string(),
            "visible".to_string(),
        ]
    }
}