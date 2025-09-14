//! # Enhanced Code Generation System with IR Integration
//!
//! This module provides an advanced code generation system that leverages the
//! Intermediate Representation (IR) for sophisticated code transformations,
//! multi-target output, and enhanced guarded sections with fine-grained control.

use std::collections::{HashMap, BTreeMap};
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::code_ir::{
    IrNode, IrNodeType, IrValue, GuardType, ValidationError,
    Visibility, TypeReference, Parameter, ScopeType, CommentType
};
use crate::core::event_bus::{IdeEvent, global_event_bus};

/// Enhanced code generation errors
#[derive(Debug, Clone)]
pub enum EnhancedCodeGenError {
    /// IR validation failed
    InvalidIR(Vec<ValidationError>),
    /// Target language not supported
    UnsupportedTarget(String),
    /// Code generation pass failed
    PassFailed { pass_name: String, reason: String },
    /// Template processing error
    TemplateError { template: String, error: String },
    /// File I/O error
    IoError(String),
    /// Serialization error
    SerializationError(String),
    /// Guard section conflict
    GuardConflict { guard_id: String, reason: String },
    /// Dependency resolution error
    DependencyError(String),
}

impl std::fmt::Display for EnhancedCodeGenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidIR(errors) => {
                write!(f, "Invalid IR: ")?;
                for (i, error) in errors.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", error)?;
                }
                Ok(())
            }
            Self::UnsupportedTarget(target) => write!(f, "Unsupported target: {}", target),
            Self::PassFailed { pass_name, reason } => 
                write!(f, "Pass '{}' failed: {}", pass_name, reason),
            Self::TemplateError { template, error } => 
                write!(f, "Template '{}' error: {}", template, error),
            Self::IoError(msg) => write!(f, "I/O error: {}", msg),
            Self::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            Self::GuardConflict { guard_id, reason } => 
                write!(f, "Guard conflict '{}': {}", guard_id, reason),
            Self::DependencyError(msg) => write!(f, "Dependency error: {}", msg),
        }
    }
}

impl std::error::Error for EnhancedCodeGenError {}

pub type EnhancedResult<T> = Result<T, EnhancedCodeGenError>;

/// Target language for code generation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TargetLanguage {
    Rust,
    TypeScript,
    JavaScript,
    Python,
    Go,
    CSharp,
    Java,
    Cpp,
    Swift,
    Kotlin,
}

impl TargetLanguage {
    /// Get file extension for this language
    pub fn file_extension(&self) -> &'static str {
        match self {
            Self::Rust => "rs",
            Self::TypeScript => "ts",
            Self::JavaScript => "js",
            Self::Python => "py",
            Self::Go => "go",
            Self::CSharp => "cs",
            Self::Java => "java",
            Self::Cpp => "cpp",
            Self::Swift => "swift",
            Self::Kotlin => "kt",
        }
    }
    
    /// Get comment syntax for this language
    pub fn comment_syntax(&self) -> (&'static str, Option<&'static str>) {
        match self {
            Self::Rust | Self::JavaScript | Self::TypeScript | Self::Go | 
            Self::CSharp | Self::Java | Self::Cpp | Self::Swift | Self::Kotlin => ("//", Some(("/*", "*/"))),
            Self::Python => ("#", Some(("\"\"\"", "\"\"\""))),
        }
    }
}

/// Configuration for code generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGenConfig {
    /// Target language
    pub target_language: TargetLanguage,
    /// Output directory
    pub output_dir: PathBuf,
    /// Whether to generate debug information
    pub include_debug_info: bool,
    /// Code formatting options
    pub formatting: FormattingConfig,
    /// Guard section configuration
    pub guard_config: GuardConfig,
    /// Template search paths
    pub template_paths: Vec<PathBuf>,
    /// Custom code generation passes
    pub custom_passes: Vec<String>,
}

/// Code formatting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattingConfig {
    /// Indentation style
    pub indent_style: IndentStyle,
    /// Line width for formatting
    pub line_width: usize,
    /// Whether to add trailing commas
    pub trailing_commas: bool,
    /// Whether to format on generation
    pub format_on_generate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndentStyle {
    Spaces(usize),
    Tabs,
}

/// Guard section configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardConfig {
    /// Guard marker style
    pub marker_style: GuardMarkerStyle,
    /// Whether to preserve formatting in guards
    pub preserve_formatting: bool,
    /// Default guard content for new sections
    pub default_content: HashMap<GuardType, String>,
    /// Whether to auto-create missing guards
    pub auto_create_guards: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GuardMarkerStyle {
    Comment,   // Use comment-based markers
    Attribute, // Use language-specific attributes
    Pragma,    // Use pragma-style markers
}

