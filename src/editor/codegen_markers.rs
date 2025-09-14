//! Enhanced Code Generation Markers and Rewrite System
//!
//! This module provides advanced code generation markers that enable sophisticated
//! code rewriting while preserving user modifications. It extends the basic guard
//! system with typed markers, conditional generation, and intelligent merging.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Enhanced marker types for different code generation scenarios
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MarkerType {
    /// Simple guard section (user-editable content)
    Guard { 
        /// Guard identifier
        id: String,
        /// Whether the guard preserves indentation
        preserve_indent: bool,
        /// Default content if guard is empty
        default_content: Option<String>,
    },
    
    /// Generated section (auto-generated, not user-editable)  
    Generated {
        /// Generator identifier
        id: String,
        /// Generation strategy
        strategy: GenerationStrategy,
        /// Dependencies for regeneration
        dependencies: Vec<String>,
    },
    
    /// Conditional section (generated based on conditions)
    Conditional {
        /// Condition identifier
        id: String,
        /// Condition expression
        condition: String,
        /// Conditional strategy
        strategy: ConditionalStrategy,
    },
    
    /// Import section (manages imports/dependencies)
    Import {
        /// Import type (module, dependency, etc.)
        import_type: ImportType,
        /// Merge strategy for import conflicts
        merge_strategy: ImportMergeStrategy,
    },
    
    /// Template section (parameterized code generation)
    Template {
        /// Template identifier
        id: String,
        /// Template parameters
        parameters: HashMap<String, TemplateParameter>,
        /// Iteration settings if template repeats
        iteration: Option<IterationSettings>,
    },
}

/// Strategies for code generation within markers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GenerationStrategy {
    /// Always regenerate (overwrite existing)
    Replace,
    /// Merge with existing content
    Merge,
    /// Only generate if section is empty
    IfEmpty,
    /// Append to existing content
    Append,
    /// Prepend to existing content 
    Prepend,
}

/// Strategies for conditional code generation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionalStrategy {
    /// Include section if condition is true
    Include,
    /// Exclude section if condition is true  
    Exclude,
    /// Switch between multiple alternatives
    Switch { alternatives: HashMap<String, String> },
}

/// Types of imports for import markers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImportType {
    /// Module import (e.g., use std::collections::HashMap)
    Module,
    /// External dependency
    Dependency,
    /// Local file import
    Local,
    /// Namespace import
    Namespace,
}

/// Strategies for merging import conflicts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImportMergeStrategy {
    /// Keep existing imports
    KeepExisting,
    /// Replace with new imports
    Replace,
    /// Merge and deduplicate
    Merge,
    /// Ask user for conflict resolution
    Interactive,
}

/// Template parameter definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplateParameter {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: ParameterType,
    /// Default value
    pub default_value: Option<serde_json::Value>,
    /// Whether parameter is required
    pub required: bool,
    /// Parameter description
    pub description: Option<String>,
}

/// Types of template parameters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParameterType {
    String,
    Integer,
    Float,
    Boolean,
    Array(Box<ParameterType>),
    Object(HashMap<String, ParameterType>),
    Custom(String),
}

/// Settings for template iteration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IterationSettings {
    /// Data source for iteration
    pub data_source: String,
    /// Item variable name in template
    pub item_var: String,
    /// Index variable name (optional)
    pub index_var: Option<String>,
    /// Separator between iterations
    pub separator: Option<String>,
}

/// Enhanced code marker with position and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeMarker {
    /// Unique marker ID
    pub id: Uuid,
    /// Marker type and configuration
    pub marker_type: MarkerType,
    /// Start position in source code
    pub start_position: CodePosition,
    /// End position in source code
    pub end_position: CodePosition,
    /// Current content within marker
    pub content: String,
    /// Original generated content (for diff comparison)
    pub original_content: Option<String>,
    /// Whether marker has been modified by user
    pub is_modified: bool,
    /// Metadata for marker
    pub metadata: HashMap<String, String>,
    /// Timestamp of last modification
    pub last_modified: Option<u64>,
}

