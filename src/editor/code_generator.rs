//! # Code Generation System with Guarded Sections
//!
//! This module provides code generation capabilities for the visual designer,
//! implementing guarded sections that preserve user modifications while allowing
//! automated code updates. The system ensures that generated code can be safely
//! regenerated without losing custom user code.
//!
//! ## Core Features
//!
//! - **Guarded Sections**: Protected code regions that preserve user modifications
//! - **Snapshot Serialization**: State persistence for incremental updates
//! - **Live Preview**: Real-time code preview for visual designer changes
//! - **Template System**: Extensible code generation templates
//! - **Diff-based Updates**: Minimal code changes to preserve history
//!
//! ## Architecture
//!
//! The code generator uses a template-based approach with guarded sections that
//! mark boundaries between generated and user-editable code. This allows the
//! visual designer to update generated portions while preserving user customizations.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for code generation templates
pub type TemplateId = String;

/// Unique identifier for guarded sections within generated code
pub type GuardId = String;

/// Code generation errors
#[derive(Debug, Clone)]
pub enum CodeGenError {
    /// Template not found
    TemplateNotFound(TemplateId),
    /// Invalid guard section syntax
    InvalidGuardSyntax(String),
    /// File I/O error during generation
    FileError(String),
    /// Serialization error for snapshots
    SerializationError(String),
    /// Template compilation error
    TemplateError(String),
}

impl std::fmt::Display for CodeGenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TemplateNotFound(id) => write!(f, "Template not found: {}", id),
            Self::InvalidGuardSyntax(msg) => write!(f, "Invalid guard syntax: {}", msg),
            Self::FileError(msg) => write!(f, "File error: {}", msg),
            Self::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            Self::TemplateError(msg) => write!(f, "Template error: {}", msg),
        }
    }
}

impl std::error::Error for CodeGenError {}

/// Result type for code generation operations
pub type CodeGenResult<T> = Result<T, CodeGenError>;

/// Represents a guarded section in generated code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardedSection {
    /// Unique identifier for this guard
    pub guard_id: GuardId,
    /// User-editable content within the guard
    pub user_content: String,
    /// Metadata for the guard (optional)
    pub metadata: HashMap<String, String>,
    /// Whether this section has been modified by the user
    pub is_modified: bool,
}

impl GuardedSection {
    /// Create a new guarded section with default content
    pub fn new(guard_id: GuardId, default_content: String) -> Self {
        Self {
            guard_id,
            user_content: default_content,
            metadata: HashMap::new(),
            is_modified: false,
        }
    }

    /// Update the user content and mark as modified
    pub fn update_content(&mut self, content: String) {
        if content != self.user_content {
            self.user_content = content;
            self.is_modified = true;
        }
    }

    /// Add metadata to the guard section
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Generate the guard markers for this section
    pub fn generate_markers(&self) -> (String, String) {
        let start_marker = format!("// <guard:{}:start>", self.guard_id);
        let end_marker = format!("// <guard:{}:end>", self.guard_id);
        (start_marker, end_marker)
    }
}

/// Code generation template with embedded guard sections
#[derive(Debug, Clone)]
pub struct CodeTemplate {
    /// Unique identifier for this template
    pub template_id: TemplateId,
    /// Template content with placeholder syntax
    pub content: String,
    /// Default guard sections for this template
    pub default_guards: HashMap<GuardId, String>,
    /// Template variables and their types
    pub variables: HashMap<String, TemplateVariableType>,
    /// Template metadata
    pub metadata: HashMap<String, String>,
}

/// Types of template variables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemplateVariableType {
    String,
    Boolean,
    Integer,
    Float,
    Array(Box<TemplateVariableType>),
    Object(HashMap<String, TemplateVariableType>),
}

