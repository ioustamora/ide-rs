//! Enhanced Code Generation Integration
//!
//! This module integrates the enhanced marker system with the existing code generator
//! to provide a complete solution for marker-based code generation and rewriting.

use super::code_generator::{CodeGenerator, CodeTemplate, GuardedSection, CodeGenResult, CodeGenError};
use super::codegen_markers::{
    CodeMarker, MarkerType, GenerationStrategy, CodeRewriter, CodeLanguage,
    TemplateParameter, ParameterType, ConditionalStrategy, ImportMergeStrategy
};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use serde_json::Value;
use uuid::Uuid;

/// Enhanced code generator with marker support
pub struct EnhancedCodeGenerator {
    /// Base code generator
    base_generator: CodeGenerator,
    /// Language-specific rewriters
    rewriters: HashMap<PathBuf, CodeRewriter>,
    /// Template registry with enhanced markers
    enhanced_templates: HashMap<String, EnhancedTemplate>,
    /// Global generation context
    generation_context: GenerationContext,
}

/// Enhanced template with marker support
#[derive(Debug, Clone)]
pub struct EnhancedTemplate {
    /// Base template
    pub base_template: CodeTemplate,
    /// Marker definitions
    pub markers: Vec<MarkerDefinition>,
    /// Supported languages
    pub supported_languages: Vec<CodeLanguage>,
    /// Template dependencies
    pub dependencies: Vec<String>,
}

/// Marker definition within a template
#[derive(Debug, Clone)]
pub struct MarkerDefinition {
    /// Marker identifier
    pub id: String,
    /// Marker type and configuration
    pub marker_type: MarkerType,
    /// Position within template (line number)
    pub template_line: usize,
    /// Content generation rules
    pub content_rules: ContentRules,
}

/// Rules for generating content within markers
#[derive(Debug, Clone)]
pub struct ContentRules {
    /// Content generation function or template
    pub generator: ContentGenerator,
    /// Conditions for content generation
    pub conditions: Vec<GenerationCondition>,
    /// Dependencies that trigger regeneration
    pub dependencies: Vec<String>,
}

/// Content generation strategies
#[derive(Debug, Clone)]
pub enum ContentGenerator {
    /// Static content
    Static(String),
    /// Template with variables
    Template { template: String, variables: HashMap<String, ParameterType> },
    /// Function-based generation
    Function(String),
    /// Component-based generation (uses derive macro metadata)
    Component { component_type: String, properties: HashMap<String, Value> },
}

/// Conditions for content generation
#[derive(Debug, Clone)]
pub struct GenerationCondition {
    /// Condition expression
    pub expression: String,
    /// Variables referenced in expression
    pub variables: Vec<String>,
    /// Condition result affects generation
    pub affects_generation: bool,
}

/// Global context for code generation
#[derive(Debug, Clone)]
pub struct GenerationContext {
    /// Global variables available to all templates
    pub global_variables: HashMap<String, Value>,
    /// Dependency graph for change tracking
    pub dependencies: DependencyGraph,
    /// Generation settings
    pub settings: GenerationSettings,
}

/// Dependency tracking for intelligent regeneration
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    /// Dependencies between files/components
    pub file_dependencies: HashMap<PathBuf, HashSet<PathBuf>>,
    /// Component dependencies
    pub component_dependencies: HashMap<String, HashSet<String>>,
    /// Template dependencies
    pub template_dependencies: HashMap<String, HashSet<String>>,
}

/// Settings for code generation
#[derive(Debug, Clone)]
pub struct GenerationSettings {
    /// Enable incremental generation
    pub incremental: bool,
    /// Preserve user modifications
    pub preserve_user_code: bool,
    /// Auto-format generated code
    pub auto_format: bool,
    /// Generate documentation
    pub generate_docs: bool,
    /// Backup files before overwriting
    pub backup_files: bool,
}

impl Default for GenerationSettings {
    fn default() -> Self {
        Self {
            incremental: true,
            preserve_user_code: true,
            auto_format: true,
            generate_docs: false,
            backup_files: true,
        }
    }
}

