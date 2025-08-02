//! Code Generation Engine
//!
//! This module provides advanced code generation capabilities for
//! creating components, layouts, and complete applications.

use egui::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::editor::inspector::PropertyValue;

/// Code generation engine
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodeGenerator {
    /// Code generation templates
    pub templates: HashMap<String, CodeTemplate>,
    /// Generator settings
    pub settings: GeneratorSettings,
    /// Output formatters
    pub formatters: HashMap<String, CodeFormatter>,
    /// Code optimization rules
    pub optimizations: Vec<OptimizationRule>,
}

/// Code template definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodeTemplate {
    /// Template identifier
    pub id: String,
    /// Template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Template content with placeholders
    pub content: String,
    /// Template variables and their types
    pub variables: HashMap<String, VariableType>,
    /// Template metadata
    pub metadata: TemplateMetadata,
}

/// Code generator settings
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeneratorSettings {
    /// Target language
    pub target_language: String,
    /// Code style preferences
    pub style: CodeStyle,
    /// Component naming conventions
    pub naming: NamingConventions,
    /// Import/export preferences
    pub imports: ImportSettings,
    /// Documentation generation
    pub documentation: DocumentationSettings,
}

/// Code formatter for different languages/frameworks
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodeFormatter {
    /// Formatter name
    pub name: String,
    /// Language/framework this formatter targets
    pub target: String,
    /// Formatting rules
    pub rules: FormattingRules,
}

/// Code optimization rule
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptimizationRule {
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Pattern to match
    pub pattern: String,
    /// Replacement pattern
    pub replacement: String,
    /// Rule priority
    pub priority: u32,
}

/// Template metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TemplateMetadata {
    /// Template author
    pub author: String,
    /// Template version
    pub version: String,
    /// Template tags
    pub tags: Vec<String>,
    /// Template creation date
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Template last modified date
    pub modified_at: chrono::DateTime<chrono::Utc>,
}

/// Variable type for template placeholders
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VariableType {
    String,
    Number,
    Boolean,
    Array(Box<VariableType>),
    Object(HashMap<String, VariableType>),
    Custom(String),
}

/// Code style preferences
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodeStyle {
    /// Indentation type
    pub indentation: IndentationType,
    /// Indentation size
    pub indent_size: u32,
    /// Maximum line length
    pub max_line_length: u32,
    /// Quote style
    pub quote_style: QuoteStyle,
    /// Semicolon usage
    pub semicolons: SemicolonUsage,
    /// Trailing comma usage
    pub trailing_commas: TrailingCommaUsage,
    /// Brace style
    pub brace_style: BraceStyle,
}

/// Naming conventions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NamingConventions {
    /// Component naming
    pub components: NamingCase,
    /// Function naming
    pub functions: NamingCase,
    /// Variable naming
    pub variables: NamingCase,
    /// Constant naming
    pub constants: NamingCase,
    /// File naming
    pub files: NamingCase,
}

/// Import/export settings
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImportSettings {
    /// Import style (named, default, namespace)
    pub style: ImportStyle,
    /// Import organization
    pub organization: ImportOrganization,
    /// Auto-import unused modules
    pub auto_import: bool,
    /// Remove unused imports
    pub remove_unused: bool,
}

/// Documentation generation settings
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocumentationSettings {
    /// Generate JSDoc/TSDoc comments
    pub generate_comments: bool,
    /// Include parameter documentation
    pub document_parameters: bool,
    /// Include return value documentation
    pub document_returns: bool,
    /// Include example usage
    pub include_examples: bool,
    /// Documentation style
    pub style: DocumentationStyle,
}

/// Formatting rules
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FormattingRules {
    /// Prettier configuration (for JavaScript/TypeScript)
    pub prettier_config: Option<PrettierConfig>,
    /// Custom formatting rules
    pub custom_rules: HashMap<String, String>,
}

/// Prettier configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrettierConfig {
    /// Print width
    pub print_width: u32,
    /// Tab width
    pub tab_width: u32,
    /// Use tabs
    pub use_tabs: bool,
    /// Semicolons
    pub semi: bool,
    /// Single quotes
    pub single_quote: bool,
    /// Quote props
    pub quote_props: String,
    /// Trailing comma
    pub trailing_comma: String,
    /// Bracket spacing
    pub bracket_spacing: bool,
}

/// Indentation type
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum IndentationType {
    Spaces,
    Tabs,
}

