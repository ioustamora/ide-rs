//! Breadcrumb Navigation Component
//!
//! A navigation component that shows the user's location within a hierarchical structure.
//! Commonly used for file paths, page navigation, or category hierarchies.

use egui::{Ui, Vec2, Color32, Rounding, Stroke, Pos2, Align2, FontId, Sense};
use crate::rcl::ui::component::Component;

/// Breadcrumb navigation component
#[derive(Clone, Debug)]
pub struct Breadcrumb {
    /// Breadcrumb items
    pub items: Vec<BreadcrumbItem>,
    /// Separator between items
    pub separator: String,
    /// Maximum visible items (0 = show all)
    pub max_items: usize,
    /// Whether to show home icon
    pub show_home: bool,
    /// Home icon text
    pub home_icon: String,
    /// Breadcrumb style
    pub style: BreadcrumbStyle,
    /// Text size
    pub text_size: f32,
    /// Item spacing
    pub spacing: f32,
}

/// Individual breadcrumb item
#[derive(Clone, Debug)]
pub struct BreadcrumbItem {
    /// Display text
    pub text: String,
    /// Optional URL or identifier
    pub href: Option<String>,
    /// Whether item is clickable
    pub clickable: bool,
    /// Whether this is the current/active item
    pub active: bool,
    /// Optional icon
    pub icon: Option<String>,
    /// Custom data
    pub data: Option<String>,
}

/// Breadcrumb styling options
#[derive(Clone, Debug, PartialEq)]
pub enum BreadcrumbStyle {
    Default,
    Pills,
    Underlined,
    Minimal,
}

impl Default for Breadcrumb {
    fn default() -> Self {  
        Self {
            items: Vec::new(),
            separator: "/".to_string(),
            max_items: 0,
            show_home: true,
            home_icon: "🏠".to_string(),
            style: BreadcrumbStyle::Default,
            text_size: 14.0,
            spacing: 8.0,
        }
    }
}

impl Default for BreadcrumbItem {
    fn default() -> Self {
        Self {
            text: String::new(),
            href: None,
            clickable: true,
            active: false,
            icon: None,
            data: None,
        }
    }
}

impl Breadcrumb {
    /// Create a new breadcrumb
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an item to the breadcrumb
    pub fn add_item(mut self, text: impl Into<String>) -> Self {
        self.items.push(BreadcrumbItem {
            text: text.into(),
            ..Default::default()
        });
        self
    }

    /// Add a clickable item with href
    pub fn add_link(mut self, text: impl Into<String>, href: impl Into<String>) -> Self {
        self.items.push(BreadcrumbItem {
            text: text.into(),
            href: Some(href.into()),
            clickable: true,
            ..Default::default()
        });
        self
    }

    /// Add an item with icon
    pub fn add_item_with_icon(mut self, text: impl Into<String>, icon: impl Into<String>) -> Self {
        self.items.push(BreadcrumbItem {
            text: text.into(),
            icon: Some(icon.into()),
            ..Default::default()
        });
        self
    }

    /// Set separator
    pub fn separator(mut self, separator: impl Into<String>) -> Self {
        self.separator = separator.into();
        self
    }

    /// Set maximum visible items
    pub fn max_items(mut self, max: usize) -> Self {
        self.max_items = max;
        self
    }

    /// Set breadcrumb style
    pub fn style(mut self, style: BreadcrumbStyle) -> Self {
        self.style = style;
        self
    }

    /// Enable/disable home icon
    pub fn show_home(mut self, show: bool) -> Self {
        self.show_home = show;
        self
    }

    /// Set text size
    pub fn text_size(mut self, size: f32) -> Self {
        self.text_size = size;
        self
    }

    /// Set the last item as active
    pub fn set_active(mut self, index: usize) -> Self {
        for (i, item) in self.items.iter_mut().enumerate() {
            item.active = i == index;
        }
        self
    }

