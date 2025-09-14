//! Unified Language Service Abstraction
//!
//! This module provides a unified trait for language services that abstracts
//! over different language server implementations (rust-analyzer, LSP clients, etc.)
//! and provides a consistent interface for the IDE.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::future::Future;
use std::pin::Pin;
use serde_json::Value;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::core::event_bus::{IdeEvent, global_event_bus};
use super::lsp_integration::{Diagnostic, Range, Position, LspError};

/// Core trait for language services
#[async_trait::async_trait]
pub trait LanguageService: Send + Sync {
    /// Initialize the language service
    async fn initialize(&mut self, workspace_root: &Path) -> Result<(), LanguageServiceError>;
    
    /// Shutdown the language service
    async fn shutdown(&mut self) -> Result<(), LanguageServiceError>;
    
    /// Check if the service supports a given file
    fn supports_file(&self, file_path: &Path) -> bool;
    
    /// Get supported file extensions
    fn supported_extensions(&self) -> Vec<&'static str>;
    
    /// Open/synchronize a document with the language service
    async fn open_document(&mut self, file_path: &Path, content: &str) -> Result<(), LanguageServiceError>;
    
    /// Update document content
    async fn update_document(&mut self, file_path: &Path, content: &str, version: u64) -> Result<(), LanguageServiceError>;
    
    /// Close a document
    async fn close_document(&mut self, file_path: &Path) -> Result<(), LanguageServiceError>;
    
    /// Request code completion at a position
    async fn completion(&mut self, file_path: &Path, position: Position) -> Result<Vec<CompletionItem>, LanguageServiceError>;
    
    /// Request hover information at a position
    async fn hover(&mut self, file_path: &Path, position: Position) -> Result<Option<HoverInfo>, LanguageServiceError>;
    
    /// Go to definition at a position
    async fn goto_definition(&mut self, file_path: &Path, position: Position) -> Result<Vec<Location>, LanguageServiceError>;
    
    /// Find references for a symbol at a position
    async fn find_references(&mut self, file_path: &Path, position: Position) -> Result<Vec<Location>, LanguageServiceError>;
    
    /// Get diagnostics for a file
    async fn diagnostics(&mut self, file_path: &Path) -> Result<Vec<Diagnostic>, LanguageServiceError>;
    
    /// Request code actions for a range
    async fn code_actions(&mut self, file_path: &Path, range: Range) -> Result<Vec<CodeAction>, LanguageServiceError>;
    
    /// Format document
    async fn format_document(&mut self, file_path: &Path) -> Result<Vec<TextEdit>, LanguageServiceError>;
    
    /// Format selection
    async fn format_range(&mut self, file_path: &Path, range: Range) -> Result<Vec<TextEdit>, LanguageServiceError>;
    
    /// Rename symbol
    async fn rename(&mut self, file_path: &Path, position: Position, new_name: &str) -> Result<HashMap<PathBuf, Vec<TextEdit>>, LanguageServiceError>;
    
    /// Get document symbols
    async fn document_symbols(&mut self, file_path: &Path) -> Result<Vec<DocumentSymbol>, LanguageServiceError>;
    
    /// Search workspace symbols
    async fn workspace_symbols(&mut self, query: &str) -> Result<Vec<WorkspaceSymbol>, LanguageServiceError>;
    
    /// Get signature help
    async fn signature_help(&mut self, file_path: &Path, position: Position) -> Result<Option<SignatureHelp>, LanguageServiceError>;
    
    /// Get semantic tokens
    async fn semantic_tokens(&mut self, file_path: &Path) -> Result<Vec<SemanticToken>, LanguageServiceError>;
    
    /// Get folding ranges
    async fn folding_ranges(&mut self, file_path: &Path) -> Result<Vec<FoldingRange>, LanguageServiceError>;
}

