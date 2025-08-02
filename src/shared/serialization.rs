//! Serialization Utilities
//!
//! Common serialization and deserialization utilities for components,
//! project data, and export/import functionality.

use std::collections::HashMap;
use serde_json::Value;

/// Trait for serializable components
pub trait SerializableComponent {
    /// Serialize component to JSON
    fn to_json(&self) -> Result<String, SerializationError>;
    
    /// Deserialize component from JSON
    fn from_json(json: &str) -> Result<Self, SerializationError>
    where
        Self: Sized;
    
    /// Get component schema version
    fn schema_version(&self) -> String;
    
    /// Migrate component from older version
    fn migrate_from_version(&mut self, from_version: &str) -> Result<(), SerializationError>;
}

/// Export format options
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ExportFormat {
    /// Native IDE format (JSON)
    Native,
    /// React JSX
    ReactJsx,
    /// Vue template
    VueTemplate,
    /// Angular template
    AngularTemplate,
    /// HTML/CSS
    HtmlCss,
    /// Figma JSON
    FigmaJson,
    /// Sketch JSON
    SketchJson,
    /// Adobe XD
    AdobeXd,
    /// SVG export
    Svg,
    /// PDF export
    Pdf,
    /// PNG image
    Png,
}

/// Serialization error types
#[derive(Debug, thiserror::Error)]
pub enum SerializationError {
    #[error("JSON parsing error: {0}")]
    JsonError(String),
    
    #[error("Schema version mismatch: expected {expected}, found {found}")]
    VersionMismatch { expected: String, found: String },
    
    #[error("Missing required field: {field}")]
    MissingField { field: String },
    
    #[error("Invalid data format: {message}")]
    InvalidFormat { message: String },
    
    #[error("Migration error: {message}")]
    MigrationError { message: String },
    
    #[error("Export format not supported: {format:?}")]
    UnsupportedFormat { format: ExportFormat },
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Component serialization data
#[derive(Clone, Debug)]
pub struct ComponentData {
    /// Component type identifier
    pub component_type: String,
    /// Component unique ID
    pub id: String,
    /// Component properties
    pub properties: HashMap<String, PropertyValue>,
    /// Child components
    pub children: Vec<ComponentData>,
    /// Component metadata
    pub metadata: ComponentMetadata,
    /// Schema version
    pub schema_version: String,
}

/// Component metadata
#[derive(Clone, Debug)]
pub struct ComponentMetadata {
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last modified timestamp
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// Component version
    pub version: String,
    /// Creator information
    pub creator: Option<String>,
    /// Component tags
    pub tags: Vec<String>,
    /// Additional custom metadata
    pub custom: HashMap<String, Value>,
}

/// Property value for serialization
#[derive(Clone, Debug)]
pub enum PropertyValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Color([u8; 4]),
    Array(Vec<PropertyValue>),
    Object(HashMap<String, PropertyValue>),
    Null,
}

/// Project serialization data
#[derive(Clone, Debug)]
pub struct ProjectData {
    /// Project metadata
    pub metadata: ProjectMetadata,
    /// Root components
    pub components: Vec<ComponentData>,
    /// Project assets
    pub assets: Vec<AssetData>,
    /// Design system data
    pub design_system: Option<DesignSystemData>,
    /// Project settings
    pub settings: ProjectSettings,
    /// Schema version
    pub schema_version: String,
}

/// Project metadata
#[derive(Clone, Debug)]
pub struct ProjectMetadata {
    /// Project name
    pub name: String,
    /// Project description
    pub description: Option<String>,
    /// Project version
    pub version: String,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last modified timestamp
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// Project author
    pub author: Option<String>,
    /// Project tags
    pub tags: Vec<String>,
    /// Target platforms
    pub target_platforms: Vec<String>,
}

/// Asset data
#[derive(Clone, Debug)]
pub struct AssetData {
    /// Asset ID
    pub id: String,
    /// Asset name
    pub name: String,
    /// Asset type
    pub asset_type: AssetType,
    /// Asset data (base64 encoded for binary data)
    pub data: String,
    /// Asset metadata
    pub metadata: HashMap<String, Value>,
}

/// Asset type enumeration
#[derive(Clone, Debug, PartialEq)]
pub enum AssetType {
    Image,
    Font,
    Icon,
    Audio,
    Video,
    Document,
    Other(String),
}

/// Design system serialization data
#[derive(Clone, Debug)]
pub struct DesignSystemData {
    /// Design tokens
    pub tokens: HashMap<String, DesignToken>,
    /// Component library
    pub components: Vec<ComponentTemplate>,
    /// Style guide settings
    pub style_guide: StyleGuideData,
}

