//! # RCL Property System
//!
//! This module provides a comprehensive property system for RCL components,
//! including standardized property categories, types, validation, and metadata.
//!
//! ## Design Philosophy
//!
//! The property system is designed to provide:
//! - **Consistency**: All components follow the same property naming conventions
//! - **Type Safety**: Properties have defined types with validation
//! - **Extensibility**: Components can add their own unique properties
//! - **IDE Integration**: Rich metadata for property inspectors
//! - **Performance**: Efficient property access and validation

use egui::{Color32, Vec2, Pos2};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Property categories for organizing component properties
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PropertyCategory {
    /// Content properties (text, value, items)
    Content,
    /// Layout properties (position, size, spacing)
    Layout,
    /// Visual styling properties (colors, fonts, borders)
    Style,
    /// Behavior properties (enabled, editable, events)
    Behavior,
    /// Accessibility properties (ARIA labels, roles)
    Accessibility,
    /// Component-specific properties
    Custom(String),
}

/// Property data types supported by the system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PropertyType {
    /// String text value
    Text,
    /// Integer number
    Integer,
    /// Floating point number
    Float,
    /// Boolean true/false
    Boolean,
    /// Color value (hex, rgb, named)
    Color,
    /// 2D vector (width/height, x/y)
    Vector2,
    /// 2D position
    Position,
    /// List of string options
    StringList,
    /// Enumeration with specific values
    Enum(Vec<String>),
    /// File path
    FilePath,
    /// URL
    Url,
}

/// Property constraints for validation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropertyConstraints {
    /// Minimum value (for numbers)
    pub min: Option<f64>,
    /// Maximum value (for numbers)
    pub max: Option<f64>,
    /// Step value (for sliders)
    pub step: Option<f64>,
    /// Required property (cannot be empty)
    pub required: bool,
    /// Pattern validation (regex)
    pub pattern: Option<String>,
    /// Allowed values (for enums)
    pub allowed_values: Option<Vec<String>>,
}

/// Property metadata for IDE integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyInfo {
    /// Property name (used as key)
    pub name: String,
    /// Display name for UI
    pub display_name: String,
    /// Property category
    pub category: PropertyCategory,
    /// Property type
    pub property_type: PropertyType,
    /// Default value as string
    pub default_value: String,
    /// Description for tooltips/help
    pub description: String,
    /// Validation constraints
    pub constraints: Option<PropertyConstraints>,
    /// Whether property is read-only
    pub read_only: bool,
    /// Whether property affects layout
    pub affects_layout: bool,
}

/// Property value that can hold different types
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    Text(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Color(Color32),
    Vector2(Vec2),
    Position(Pos2),
    StringList(Vec<String>),
    None,
}

impl PropertyValue {
    /// Convert property value to string for serialization
    pub fn to_string(&self) -> String {
        match self {
            PropertyValue::Text(s) => s.clone(),
            PropertyValue::Integer(i) => i.to_string(),
            PropertyValue::Float(f) => f.to_string(),
            PropertyValue::Boolean(b) => b.to_string(),
            PropertyValue::Color(c) => format!("#{:02x}{:02x}{:02x}{:02x}", c.r(), c.g(), c.b(), c.a()),
            PropertyValue::Vector2(v) => format!("{},{}", v.x, v.y),
            PropertyValue::Position(p) => format!("{},{}", p.x, p.y),
            PropertyValue::StringList(list) => list.join(","),
            PropertyValue::None => String::new(),
        }
    }

