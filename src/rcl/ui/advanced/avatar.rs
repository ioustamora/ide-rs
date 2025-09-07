//! Avatar Component
//!
//! A component for displaying user profile pictures, initials, or placeholder icons.
//! Supports various sizes, shapes, and fallback behaviors.

use egui::{Ui, Vec2, Color32, Rounding, Stroke, Pos2, Align2, FontId};
use crate::rcl::ui::component::Component;

/// Avatar component for profile pictures and user representation
#[derive(Clone, Debug)]
pub struct Avatar {
    /// User's name for initials fallback
    pub name: String,
    /// Image URL or path (if available)
    pub image_url: Option<String>,
    /// Avatar size
    pub size: AvatarSize,
    /// Avatar shape
    pub shape: AvatarShape,
    /// Background color for initials
    pub background_color: Option<Color32>,
    /// Text color for initials
    pub text_color: Color32,
    /// Whether to show border
    pub show_border: bool,
    /// Border color
    pub border_color: Color32,
    /// Border width
    pub border_width: f32,
    /// Status indicator
    pub status: Option<AvatarStatus>,
    /// Whether avatar is clickable
    pub clickable: bool,
    /// Tooltip text
    pub tooltip: Option<String>,
}

/// Avatar size options
#[derive(Clone, Debug, PartialEq)]
pub enum AvatarSize {
    ExtraSmall, // 24px
    Small,      // 32px
    Medium,     // 40px
    Large,      // 56px
    ExtraLarge, // 80px
    Custom(f32),
}

/// Avatar shape options
#[derive(Clone, Debug, PartialEq)]
pub enum AvatarShape {
    Circle,
    Square,
    RoundedSquare(f32), // Corner radius
}

/// Status indicator for avatar
#[derive(Clone, Debug, PartialEq)]
pub struct AvatarStatus {
    /// Status type
    pub status_type: StatusType,
    /// Status position
    pub position: StatusPosition,
    /// Custom status color
    pub custom_color: Option<Color32>,
}

/// Status type options
#[derive(Clone, Debug, PartialEq)]
pub enum StatusType {
    Online,
    Offline,
    Away,
    Busy,
    DoNotDisturb,
    Custom(String),
}

/// Status indicator position
#[derive(Clone, Debug, PartialEq)]
pub enum StatusPosition {
    BottomRight,
    BottomLeft,
    TopRight,
    TopLeft,
}

impl Default for Avatar {
    fn default() -> Self {
        Self {
            name: "User".to_string(),
            image_url: None,
            size: AvatarSize::Medium,
            shape: AvatarShape::Circle,
            background_color: None,
            text_color: Color32::WHITE,
            show_border: false,
            border_color: Color32::GRAY,
            border_width: 2.0,
            status: None,
            clickable: false,
            tooltip: None,
        }
    }
}

impl Avatar {
    /// Create a new avatar with name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Set avatar size
    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    /// Set avatar shape
    pub fn shape(mut self, shape: AvatarShape) -> Self {
        self.shape = shape;
        self
    }

    /// Set image URL
    pub fn image(mut self, url: impl Into<String>) -> Self {
        self.image_url = Some(url.into());
        self
    }

    /// Set background color
    pub fn background_color(mut self, color: Color32) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Set text color
    pub fn text_color(mut self, color: Color32) -> Self {
        self.text_color = color;
        self
    }

    /// Enable border
    pub fn with_border(mut self, color: Color32, width: f32) -> Self {
        self.show_border = true;
        self.border_color = color;
        self.border_width = width;
        self
    }

    /// Set status indicator
    pub fn status(mut self, status: AvatarStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Make avatar clickable
    pub fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }

    /// Set tooltip
    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Get avatar dimensions
    fn get_size(&self) -> f32 {
        match self.size {
            AvatarSize::ExtraSmall => 24.0,
            AvatarSize::Small => 32.0,
            AvatarSize::Medium => 40.0,
            AvatarSize::Large => 56.0,
            AvatarSize::ExtraLarge => 80.0,
            AvatarSize::Custom(size) => size,
        }
    }