/// Code generation pass trait
pub trait CodeGenPass: Send + Sync {
    /// Name of this pass
    fn name(&self) -> &str;
    
    /// Execute the pass on the IR tree
    fn execute(&self, ir: &mut IrNode, config: &CodeGenConfig) -> EnhancedResult<()>;
    
    /// Dependencies (other passes that must run first)
    fn dependencies(&self) -> Vec<&str> { Vec::new() }
    
    /// Whether this pass modifies the IR structure
    fn is_structural(&self) -> bool { false }
    
    /// Pass priority (higher runs first)
    fn priority(&self) -> i32 { 0 }
}

/// Enhanced code generator with IR integration
pub struct EnhancedCodeGenerator {
    /// Configuration
    config: CodeGenConfig,
    /// Registered code generation passes
    passes: HashMap<String, Box<dyn CodeGenPass>>,
    /// Pass execution order (resolved from dependencies)
    pass_order: Vec<String>,
    /// Target-specific code emitters
    emitters: HashMap<TargetLanguage, Box<dyn CodeEmitter>>,
    /// Template cache
    templates: HashMap<String, IrTemplate>,
    /// Guard section cache
    guard_cache: HashMap<String, GuardSection>,
    /// Generation statistics
    stats: GenerationStats,
}

/// Target-specific code emitter
pub trait CodeEmitter: Send + Sync {
    /// Target language this emitter handles
    fn target_language(&self) -> TargetLanguage;
    
    /// Emit code for an IR node
    fn emit_node(&self, node: &IrNode, context: &EmitContext) -> EnhancedResult<String>;
    
    /// Emit a complete file from IR tree
    fn emit_file(&self, ir: &IrNode, context: &EmitContext) -> EnhancedResult<String> {
        let mut output = String::new();
        output.push_str(&self.emit_node(ir, context)?);
        Ok(output)
    }
    
    /// Format generated code
    fn format_code(&self, code: &str, config: &FormattingConfig) -> EnhancedResult<String> {
        // Default implementation: return as-is
        Ok(code.to_string())
    }
}

/// Context for code emission
pub struct EmitContext {
    /// Current indentation level
    pub indent_level: usize,
    /// Current scope information
    pub scope_stack: Vec<String>,
    /// Available variables in scope
    pub variables: HashMap<String, TypeReference>,
    /// Guard sections to preserve
    pub guards: HashMap<String, GuardSection>,
    /// Template parameters
    pub template_params: HashMap<String, IrValue>,
    /// Generation metadata
    pub metadata: HashMap<String, String>,
}

/// IR-based template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrTemplate {
    /// Template identifier
    pub id: String,
    /// Template IR tree
    pub ir_tree: IrNode,
    /// Template parameters with types
    pub parameters: HashMap<String, IrValue>,
    /// Target languages this template supports
    pub target_languages: Vec<TargetLanguage>,
    /// Template metadata
    pub metadata: HashMap<String, String>,
}

/// Enhanced guard section with fine-grained control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardSection {
    /// Guard identifier
    pub guard_id: String,
    /// Guard type
    pub guard_type: GuardType,
    /// Current user content
    pub user_content: String,
    /// Default content for new sections
    pub default_content: Option<String>,
    /// Whether content has been modified by user
    pub is_modified: bool,
    /// Last modification timestamp
    pub last_modified: u64,
    /// Content hash for change detection
    pub content_hash: String,
    /// Guard-specific metadata
    pub metadata: HashMap<String, String>,
    /// Validation rules for user content
    pub validation_rules: Vec<ValidationRule>,
}

/// Validation rule for guard content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    /// Rule type
    pub rule_type: ValidationRuleType,
    /// Rule parameters
    pub parameters: HashMap<String, IrValue>,
    /// Error message if validation fails
    pub error_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationRuleType {
    /// Content must match regex pattern
    Regex,
    /// Content must not exceed length
    MaxLength,
    /// Content must contain required elements
    RequiredElements,
    /// Content must be valid syntax for language
    SyntaxCheck,
    /// Custom validation function
    Custom(String),
}

/// Code generation statistics
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GenerationStats {
    /// Total files generated
    pub files_generated: usize,
    /// Total lines of code generated
    pub lines_generated: usize,
    /// Number of guard sections processed
    pub guards_processed: usize,
    /// Number of user-modified guards preserved
    pub guards_preserved: usize,
    /// Total generation time in milliseconds
    pub generation_time_ms: u64,
    /// Number of passes executed
    pub passes_executed: usize,
    /// Errors encountered
    pub errors: Vec<String>,
    /// Warnings generated
    pub warnings: Vec<String>,
}