    /// Parse property value from string
    pub fn from_string(s: &str, property_type: &PropertyType) -> Result<PropertyValue, String> {
        if s.is_empty() {
            return Ok(PropertyValue::None);
        }

        match property_type {
            PropertyType::Text => Ok(PropertyValue::Text(s.to_string())),
            PropertyType::Integer => {
                s.parse::<i64>()
                    .map(PropertyValue::Integer)
                    .map_err(|_| format!("Invalid integer: {}", s))
            }
            PropertyType::Float => {
                s.parse::<f64>()
                    .map(PropertyValue::Float)
                    .map_err(|_| format!("Invalid float: {}", s))
            }
            PropertyType::Boolean => {
                s.parse::<bool>()
                    .map(PropertyValue::Boolean)
                    .map_err(|_| format!("Invalid boolean: {}", s))
            }
            PropertyType::Color => {
                parse_color(s).map(PropertyValue::Color)
            }
            PropertyType::Vector2 => {
                parse_vector2(s).map(PropertyValue::Vector2)
            }
            PropertyType::Position => {
                parse_position(s).map(PropertyValue::Position)
            }
            PropertyType::StringList => {
                Ok(PropertyValue::StringList(
                    s.split(',').map(|s| s.trim().to_string()).collect()
                ))
            }
            PropertyType::Enum(allowed) => {
                if allowed.contains(&s.to_string()) {
                    Ok(PropertyValue::Text(s.to_string()))
                } else {
                    Err(format!("Invalid enum value: {}. Allowed: {:?}", s, allowed))
                }
            }
            PropertyType::FilePath | PropertyType::Url => {
                Ok(PropertyValue::Text(s.to_string()))
            }
        }
    }

    /// Validate property value against constraints
    pub fn validate(&self, constraints: &PropertyConstraints) -> Result<(), String> {
        if constraints.required && matches!(self, PropertyValue::None) {
            return Err("Property is required".to_string());
        }

        match self {
            PropertyValue::Integer(i) => {
                if let Some(min) = constraints.min {
                    if (*i as f64) < min {
                        return Err(format!("Value {} is below minimum {}", i, min));
                    }
                }
                if let Some(max) = constraints.max {
                    if (*i as f64) > max {
                        return Err(format!("Value {} is above maximum {}", i, max));
                    }
                }
            }
            PropertyValue::Float(f) => {
                if let Some(min) = constraints.min {
                    if *f < min {
                        return Err(format!("Value {} is below minimum {}", f, min));
                    }
                }
                if let Some(max) = constraints.max {
                    if *f > max {
                        return Err(format!("Value {} is above maximum {}", f, max));
                    }
                }
            }
            PropertyValue::Text(s) => {
                if let Some(allowed) = &constraints.allowed_values {
                    if !allowed.contains(s) {
                        return Err(format!("Invalid value: {}. Allowed: {:?}", s, allowed));
                    }
                }
            }
            _ => {} // Other types don't have validation yet
        }

        Ok(())
    }
}

/// Parse color from string (hex, rgb, or named colors)
fn parse_color(s: &str) -> Result<Color32, String> {
    let s = s.trim();
    
    // Hex color (#RRGGBB or #RRGGBBAA)
    if s.starts_with('#') {
        let hex = &s[1..];
        match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid hex color")?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid hex color")?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid hex color")?;
                Ok(Color32::from_rgb(r, g, b))
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid hex color")?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid hex color")?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid hex color")?;
                let a = u8::from_str_radix(&hex[6..8], 16).map_err(|_| "Invalid hex color")?;
                Ok(Color32::from_rgba_unmultiplied(r, g, b, a))
            }
            _ => Err("Invalid hex color format".to_string())
        }
    }
    // Named colors
    else {
        match s.to_lowercase().as_str() {
            "black" => Ok(Color32::BLACK),
            "white" => Ok(Color32::WHITE),
            "red" => Ok(Color32::RED),
            "green" => Ok(Color32::GREEN),
            "blue" => Ok(Color32::BLUE),
            "yellow" => Ok(Color32::YELLOW),
            "cyan" => Ok(Color32::LIGHT_BLUE),
            "magenta" => Ok(Color32::from_rgb(255, 0, 255)),
            "gray" | "grey" => Ok(Color32::GRAY),
            "darkgray" | "darkgrey" => Ok(Color32::DARK_GRAY),
            "lightgray" | "lightgrey" => Ok(Color32::LIGHT_GRAY),
            "transparent" => Ok(Color32::TRANSPARENT),
            _ => Err(format!("Unknown color name: {}", s))
        }
    }
}

/// Parse Vec2 from string "x,y"
fn parse_vector2(s: &str) -> Result<Vec2, String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err("Vector2 must be in format 'x,y'".to_string());
    }
    
    let x = parts[0].trim().parse::<f32>()
        .map_err(|_| "Invalid x coordinate")?;
    let y = parts[1].trim().parse::<f32>()
        .map_err(|_| "Invalid y coordinate")?;
    
    Ok(Vec2::new(x, y))
}

