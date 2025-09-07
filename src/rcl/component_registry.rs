//! ComponentRegistry with PropertySchema and auto-generated property inspector
//!
//! Provides unified metadata registry & property typing with derive macro ComponentMeta
//! as specified in the improvement plan Phase P0.

use std::collections::HashMap;
use std::any::{Any, TypeId};
use serde::{Serialize, Deserialize};

/// Central registry for all RCL components with metadata and schemas
pub struct ComponentRegistry {
    /// Registered components by name
    pub components: HashMap<String, ComponentMetadata>,
    /// Property schemas by component type
    pub schemas: HashMap<String, PropertySchema>,
    /// Component factory functions
    pub factories: HashMap<String, ComponentFactory>,
    /// Inspector generators
    pub inspectors: HashMap<String, InspectorGenerator>,
    /// Event definitions
    pub events: HashMap<String, EventDefinition>,
}

/// Metadata for a registered component
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComponentMetadata {
    /// Component type name
    pub component_type: String,
    /// Display name for UI
    pub display_name: String,
    /// Component description
    pub description: String,
    /// Category for organization
    pub category: ComponentCategory,
    /// Component version
    pub version: String,
    /// Property schema
    pub schema: PropertySchema,
    /// Default property values
    pub defaults: HashMap<String, PropertyValue>,
    /// Supported events
    pub events: Vec<String>,
    /// Component icon (optional)
    pub icon: Option<String>,
    /// Component tags for search/filtering
    pub tags: Vec<String>,
}

/// Component categories for organization
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ComponentCategory {
    Layout,
    Input,
    Display,
    Navigation,
    Data,
    Media,
    Advanced,
    Custom(String),
}

/// Property schema definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropertySchema {
    /// Properties by name
    pub properties: HashMap<String, PropertyDefinition>,
    /// Required properties
    pub required: Vec<String>,
    /// Property groups for organization
    pub groups: Vec<PropertyGroup>,
}

/// Individual property definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropertyDefinition {
    /// Property name
    pub name: String,
    /// Property type
    pub property_type: PropertyType,
    /// Display name
    pub display_name: String,
    /// Property description
    pub description: String,
    /// Default value
    pub default_value: PropertyValue,
    /// Validation constraints
    pub constraints: Vec<PropertyConstraint>,
    /// UI hints for inspector
    pub ui_hints: PropertyUIHints,
    /// Whether property is readonly
    pub readonly: bool,
    /// Whether property is advanced (hidden by default)
    pub advanced: bool,
}

/// Property value enum supporting all common types
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum PropertyValue {
    // Primitive types
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    
    // Collections
    Array(Vec<PropertyValue>),
    Object(HashMap<String, PropertyValue>),
    
    // UI-specific types
    Color(Color),
    Size(Size),
    Position(Position),
    Margin(Margin),
    Padding(Padding),
    
    // Layout types
    FlexDirection(FlexDirection),
    Alignment(Alignment),
    JustifyContent(JustifyContent),
    
    // Event handlers
    EventHandler(String), // Function name or code
    
    // References
    ComponentRef(String), // Reference to another component
    ResourceRef(String),  // Reference to external resource
    
    // Special values
    Null,
    Undefined,
}

/// Property type system
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum PropertyType {
    // Basic types
    String,
    Integer,
    Float,
    Boolean,
    
    // Enum types
    Enum(Vec<String>), // Possible values
    
    // Complex types
    Object(PropertySchema), // Nested object with schema
    Array(Box<PropertyType>), // Array of specific type
    Union(Vec<PropertyType>), // One of several types
    
    // UI types
    Color,
    Size,
    Position,
    Margin,
    Padding,
    Font,
    
    // Layout types
    FlexDirection,
    Alignment,
    JustifyContent,
    
    // Function types
    EventHandler,
    
    // Reference types
    ComponentRef,
    ResourceRef,
    
    // Custom types
    Custom(String),
}

/// Property validation constraints
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PropertyConstraint {
    // Numeric constraints
    Min(f64),
    Max(f64),
    Range(f64, f64),
    
    // String constraints
    MinLength(usize),
    MaxLength(usize),
    Pattern(String), // Regex pattern
    
    // Array constraints
    MinItems(usize),
    MaxItems(usize),
    UniqueItems,
    
    // Custom validation
    Custom(String), // Validation function name
}

