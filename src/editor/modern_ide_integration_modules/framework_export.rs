//! Framework Export System
//!
//! This module handles exporting designs to various web frameworks
//! including React, Vue, Angular, and others.

use egui::*;
use std::collections::HashMap;
use crate::editor::inspector::PropertyValue;

/// Framework export manager
#[derive(Clone, Debug)]
pub struct FrameworkExportManager {
    /// Available export targets
    pub targets: Vec<ExportTarget>,
    /// Export settings
    pub settings: ExportSettings,
    /// Template engine for code generation
    pub template_engine: TemplateEngine,
}

/// Export target definition
#[derive(Clone, Debug)]
pub struct ExportTarget {
    /// Target framework name
    pub name: String,
    /// Target description
    pub description: String,
    /// File extension for generated files
    pub file_extension: String,
    /// Template path for this target
    pub template_path: String,
    /// Whether this target supports TypeScript
    pub typescript_support: bool,
    /// Framework version
    pub version: String,
    /// Additional dependencies
    pub dependencies: Vec<String>,
}

/// Export settings
#[derive(Clone, Debug)]
pub struct ExportSettings {
    /// Output directory
    pub output_directory: String,
    /// Whether to use TypeScript
    pub use_typescript: bool,
    /// Code formatting options
    pub formatting: FormattingOptions,
    /// Whether to generate CSS modules
    pub css_modules: bool,
    /// Whether to generate Storybook stories
    pub generate_stories: bool,
    /// Whether to generate tests
    pub generate_tests: bool,
    /// Component naming convention
    pub naming_convention: NamingConvention,
}

/// Code formatting options
#[derive(Clone, Debug)]
pub struct FormattingOptions {
    /// Indentation type
    pub indent_type: IndentType,
    /// Indentation size
    pub indent_size: u32,
    /// Maximum line length
    pub max_line_length: u32,
    /// Whether to use semicolons
    pub semicolons: bool,
    /// Quote style
    pub quote_style: QuoteStyle,
    /// Whether to use trailing commas
    pub trailing_commas: bool,
}

/// Template engine for code generation
#[derive(Clone, Debug)]
pub struct TemplateEngine {
    /// Template registry
    pub templates: HashMap<String, CodeTemplate>,
    /// Generator settings
    pub settings: GeneratorSettings,
}

/// Code template definition
#[derive(Clone, Debug)]
pub struct CodeTemplate {
    /// Template name
    pub name: String,
    /// Template content
    pub content: String,
    /// Template variables
    pub variables: Vec<String>,
    /// Template description
    pub description: String,
}

/// Code generator settings
#[derive(Clone, Debug)]
pub struct GeneratorSettings {
    /// Component prefix
    pub component_prefix: String,
    /// Component suffix
    pub component_suffix: String,
    /// Props interface suffix
    pub props_suffix: String,
    /// Style class prefix
    pub style_prefix: String,
}

/// Indentation type
#[derive(Clone, Debug)]
pub enum IndentType {
    Spaces,
    Tabs,
}

/// Quote style
#[derive(Clone, Debug)]
pub enum QuoteStyle {
    Single,
    Double,
}

/// Naming convention
#[derive(Clone, Debug, PartialEq)]
pub enum NamingConvention {
    PascalCase,
    CamelCase,
    KebabCase,
    SnakeCase,
}

impl Default for FrameworkExportManager {
    fn default() -> Self {
        let mut manager = Self {
            targets: Vec::new(),
            settings: ExportSettings::default(),
            template_engine: TemplateEngine::default(),
        };
        
        manager.initialize_default_targets();
        manager
    }
}

impl Default for ExportSettings {
    fn default() -> Self {
        Self {
            output_directory: "./output".to_string(),
            use_typescript: true,
            formatting: FormattingOptions::default(),
            css_modules: true,
            generate_stories: false,
            generate_tests: false,
            naming_convention: NamingConvention::PascalCase,
        }
    }
}

impl Default for FormattingOptions {
    fn default() -> Self {
        Self {
            indent_type: IndentType::Spaces,
            indent_size: 2,
            max_line_length: 100,
            semicolons: true,
            quote_style: QuoteStyle::Single,
            trailing_commas: true,
        }
    }
}

impl Default for TemplateEngine {
    fn default() -> Self {
        let mut engine = Self {
            templates: HashMap::new(),
            settings: GeneratorSettings::default(),
        };
        
        engine.initialize_default_templates();
        engine
    }
}

