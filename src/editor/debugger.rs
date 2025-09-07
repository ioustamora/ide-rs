//! Integrated Debugger System
//!
//! Provides debugging capabilities with breakpoints, variable inspection,
//! call stack navigation, and interactive debugging sessions.

use egui::*;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

/// Main debugger interface
pub struct Debugger {
    /// Current debugging session
    pub session: Option<DebugSession>,
    /// Breakpoint manager
    pub breakpoints: BreakpointManager,
    /// Variable inspector
    pub variable_inspector: VariableInspector,
    /// Call stack viewer
    pub call_stack: CallStackViewer,
    /// Debug console
    pub console: DebugConsole,
    /// Debugger settings
    pub settings: DebuggerSettings,
    /// Debug adapters for different languages
    pub adapters: HashMap<String, Box<dyn DebugAdapter>>,
}

/// Active debugging session
pub struct DebugSession {
    /// Session ID
    pub id: String,
    /// Target executable path
    pub executable: PathBuf,
    /// Program arguments
    pub args: Vec<String>,
    /// Environment variables
    pub env: HashMap<String, String>,
    /// Working directory
    pub working_dir: PathBuf,
    /// Current execution state
    pub state: ExecutionState,
    /// Current thread information
    pub threads: Vec<ThreadInfo>,
    /// Active thread ID
    pub active_thread: Option<u64>,
    /// Current stack frame
    pub current_frame: Option<StackFrame>,
    /// Language being debugged
    pub language: String,
}

/// Execution state of the debugged program
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionState {
    /// Program is not running
    Stopped,
    /// Program is running
    Running,
    /// Program is paused (hit breakpoint, stepped, etc.)
    Paused {
        reason: PauseReason,
        location: Option<SourceLocation>,
    },
    /// Program has finished execution
    Exited { exit_code: i32 },
    /// Debugging session ended with error
    Error { message: String },
}

/// Reason for program pause
#[derive(Debug, Clone, PartialEq)]
pub enum PauseReason {
    /// Hit a breakpoint
    Breakpoint { breakpoint_id: u64 },
    /// Step operation completed
    Step,
    /// Program crashed/exception
    Exception { message: String },
    /// Manual pause requested
    Pause,
    /// Program entry point
    Entry,
}

/// Source code location
#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    /// File path
    pub file: PathBuf,
    /// Line number (1-based)
    pub line: u32,
    /// Column number (1-based)
    pub column: Option<u32>,
}

/// Thread information
#[derive(Debug, Clone)]
pub struct ThreadInfo {
    /// Thread ID
    pub id: u64,
    /// Thread name
    pub name: String,
    /// Current location
    pub location: Option<SourceLocation>,
    /// Thread state
    pub state: ThreadState,
}

/// Thread execution state
#[derive(Debug, Clone, PartialEq)]
pub enum ThreadState {
    Running,
    Stopped,
    Paused,
    Exited,
}

/// Breakpoint management system
pub struct BreakpointManager {
    /// All breakpoints indexed by ID
    pub breakpoints: HashMap<u64, Breakpoint>,
    /// Breakpoints by file for quick lookup
    pub by_file: HashMap<PathBuf, HashSet<u64>>,
    /// Next breakpoint ID
    pub next_id: u64,
    /// Breakpoint verification status
    pub verification_status: HashMap<u64, VerificationStatus>,
}

/// Individual breakpoint
#[derive(Debug, Clone)]
pub struct Breakpoint {
    /// Unique breakpoint ID
    pub id: u64,
    /// Source location
    pub location: SourceLocation,
    /// Whether breakpoint is enabled
    pub enabled: bool,
    /// Breakpoint condition (optional)
    pub condition: Option<String>,
    /// Hit count condition
    pub hit_condition: Option<HitCondition>,
    /// Log message (for logging breakpoints)
    pub log_message: Option<String>,
    /// Current hit count
    pub hit_count: u32,
}

/// Hit count condition for breakpoints
#[derive(Debug, Clone)]
pub struct HitCondition {
    /// Condition type
    pub condition_type: HitConditionType,
    /// Count value
    pub count: u32,
}

/// Types of hit count conditions
#[derive(Debug, Clone, PartialEq)]
pub enum HitConditionType {
    /// Break when hit count equals value
    Equals,
    /// Break when hit count is greater than value
    GreaterThan,
    /// Break when hit count is multiple of value
    Multiple,
}

