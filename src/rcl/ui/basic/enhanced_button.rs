//! # Enhanced Button Component
//!
//! This module provides an enhanced button component that demonstrates the new
//! standardized property system. It serves as a reference implementation for
//! how to create components with the enhanced property system.

use egui::{Ui, Response, Color32};
use crate::rcl::ui::{
    enhanced_component::{EnhancedComponent, StandardProperties},
    properties::{PropertyInfo, PropertyValue, PropertyCategory, PropertyType, PropertyConstraints},
};

/// Enhanced Button component with standardized properties
/// 
/// This button demonstrates the new property system with:
/// - Standard layout, style, and behavior properties
/// - Custom button-specific properties
/// - Type-safe property access and validation
/// - Rich property metadata for IDE integration
pub struct EnhancedButton {
    /// Standard properties (layout, style, behavior, accessibility)
    pub standard_props: StandardProperties,
    
    /// Button text
    pub label: String,
    
    /// Button click state (for internal tracking)
    pub clicked: bool,
    
    /// Button style variant
    pub style: ButtonStyle,
    
    /// Icon to display on button (optional)
    pub icon: Option<String>,
    
    /// Icon position relative to text
    pub icon_position: IconPosition,
}

/// Button style variants
#[derive(Debug, Clone, PartialEq)]
pub enum ButtonStyle {
    /// Standard button
    Primary,
    /// Secondary button
    Secondary,
    /// Success button (green)
    Success,
    /// Warning button (orange)
    Warning,
    /// Danger button (red)
    Danger,
    /// Ghost button (transparent background)
    Ghost,
    /// Link button (text only)
    Link,
}

/// Icon position relative to text
#[derive(Debug, Clone, PartialEq)]
pub enum IconPosition {
    /// No icon
    None,
    /// Icon to the left of text
    Left,
    /// Icon to the right of text
    Right,
    /// Icon above text
    Top,
    /// Icon below text
    Bottom,
    /// Icon only (no text)
    Only,
}

impl EnhancedButton {
    /// Create a new enhanced button with default properties
    pub fn new(label: String) -> Self {
        let mut standard_props = StandardProperties::default();
        
        // Set button-specific defaults
        standard_props.width = 120.0;
        standard_props.height = 32.0;
        standard_props.padding = 8.0;
        standard_props.corner_radius = 4.0;
        standard_props.background_color = Color32::from_rgb(70, 130, 200);
        standard_props.color = Color32::WHITE;
        standard_props.role = "button".to_string();
        
        Self {
            standard_props,
            label,
            clicked: false,
            style: ButtonStyle::Primary,
            icon: None,
            icon_position: IconPosition::None,
        }
    }
    
    /// Create a button with a specific style
    pub fn with_style(label: String, style: ButtonStyle) -> Self {
        let mut button = Self::new(label);
        button.style = style;
        button.apply_style_colors();
        button
    }
    
    /// Create a button with an icon
    pub fn with_icon(label: String, icon: String, position: IconPosition) -> Self {
        let mut button = Self::new(label);
        button.icon = Some(icon);
        button.icon_position = position;
        button
    }
    
    /// Apply style-specific colors
    fn apply_style_colors(&mut self) {
        match self.style {
            ButtonStyle::Primary => {
                self.standard_props.background_color = Color32::from_rgb(70, 130, 200);
                self.standard_props.color = Color32::WHITE;
            }
            ButtonStyle::Secondary => {
                self.standard_props.background_color = Color32::from_rgb(108, 117, 125);
                self.standard_props.color = Color32::WHITE;
            }
            ButtonStyle::Success => {
                self.standard_props.background_color = Color32::from_rgb(40, 167, 69);
                self.standard_props.color = Color32::WHITE;
            }
            ButtonStyle::Warning => {
                self.standard_props.background_color = Color32::from_rgb(255, 193, 7);
                self.standard_props.color = Color32::BLACK;
            }
            ButtonStyle::Danger => {
                self.standard_props.background_color = Color32::from_rgb(220, 53, 69);
                self.standard_props.color = Color32::WHITE;
            }
            ButtonStyle::Ghost => {
                self.standard_props.background_color = Color32::TRANSPARENT;
                self.standard_props.color = Color32::from_rgb(70, 130, 200);
                self.standard_props.border_width = 1.0;
                self.standard_props.border_color = Color32::from_rgb(70, 130, 200);
            }
            ButtonStyle::Link => {
                self.standard_props.background_color = Color32::TRANSPARENT;
                self.standard_props.color = Color32::from_rgb(70, 130, 200);
                self.standard_props.border_width = 0.0;
            }
        }
    }
    