/// Design token data
#[derive(Clone, Debug)]
pub struct DesignToken {
    /// Token name
    pub name: String,
    /// Token value
    pub value: PropertyValue,
    /// Token category
    pub category: String,
    /// Token description
    pub description: Option<String>,
}

/// Component template data
#[derive(Clone, Debug)]
pub struct ComponentTemplate {
    /// Template name
    pub name: String,
    /// Template description
    pub description: Option<String>,
    /// Template component data
    pub template: ComponentData,
    /// Default properties
    pub default_properties: HashMap<String, PropertyValue>,
}

/// Style guide data
#[derive(Clone, Debug)]
pub struct StyleGuideData {
    /// Color palette
    pub colors: Vec<String>,
    /// Typography settings
    pub typography: TypographySettings,
    /// Spacing scale
    pub spacing: Vec<f64>,
    /// Border radius scale
    pub border_radius: Vec<f64>,
}

/// Typography settings
#[derive(Clone, Debug)]
pub struct TypographySettings {
    /// Font families
    pub font_families: Vec<String>,
    /// Font size scale
    pub font_sizes: Vec<f64>,
    /// Line height scale
    pub line_heights: Vec<f64>,
    /// Font weight options
    pub font_weights: Vec<u32>,
}

/// Project settings
#[derive(Clone, Debug)]
pub struct ProjectSettings {
    /// Default canvas size
    pub canvas_size: (f64, f64),
    /// Grid settings
    pub grid: GridSettings,
    /// Snap settings
    pub snap: SnapSettings,
    /// Export settings
    pub export: ExportSettings,
}

/// Grid settings
#[derive(Clone, Debug)]
pub struct GridSettings {
    /// Grid enabled
    pub enabled: bool,
    /// Grid size
    pub size: f64,
    /// Grid color
    pub color: [u8; 4],
    /// Grid opacity
    pub opacity: f32,
}

/// Snap settings
#[derive(Clone, Debug)]
pub struct SnapSettings {
    /// Snap to grid enabled
    pub to_grid: bool,
    /// Snap to objects enabled
    pub to_objects: bool,
    /// Snap distance threshold
    pub distance: f64,
}

/// Export settings
#[derive(Clone, Debug)]
pub struct ExportSettings {
    /// Default export format
    pub default_format: ExportFormat,
    /// Export quality settings
    pub quality: HashMap<ExportFormat, f64>,
    /// Export scale factors
    pub scale_factors: Vec<f64>,
}

/// Serialization utilities
pub struct SerializationUtils;

impl Default for ComponentMetadata {
    fn default() -> Self {
        let now = chrono::Utc::now();
        Self {
            created_at: now,
            modified_at: now,
            version: "1.0.0".to_string(),
            creator: None,
            tags: Vec::new(),
            custom: HashMap::new(),
        }
    }
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            canvas_size: (1920.0, 1080.0),
            grid: GridSettings::default(),
            snap: SnapSettings::default(),
            export: ExportSettings::default(),
        }
    }
}

impl Default for GridSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            size: 8.0,
            color: [200, 200, 200, 100],
            opacity: 0.5,
        }
    }
}

impl Default for SnapSettings {
    fn default() -> Self {
        Self {
            to_grid: true,
            to_objects: true,
            distance: 5.0,
        }
    }
}

impl Default for ExportSettings {
    fn default() -> Self {
        Self {
            default_format: ExportFormat::Native,
            quality: HashMap::from([
                (ExportFormat::Png, 0.9),
                (ExportFormat::Pdf, 1.0),
            ]),
            scale_factors: vec![1.0, 2.0, 3.0],
        }
    }
}

impl SerializationUtils {
    /// Serialize component to JSON string (placeholder)
    pub fn serialize_component(_component: &ComponentData) -> Result<String, SerializationError> {
        Err(SerializationError::MigrationError { message: "Serialization not implemented".to_string() })
    }
    
    /// Deserialize component from JSON string (placeholder)
    pub fn deserialize_component(_json: &str) -> Result<ComponentData, SerializationError> {
        Err(SerializationError::MigrationError { message: "Deserialization not implemented".to_string() })
    }
    
    /// Serialize project to JSON string (placeholder)
    pub fn serialize_project(_project: &ProjectData) -> Result<String, SerializationError> {
        Err(SerializationError::MigrationError { message: "Project serialization not implemented".to_string() })
    }
    
    /// Deserialize project from JSON string (placeholder)
    pub fn deserialize_project(_json: &str) -> Result<ProjectData, SerializationError> {
        Err(SerializationError::MigrationError { message: "Project deserialization not implemented".to_string() })
    }
    