/// Breakpoint verification status
#[derive(Debug, Clone, PartialEq)]
pub enum VerificationStatus {
    /// Breakpoint is verified and active
    Verified,
    /// Breakpoint could not be set
    Failed { reason: String },
    /// Breakpoint is pending verification
    Pending,
}

/// Variable inspector for examining program state
pub struct VariableInspector {
    /// Current variable scope (locals, parameters, etc.)
    pub scopes: Vec<VariableScope>,
    /// Watched expressions
    pub watches: Vec<WatchExpression>,
    /// Variable expansion state (for complex objects)
    pub expanded: HashSet<String>,
    /// Variable evaluation cache
    pub evaluation_cache: HashMap<String, VariableValue>,
}

/// Variable scope (locals, parameters, globals, etc.)
#[derive(Debug, Clone)]
pub struct VariableScope {
    /// Scope name
    pub name: String,
    /// Variables in this scope
    pub variables: Vec<Variable>,
    /// Whether scope is expensive to evaluate
    pub expensive: bool,
}

/// Program variable
#[derive(Debug, Clone)]
pub struct Variable {
    /// Variable name
    pub name: String,
    /// Variable value
    pub value: VariableValue,
    /// Variable type
    pub var_type: String,
    /// Whether variable can be modified
    pub modifiable: bool,
    /// Memory reference (for pointers)
    pub memory_reference: Option<String>,
}

/// Variable value representation
#[derive(Debug, Clone)]
pub enum VariableValue {
    /// Simple value (numbers, strings, booleans)
    Simple(String),
    /// Complex object with children
    Complex {
        summary: String,
        children: Option<Vec<Variable>>,
        child_count: Option<u32>,
    },
    /// Error evaluating variable
    Error(String),
}

/// Watch expression for monitoring values
#[derive(Debug, Clone)]
pub struct WatchExpression {
    /// Expression to evaluate
    pub expression: String,
    /// Current value
    pub value: Option<VariableValue>,
    /// Whether expression is valid
    pub valid: bool,
    /// Error message if invalid
    pub error: Option<String>,
}

/// Call stack viewer for navigation
pub struct CallStackViewer {
    /// Current call stack frames
    pub frames: Vec<StackFrame>,
    /// Currently selected frame
    pub selected_frame: Option<usize>,
    /// Frame expansion state
    pub expanded_frames: HashSet<usize>,
}

/// Stack frame information
#[derive(Debug, Clone)]
pub struct StackFrame {
    /// Frame ID
    pub id: u64,
    /// Function/method name
    pub name: String,
    /// Source location
    pub location: Option<SourceLocation>,
    /// Frame instruction pointer
    pub instruction_pointer: Option<u64>,
    /// Module/library name
    pub module: Option<String>,
    /// Presentation hint for UI
    pub presentation_hint: Option<FramePresentationHint>,
}

/// Presentation hint for stack frames
#[derive(Debug, Clone, PartialEq)]
pub enum FramePresentationHint {
    /// Normal user code frame
    Normal,
    /// System/library code frame
    Subtle,
    /// Frame with special meaning
    Label,
}

/// Debug console for interactive debugging
pub struct DebugConsole {
    /// Console output history
    pub output: Vec<ConsoleMessage>,
    /// Current input
    pub input: String,
    /// Command history
    pub history: Vec<String>,
    /// History navigation index
    pub history_index: Option<usize>,
    /// Maximum output lines
    pub max_lines: usize,
}

/// Console message
#[derive(Debug, Clone)]
pub struct ConsoleMessage {
    /// Message text
    pub text: String,
    /// Message type
    pub message_type: ConsoleMessageType,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
}

/// Types of console messages
#[derive(Debug, Clone, PartialEq)]
pub enum ConsoleMessageType {
    /// Program output
    Output,
    /// Debug command result
    Result,
    /// Error message
    Error,
    /// Warning message
    Warning,
    /// Input command
    Input,
}

/// Debug adapter interface for language-specific debugging
pub trait DebugAdapter: Send + Sync {
    /// Start a debugging session
    fn start_session(&mut self, config: &DebugConfiguration) -> Result<String, DebugError>;
    
    /// Stop the current debugging session
    fn stop_session(&mut self, session_id: &str) -> Result<(), DebugError>;
    
    /// Set a breakpoint
    fn set_breakpoint(&mut self, breakpoint: &Breakpoint) -> Result<VerificationStatus, DebugError>;
    
    /// Remove a breakpoint
    fn remove_breakpoint(&mut self, breakpoint_id: u64) -> Result<(), DebugError>;
    
