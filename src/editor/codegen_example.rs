//! Comprehensive Example of Enhanced Code Generation System
//!
//! This example demonstrates the complete codegen markers and rewrite prototype,
//! showing how the system integrates derive macros, enhanced markers, and intelligent
//! code rewriting to create a powerful code generation framework.

use super::enhanced_codegen::{
    EnhancedCodeGenerator, EnhancedTemplateBuilder, ContentGenerator, GenerationStatistics
};
use super::codegen_markers::{CodeLanguage, MarkerType, CodeRewriter};
use std::collections::HashMap;
use serde_json::{Value, json};
use tempfile::TempDir;

/// Example demonstrating the complete code generation workflow
#[allow(dead_code)]
pub struct CodeGenExample {
    generator: EnhancedCodeGenerator,
    temp_dir: TempDir,
}

impl CodeGenExample {
    /// Create a new example instance
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let generator = EnhancedCodeGenerator::new(temp_dir.path().to_path_buf());
        
        Ok(Self {
            generator,
            temp_dir,
        })
    }
    
    /// Demonstrate the complete workflow
    pub fn run_complete_example(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Running Enhanced Code Generation Example");
        
        // Step 1: Create enhanced templates
        self.create_component_template();
        self.create_module_template();
        
        // Step 2: Generate initial code
        let component_code = self.generate_component_code()?;
        let module_code = self.generate_module_code()?;
        
        println!("📝 Generated initial code:");
        println!("Component code:\n{}", component_code);
        println!("Module code:\n{}", module_code);
        
        // Step 3: Simulate user modifications
        self.simulate_user_modifications()?;
        
        // Step 4: Regenerate with preserved user code
        let regenerated_code = self.regenerate_with_preserved_code()?;
        println!("🔄 Regenerated code with preserved user modifications:\n{}", regenerated_code);
        
        // Step 5: Show statistics
        self.show_statistics();
        
        println!("✅ Enhanced Code Generation Example completed successfully!");
        Ok(())
    }
    
    /// Create a template for React-like components
    fn create_component_template(&mut self) {
        let template_content = r#"
import React from 'react';
// <codegen:import:dependencies:start>
// <codegen:import:dependencies:end>

interface {{component_name}}Props {
    // <codegen:generated:props:start>
    // <codegen:generated:props:end>
}

const {{component_name}}: React.FC<{{component_name}}Props> = (props) => {
    // <codegen:guard:component_logic:start>
    // Add your component logic here
    // <codegen:guard:component_logic:end>
    
    return (
        <div className="{{component_name}}">
            {/* <codegen:template:render_props:start> */}
            {/* <codegen:template:render_props:end> */}
            
            {/* <codegen:guard:custom_jsx:start> */}
            {/* Add your custom JSX here */}
            {/* <codegen:guard:custom_jsx:end> */}
        </div>
    );
};

// <codegen:guard:additional_exports:start>
// <codegen:guard:additional_exports:end>

export default {{component_name}};
"#;
        
        let template = EnhancedTemplateBuilder::new(
            "react_component".to_string(),
            template_content.to_string()
        )
        .support_languages(vec![CodeLanguage::TypeScript, CodeLanguage::JavaScript])
        .add_guard(
            "component_logic".to_string(),
            10,
            Some("// Add your component logic here".to_string())
        )
        .add_guard(
            "custom_jsx".to_string(),
            18,
            Some("// Add your custom JSX here".to_string())
        )
        .add_guard(
            "additional_exports".to_string(),
            24,
            None
        )
        .add_generated(
            "props".to_string(),
            6,
            ContentGenerator::Component {
                component_type: "Button".to_string(),
                properties: json!({
                    "text": "string",
                    "disabled": "boolean",
                    "onClick": "function"
                }).as_object().unwrap().clone()
            }
        )
        .build();
        
        self.generator.register_enhanced_template(template);
    }
    
    /// Create a template for Rust modules
    fn create_module_template(&mut self) {
        let template_content = r#"
//! {{module_description}}

// <codegen:import:dependencies:start>
// <codegen:import:dependencies:end>

/// {{struct_name}} component
#[derive(Debug, Clone)]
pub struct {{struct_name}} {
    // <codegen:generated:fields:start>
    // <codegen:generated:fields:end>
}

impl {{struct_name}} {
    /// Create a new {{struct_name}}
    pub fn new() -> Self {
        Self {
            // <codegen:generated:field_defaults:start>
            // <codegen:generated:field_defaults:end>
        }
    }
    
    // <codegen:guard:custom_methods:start>
    // Add your custom methods here
    // <codegen:guard:custom_methods:end>
}

// <codegen:guard:additional_implementations:start>
// Add additional trait implementations here
// <codegen:guard:additional_implementations:end>

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_{{struct_name_lower}}_creation() {
        let instance = {{struct_name}}::new();
        // <codegen:guard:test_assertions:start>
        // Add your test assertions here
        // <codegen:guard:test_assertions:end>
    }
    
    // <codegen:guard:additional_tests:start>
    // <codegen:guard:additional_tests:end>
}
"#;
        
        let template = EnhancedTemplateBuilder::new(
            "rust_module".to_string(),
            template_content.to_string()
        )
        .support_languages(vec![CodeLanguage::Rust])
        .add_guard(
            "custom_methods".to_string(),
            24,
            Some("    // Add your custom methods here".to_string())
        )
        .add_guard(
            "additional_implementations".to_string(),
            29,
            Some("// Add additional trait implementations here".to_string())
        )
        .add_guard(
            "test_assertions".to_string(),
            37,
            Some("        // Add your test assertions here".to_string())
        )
        .add_guard(
            "additional_tests".to_string(),
            42,
            None
        )
        .add_generated(
            "fields".to_string(),
            11,
            ContentGenerator::Template {
                template: "    pub {{field_name}}: {{field_type}},".to_string(),
                variables: {
                    let mut vars = HashMap::new();
                    vars.insert("field_name".to_string(), super::codegen_markers::ParameterType::String);
                    vars.insert("field_type".to_string(), super::codegen_markers::ParameterType::String);
                    vars
                }
            }
        )
        .build();
        
        self.generator.register_enhanced_template(template);
    }
    
    /// Generate component code using the template
    fn generate_component_code(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let variables = json!({
            "component_name": "UserProfile",
            "field_name": "name",
            "field_type": "String"
        }).as_object().unwrap().clone();
        
        let output_file = self.temp_dir.path().join("UserProfile.tsx");
        let code = self.generator.generate_enhanced_code(
            "react_component",
            variables,
            output_file
        )?;
        
        Ok(code)
    }
    
    /// Generate module code using the template
    fn generate_module_code(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let variables = json!({
            "module_description": "User profile management module",
            "struct_name": "UserProfile",
            "struct_name_lower": "user_profile",
            "field_name": "name",
            "field_type": "String"
        }).as_object().unwrap().clone();
        
        let output_file = self.temp_dir.path().join("user_profile.rs");
        let code = self.generator.generate_enhanced_code(
            "rust_module",
            variables,
            output_file
        )?;
        
        Ok(code)
    }
    
    /// Simulate user modifications to guard sections
    fn simulate_user_modifications(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // This simulates a user editing the generated files
        let modified_rust_code = r#"
//! User profile management module

// <codegen:import:dependencies:start>
use serde::{Serialize, Deserialize};
// <codegen:import:dependencies:end>

/// UserProfile component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    // <codegen:generated:fields:start>
    pub name: String,
    // <codegen:generated:fields:end>
}