/// Position within source code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodePosition {
    /// Line number (0-based)
    pub line: usize,
    /// Column number (0-based)  
    pub column: usize,
    /// Byte offset from start of file
    pub offset: usize,
}

impl CodeMarker {
    /// Create a new code marker
    pub fn new(marker_type: MarkerType, start_pos: CodePosition, end_pos: CodePosition) -> Self {
        Self {
            id: Uuid::new_v4(),
            marker_type,
            start_position: start_pos,
            end_position: end_pos,
            content: String::new(),
            original_content: None,
            is_modified: false,
            metadata: HashMap::new(),
            last_modified: None,
        }
    }
    
    /// Generate marker start comment for the given language
    pub fn start_marker(&self, language: &CodeLanguage) -> String {
        let comment_style = language.comment_style();
        let marker_id = match &self.marker_type {
            MarkerType::Guard { id, .. } => format!("guard:{}", id),
            MarkerType::Generated { id, .. } => format!("generated:{}", id),
            MarkerType::Conditional { id, .. } => format!("conditional:{}", id),
            MarkerType::Import { import_type, .. } => format!("import:{:?}", import_type),
            MarkerType::Template { id, .. } => format!("template:{}", id),
        };
        
        match comment_style {
            CommentStyle::DoubleSlash => format!("// <codegen:{}:start>", marker_id),
            CommentStyle::Hash => format!("# <codegen:{}:start>", marker_id),
            CommentStyle::Block => format!("/* <codegen:{}:start> */", marker_id),
        }
    }
    
    /// Generate marker end comment for the given language
    pub fn end_marker(&self, language: &CodeLanguage) -> String {
        let comment_style = language.comment_style();
        let marker_id = match &self.marker_type {
            MarkerType::Guard { id, .. } => format!("guard:{}", id),
            MarkerType::Generated { id, .. } => format!("generated:{}", id),
            MarkerType::Conditional { id, .. } => format!("conditional:{}", id),
            MarkerType::Import { import_type, .. } => format!("import:{:?}", import_type),
            MarkerType::Template { id, .. } => format!("template:{}", id),
        };
        
        match comment_style {
            CommentStyle::DoubleSlash => format!("// <codegen:{}:end>", marker_id),
            CommentStyle::Hash => format!("# <codegen:{}:end>", marker_id),
            CommentStyle::Block => format!("/* <codegen:{}:end> */", marker_id),
        }
    }
    
    /// Update marker content and track modifications
    pub fn update_content(&mut self, new_content: String) {
        if let Some(original) = &self.original_content {
            self.is_modified = new_content != *original;
        }
        self.content = new_content;
        self.last_modified = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
    }
    
    /// Check if marker should be regenerated based on dependencies
    pub fn should_regenerate(&self, changed_dependencies: &HashSet<String>) -> bool {
        match &self.marker_type {
            MarkerType::Generated { dependencies, .. } => {
                dependencies.iter().any(|dep| changed_dependencies.contains(dep))
            }
            MarkerType::Template { .. } => true, // Templates can always be regenerated
            _ => false, // Guards and other types preserve user content
        }
    }
}

/// Supported programming languages for code generation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CodeLanguage {
    Rust,
    JavaScript,
    TypeScript,
    Python,
    Java,
    CSharp,
    Cpp,
    Go,
    Html,
    Css,
    Json,
    Yaml,
    Toml,
}

impl CodeLanguage {
    /// Get the comment style for this language
    pub fn comment_style(&self) -> CommentStyle {
        match self {
            Self::Rust | Self::JavaScript | Self::TypeScript | Self::Java | 
            Self::CSharp | Self::Cpp | Self::Go => CommentStyle::DoubleSlash,
            Self::Python | Self::Yaml => CommentStyle::Hash,
            Self::Html => CommentStyle::Block,
            Self::Css => CommentStyle::Block,
            Self::Json | Self::Toml => CommentStyle::DoubleSlash, // Approximation
        }
    }
    