impl CodeTemplate {
    /// Create a new code template
    pub fn new(template_id: TemplateId, content: String) -> Self {
        Self {
            template_id,
            content,
            default_guards: HashMap::new(),
            variables: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add a default guard section to the template
    pub fn add_guard(&mut self, guard_id: GuardId, default_content: String) {
        self.default_guards.insert(guard_id, default_content);
    }

    /// Add a template variable with its type
    pub fn add_variable(&mut self, name: String, var_type: TemplateVariableType) {
        self.variables.insert(name, var_type);
    }

    /// Extract guard section IDs from template content
    pub fn extract_guard_ids(&self) -> CodeGenResult<Vec<GuardId>> {
        let mut guard_ids = Vec::new();
        let lines: Vec<&str> = self.content.lines().collect();
        
        for line in lines {
            let trimmed = line.trim();
            if trimmed.starts_with("{{guard:") && trimmed.ends_with("}}") {
                let guard_content = &trimmed[8..trimmed.len()-2];
                guard_ids.push(guard_content.to_string());
            }
        }
        
        Ok(guard_ids)
    }
}

/// Snapshot of generated code state for incremental updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSnapshot {
    /// Unique identifier for this snapshot
    pub snapshot_id: Uuid,
    /// Timestamp when snapshot was created
    pub timestamp: u64,
    /// File path this snapshot represents
    pub file_path: PathBuf,
    /// Template used for generation
    pub template_id: TemplateId,
    /// Template variables at time of generation
    pub template_variables: HashMap<String, serde_json::Value>,
    /// Preserved guard sections with user content
    pub guard_sections: HashMap<GuardId, GuardedSection>,
    /// Generated code hash for change detection
    pub code_hash: String,
    /// Metadata for the snapshot
    pub metadata: HashMap<String, String>,
}

impl CodeSnapshot {
    /// Create a new code snapshot
    pub fn new(
        file_path: PathBuf,
        template_id: TemplateId,
        template_variables: HashMap<String, serde_json::Value>,
        guard_sections: HashMap<GuardId, GuardedSection>,
        generated_code: &str,
    ) -> Self {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        generated_code.hash(&mut hasher);
        let code_hash = format!("{:x}", hasher.finish());
        
        Self {
            snapshot_id: Uuid::new_v4(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            file_path,
            template_id,
            template_variables,
            guard_sections,
            code_hash,
            metadata: HashMap::new(),
        }
    }

    /// Check if the generated code has changed since this snapshot
    pub fn has_code_changed(&self, new_code: &str) -> bool {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        new_code.hash(&mut hasher);
        let new_hash = format!("{:x}", hasher.finish());
        
        new_hash != self.code_hash
    }

    /// Merge guard sections from another snapshot
    pub fn merge_guards(&mut self, other: &CodeSnapshot) {
        for (guard_id, guard_section) in &other.guard_sections {
            if guard_section.is_modified {
                self.guard_sections.insert(guard_id.clone(), guard_section.clone());
            }
        }
    }
}

/// Live preview panel for real-time code generation
#[derive(Debug)]
pub struct LivePreviewPanel {
    /// Currently displayed code
    pub current_code: String,
    /// Template being previewed
    pub template_id: Option<TemplateId>,
    /// Template variables for preview
    pub template_variables: HashMap<String, serde_json::Value>,
    /// Whether preview is enabled
    pub is_enabled: bool,
    /// Preview update callback
    pub update_callback: Option<Box<dyn Fn(&str) + Send + Sync>>,
}

impl LivePreviewPanel {
    /// Create a new live preview panel
    pub fn new() -> Self {
        Self {
            current_code: String::new(),
            template_id: None,
            template_variables: HashMap::new(),
            is_enabled: true,
            update_callback: None,
        }
    }

    /// Update the preview with new code
    pub fn update_preview(&mut self, code: String) {
        self.current_code = code.clone();
        if let Some(callback) = &self.update_callback {
            callback(&code);
        }
    }

    /// Set template variables for preview
    pub fn set_variables(&mut self, variables: HashMap<String, serde_json::Value>) {
        self.template_variables = variables;
    }

    /// Enable or disable live preview
    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }

    /// Set callback for preview updates
    pub fn set_update_callback<F>(&mut self, callback: F)
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        self.update_callback = Some(Box::new(callback));
    }
}

/// Main code generator with guarded sections support
pub struct CodeGenerator {
    /// Available code templates
    templates: HashMap<TemplateId, CodeTemplate>,
    /// Code generation snapshots for incremental updates
    snapshots: HashMap<PathBuf, CodeSnapshot>,
    /// Live preview panel
    preview_panel: LivePreviewPanel,
    /// Output directory for generated files
    output_dir: PathBuf,
}

impl CodeGenerator {
    /// Create a new code generator
    pub fn new(output_dir: PathBuf) -> Self {
        Self {
            templates: HashMap::new(),
            snapshots: HashMap::new(),
            preview_panel: LivePreviewPanel::new(),
            output_dir,
        }
    }

