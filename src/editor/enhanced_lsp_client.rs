/// Enhanced LSP Client with VS Code-style features
/// 
/// This module extends the basic LSP client with advanced features like:
/// - Go-to-definition and find references
/// - Code actions and quick fixes
/// - Enhanced diagnostics with error squiggles
/// - Symbol navigation and workspace symbols
/// - Signature help and parameter hints

use std::collections::HashMap;
use serde_json::{json, Value};
use egui::*;

use crate::editor::lsp_integration::{LspClient, LspError, Position};

/// Enhanced LSP features and capabilities
pub struct EnhancedLspClient {
    /// Base LSP client for communication
    pub base_client: LspClient,
    /// Pending go-to-definition requests
    pub goto_definition_requests: HashMap<u64, Box<dyn FnOnce(Result<Vec<Location>, LspError>) + Send>>,
    /// Pending find references requests
    pub find_references_requests: HashMap<u64, Box<dyn FnOnce(Result<Vec<Location>, LspError>) + Send>>,
    /// Pending code action requests
    pub code_action_requests: HashMap<u64, Box<dyn FnOnce(Result<Vec<CodeAction>, LspError>) + Send>>,
    /// Pending signature help requests
    pub signature_help_requests: HashMap<u64, Box<dyn FnOnce(Result<SignatureHelp, LspError>) + Send>>,
    /// Cached document symbols for navigation
    pub document_symbols: HashMap<String, Vec<DocumentSymbol>>,
    /// Workspace symbols cache
    pub workspace_symbols: Vec<WorkspaceSymbol>,
    /// Current signature help (for parameter hints)
    pub current_signature_help: Option<SignatureHelp>,
    /// Request ID counter
    pub next_request_id: u64,
}

/// LSP Location for navigation
#[derive(Debug, Clone)]
pub struct Location {
    pub uri: String,
    pub range: Range,
}

/// LSP Range for text positions
#[derive(Debug, Clone)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}


/// Code action for quick fixes and refactoring
#[derive(Debug, Clone)]
pub struct CodeAction {
    pub title: String,
    pub kind: Option<String>,
    pub diagnostics: Option<Vec<Diagnostic>>,
    pub edit: Option<WorkspaceEdit>,
    pub command: Option<Command>,
}

/// Workspace edit for applying changes
#[derive(Debug, Clone)]
pub struct WorkspaceEdit {
    pub changes: HashMap<String, Vec<TextEdit>>,
}

/// Text edit for document modifications
#[derive(Debug, Clone)]
pub struct TextEdit {
    pub range: Range,
    pub new_text: String,
}

/// LSP Command for actions
#[derive(Debug, Clone)]
pub struct Command {
    pub title: String,
    pub command: String,
    pub arguments: Option<Vec<Value>>,
}

/// Signature help for parameter hints
#[derive(Debug, Clone)]
pub struct SignatureHelp {
    pub signatures: Vec<SignatureInformation>,
    pub active_signature: Option<u32>,
    pub active_parameter: Option<u32>,
}

/// Signature information for functions
#[derive(Debug, Clone)]
pub struct SignatureInformation {
    pub label: String,
    pub documentation: Option<String>,
    pub parameters: Option<Vec<ParameterInformation>>,
}

/// Parameter information for function parameters
#[derive(Debug, Clone)]
pub struct ParameterInformation {
    pub label: String,
    pub documentation: Option<String>,
}

/// Document symbol for navigation
#[derive(Debug, Clone)]
pub struct DocumentSymbol {
    pub name: String,
    pub detail: Option<String>,
    pub kind: SymbolKind,
    pub range: Range,
    pub selection_range: Range,
    pub children: Option<Vec<DocumentSymbol>>,
}

/// Workspace symbol for project-wide search
#[derive(Debug, Clone)]
pub struct WorkspaceSymbol {
    pub name: String,
    pub kind: SymbolKind,
    pub location: Location,
}

/// Symbol kinds for different code elements
#[derive(Debug, Clone)]
pub enum SymbolKind {
    File = 1,
    Module = 2,
    Namespace = 3,
    Package = 4,
    Class = 5,
    Method = 6,
    Property = 7,
    Field = 8,
    Constructor = 9,
    Enum = 10,
    Interface = 11,
    Function = 12,
    Variable = 13,
    Constant = 14,
    String = 15,
    Number = 16,
    Boolean = 17,
    Array = 18,
    Object = 19,
    Key = 20,
    Null = 21,
    EnumMember = 22,
    Struct = 23,
    Event = 24,
    Operator = 25,
    TypeParameter = 26,
}