    /// Check if button was clicked in the last render
    pub fn was_clicked(&self) -> bool {
        self.clicked
    }
    
    /// Set button style
    pub fn set_style(&mut self, style: ButtonStyle) {
        self.style = style;
        self.apply_style_colors();
    }
    
    /// Set button icon
    pub fn set_icon(&mut self, icon: Option<String>, position: IconPosition) {
        self.icon = icon;
        self.icon_position = position;
    }
}

impl EnhancedComponent for EnhancedButton {
    fn standard_properties(&self) -> &StandardProperties {
        &self.standard_props
    }
    
    fn standard_properties_mut(&mut self) -> &mut StandardProperties {
        &mut self.standard_props
    }
    
    fn custom_properties(&self) -> Vec<PropertyInfo> {
        vec![
            PropertyInfo {
                name: "label".to_string(),
                display_name: "Button Text".to_string(),
                category: PropertyCategory::Content,
                property_type: PropertyType::Text,
                default_value: "Button".to_string(),
                description: "Text displayed on the button".to_string(),
                constraints: None,
                read_only: false,
                affects_layout: true,
            },
            PropertyInfo {
                name: "style".to_string(),
                display_name: "Button Style".to_string(),
                category: PropertyCategory::Style,
                property_type: PropertyType::Enum(vec![
                    "Primary".to_string(),
                    "Secondary".to_string(),
                    "Success".to_string(),
                    "Warning".to_string(),
                    "Danger".to_string(),
                    "Ghost".to_string(),
                    "Link".to_string(),
                ]),
                default_value: "Primary".to_string(),
                description: "Visual style variant of the button".to_string(),
                constraints: None,
                read_only: false,
                affects_layout: false,
            },
            PropertyInfo {
                name: "icon".to_string(),
                display_name: "Icon".to_string(),
                category: PropertyCategory::Content,
                property_type: PropertyType::Text,
                default_value: String::new(),
                description: "Icon to display on the button (emoji or symbol)".to_string(),
                constraints: None,
                read_only: false,
                affects_layout: true,
            },
            PropertyInfo {
                name: "icon_position".to_string(),
                display_name: "Icon Position".to_string(),
                category: PropertyCategory::Style,
                property_type: PropertyType::Enum(vec![
                    "None".to_string(),
                    "Left".to_string(),
                    "Right".to_string(),
                    "Top".to_string(),
                    "Bottom".to_string(),
                    "Only".to_string(),
                ]),
                default_value: "None".to_string(),
                description: "Position of the icon relative to the text".to_string(),
                constraints: None,
                read_only: false,
                affects_layout: true,
            },
            PropertyInfo {
                name: "clicked".to_string(),
                display_name: "Was Clicked".to_string(),
                category: PropertyCategory::Behavior,
                property_type: PropertyType::Boolean,
                default_value: "false".to_string(),
                description: "Whether the button was clicked in the last frame (read-only)".to_string(),
                constraints: None,
                read_only: true,
                affects_layout: false,
            },
        ]
    }
    
    fn get_custom_property_value(&self, name: &str) -> Option<PropertyValue> {
        match name {
            "label" => Some(PropertyValue::Text(self.label.clone())),
            "style" => Some(PropertyValue::Text(format!("{:?}", self.style))),
            "icon" => Some(PropertyValue::Text(self.icon.as_ref().unwrap_or(&String::new()).clone())),
            "icon_position" => Some(PropertyValue::Text(format!("{:?}", self.icon_position))),
            "clicked" => Some(PropertyValue::Boolean(self.clicked)),
            _ => None,
        }
    }
    