    /// Register a new code template
    pub fn register_template(&mut self, template: CodeTemplate) {
        self.templates.insert(template.template_id.clone(), template);
    }

    /// Generate code from a template with guarded sections
    pub fn generate_code(
        &mut self,
        template_id: &TemplateId,
        variables: HashMap<String, serde_json::Value>,
        output_file: PathBuf,
    ) -> CodeGenResult<String> {
        let template = self.templates.get(template_id)
            .ok_or_else(|| CodeGenError::TemplateNotFound(template_id.clone()))?;

        // Load existing snapshot if available
        let existing_snapshot = self.snapshots.get(&output_file);
        
        // Parse existing guard sections if file exists
        let existing_guards = if let Some(snapshot) = existing_snapshot {
            snapshot.guard_sections.clone()
        } else {
            self.parse_existing_guards(&output_file)?
        };

        // Generate code with template variables
        let mut generated_code = template.content.clone();
        
        // Replace template variables
        for (var_name, value) in &variables {
            let placeholder = format!("{{{{{}}}}}", var_name);
            let string_value = self.serialize_template_value(value);
            generated_code = generated_code.replace(&placeholder, &string_value);
        }

        // Process guard sections
        generated_code = self.process_guard_sections(
            generated_code,
            &template.default_guards,
            &existing_guards,
        )?;

        // Create or update snapshot
        let guard_sections = self.extract_guard_sections(&generated_code)?;
        let snapshot = CodeSnapshot::new(
            output_file.clone(),
            template_id.clone(),
            variables,
            guard_sections,
            &generated_code,
        );
        
        self.snapshots.insert(output_file, snapshot);

        // Update live preview if enabled
        if self.preview_panel.is_enabled {
            self.preview_panel.update_preview(generated_code.clone());
        }

        Ok(generated_code)
    }

    /// Parse existing guard sections from a file
    fn parse_existing_guards(&self, file_path: &PathBuf) -> CodeGenResult<HashMap<GuardId, GuardedSection>> {
        if !file_path.exists() {
            return Ok(HashMap::new());
        }

        let content = std::fs::read_to_string(file_path)
            .map_err(|e| CodeGenError::FileError(e.to_string()))?;

        self.extract_guard_sections(&content)
    }

    /// Extract guard sections from generated code
    fn extract_guard_sections(&self, code: &str) -> CodeGenResult<HashMap<GuardId, GuardedSection>> {
        let mut guard_sections = HashMap::new();
        let lines: Vec<&str> = code.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();
            
            if line.starts_with("// <guard:") && line.ends_with(":start>") {
                let guard_id = line[10..line.len()-7].to_string();
                let mut content = String::new();
                i += 1;

                // Collect content until end marker
                while i < lines.len() {
                    let current_line = lines[i].trim();
                    if current_line == format!("// <guard:{}:end>", guard_id) {
                        break;
                    }
                    content.push_str(lines[i]);
                    content.push('\n');
                    i += 1;
                }

                let guard_section = GuardedSection::new(guard_id.clone(), content);
                guard_sections.insert(guard_id, guard_section);
            }
            i += 1;
        }