    /// Get file extensions for this language
    pub fn file_extensions(&self) -> Vec<&'static str> {
        match self {
            Self::Rust => vec!["rs"],
            Self::JavaScript => vec!["js", "mjs"],
            Self::TypeScript => vec!["ts", "tsx"],
            Self::Python => vec!["py"],
            Self::Java => vec!["java"],
            Self::CSharp => vec!["cs"],
            Self::Cpp => vec!["cpp", "cc", "cxx"],
            Self::Go => vec!["go"],
            Self::Html => vec!["html", "htm"],
            Self::Css => vec!["css"],
            Self::Json => vec!["json"],
            Self::Yaml => vec!["yaml", "yml"],
            Self::Toml => vec!["toml"],
        }
    }
    
    /// Detect language from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "rs" => Some(Self::Rust),
            "js" | "mjs" => Some(Self::JavaScript),
            "ts" | "tsx" => Some(Self::TypeScript),
            "py" => Some(Self::Python),
            "java" => Some(Self::Java),
            "cs" => Some(Self::CSharp),
            "cpp" | "cc" | "cxx" => Some(Self::Cpp),
            "go" => Some(Self::Go),
            "html" | "htm" => Some(Self::Html),
            "css" => Some(Self::Css),
            "json" => Some(Self::Json),
            "yaml" | "yml" => Some(Self::Yaml),
            "toml" => Some(Self::Toml),
            _ => None,
        }
    }
}

/// Comment styles for different languages
#[derive(Debug, Clone, PartialEq)]
pub enum CommentStyle {
    /// // style comments
    DoubleSlash,
    /// # style comments  
    Hash,
    /// /* */ style comments
    Block,
}

/// Enhanced code rewriter with marker support
pub struct CodeRewriter {
    /// Detected code language
    pub language: CodeLanguage,
    /// Currently parsed markers
    pub markers: Vec<CodeMarker>,
    /// Original source code
    pub original_code: String,
    /// Modified source code
    pub modified_code: String,
}

impl CodeRewriter {
    /// Create a new code rewriter for the given language
    pub fn new(language: CodeLanguage, source_code: String) -> Self {
        Self {
            language,
            markers: Vec::new(),
            original_code: source_code.clone(),
            modified_code: source_code,
        }
    }
    
    /// Parse markers from source code
    pub fn parse_markers(&mut self) -> Result<(), String> {
        let lines: Vec<&str> = self.original_code.lines().collect();
        let mut i = 0;
        
        while i < lines.len() {
            if let Some(marker) = self.try_parse_marker_at_line(&lines, i)? {
                // Find the end of this marker
                let end_line = self.find_marker_end(&lines, i + 1, &marker)?;
                
                // Extract content between markers
                let content_lines = &lines[i + 1..end_line];
                let content = content_lines.join("\n");
                
                let mut marker = marker;
                marker.content = content;
                marker.end_position = self.position_at_line(end_line);
                
                self.markers.push(marker);
                i = end_line + 1;
            } else {
                i += 1;
            }
        }
        
        Ok(())
    }
    
    /// Try to parse a marker at the given line
    fn try_parse_marker_at_line(&self, lines: &[&str], line_idx: usize) -> Result<Option<CodeMarker>, String> {
        let line = lines[line_idx].trim();
        let comment_prefix = match self.language.comment_style() {
            CommentStyle::DoubleSlash => "//",
            CommentStyle::Hash => "#",
            CommentStyle::Block => "/*",
        };
        
        if !line.starts_with(comment_prefix) || !line.contains("<codegen:") || !line.contains(":start>") {
            return Ok(None);
        }
        
        // Extract marker type and ID
        let marker_start = line.find("<codegen:").unwrap() + 9;
        let marker_end = line.find(":start>").unwrap();
        let marker_spec = &line[marker_start..marker_end];
        
        let marker_type = self.parse_marker_type(marker_spec)?;
        let start_pos = self.position_at_line(line_idx);
        let end_pos = start_pos.clone(); // Will be updated when end is found
        
        Ok(Some(CodeMarker::new(marker_type, start_pos, end_pos)))
    }
    