impl EnhancedCodeGenerator {
    /// Create a new enhanced code generator
    pub fn new(output_dir: PathBuf) -> Self {
        Self {
            base_generator: CodeGenerator::new(output_dir),
            rewriters: HashMap::new(),
            enhanced_templates: HashMap::new(),
            generation_context: GenerationContext {
                global_variables: HashMap::new(),
                dependencies: DependencyGraph {
                    file_dependencies: HashMap::new(),
                    component_dependencies: HashMap::new(),
                    template_dependencies: HashMap::new(),
                },
                settings: GenerationSettings::default(),
            },
        }
    }
    
    /// Register an enhanced template
    pub fn register_enhanced_template(&mut self, template: EnhancedTemplate) {
        // Register base template
        self.base_generator.register_template(template.base_template.clone());
        
        // Store enhanced template
        self.enhanced_templates.insert(template.base_template.template_id.clone(), template);
    }
    
    /// Generate code with enhanced marker support
    pub fn generate_enhanced_code(
        &mut self,
        template_id: &str,
        variables: HashMap<String, Value>,
        output_file: PathBuf,
    ) -> CodeGenResult<String> {
        // Get enhanced template
        let template = self.enhanced_templates.get(template_id)
            .ok_or_else(|| CodeGenError::TemplateNotFound(template_id.to_string()))?;
        
        // Detect language from file extension
        let language = output_file.extension()
            .and_then(|ext| ext.to_str())
            .and_then(CodeLanguage::from_extension)
            .unwrap_or(CodeLanguage::Rust);
        
        // Check if template supports this language
        if !template.supported_languages.contains(&language) {
            return Err(CodeGenError::TemplateError(
                format!("Template {} does not support language {:?}", template_id, language)
            ));
        }
        
        // Load existing file and parse markers
        let existing_code = std::fs::read_to_string(&output_file).unwrap_or_default();
        let mut rewriter = CodeRewriter::new(language, existing_code);
        rewriter.parse_markers().map_err(|e| CodeGenError::InvalidGuardSyntax(e))?;
        
        // Generate content for each marker
        let mut updated_markers = Vec::new();
        for marker_def in &template.markers {
            let mut marker = self.create_marker_from_definition(marker_def, &rewriter)?;
            
            // Generate content based on marker type and rules
            let content = self.generate_marker_content(marker_def, &variables)?;
            marker.update_content(content);
            
            updated_markers.push(marker);
        }
        
        // Preserve existing user-modified markers
        for existing_marker in &rewriter.markers {
            if existing_marker.is_modified {
                // Find corresponding marker in updated list
                if let Some(updated_marker) = updated_markers.iter_mut()
                    .find(|m| self.markers_match(&m.marker_type, &existing_marker.marker_type)) {
                    // Preserve user modifications for guard markers
                    if matches!(existing_marker.marker_type, MarkerType::Guard { .. }) {
                        updated_marker.content = existing_marker.content.clone();
                        updated_marker.is_modified = true;
                    }
                }
            }
        }
        
        // Generate final code with markers
        let mut new_rewriter = CodeRewriter::new(rewriter.language.clone(), String::new());
        let final_code = new_rewriter.rewrite_with_markers(updated_markers)
            .map_err(|e| CodeGenError::TemplateError(e))?;
        
        // Store rewriter for future use
        self.rewriters.insert(output_file.clone(), new_rewriter);
        
        // Apply post-processing
        let processed_code = self.post_process_code(&final_code, &language)?;
        
        Ok(processed_code)
    }
    
    /// Create a marker from a marker definition
    fn create_marker_from_definition(
        &self,
        def: &MarkerDefinition,
        rewriter: &CodeRewriter,
    ) -> CodeGenResult<CodeMarker> {
        // Find position in template (this is simplified - real implementation would be more sophisticated)
        let start_pos = super::codegen_markers::CodePosition {
            line: def.template_line,
            column: 0,
            offset: 0,
        };
        let end_pos = start_pos.clone();
        
        Ok(CodeMarker::new(def.marker_type.clone(), start_pos, end_pos))
    }
    