/// Error types for language services
#[derive(Debug, Clone)]
pub enum LanguageServiceError {
    /// Service not initialized
    NotInitialized,
    /// Communication error with language server
    Communication(String),
    /// Request timeout
    Timeout,
    /// Invalid request/parameters
    InvalidRequest(String),
    /// File not found or not supported
    FileNotSupported(PathBuf),
    /// Server crashed or stopped responding
    ServerUnresponsive,
    /// General error
    Other(String),
}

impl std::fmt::Display for LanguageServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotInitialized => write!(f, "Language service not initialized"),
            Self::Communication(msg) => write!(f, "Communication error: {}", msg),
            Self::Timeout => write!(f, "Request timeout"),
            Self::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            Self::FileNotSupported(path) => write!(f, "File not supported: {}", path.display()),
            Self::ServerUnresponsive => write!(f, "Language server is unresponsive"),
            Self::Other(msg) => write!(f, "Language service error: {}", msg),
        }
    }
}

impl std::error::Error for LanguageServiceError {}

/// Completion item for code completion
#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionKind,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub insert_text: Option<String>,
    pub filter_text: Option<String>,
    pub sort_text: Option<String>,
    pub additional_text_edits: Vec<TextEdit>,
}

/// Types of completion items
#[derive(Debug, Clone)]
pub enum CompletionKind {
    Text,
    Method,
    Function,
    Constructor,
    Field,
    Variable,
    Class,
    Interface,
    Module,
    Property,
    Unit,
    Value,
    Enum,
    Keyword,
    Snippet,
    Color,
    File,
    Reference,
    Folder,
    EnumMember,
    Constant,
    Struct,
    Event,
    Operator,
    TypeParameter,
}

/// Hover information
#[derive(Debug, Clone)]
pub struct HoverInfo {
    pub contents: String,
    pub range: Option<Range>,
}

/// Location in source code
#[derive(Debug, Clone)]
pub struct Location {
    pub uri: PathBuf,
    pub range: Range,
}

/// Code action information
#[derive(Debug, Clone)]
pub struct CodeAction {
    pub title: String,
    pub kind: CodeActionKind,
    pub diagnostics: Vec<Diagnostic>,
    pub edit: Option<WorkspaceEdit>,
    pub command: Option<Command>,
}

/// Types of code actions
#[derive(Debug, Clone)]
pub enum CodeActionKind {
    QuickFix,
    Refactor,
    RefactorExtract,
    RefactorInline,
    RefactorRewrite,
    Source,
    SourceOrganizeImports,
    SourceFixAll,
}

/// Workspace edit containing changes to multiple files
#[derive(Debug, Clone)]
pub struct WorkspaceEdit {
    pub changes: HashMap<PathBuf, Vec<TextEdit>>,
}

/// Command that can be executed
#[derive(Debug, Clone)]
pub struct Command {
    pub title: String,
    pub command: String,
    pub arguments: Vec<Value>,
}

/// Text edit operation
#[derive(Debug, Clone)]
pub struct TextEdit {
    pub range: Range,
    pub new_text: String,
}

/// Document symbol
#[derive(Debug, Clone)]
pub struct DocumentSymbol {
    pub name: String,
    pub detail: Option<String>,
    pub kind: SymbolKind,
    pub range: Range,
    pub selection_range: Range,
    pub children: Vec<DocumentSymbol>,
}

/// Workspace symbol
#[derive(Debug, Clone)]
pub struct WorkspaceSymbol {
    pub name: String,
    pub kind: SymbolKind,
    pub location: Location,
    pub container_name: Option<String>,
}

/// Symbol kinds
#[derive(Debug, Clone)]
pub enum SymbolKind {
    File,
    Module,
    Namespace,
    Package,
    Class,
    Method,
    Property,
    Field,
    Constructor,
    Enum,
    Interface,
    Function,
    Variable,
    Constant,
    String,
    Number,
    Boolean,
    Array,
    Object,
    Key,
    Null,
    EnumMember,
    Struct,
    Event,
    Operator,
    TypeParameter,
}