impl UserProfile {
    /// Create a new UserProfile
    pub fn new() -> Self {
        Self {
            // <codegen:generated:field_defaults:start>
            name: String::new(),
            // <codegen:generated:field_defaults:end>
        }
    }
    
    // <codegen:guard:custom_methods:start>
    /// Get the display name for this user
    pub fn display_name(&self) -> &str {
        &self.name
    }
    
    /// Set the user's name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    // <codegen:guard:custom_methods:end>
}

// <codegen:guard:additional_implementations:start>
impl Default for UserProfile {
    fn default() -> Self {
        Self::new()
    }
}
// <codegen:guard:additional_implementations:end>

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_user_profile_creation() {
        let instance = UserProfile::new();
        // <codegen:guard:test_assertions:start>
        assert_eq!(instance.name, "");
        assert_eq!(instance.display_name(), "");
        // <codegen:guard:test_assertions:end>
    }
    
    // <codegen:guard:additional_tests:start>
    #[test]
    fn test_user_profile_name_setting() {
        let mut profile = UserProfile::new();
        profile.set_name("John Doe".to_string());
        assert_eq!(profile.display_name(), "John Doe");
    }
    // <codegen:guard:additional_tests:end>
}
"#;
        
        // Write the modified code to file
        let rust_file = self.temp_dir.path().join("user_profile.rs");
        std::fs::write(&rust_file, modified_rust_code)?;
        
        // Parse the modifications
        let mut rewriter = CodeRewriter::new(CodeLanguage::Rust, modified_rust_code.to_string());
        rewriter.parse_markers().map_err(|e| format!("Failed to parse markers: {}", e))?;
        
        println!("📝 Simulated user modifications:");
        println!("- Added serde imports");
        println!("- Added custom methods: display_name(), set_name()");
        println!("- Added Default trait implementation");
        println!("- Added comprehensive tests");
        
        Ok(())
    }
    
    /// Regenerate code while preserving user modifications
    fn regenerate_with_preserved_code(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        // Regenerate with updated variables (simulating component changes)
        let updated_variables = json!({
            "module_description": "Enhanced user profile management module with validation",
            "struct_name": "UserProfile",
            "struct_name_lower": "user_profile",
            "field_name": "email",  // Changed field
            "field_type": "Option<String>"  // Changed type
        }).as_object().unwrap().clone();
        
        let output_file = self.temp_dir.path().join("user_profile.rs");
        let regenerated_code = self.generator.generate_enhanced_code(
            "rust_module",
            updated_variables,
            output_file
        )?;
        
        println!("🔄 Regeneration preserved:");
        println!("- User's custom methods");
        println!("- User's trait implementations");  
        println!("- User's test cases");
        println!("- User's import additions");
        
        Ok(regenerated_code)
    }
    
    /// Show generation statistics
    fn show_statistics(&self) {
        let stats = self.generator.get_statistics();
        
        println!("📊 Code Generation Statistics:");
        println!("  - Templates registered: {}", stats.total_templates);
        println!("  - Files managed: {}", stats.total_files);
        println!("  - Total markers: {}", stats.total_markers);
        println!("  - User-modified markers: {}", stats.modified_markers);
        
        if stats.total_markers > 0 {
            let modification_rate = (stats.modified_markers as f64 / stats.total_markers as f64) * 100.0;
            println!("  - Modification rate: {:.1}%", modification_rate);
        }
    }
}