impl Default for CodeGenConfig {
    fn default() -> Self {
        Self {
            target_language: TargetLanguage::Rust,
            output_dir: PathBuf::from("generated"),
            include_debug_info: true,
            formatting: FormattingConfig {
                indent_style: IndentStyle::Spaces(4),
                line_width: 100,
                trailing_commas: true,
                format_on_generate: true,
            },
            guard_config: GuardConfig {
                marker_style: GuardMarkerStyle::Comment,
                preserve_formatting: true,
                default_content: HashMap::new(),
                auto_create_guards: true,
            },
            template_paths: vec![PathBuf::from("templates")],
            custom_passes: Vec::new(),
        }
    }
}

impl EnhancedCodeGenerator {
    /// Create a new enhanced code generator
    pub fn new(config: CodeGenConfig) -> Self {
        let mut generator = Self {
            config,
            passes: HashMap::new(),
            pass_order: Vec::new(),
            emitters: HashMap::new(),
            templates: HashMap::new(),
            guard_cache: HashMap::new(),
            stats: GenerationStats::default(),
        };
        
        // Register built-in passes
        generator.register_builtin_passes();
        
        // Register built-in emitters
        generator.register_builtin_emitters();
        
        generator
    }
    
    /// Register a code generation pass
    pub fn register_pass(&mut self, pass: Box<dyn CodeGenPass>) {
        let name = pass.name().to_string();
        self.passes.insert(name.clone(), pass);
        self.resolve_pass_order();
    }
    
    /// Register a code emitter for a target language
    pub fn register_emitter(&mut self, emitter: Box<dyn CodeEmitter>) {
        let language = emitter.target_language();
        self.emitters.insert(language, emitter);
    }
    
    /// Register an IR template
    pub fn register_template(&mut self, template: IrTemplate) {
        self.templates.insert(template.id.clone(), template);
    }
    