/// Signature help information
#[derive(Debug, Clone)]
pub struct SignatureHelp {
    pub signatures: Vec<SignatureInformation>,
    pub active_signature: Option<usize>,
    pub active_parameter: Option<usize>,
}

/// Information about a function signature
#[derive(Debug, Clone)]
pub struct SignatureInformation {
    pub label: String,
    pub documentation: Option<String>,
    pub parameters: Vec<ParameterInformation>,
}

/// Information about a parameter
#[derive(Debug, Clone)]
pub struct ParameterInformation {
    pub label: String,
    pub documentation: Option<String>,
}

/// Semantic token for syntax highlighting
#[derive(Debug, Clone)]
pub struct SemanticToken {
    pub line: u32,
    pub start_char: u32,
    pub length: u32,
    pub token_type: u32,
    pub token_modifiers: u32,
}

/// Folding range for code folding
#[derive(Debug, Clone)]
pub struct FoldingRange {
    pub start_line: u32,
    pub start_character: Option<u32>,
    pub end_line: u32,
    pub end_character: Option<u32>,
    pub kind: Option<FoldingRangeKind>,
}

/// Types of folding ranges
#[derive(Debug, Clone)]
pub enum FoldingRangeKind {
    Comment,
    Imports,
    Region,
}

/// Language service manager that handles multiple language services
pub struct LanguageServiceManager {
    /// Registered language services by name
    services: HashMap<String, Box<dyn LanguageService>>,
    /// File extension to service mapping
    extension_mapping: HashMap<String, String>,
    /// Request queue for async operations
    request_queue: mpsc::UnboundedSender<LanguageServiceRequest>,
    /// Request processor handle
    processor_handle: Option<tokio::task::JoinHandle<()>>,
    /// Service statistics
    stats: LanguageServiceStats,
}

/// Language service request for async processing
#[derive(Debug)]
pub struct LanguageServiceRequest {
    pub id: Uuid,
    pub service_name: String,
    pub request_type: LanguageServiceRequestType,
    pub response_sender: tokio::sync::oneshot::Sender<Result<LanguageServiceResponse, LanguageServiceError>>,
}

/// Types of language service requests
#[derive(Debug)]
pub enum LanguageServiceRequestType {
    Completion { file_path: PathBuf, position: Position },
    Hover { file_path: PathBuf, position: Position },
    GotoDefinition { file_path: PathBuf, position: Position },
    FindReferences { file_path: PathBuf, position: Position },
    Diagnostics { file_path: PathBuf },
    CodeActions { file_path: PathBuf, range: Range },
    FormatDocument { file_path: PathBuf },
    DocumentSymbols { file_path: PathBuf },
    WorkspaceSymbols { query: String },
}

/// Language service response types
#[derive(Debug)]
pub enum LanguageServiceResponse {
    Completion(Vec<CompletionItem>),
    Hover(Option<HoverInfo>),
    Locations(Vec<Location>),
    Diagnostics(Vec<Diagnostic>),
    CodeActions(Vec<CodeAction>),
    TextEdits(Vec<TextEdit>),
    DocumentSymbols(Vec<DocumentSymbol>),
    WorkspaceSymbols(Vec<WorkspaceSymbol>),
}

/// Statistics for language service performance
#[derive(Debug, Default, Clone)]
pub struct LanguageServiceStats {
    pub total_requests: usize,
    pub successful_requests: usize,
    pub failed_requests: usize,
    pub average_response_time_ms: f64,
    pub active_documents: usize,
    pub services_count: usize,
}

