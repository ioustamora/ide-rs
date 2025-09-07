//! Badge Component
//!
//! A small count or status indicator that appears near another element.
//! Commonly used for notifications, status indicators, or counters.

use egui::{Ui, Vec2, Color32, Rounding, Stroke, Pos2, FontId};
use crate::rcl::ui::component::Component;

/// Badge component for status indicators and counters
#[derive(Clone, Debug)]
pub struct Badge {
    /// Badge text content
    pub text: String,
    /// Badge variant/style
    pub variant: BadgeVariant,
    /// Badge size
    pub size: BadgeSize,
    /// Whether the badge is visible
    pub visible: bool,
    /// Position relative to parent element
    pub position: BadgePosition,
    /// Custom background color (overrides variant color)
    pub custom_color: Option<Color32>,
    /// Whether badge should pulse/animate
    pub animated: bool,
}

/// Badge style variants
#[derive(Clone, Debug, PartialEq)]
pub enum BadgeVariant {
    Primary,
    Secondary,
    Success,
    Warning,
    Error,
    Info,
    Light,
    Dark,
}

/// Badge size options
#[derive(Clone, Debug, PartialEq)]
pub enum BadgeSize {
    Small,
    Medium,
    Large,
}

/// Badge positioning options
#[derive(Clone, Debug, PartialEq)]
pub enum BadgePosition {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    Inline,
}

impl Default for Badge {
    fn default() -> Self {
        Self {
            text: "1".to_string(),
            variant: BadgeVariant::Primary,
            size: BadgeSize::Medium,
            visible: true,
            position: BadgePosition::TopRight,
            custom_color: None,
            animated: false,
        }
    }
}

impl Badge {
    /// Create a new badge with text
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    /// Set badge variant
    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set badge size
    pub fn size(mut self, size: BadgeSize) -> Self {
        self.size = size;
        self
    }

    /// Set badge position
    pub fn position(mut self, position: BadgePosition) -> Self {
        self.position = position;
        self
    }

    /// Set custom color
    pub fn color(mut self, color: Color32) -> Self {
        self.custom_color = Some(color);
        self
    }

    /// Enable animation
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Set visibility
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Get variant color
    fn get_color(&self) -> Color32 {
        if let Some(color) = self.custom_color {
            return color;
        }

        match self.variant {
            BadgeVariant::Primary => Color32::from_rgb(0, 123, 255),
            BadgeVariant::Secondary => Color32::from_rgb(108, 117, 125),
            BadgeVariant::Success => Color32::from_rgb(40, 167, 69),
            BadgeVariant::Warning => Color32::from_rgb(255, 193, 7),
            BadgeVariant::Error => Color32::from_rgb(220, 53, 69),
            BadgeVariant::Info => Color32::from_rgb(23, 162, 184),
            BadgeVariant::Light => Color32::from_rgb(248, 249, 250),
            BadgeVariant::Dark => Color32::from_rgb(52, 58, 64),
        }
    }

    /// Get text color based on background
    fn get_text_color(&self) -> Color32 {
        match self.variant {
            BadgeVariant::Light => Color32::from_rgb(52, 58, 64),
            BadgeVariant::Warning => Color32::from_rgb(52, 58, 64),
            _ => Color32::WHITE,
        }
    }

    /// Get size dimensions
    fn get_size_params(&self) -> (f32, f32, FontId) {
        match self.size {
            BadgeSize::Small => (6.0, 12.0, FontId::proportional(10.0)),
            BadgeSize::Medium => (8.0, 16.0, FontId::proportional(12.0)),
            BadgeSize::Large => (10.0, 20.0, FontId::proportional(14.0)),
        }
    }