/// Parse Pos2 from string "x,y"
fn parse_position(s: &str) -> Result<Pos2, String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err("Position must be in format 'x,y'".to_string());
    }
    
    let x = parts[0].trim().parse::<f32>()
        .map_err(|_| "Invalid x coordinate")?;
    let y = parts[1].trim().parse::<f32>()
        .map_err(|_| "Invalid y coordinate")?;
    
    Ok(Pos2::new(x, y))
}

/// Property registry for managing component property metadata
pub struct PropertyRegistry {
    properties: HashMap<String, PropertyInfo>,
}

impl PropertyRegistry {
    /// Create a new property registry
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    /// Register a property with metadata
    pub fn register(&mut self, property: PropertyInfo) {
        self.properties.insert(property.name.clone(), property);
    }

    /// Get property metadata by name
    pub fn get(&self, name: &str) -> Option<&PropertyInfo> {
        self.properties.get(name)
    }

    /// Get all properties in a category
    pub fn get_by_category(&self, category: &PropertyCategory) -> Vec<&PropertyInfo> {
        self.properties.values()
            .filter(|p| &p.category == category)
            .collect()
    }

    /// Get all property names
    pub fn get_names(&self) -> Vec<String> {
        self.properties.keys().cloned().collect()
    }
}

impl Default for PropertyRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Standard property definitions for all components
pub mod standard_properties {
    use super::*;

    /// Create standard content properties
    pub fn content_properties() -> Vec<PropertyInfo> {
        vec![
            PropertyInfo {
                name: "text".to_string(),
                display_name: "Text".to_string(),
                category: PropertyCategory::Content,
                property_type: PropertyType::Text,
                default_value: String::new(),
                description: "Text content displayed by the component".to_string(),
                constraints: None,
                read_only: false,
                affects_layout: true,
            },
            PropertyInfo {
                name: "value".to_string(),
                display_name: "Value".to_string(),
                category: PropertyCategory::Content,
                property_type: PropertyType::Text,
                default_value: String::new(),
                description: "Current value of the component".to_string(),
                constraints: None,
                read_only: false,
                affects_layout: false,
            },
        ]
    }

    /// Create standard layout properties
    pub fn layout_properties() -> Vec<PropertyInfo> {
        vec![
            PropertyInfo {
                name: "x".to_string(),
                display_name: "X Position".to_string(),
                category: PropertyCategory::Layout,
                property_type: PropertyType::Float,
                default_value: "0".to_string(),
                description: "Horizontal position of the component".to_string(),
                constraints: Some(PropertyConstraints {
                    min: Some(0.0),
                    max: None,
                    step: Some(1.0),
                    required: false,
                    pattern: None,
                    allowed_values: None,
                }),
                read_only: false,
                affects_layout: true,
            },
            PropertyInfo {
                name: "y".to_string(),
                display_name: "Y Position".to_string(),
                category: PropertyCategory::Layout,
                property_type: PropertyType::Float,
                default_value: "0".to_string(),
                description: "Vertical position of the component".to_string(),
                constraints: Some(PropertyConstraints {
                    min: Some(0.0),
                    max: None,
                    step: Some(1.0),
                    required: false,
                    pattern: None,
                    allowed_values: None,
                }),
                read_only: false,
                affects_layout: true,
            },
            PropertyInfo {
                name: "width".to_string(),
                display_name: "Width".to_string(),
                category: PropertyCategory::Layout,
                property_type: PropertyType::Float,
                default_value: "100".to_string(),
                description: "Width of the component in pixels".to_string(),
                constraints: Some(PropertyConstraints {
                    min: Some(1.0),
                    max: Some(10000.0),
                    step: Some(1.0),
                    required: false,
                    pattern: None,
                    allowed_values: None,
                }),
                read_only: false,
                affects_layout: true,
            },
            PropertyInfo {
                name: "height".to_string(),
                display_name: "Height".to_string(),
                category: PropertyCategory::Layout,
                property_type: PropertyType::Float,
                default_value: "30".to_string(),
                description: "Height of the component in pixels".to_string(),
                constraints: Some(PropertyConstraints {
                    min: Some(1.0),
                    max: Some(10000.0),
                    step: Some(1.0),
                    required: false,
                    pattern: None,
                    allowed_values: None,
                }),
                read_only: false,
                affects_layout: true,
            },
            PropertyInfo {
                name: "margin".to_string(),
                display_name: "Margin".to_string(),
                category: PropertyCategory::Layout,
                property_type: PropertyType::Float,
                default_value: "0".to_string(),
                description: "Margin around the component".to_string(),
                constraints: Some(PropertyConstraints {
                    min: Some(0.0),
                    max: Some(100.0),
                    step: Some(1.0),
                    required: false,
                    pattern: None,
                    allowed_values: None,
                }),
                read_only: false,
                affects_layout: true,
            },
            PropertyInfo {
                name: "padding".to_string(),
                display_name: "Padding".to_string(),
                category: PropertyCategory::Layout,
                property_type: PropertyType::Float,
                default_value: "5".to_string(),
                description: "Padding inside the component".to_string(),
                constraints: Some(PropertyConstraints {
                    min: Some(0.0),
                    max: Some(100.0),
                    step: Some(1.0),
                    required: false,
                    pattern: None,
                    allowed_values: None,
                }),
                read_only: false,
                affects_layout: true,
            },
        ]
    }