    /// Build breadcrumb from path string
    pub fn from_path(path: &str, separator: &str) -> Self {
        let parts: Vec<&str> = path.split(separator).filter(|s| !s.is_empty()).collect();
        let mut breadcrumb = Self::new().separator(separator);
        
        for (i, part) in parts.iter().enumerate() {
            let is_active = i == parts.len() - 1;
            breadcrumb.items.push(BreadcrumbItem {
                text: part.to_string(),
                active: is_active,
                clickable: !is_active,
                ..Default::default()
            });
        }
        
        breadcrumb
    }

    /// Get visible items (considering max_items)
    fn get_visible_items(&self) -> Vec<&BreadcrumbItem> {
        if self.max_items == 0 || self.items.len() <= self.max_items {
            self.items.iter().collect()
        } else {
            // Show first item, ellipsis, and last (max_items - 2) items
            let mut visible = Vec::new();
            visible.push(&self.items[0]);
            
            if self.items.len() > self.max_items {
                // Add ellipsis placeholder (we'll handle this in rendering)
                let start_idx = self.items.len() - (self.max_items - 2);
                for item in &self.items[start_idx..] {
                    visible.push(item);
                }
            }
            
            visible
        }
    }

    /// Render item based on style
    fn render_item(&self, ui: &mut Ui, item: &BreadcrumbItem, _is_last: bool) -> egui::Response {
        let font_id = FontId::proportional(self.text_size);
        let text_color = if item.active {
            ui.visuals().text_color()
        } else if item.clickable {
            Color32::from_rgb(0, 123, 255) // Link blue
        } else {
            ui.visuals().weak_text_color()
        };

        let display_text = if let Some(icon) = &item.icon {
            format!("{} {}", icon, item.text)
        } else {
            item.text.clone()
        };

        let sense = if item.clickable && !item.active {
            Sense::click()
        } else {
            Sense::hover()
        };

        match self.style {
            BreadcrumbStyle::Default => {
                let response = ui.add(egui::Label::new(
                    egui::RichText::new(display_text)
                        .font(font_id.clone())
                        .color(text_color)
                ).sense(sense));
                
                if response.hovered() && item.clickable && !item.active {
                    ui.painter().text(
                        response.rect.min,
                        Align2::LEFT_TOP,
                        &item.text,
                        font_id,
                        text_color.gamma_multiply(1.2)
                    );
                }
                
                response
            },
            
            BreadcrumbStyle::Pills => {
                let padding = Vec2::new(12.0, 6.0);
                let text_galley = ui.fonts(|f| f.layout_no_wrap(
                    display_text.clone(),
                    font_id.clone(),
                    text_color
                ));
                
                let size = text_galley.size() + padding * 2.0;
                let (rect, response) = ui.allocate_exact_size(size, sense);
                
                let bg_color = if item.active {
                    Color32::from_rgb(0, 123, 255)
                } else if response.hovered() && item.clickable {
                    Color32::from_rgb(248, 249, 250)
                } else {
                    Color32::TRANSPARENT
                };
                
                ui.painter().rect_filled(rect, Rounding::same(size.y * 0.5), bg_color);
                
                let final_text_color = if item.active { Color32::WHITE } else { text_color };
                let final_galley = ui.fonts(|f| f.layout_no_wrap(
                    display_text,
                    font_id,
                    final_text_color
                ));
                ui.painter().galley(
                    rect.center() - final_galley.size() * 0.5,
                    final_galley,
                    final_text_color
                );
                
                response
            },
            
            BreadcrumbStyle::Underlined => {
                let response = ui.add(egui::Label::new(
                    egui::RichText::new(display_text)
                        .font(font_id.clone())
                        .color(text_color)
                ).sense(sense));
                
                if item.active || (response.hovered() && item.clickable) {
                    let underline_y = response.rect.max.y + 2.0;
                    ui.painter().line_segment(
                        [
                            Pos2::new(response.rect.min.x, underline_y),
                            Pos2::new(response.rect.max.x, underline_y)
                        ],
                        Stroke::new(2.0, text_color)
                    );
                }
                
                response
            },
            
            BreadcrumbStyle::Minimal => {
                ui.add(egui::Label::new(
                    egui::RichText::new(display_text)
                        .font(font_id.clone())
                        .color(text_color)
                ).sense(sense))
            }
        }
    }