    /// Convert component to different export format
    pub fn export_component(
        component: &ComponentData,
        format: &ExportFormat,
    ) -> Result<String, SerializationError> {
        match format {
            ExportFormat::Native => Self::serialize_component(component),
            ExportFormat::ReactJsx => Self::to_react_jsx(component),
            ExportFormat::VueTemplate => Self::to_vue_template(component),
            ExportFormat::AngularTemplate => Self::to_angular_template(component),
            ExportFormat::HtmlCss => Self::to_html_css(component),
            _ => Err(SerializationError::UnsupportedFormat {
                format: format.clone(),
            }),
        }
    }
    
    /// Convert component to React JSX
    fn to_react_jsx(component: &ComponentData) -> Result<String, SerializationError> {
        let mut jsx = String::new();
        
        jsx.push_str(&format!("<{}", component.component_type));
        
        // Add properties as JSX props
        for (key, value) in &component.properties {
            jsx.push_str(&format!(" {}={}", key, Self::property_value_to_jsx(value)?));
        }
        
        if component.children.is_empty() {
            jsx.push_str(" />");
        } else {
            jsx.push_str(">");
            
            // Add children
            for child in &component.children {
                jsx.push_str(&Self::to_react_jsx(child)?);
            }
            
            jsx.push_str(&format!("</{}>", component.component_type));
        }
        
        Ok(jsx)
    }
    
    /// Convert property value to JSX format
    fn property_value_to_jsx(value: &PropertyValue) -> Result<String, SerializationError> {
        match value {
            PropertyValue::String(s) => Ok(format!("\"{}\"", s)),
            PropertyValue::Number(n) => Ok(format!("{{{}}}", n)),
            PropertyValue::Boolean(b) => Ok(format!("{{{}}}", b)),
            PropertyValue::Color(color) => Ok(format!(
                "\"rgba({}, {}, {}, {})\"",
                color[0], color[1], color[2], color[3] as f32 / 255.0
            )),
            PropertyValue::Array(_) => Ok("{[]}".to_string()), // Simplified
            PropertyValue::Object(_) => Ok("{{}}".to_string()), // Simplified
            PropertyValue::Null => Ok("null".to_string()),
        }
    }
    
    /// Convert component to Vue template
    fn to_vue_template(component: &ComponentData) -> Result<String, SerializationError> {
        let mut template = String::new();
        
        template.push_str(&format!("<{}", Self::component_type_to_vue(&component.component_type)));
        
        // Add properties as Vue props
        for (key, value) in &component.properties {
            template.push_str(&format!(" :{}", key));
            template.push_str(&format!("=\"{}\"", Self::property_value_to_string(value)?));
        }
        
        if component.children.is_empty() {
            template.push_str(" />");
        } else {
            template.push_str(">");
            
            // Add children
            for child in &component.children {
                template.push_str(&Self::to_vue_template(child)?);
            }
            
            template.push_str(&format!("</{}>", Self::component_type_to_vue(&component.component_type)));
        }
        
        Ok(template)
    }
    
    /// Convert component to Angular template
    fn to_angular_template(component: &ComponentData) -> Result<String, SerializationError> {
        let mut template = String::new();
        
        template.push_str(&format!("<{}", Self::component_type_to_angular(&component.component_type)));
        
        // Add properties as Angular bindings
        for (key, value) in &component.properties {
            template.push_str(&format!(" [{}]", key));
            template.push_str(&format!("=\"{}\"", Self::property_value_to_string(value)?));
        }
        
        if component.children.is_empty() {
            template.push_str(&format!("></{}>>", Self::component_type_to_angular(&component.component_type)));
        } else {
            template.push_str(">");
            
            // Add children
            for child in &component.children {
                template.push_str(&Self::to_angular_template(child)?);
            }
            
            template.push_str(&format!("</{}>", Self::component_type_to_angular(&component.component_type)));
        }
        
        Ok(template)
    }
    
    /// Convert component to HTML/CSS
    fn to_html_css(component: &ComponentData) -> Result<String, SerializationError> {
        let mut html = String::new();
        
        // Map component type to HTML element
        let html_element = Self::component_type_to_html(&component.component_type);
        html.push_str(&format!("<{}", html_element));
        
        // Add ID and class
        html.push_str(&format!(" id=\"{}\"", component.id));
        html.push_str(&format!(" class=\"{}\"", component.component_type));
        
        // Add inline styles from properties
        let mut styles = Vec::new();
        for (key, value) in &component.properties {
            if let Some(css_property) = Self::property_to_css(key, value) {
                styles.push(css_property);
            }
        }
        
        if !styles.is_empty() {
            html.push_str(&format!(" style=\"{}\"", styles.join("; ")));
        }
        
        if component.children.is_empty() && Self::is_void_element(&html_element) {
            html.push_str(" />");
        } else {
            html.push_str(">");
            
            // Add children
            for child in &component.children {
                html.push_str(&Self::to_html_css(child)?);
            }
            
            html.push_str(&format!("</{}>", html_element));
        }
        
        Ok(html)
    }
    