impl Default for GeneratorSettings {
    fn default() -> Self {
        Self {
            component_prefix: "".to_string(),
            component_suffix: "Component".to_string(),
            props_suffix: "Props".to_string(),
            style_prefix: "".to_string(),
        }
    }
}

impl FrameworkExportManager {
    /// Create a new framework export manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Initialize default export targets
    fn initialize_default_targets(&mut self) {
        // React target
        self.targets.push(ExportTarget {
            name: "React".to_string(),
            description: "Export to React components".to_string(),
            file_extension: "tsx".to_string(),
            template_path: "templates/react.hbs".to_string(),
            typescript_support: true,
            version: "18.x".to_string(),
            dependencies: vec!["react".to_string(), "@types/react".to_string()],
        });
        
        // Vue target
        self.targets.push(ExportTarget {
            name: "Vue".to_string(),
            description: "Export to Vue 3 components".to_string(),
            file_extension: "vue".to_string(),
            template_path: "templates/vue.hbs".to_string(),
            typescript_support: true,
            version: "3.x".to_string(),
            dependencies: vec!["vue".to_string()],
        });
        
        // Angular target
        self.targets.push(ExportTarget {
            name: "Angular".to_string(),
            description: "Export to Angular components".to_string(),
            file_extension: "ts".to_string(),
            template_path: "templates/angular.hbs".to_string(),
            typescript_support: true,
            version: "16.x".to_string(),
            dependencies: vec!["@angular/core".to_string()],
        });
    }
    
    /// Export components to target framework
    pub fn export_to_framework(
        &self,
        target_name: &str,
        components: &[ComponentData],
    ) -> Result<Vec<GeneratedFile>, ExportError> {
        let target = self.targets.iter()
            .find(|t| t.name == target_name)
            .ok_or(ExportError::TargetNotFound(target_name.to_string()))?;
        
        let mut generated_files = Vec::new();
        
        for component in components {
            let generated_code = self.generate_component_code(target, component)?;
            let filename = self.generate_filename(target, &component.component_type);
            
            generated_files.push(GeneratedFile {
                filename,
                content: generated_code,
                target: target.name.clone(),
            });
        }
        
        Ok(generated_files)
    }
    
    /// Generate component code for target framework
    fn generate_component_code(
        &self,
        target: &ExportTarget,
        component: &ComponentData,
    ) -> Result<String, ExportError> {
        let template = self.template_engine.templates.get(&target.name)
            .ok_or(ExportError::TemplateNotFound(target.name.clone()))?;
        
        // Create template context
        let mut context = HashMap::new();
        context.insert("component_name".to_string(), component.component_type.clone());
        context.insert("properties".to_string(), serde_json::to_string(&component.properties).unwrap());
        context.insert("use_typescript".to_string(), self.settings.use_typescript.to_string());
        
        // Simple template substitution (in a real implementation, use a proper template engine)
        let mut code = template.content.clone();
        for (key, value) in context {
            code = code.replace(&format!("{{{{{}}}}}", key), &value);
        }
        
        Ok(code)
    }
    
    /// Generate filename for component
    fn generate_filename(&self, target: &ExportTarget, component_name: &str) -> String {
        let formatted_name = self.format_component_name(component_name);
        format!("{}.{}", formatted_name, target.file_extension)
    }
    
    /// Format component name according to naming convention
    fn format_component_name(&self, name: &str) -> String {
        match self.settings.naming_convention {
            NamingConvention::PascalCase => self.to_pascal_case(name),
            NamingConvention::CamelCase => self.to_camel_case(name),
            NamingConvention::KebabCase => self.to_kebab_case(name),
            NamingConvention::SnakeCase => self.to_snake_case(name),
        }
    }
    
    /// Convert to PascalCase
    fn to_pascal_case(&self, s: &str) -> String {
        s.chars()
            .enumerate()
            .map(|(i, c)| {
                if i == 0 || s.chars().nth(i - 1) == Some('_') || s.chars().nth(i - 1) == Some('-') {
                    c.to_uppercase().collect::<String>()
                } else if c == '_' || c == '-' {
                    String::new()
                } else {
                    c.to_lowercase().collect::<String>()
                }
            })
            .collect()
    }
    
    /// Convert to camelCase
    fn to_camel_case(&self, s: &str) -> String {
        let pascal = self.to_pascal_case(s);
        pascal.chars()
            .enumerate()
            .map(|(i, c)| {
                if i == 0 {
                    c.to_lowercase().collect::<String>()
                } else {
                    c.to_string()
                }
            })
            .collect()
    }
    