/// Diagnostic for error reporting
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub range: Range,
    pub severity: Option<DiagnosticSeverity>,
    pub code: Option<String>,
    pub source: Option<String>,
    pub message: String,
    pub related_information: Option<Vec<DiagnosticRelatedInformation>>,
}

/// Diagnostic severity levels
#[derive(Debug, Clone, Copy)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

/// Related diagnostic information
#[derive(Debug, Clone)]
pub struct DiagnosticRelatedInformation {
    pub location: Location,
    pub message: String,
}

impl Default for EnhancedLspClient {
    fn default() -> Self {
        Self::new()
    }
}

impl EnhancedLspClient {
    /// Create a new enhanced LSP client
    pub fn new() -> Self {
        Self {
            base_client: LspClient::new(),
            goto_definition_requests: HashMap::new(),
            find_references_requests: HashMap::new(),
            code_action_requests: HashMap::new(),
            signature_help_requests: HashMap::new(),
            document_symbols: HashMap::new(),
            workspace_symbols: Vec::new(),
            current_signature_help: None,
            next_request_id: 1,
        }
    }

    /// Start the LSP server
    pub fn start(&mut self) -> Result<(), LspError> {
        // Try to start rust-analyzer
        self.base_client.start_rust_analyzer().map_err(|e| LspError {
            code: -1,
            message: format!("Failed to start LSP: {}", e),
            data: None,
        })
    }

    /// Stop the LSP server
    pub fn stop(&mut self) {
        self.base_client.stop();
    }

    /// Check if LSP is connected
    pub fn is_connected(&self) -> bool {
        self.base_client.is_connected()
    }

    /// Send go-to-definition request
    pub fn goto_definition<F>(&mut self, uri: &str, position: Position, callback: F) -> Result<(), LspError>
    where
        F: FnOnce(Result<Vec<Location>, LspError>) + Send + 'static,
    {
        let request_id = self.next_request_id;
        self.next_request_id += 1;

        let _params = json!({
            "textDocument": {
                "uri": uri
            },
            "position": {
                "line": position.line,
                "character": position.character
            }
        });

        // For now, store the callback and handle it later
        // This avoids the complex ownership issues with closures
        self.goto_definition_requests.insert(request_id, Box::new(callback));
        
        // TODO: Implement actual LSP communication
        // For now, return empty result to avoid compilation errors

        Ok(())
    }
    
    /// Parse LSP goto definition response into our Location format
    fn parse_goto_definition_response(value: Value) -> Vec<Location> {
        let mut locations = Vec::new();
        
        if let Some(array) = value.as_array() {
            for item in array {
                if let Some(location) = Self::parse_lsp_location(item) {
                    locations.push(location);
                }
            }
        } else if let Some(location) = Self::parse_lsp_location(&value) {
            locations.push(location);
        }
        
        locations
    }
    
    /// Parse a single LSP location object
    fn parse_lsp_location(value: &Value) -> Option<Location> {
        let uri = value.get("uri")?.as_str()?.to_string();
        let range_obj = value.get("range")?;
        
        let start_obj = range_obj.get("start")?;
        let end_obj = range_obj.get("end")?;
        
        let start = Position {
            line: start_obj.get("line")?.as_u64()?,
            character: start_obj.get("character")?.as_u64()?,
        };
        
        let end = Position {
            line: end_obj.get("line")?.as_u64()?,
            character: end_obj.get("character")?.as_u64()?,
        };
        
        Some(Location {
            uri,
            range: Range { start, end },
        })
    }

    /// Send find references request
    pub fn find_references<F>(&mut self, uri: &str, position: Position, include_declaration: bool, callback: F) -> Result<(), LspError>
    where
        F: FnOnce(Result<Vec<Location>, LspError>) + Send + 'static,
    {
        let request_id = self.next_request_id;
        self.next_request_id += 1;

        let _params = json!({
            "textDocument": {
                "uri": uri
            },
            "position": {
                "line": position.line,
                "character": position.character
            },
            "context": {
                "includeDeclaration": include_declaration
            }
        });

        // For now, store the callback and handle it later
        self.find_references_requests.insert(request_id, Box::new(callback));
        
        // TODO: Implement actual LSP communication

        Ok(())
    }