/// UI hints for property inspector rendering
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropertyUIHints {
    /// Control type for rendering
    pub control_type: ControlType,
    /// Placeholder text
    pub placeholder: Option<String>,
    /// Help text
    pub help_text: Option<String>,
    /// Unit label (e.g., "px", "%")
    pub unit: Option<String>,
    /// Step size for numeric inputs
    pub step: Option<f64>,
    /// Options for select/dropdown
    pub options: Option<Vec<SelectOption>>,
    /// Custom renderer
    pub custom_renderer: Option<String>,
}

/// Control types for property inspector
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ControlType {
    // Basic controls
    TextInput,
    TextArea,
    NumberInput,
    Checkbox,
    
    // Selection controls
    Select,
    Radio,
    
    // Special controls
    ColorPicker,
    Slider,
    RangeSlider,
    FilePicker,
    
    // Layout controls
    SpacingControl,
    AlignmentControl,
    
    // Custom control
    Custom(String),
}

/// Option for select controls
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub icon: Option<String>,
}

/// Property groups for organization in inspector
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropertyGroup {
    /// Group name
    pub name: String,
    /// Group display name
    pub display_name: String,
    /// Properties in this group
    pub properties: Vec<String>,
    /// Whether group is collapsible
    pub collapsible: bool,
    /// Whether group starts collapsed
    pub collapsed: bool,
}