/// Quote style
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum QuoteStyle {
    Single,
    Double,
    Backtick,
}

/// Semicolon usage
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SemicolonUsage {
    Always,
    Never,
    Auto,
}

/// Trailing comma usage
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TrailingCommaUsage {
    Always,
    Never,
    MultilineOnly,
}

/// Brace style
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BraceStyle {
    SameLine,
    NextLine,
    Allman,
    Stroustrup,
}

/// Naming case conventions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NamingCase {
    CamelCase,
    PascalCase,
    SnakeCase,
    KebabCase,
    ScreamingSnakeCase,
}

/// Import style
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ImportStyle {
    Named,
    Default,
    Namespace,
    Mixed,
}

/// Import organization
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ImportOrganization {
    Alphabetical,
    GroupedByType,
    CustomOrder(Vec<String>),
}

/// Documentation style
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DocumentationStyle {
    JSDoc,
    TSDoc,
    Rustdoc,
    Custom(String),
}

/// Generated code result
#[derive(Clone, Debug)]
pub struct GeneratedCode {
    /// Generated code content
    pub content: String,
    /// Language/framework
    pub language: String,
    /// Suggested filename
    pub filename: String,
    /// Import statements
    pub imports: Vec<String>,
    /// Export statements
    pub exports: Vec<String>,
}

/// Code generation context
#[derive(Clone, Debug)]
pub struct GenerationContext {
    /// Component data
    pub component: ComponentGenerationData,
    /// Target framework
    pub framework: String,
    /// Additional context variables
    pub variables: HashMap<String, ContextValue>,
}

/// Component data for generation
#[derive(Clone, Debug)]
pub struct ComponentGenerationData {
    /// Component name
    pub name: String,
    /// Component type
    pub component_type: String,
    /// Component properties
    pub properties: HashMap<String, PropertyValue>,
    /// Component children
    pub children: Vec<ComponentGenerationData>,
    /// Component layout information
    pub layout: Option<LayoutInfo>,
}

/// Layout information for generation
#[derive(Clone, Debug)]
pub struct LayoutInfo {
    /// Position
    pub position: Option<Pos2>,
    /// Size
    pub size: Option<Vec2>,
    /// Z-index
    pub z_index: Option<i32>,
    /// Layout constraints
    pub constraints: HashMap<String, String>,
}

/// Context value for template variables
#[derive(Clone, Debug)]
pub enum ContextValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<ContextValue>),
    Object(HashMap<String, ContextValue>),
}

impl Default for CodeGenerator {
    fn default() -> Self {
        let mut generator = Self {
            templates: HashMap::new(),
            settings: GeneratorSettings::default(),
            formatters: HashMap::new(),
            optimizations: Vec::new(),
        };
        
        generator.initialize_default_templates();
        generator.initialize_default_formatters();
        generator.initialize_default_optimizations();
        generator
    }
}

impl Default for GeneratorSettings {
    fn default() -> Self {
        Self {
            target_language: "typescript".to_string(),
            style: CodeStyle::default(),
            naming: NamingConventions::default(),
            imports: ImportSettings::default(),
            documentation: DocumentationSettings::default(),
        }
    }
}

impl Default for CodeStyle {
    fn default() -> Self {
        Self {
            indentation: IndentationType::Spaces,
            indent_size: 2,
            max_line_length: 100,
            quote_style: QuoteStyle::Single,
            semicolons: SemicolonUsage::Always,
            trailing_commas: TrailingCommaUsage::MultilineOnly,
            brace_style: BraceStyle::SameLine,
        }
    }
}

impl Default for NamingConventions {
    fn default() -> Self {
        Self {
            components: NamingCase::PascalCase,
            functions: NamingCase::CamelCase,
            variables: NamingCase::CamelCase,
            constants: NamingCase::ScreamingSnakeCase,
            files: NamingCase::KebabCase,
        }
    }
}

impl Default for ImportSettings {
    fn default() -> Self {
        Self {
            style: ImportStyle::Named,
            organization: ImportOrganization::GroupedByType,
            auto_import: true,
            remove_unused: true,
        }
    }
}

impl Default for DocumentationSettings {
    fn default() -> Self {
        Self {
            generate_comments: true,
            document_parameters: true,
            document_returns: true,
            include_examples: false,
            style: DocumentationStyle::TSDoc,
        }
    }
}