    /// Create standard style properties
    pub fn style_properties() -> Vec<PropertyInfo> {
        vec![
            PropertyInfo {
                name: "color".to_string(),
                display_name: "Text Color".to_string(),
                category: PropertyCategory::Style,
                property_type: PropertyType::Color,
                default_value: "#000000".to_string(),
                description: "Color of the text".to_string(),
                constraints: None,
                read_only: false,
                affects_layout: false,
            },
            PropertyInfo {
                name: "background_color".to_string(),
                display_name: "Background Color".to_string(),
                category: PropertyCategory::Style,
                property_type: PropertyType::Color,
                default_value: "#ffffff".to_string(),
                description: "Background color of the component".to_string(),
                constraints: None,
                read_only: false,
                affects_layout: false,
            },
            PropertyInfo {
                name: "border_color".to_string(),
                display_name: "Border Color".to_string(),
                category: PropertyCategory::Style,
                property_type: PropertyType::Color,
                default_value: "#cccccc".to_string(),
                description: "Color of the border".to_string(),
                constraints: None,
                read_only: false,
                affects_layout: false,
            },
            PropertyInfo {
                name: "border_width".to_string(),
                display_name: "Border Width".to_string(),
                category: PropertyCategory::Style,
                property_type: PropertyType::Float,
                default_value: "1".to_string(),
                description: "Width of the border in pixels".to_string(),
                constraints: Some(PropertyConstraints {
                    min: Some(0.0),
                    max: Some(20.0),
                    step: Some(0.5),
                    required: false,
                    pattern: None,
                    allowed_values: None,
                }),
                read_only: false,
                affects_layout: false,
            },
            PropertyInfo {
                name: "corner_radius".to_string(),
                display_name: "Corner Radius".to_string(),
                category: PropertyCategory::Style,
                property_type: PropertyType::Float,
                default_value: "3".to_string(),
                description: "Radius of rounded corners".to_string(),
                constraints: Some(PropertyConstraints {
                    min: Some(0.0),
                    max: Some(50.0),
                    step: Some(1.0),
                    required: false,
                    pattern: None,
                    allowed_values: None,
                }),
                read_only: false,
                affects_layout: false,
            },
            PropertyInfo {
                name: "font_size".to_string(),
                display_name: "Font Size".to_string(),
                category: PropertyCategory::Style,
                property_type: PropertyType::Float,
                default_value: "14".to_string(),
                description: "Size of the font in pixels".to_string(),
                constraints: Some(PropertyConstraints {
                    min: Some(6.0),
                    max: Some(72.0),
                    step: Some(1.0),
                    required: false,
                    pattern: None,
                    allowed_values: None,
                }),
                read_only: false,
                affects_layout: true,
            },
            PropertyInfo {
                name: "opacity".to_string(),
                display_name: "Opacity".to_string(),
                category: PropertyCategory::Style,
                property_type: PropertyType::Float,
                default_value: "1.0".to_string(),
                description: "Opacity of the component (0.0 = transparent, 1.0 = opaque)".to_string(),
                constraints: Some(PropertyConstraints {
                    min: Some(0.0),
                    max: Some(1.0),
                    step: Some(0.1),
                    required: false,
                    pattern: None,
                    allowed_values: None,
                }),
                read_only: false,
                affects_layout: false,
            },
        ]
    }