/// UI-specific value types
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Margin {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Alignment {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum JustifyContent {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

/// Component factory function type
pub type ComponentFactory = Box<dyn Fn(&HashMap<String, PropertyValue>) -> Box<dyn Any>>;

/// Inspector generator function type
pub type InspectorGenerator = Box<dyn Fn(&PropertySchema) -> InspectorDefinition>;

/// Generated inspector definition
#[derive(Clone, Debug)]
pub struct InspectorDefinition {
    /// Inspector sections
    pub sections: Vec<InspectorSection>,
    /// Validation rules
    pub validation: Vec<ValidationRule>,
}

/// Inspector section
#[derive(Clone, Debug)]
pub struct InspectorSection {
    /// Section title
    pub title: String,
    /// Section controls
    pub controls: Vec<InspectorControl>,
    /// Whether section is collapsible
    pub collapsible: bool,
    /// Whether section starts collapsed
    pub collapsed: bool,
}

/// Inspector control
#[derive(Clone, Debug)]
pub struct InspectorControl {
    /// Property name this control edits
    pub property_name: String,
    /// Control label
    pub label: String,
    /// Control type
    pub control_type: ControlType,
    /// Control configuration
    pub config: HashMap<String, PropertyValue>,
}

/// Validation rule for inspector
#[derive(Clone, Debug)]
pub struct ValidationRule {
    /// Properties this rule applies to
    pub properties: Vec<String>,
    /// Validation function
    pub validator: String, // Function name
    /// Error message
    pub error_message: String,
}

/// Event definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventDefinition {
    /// Event name
    pub name: String,
    /// Event description
    pub description: String,
    /// Event parameters
    pub parameters: Vec<EventParameter>,
    /// Whether event bubbles
    pub bubbles: bool,
}

/// Event parameter
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventParameter {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub parameter_type: PropertyType,
    /// Parameter description
    pub description: String,
}

/// Trait for components that provide metadata
pub trait ComponentMeta {
    /// Get component metadata
    fn metadata() -> ComponentMetadata;
    /// Get property schema
    fn schema() -> PropertySchema;
    /// Get default property values
    fn defaults() -> HashMap<String, PropertyValue>;
    /// Get supported events
    fn events() -> Vec<EventDefinition>;
}

impl ComponentRegistry {
    /// Create new component registry
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            schemas: HashMap::new(),
            factories: HashMap::new(),
            inspectors: HashMap::new(),
            events: HashMap::new(),
        }
    }

    /// Register a component with metadata
    pub fn register_component<T: ComponentMeta + 'static>(&mut self, factory: ComponentFactory) {
        let metadata = T::metadata();
        let schema = T::schema();
        let events = T::events();
        
        // Store metadata
        self.components.insert(metadata.component_type.clone(), metadata.clone());
        self.schemas.insert(metadata.component_type.clone(), schema.clone());
        self.factories.insert(metadata.component_type.clone(), factory);
        
        // Generate inspector
        let inspector = self.generate_inspector(&schema);
        self.inspectors.insert(metadata.component_type.clone(), Box::new(move |_| inspector.clone()));
        
        // Register events
        for event in events {
            self.events.insert(event.name.clone(), event);
        }
    }

    /// Get component metadata by type
    pub fn get_metadata(&self, component_type: &str) -> Option<&ComponentMetadata> {
        self.components.get(component_type)
    }

    /// Get property schema by component type
    pub fn get_schema(&self, component_type: &str) -> Option<&PropertySchema> {
        self.schemas.get(component_type)
    }

    /// Create component instance
    pub fn create_component(&self, component_type: &str, properties: &HashMap<String, PropertyValue>) -> Option<Box<dyn Any>> {
        self.factories.get(component_type).map(|factory| factory(properties))
    }

    /// Generate property inspector for component
    pub fn generate_inspector(&self, schema: &PropertySchema) -> InspectorDefinition {
        let mut sections = Vec::new();
        
        // Create sections based on property groups
        for group in &schema.groups {
            let mut controls = Vec::new();
            
            for property_name in &group.properties {
                if let Some(property_def) = schema.properties.get(property_name) {
                    let control = InspectorControl {
                        property_name: property_name.clone(),
                        label: property_def.display_name.clone(),
                        control_type: property_def.ui_hints.control_type.clone(),
                        config: self.create_control_config(property_def),
                    };
                    controls.push(control);
                }
            }
            
            sections.push(InspectorSection {
                title: group.display_name.clone(),
                controls,
                collapsible: group.collapsible,
                collapsed: group.collapsed,
            });
        }
        
        // Add ungrouped properties to default section
        let grouped_properties: std::collections::HashSet<_> = schema.groups
            .iter()
            .flat_map(|g| g.properties.iter())
            .collect();
        
        let ungrouped: Vec<_> = schema.properties
            .iter()
            .filter(|(name, _)| !grouped_properties.contains(name))
            .collect();
        
        if !ungrouped.is_empty() {
            let mut controls = Vec::new();
            for (property_name, property_def) in ungrouped {
                let control = InspectorControl {
                    property_name: property_name.clone(),
                    label: property_def.display_name.clone(),
                    control_type: property_def.ui_hints.control_type.clone(),
                    config: self.create_control_config(property_def),
                };
                controls.push(control);
            }
            
            sections.push(InspectorSection {
                title: "Properties".to_string(),
                controls,
                collapsible: false,
                collapsed: false,
            });
        }
        
        // Generate validation rules
        let validation = self.generate_validation_rules(schema);
        
        InspectorDefinition {
            sections,
            validation,
        }
    }

    /// Create control configuration from property definition
    fn create_control_config(&self, property_def: &PropertyDefinition) -> HashMap<String, PropertyValue> {
        let mut config = HashMap::new();
        
        // Add basic configuration
        config.insert("default_value".to_string(), property_def.default_value.clone());
        config.insert("readonly".to_string(), PropertyValue::Boolean(property_def.readonly));
        
        // Add UI hints
        if let Some(placeholder) = &property_def.ui_hints.placeholder {
            config.insert("placeholder".to_string(), PropertyValue::String(placeholder.clone()));
        }
        
        if let Some(help_text) = &property_def.ui_hints.help_text {
            config.insert("help_text".to_string(), PropertyValue::String(help_text.clone()));
        }
        
        if let Some(unit) = &property_def.ui_hints.unit {
            config.insert("unit".to_string(), PropertyValue::String(unit.clone()));
        }
        
        if let Some(step) = property_def.ui_hints.step {
            config.insert("step".to_string(), PropertyValue::Float(step));
        }
        
        if let Some(options) = &property_def.ui_hints.options {
            let option_values: Vec<PropertyValue> = options
                .iter()
                .map(|opt| PropertyValue::Object({
                    let mut obj = HashMap::new();
                    obj.insert("value".to_string(), PropertyValue::String(opt.value.clone()));
                    obj.insert("label".to_string(), PropertyValue::String(opt.label.clone()));
                    obj
                }))
                .collect();
            config.insert("options".to_string(), PropertyValue::Array(option_values));
        }
        
        // Add constraints
        for constraint in &property_def.constraints {
            match constraint {
                PropertyConstraint::Min(min) => {
                    config.insert("min".to_string(), PropertyValue::Float(*min));
                }
                PropertyConstraint::Max(max) => {
                    config.insert("max".to_string(), PropertyValue::Float(*max));
                }
                PropertyConstraint::Range(min, max) => {
                    config.insert("min".to_string(), PropertyValue::Float(*min));
                    config.insert("max".to_string(), PropertyValue::Float(*max));
                }
                PropertyConstraint::MinLength(min_len) => {
                    config.insert("min_length".to_string(), PropertyValue::Integer(*min_len as i64));
                }
                PropertyConstraint::MaxLength(max_len) => {
                    config.insert("max_length".to_string(), PropertyValue::Integer(*max_len as i64));
                }
                PropertyConstraint::Pattern(pattern) => {
                    config.insert("pattern".to_string(), PropertyValue::String(pattern.clone()));
                }
                _ => {}
            }
        }
        
        config
    }

    /// Generate validation rules from schema
    fn generate_validation_rules(&self, schema: &PropertySchema) -> Vec<ValidationRule> {
        let mut rules = Vec::new();
        
        // Required property validation
        if !schema.required.is_empty() {
            rules.push(ValidationRule {
                properties: schema.required.clone(),
                validator: "required".to_string(),
                error_message: "This field is required".to_string(),
            });
        }
        
        // Type-specific validation
        for (property_name, property_def) in &schema.properties {
            for constraint in &property_def.constraints {
                let rule = match constraint {
                    PropertyConstraint::Min(min) => ValidationRule {
                        properties: vec![property_name.clone()],
                        validator: format!("min:{}", min),
                        error_message: format!("Value must be at least {}", min),
                    },
                    PropertyConstraint::Max(max) => ValidationRule {
                        properties: vec![property_name.clone()],
                        validator: format!("max:{}", max),
                        error_message: format!("Value must be at most {}", max),
                    },
                    PropertyConstraint::Range(min, max) => ValidationRule {
                        properties: vec![property_name.clone()],
                        validator: format!("range:{}:{}", min, max),
                        error_message: format!("Value must be between {} and {}", min, max),
                    },
                    PropertyConstraint::MinLength(min_len) => ValidationRule {
                        properties: vec![property_name.clone()],
                        validator: format!("min_length:{}", min_len),
                        error_message: format!("Must be at least {} characters", min_len),
                    },
                    PropertyConstraint::MaxLength(max_len) => ValidationRule {
                        properties: vec![property_name.clone()],
                        validator: format!("max_length:{}", max_len),
                        error_message: format!("Must be at most {} characters", max_len),
                    },
                    PropertyConstraint::Pattern(pattern) => ValidationRule {
                        properties: vec![property_name.clone()],
                        validator: format!("pattern:{}", pattern),
                        error_message: "Invalid format".to_string(),
                    },
                    PropertyConstraint::Custom(validator) => ValidationRule {
                        properties: vec![property_name.clone()],
                        validator: validator.clone(),
                        error_message: "Validation failed".to_string(),
                    },
                    _ => continue,
                };
                rules.push(rule);
            }
        }
        
        rules
    }

    /// Get all registered component types
    pub fn get_component_types(&self) -> Vec<String> {
        self.components.keys().cloned().collect()
    }

    /// Get components by category
    pub fn get_components_by_category(&self, category: &ComponentCategory) -> Vec<&ComponentMetadata> {
        self.components
            .values()
            .filter(|meta| &meta.category == category)
            .collect()
    }

    /// Search components by tag or name
    pub fn search_components(&self, query: &str) -> Vec<&ComponentMetadata> {
        let query = query.to_lowercase();
        self.components
            .values()
            .filter(|meta| {
                meta.component_type.to_lowercase().contains(&query)
                    || meta.display_name.to_lowercase().contains(&query)
                    || meta.description.to_lowercase().contains(&query)
                    || meta.tags.iter().any(|tag| tag.to_lowercase().contains(&query))
            })
            .collect()
    }

    /// Validate property values against schema
    pub fn validate_properties(&self, component_type: &str, properties: &HashMap<String, PropertyValue>) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        
        if let Some(schema) = self.get_schema(component_type) {
            // Check required properties
            for required_prop in &schema.required {
                if !properties.contains_key(required_prop) {
                    errors.push(ValidationError {
                        property: required_prop.clone(),
                        message: "Required property missing".to_string(),
                    });
                }
            }
            
            // Validate individual properties
            for (prop_name, prop_value) in properties {
                if let Some(prop_def) = schema.properties.get(prop_name) {
                    if let Err(error) = self.validate_property_value(prop_value, &prop_def.property_type, &prop_def.constraints) {
                        errors.push(ValidationError {
                            property: prop_name.clone(),
                            message: error,
                        });
                    }
                }
            }
        }
        
        errors
    }

    /// Validate individual property value
    fn validate_property_value(&self, value: &PropertyValue, expected_type: &PropertyType, constraints: &[PropertyConstraint]) -> Result<(), String> {
        // Type validation
        match (value, expected_type) {
            (PropertyValue::String(_), PropertyType::String) => {}
            (PropertyValue::Integer(_), PropertyType::Integer) => {}
            (PropertyValue::Float(_), PropertyType::Float) => {}
            (PropertyValue::Boolean(_), PropertyType::Boolean) => {}
            (PropertyValue::Color(_), PropertyType::Color) => {}
            _ => return Err(format!("Type mismatch: expected {:?}", expected_type)),
        }
        
        // Constraint validation
        for constraint in constraints {
            match (constraint, value) {
                (PropertyConstraint::Min(min), PropertyValue::Float(val)) => {
                    if val < min {
                        return Err(format!("Value {} is less than minimum {}", val, min));
                    }
                }
                (PropertyConstraint::Max(max), PropertyValue::Float(val)) => {
                    if val > max {
                        return Err(format!("Value {} is greater than maximum {}", val, max));
                    }
                }
                (PropertyConstraint::MinLength(min_len), PropertyValue::String(val)) => {
                    if val.len() < *min_len {
                        return Err(format!("String length {} is less than minimum {}", val.len(), min_len));
                    }
                }
                (PropertyConstraint::MaxLength(max_len), PropertyValue::String(val)) => {
                    if val.len() > *max_len {
                        return Err(format!("String length {} is greater than maximum {}", val.len(), max_len));
                    }
                }
                _ => {}
            }
        }
        
        Ok(())
    }
}