        Ok(guard_sections)
    }

    /// Process guard sections in generated code
    fn process_guard_sections(
        &self,
        mut code: String,
        default_guards: &HashMap<GuardId, String>,
        existing_guards: &HashMap<GuardId, GuardedSection>,
    ) -> CodeGenResult<String> {
        // Replace guard placeholders with actual guard sections
        for (guard_id, default_content) in default_guards {
            let placeholder = format!("{{{{guard:{}}}}}", guard_id);
            
            let guard_content = if let Some(existing_guard) = existing_guards.get(guard_id) {
                // Use existing user content if available
                existing_guard.user_content.clone()
            } else {
                // Use default content for new guards
                default_content.clone()
            };

            let guard_section = GuardedSection::new(guard_id.clone(), guard_content);
            let (start_marker, end_marker) = guard_section.generate_markers();
            
            let full_guard = format!(
                "{}\n{}\n{}",
                start_marker,
                guard_section.user_content,
                end_marker
            );

            code = code.replace(&placeholder, &full_guard);
        }

        Ok(code)
    }

    /// Serialize a template value to string
    fn serialize_template_value(&self, value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => b.to_string(),
            serde_json::Value::Array(arr) => {
                let items: Vec<String> = arr.iter()
                    .map(|v| self.serialize_template_value(v))
                    .collect();
                format!("[{}]", items.join(", "))
            }
            serde_json::Value::Object(obj) => {
                let items: Vec<String> = obj.iter()
                    .map(|(k, v)| format!("{}: {}", k, self.serialize_template_value(v)))
                    .collect();
                format!("{{{}}}", items.join(", "))
            }
            serde_json::Value::Null => "null".to_string(),
        }
    }

    /// Save snapshot to disk for persistence
    pub fn save_snapshot(&self, file_path: &PathBuf) -> CodeGenResult<()> {
        if let Some(snapshot) = self.snapshots.get(file_path) {
            let snapshot_path = self.output_dir.join(format!("{}.snapshot", 
                file_path.file_name().unwrap().to_string_lossy()));
            
            let serialized = serde_json::to_string_pretty(snapshot)
                .map_err(|e| CodeGenError::SerializationError(e.to_string()))?;
            
            std::fs::write(snapshot_path, serialized)
                .map_err(|e| CodeGenError::FileError(e.to_string()))?;
        }
        Ok(())
    }

    /// Load snapshot from disk
    pub fn load_snapshot(&mut self, file_path: &PathBuf) -> CodeGenResult<()> {
        let snapshot_path = self.output_dir.join(format!("{}.snapshot", 
            file_path.file_name().unwrap().to_string_lossy()));
        
        if snapshot_path.exists() {
            let content = std::fs::read_to_string(snapshot_path)
                .map_err(|e| CodeGenError::FileError(e.to_string()))?;
            
            let snapshot: CodeSnapshot = serde_json::from_str(&content)
                .map_err(|e| CodeGenError::SerializationError(e.to_string()))?;
            
            self.snapshots.insert(file_path.clone(), snapshot);
        }
        Ok(())
    }

    /// Get the live preview panel
    pub fn get_preview_panel(&mut self) -> &mut LivePreviewPanel {
        &mut self.preview_panel
    }

    /// Get available templates
    pub fn get_templates(&self) -> Vec<&CodeTemplate> {
        self.templates.values().collect()
    }

    /// Remove a template
    pub fn remove_template(&mut self, template_id: &TemplateId) -> bool {
        self.templates.remove(template_id).is_some()
    }

    /// Clear all snapshots
    pub fn clear_snapshots(&mut self) {
        self.snapshots.clear();
    }

    /// Get snapshot for file
    pub fn get_snapshot(&self, file_path: &PathBuf) -> Option<&CodeSnapshot> {
        self.snapshots.get(file_path)
    }

    /// Update guard section in existing snapshot
    pub fn update_guard_section(
        &mut self,
        file_path: &PathBuf,
        guard_id: &GuardId,
        new_content: String,
    ) -> CodeGenResult<()> {
        if let Some(snapshot) = self.snapshots.get_mut(file_path) {
            if let Some(guard) = snapshot.guard_sections.get_mut(guard_id) {
                guard.update_content(new_content);
            }
        }
        Ok(())
    }
}