    /// Generate content for a marker based on its definition
    fn generate_marker_content(
        &self,
        marker_def: &MarkerDefinition,
        variables: &HashMap<String, Value>,
    ) -> CodeGenResult<String> {
        match &marker_def.content_rules.generator {
            ContentGenerator::Static(content) => Ok(content.clone()),
            
            ContentGenerator::Template { template, variables: template_vars } => {
                let mut content = template.clone();
                
                // Replace template variables
                for (var_name, _var_type) in template_vars {
                    if let Some(value) = variables.get(var_name) {
                        let placeholder = format!("{{{{{}}}}}", var_name);
                        let string_value = self.serialize_json_value(value);
                        content = content.replace(&placeholder, &string_value);
                    }
                }
                
                Ok(content)
            }
            
            ContentGenerator::Function(func_name) => {
                // This would call a registered generation function
                // For now, return placeholder
                Ok(format!("// Generated by function: {}", func_name))
            }
            
            ContentGenerator::Component { component_type, properties } => {
                // Use component metadata from derive macro system
                Ok(format!(
                    "// Component: {}\n// Properties: {:?}",
                    component_type, properties
                ))
            }
        }
    }
    
    /// Check if two marker types match for preservation purposes
    fn markers_match(&self, type1: &MarkerType, type2: &MarkerType) -> bool {
        match (type1, type2) {
            (MarkerType::Guard { id: id1, .. }, MarkerType::Guard { id: id2, .. }) => id1 == id2,
            (MarkerType::Generated { id: id1, .. }, MarkerType::Generated { id: id2, .. }) => id1 == id2,
            (MarkerType::Template { id: id1, .. }, MarkerType::Template { id: id2, .. }) => id1 == id2,
            _ => false,
        }
    }
    
    /// Serialize JSON value to string
    fn serialize_json_value(&self, value: &Value) -> String {
        match value {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Array(arr) => format!("[{}]", arr.iter()
                .map(|v| self.serialize_json_value(v))
                .collect::<Vec<_>>()
                .join(", ")),
            Value::Object(obj) => format!("{{{}}}", obj.iter()
                .map(|(k, v)| format!("{}: {}", k, self.serialize_json_value(v)))
                .collect::<Vec<_>>()
                .join(", ")),
            Value::Null => "null".to_string(),
        }
    }
    
    /// Post-process generated code (formatting, etc.)
    fn post_process_code(&self, code: &str, language: &CodeLanguage) -> CodeGenResult<String> {
        let mut processed = code.to_string();
        
        if self.generation_context.settings.auto_format {
            processed = self.format_code(&processed, language)?;
        }
        
        Ok(processed)
    }
    
    /// Format code for the given language (basic implementation)
    fn format_code(&self, code: &str, language: &CodeLanguage) -> CodeGenResult<String> {
        // This is a simplified formatter - real implementation would use language-specific formatters
        let lines: Vec<&str> = code.lines().collect();
        let mut formatted_lines = Vec::new();
        let mut indent_level = 0;
        
        for line in lines {
            let trimmed = line.trim();
            
            // Adjust indent for closing braces
            if trimmed.starts_with('}') || trimmed.starts_with(']') || trimmed.starts_with(')') {
                indent_level = indent_level.saturating_sub(1);
            }
            
            // Apply indentation
            let formatted_line = if trimmed.is_empty() {
                String::new()
            } else {
                format!("{}{}", "    ".repeat(indent_level), trimmed)
            };
            
            formatted_lines.push(formatted_line);
            
            // Adjust indent for opening braces
            if trimmed.ends_with('{') || trimmed.ends_with('[') || trimmed.ends_with('(') {
                indent_level += 1;
            }
        }
        
        Ok(formatted_lines.join("\n"))
    }
    
    /// Update dependencies and trigger regeneration as needed
    pub fn update_dependencies(&mut self, changed_files: HashSet<PathBuf>) -> CodeGenResult<Vec<PathBuf>> {
        let mut files_to_regenerate = Vec::new();
        
        // Check file dependencies
        for (file, deps) in &self.generation_context.dependencies.file_dependencies {
            if deps.iter().any(|dep| changed_files.contains(dep)) {
                files_to_regenerate.push(file.clone());
            }
        }
        
        // Check if any markers need regeneration
        for (file_path, rewriter) in &self.rewriters {
            for marker in &rewriter.markers {
                let changed_deps: HashSet<String> = changed_files.iter()
                    .map(|p| p.to_string_lossy().to_string())
                    .collect();
                    
                if marker.should_regenerate(&changed_deps) {
                    files_to_regenerate.push(file_path.clone());
                    break;
                }
            }
        }
        
        files_to_regenerate.dedup();
        Ok(files_to_regenerate)
    }
    
