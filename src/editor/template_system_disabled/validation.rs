//! Template Validation System
//!
//! Validates templates and their properties for correctness and consistency.

use std::collections::HashMap;
use super::template::ComponentTemplate;
use super::inheritance::InheritanceTree;

/// Template validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Validation passed
    pub valid: bool,
    /// Error messages
    pub errors: Vec<String>,
    /// Warning messages
    pub warnings: Vec<String>,
    /// Info messages
    pub info: Vec<String>,
}

/// Template validator
#[derive(Debug, Clone)]
pub struct TemplateValidator {
    /// Validation rules
    pub rules: ValidationRules,
}

/// Validation rules configuration
#[derive(Debug, Clone)]
pub struct ValidationRules {
    /// Require template name
    pub require_name: bool,
    /// Require template ID
    pub require_id: bool,
    /// Maximum template name length
    pub max_name_length: usize,
    /// Validate property names
    pub validate_property_names: bool,
}

impl ValidationResult {
    /// Create a successful validation result
    pub fn success() -> Self {
        Self {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            info: Vec::new(),
        }
    }
    
    /// Create an error validation result
    pub fn error(message: String) -> Self {
        Self {
            valid: false,
            errors: vec![message],
            warnings: Vec::new(),
            info: Vec::new(),
        }
    }
    
    /// Add an error
    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
        self.valid = false;
    }
    
    /// Add a warning
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
    
    /// Add an info message
    pub fn add_info(&mut self, info: String) {
        self.info.push(info);
    }
}

impl TemplateValidator {
    /// Create a new validator with default rules
    pub fn new() -> Self {
        Self {
            rules: ValidationRules::default(),
        }
    }
    
    /// Validate a template
    pub fn validate_template(
        &self,
        template: &ComponentTemplate,
        _all_templates: &HashMap<String, ComponentTemplate>,
        _inheritance_tree: &InheritanceTree,
    ) -> ValidationResult {
        let mut result = ValidationResult::success();
        
        // Validate basic template info
        self.validate_basic_info(template, &mut result);
        
        // Validate properties
        self.validate_properties(template, &mut result);
        
        result
    }
    
    /// Validate basic template information
    fn validate_basic_info(&self, template: &ComponentTemplate, result: &mut ValidationResult) {
        if self.rules.require_name && template.name.trim().is_empty() {
            result.add_error("Template name is required".to_string());
        }
        
        if self.rules.require_id && template.id.trim().is_empty() {
            result.add_error("Template ID is required".to_string());
        }
        
        if template.name.len() > self.rules.max_name_length {
            result.add_error(format!(
                "Template name exceeds maximum length of {} characters",
                self.rules.max_name_length
            ));
        }
        
        if !template.id.chars().all(|c| c.is_alphanumeric() || c == '_') {
            result.add_error("Template ID can only contain alphanumeric characters and underscores".to_string());
        }
    }
    
    /// Validate template properties
    fn validate_properties(&self, template: &ComponentTemplate, result: &mut ValidationResult) {
        for (name, property) in &template.properties {
            if self.rules.validate_property_names && !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                result.add_error(format!("Property name '{}' contains invalid characters", name));
            }
            
            if name != &property.name {
                result.add_error(format!("Property key '{}' doesn't match property name '{}'", name, property.name));
            }
        }
    }
}

impl Default for ValidationRules {
    fn default() -> Self {
        Self {
            require_name: true,
            require_id: true,
            max_name_length: 100,
            validate_property_names: true,
        }
    }
}

impl Default for TemplateValidator {
    fn default() -> Self {
        Self::new()
    }
}