    /// Create standard behavior properties
    pub fn behavior_properties() -> Vec<PropertyInfo> {
        vec![
            PropertyInfo {
                name: "visible".to_string(),
                display_name: "Visible".to_string(),
                category: PropertyCategory::Behavior,
                property_type: PropertyType::Boolean,
                default_value: "true".to_string(),
                description: "Whether the component is visible".to_string(),
                constraints: None,
                read_only: false,
                affects_layout: true,
            },
            PropertyInfo {
                name: "enabled".to_string(),
                display_name: "Enabled".to_string(),
                category: PropertyCategory::Behavior,
                property_type: PropertyType::Boolean,
                default_value: "true".to_string(),
                description: "Whether the component can be interacted with".to_string(),
                constraints: None,
                read_only: false,
                affects_layout: false,
            },
            PropertyInfo {
                name: "editable".to_string(),
                display_name: "Editable".to_string(),
                category: PropertyCategory::Behavior,
                property_type: PropertyType::Boolean,
                default_value: "false".to_string(),
                description: "Whether the component is in edit mode".to_string(),
                constraints: None,
                read_only: false,
                affects_layout: false,
            },
            PropertyInfo {
                name: "tooltip".to_string(),
                display_name: "Tooltip".to_string(),
                category: PropertyCategory::Behavior,
                property_type: PropertyType::Text,
                default_value: String::new(),
                description: "Tooltip text shown on hover".to_string(),
                constraints: None,
                read_only: false,
                affects_layout: false,
            },
            PropertyInfo {
                name: "tab_index".to_string(),
                display_name: "Tab Index".to_string(),
                category: PropertyCategory::Behavior,
                property_type: PropertyType::Integer,
                default_value: "0".to_string(),
                description: "Tab order for keyboard navigation".to_string(),
                constraints: Some(PropertyConstraints {
                    min: Some(0.0),
                    max: Some(1000.0),
                    step: Some(1.0),
                    required: false,
                    pattern: None,
                    allowed_values: None,
                }),
                read_only: false,
                affects_layout: false,
            },
        ]
    }

    /// Create standard accessibility properties
    pub fn accessibility_properties() -> Vec<PropertyInfo> {
        vec![
            PropertyInfo {
                name: "aria_label".to_string(),
                display_name: "ARIA Label".to_string(),
                category: PropertyCategory::Accessibility,
                property_type: PropertyType::Text,
                default_value: String::new(),
                description: "Accessible label for screen readers".to_string(),
                constraints: None,
                read_only: false,
                affects_layout: false,
            },
            PropertyInfo {
                name: "aria_describedby".to_string(),
                display_name: "ARIA Described By".to_string(),
                category: PropertyCategory::Accessibility,
                property_type: PropertyType::Text,
                default_value: String::new(),
                description: "ID of element that describes this component".to_string(),
                constraints: None,
                read_only: false,
                affects_layout: false,
            },
            PropertyInfo {
                name: "role".to_string(),
                display_name: "ARIA Role".to_string(),
                category: PropertyCategory::Accessibility,
                property_type: PropertyType::Enum(vec![
                    "button".to_string(),
                    "textbox".to_string(),
                    "checkbox".to_string(),
                    "slider".to_string(),
                    "menuitem".to_string(),
                    "tab".to_string(),
                    "tabpanel".to_string(),
                    "dialog".to_string(),
                    "alert".to_string(),
                    "status".to_string(),
                ]),
                default_value: String::new(),
                description: "Semantic role for accessibility".to_string(),
                constraints: None,
                read_only: false,
                affects_layout: false,
            },
        ]
    }

    /// Get all standard properties
    pub fn all_standard_properties() -> Vec<PropertyInfo> {
        let mut properties = Vec::new();
        properties.extend(content_properties());
        properties.extend(layout_properties());
        properties.extend(style_properties());
        properties.extend(behavior_properties());
        properties.extend(accessibility_properties());
        properties
    }
}