    fn set_custom_property_value(&mut self, name: &str, value: PropertyValue) -> Result<(), String> {
        match name {
            "label" => {
                if let PropertyValue::Text(text) = value {
                    self.label = text;
                    Ok(())
                } else {
                    Err("Invalid type for label property".to_string())
                }
            }
            "style" => {
                if let PropertyValue::Text(style_str) = value {
                    match style_str.as_str() {
                        "Primary" => {
                            self.set_style(ButtonStyle::Primary);
                            Ok(())
                        }
                        "Secondary" => {
                            self.set_style(ButtonStyle::Secondary);
                            Ok(())
                        }
                        "Success" => {
                            self.set_style(ButtonStyle::Success);
                            Ok(())
                        }
                        "Warning" => {
                            self.set_style(ButtonStyle::Warning);
                            Ok(())
                        }
                        "Danger" => {
                            self.set_style(ButtonStyle::Danger);
                            Ok(())
                        }
                        "Ghost" => {
                            self.set_style(ButtonStyle::Ghost);
                            Ok(())
                        }
                        "Link" => {
                            self.set_style(ButtonStyle::Link);
                            Ok(())
                        }
                        _ => Err("Invalid button style".to_string()),
                    }
                } else {
                    Err("Invalid type for style property".to_string())
                }
            }
            "icon" => {
                if let PropertyValue::Text(icon_str) = value {
                    self.icon = if icon_str.is_empty() { None } else { Some(icon_str) };
                    Ok(())
                } else {
                    Err("Invalid type for icon property".to_string())
                }
            }
            "icon_position" => {
                if let PropertyValue::Text(pos_str) = value {
                    match pos_str.as_str() {
                        "None" => {
                            self.icon_position = IconPosition::None;
                            Ok(())
                        }
                        "Left" => {
                            self.icon_position = IconPosition::Left;
                            Ok(())
                        }
                        "Right" => {
                            self.icon_position = IconPosition::Right;
                            Ok(())
                        }
                        "Top" => {
                            self.icon_position = IconPosition::Top;
                            Ok(())
                        }
                        "Bottom" => {
                            self.icon_position = IconPosition::Bottom;
                            Ok(())
                        }
                        "Only" => {
                            self.icon_position = IconPosition::Only;
                            Ok(())
                        }
                        _ => Err("Invalid icon position".to_string()),
                    }
                } else {
                    Err("Invalid type for icon_position property".to_string())
                }
            }
            "clicked" => {
                Err("clicked property is read-only".to_string())
            }
            _ => Err("Unknown property".to_string()),
        }
    }
    
    fn render_content(&mut self, ui: &mut Ui) {
        self.clicked = false; // Reset click state
        
        let props = self.standard_properties();
        
        // Create button text based on icon and text
        let button_text = match (&self.icon, &self.icon_position) {
            (Some(icon), IconPosition::Left) => format!("{} {}", icon, self.label),
            (Some(icon), IconPosition::Right) => format!("{} {}", self.label, icon),
            (Some(icon), IconPosition::Only) => icon.clone(),
            (Some(icon), IconPosition::Top) => format!("{}\n{}", icon, self.label),
            (Some(icon), IconPosition::Bottom) => format!("{}\n{}", self.label, icon),
            _ => self.label.clone(),
        };
        
        // Create the button
        let button = egui::Button::new(&button_text)
            .min_size(egui::Vec2::new(props.width - props.padding * 2.0, props.height - props.padding * 2.0));
        
        // Render the button and check for clicks
        let response = ui.add(button);
        
        if response.clicked() {
            self.clicked = true;
        }
        
        // Handle hover effects for ghost and link styles
        if matches!(self.style, ButtonStyle::Ghost | ButtonStyle::Link) && response.hovered() {
            // Could add hover color changes here
        }
    }
    
    fn name(&self) -> &str {
        "EnhancedButton"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_enhanced_button_creation() {
        let button = EnhancedButton::new("Test Button".to_string());
        assert_eq!(button.label, "Test Button");
        assert_eq!(button.style, ButtonStyle::Primary);
        assert!(!button.clicked);
    }
    
    #[test]
    fn test_button_properties() {
        let mut button = EnhancedButton::new("Test".to_string());
        
        // Test property access
        let label_value = button.get_custom_property_value("label");
        assert!(matches!(label_value, Some(PropertyValue::Text(ref s)) if s == "Test"));
        
        // Test property setting
        let result = button.set_custom_property_value("label", PropertyValue::Text("New Label".to_string()));
        assert!(result.is_ok());
        assert_eq!(button.label, "New Label");
    }
    
    #[test]
    fn test_button_styles() {
        let mut button = EnhancedButton::new("Test".to_string());
        
        // Test style changes
        button.set_style(ButtonStyle::Success);
        assert_eq!(button.style, ButtonStyle::Success);
        assert_eq!(button.standard_props.background_color, Color32::from_rgb(40, 167, 69));
    }
    
    #[test]
    fn test_custom_properties() {
        let button = EnhancedButton::new("Test".to_string());
        let custom_props = button.custom_properties();
        
        // Should have label, style, icon, icon_position, and clicked properties
        assert_eq!(custom_props.len(), 5);
        
        let label_prop = custom_props.iter().find(|p| p.name == "label");
        assert!(label_prop.is_some());
        assert_eq!(label_prop.unwrap().category, PropertyCategory::Content);
    }
}