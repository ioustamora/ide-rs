//! Language Server Protocol (LSP) Integration
//! 
//! This module provides integration with rust-analyzer and other language servers
//! to enable advanced IDE features like:
//! - Intelligent code completion
//! - Error detection and diagnostics
//! - Go to definition and find references
//! - Code actions and quick fixes
//! - Symbol navigation and workspace symbols

use serde_json::{json, Value};
use std::collections::HashMap;
use std::process::{Child, Command, Stdio};
use std::io::{BufRead, BufReader, Write};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

/// LSP client for communicating with language servers
pub struct LspClient {
    /// Language server process
    pub server_process: Option<Child>,
    /// Request counter for LSP message IDs
    pub request_id: u64,
    /// Pending requests waiting for responses
    pub pending_requests: HashMap<u64, PendingRequest>,
    /// Diagnostics cache
    pub diagnostics: HashMap<String, Vec<Diagnostic>>,
    /// Document versions for synchronization
    pub document_versions: HashMap<String, u64>,
    /// LSP capabilities
    pub server_capabilities: Option<ServerCapabilities>,
    /// Message sender/receiver
    pub message_tx: Option<Sender<LspMessage>>,
    pub message_rx: Option<Receiver<LspMessage>>,
}

/// LSP message types
#[derive(Clone, Debug)]
pub enum LspMessage {
    Request {
        id: u64,
        method: String,
        params: Value,
    },
    Response {
        id: u64,
        result: Option<Value>,
        error: Option<LspError>,
    },
    Notification {
        method: String,
        params: Value,
    },
}

/// LSP error information
#[derive(Clone, Debug)]
pub struct LspError {
    pub code: i32,
    pub message: String,
    pub data: Option<Value>,
}

/// Pending LSP request
pub struct PendingRequest {
    pub method: String,
    pub callback: Box<dyn FnOnce(Result<Value, LspError>) + Send>,
}

/// LSP diagnostic information
#[derive(Clone, Debug)]
pub struct Diagnostic {
    pub range: Range,
    pub severity: Option<DiagnosticSeverity>,
    pub code: Option<String>,
    pub source: Option<String>,
    pub message: String,
    pub related_information: Option<Vec<DiagnosticRelatedInformation>>,
}

/// LSP position and range
#[derive(Clone, Debug, serde::Serialize)]
pub struct Position {
    pub line: u64,
    pub character: u64,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

/// Diagnostic severity levels
#[derive(Clone, Debug)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

/// Related diagnostic information
#[derive(Clone, Debug)]
pub struct DiagnosticRelatedInformation {
    pub location: Location,
    pub message: String,
}

/// LSP location
#[derive(Clone, Debug)]
pub struct Location {
    pub uri: String,
    pub range: Range,
}

/// Server capabilities
#[derive(Clone, Debug)]
pub struct ServerCapabilities {
    pub text_document_sync: Option<TextDocumentSyncCapability>,
    pub completion_provider: Option<CompletionOptions>,
    pub hover_provider: Option<bool>,
    pub signature_help_provider: Option<SignatureHelpOptions>,
    pub definition_provider: Option<bool>,
    pub references_provider: Option<bool>,
    pub document_highlight_provider: Option<bool>,
    pub document_symbol_provider: Option<bool>,
    pub workspace_symbol_provider: Option<bool>,
    pub code_action_provider: Option<bool>,
    pub code_lens_provider: Option<CodeLensOptions>,
    pub document_formatting_provider: Option<bool>,
    pub document_range_formatting_provider: Option<bool>,
    pub rename_provider: Option<bool>,
}

/// Text document synchronization options
#[derive(Clone, Debug)]
pub struct TextDocumentSyncCapability {
    pub open_close: Option<bool>,
    pub change: Option<TextDocumentSyncKind>,
}

/// Text document sync kinds
#[derive(Clone, Debug)]
pub enum TextDocumentSyncKind {
    None = 0,
    Full = 1,
    Incremental = 2,
}

/// Completion options
#[derive(Clone, Debug)]
pub struct CompletionOptions {
    pub resolve_provider: Option<bool>,
    pub trigger_characters: Option<Vec<String>>,
}

/// Signature help options
#[derive(Clone, Debug)]
pub struct SignatureHelpOptions {
    pub trigger_characters: Option<Vec<String>>,
}

/// Code lens options
#[derive(Clone, Debug)]
pub struct CodeLensOptions {
    pub resolve_provider: Option<bool>,
}

/// Completion item
#[derive(Clone, Debug)]
pub struct CompletionItem {
    pub label: String,
    pub kind: Option<CompletionItemKind>,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub sort_text: Option<String>,
    pub filter_text: Option<String>,
    pub insert_text: Option<String>,
    pub insert_text_format: Option<InsertTextFormat>,
}

/// Completion item kinds
#[derive(Clone, Debug)]
pub enum CompletionItemKind {
    Text = 1,
    Method = 2,
    Function = 3,
    Constructor = 4,
    Field = 5,
    Variable = 6,
    Class = 7,
    Interface = 8,
    Module = 9,
    Property = 10,
    Unit = 11,
    Value = 12,
    Enum = 13,
    Keyword = 14,
    Snippet = 15,
    Color = 16,
    File = 17,
    Reference = 18,
    Folder = 19,
    EnumMember = 20,
    Constant = 21,
    Struct = 22,
    Event = 23,
    Operator = 24,
    TypeParameter = 25,
}

/// Insert text format
#[derive(Clone, Debug)]
pub enum InsertTextFormat {
    PlainText = 1,
    Snippet = 2,
}

/// Hover information
#[derive(Clone, Debug)]
pub struct Hover {
    pub contents: HoverContents,
    pub range: Option<Range>,
}

/// Hover contents
#[derive(Clone, Debug)]
pub enum HoverContents {
    String(String),
    Array(Vec<String>),
    MarkupContent { kind: String, value: String },
}

impl LspClient {
    /// Create a new LSP client
    pub fn new() -> Self {
        Self {
            server_process: None,
            request_id: 0,
            pending_requests: HashMap::new(),
            diagnostics: HashMap::new(),
            document_versions: HashMap::new(),
            server_capabilities: None,
            message_tx: None,
            message_rx: None,
        }
    }