    /// Continue program execution
    fn continue_execution(&mut self, session_id: &str) -> Result<(), DebugError>;
    
    /// Step over next statement
    fn step_over(&mut self, session_id: &str, thread_id: u64) -> Result<(), DebugError>;
    
    /// Step into function call
    fn step_into(&mut self, session_id: &str, thread_id: u64) -> Result<(), DebugError>;
    
    /// Step out of current function
    fn step_out(&mut self, session_id: &str, thread_id: u64) -> Result<(), DebugError>;
    
    /// Pause program execution
    fn pause(&mut self, session_id: &str) -> Result<(), DebugError>;
    
    /// Get current call stack
    fn get_call_stack(&mut self, session_id: &str, thread_id: u64) -> Result<Vec<StackFrame>, DebugError>;
    
    /// Get variables in scope
    fn get_variables(&mut self, session_id: &str, frame_id: u64) -> Result<Vec<VariableScope>, DebugError>;
    
    /// Evaluate expression
    fn evaluate_expression(&mut self, session_id: &str, expression: &str, frame_id: Option<u64>) -> Result<VariableValue, DebugError>;
}

/// Debug configuration
#[derive(Debug, Clone)]
pub struct DebugConfiguration {
    /// Configuration name
    pub name: String,
    /// Debug adapter type
    pub adapter_type: String,
    /// Program to debug
    pub program: PathBuf,
    /// Program arguments
    pub args: Vec<String>,
    /// Working directory
    pub cwd: PathBuf,
    /// Environment variables
    pub env: HashMap<String, String>,
    /// Stop at entry point
    pub stop_at_entry: bool,
    /// Additional adapter-specific configuration
    pub additional_config: HashMap<String, serde_json::Value>,
}

/// Debugger settings
#[derive(Debug, Clone)]
pub struct DebuggerSettings {
    /// Show system frames in call stack
    pub show_system_frames: bool,
    /// Auto-expand complex variables
    pub auto_expand_variables: bool,
    /// Maximum variable string length
    pub max_variable_string_length: usize,
    /// Enable expression evaluation
    pub enable_expression_evaluation: bool,
    /// Confirm before stopping debug session
    pub confirm_stop_session: bool,
}