/// Default code templates for common use cases
impl CodeGenerator {
    /// Create a basic Rust component template
    pub fn create_rust_component_template() -> CodeTemplate {
        let mut template = CodeTemplate::new(
            "rust_component".to_string(),
            r#"//! {{description}}
//!
//! This component was generated by the visual designer.
//! Custom code should be placed in the guarded sections below.

use crate::rcl::ui::prelude::*;

/// {{component_name}} component
#[derive(Debug, Clone)]
pub struct {{component_name}} {
    {{guard:fields}}
    // Add custom fields here
    {{guard:fields}}
    
    /// Component properties
    pub props: {{component_name}}Props,
}

/// Properties for {{component_name}}
#[derive(Debug, Clone, Default)]
pub struct {{component_name}}Props {
    pub name: String,
    pub enabled: bool,
    {{guard:props}}
    // Add custom properties here
    {{guard:props}}
}

impl {{component_name}} {
    /// Create a new {{component_name}}
    pub fn new(props: {{component_name}}Props) -> Self {
        Self {
            {{guard:constructor}}
            // Initialize custom fields here
            {{guard:constructor}}
            props,
        }
    }

    {{guard:methods}}
    // Add custom methods here
    {{guard:methods}}
}

impl Component for {{component_name}} {
    type Props = {{component_name}}Props;

    fn render(&self, ctx: &mut RenderContext) -> RenderResult {
        {{guard:render}}
        // Custom render logic here
        {{guard:render}}
        Ok(())
    }

    fn update(&mut self, props: Self::Props) -> UpdateResult {
        self.props = props;
        {{guard:update}}
        // Custom update logic here
        {{guard:update}}
        Ok(())
    }
}

{{guard:impl_blocks}}
// Add custom implementations here
{{guard:impl_blocks}}
"#.to_string(),
        );

        // Add default guard sections
        template.add_guard("fields".to_string(), "    // Custom fields".to_string());
        template.add_guard("props".to_string(), "    // Custom properties".to_string());
        template.add_guard("constructor".to_string(), "        // Custom initialization".to_string());
        template.add_guard("methods".to_string(), "    // Custom methods".to_string());
        template.add_guard("render".to_string(), "        // Custom render logic".to_string());
        template.add_guard("update".to_string(), "        // Custom update logic".to_string());
        template.add_guard("impl_blocks".to_string(), "// Custom implementations".to_string());

        // Add template variables
        template.add_variable("component_name".to_string(), TemplateVariableType::String);
        template.add_variable("description".to_string(), TemplateVariableType::String);

        template
    }

    /// Create a basic form template
    pub fn create_form_template() -> CodeTemplate {
        let mut template = CodeTemplate::new(
            "form_component".to_string(),
            r#"//! {{form_name}} form component
//!
//! This form was generated by the visual designer.

use crate::rcl::ui::prelude::*;

/// {{form_name}} form
#[derive(Debug)]
pub struct {{form_name}} {
    {{guard:form_fields}}
    // Add form fields here
    {{guard:form_fields}}
}

impl {{form_name}} {
    pub fn new() -> Self {
        Self {
            {{guard:form_init}}
            // Initialize form fields
            {{guard:form_init}}
        }
    }

    {{guard:form_methods}}
    // Add form methods here
    {{guard:form_methods}}
}

impl Component for {{form_name}} {
    type Props = ();

    fn render(&self, ctx: &mut RenderContext) -> RenderResult {
        {{guard:form_render}}
        // Form rendering logic
        {{guard:form_render}}
        Ok(())
    }
}
"#.to_string(),
        );

        template.add_guard("form_fields".to_string(), "    // Form fields".to_string());
        template.add_guard("form_init".to_string(), "        // Form initialization".to_string());
        template.add_guard("form_methods".to_string(), "    // Form methods".to_string());
        template.add_guard("form_render".to_string(), "        // Form render logic".to_string());

        template.add_variable("form_name".to_string(), TemplateVariableType::String);

        template
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_guarded_section_creation() {
        let guard = GuardedSection::new("test_guard".to_string(), "default content".to_string());
        assert_eq!(guard.guard_id, "test_guard");
        assert_eq!(guard.user_content, "default content");
        assert!(!guard.is_modified);
    }

    #[test]
    fn test_guarded_section_update() {
        let mut guard = GuardedSection::new("test_guard".to_string(), "default".to_string());
        guard.update_content("new content".to_string());
        assert_eq!(guard.user_content, "new content");
        assert!(guard.is_modified);
    }

    #[test]
    fn test_template_creation() {
        let template = CodeTemplate::new("test_template".to_string(), "{{variable}}".to_string());
        assert_eq!(template.template_id, "test_template");
        assert_eq!(template.content, "{{variable}}");
    }

    #[test]
    fn test_code_generator_creation() {
        let output_dir = PathBuf::from("./output");
        let generator = CodeGenerator::new(output_dir.clone());
        assert_eq!(generator.output_dir, output_dir);
        assert!(generator.templates.is_empty());
    }

    #[test]
    fn test_template_registration() {
        let mut generator = CodeGenerator::new(PathBuf::from("./output"));
        let template = CodeTemplate::new("test".to_string(), "content".to_string());
        
        generator.register_template(template);
        assert!(generator.templates.contains_key("test"));
    }

    #[test]
    fn test_rust_component_template() {
        let template = CodeGenerator::create_rust_component_template();
        assert_eq!(template.template_id, "rust_component");
        assert!(template.content.contains("{{component_name}}"));
        assert!(template.default_guards.contains_key("fields"));
    }
}