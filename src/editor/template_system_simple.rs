//! Simplified Template System (Placeholder)
//!
//! This is a simplified version of the template system to maintain compilation
//! while the full modular version is being developed.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Simple component template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub version: String,
    pub parent_template: Option<String>,
    pub properties: HashMap<String, TemplateProperty>,
}

/// Simple template property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateProperty {
    pub name: String,
    pub property_type: PropertyType,
    pub default_value: PropertyValue,
    pub overridable: bool,
    pub inherited: bool,
    pub validation: PropertyValidation,
    pub description: String,
    pub category: String,
    pub designer_visible: bool,
}

/// Simple property types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PropertyType {
    String,
    Integer,
    Float,
    Boolean,
    Color,
}

/// Simple property values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyValue {
    String(String),
    Integer(i32),
    Float(f32),
    Boolean(bool),
    Color([u8; 4]),
}

/// Simple property validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyValidation {
    pub required: bool,
    pub min_value: Option<f32>,
    pub max_value: Option<f32>,
    pub pattern: Option<String>,
    pub valid_options: Option<Vec<String>>,
    pub custom_validation: Option<String>,
}

impl Default for PropertyValidation {
    fn default() -> Self {
        Self {
            required: false,
            min_value: None,
            max_value: None,
            pattern: None,
            valid_options: None,
            custom_validation: None,
        }
    }
}

impl ComponentTemplate {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            description: String::new(),
            category: "General".to_string(),
            version: "1.0.0".to_string(),
            parent_template: None,
            properties: HashMap::new(),
        }
    }
    
    pub fn add_property(&mut self, property: TemplateProperty) {
        self.properties.insert(property.name.clone(), property);
    }
    
    pub fn remove_property(&mut self, name: &str) {
        self.properties.remove(name);
    }
    
    pub fn get_property(&self, name: &str) -> Option<&TemplateProperty> {
        self.properties.get(name)
    }
}