/// Debug error types
#[derive(Debug, thiserror::Error)]
pub enum DebugError {
    #[error("Debug adapter not found for language: {0}")]
    AdapterNotFound(String),
    #[error("Session not found: {0}")]
    SessionNotFound(String),
    #[error("Debugger communication error: {0}")]
    CommunicationError(String),
    #[error("Invalid breakpoint location: {0:?}")]
    InvalidBreakpointLocation(SourceLocation),
    #[error("Expression evaluation failed: {0}")]
    ExpressionEvaluationFailed(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl Default for DebuggerSettings {
    fn default() -> Self {
        Self {
            show_system_frames: false,
            auto_expand_variables: false,
            max_variable_string_length: 1000,
            enable_expression_evaluation: true,
            confirm_stop_session: true,
        }
    }
}

impl BreakpointManager {
    pub fn new() -> Self {
        Self {
            breakpoints: HashMap::new(),
            by_file: HashMap::new(),
            next_id: 1,
            verification_status: HashMap::new(),
        }
    }

    /// Add a new breakpoint
    pub fn add_breakpoint(&mut self, location: SourceLocation) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let breakpoint = Breakpoint {
            id,
            location: location.clone(),
            enabled: true,
            condition: None,
            hit_condition: None,
            log_message: None,
            hit_count: 0,
        };

        self.breakpoints.insert(id, breakpoint);
        self.by_file
            .entry(location.file.clone())
            .or_default()
            .insert(id);

        id
    }

    /// Remove a breakpoint
    pub fn remove_breakpoint(&mut self, id: u64) -> bool {
        if let Some(breakpoint) = self.breakpoints.remove(&id) {
            if let Some(file_breakpoints) = self.by_file.get_mut(&breakpoint.location.file) {
                file_breakpoints.remove(&id);
                if file_breakpoints.is_empty() {
                    self.by_file.remove(&breakpoint.location.file);
                }
            }
            self.verification_status.remove(&id);
            true
        } else {
            false
        }
    }

    /// Toggle breakpoint enabled state
    pub fn toggle_breakpoint(&mut self, id: u64) -> bool {
        if let Some(breakpoint) = self.breakpoints.get_mut(&id) {
            breakpoint.enabled = !breakpoint.enabled;
            breakpoint.enabled
        } else {
            false
        }
    }

    /// Get breakpoints for a specific file
    pub fn get_breakpoints_for_file(&self, file: &PathBuf) -> Vec<&Breakpoint> {
        if let Some(breakpoint_ids) = self.by_file.get(file) {
            breakpoint_ids
                .iter()
                .filter_map(|id| self.breakpoints.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl VariableInspector {
    pub fn new() -> Self {
        Self {
            scopes: Vec::new(),
            watches: Vec::new(),
            expanded: HashSet::new(),
            evaluation_cache: HashMap::new(),
        }
    }

    /// Add a watch expression
    pub fn add_watch(&mut self, expression: String) {
        self.watches.push(WatchExpression {
            expression,
            value: None,
            valid: true,
            error: None,
        });
    }

    /// Remove a watch expression
    pub fn remove_watch(&mut self, index: usize) {
        if index < self.watches.len() {
            self.watches.remove(index);
        }
    }

    /// Toggle variable expansion
    pub fn toggle_expansion(&mut self, variable_path: String) {
        if self.expanded.contains(&variable_path) {
            self.expanded.remove(&variable_path);
        } else {
            self.expanded.insert(variable_path);
        }
    }
}

impl CallStackViewer {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            selected_frame: None,
            expanded_frames: HashSet::new(),
        }
    }

    /// Select a specific frame
    pub fn select_frame(&mut self, index: usize) -> Option<&StackFrame> {
        if index < self.frames.len() {
            self.selected_frame = Some(index);
            Some(&self.frames[index])
        } else {
            None
        }
    }
}

impl DebugConsole {
    pub fn new() -> Self {
        Self {
            output: Vec::new(),
            input: String::new(),
            history: Vec::new(),
            history_index: None,
            max_lines: 1000,
        }
    }

    /// Add a message to console output
    pub fn add_message(&mut self, text: String, message_type: ConsoleMessageType) {
        let message = ConsoleMessage {
            text,
            message_type,
            timestamp: std::time::SystemTime::now(),
        };

        self.output.push(message);

        // Limit output lines
        if self.output.len() > self.max_lines {
            self.output.remove(0);
        }
    }

    /// Execute a debug command
    pub fn execute_command(&mut self, command: String) -> String {
        // Add to history
        if !command.is_empty() {
            self.history.push(command.clone());
            self.history_index = None;
        }

        // Add command to output
        self.add_message(format!("> {}", command), ConsoleMessageType::Input);

        // Parse and execute command
        let result = self.parse_and_execute(&command);
        
        // Add result to output
        self.add_message(result.clone(), ConsoleMessageType::Result);

        result
    }

    /// Parse and execute debug command
    fn parse_and_execute(&self, command: &str) -> String {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return String::new();
        }

        match parts[0] {
            "help" | "h" => self.show_help(),
            "continue" | "c" => "Continuing execution...".to_string(),
            "step" | "s" => "Stepping over...".to_string(),
            "stepi" | "si" => "Stepping into...".to_string(),
            "stepo" | "so" => "Stepping out...".to_string(),
            "print" | "p" => {
                if parts.len() > 1 {
                    format!("Evaluating: {}", parts[1..].join(" "))
                } else {
                    "Usage: print <expression>".to_string()
                }
            }
            "break" | "b" => {
                if parts.len() > 1 {
                    format!("Setting breakpoint at: {}", parts[1])
                } else {
                    "Usage: break <location>".to_string()
                }
            }
            "list" | "l" => "Listing breakpoints...".to_string(),
            "clear" => "Console cleared.".to_string(),
            _ => format!("Unknown command: {}. Type 'help' for available commands.", parts[0]),
        }
    }

    /// Show available debug commands
    fn show_help(&self) -> String {
        r#"Available debug commands:
  help, h              - Show this help
  continue, c          - Continue execution
  step, s              - Step over next line
  stepi, si           - Step into function
  stepo, so           - Step out of function
  print, p <expr>     - Print expression value
  break, b <location> - Set breakpoint
  list, l             - List breakpoints
  clear               - Clear console"#.to_string()
    }
}

impl Debugger {
    /// Create a new debugger instance
    pub fn new() -> Self {
        Self {
            session: None,
            breakpoints: BreakpointManager::new(),
            variable_inspector: VariableInspector::new(),
            call_stack: CallStackViewer::new(),
            console: DebugConsole::new(),
            settings: DebuggerSettings::default(),
            adapters: HashMap::new(),
        }
    }

    /// Register a debug adapter for a language
    pub fn register_adapter(&mut self, language: String, adapter: Box<dyn DebugAdapter>) {
        self.adapters.insert(language, adapter);
    }

    /// Start a debugging session
    pub fn start_debug_session(&mut self, config: DebugConfiguration) -> Result<(), DebugError> {
        let adapter = self.adapters.get_mut(&config.adapter_type)
            .ok_or_else(|| DebugError::AdapterNotFound(config.adapter_type.clone()))?;

        let session_id = adapter.start_session(&config)?;

        let session = DebugSession {
            id: session_id,
            executable: config.program.clone(),
            args: config.args.clone(),
            env: config.env.clone(),
            working_dir: config.cwd.clone(),
            state: ExecutionState::Stopped,
            threads: Vec::new(),
            active_thread: None,
            current_frame: None,
            language: config.adapter_type.clone(),
        };

        self.session = Some(session);
        self.console.add_message("Debug session started.".to_string(), ConsoleMessageType::Output);

        Ok(())
    }

    /// Stop the current debugging session
    pub fn stop_debug_session(&mut self) -> Result<(), DebugError> {
        if let Some(session) = &self.session {
            if let Some(adapter) = self.adapters.get_mut(&session.language) {
                adapter.stop_session(&session.id)?;
            }
            self.session = None;
            self.console.add_message("Debug session stopped.".to_string(), ConsoleMessageType::Output);
        }
        Ok(())
    }

    /// Render debugger UI
    pub fn render_debugger_ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            // Debug controls
            if self.session.is_some() {
                if ui.button("⏸ Pause").clicked() {
                    // TODO: Implement pause
                }
                if ui.button("▶ Continue").clicked() {
                    // TODO: Implement continue
                }
                if ui.button("⏭ Step Over").clicked() {
                    // TODO: Implement step over
                }
                if ui.button("⏬ Step Into").clicked() {
                    // TODO: Implement step into
                }
                if ui.button("⏫ Step Out").clicked() {
                    // TODO: Implement step out
                }
                if ui.button("⏹ Stop").clicked() {
                    let _ = self.stop_debug_session();
                }
            } else {
                if ui.button("▶ Start Debug").clicked() {
                    // TODO: Show debug configuration dialog
                }
            }
        });

