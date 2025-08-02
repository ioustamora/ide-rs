//! Component Template Definition
//!
//! Core template structures and property definitions for the template system.

use egui::*;
use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

/// Component template definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentTemplate {
    /// Template unique identifier
    pub id: String,
    /// Template display name
    pub name: String,
    /// Template description
    pub description: String,
    /// Template category
    pub category: String,
    /// Template version
    pub version: String,
    /// Parent template ID (for inheritance)
    pub parent_template: Option<String>,
    /// Component properties definition
    pub properties: BTreeMap<String, TemplateProperty>,
    /// Default component layout
    pub layout: TemplateLayout,
    /// Template creation metadata
    pub metadata: TemplateMetadata,
    /// Visual appearance settings
    pub visual_settings: VisualSettings,
    /// Behavioral settings
    pub behavior_settings: BehaviorSettings,
    /// Template usage statistics
    pub usage_stats: UsageStatistics,
}

/// Template property definition with inheritance support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateProperty {
    /// Property name
    pub name: String,
    /// Property type
    pub property_type: PropertyType,
    /// Default value
    pub default_value: PropertyValue,
    /// Whether property can be overridden in child templates
    pub overridable: bool,
    /// Whether property is inherited from parent
    pub inherited: bool,
    /// Property validation rules
    pub validation: PropertyValidation,
    /// Property description/documentation
    pub description: String,
    /// Property category for grouping
    pub category: String,
    /// Property visibility in designer
    pub designer_visible: bool,
}

/// Property type system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PropertyType {
    String,
    Integer,
    Float,
    Boolean,
    Color,
    Font,
    Size,
    Position,
    Alignment,
    List(Vec<String>),
    Custom(String),
}

/// Property value container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyValue {
    String(String),
    Integer(i32),
    Float(f32),
    Boolean(bool),
    Color([u8; 4]),
    List(Vec<String>),
    Custom(String),
}

/// Property validation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyValidation {
    /// Property is required
    pub required: bool,
    /// Minimum value (for numbers)
    pub min_value: Option<f32>,
    /// Maximum value (for numbers)
    pub max_value: Option<f32>,
    /// Valid string pattern (regex)
    pub pattern: Option<String>,
    /// Valid options (for enums)
    pub valid_options: Option<Vec<String>>,
    /// Custom validation expression
    pub custom_validation: Option<String>,
}

/// Template layout definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateLayout {
    /// Layout type
    pub layout_type: LayoutType,
    /// Child component templates
    pub children: Vec<ChildTemplate>,
    /// Layout constraints
    pub constraints: LayoutConstraints,
    /// Responsive behavior
    pub responsive: ResponsiveSettings,
}

/// Layout type options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayoutType {
    Fixed,
    Flow,
    Grid,
    Flex,
    Stack,
    Absolute,
}

/// Child component in template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildTemplate {
    /// Child template ID
    pub template_id: String,
    /// Position within parent
    pub position: (f32, f32),
    /// Size constraints
    pub size: (f32, f32),
    /// Z-order
    pub z_index: i32,
    /// Visibility
    pub visible: bool,
    /// Property overrides
    pub property_overrides: BTreeMap<String, PropertyValue>,
}

/// Layout constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutConstraints {
    /// Minimum size
    pub min_size: Option<(f32, f32)>,
    /// Maximum size
    pub max_size: Option<(f32, f32)>,
    /// Padding
    pub padding: (f32, f32, f32, f32),
    /// Margins
    pub margin: (f32, f32, f32, f32),
    /// Alignment
    pub alignment: (i32, i32),
}

/// Responsive settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsiveSettings {
    /// Enable responsive behavior
    pub enabled: bool,
    /// Breakpoints
    pub breakpoints: Vec<Breakpoint>,
    /// Responsive layout rules
    pub layout_rules: Vec<ResponsiveRule>,
}

/// Responsive breakpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breakpoint {
    /// Breakpoint name
    pub name: String,
    /// Minimum width
    pub min_width: f32,
    /// Maximum width
    pub max_width: Option<f32>,
}

/// Responsive layout rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsiveRule {
    /// Target breakpoint
    pub breakpoint: String,
    /// Property modifications
    pub property_changes: BTreeMap<String, PropertyValue>,
    /// Layout modifications
    pub layout_changes: LayoutConstraints,
}

/// Template metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMetadata {
    /// Author information
    pub author: String,
    /// Creation date
    pub created_date: String,
    /// Last modified date
    pub modified_date: String,
    /// Template tags
    pub tags: Vec<String>,
    /// Template icon
    pub icon: Option<String>,
    /// Preview image
    pub preview_image: Option<String>,
    /// Documentation URL
    pub documentation_url: Option<String>,
}

/// Visual appearance settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualSettings {
    /// Default background color
    pub background_color: Option<[u8; 4]>,
    /// Default foreground color
    pub foreground_color: Option<[u8; 4]>,
    /// Default font
    pub font_family: Option<String>,
    /// Default font size
    pub font_size: Option<f32>,
    /// Border settings
    pub border: BorderSettings,
    /// Shadow settings
    pub shadow: ShadowSettings,
}