    /// Send code action request
    pub fn code_action<F>(&mut self, uri: &str, range: Range, diagnostics: Vec<Diagnostic>, callback: F) -> Result<(), LspError>
    where
        F: FnOnce(Result<Vec<CodeAction>, LspError>) + Send + 'static,
    {
        let request_id = self.next_request_id;
        self.next_request_id += 1;

        let _params = json!({
            "textDocument": {
                "uri": uri
            },
            "range": {
                "start": {
                    "line": range.start.line,
                    "character": range.start.character
                },
                "end": {
                    "line": range.end.line,
                    "character": range.end.character
                }
            },
            "context": {
                "diagnostics": diagnostics.iter().map(|d| {
                    json!({
                        "range": {
                            "start": {
                                "line": d.range.start.line,
                                "character": d.range.start.character
                            },
                            "end": {
                                "line": d.range.end.line,
                                "character": d.range.end.character
                            }
                        },
                        "severity": d.severity.as_ref().map(|s| *s as u8),
                        "message": d.message
                    })
                }).collect::<Vec<_>>()
            }
        });

        // For now, store the callback and handle it later
        self.code_action_requests.insert(request_id, Box::new(callback));
        
        // TODO: Implement actual LSP communication

        Ok(())
    }
    
    /// Parse LSP code actions response
    fn parse_code_actions_response(value: Value) -> Vec<CodeAction> {
        let mut actions = Vec::new();
        
        if let Some(array) = value.as_array() {
            for item in array {
                if let Some(action) = Self::parse_code_action(item) {
                    actions.push(action);
                }
            }
        }
        
        actions
    }
    
    /// Parse a single code action object
    fn parse_code_action(value: &Value) -> Option<CodeAction> {
        let title = value.get("title")?.as_str()?.to_string();
        let kind = value.get("kind").and_then(|k| k.as_str()).map(|s| s.to_string());
        
        // For now, create a simple code action
        // In a full implementation, we'd parse the edit details
        Some(CodeAction {
            title,
            kind,
            diagnostics: None,
            edit: None, // TODO: Parse workspace edit
            command: None, // TODO: Parse command
        })
    }

    /// Send signature help request
    pub fn signature_help<F>(&mut self, uri: &str, position: Position, callback: F) -> Result<(), LspError>
    where
        F: FnOnce(Result<SignatureHelp, LspError>) + Send + 'static,
    {
        let request_id = self.next_request_id;
        self.next_request_id += 1;

        let _params = json!({
            "textDocument": {
                "uri": uri
            },
            "position": {
                "line": position.line,
                "character": position.character
            }
        });

        // For now, store the callback and handle it later
        self.signature_help_requests.insert(request_id, Box::new(callback));
        
        // TODO: Implement actual LSP communication

        Ok(())
    }
    
    /// Parse LSP signature help response
    fn parse_signature_help_response(value: Value) -> SignatureHelp {
        let signatures = value.get("signatures")
            .and_then(|s| s.as_array())
            .map(|arr| {
                arr.iter().filter_map(|sig| {
                    let label = sig.get("label")?.as_str()?.to_string();
                    let documentation = sig.get("documentation")
                        .and_then(|d| d.as_str())
                        .map(|s| s.to_string());
                    
                    Some(SignatureInformation {
                        label,
                        documentation,
                        parameters: Some(Vec::new()), // TODO: Parse parameters
                    })
                }).collect()
            })
            .unwrap_or_default();
            
        let active_signature = value.get("activeSignature")
            .and_then(|s| s.as_u64())
            .map(|n| n as u32);
            
        let active_parameter = value.get("activeParameter")
            .and_then(|p| p.as_u64())
            .map(|n| n as u32);
            
        SignatureHelp {
            signatures,
            active_signature,
            active_parameter,
        }
    }

    /// Request document symbols for navigation
    pub fn document_symbols(&mut self, uri: &str) -> Result<(), LspError> {
        let _params = json!({
            "textDocument": {
                "uri": uri
            }
        });

        // For now, use a simplified approach without actual LSP communication
        // This can be expanded when the full LSP integration is implemented

        Ok(())
    }

    /// Request workspace symbols for project-wide search
    pub fn workspace_symbols(&mut self, query: &str) -> Result<(), LspError> {
        let _params = json!({
            "query": query
        });

        // For now, use a simplified approach without actual LSP communication
        // This can be expanded when the full LSP integration is implemented

        Ok(())
    }

    /// Process pending LSP responses
    pub fn process_responses(&mut self) {
        let _ = self.base_client.process_messages();
        
        // Process any completed requests and call their callbacks
        // This would need integration with the base client's response handling
    }

    /// Get current signature help for UI display
    pub fn get_current_signature_help(&self) -> Option<&SignatureHelp> {
        self.current_signature_help.as_ref()
    }

    /// Get document symbols for a file
    pub fn get_document_symbols(&self, uri: &str) -> Option<&Vec<DocumentSymbol>> {
        self.document_symbols.get(uri)
    }