/// Validation error
#[derive(Clone, Debug)]
pub struct ValidationError {
    pub property: String,
    pub message: String,
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Derive macro for ComponentMeta (would be implemented as a proc macro)
/// For now, we'll provide a macro_rules! version for demonstration
#[macro_export]
macro_rules! impl_component_meta {
    (
        $component:ty,
        name: $name:expr,
        display_name: $display_name:expr,
        description: $description:expr,
        category: $category:expr,
        properties: {
            $(
                $prop_name:ident: {
                    type: $prop_type:expr,
                    display_name: $prop_display_name:expr,
                    description: $prop_description:expr,
                    default: $prop_default:expr,
                    $(constraints: [$($constraint:expr),*],)?
                    $(ui_hints: $ui_hints:expr,)?
                }
            ),*
        }
        $(, events: [$($event:expr),*])?
    ) => {
        impl ComponentMeta for $component {
            fn metadata() -> ComponentMetadata {
                ComponentMetadata {
                    component_type: $name.to_string(),
                    display_name: $display_name.to_string(),
                    description: $description.to_string(),
                    category: $category,
                    version: "1.0.0".to_string(),
                    schema: Self::schema(),
                    defaults: Self::defaults(),
                    events: Self::events().into_iter().map(|e| e.name).collect(),
                    icon: None,
                    tags: Vec::new(),
                }
            }
            
            fn schema() -> PropertySchema {
                let mut properties = HashMap::new();
                let mut groups = Vec::new();
                let mut required = Vec::new();
                
                $(
                    let mut constraints = Vec::new();
                    $($(constraints.push($constraint);)*)?
                    
                    let ui_hints = $($ui_hints)?
                        .unwrap_or(PropertyUIHints {
                            control_type: ControlType::TextInput,
                            placeholder: None,
                            help_text: None,
                            unit: None,
                            step: None,
                            options: None,
                            custom_renderer: None,
                        });
                    
                    properties.insert(
                        stringify!($prop_name).to_string(),
                        PropertyDefinition {
                            name: stringify!($prop_name).to_string(),
                            property_type: $prop_type,
                            display_name: $prop_display_name.to_string(),
                            description: $prop_description.to_string(),
                            default_value: $prop_default,
                            constraints,
                            ui_hints,
                            readonly: false,
                            advanced: false,
                        }
                    );
                )*
                
                // Create default group
                groups.push(PropertyGroup {
                    name: "general".to_string(),
                    display_name: "General".to_string(),
                    properties: vec![$(stringify!($prop_name).to_string()),*],
                    collapsible: false,
                    collapsed: false,
                });
                
                PropertySchema {
                    properties,
                    required,
                    groups,
                }
            }
            
            fn defaults() -> HashMap<String, PropertyValue> {
                let mut defaults = HashMap::new();
                $(
                    defaults.insert(stringify!($prop_name).to_string(), $prop_default);
                )*
                defaults
            }
            
            fn events() -> Vec<EventDefinition> {
                vec![
                    $($(
                        $event
                    ),*)?
                ]
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    // Example component for testing
    struct TestButton;

    impl_component_meta! {
        TestButton,
        name: "Button",
        display_name: "Button",
        description: "Interactive button component",
        category: ComponentCategory::Input,
        properties: {
            text: {
                type: PropertyType::String,
                display_name: "Text",
                description: "Button text content",
                default: PropertyValue::String("Button".to_string()),
                constraints: [PropertyConstraint::MaxLength(100)],
                ui_hints: Some(PropertyUIHints {
                    control_type: ControlType::TextInput,
                    placeholder: Some("Enter button text".to_string()),
                    help_text: None,
                    unit: None,
                    step: None,
                    options: None,
                    custom_renderer: None,
                })
            },
            enabled: {
                type: PropertyType::Boolean,
                display_name: "Enabled",
                description: "Whether the button is enabled",
                default: PropertyValue::Boolean(true)
            }
        },
        events: [
            EventDefinition {
                name: "click".to_string(),
                description: "Fired when button is clicked".to_string(),
                parameters: vec![],
                bubbles: true,
            }
        ]
    }

    #[test]
    fn test_component_registration() {
        let mut registry = ComponentRegistry::new();
        
        // Register test component
        registry.register_component::<TestButton>(Box::new(|_| Box::new(TestButton)));
        
        // Test metadata retrieval
        let metadata = registry.get_metadata("Button").unwrap();
        assert_eq!(metadata.component_type, "Button");
        assert_eq!(metadata.display_name, "Button");
        
        // Test schema retrieval
        let schema = registry.get_schema("Button").unwrap();
        assert!(schema.properties.contains_key("text"));
        assert!(schema.properties.contains_key("enabled"));
    }

    #[test]
    fn test_inspector_generation() {
        let registry = ComponentRegistry::new();
        let schema = TestButton::schema();
        
        let inspector = registry.generate_inspector(&schema);
        assert!(!inspector.sections.is_empty());
        
        let section = &inspector.sections[0];
        assert_eq!(section.controls.len(), 2); // text and enabled
    }

    #[test]
    fn test_property_validation() {
        let mut registry = ComponentRegistry::new();
        registry.register_component::<TestButton>(Box::new(|_| Box::new(TestButton)));
        
        let mut properties = HashMap::new();
        properties.insert("text".to_string(), PropertyValue::String("Test".to_string()));
        properties.insert("enabled".to_string(), PropertyValue::Boolean(true));
        
        let errors = registry.validate_properties("Button", &properties);
        assert!(errors.is_empty());
        
        // Test validation error
        properties.insert("text".to_string(), PropertyValue::String("a".repeat(150))); // Too long
        let errors = registry.validate_properties("Button", &properties);
        assert!(!errors.is_empty());
    }
}