impl LanguageServiceManager {
    /// Create a new language service manager
    pub fn new() -> Self {
        let (request_queue, mut request_receiver) = mpsc::unbounded_channel();
        
        // Spawn request processor task
        let processor_handle = tokio::spawn(async move {
            while let Some(request) = request_receiver.recv().await {
                // Process request asynchronously
                // TODO: Implement request processing
            }
        });
        
        Self {
            services: HashMap::new(),
            extension_mapping: HashMap::new(),
            request_queue,
            processor_handle: Some(processor_handle),
            stats: LanguageServiceStats::default(),
        }
    }
    
    /// Register a language service
    pub fn register_service<S>(&mut self, name: String, service: S, extensions: Vec<String>) 
    where 
        S: LanguageService + 'static,
    {
        // Map extensions to service
        for ext in extensions {
            self.extension_mapping.insert(ext, name.clone());
        }
        
        // Store service
        self.services.insert(name, Box::new(service));
        self.stats.services_count += 1;
        
        // Emit event
        global_event_bus().publish(IdeEvent::LanguageServiceRegistered {
            service_name: name,
        });
    }
    
    /// Get language service for a file
    pub fn get_service_for_file(&self, file_path: &Path) -> Option<&str> {
        file_path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(|ext| self.extension_mapping.get(ext))
            .map(|s| s.as_str())
    }
    
    /// Initialize all services
    pub async fn initialize_all(&mut self, workspace_root: &Path) -> Result<(), LanguageServiceError> {
        let mut errors = Vec::new();
        
        for (name, service) in &mut self.services {
            if let Err(e) = service.initialize(workspace_root).await {
                errors.push(format!("Failed to initialize {}: {}", name, e));
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(LanguageServiceError::Other(errors.join("; ")))
        }
    }
    
    /// Shutdown all services
    pub async fn shutdown_all(&mut self) -> Result<(), LanguageServiceError> {
        let mut errors = Vec::new();
        
        for (name, service) in &mut self.services {
            if let Err(e) = service.shutdown().await {
                errors.push(format!("Failed to shutdown {}: {}", name, e));
            }
        }
        
        // Cancel processor task
        if let Some(handle) = self.processor_handle.take() {
            handle.abort();
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(LanguageServiceError::Other(errors.join("; ")))
        }
    }
    
    /// Request completion for a file
    pub async fn completion(&mut self, file_path: &Path, position: Position) -> Result<Vec<CompletionItem>, LanguageServiceError> {
        let service_name = self.get_service_for_file(file_path)
            .ok_or_else(|| LanguageServiceError::FileNotSupported(file_path.to_path_buf()))?;
            
        if let Some(service) = self.services.get_mut(service_name) {
            let result = service.completion(file_path, position).await;
            
            // Update stats
            self.stats.total_requests += 1;
            if result.is_ok() {
                self.stats.successful_requests += 1;
            } else {
                self.stats.failed_requests += 1;
            }
            
            result
        } else {
            Err(LanguageServiceError::Other(format!("Service {} not found", service_name)))
        }
    }
    
    /// Get statistics
    pub fn stats(&self) -> &LanguageServiceStats {
        &self.stats
    }
    
    /// Get list of registered services
    pub fn list_services(&self) -> Vec<&String> {
        self.services.keys().collect()
    }
}

impl Default for LanguageServiceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    
    struct MockLanguageService {
        initialized: bool,
    }
    
    #[async_trait::async_trait]
    impl LanguageService for MockLanguageService {
        async fn initialize(&mut self, _workspace_root: &Path) -> Result<(), LanguageServiceError> {
            self.initialized = true;
            Ok(())
        }
        
        async fn shutdown(&mut self) -> Result<(), LanguageServiceError> {
            self.initialized = false;
            Ok(())
        }
        
        fn supports_file(&self, _file_path: &Path) -> bool {
            true
        }
        
        fn supported_extensions(&self) -> Vec<&'static str> {
            vec!["rs", "toml"]
        }
        
        async fn open_document(&mut self, _file_path: &Path, _content: &str) -> Result<(), LanguageServiceError> {
            Ok(())
        }
        