    /// Start rust-analyzer language server
    pub fn start_rust_analyzer(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut child = Command::new("rust-analyzer")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let (tx, rx) = std::sync::mpsc::channel();
        self.message_rx = Some(rx);

        // Start message handling thread
        if let Some(stdout) = child.stdout.take() {
            let tx_clone = tx.clone();
            thread::spawn(move || {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        if let Ok(message) = Self::parse_lsp_message(&line) {
                            let _ = tx_clone.send(message);
                        }
                    }
                }
            });
        }

        self.server_process = Some(child);
        self.message_tx = Some(tx);

        // Send initialize request
        self.initialize()?;

        Ok(())
    }

    /// Initialize the language server
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let params = json!({
            "processId": std::process::id(),
            "clientInfo": {
                "name": "Rust RAD IDE",
                "version": "0.1.0"
            },
            "capabilities": {
                "textDocument": {
                    "synchronization": {
                        "dynamicRegistration": false,
                        "willSave": false,
                        "willSaveWaitUntil": false,
                        "didSave": false
                    },
                    "completion": {
                        "dynamicRegistration": false,
                        "completionItem": {
                            "snippetSupport": true,
                            "commitCharactersSupport": false,
                            "documentationFormat": ["markdown", "plaintext"]
                        }
                    },
                    "hover": {
                        "dynamicRegistration": false,
                        "contentFormat": ["markdown", "plaintext"]
                    },
                    "definition": {
                        "dynamicRegistration": false
                    },
                    "references": {
                        "dynamicRegistration": false
                    }
                }
            },
            "workspaceFolders": null
        });

        self.send_request("initialize", params, Box::new(|result| {
            match result {
                Ok(_response) => {
                    println!("LSP initialized successfully");
                    // Parse server capabilities here
                }
                Err(error) => {
                    eprintln!("LSP initialization failed: {:?}", error);
                }
            }
        }))?;

        Ok(())
    }

    /// Send LSP request
    fn send_request<F>(&mut self, method: &str, params: Value, callback: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(Result<Value, LspError>) + Send + 'static,
    {
        let id = self.next_request_id();
        let message = LspMessage::Request {
            id,
            method: method.to_string(),
            params,
        };

        self.pending_requests.insert(id, PendingRequest {
            method: method.to_string(),
            callback: Box::new(callback),
        });

        self.send_message(message)?;
        Ok(())
    }

    /// Send LSP notification
    fn send_notification(&mut self, method: &str, params: Value) -> Result<(), Box<dyn std::error::Error>> {
        let message = LspMessage::Notification {
            method: method.to_string(),
            params,
        };
        self.send_message(message)?;
        Ok(())
    }

    /// Send message to language server
    fn send_message(&mut self, message: LspMessage) -> Result<(), Box<dyn std::error::Error>> {
        let json_message = self.serialize_message(message)?;
        let content_length = json_message.len();
        let lsp_message = format!("Content-Length: {}\r\n\r\n{}", content_length, json_message);
        
        if let Some(ref mut process) = self.server_process {
            if let Some(ref mut stdin) = process.stdin.as_mut() {
                stdin.write_all(lsp_message.as_bytes())?;
                stdin.flush()?;
            }
        }
        Ok(())
    }

    /// Serialize LSP message to JSON
    fn serialize_message(&self, message: LspMessage) -> Result<String, Box<dyn std::error::Error>> {
        let json_value = match message {
            LspMessage::Request { id, method, params } => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "method": method,
                    "params": params
                })
            }
            LspMessage::Response { id, result, error } => {
                let mut response = json!({
                    "jsonrpc": "2.0",
                    "id": id
                });
                if let Some(result) = result {
                    response["result"] = result;
                }
                if let Some(error) = error {
                    response["error"] = json!({
                        "code": error.code,
                        "message": error.message,
                        "data": error.data
                    });
                }
                response
            }
            LspMessage::Notification { method, params } => {
                json!({
                    "jsonrpc": "2.0",
                    "method": method,
                    "params": params
                })
            }
        };

        Ok(serde_json::to_string(&json_value)?)
    }

    /// Parse LSP message from JSON
    fn parse_lsp_message(content: &str) -> Result<LspMessage, Box<dyn std::error::Error>> {
        let json: Value = serde_json::from_str(content)?;
        
        if let Some(id) = json.get("id") {
            if json.get("method").is_some() {
                // Request
                Ok(LspMessage::Request {
                    id: id.as_u64().unwrap_or(0),
                    method: json["method"].as_str().unwrap_or("").to_string(),
                    params: json.get("params").cloned().unwrap_or(Value::Null),
                })
            } else {
                // Response
                let error = if let Some(error_obj) = json.get("error") {
                    Some(LspError {
                        code: error_obj["code"].as_i64().unwrap_or(0) as i32,
                        message: error_obj["message"].as_str().unwrap_or("").to_string(),
                        data: error_obj.get("data").cloned(),
                    })
                } else {
                    None
                };

                Ok(LspMessage::Response {
                    id: id.as_u64().unwrap_or(0),
                    result: json.get("result").cloned(),
                    error,
                })
            }
        } else {
            // Notification
            Ok(LspMessage::Notification {
                method: json["method"].as_str().unwrap_or("").to_string(),
                params: json.get("params").cloned().unwrap_or(Value::Null),
            })
        }
    }

    /// Get next request ID
    fn next_request_id(&mut self) -> u64 {
        self.request_id += 1;
        self.request_id
    }

    /// Open document
    pub fn did_open(&mut self, uri: &str, language_id: &str, version: u64, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        let params = json!({
            "textDocument": {
                "uri": uri,
                "languageId": language_id,
                "version": version,
                "text": text
            }
        });

        self.document_versions.insert(uri.to_string(), version);
        self.send_notification("textDocument/didOpen", params)?;
        Ok(())
    }

    /// Document changed
    pub fn did_change(&mut self, uri: &str, version: u64, changes: Vec<TextDocumentContentChangeEvent>) -> Result<(), Box<dyn std::error::Error>> {
        let params = json!({
            "textDocument": {
                "uri": uri,
                "version": version
            },
            "contentChanges": changes
        });

        self.document_versions.insert(uri.to_string(), version);
        self.send_notification("textDocument/didChange", params)?;
        Ok(())
    }

    /// Request completion
    pub fn completion<F>(&mut self, uri: &str, line: u64, character: u64, callback: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(Result<Vec<CompletionItem>, LspError>) + Send + 'static,
    {
        let params = json!({
            "textDocument": {
                "uri": uri
            },
            "position": {
                "line": line,
                "character": character
            }
        });

        self.send_request("textDocument/completion", params, Box::new(move |result| {
            match result {
                Ok(response) => {
                    let items = Self::parse_completion_items(&response);
                    callback(Ok(items));
                }
                Err(error) => {
                    callback(Err(error));
                }
            }
        }))?;

        Ok(())
    }

    /// Request hover information
    pub fn hover<F>(&mut self, uri: &str, line: u64, character: u64, callback: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(Result<Option<Hover>, LspError>) + Send + 'static,
    {
        let params = json!({
            "textDocument": {
                "uri": uri
            },
            "position": {
                "line": line,
                "character": character
            }
        });

        self.send_request("textDocument/hover", params, Box::new(move |result| {
            match result {
                Ok(response) => {
                    let hover = Self::parse_hover(&response);
                    callback(Ok(hover));
                }
                Err(error) => {
                    callback(Err(error));
                }
            }
        }))?;

        Ok(())
    }

    /// Parse completion items from LSP response
    fn parse_completion_items(response: &Value) -> Vec<CompletionItem> {
        let mut items = Vec::new();
        
        if let Some(items_array) = response.get("items").and_then(|v| v.as_array()) {
            for item in items_array {
                if let Some(label) = item.get("label").and_then(|v| v.as_str()) {
                    items.push(CompletionItem {
                        label: label.to_string(),
                        kind: Self::parse_completion_item_kind(item.get("kind")),
                        detail: item.get("detail").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        documentation: item.get("documentation").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        sort_text: item.get("sortText").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        filter_text: item.get("filterText").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        insert_text: item.get("insertText").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        insert_text_format: Self::parse_insert_text_format(item.get("insertTextFormat")),
                    });
                }
            }
        }
        
        items
    }

    /// Parse completion item kind
    fn parse_completion_item_kind(kind: Option<&Value>) -> Option<CompletionItemKind> {
        kind.and_then(|v| v.as_u64()).and_then(|k| {
            match k {
                1 => Some(CompletionItemKind::Text),
                2 => Some(CompletionItemKind::Method),
                3 => Some(CompletionItemKind::Function),
                4 => Some(CompletionItemKind::Constructor),
                5 => Some(CompletionItemKind::Field),
                6 => Some(CompletionItemKind::Variable),
                7 => Some(CompletionItemKind::Class),
                8 => Some(CompletionItemKind::Interface),
                9 => Some(CompletionItemKind::Module),
                10 => Some(CompletionItemKind::Property),
                11 => Some(CompletionItemKind::Unit),
                12 => Some(CompletionItemKind::Value),
                13 => Some(CompletionItemKind::Enum),
                14 => Some(CompletionItemKind::Keyword),
                15 => Some(CompletionItemKind::Snippet),
                21 => Some(CompletionItemKind::Constant),
                22 => Some(CompletionItemKind::Struct),
                _ => None,
            }
        })
    }

    /// Parse insert text format
    fn parse_insert_text_format(format: Option<&Value>) -> Option<InsertTextFormat> {
        format.and_then(|v| v.as_u64()).and_then(|f| {
            match f {
                1 => Some(InsertTextFormat::PlainText),
                2 => Some(InsertTextFormat::Snippet),
                _ => None,
            }
        })
    }

    /// Parse hover information
    fn parse_hover(response: &Value) -> Option<Hover> {
        if response.is_null() {
            return None;
        }

        let contents = if let Some(contents_str) = response.get("contents").and_then(|v| v.as_str()) {
            HoverContents::String(contents_str.to_string())
        } else if let Some(contents_obj) = response.get("contents").and_then(|v| v.as_object()) {
            if let (Some(kind), Some(value)) = (
                contents_obj.get("kind").and_then(|v| v.as_str()),
                contents_obj.get("value").and_then(|v| v.as_str())
            ) {
                HoverContents::MarkupContent {
                    kind: kind.to_string(),
                    value: value.to_string(),
                }
            } else {
                HoverContents::String("No content".to_string())
            }
        } else {
            HoverContents::String("No content".to_string())
        };

        Some(Hover {
            contents,
            range: None, // TODO: Parse range if present
        })
    }

    /// Process incoming messages
    pub fn process_messages(&mut self) -> Vec<LspMessage> {
        let mut messages = Vec::new();
        let mut notifications_to_handle = Vec::new();
        
        if let Some(ref rx) = self.message_rx {
            while let Ok(message) = rx.try_recv() {
                match &message {
                    LspMessage::Response { id, .. } => {
                        if let Some(pending) = self.pending_requests.remove(id) {
                            // Handle response
                            match &message {
                                LspMessage::Response { result, error, .. } => {
                                    if let Some(error) = error {
                                        (pending.callback)(Err(error.clone()));
                                    } else {
                                        (pending.callback)(Ok(result.clone().unwrap_or(serde_json::Value::Null)));
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    LspMessage::Notification { method, params } => {
                        notifications_to_handle.push((method.clone(), params.clone()));
                    }
                    _ => {}
                }
                messages.push(message);
            }
        }
        
        // Handle notifications after borrowing is done
        for (method, params) in notifications_to_handle {
            self.handle_notification(&method, &params);
        }
        
        messages
    }

    /// Handle incoming notifications
    fn handle_notification(&mut self, method: &str, params: &Value) {
        match method {
            "textDocument/publishDiagnostics" => {
                self.handle_diagnostics(params);
            }
            _ => {
                // Handle other notifications
            }
        }
    }

    /// Handle diagnostics notification
    fn handle_diagnostics(&mut self, params: &Value) {
        if let Some(uri) = params.get("uri").and_then(|v| v.as_str()) {
            let mut diagnostics = Vec::new();
            
            if let Some(diag_array) = params.get("diagnostics").and_then(|v| v.as_array()) {
                for diag in diag_array {
                    if let Some(message) = diag.get("message").and_then(|v| v.as_str()) {
                        // Parse range
                        let range = if let Some(range_obj) = diag.get("range") {
                            Self::parse_range(range_obj)
                        } else {
                            Range {
                                start: Position { line: 0, character: 0 },
                                end: Position { line: 0, character: 0 },
                            }
                        };

                        diagnostics.push(Diagnostic {
                            range,
                            severity: Self::parse_diagnostic_severity(diag.get("severity")),
                            code: diag.get("code").and_then(|v| v.as_str()).map(|s| s.to_string()),
                            source: diag.get("source").and_then(|v| v.as_str()).map(|s| s.to_string()),
                            message: message.to_string(),
                            related_information: None, // TODO: Parse if needed
                        });
                    }
                }
            }
            
            self.diagnostics.insert(uri.to_string(), diagnostics);
        }
    }

    /// Parse LSP range
    fn parse_range(range_obj: &Value) -> Range {
        let start = if let Some(start_obj) = range_obj.get("start") {
            Position {
                line: start_obj.get("line").and_then(|v| v.as_u64()).unwrap_or(0),
                character: start_obj.get("character").and_then(|v| v.as_u64()).unwrap_or(0),
            }
        } else {
            Position { line: 0, character: 0 }
        };

        let end = if let Some(end_obj) = range_obj.get("end") {
            Position {
                line: end_obj.get("line").and_then(|v| v.as_u64()).unwrap_or(0),
                character: end_obj.get("character").and_then(|v| v.as_u64()).unwrap_or(0),
            }
        } else {
            Position { line: 0, character: 0 }
        };

        Range { start, end }
    }

    /// Parse diagnostic severity
    fn parse_diagnostic_severity(severity: Option<&Value>) -> Option<DiagnosticSeverity> {
        severity.and_then(|v| v.as_u64()).and_then(|s| {
            match s {
                1 => Some(DiagnosticSeverity::Error),
                2 => Some(DiagnosticSeverity::Warning),
                3 => Some(DiagnosticSeverity::Information),
                4 => Some(DiagnosticSeverity::Hint),
                _ => None,
            }
        })
    }

    /// Get diagnostics for a document
    pub fn get_diagnostics(&self, uri: &str) -> Vec<&Diagnostic> {
        self.diagnostics.get(uri).map_or(Vec::new(), |diags| diags.iter().collect())
    }

    /// Check if the LSP client is connected to a language server
    pub fn is_connected(&self) -> bool {
        self.server_process.is_some() && self.server_capabilities.is_some()
    }

    /// Check if there are any diagnostics available
    pub fn has_diagnostics(&self) -> bool {
        !self.diagnostics.is_empty()
    }

    /// Stop the language server
    pub fn stop(&mut self) {
        if let Some(mut process) = self.server_process.take() {
            let _ = process.kill();
            let _ = process.wait();
        }
        self.pending_requests.clear();
        self.diagnostics.clear();
        self.document_versions.clear();
        self.server_capabilities = None;
    }

    /// Render diagnostics in the UI
    pub fn render_diagnostics(&self, ui: &mut egui::Ui) {
        ui.heading("Diagnostics");
        ui.separator();
        
        for (uri, diagnostics) in &self.diagnostics {
            ui.label(format!("File: {}", uri));
            for diagnostic in diagnostics {
                ui.horizontal(|ui| {
                    match diagnostic.severity {
                        Some(DiagnosticSeverity::Error) => ui.label("❌"),
                        Some(DiagnosticSeverity::Warning) => ui.label("⚠️"),
                        Some(DiagnosticSeverity::Information) => ui.label("ℹ️"),
                        Some(DiagnosticSeverity::Hint) => ui.label("💡"),
                        None => ui.label("•"),
                    };
                    ui.label(&diagnostic.message);
                });
            }
            ui.separator();
        }
        
        if self.diagnostics.is_empty() {
            ui.label("No diagnostics available");
        }
    }
}

/// Text document content change event
#[derive(Clone, Debug, serde::Serialize)]
pub struct TextDocumentContentChangeEvent {
    pub range: Option<Range>,
    pub range_length: Option<u64>,
    pub text: String,
}