    /// Convert to kebab-case
    fn to_kebab_case(&self, s: &str) -> String {
        s.chars()
            .map(|c| {
                if c.is_uppercase() {
                    format!("-{}", c.to_lowercase())
                } else if c == '_' {
                    "-".to_string()
                } else {
                    c.to_string()
                }
            })
            .collect::<String>()
            .trim_start_matches('-')
            .to_string()
    }
    
    /// Convert to snake_case
    fn to_snake_case(&self, s: &str) -> String {
        s.chars()
            .map(|c| {
                if c.is_uppercase() {
                    format!("_{}", c.to_lowercase())
                } else if c == '-' {
                    "_".to_string()
                } else {
                    c.to_string()
                }
            })
            .collect::<String>()
            .trim_start_matches('_')
            .to_string()
    }
    
    /// Render export UI
    pub fn render_ui(&mut self, ui: &mut Ui) {
        ui.heading("Framework Export");
        
        ui.horizontal(|ui| {
            ui.label("Target Framework:");
            egui::ComboBox::from_label("")
                .selected_text("Select Framework")
                .show_ui(ui, |ui| {
                    for target in &self.targets {
                        ui.selectable_value(&mut "", &target.name, &target.name);
                    }
                });
        });
        
        ui.separator();
        
        ui.collapsing("Export Settings", |ui| {
            ui.checkbox(&mut self.settings.use_typescript, "Use TypeScript");
            ui.checkbox(&mut self.settings.css_modules, "Generate CSS Modules");
            ui.checkbox(&mut self.settings.generate_stories, "Generate Storybook Stories");
            ui.checkbox(&mut self.settings.generate_tests, "Generate Tests");
            
            ui.horizontal(|ui| {
                ui.label("Naming Convention:");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", self.settings.naming_convention))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.settings.naming_convention, NamingConvention::PascalCase, "PascalCase");
                        ui.selectable_value(&mut self.settings.naming_convention, NamingConvention::CamelCase, "camelCase");
                        ui.selectable_value(&mut self.settings.naming_convention, NamingConvention::KebabCase, "kebab-case");
                        ui.selectable_value(&mut self.settings.naming_convention, NamingConvention::SnakeCase, "snake_case");
                    });
            });
        });
        
        ui.separator();
        
        if ui.button("Export Components").clicked() {
            // Handle export
        }
    }
}

impl TemplateEngine {
    /// Initialize default templates
    fn initialize_default_templates(&mut self) {
        // React template
        self.templates.insert("React".to_string(), CodeTemplate {
            name: "React".to_string(),
            content: r#"import React from 'react';

interface {{component_name}}Props {
  // Add props here
}

const {{component_name}}: React.FC<{{component_name}}Props> = (props) => {
  return (
    <div className="{{component_name}}">
      {/* Component content */}
    </div>
  );
};

export default {{component_name}};"#.to_string(),
            variables: vec!["component_name".to_string()],
            description: "React functional component template".to_string(),
        });
        
        // Vue template
        self.templates.insert("Vue".to_string(), CodeTemplate {
            name: "Vue".to_string(),
            content: r#"<template>
  <div class="{{component_name}}">
    <!-- Component content -->
  </div>
</template>

<script setup lang="ts">
// Component logic
</script>

<style scoped>
.{{component_name}} {
  /* Component styles */
}
</style>"#.to_string(),
            variables: vec!["component_name".to_string()],
            description: "Vue 3 composition API component template".to_string(),
        });
    }
}

/// Component data for export
#[derive(Clone, Debug)]
pub struct ComponentData {
    /// Component type name
    pub component_type: String,
    /// Component properties
    pub properties: HashMap<String, PropertyValue>,
    /// Component position
    pub position: Option<Pos2>,
    /// Component size
    pub size: Option<Vec2>,
    /// Child components
    pub children: Vec<ComponentData>,
}

/// Generated file structure
#[derive(Clone, Debug)]
pub struct GeneratedFile {
    /// Filename
    pub filename: String,
    /// File content
    pub content: String,
    /// Target framework
    pub target: String,
}

/// Export error types
#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error("Target framework not found: {0}")]
    TargetNotFound(String),
    #[error("Template not found for framework: {0}")]
    TemplateNotFound(String),
    #[error("Code generation failed: {0}")]
    GenerationFailed(String),
}