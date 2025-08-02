//! Template Serialization System
//!
//! Handles serialization and deserialization of templates in various formats.

use serde_json;
use super::template::ComponentTemplate;

/// Serialization formats
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SerializationFormat {
    Json,
    JsonPretty,
}

/// Template serializer
#[derive(Debug, Clone)]
pub struct TemplateSerializer;

impl TemplateSerializer {
    /// Create a new serializer
    pub fn new() -> Self {
        Self
    }
    
    /// Serialize a template to string
    pub fn serialize_template(
        &self,
        template: &ComponentTemplate,
        format: SerializationFormat,
    ) -> Result<String, Box<dyn std::error::Error>> {
        match format {
            SerializationFormat::Json => {
                Ok(serde_json::to_string(template)?)
            }
            SerializationFormat::JsonPretty => {
                Ok(serde_json::to_string_pretty(template)?)
            }
        }
    }
    
    /// Deserialize a template from string
    pub fn deserialize_template(
        &self,
        data: &str,
        _format: SerializationFormat,
    ) -> Result<ComponentTemplate, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(data)?)
    }
}

impl Default for TemplateSerializer {
    fn default() -> Self {
        Self::new()
    }
}