    /// Generate code from IR tree
    pub fn generate(&mut self, ir: &mut IrNode, output_path: PathBuf) -> EnhancedResult<GenerationStats> {
        let start_time = std::time::Instant::now();
        self.stats = GenerationStats::default();
        
        // Step 1: Validate IR
        let validation_errors = ir.validate();
        if !validation_errors.is_empty() {
            return Err(EnhancedCodeGenError::InvalidIR(validation_errors));
        }
        
        // Step 2: Load existing guard sections
        self.load_guard_cache(&output_path)?;
        
        // Step 3: Execute passes in dependency order
        for pass_name in &self.pass_order.clone() {
            if let Some(pass) = self.passes.get(pass_name) {
                pass.execute(ir, &self.config)?;
                self.stats.passes_executed += 1;
            }
        }
        
        // Step 4: Generate code using appropriate emitter
        let emitter = self.emitters.get(&self.config.target_language)
            .ok_or_else(|| EnhancedCodeGenError::UnsupportedTarget(
                format!("{:?}", self.config.target_language)
            ))?;
        
        let context = EmitContext {
            indent_level: 0,
            scope_stack: Vec::new(),
            variables: HashMap::new(),
            guards: self.guard_cache.clone(),
            template_params: HashMap::new(),
            metadata: HashMap::new(),
        };
        
        let generated_code = emitter.emit_file(ir, &context)?;
        
        // Step 5: Format code if requested
        let final_code = if self.config.formatting.format_on_generate {
            emitter.format_code(&generated_code, &self.config.formatting)?
        } else {
            generated_code
        };
        
        // Step 6: Write output file
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| EnhancedCodeGenError::IoError(e.to_string()))?;
        }
        
        fs::write(&output_path, &final_code)
            .map_err(|e| EnhancedCodeGenError::IoError(e.to_string()))?;
        
        // Step 7: Update statistics
        self.stats.files_generated = 1;
        self.stats.lines_generated = final_code.lines().count();
        self.stats.generation_time_ms = start_time.elapsed().as_millis() as u64;
        
        // Step 8: Emit IDE events
        global_event_bus().emit(IdeEvent::CodeGenerationCompleted {
            files_updated: vec![output_path],
            success: true,
        });
        
        Ok(self.stats.clone())
    }
    
    /// Generate multiple files from IR forest
    pub fn generate_multiple(&mut self, ir_files: Vec<(IrNode, PathBuf)>) -> EnhancedResult<GenerationStats> {
        let start_time = std::time::Instant::now();
        let mut combined_stats = GenerationStats::default();
        let mut successful_files = Vec::new();
        
        for (mut ir, output_path) in ir_files {
            match self.generate(&mut ir, output_path.clone()) {
                Ok(stats) => {
                    combined_stats.files_generated += stats.files_generated;
                    combined_stats.lines_generated += stats.lines_generated;
                    combined_stats.guards_processed += stats.guards_processed;
                    combined_stats.guards_preserved += stats.guards_preserved;
                    combined_stats.passes_executed += stats.passes_executed;
                    successful_files.push(output_path);
                }
                Err(e) => {
                    combined_stats.errors.push(format!("Error generating {}: {}", 
                        output_path.display(), e));
                }
            }
        }
        
        combined_stats.generation_time_ms = start_time.elapsed().as_millis() as u64;
        
        if !successful_files.is_empty() {
            global_event_bus().emit(IdeEvent::CodeGenerationCompleted {
                files_updated: successful_files,
                success: combined_stats.errors.is_empty(),
            });
        }
        
        if !combined_stats.errors.is_empty() {
            global_event_bus().emit(IdeEvent::CodeGenerationFailed {
                error: format!("{} files failed generation", combined_stats.errors.len()),
                target_files: combined_stats.errors.iter()
                    .map(|e| PathBuf::from(e.clone()))
                    .collect(),
            });
        }
        
        Ok(combined_stats)
    }
    
    /// Expand template with parameters
    pub fn expand_template(&self, template_id: &str, parameters: HashMap<String, IrValue>) -> EnhancedResult<IrNode> {
        let template = self.templates.get(template_id)
            .ok_or_else(|| EnhancedCodeGenError::TemplateError {
                template: template_id.to_string(),
                error: "Template not found".to_string(),
            })?;
        
        // Create a copy of the template IR and substitute parameters
        let mut expanded_ir = template.ir_tree.clone();
        self.substitute_template_parameters(&mut expanded_ir, &parameters)?;
        
        Ok(expanded_ir)
    }
    
    /// Get generation statistics
    pub fn get_stats(&self) -> &GenerationStats {
        &self.stats
    }
    
    /// Clear generation statistics
    pub fn clear_stats(&mut self) {
        self.stats = GenerationStats::default();
    }
    
    // Private helper methods
    
    fn register_builtin_passes(&mut self) {
        // Register core passes
        self.register_pass(Box::new(GuardProcessingPass));
        self.register_pass(Box::new(TemplateExpansionPass));
        self.register_pass(Box::new(ValidationPass));
        self.register_pass(Box::new(OptimizationPass));
    }
    
    fn register_builtin_emitters(&mut self) {
        // Register built-in emitters for supported languages
        self.register_emitter(Box::new(RustEmitter));
        self.register_emitter(Box::new(TypeScriptEmitter));
        // Additional emitters would be registered here
    }
    
    fn resolve_pass_order(&mut self) {
        // Topological sort of passes based on dependencies
        let mut order = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut visiting = std::collections::HashSet::new();
        
        for pass_name in self.passes.keys() {
            if !visited.contains(pass_name) {
                self.visit_pass(pass_name, &mut order, &mut visited, &mut visiting);
            }
        }
        
        // Sort by priority within dependency constraints
        order.sort_by(|a, b| {
            let priority_a = self.passes.get(a).map_or(0, |p| p.priority());
            let priority_b = self.passes.get(b).map_or(0, |p| p.priority());
            priority_b.cmp(&priority_a) // Higher priority first
        });
        
        self.pass_order = order;
    }
    
    fn visit_pass(&self, pass_name: &str, order: &mut Vec<String>, 
                  visited: &mut std::collections::HashSet<String>,
                  visiting: &mut std::collections::HashSet<String>) {
        if visiting.contains(pass_name) {
            // Circular dependency detected - skip
            return;
        }
        
        visiting.insert(pass_name.to_string());
        
        if let Some(pass) = self.passes.get(pass_name) {
            for dep in pass.dependencies() {
                if !visited.contains(dep) {
                    self.visit_pass(dep, order, visited, visiting);
                }
            }
        }
        
        visiting.remove(pass_name);
        visited.insert(pass_name.to_string());
        order.push(pass_name.to_string());
    }
    
    fn load_guard_cache(&mut self, _output_path: &Path) -> EnhancedResult<()> {
        // Load existing guard sections from generated files
        // This is a placeholder for the full implementation
        Ok(())
    }
    
    fn substitute_template_parameters(&self, _ir: &mut IrNode, _parameters: &HashMap<String, IrValue>) -> EnhancedResult<()> {
        // Substitute template parameters in IR tree
        // This is a placeholder for the full implementation
        Ok(())
    }
}