    /// Parse marker type from specification string
    fn parse_marker_type(&self, spec: &str) -> Result<MarkerType, String> {
        let parts: Vec<&str> = spec.split(':').collect();
        if parts.is_empty() {
            return Err("Empty marker specification".to_string());
        }
        
        match parts[0] {
            "guard" => {
                let id = parts.get(1).unwrap_or(&"default").to_string();
                Ok(MarkerType::Guard {
                    id,
                    preserve_indent: true,
                    default_content: None,
                })
            }
            "generated" => {
                let id = parts.get(1).unwrap_or(&"default").to_string();
                Ok(MarkerType::Generated {
                    id,
                    strategy: GenerationStrategy::Replace,
                    dependencies: Vec::new(),
                })
            }
            "conditional" => {
                let id = parts.get(1).unwrap_or(&"default").to_string();
                Ok(MarkerType::Conditional {
                    id,
                    condition: "true".to_string(),
                    strategy: ConditionalStrategy::Include,
                })
            }
            "import" => {
                Ok(MarkerType::Import {
                    import_type: ImportType::Module,
                    merge_strategy: ImportMergeStrategy::Merge,
                })
            }
            "template" => {
                let id = parts.get(1).unwrap_or(&"default").to_string();
                Ok(MarkerType::Template {
                    id,
                    parameters: HashMap::new(),
                    iteration: None,
                })
            }
            _ => Err(format!("Unknown marker type: {}", parts[0])),
        }
    }
    
    /// Find the end marker for a given start marker
    fn find_marker_end(&self, lines: &[&str], start_line: usize, marker: &CodeMarker) -> Result<usize, String> {
        let marker_id = self.get_marker_id(marker);
        let comment_prefix = match self.language.comment_style() {
            CommentStyle::DoubleSlash => "//",
            CommentStyle::Hash => "#", 
            CommentStyle::Block => "/*",
        };
        
        let end_marker_text = format!("{} <codegen:{}:end>", comment_prefix, marker_id);
        
        for i in start_line..lines.len() {
            if lines[i].trim().starts_with(&end_marker_text) {
                return Ok(i);
            }
        }
        
        Err(format!("End marker not found for {}", marker_id))
    }
    
    /// Get marker ID string for matching start/end markers
    fn get_marker_id(&self, marker: &CodeMarker) -> String {
        match &marker.marker_type {
            MarkerType::Guard { id, .. } => format!("guard:{}", id),
            MarkerType::Generated { id, .. } => format!("generated:{}", id),
            MarkerType::Conditional { id, .. } => format!("conditional:{}", id),
            MarkerType::Import { import_type, .. } => format!("import:{:?}", import_type),
            MarkerType::Template { id, .. } => format!("template:{}", id),
        }
    }
    
    /// Calculate code position for a given line
    fn position_at_line(&self, line_idx: usize) -> CodePosition {
        let lines: Vec<&str> = self.original_code.lines().collect();
        let mut offset = 0;
        
        for i in 0..line_idx.min(lines.len()) {
            offset += lines[i].len() + 1; // +1 for newline
        }
        
        CodePosition {
            line: line_idx,
            column: 0,
            offset,
        }
    }
    