        ui.separator();

        // Debug panels
        ui.horizontal(|ui| {
            // Variables panel
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.heading("Variables");
                    ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                        for scope in &self.variable_inspector.scopes {
                            ui.collapsing(&scope.name, |ui| {
                                for var in &scope.variables {
                                    ui.horizontal(|ui| {
                                        ui.label(&var.name);
                                        ui.label(":");
                                        match &var.value {
                                            VariableValue::Simple(value) => {
                                                ui.label(value);
                                            }
                                            VariableValue::Complex { summary, .. } => {
                                                ui.label(summary);
                                            }
                                            VariableValue::Error(error) => {
                                                ui.colored_label(Color32::RED, error);
                                            }
                                        }
                                    });
                                }
                            });
                        }
                    });
                });
            });

            // Call stack panel
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.heading("Call Stack");
                    ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                        let frames_len = self.call_stack.frames.len();
                        for i in 0..frames_len {
                            if let Some(frame) = self.call_stack.frames.get(i) {
                                let is_selected = self.call_stack.selected_frame == Some(i);
                                if ui.selectable_label(is_selected, &frame.name).clicked() {
                                    self.call_stack.select_frame(i);
                                }
                            }
                        }
                    });
                });
            });
        });

        ui.separator();

        // Debug console
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("Debug Console");
                
                // Output area
                ScrollArea::vertical()
                    .max_height(100.0)
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for message in &self.console.output {
                            let color = match message.message_type {
                                ConsoleMessageType::Output => Color32::WHITE,
                                ConsoleMessageType::Result => Color32::LIGHT_BLUE,
                                ConsoleMessageType::Error => Color32::RED,
                                ConsoleMessageType::Warning => Color32::YELLOW,
                                ConsoleMessageType::Input => Color32::GREEN,
                            };
                            ui.colored_label(color, &message.text);
                        }
                    });

                // Input area
                ui.horizontal(|ui| {
                    ui.label(">");
                    let response = ui.text_edit_singleline(&mut self.console.input);
                    if response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                        let command = self.console.input.clone();
                        self.console.input.clear();
                        self.console.execute_command(command);
                    }
                });
            });
        });
    }
}

impl Default for Debugger {
    fn default() -> Self {
        Self::new()
    }
}