    /// Convert property value to string representation
    fn property_value_to_string(value: &PropertyValue) -> Result<String, SerializationError> {
        match value {
            PropertyValue::String(s) => Ok(s.clone()),
            PropertyValue::Number(n) => Ok(n.to_string()),
            PropertyValue::Boolean(b) => Ok(b.to_string()),
            PropertyValue::Color(color) => Ok(format!(
                "rgba({}, {}, {}, {})",
                color[0], color[1], color[2], color[3] as f32 / 255.0
            )),
            PropertyValue::Array(_) => Ok("[]".to_string()),
            PropertyValue::Object(_) => Ok("{}".to_string()),
            PropertyValue::Null => Ok("null".to_string()),
        }
    }
    
    /// Convert component type to Vue component name
    fn component_type_to_vue(component_type: &str) -> String {
        // Convert to PascalCase for Vue components
        component_type.split('_')
            .map(|word| {
                let mut chars: Vec<char> = word.chars().collect();
                if let Some(first) = chars.first_mut() {
                    *first = first.to_uppercase().next().unwrap_or(*first);
                }
                chars.iter().collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("")
    }
    
    /// Convert component type to Angular component selector
    fn component_type_to_angular(component_type: &str) -> String {
        format!("app-{}", component_type.replace('_', "-"))
    }
    
    /// Convert component type to HTML element
    fn component_type_to_html(component_type: &str) -> String {
        match component_type {
            "button" => "button".to_string(),
            "text" => "span".to_string(),
            "input" => "input".to_string(),
            "container" => "div".to_string(),
            "image" => "img".to_string(),
            _ => "div".to_string(),
        }
    }
    
    /// Check if HTML element is void (self-closing)
    fn is_void_element(element: &str) -> bool {
        matches!(element, "img" | "input" | "br" | "hr" | "meta" | "link")
    }
    
    /// Convert property to CSS
    fn property_to_css(property: &str, value: &PropertyValue) -> Option<String> {
        let css_property = match property {
            "width" => "width",
            "height" => "height",
            "background_color" => "background-color",
            "color" => "color",
            "font_size" => "font-size",
            "margin" => "margin",
            "padding" => "padding",
            "border_radius" => "border-radius",
            _ => return None,
        };
        
        match value {
            PropertyValue::String(s) => Some(format!("{}: {}", css_property, s)),
            PropertyValue::Number(n) => Some(format!("{}: {}px", css_property, n)),
            PropertyValue::Color(color) => Some(format!(
                "{}: rgba({}, {}, {}, {})",
                css_property, color[0], color[1], color[2], color[3] as f32 / 255.0
            )),
            _ => None,
        }
    }
    
    /// Validate schema version compatibility
    pub fn validate_schema_version(
        data_version: &str,
        expected_version: &str,
    ) -> Result<(), SerializationError> {
        if data_version != expected_version {
            Err(SerializationError::VersionMismatch {
                expected: expected_version.to_string(),
                found: data_version.to_string(),
            })
        } else {
            Ok(())
        }
    }
    
    /// Migrate data from older schema version
    pub fn migrate_component_data(
        mut data: ComponentData,
        target_version: &str,
    ) -> Result<ComponentData, SerializationError> {
        // Simple migration example
        if data.schema_version == "1.0.0" && target_version == "1.1.0" {
            // Add default metadata if missing
            if data.metadata.tags.is_empty() {
                data.metadata.tags.push("migrated".to_string());
            }
            data.schema_version = target_version.to_string();
        }
        
        Ok(data)
    }
}

/// Custom serializer for DateTime (placeholder)
pub fn serialize_datetime(date: &chrono::DateTime<chrono::Utc>) -> String {
    date.to_rfc3339()
}

/// Custom deserializer for DateTime (placeholder) 
pub fn deserialize_datetime(s: &str) -> Result<chrono::DateTime<chrono::Utc>, chrono::ParseError> {
    chrono::DateTime::parse_from_rfc3339(s).map(|dt| dt.with_timezone(&chrono::Utc))
}