        async fn update_document(&mut self, _file_path: &Path, _content: &str, _version: u64) -> Result<(), LanguageServiceError> {
            Ok(())
        }
        
        async fn close_document(&mut self, _file_path: &Path) -> Result<(), LanguageServiceError> {
            Ok(())
        }
        
        async fn completion(&mut self, _file_path: &Path, _position: Position) -> Result<Vec<CompletionItem>, LanguageServiceError> {
            Ok(vec![CompletionItem {
                label: "test_completion".to_string(),
                kind: CompletionKind::Function,
                detail: Some("Mock completion".to_string()),
                documentation: None,
                insert_text: None,
                filter_text: None,
                sort_text: None,
                additional_text_edits: Vec::new(),
            }])
        }
        
        async fn hover(&mut self, _file_path: &Path, _position: Position) -> Result<Option<HoverInfo>, LanguageServiceError> {
            Ok(None)
        }
        
        async fn goto_definition(&mut self, _file_path: &Path, _position: Position) -> Result<Vec<Location>, LanguageServiceError> {
            Ok(Vec::new())
        }
        
        async fn find_references(&mut self, _file_path: &Path, _position: Position) -> Result<Vec<Location>, LanguageServiceError> {
            Ok(Vec::new())
        }
        
        async fn diagnostics(&mut self, _file_path: &Path) -> Result<Vec<Diagnostic>, LanguageServiceError> {
            Ok(Vec::new())
        }
        
        async fn code_actions(&mut self, _file_path: &Path, _range: Range) -> Result<Vec<CodeAction>, LanguageServiceError> {
            Ok(Vec::new())
        }
        
        async fn format_document(&mut self, _file_path: &Path) -> Result<Vec<TextEdit>, LanguageServiceError> {
            Ok(Vec::new())
        }
        
        async fn format_range(&mut self, _file_path: &Path, _range: Range) -> Result<Vec<TextEdit>, LanguageServiceError> {
            Ok(Vec::new())
        }
        
        async fn rename(&mut self, _file_path: &Path, _position: Position, _new_name: &str) -> Result<HashMap<PathBuf, Vec<TextEdit>>, LanguageServiceError> {
            Ok(HashMap::new())
        }
        
        async fn document_symbols(&mut self, _file_path: &Path) -> Result<Vec<DocumentSymbol>, LanguageServiceError> {
            Ok(Vec::new())
        }
        
        async fn workspace_symbols(&mut self, _query: &str) -> Result<Vec<WorkspaceSymbol>, LanguageServiceError> {
            Ok(Vec::new())
        }
        
        async fn signature_help(&mut self, _file_path: &Path, _position: Position) -> Result<Option<SignatureHelp>, LanguageServiceError> {
            Ok(None)
        }
        
        async fn semantic_tokens(&mut self, _file_path: &Path) -> Result<Vec<SemanticToken>, LanguageServiceError> {
            Ok(Vec::new())
        }
        
        async fn folding_ranges(&mut self, _file_path: &Path) -> Result<Vec<FoldingRange>, LanguageServiceError> {
            Ok(Vec::new())
        }
    }
    
    #[tokio::test]
    async fn test_service_registration() {
        let mut manager = LanguageServiceManager::new();
        let service = MockLanguageService { initialized: false };
        
        manager.register_service(
            "rust".to_string(),
            service,
            vec!["rs".to_string(), "toml".to_string()],
        );
        
        assert_eq!(manager.list_services().len(), 1);
        assert_eq!(manager.get_service_for_file(Path::new("test.rs")), Some("rust"));
    }
    
    #[tokio::test]
    async fn test_service_initialization() {
        let mut manager = LanguageServiceManager::new();
        let service = MockLanguageService { initialized: false };
        
        manager.register_service(
            "rust".to_string(),
            service,
            vec!["rs".to_string()],
        );
        
        let result = manager.initialize_all(Path::new(".")).await;
        assert!(result.is_ok());
    }
}