    /// Get font size for initials
    fn get_font_size(&self) -> f32 {
        self.get_size() * 0.4
    }

    /// Generate background color from name
    fn generate_background_color(&self) -> Color32 {
        if let Some(color) = self.background_color {
            return color;
        }

        // Generate color based on name hash
        let mut hash = 0u32;
        for byte in self.name.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u32);
        }

        let colors = [
            Color32::from_rgb(244, 67, 54),   // Red
            Color32::from_rgb(233, 30, 99),   // Pink
            Color32::from_rgb(156, 39, 176),  // Purple
            Color32::from_rgb(103, 58, 183),  // Deep Purple
            Color32::from_rgb(63, 81, 181),   // Indigo
            Color32::from_rgb(33, 150, 243),  // Blue
            Color32::from_rgb(3, 169, 244),   // Light Blue
            Color32::from_rgb(0, 188, 212),   // Cyan
            Color32::from_rgb(0, 150, 136),   // Teal
            Color32::from_rgb(76, 175, 80),   // Green
            Color32::from_rgb(139, 195, 74),  // Light Green
            Color32::from_rgb(205, 220, 57),  // Lime
            Color32::from_rgb(255, 235, 59),  // Yellow
            Color32::from_rgb(255, 193, 7),   // Amber
            Color32::from_rgb(255, 152, 0),   // Orange
            Color32::from_rgb(255, 87, 34),   // Deep Orange
        ];

        colors[(hash as usize) % colors.len()]
    }

    /// Get initials from name
    fn get_initials(&self) -> String {
        let words: Vec<&str> = self.name.split_whitespace().collect();
        match words.len() {
            0 => "?".to_string(),
            1 => words[0].chars().take(2).collect::<String>().to_uppercase(),
            _ => {
                let first = words[0].chars().next().unwrap_or('?');
                let last = words[words.len() - 1].chars().next().unwrap_or('?');
                format!("{}{}", first, last).to_uppercase()
            }
        }
    }

    /// Get rounding based on shape
    fn get_rounding(&self) -> Rounding {
        match self.shape {
            AvatarShape::Circle => Rounding::same(self.get_size() * 0.5),
            AvatarShape::Square => Rounding::ZERO,
            AvatarShape::RoundedSquare(radius) => Rounding::same(radius),
        }
    }

    /// Get status color
    fn get_status_color(status_type: &StatusType, custom_color: Option<Color32>) -> Color32 {
        if let Some(color) = custom_color {
            return color;
        }

        match status_type {
            StatusType::Online => Color32::from_rgb(76, 175, 80),
            StatusType::Offline => Color32::from_rgb(158, 158, 158),
            StatusType::Away => Color32::from_rgb(255, 193, 7),
            StatusType::Busy => Color32::from_rgb(244, 67, 54),
            StatusType::DoNotDisturb => Color32::from_rgb(156, 39, 176),
            StatusType::Custom(_) => Color32::from_rgb(33, 150, 243),
        }
    }

    /// Render status indicator
    fn render_status(&self, ui: &mut Ui, avatar_rect: egui::Rect) {
        if let Some(status) = &self.status {
            let status_size = self.get_size() * 0.25;
            let status_offset = status_size * 0.5;

            let status_pos = match status.position {
                StatusPosition::BottomRight => Pos2::new(
                    avatar_rect.max.x - status_offset,
                    avatar_rect.max.y - status_offset
                ),
                StatusPosition::BottomLeft => Pos2::new(
                    avatar_rect.min.x + status_offset,
                    avatar_rect.max.y - status_offset
                ),
                StatusPosition::TopRight => Pos2::new(
                    avatar_rect.max.x - status_offset,
                    avatar_rect.min.y + status_offset
                ),
                StatusPosition::TopLeft => Pos2::new(
                    avatar_rect.min.x + status_offset,
                    avatar_rect.min.y + status_offset
                ),
            };

            let _status_rect = egui::Rect::from_center_size(
                status_pos,
                Vec2::splat(status_size)
            );

            let status_color = Self::get_status_color(&status.status_type, status.custom_color);

            // Draw status background (white border)
            ui.painter().circle_filled(
                status_pos,
                status_size * 0.6,
                Color32::WHITE
            );

            // Draw status indicator
            ui.painter().circle_filled(
                status_pos,
                status_size * 0.4,
                status_color
            );
        }
    }
}