/// Border settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderSettings {
    /// Border width
    pub width: f32,
    /// Border color
    pub color: [u8; 4],
    /// Border style
    pub style: BorderStyle,
    /// Border radius
    pub radius: f32,
}

/// Border style options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BorderStyle {
    Solid,
    Dashed,
    Dotted,
    Double,
    None,
}

/// Shadow settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowSettings {
    /// Shadow enabled
    pub enabled: bool,
    /// Shadow color
    pub color: [u8; 4],
    /// Shadow offset
    pub offset: Vec2,
    /// Shadow blur radius
    pub blur_radius: f32,
    /// Shadow spread radius
    pub spread_radius: f32,
}

/// Behavioral settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorSettings {
    /// Is the component interactive
    pub interactive: bool,
    /// Can the component receive focus
    pub focusable: bool,
    /// Default cursor when hovering
    pub cursor: CursorIcon,
    /// Animation settings
    pub animations: AnimationSettings,
    /// Event handlers
    pub event_handlers: Vec<EventHandler>,
}

/// Animation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationSettings {
    /// Enable animations
    pub enabled: bool,
    /// Default animation duration
    pub duration_ms: u32,
    /// Default easing function
    pub easing: EasingFunction,
    /// Animate property changes
    pub animate_properties: bool,
}

/// Easing function types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bounce,
    Elastic,
}

/// Event handler definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventHandler {
    /// Event type
    pub event_type: EventType,
    /// Handler code or reference
    pub handler: String,
    /// Handler language
    pub language: String,
}

/// Event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Click,
    DoubleClick,
    Hover,
    Focus,
    Blur,
    Change,
    KeyPress,
    Custom(String),
}

/// Usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStatistics {
    /// Number of times used
    pub usage_count: u32,
    /// Last used date
    pub last_used: String,
    /// User ratings
    pub ratings: Vec<f32>,
    /// Average rating
    pub average_rating: f32,
}

impl ComponentTemplate {
    /// Create a new template with default values
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            description: String::new(),
            category: "General".to_string(),
            version: "1.0.0".to_string(),
            parent_template: None,
            properties: BTreeMap::new(),
            layout: TemplateLayout::default(),
            metadata: TemplateMetadata::default(),
            visual_settings: VisualSettings::default(),
            behavior_settings: BehaviorSettings::default(),
            usage_stats: UsageStatistics::default(),
        }
    }
    
    /// Add a property to the template
    pub fn add_property(&mut self, property: TemplateProperty) {
        self.properties.insert(property.name.clone(), property);
    }
    
    /// Get a property by name
    pub fn get_property(&self, name: &str) -> Option<&TemplateProperty> {
        self.properties.get(name)
    }
    
    /// Remove a property by name
    pub fn remove_property(&mut self, name: &str) -> Option<TemplateProperty> {
        self.properties.remove(name)
    }
    
    /// Check if template has a parent
    pub fn has_parent(&self) -> bool {
        self.parent_template.is_some()
    }
    
    /// Get parent template ID
    pub fn parent_id(&self) -> Option<&str> {
        self.parent_template.as_deref()
    }
}

impl Default for TemplateLayout {
    fn default() -> Self {
        Self {
            layout_type: LayoutType::Fixed,
            children: Vec::new(),
            constraints: LayoutConstraints::default(),
            responsive: ResponsiveSettings::default(),
        }
    }
}

impl Default for LayoutConstraints {
    fn default() -> Self {
        Self {
            min_size: None,
            max_size: None,
            padding: Margin::same(0.0),
            margin: Margin::same(0.0),
            alignment: Align2::LEFT_TOP,
        }
    }
}

impl Default for ResponsiveSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            breakpoints: Vec::new(),
            layout_rules: Vec::new(),
        }
    }
}

impl Default for TemplateMetadata {
    fn default() -> Self {
        Self {
            author: "Unknown".to_string(),
            created_date: "Unknown".to_string(),
            modified_date: "Unknown".to_string(),
            tags: Vec::new(),
            icon: None,
            preview_image: None,
            documentation_url: None,
        }
    }
}

impl Default for VisualSettings {
    fn default() -> Self {
        Self {
            background_color: None,
            foreground_color: None,
            font_family: None,
            font_size: None,
            border: BorderSettings::default(),
            shadow: ShadowSettings::default(),
        }
    }
}

impl Default for BorderSettings {
    fn default() -> Self {
        Self {
            width: 1.0,
            color: [128, 128, 128, 255],
            style: BorderStyle::Solid,
            radius: 0.0,
        }
    }
}

impl Default for ShadowSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            color: [0, 0, 0, 128],
            offset: Vec2::ZERO,
            blur_radius: 4.0,
            spread_radius: 0.0,
        }
    }
}

impl Default for BehaviorSettings {
    fn default() -> Self {
        Self {
            interactive: true,
            focusable: true,
            cursor: CursorIcon::Default,
            animations: AnimationSettings::default(),
            event_handlers: Vec::new(),
        }
    }
}

impl Default for AnimationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            duration_ms: 200,
            easing: EasingFunction::EaseInOut,
            animate_properties: true,
        }
    }
}

impl Default for UsageStatistics {
    fn default() -> Self {
        Self {
            usage_count: 0,
            last_used: "Never".to_string(),
            ratings: Vec::new(),
            average_rating: 0.0,
        }
    }
}