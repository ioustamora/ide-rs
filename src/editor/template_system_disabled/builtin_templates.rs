//! Built-in Template Definitions
//!
//! Provides default templates for common UI components.

use std::collections::BTreeMap;
use super::template::{ComponentTemplate, TemplateProperty, PropertyType, PropertyValue, PropertyValidation};

/// Create built-in templates
pub fn create_builtin_templates() -> Vec<ComponentTemplate> {
    vec![
        create_button_template(),
        create_label_template(),
        create_textbox_template(),
        create_checkbox_template(),
        create_slider_template(),
        create_dropdown_template(),
    ]
}

/// Create button template
fn create_button_template() -> ComponentTemplate {
    let mut template = ComponentTemplate::new("builtin_button".to_string(), "Button".to_string());
    template.description = "Standard button component".to_string();
    template.category = "Basic".to_string();
    
    // Add basic properties
    template.add_property(create_text_property("text", "Button"));
    template.add_property(create_bool_property("enabled", true));
    template.add_property(create_bool_property("visible", true));
    
    template
}

/// Create label template
fn create_label_template() -> ComponentTemplate {
    let mut template = ComponentTemplate::new("builtin_label".to_string(), "Label".to_string());
    template.description = "Text label component".to_string();
    template.category = "Basic".to_string();
    
    template.add_property(create_text_property("text", "Label"));
    template.add_property(create_bool_property("visible", true));
    
    template
}

/// Create textbox template
fn create_textbox_template() -> ComponentTemplate {
    let mut template = ComponentTemplate::new("builtin_textbox".to_string(), "TextBox".to_string());
    template.description = "Text input component".to_string();
    template.category = "Basic".to_string();
    
    template.add_property(create_text_property("text", ""));
    template.add_property(create_text_property("placeholder", "Enter text..."));
    template.add_property(create_bool_property("enabled", true));
    template.add_property(create_bool_property("visible", true));
    
    template
}

/// Create checkbox template
fn create_checkbox_template() -> ComponentTemplate {
    let mut template = ComponentTemplate::new("builtin_checkbox".to_string(), "Checkbox".to_string());
    template.description = "Checkbox component".to_string();
    template.category = "Basic".to_string();
    
    template.add_property(create_text_property("text", "Checkbox"));
    template.add_property(create_bool_property("checked", false));
    template.add_property(create_bool_property("enabled", true));
    template.add_property(create_bool_property("visible", true));
    
    template
}

/// Create slider template
fn create_slider_template() -> ComponentTemplate {
    let mut template = ComponentTemplate::new("builtin_slider".to_string(), "Slider".to_string());
    template.description = "Slider component".to_string();
    template.category = "Basic".to_string();
    
    template.add_property(create_float_property("value", 50.0));
    template.add_property(create_float_property("min", 0.0));
    template.add_property(create_float_property("max", 100.0));
    template.add_property(create_bool_property("enabled", true));
    template.add_property(create_bool_property("visible", true));
    
    template
}

/// Create dropdown template
fn create_dropdown_template() -> ComponentTemplate {
    let mut template = ComponentTemplate::new("builtin_dropdown".to_string(), "Dropdown".to_string());
    template.description = "Dropdown selection component".to_string();
    template.category = "Basic".to_string();
    
    template.add_property(create_text_property("selected_text", "Select..."));
    template.add_property(create_int_property("selected_index", 0));
    template.add_property(create_bool_property("enabled", true));
    template.add_property(create_bool_property("visible", true));
    
    template
}

/// Helper to create text property
fn create_text_property(name: &str, default_value: &str) -> TemplateProperty {
    TemplateProperty {
        name: name.to_string(),
        property_type: PropertyType::String,
        default_value: PropertyValue::String(default_value.to_string()),
        overridable: true,
        inherited: false,
        validation: PropertyValidation::default(),
        description: format!("{} property", name),
        category: "General".to_string(),
        designer_visible: true,
    }
}

/// Helper to create boolean property
fn create_bool_property(name: &str, default_value: bool) -> TemplateProperty {
    TemplateProperty {
        name: name.to_string(),
        property_type: PropertyType::Boolean,
        default_value: PropertyValue::Boolean(default_value),
        overridable: true,
        inherited: false,
        validation: PropertyValidation::default(),
        description: format!("{} property", name),
        category: "General".to_string(),
        designer_visible: true,
    }
}

/// Helper to create integer property
fn create_int_property(name: &str, default_value: i32) -> TemplateProperty {
    TemplateProperty {
        name: name.to_string(),
        property_type: PropertyType::Integer,
        default_value: PropertyValue::Integer(default_value),
        overridable: true,
        inherited: false,
        validation: PropertyValidation::default(),
        description: format!("{} property", name),
        category: "General".to_string(),
        designer_visible: true,
    }
}

/// Helper to create float property
fn create_float_property(name: &str, default_value: f32) -> TemplateProperty {
    TemplateProperty {
        name: name.to_string(),
        property_type: PropertyType::Float,
        default_value: PropertyValue::Float(default_value),
        overridable: true,
        inherited: false,
        validation: PropertyValidation::default(),
        description: format!("{} property", name),
        category: "General".to_string(),
        designer_visible: true,
    }
}