    /// Render badge at specific position
    pub fn render_at_position(&mut self, ui: &mut Ui, parent_rect: egui::Rect) -> egui::Response {
        if !self.visible {
            return ui.allocate_response(Vec2::ZERO, egui::Sense::hover());
        }

        let (padding, min_height, font_id) = self.get_size_params();
        let text_galley = ui.fonts(|f| f.layout_no_wrap(
            self.text.clone(),
            font_id.clone(),
            self.get_text_color()
        ));

        let badge_size = Vec2::new(
            (text_galley.size().x + padding * 2.0).max(min_height),
            min_height
        );

        let badge_pos = match self.position {
            BadgePosition::TopRight => Pos2::new(
                parent_rect.max.x - badge_size.x * 0.5,
                parent_rect.min.y + badge_size.y * 0.5
            ),
            BadgePosition::TopLeft => Pos2::new(
                parent_rect.min.x + badge_size.x * 0.5,
                parent_rect.min.y + badge_size.y * 0.5
            ),
            BadgePosition::BottomRight => Pos2::new(
                parent_rect.max.x - badge_size.x * 0.5,
                parent_rect.max.y - badge_size.y * 0.5
            ),
            BadgePosition::BottomLeft => Pos2::new(
                parent_rect.min.x + badge_size.x * 0.5,
                parent_rect.max.y - badge_size.y * 0.5
            ),
            BadgePosition::Inline => parent_rect.center(),
        };

        let badge_rect = egui::Rect::from_center_size(badge_pos, badge_size);

        // Apply animation if enabled
        let bg_color = if self.animated {
            let time = ui.ctx().input(|i| i.time) as f32;
            let pulse = (time * 2.0).sin() * 0.2 + 0.8;
            let base_color = self.get_color();
            Color32::from_rgba_unmultiplied(
                (base_color.r() as f32 * pulse) as u8,
                (base_color.g() as f32 * pulse) as u8,
                (base_color.b() as f32 * pulse) as u8,
                base_color.a()
            )
        } else {
            self.get_color()
        };

        // Draw badge background
        ui.painter().rect_filled(
            badge_rect,
            Rounding::same(badge_size.y * 0.5),
            bg_color
        );

        // Draw badge border
        ui.painter().rect_stroke(
            badge_rect,
            Rounding::same(badge_size.y * 0.5),
            Stroke::new(1.0, bg_color.gamma_multiply(0.8))
        );

        // Draw text
        ui.painter().galley(
            badge_rect.center() - text_galley.size() * 0.5,
            text_galley,
            self.get_text_color()
        );

        ui.allocate_rect(badge_rect, egui::Sense::hover())
    }
}

impl Component for Badge {
    fn name(&self) -> &str {
        "Badge"
    }

    fn render(&mut self, ui: &mut Ui) {
        if !self.visible {
            return;
        }

        let (padding, min_height, font_id) = self.get_size_params();
        let text_galley = ui.fonts(|f| f.layout_no_wrap(
            self.text.clone(),
            font_id.clone(),
            self.get_text_color()
        ));

        let badge_size = Vec2::new(
            (text_galley.size().x + padding * 2.0).max(min_height),
            min_height
        );

        let (rect, _response) = ui.allocate_exact_size(badge_size, egui::Sense::hover());

        // Apply animation if enabled
        let bg_color = if self.animated {
            let time = ui.ctx().input(|i| i.time) as f32;
            let pulse = (time * 2.0).sin() * 0.2 + 0.8;
            let base_color = self.get_color();
            Color32::from_rgba_unmultiplied(
                (base_color.r() as f32 * pulse) as u8,
                (base_color.g() as f32 * pulse) as u8,
                (base_color.b() as f32 * pulse) as u8,
                base_color.a()
            )
        } else {
            self.get_color()
        };

        // Draw badge background
        ui.painter().rect_filled(
            rect,
            Rounding::same(badge_size.y * 0.5),
            bg_color
        );

        // Draw badge border
        ui.painter().rect_stroke(
            rect,
            Rounding::same(badge_size.y * 0.5),
            Stroke::new(1.0, bg_color.gamma_multiply(0.8))
        );

        // Draw text
        ui.painter().galley(
            rect.center() - text_galley.size() * 0.5,
            text_galley,
            self.get_text_color()
        );

        // Don't return response since trait expects ()
    }

    fn get_property(&self, name: &str) -> Option<String> {
        match name {
            "text" => Some(self.text.clone()),
            "variant" => Some(format!("{:?}", self.variant)),
            "size" => Some(format!("{:?}", self.size)),
            "visible" => Some(self.visible.to_string()),
            "animated" => Some(self.animated.to_string()),
            _ => None,
        }
    }

    fn set_property(&mut self, name: &str, value: &str) -> bool {
        match name {
            "text" => {
                self.text = value.to_string();
                true
            }
            "visible" => {
                if let Ok(visible) = value.parse::<bool>() {
                    self.visible = visible;
                    true
                } else {
                    false
                }
            }
            "animated" => {
                if let Ok(animated) = value.parse::<bool>() {
                    self.animated = animated;
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
            "text".to_string(),
            "variant".to_string(),
            "size".to_string(),
            "visible".to_string(),
            "animated".to_string(),
        ]
    }
}