    /// Render separator
    fn render_separator(&self, ui: &mut Ui) {
        ui.add_space(self.spacing);
        ui.label(egui::RichText::new(&self.separator)
            .font(FontId::proportional(self.text_size))
            .color(ui.visuals().weak_text_color()));
        ui.add_space(self.spacing);
    }

    /// Get clicked item index
    pub fn get_clicked_item(&self, responses: &[egui::Response]) -> Option<usize> {
        for (i, response) in responses.iter().enumerate() {
            if response.clicked() {
                return Some(i);
            }
        }
        None
    }
}

impl Component for Breadcrumb {
    fn name(&self) -> &str {
        "Breadcrumb"
    }

    fn render(&mut self, ui: &mut Ui) {
        let mut responses = Vec::new();
        let mut overall_rect = egui::Rect::NOTHING;

        ui.horizontal(|ui| {
            // Show home icon if enabled
            if self.show_home && !self.items.is_empty() {
                let home_response = ui.add(egui::Label::new(
                    egui::RichText::new(&self.home_icon)
                        .font(FontId::proportional(self.text_size))
                        .color(Color32::from_rgb(0, 123, 255))
                ).sense(Sense::click()));
                
                responses.push(home_response.clone());
                overall_rect = overall_rect.union(home_response.rect);
                
                if !self.items.is_empty() {
                    self.render_separator(ui);
                }
            }

            let visible_items = self.get_visible_items();
            
            // Check if we need to show ellipsis
            let show_ellipsis = self.max_items > 0 && self.items.len() > self.max_items;
            
            for (i, item) in visible_items.iter().enumerate() {
                // Show ellipsis after first item if needed
                if show_ellipsis && i == 1 {
                    ui.label("...");
                    self.render_separator(ui);
                }
                
                let is_last = i == visible_items.len() - 1;
                let response = self.render_item(ui, item, is_last);
                
                responses.push(response.clone());
                overall_rect = overall_rect.union(response.rect);
                
                // Add separator if not last item
                if !is_last {
                    self.render_separator(ui);
                }
            }
        });

        // Don't return response since trait expects ()
    }

    fn get_property(&self, name: &str) -> Option<String> {
        match name {
            "separator" => Some(self.separator.clone()),
            "max_items" => Some(self.max_items.to_string()),
            "show_home" => Some(self.show_home.to_string()),
            "text_size" => Some(self.text_size.to_string()),
            _ => None,
        }
    }

    fn set_property(&mut self, name: &str, value: &str) -> bool {
        match name {
            "separator" => {
                self.separator = value.to_string();
                true
            }
            "max_items" => {
                if let Ok(max_items) = value.parse::<usize>() {
                    self.max_items = max_items;
                    true
                } else {
                    false
                }
            }
            "show_home" => {
                if let Ok(show_home) = value.parse::<bool>() {
                    self.show_home = show_home;
                    true
                } else {
                    false
                }
            }
            "text_size" => {
                if let Ok(text_size) = value.parse::<f32>() {
                    self.text_size = text_size;
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
            "separator".to_string(),
            "max_items".to_string(),
            "show_home".to_string(),
            "text_size".to_string(),
        ]
    }
}

impl BreadcrumbItem {
    /// Create a new breadcrumb item
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    /// Set as clickable with href
    pub fn href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self.clickable = true;
        self
    }

    /// Set as active
    pub fn active(mut self) -> Self {
        self.active = true;
        self.clickable = false;
        self
    }

    /// Add icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set custom data
    pub fn data(mut self, data: impl Into<String>) -> Self {
        self.data = Some(data.into());
        self
    }

    /// Set clickable state
    pub fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }
}