    /// Get workspace symbols for search
    pub fn get_workspace_symbols(&self) -> &Vec<WorkspaceSymbol> {
        &self.workspace_symbols
    }

    /// Render signature help popup
    pub fn render_signature_help(&self, ui: &mut Ui, cursor_pos: Pos2) {
        if let Some(signature_help) = &self.current_signature_help {
            if !signature_help.signatures.is_empty() {
                let active_sig_idx = signature_help.active_signature.unwrap_or(0) as usize;
                if let Some(signature) = signature_help.signatures.get(active_sig_idx) {
                    
                    // Show signature help popup above cursor
                    Area::new("signature_help".into())
                        .fixed_pos(cursor_pos + Vec2::new(0.0, -40.0))
                        .order(Order::Foreground)
                        .show(ui.ctx(), |ui| {
                            Frame::popup(ui.style())
                                .inner_margin(Margin::same(8.0))
                                .show(ui, |ui| {
                                    ui.set_max_width(400.0);
                                    
                                    // Function signature
                                    ui.label(RichText::new(&signature.label).strong());
                                    
                                    // Documentation if available
                                    if let Some(doc) = &signature.documentation {
                                        ui.separator();
                                        ui.label(doc);
                                    }
                                    
                                    // Parameter information
                                    if let Some(params) = &signature.parameters {
                                        let active_param_idx = signature_help.active_parameter.unwrap_or(0) as usize;
                                        if let Some(active_param) = params.get(active_param_idx) {
                                            ui.separator();
                                            ui.label(RichText::new(format!("Parameter: {}", active_param.label)).italics());
                                            if let Some(param_doc) = &active_param.documentation {
                                                ui.label(param_doc);
                                            }
                                        }
                                    }
                                });
                        });
                }
            }
        }
    }

    /// Get icon for symbol kind
    pub fn get_symbol_icon(kind: &SymbolKind) -> &'static str {
        match kind {
            SymbolKind::File => "📄",
            SymbolKind::Module => "📦",
            SymbolKind::Namespace => "📁",
            SymbolKind::Package => "📦",
            SymbolKind::Class => "🏛",
            SymbolKind::Method => "🔧",
            SymbolKind::Property => "🔗",
            SymbolKind::Field => "🏷",
            SymbolKind::Constructor => "🏗",
            SymbolKind::Enum => "📋",
            SymbolKind::Interface => "🔌",
            SymbolKind::Function => "⚙",
            SymbolKind::Variable => "📊",
            SymbolKind::Constant => "🔒",
            SymbolKind::String => "💬",
            SymbolKind::Number => "🔢",
            SymbolKind::Boolean => "✓",
            SymbolKind::Array => "📚",
            SymbolKind::Object => "📦",
            SymbolKind::Key => "🔑",
            SymbolKind::Null => "⊘",
            SymbolKind::EnumMember => "🏷",
            SymbolKind::Struct => "🏗",
            SymbolKind::Event => "⚡",
            SymbolKind::Operator => "➕",
            SymbolKind::TypeParameter => "🎭",
        }
    }
}

/// Helper functions for LSP integration
impl EnhancedLspClient {
    /// Convert egui position to LSP position
    pub fn egui_to_lsp_position(line: usize, column: usize) -> Position {
        Position {
            line: line as u64,
            character: column as u64,
        }
    }

    /// Convert LSP position to egui position
    pub fn lsp_to_egui_position(position: &Position) -> (usize, usize) {
        (position.line as usize, position.character as usize)
    }

    /// Convert LSP range to egui range
    pub fn lsp_to_egui_range(range: &Range) -> ((usize, usize), (usize, usize)) {
        (
            Self::lsp_to_egui_position(&range.start),
            Self::lsp_to_egui_position(&range.end),
        )
    }

    /// Get diagnostic color for UI display
    pub fn get_diagnostic_color(severity: &DiagnosticSeverity) -> Color32 {
        match severity {
            DiagnosticSeverity::Error => Color32::RED,
            DiagnosticSeverity::Warning => Color32::YELLOW,
            DiagnosticSeverity::Information => Color32::LIGHT_BLUE,
            DiagnosticSeverity::Hint => Color32::GRAY,
        }
    }

    /// Format diagnostic message for display
    pub fn format_diagnostic_message(diagnostic: &Diagnostic) -> String {
        let mut message = diagnostic.message.clone();
        
        if let Some(source) = &diagnostic.source {
            message = format!("[{}] {}", source, message);
        }
        
        if let Some(code) = &diagnostic.code {
            message = format!("{} ({})", message, code);
        }
        
        message
    }
}