/// Example of advanced marker types
#[allow(dead_code)]
pub fn demonstrate_advanced_markers() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Demonstrating Advanced Marker Types");
    
    // Example code with various marker types
    let complex_code = r#"
use std::collections::HashMap;

// <codegen:import:Module:start>
// Generated imports will appear here
// <codegen:import:Module:end>

// <codegen:conditional:debug_mode:start>
// Debug-only code when DEBUG=true
// <codegen:conditional:debug_mode:end>

pub struct ComponentRegistry {
    // <codegen:generated:registry_fields:start>
    // Auto-generated registry fields
    // <codegen:generated:registry_fields:end>
    
    // <codegen:guard:custom_fields:start>
    // User can add custom fields here
    custom_data: HashMap<String, String>,
    // <codegen:guard:custom_fields:end>
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self {
            // <codegen:template:field_initialization:start>
            // Template-based field initialization
            // <codegen:template:field_initialization:end>
            
            // <codegen:guard:custom_initialization:start>
            custom_data: HashMap::new(),
            // <codegen:guard:custom_initialization:end>
        }
    }
    
    // <codegen:guard:user_methods:start>
    // Users can add methods here - preserved across regenerations
    // <codegen:guard:user_methods:end>
}
"#;
    
    let mut rewriter = CodeRewriter::new(CodeLanguage::Rust, complex_code.to_string());
    rewriter.parse_markers()?;
    
    println!("🔍 Parsed {} markers from complex code:", rewriter.markers.len());
    
    for marker in &rewriter.markers {
        let marker_type_name = match &marker.marker_type {
            MarkerType::Guard { id, .. } => format!("Guard({})", id),
            MarkerType::Generated { id, .. } => format!("Generated({})", id),
            MarkerType::Conditional { id, .. } => format!("Conditional({})", id),
            MarkerType::Import { import_type, .. } => format!("Import({:?})", import_type),
            MarkerType::Template { id, .. } => format!("Template({})", id),
        };
        
        println!("  - {} at line {}", marker_type_name, marker.start_position.line);
    }
    
    println!("✅ Advanced marker demonstration completed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_code_gen_example_creation() {
        let example = CodeGenExample::new();
        assert!(example.is_ok());
    }
    
    #[test]
    fn test_advanced_marker_demonstration() {
        let result = demonstrate_advanced_markers();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_template_creation() {
        let mut example = CodeGenExample::new().unwrap();
        example.create_component_template();
        example.create_module_template();
        
        let stats = example.generator.get_statistics();
        assert_eq!(stats.total_templates, 2);
    }
}