    /// Get statistics about code generation
    pub fn get_statistics(&self) -> GenerationStatistics {
        GenerationStatistics {
            total_templates: self.enhanced_templates.len(),
            total_files: self.rewriters.len(),
            total_markers: self.rewriters.values()
                .map(|r| r.markers.len())
                .sum(),
            modified_markers: self.rewriters.values()
                .flat_map(|r| &r.markers)
                .filter(|m| m.is_modified)
                .count(),
        }
    }
}

/// Statistics about code generation
#[derive(Debug, Clone)]
pub struct GenerationStatistics {
    /// Number of registered templates
    pub total_templates: usize,
    /// Number of files being managed
    pub total_files: usize,
    /// Total number of markers across all files
    pub total_markers: usize,
    /// Number of markers modified by users
    pub modified_markers: usize,
}

/// Builder for creating enhanced templates
pub struct EnhancedTemplateBuilder {
    template_id: String,
    content: String,
    markers: Vec<MarkerDefinition>,
    supported_languages: Vec<CodeLanguage>,
}

impl EnhancedTemplateBuilder {
    /// Create a new template builder
    pub fn new(template_id: String, content: String) -> Self {
        Self {
            template_id,
            content,
            markers: Vec::new(),
            supported_languages: vec![CodeLanguage::Rust], // Default to Rust
        }
    }
    
    /// Add supported languages
    pub fn support_languages(mut self, languages: Vec<CodeLanguage>) -> Self {
        self.supported_languages = languages;
        self
    }
    
    /// Add a guard marker
    pub fn add_guard(mut self, id: String, template_line: usize, default_content: Option<String>) -> Self {
        self.markers.push(MarkerDefinition {
            id: id.clone(),
            marker_type: MarkerType::Guard {
                id,
                preserve_indent: true,
                default_content,
            },
            template_line,
            content_rules: ContentRules {
                generator: ContentGenerator::Static(default_content.unwrap_or_default()),
                conditions: Vec::new(),
                dependencies: Vec::new(),
            },
        });
        self
    }
    
    /// Add a generated marker
    pub fn add_generated(mut self, id: String, template_line: usize, generator: ContentGenerator) -> Self {
        self.markers.push(MarkerDefinition {
            id: id.clone(),
            marker_type: MarkerType::Generated {
                id,
                strategy: GenerationStrategy::Replace,
                dependencies: Vec::new(),
            },
            template_line,
            content_rules: ContentRules {
                generator,
                conditions: Vec::new(),
                dependencies: Vec::new(),
            },
        });
        self
    }
    
    /// Build the enhanced template
    pub fn build(self) -> EnhancedTemplate {
        let base_template = CodeTemplate::new(self.template_id, self.content);
        
        EnhancedTemplate {
            base_template,
            markers: self.markers,
            supported_languages: self.supported_languages,
            dependencies: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_enhanced_template_builder() {
        let template = EnhancedTemplateBuilder::new(
            "test_template".to_string(),
            "fn main() {\n    {{guard:user_code}}\n}".to_string()
        )
        .support_languages(vec![CodeLanguage::Rust])
        .add_guard("user_code".to_string(), 1, Some("// User code here".to_string()))
        .build();
        
        assert_eq!(template.base_template.template_id, "test_template");
        assert_eq!(template.markers.len(), 1);
        assert_eq!(template.supported_languages, vec![CodeLanguage::Rust]);
    }
    
    #[test]
    fn test_enhanced_code_generator_creation() {
        let temp_dir = tempdir().unwrap();
        let generator = EnhancedCodeGenerator::new(temp_dir.path().to_path_buf());
        
        assert_eq!(generator.enhanced_templates.len(), 0);
        assert_eq!(generator.rewriters.len(), 0);
        assert!(generator.generation_context.settings.preserve_user_code);
    }
    
    #[test]
    fn test_generation_statistics() {
        let temp_dir = tempdir().unwrap();
        let generator = EnhancedCodeGenerator::new(temp_dir.path().to_path_buf());
        let stats = generator.get_statistics();
        
        assert_eq!(stats.total_templates, 0);
        assert_eq!(stats.total_files, 0);
        assert_eq!(stats.total_markers, 0);
        assert_eq!(stats.modified_markers, 0);
    }
}