impl Component for Avatar {
    fn name(&self) -> &str {
        "Avatar"
    }

    fn render(&mut self, ui: &mut Ui) {
        let size = self.get_size();
        let avatar_size = Vec2::splat(size);

        let (rect, mut response) = ui.allocate_exact_size(
            avatar_size,
            if self.clickable { egui::Sense::click() } else { egui::Sense::hover() }
        );

        let bg_color = self.generate_background_color();
        let rounding = self.get_rounding();

        // Draw border if enabled
        if self.show_border {
            ui.painter().rect_stroke(
                rect,
                rounding,
                Stroke::new(self.border_width, self.border_color)
            );
        }

        // Draw avatar background
        ui.painter().rect_filled(rect, rounding, bg_color);

        // TODO: Load and display image if available
        // For now, show initials
        if self.image_url.is_some() {
            // Placeholder for image loading
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                "IMG",
                FontId::proportional(self.get_font_size() * 0.6),
                self.text_color.gamma_multiply(0.7)
            );
        } else {
            // Show initials
            let initials = self.get_initials();
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                initials,
                FontId::proportional(self.get_font_size()),
                self.text_color
            );
        }

        // Render status indicator
        self.render_status(ui, rect);

        // Add tooltip if provided
        if let Some(tooltip_text) = &self.tooltip {
            response = response.on_hover_text(tooltip_text);
        }

        // Handle hover effects
        if response.hovered() && self.clickable {
            // Add hover effect for clickable avatars
            ui.painter().rect_stroke(
                rect,
                rounding,
                Stroke::new(2.0, Color32::from_rgb(100, 149, 237))
            );
        }

        // Don't return response since trait expects ()
    }

    fn get_property(&self, name: &str) -> Option<String> {
        match name {
            "name" => Some(self.name.clone()),
            "size" => Some(format!("{:?}", self.size)),
            "shape" => Some(format!("{:?}", self.shape)),
            "clickable" => Some(self.clickable.to_string()),
            "show_border" => Some(self.show_border.to_string()),
            _ => None,
        }
    }

    fn set_property(&mut self, name: &str, value: &str) -> bool {
        match name {
            "name" => {
                self.name = value.to_string();
                true
            }
            "clickable" => {
                if let Ok(clickable) = value.parse::<bool>() {
                    self.clickable = clickable;
                    true
                } else {
                    false
                }
            }
            "show_border" => {
                if let Ok(show_border) = value.parse::<bool>() {
                    self.show_border = show_border;
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
            "name".to_string(),
            "size".to_string(),
            "shape".to_string(),
            "clickable".to_string(),
            "show_border".to_string(),
        ]
    }
}

impl AvatarStatus {
    /// Create online status
    pub fn online() -> Self {
        Self {
            status_type: StatusType::Online,
            position: StatusPosition::BottomRight,
            custom_color: None,
        }
    }

    /// Create offline status
    pub fn offline() -> Self {
        Self {
            status_type: StatusType::Offline,
            position: StatusPosition::BottomRight,
            custom_color: None,
        }
    }

    /// Create away status
    pub fn away() -> Self {
        Self {
            status_type: StatusType::Away,
            position: StatusPosition::BottomRight,
            custom_color: None,
        }
    }

    /// Create busy status
    pub fn busy() -> Self {
        Self {
            status_type: StatusType::Busy,
            position: StatusPosition::BottomRight,
            custom_color: None,
        }
    }

    /// Set status position
    pub fn position(mut self, position: StatusPosition) -> Self {
        self.position = position;
        self
    }

    /// Set custom color
    pub fn color(mut self, color: Color32) -> Self {
        self.custom_color = Some(color);
        self
    }
}