    /// Rewrite code with updated markers
    pub fn rewrite_with_markers(&mut self, updated_markers: Vec<CodeMarker>) -> Result<String, String> {
        // Sort markers by position for proper replacement
        let mut sorted_markers = updated_markers;
        sorted_markers.sort_by_key(|m| m.start_position.offset);
        
        let mut result = String::new();
        let lines: Vec<&str> = self.original_code.lines().collect();
        let mut current_line = 0;
        
        for marker in &sorted_markers {
            // Add lines before this marker
            while current_line < marker.start_position.line {
                if current_line < lines.len() {
                    result.push_str(lines[current_line]);
                    result.push('\n');
                }
                current_line += 1;
            }
            
            // Add the marker with its content
            result.push_str(&marker.start_marker(&self.language));
            result.push('\n');
            result.push_str(&marker.content);
            if !marker.content.ends_with('\n') {
                result.push('\n');
            }
            result.push_str(&marker.end_marker(&self.language));
            result.push('\n');
            
            // Skip to after the marker's end
            current_line = marker.end_position.line + 1;
        }
        
        // Add remaining lines
        while current_line < lines.len() {
            result.push_str(lines[current_line]);
            result.push('\n');
            current_line += 1;
        }
        
        self.modified_code = result.clone();
        Ok(result)
    }
    
    /// Get markers by type
    pub fn markers_of_type(&self, marker_type: &str) -> Vec<&CodeMarker> {
        self.markers.iter().filter(|marker| {
            match &marker.marker_type {
                MarkerType::Guard { .. } => marker_type == "guard",
                MarkerType::Generated { .. } => marker_type == "generated",
                MarkerType::Conditional { .. } => marker_type == "conditional",
                MarkerType::Import { .. } => marker_type == "import",
                MarkerType::Template { .. } => marker_type == "template",
            }
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_code_language_detection() {
        assert_eq!(CodeLanguage::from_extension("rs"), Some(CodeLanguage::Rust));
        assert_eq!(CodeLanguage::from_extension("ts"), Some(CodeLanguage::TypeScript));
        assert_eq!(CodeLanguage::from_extension("py"), Some(CodeLanguage::Python));
        assert_eq!(CodeLanguage::from_extension("unknown"), None);
    }
    
    #[test]
    fn test_marker_generation() {
        let guard_marker = MarkerType::Guard {
            id: "test_guard".to_string(),
            preserve_indent: true,
            default_content: None,
        };
        
        let marker = CodeMarker::new(
            guard_marker,
            CodePosition { line: 0, column: 0, offset: 0 },
            CodePosition { line: 5, column: 0, offset: 100 }
        );
        
        let start_marker = marker.start_marker(&CodeLanguage::Rust);
        let end_marker = marker.end_marker(&CodeLanguage::Rust);
        
        assert_eq!(start_marker, "// <codegen:guard:test_guard:start>");
        assert_eq!(end_marker, "// <codegen:guard:test_guard:end>");
    }
    
    #[test]
    fn test_rust_code_rewriter() {
        let source_code = r#"
fn main() {
    println!("Hello, world!");
    // <codegen:guard:user_code:start>
    // User code here
    // <codegen:guard:user_code:end>
    println!("Goodbye!");
}
"#;
        
        let mut rewriter = CodeRewriter::new(CodeLanguage::Rust, source_code.to_string());
        let result = rewriter.parse_markers();
        assert!(result.is_ok());
        assert_eq!(rewriter.markers.len(), 1);
        
        let marker = &rewriter.markers[0];
        if let MarkerType::Guard { id, .. } = &marker.marker_type {
            assert_eq!(id, "user_code");
        }
    }
    
    #[test]  
    fn test_marker_type_parsing() {
        let rewriter = CodeRewriter::new(CodeLanguage::Rust, String::new());
        
        let guard_type = rewriter.parse_marker_type("guard:test_id").unwrap();
        assert!(matches!(guard_type, MarkerType::Guard { .. }));
        
        let generated_type = rewriter.parse_marker_type("generated:component").unwrap();
        assert!(matches!(generated_type, MarkerType::Generated { .. }));
        
        let invalid_result = rewriter.parse_marker_type("invalid:type");
        assert!(invalid_result.is_err());
    }
}