// Built-in code generation passes

struct GuardProcessingPass;
impl CodeGenPass for GuardProcessingPass {
    fn name(&self) -> &str { "guard_processing" }
    
    fn execute(&self, _ir: &mut IrNode, _config: &CodeGenConfig) -> EnhancedResult<()> {
        // Process guard sections - placeholder
        Ok(())
    }
    
    fn priority(&self) -> i32 { 100 }
}

struct TemplateExpansionPass;
impl CodeGenPass for TemplateExpansionPass {
    fn name(&self) -> &str { "template_expansion" }
    
    fn execute(&self, _ir: &mut IrNode, _config: &CodeGenConfig) -> EnhancedResult<()> {
        // Expand templates - placeholder
        Ok(())
    }
    
    fn priority(&self) -> i32 { 90 }
}

struct ValidationPass;
impl CodeGenPass for ValidationPass {
    fn name(&self) -> &str { "validation" }
    
    fn execute(&self, ir: &mut IrNode, _config: &CodeGenConfig) -> EnhancedResult<()> {
        let errors = ir.validate();
        if !errors.is_empty() {
            return Err(EnhancedCodeGenError::InvalidIR(errors));
        }
        Ok(())
    }
    
    fn priority(&self) -> i32 { 1000 }
}

struct OptimizationPass;
impl CodeGenPass for OptimizationPass {
    fn name(&self) -> &str { "optimization" }
    
    fn execute(&self, _ir: &mut IrNode, _config: &CodeGenConfig) -> EnhancedResult<()> {
        // Optimize IR - placeholder
        Ok(())
    }
    
    fn priority(&self) -> i32 { 10 }
}

// Built-in code emitters

struct RustEmitter;
impl CodeEmitter for RustEmitter {
    fn target_language(&self) -> TargetLanguage {
        TargetLanguage::Rust
    }
    
    fn emit_node(&self, node: &IrNode, _context: &EmitContext) -> EnhancedResult<String> {
        match &node.node_type {
            IrNodeType::Comment { content, comment_type } => {
                match comment_type {
                    CommentType::Line => Ok(format!("// {}", content)),
                    CommentType::Block => Ok(format!("/* {} */", content)),
                    CommentType::Documentation => Ok(format!("/// {}", content)),
                    CommentType::Generated => Ok(format!("// Generated: {}", content)),
                }
            }
            _ => Ok(format!("// TODO: Implement emission for {:?}", node.node_type))
        }
    }
}

struct TypeScriptEmitter;
impl CodeEmitter for TypeScriptEmitter {
    fn target_language(&self) -> TargetLanguage {
        TargetLanguage::TypeScript
    }
    
    fn emit_node(&self, node: &IrNode, _context: &EmitContext) -> EnhancedResult<String> {
        match &node.node_type {
            IrNodeType::Comment { content, comment_type } => {
                match comment_type {
                    CommentType::Line => Ok(format!("// {}", content)),
                    CommentType::Block => Ok(format!("/* {} */", content)),
                    CommentType::Documentation => Ok(format!("/** {} */", content)),
                    CommentType::Generated => Ok(format!("// Generated: {}", content)),
                }
            }
            _ => Ok(format!("// TODO: Implement emission for {:?}", node.node_type))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_enhanced_generator_creation() {
        let config = CodeGenConfig::default();
        let generator = EnhancedCodeGenerator::new(config);
        
        assert!(!generator.passes.is_empty());
        assert!(!generator.emitters.is_empty());
        assert!(generator.emitters.contains_key(&TargetLanguage::Rust));
    }
    
    #[test]
    fn test_pass_ordering() {
        let config = CodeGenConfig::default();
        let generator = EnhancedCodeGenerator::new(config);
        
        // Validation pass should be first (highest priority)
        assert_eq!(generator.pass_order[0], "validation");
    }
    
    #[test]
    fn test_template_registration() {
        let config = CodeGenConfig::default();
        let mut generator = EnhancedCodeGenerator::new(config);
        
        let template = IrTemplate {
            id: "test_template".to_string(),
            ir_tree: IrNode::comment("Test template".to_string(), CommentType::Generated),
            parameters: HashMap::new(),
            target_languages: vec![TargetLanguage::Rust],
            metadata: HashMap::new(),
        };
        
        generator.register_template(template);
        assert!(generator.templates.contains_key("test_template"));
    }
}