impl CodeGenerator {
    /// Create a new code generator
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Initialize default templates
    fn initialize_default_templates(&mut self) {
        // React component template
        self.add_template(CodeTemplate {
            id: "react-component".to_string(),
            name: "React Component".to_string(),
            description: "Basic React functional component".to_string(),
            content: r#"import React from 'react';
{{#if use_typescript}}
interface {{component_name}}Props {
  {{#each props}}
  {{name}}{{#if optional}}?{{/if}}: {{type}};
  {{/each}}
}
{{/if}}

const {{component_name}}{{#if use_typescript}}: React.FC<{{component_name}}Props>{{/if}} = ({{#if use_typescript}}props{{else}}{...props}{{/if}}) => {
  return (
    <div className="{{class_name}}">
      {{content}}
    </div>
  );
};

export default {{component_name}};"#.to_string(),
            variables: HashMap::from([
                ("component_name".to_string(), VariableType::String),
                ("use_typescript".to_string(), VariableType::Boolean),
                ("props".to_string(), VariableType::Array(Box::new(VariableType::Object(HashMap::new())))),
                ("class_name".to_string(), VariableType::String),
                ("content".to_string(), VariableType::String),
            ]),
            metadata: TemplateMetadata {
                author: "System".to_string(),
                version: "1.0.0".to_string(),
                tags: vec!["react".to_string(), "component".to_string()],
                created_at: chrono::Utc::now(),
                modified_at: chrono::Utc::now(),
            },
        });
        
        // Vue component template
        self.add_template(CodeTemplate {
            id: "vue-component".to_string(),
            name: "Vue Component".to_string(),
            description: "Vue 3 composition API component".to_string(),
            content: r#"<template>
  <div class="{{class_name}}">
    {{content}}
  </div>
</template>

<script setup{{#if use_typescript}} lang="ts"{{/if}}>
{{#if use_typescript}}
interface Props {
  {{#each props}}
  {{name}}{{#if optional}}?{{/if}}: {{type}};
  {{/each}}
}

const props = defineProps<Props>();
{{else}}
const props = defineProps([
  {{#each props}}
  '{{name}}',
  {{/each}}
]);
{{/if}}

// Component logic here
</script>

<style scoped>
.{{class_name}} {
  /* Component styles */
}
</style>"#.to_string(),
            variables: HashMap::from([
                ("component_name".to_string(), VariableType::String),
                ("use_typescript".to_string(), VariableType::Boolean),
                ("props".to_string(), VariableType::Array(Box::new(VariableType::Object(HashMap::new())))),
                ("class_name".to_string(), VariableType::String),
                ("content".to_string(), VariableType::String),
            ]),
            metadata: TemplateMetadata {
                author: "System".to_string(),
                version: "1.0.0".to_string(),
                tags: vec!["vue".to_string(), "component".to_string()],
                created_at: chrono::Utc::now(),
                modified_at: chrono::Utc::now(),
            },
        });
    }
    
    /// Initialize default formatters
    fn initialize_default_formatters(&mut self) {
        // TypeScript/JavaScript formatter
        self.formatters.insert("typescript".to_string(), CodeFormatter {
            name: "TypeScript Formatter".to_string(),
            target: "typescript".to_string(),
            rules: FormattingRules {
                prettier_config: Some(PrettierConfig {
                    print_width: 100,
                    tab_width: 2,
                    use_tabs: false,
                    semi: true,
                    single_quote: true,
                    quote_props: "as-needed".to_string(),
                    trailing_comma: "es5".to_string(),
                    bracket_spacing: true,
                }),
                custom_rules: HashMap::new(),
            },
        });
    }
    
    /// Initialize default optimizations
    fn initialize_default_optimizations(&mut self) {
        // Remove unused imports
        self.optimizations.push(OptimizationRule {
            name: "Remove unused imports".to_string(),
            description: "Remove import statements that are not used".to_string(),
            pattern: r"import.*from.*".to_string(),
            replacement: "".to_string(),
            priority: 100,
        });
    }
    
    /// Add a code template
    pub fn add_template(&mut self, template: CodeTemplate) {
        self.templates.insert(template.id.clone(), template);
    }
    
    /// Generate code from template
    pub fn generate_code(
        &self,
        template_id: &str,
        context: &GenerationContext,
    ) -> Result<GeneratedCode, GenerationError> {
        let template = self.templates.get(template_id)
            .ok_or_else(|| GenerationError::TemplateNotFound(template_id.to_string()))?;
        
        let content = self.render_template(template, context)?;
        let formatted_content = self.format_code(&content, &self.settings.target_language)?;
        let optimized_content = self.optimize_code(&formatted_content)?;
        
        Ok(GeneratedCode {
            content: optimized_content,
            language: self.settings.target_language.clone(),
            filename: self.generate_filename(&context.component.name),
            imports: self.extract_imports(&content),
            exports: self.extract_exports(&content),
        })
    }
    
    /// Render template with context
    fn render_template(
        &self,
        template: &CodeTemplate,
        context: &GenerationContext,
    ) -> Result<String, GenerationError> {
        // Simple template rendering (in a real implementation, use a proper template engine like Handlebars)
        let mut content = template.content.clone();
        
        // Replace basic variables
        content = content.replace("{{component_name}}", &context.component.name);
        content = content.replace("{{class_name}}", &self.to_css_class_name(&context.component.name));
        
        // Replace framework-specific variables
        if let Some(ContextValue::Boolean(use_ts)) = context.variables.get("use_typescript") {
            content = content.replace("{{#if use_typescript}}", if *use_ts { "" } else { "{{!-- " });
            content = content.replace("{{/if}}", if *use_ts { "" } else { " --}}" });
        }
        
        Ok(content)
    }
    
    /// Format code using the specified formatter
    fn format_code(&self, code: &str, language: &str) -> Result<String, GenerationError> {
        if let Some(_formatter) = self.formatters.get(language) {
            // Apply formatting rules (simplified implementation)
            let mut formatted = code.to_string();
            
            // Apply indentation
            formatted = self.apply_indentation(&formatted);
            
            // Apply line length constraints
            formatted = self.apply_line_length_limits(&formatted);
            
            Ok(formatted)
        } else {
            Ok(code.to_string())
        }
    }
    
    /// Optimize generated code
    fn optimize_code(&self, code: &str) -> Result<String, GenerationError> {
        let mut optimized = code.to_string();
        
        // Apply optimization rules
        for rule in &self.optimizations {
            // Simple regex-based optimization (in a real implementation, use AST-based optimization)
            if optimized.contains(&rule.pattern) {
                optimized = optimized.replace(&rule.pattern, &rule.replacement);
            }
        }
        
        Ok(optimized)
    }
    
    /// Generate filename based on component name and settings
    fn generate_filename(&self, component_name: &str) -> String {
        let formatted_name = match self.settings.naming.files {
            NamingCase::KebabCase => self.to_kebab_case(component_name),
            NamingCase::CamelCase => self.to_camel_case(component_name),
            NamingCase::PascalCase => self.to_pascal_case(component_name),
            NamingCase::SnakeCase => self.to_snake_case(component_name),
            NamingCase::ScreamingSnakeCase => self.to_screaming_snake_case(component_name),
        };
        
        let extension = match self.settings.target_language.as_str() {
            "typescript" => "tsx",
            "javascript" => "jsx",
            "vue" => "vue",
            "angular" => "ts",
            _ => "js",
        };
        
        format!("{}.{}", formatted_name, extension)
    }
    
    /// Convert string to CSS class name
    fn to_css_class_name(&self, s: &str) -> String {
        self.to_kebab_case(s).to_lowercase()
    }
    
    /// Convert to kebab-case
    fn to_kebab_case(&self, s: &str) -> String {
        s.chars()
            .enumerate()
            .map(|(i, c)| {
                if c.is_uppercase() && i > 0 {
                    format!("-{}", c.to_lowercase())
                } else {
                    c.to_lowercase().to_string()
                }
            })
            .collect::<String>()
    }
    
    /// Convert to camelCase
    fn to_camel_case(&self, s: &str) -> String {
        let pascal = self.to_pascal_case(s);
        pascal.chars()
            .enumerate()
            .map(|(i, c)| {
                if i == 0 {
                    c.to_lowercase().to_string()
                } else {
                    c.to_string()
                }
            })
            .collect()
    }
    
    /// Convert to PascalCase
    fn to_pascal_case(&self, s: &str) -> String {
        s.split(|c: char| c == '-' || c == '_' || c.is_whitespace())
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                }
            })
            .collect()
    }
    
    /// Convert to snake_case
    fn to_snake_case(&self, s: &str) -> String {
        s.chars()
            .enumerate()
            .map(|(i, c)| {
                if c.is_uppercase() && i > 0 {
                    format!("_{}", c.to_lowercase())
                } else {
                    c.to_lowercase().to_string()
                }
            })
            .collect::<String>()
    }
    
    /// Convert to SCREAMING_SNAKE_CASE
    fn to_screaming_snake_case(&self, s: &str) -> String {
        self.to_snake_case(s).to_uppercase()
    }
    
    /// Apply indentation to code
    fn apply_indentation(&self, code: &str) -> String {
        // Simple indentation application (in a real implementation, use a proper code formatter)
        code.lines()
            .map(|line| {
                if line.trim().is_empty() {
                    line.to_string()
                } else {
                    let indent = match self.settings.style.indentation {
                        IndentationType::Spaces => " ".repeat(self.settings.style.indent_size as usize),
                        IndentationType::Tabs => "\t".to_string(),
                    };
                    format!("{}{}", indent, line.trim())
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    /// Apply line length limits
    fn apply_line_length_limits(&self, code: &str) -> String {
        // Simple line length application (in a real implementation, use a proper code formatter)
        code.lines()
            .map(|line| {
                if line.len() > self.settings.style.max_line_length as usize {
                    // Simple line breaking (in a real implementation, use smart line breaking)
                    line.to_string()
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    /// Extract import statements from code
    fn extract_imports(&self, code: &str) -> Vec<String> {
        code.lines()
            .filter(|line| line.trim().starts_with("import"))
            .map(|line| line.to_string())
            .collect()
    }
    
    /// Extract export statements from code
    fn extract_exports(&self, code: &str) -> Vec<String> {
        code.lines()
            .filter(|line| line.trim().starts_with("export"))
            .map(|line| line.to_string())
            .collect()
    }
    
    /// Render code generator UI
    pub fn render_ui(&mut self, ui: &mut Ui) {
        ui.heading("Code Generator");
        
        ui.horizontal(|ui| {
            ui.label("Target Language:");
            egui::ComboBox::from_label("")
                .selected_text(&self.settings.target_language)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.settings.target_language, "typescript".to_string(), "TypeScript");
                    ui.selectable_value(&mut self.settings.target_language, "javascript".to_string(), "JavaScript");
                    ui.selectable_value(&mut self.settings.target_language, "vue".to_string(), "Vue");
                    ui.selectable_value(&mut self.settings.target_language, "angular".to_string(), "Angular");
                });
        });
        
        ui.separator();
        
        ui.collapsing("Code Style", |ui| {
            ui.horizontal(|ui| {
                ui.label("Indentation:");
                ui.radio_value(&mut self.settings.style.indentation, IndentationType::Spaces, "Spaces");
                ui.radio_value(&mut self.settings.style.indentation, IndentationType::Tabs, "Tabs");
            });
            
            ui.horizontal(|ui| {
                ui.label("Indent Size:");
                ui.add(egui::Slider::new(&mut self.settings.style.indent_size, 1..=8));
            });
            
            ui.horizontal(|ui| {
                ui.label("Max Line Length:");
                ui.add(egui::Slider::new(&mut self.settings.style.max_line_length, 80..=200));
            });
            
            ui.horizontal(|ui| {
                ui.label("Quote Style:");
                ui.radio_value(&mut self.settings.style.quote_style, QuoteStyle::Single, "Single");
                ui.radio_value(&mut self.settings.style.quote_style, QuoteStyle::Double, "Double");
            });
        });
        
        ui.separator();
        
        ui.collapsing("Documentation", |ui| {
            ui.checkbox(&mut self.settings.documentation.generate_comments, "Generate Comments");
            ui.checkbox(&mut self.settings.documentation.document_parameters, "Document Parameters");
            ui.checkbox(&mut self.settings.documentation.document_returns, "Document Returns");
            ui.checkbox(&mut self.settings.documentation.include_examples, "Include Examples");
        });
        
        ui.separator();
        
        if ui.button("Generate Code").clicked() {
            // Handle code generation
        }
    }
}

/// Code generation error types
#[derive(Debug, thiserror::Error)]
pub enum GenerationError {
    #[error("Template not found: {0}")]
    TemplateNotFound(String),
    #[error("Template rendering failed: {0}")]
    RenderingFailed(String),
    #[error("Code formatting failed: {0}")]
    FormattingFailed(String),
    #[error("Code optimization failed: {0}")]
    OptimizationFailed(String),
}