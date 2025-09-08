//! # Terminal Core Functionality
//!
//! Core terminal abstractions shared between basic and advanced terminal implementations.
//! This module consolidates the common functionality and provides the foundation
//! for both simple and feature-rich terminal usage.

use egui::*;
use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{Receiver, Sender};
use std::time::{Duration, Instant};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

/// Unique identifier for terminal instances
pub type TerminalId = Uuid;

/// Core terminal manager handling multiple terminal instances
pub struct TerminalManager {
    /// Active terminal instances
    pub terminals: HashMap<TerminalId, Terminal>,
    /// Currently active terminal
    pub active_terminal: Option<TerminalId>,
    /// Terminal settings
    pub settings: TerminalSettings,
    /// UI state
    pub ui_state: TerminalUIState,
    /// Event publisher for terminal events
    event_sender: Option<Sender<TerminalEvent>>,
}

/// Individual terminal instance with core functionality
pub struct Terminal {
    /// Unique terminal identifier
    pub id: TerminalId,
    /// Terminal name/title
    pub name: String,
    /// Working directory
    pub working_dir: PathBuf,
    /// Shell process
    pub process: Option<TerminalProcess>,
    /// Output buffer with scrollback
    pub output_buffer: TerminalBuffer,
    /// Input history
    pub input_history: InputHistory,
    /// Current input line
    pub current_input: String,
    /// Terminal state
    pub state: TerminalState,
    /// Shell type
    pub shell_type: ShellType,
    /// Terminal creation time
    pub created_at: Instant,
    /// Last activity timestamp
    pub last_activity: Instant,
    /// Environment variables
    pub environment: HashMap<String, String>,
}

/// Terminal process wrapper for better management
pub struct TerminalProcess {
    /// Child process handle
    pub child: Child,
    /// Process ID
    pub pid: u32,
    /// Output receiver channel
    pub output_receiver: Receiver<String>,
    /// Error output receiver
    pub error_receiver: Receiver<String>,
    /// Input sender channel
    pub input_sender: Sender<String>,
    /// Process start time
    pub started_at: Instant,
}

/// Terminal output buffer with efficient scrollback management
pub struct TerminalBuffer {
    /// Output lines
    pub lines: VecDeque<TerminalLine>,
    /// Maximum number of lines to keep
    pub max_lines: usize,
    /// Current scroll position (0 = bottom)
    pub scroll_offset: usize,
    /// Total lines ever added (for statistics)
    pub total_lines_added: u64,
    /// Buffer memory usage estimate
    pub estimated_memory_bytes: usize,
}

/// Input history management
pub struct InputHistory {
    /// Command history
    pub commands: VecDeque<String>,
    /// Maximum history size
    pub max_size: usize,
    /// Current navigation index (None = not navigating)
    pub navigation_index: Option<usize>,
    /// Temporary input while navigating
    pub temp_input: Option<String>,
}

/// Individual terminal output line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalLine {
    /// Line content
    pub content: String,
    /// Line type (normal, error, etc.)
    pub line_type: LineType,
    /// Timestamp when line was added
    pub timestamp: Instant,
    /// Whether line ends with newline
    pub ends_with_newline: bool,
    /// ANSI escape sequences metadata
    pub ansi_metadata: Option<AnsiMetadata>,
}

/// ANSI escape sequences metadata for styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnsiMetadata {
    /// Foreground color
    pub fg_color: Option<Color32>,
    /// Background color  
    pub bg_color: Option<Color32>,
    /// Text styling flags
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
}

/// Type of terminal line content
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineType {
    /// Normal command output
    Normal,
    /// Error output (stderr)
    Error,
    /// User input command
    Input,
    /// System message
    System,
    /// Debug/verbose output
    Debug,
}

/// Terminal state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerminalState {
    /// Terminal is ready for input
    Ready,
    /// Terminal is running a command
    Running,
    /// Terminal process has exited
    Exited(i32),
    /// Terminal encountered an error
    Error,
    /// Terminal is being initialized
    Initializing,
    /// Terminal is suspended
    Suspended,
}

/// Supported shell types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShellType {
    /// Bash shell
    Bash,
    /// Zsh shell
    Zsh,
    /// Fish shell
    Fish,
    /// PowerShell (Windows/Cross-platform)
    PowerShell,
    /// Command Prompt (Windows)
    Cmd,
    /// Custom shell
    Custom(u32), // Hash of shell name
}

/// Terminal configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalSettings {
    /// Default shell type
    pub default_shell: ShellType,
    /// Font family
    pub font_family: String,
    /// Font size
    pub font_size: f32,
    /// Scrollback buffer size
    pub scrollback_lines: usize,
    /// Working directory for new terminals
    pub default_working_dir: Option<PathBuf>,
    /// Environment variables to set
    pub environment_variables: HashMap<String, String>,
    /// Whether to close terminal when process exits
    pub close_on_exit: bool,
    /// Confirm before closing
    pub confirm_close: bool,
    /// Enable bell/notification sounds
    pub enable_bell: bool,
    /// Cursor blink rate (ms)
    pub cursor_blink_rate: u32,
    /// Tab width in characters
    pub tab_width: u32,
    /// Maximum memory usage for buffer (bytes)
    pub max_buffer_memory: usize,
}

/// Terminal UI state for rendering
#[derive(Debug, Default)]
pub struct TerminalUIState {
    /// Whether input field has focus
    pub input_focused: bool,
    /// Auto-scroll to bottom
    pub auto_scroll: bool,
    /// Show line numbers
    pub show_line_numbers: bool,
    /// Word wrap long lines
    pub word_wrap: bool,
    /// Current search query
    pub search_query: String,
    /// Search results highlighting
    pub search_results: Vec<SearchResult>,
    /// Current search result index
    pub current_search_index: usize,
}

/// Search result in terminal output
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// Line number
    pub line: usize,
    /// Character start position in line
    pub start: usize,
    /// Character end position in line
    pub end: usize,
}

/// Terminal events for communication with other IDE components
#[derive(Debug, Clone)]
pub enum TerminalEvent {
    /// Terminal was created
    Created { terminal_id: TerminalId, name: String },
    /// Terminal was closed
    Closed { terminal_id: TerminalId },
    /// Terminal state changed
    StateChanged { terminal_id: TerminalId, old_state: TerminalState, new_state: TerminalState },
    /// Command was executed
    CommandExecuted { terminal_id: TerminalId, command: String },
    /// Output was received
    OutputReceived { terminal_id: TerminalId, content: String, line_type: LineType },
    /// Working directory changed
    WorkingDirectoryChanged { terminal_id: TerminalId, old_dir: PathBuf, new_dir: PathBuf },
    /// Process exited
    ProcessExited { terminal_id: TerminalId, exit_code: i32 },
}

impl Default for TerminalSettings {
    fn default() -> Self {
        Self {
            default_shell: if cfg!(windows) { ShellType::PowerShell } else { ShellType::Bash },
            font_family: "JetBrains Mono".to_string(),
            font_size: 12.0,
            scrollback_lines: 10000,
            default_working_dir: None,
            environment_variables: HashMap::new(),
            close_on_exit: false,
            confirm_close: true,
            enable_bell: true,
            cursor_blink_rate: 530,
            tab_width: 4,
            max_buffer_memory: 50 * 1024 * 1024, // 50MB
        }
    }
}

impl TerminalManager {
    /// Create a new terminal manager
    pub fn new() -> Self {
        Self::with_settings(TerminalSettings::default())
    }
    
    /// Create terminal manager with custom settings
    pub fn with_settings(settings: TerminalSettings) -> Self {
        Self {
            terminals: HashMap::new(),
            active_terminal: None,
            settings,
            ui_state: TerminalUIState::default(),
            event_sender: None,
        }
    }
    
    /// Set event sender for publishing terminal events
    pub fn set_event_sender(&mut self, sender: Sender<TerminalEvent>) {
        self.event_sender = Some(sender);
    }
    
    /// Create a new terminal instance
    pub fn create_terminal(&mut self, name: Option<String>) -> TerminalId {
        let terminal_id = Uuid::new_v4();
        let terminal_name = name.unwrap_or_else(|| format!("Terminal {}", self.terminals.len() + 1));
        
        let mut terminal = Terminal::new(terminal_id, terminal_name.clone(), &self.settings);
        
        // Start the terminal process
        if let Err(e) = terminal.start_process(&self.settings) {
            crate::log_error!("Failed to start terminal process: {}", e);
            terminal.state = TerminalState::Error;
        }
        
        self.terminals.insert(terminal_id, terminal);
        
        // Set as active if it's the first terminal
        if self.active_terminal.is_none() {
            self.active_terminal = Some(terminal_id);
        }
        
        // Send event
        if let Some(sender) = &self.event_sender {
            let _ = sender.send(TerminalEvent::Created {
                terminal_id,
                name: terminal_name,
            });
        }
        
        terminal_id
    }
    
    /// Close a terminal
    pub fn close_terminal(&mut self, terminal_id: TerminalId) -> bool {
        if let Some(mut terminal) = self.terminals.remove(&terminal_id) {
            terminal.cleanup();
            
            // Update active terminal if we closed the active one
            if self.active_terminal == Some(terminal_id) {
                self.active_terminal = self.terminals.keys().next().copied();
            }
            
            // Send event
            if let Some(sender) = &self.event_sender {
                let _ = sender.send(TerminalEvent::Closed { terminal_id });
            }
            
            true
        } else {
            false
        }
    }
    
    /// Get active terminal
    pub fn active_terminal(&self) -> Option<&Terminal> {
        self.active_terminal.and_then(|id| self.terminals.get(&id))
    }
    
    /// Get active terminal mutably
    pub fn active_terminal_mut(&mut self) -> Option<&mut Terminal> {
        self.active_terminal.and_then(|id| self.terminals.get_mut(&id))
    }
    
    /// Set active terminal
    pub fn set_active_terminal(&mut self, terminal_id: TerminalId) -> bool {
        if self.terminals.contains_key(&terminal_id) {
            self.active_terminal = Some(terminal_id);
            true
        } else {
            false
        }
    }
    
    /// Get all terminals
    pub fn get_terminals(&self) -> &HashMap<TerminalId, Terminal> {
        &self.terminals
    }
    
    /// Process terminal outputs (should be called regularly)
    pub fn update(&mut self) {
        let terminal_ids: Vec<TerminalId> = self.terminals.keys().copied().collect();
        
        for terminal_id in terminal_ids {
            if let Some(terminal) = self.terminals.get_mut(&terminal_id) {
                terminal.update(&self.event_sender);
            }
        }
    }
    
    /// Send input to active terminal
    pub fn send_input(&mut self, input: &str) -> Result<(), String> {
        if let Some(terminal) = self.active_terminal_mut() {
            terminal.send_input(input)
        } else {
            Err("No active terminal".to_string())
        }
    }
    
    /// Get terminal statistics
    pub fn get_statistics(&self) -> TerminalStatistics {
        let mut stats = TerminalStatistics::default();
        
        for terminal in self.terminals.values() {
            stats.total_terminals += 1;
            stats.total_lines += terminal.output_buffer.lines.len();
            stats.total_memory_bytes += terminal.output_buffer.estimated_memory_bytes;
            
            match terminal.state {
                TerminalState::Running => stats.running_terminals += 1,
                TerminalState::Exited(_) => stats.exited_terminals += 1,
                TerminalState::Error => stats.error_terminals += 1,
                _ => {}
            }
        }
        
        stats
    }
}

impl Terminal {
    /// Create a new terminal instance
    pub fn new(id: TerminalId, name: String, settings: &TerminalSettings) -> Self {
        let working_dir = settings.default_working_dir.clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")));
        
        Self {
            id,
            name,
            working_dir,
            process: None,
            output_buffer: TerminalBuffer::new(settings.scrollback_lines, settings.max_buffer_memory),
            input_history: InputHistory::new(1000), // Keep last 1000 commands
            current_input: String::new(),
            state: TerminalState::Initializing,
            shell_type: settings.default_shell,
            created_at: Instant::now(),
            last_activity: Instant::now(),
            environment: settings.environment_variables.clone(),
        }
    }
    
    /// Start the terminal process
    pub fn start_process(&mut self, settings: &TerminalSettings) -> Result<(), String> {
        let (shell_cmd, shell_args) = self.get_shell_command();
        
        let mut command = Command::new(shell_cmd);
        command
            .args(shell_args)
            .current_dir(&self.working_dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        
        // Set environment variables
        for (key, value) in &self.environment {
            command.env(key, value);
        }
        
        match command.spawn() {
            Ok(mut child) => {
                let pid = child.id();
                
                // Create channels for process communication
                let (output_sender, output_receiver) = std::sync::mpsc::channel();
                let (error_sender, error_receiver) = std::sync::mpsc::channel();
                let (input_sender, input_receiver) = std::sync::mpsc::channel();
                
                // Spawn threads for process I/O
                self.spawn_io_threads(&mut child, output_sender, error_sender, input_receiver);
                
                self.process = Some(TerminalProcess {
                    child,
                    pid,
                    output_receiver,
                    error_receiver,
                    input_sender,
                    started_at: Instant::now(),
                });
                
                self.state = TerminalState::Ready;
                self.add_system_message(&format!("Terminal started (PID: {})", pid));
                
                Ok(())
            }
            Err(e) => {
                self.state = TerminalState::Error;
                self.add_error_message(&format!("Failed to start shell: {}", e));
                Err(e.to_string())
            }
        }
    }
    
    /// Get shell command and arguments
    fn get_shell_command(&self) -> (&str, Vec<&str>) {
        match self.shell_type {
            ShellType::Bash => ("bash", vec!["-i"]),
            ShellType::Zsh => ("zsh", vec!["-i"]),
            ShellType::Fish => ("fish", vec!["-i"]),
            ShellType::PowerShell => {
                if cfg!(windows) {
                    ("powershell.exe", vec!["-NoExit", "-Command", "-"])
                } else {
                    ("pwsh", vec!["-i"])
                }
            }
            ShellType::Cmd => ("cmd.exe", vec!["/Q"]),
            ShellType::Custom(_) => ("sh", vec!["-i"]), // Fallback
        }
    }
    
    /// Spawn I/O threads for process communication
    fn spawn_io_threads(
        &self,
        child: &mut std::process::Child,
        output_sender: Sender<String>,
        error_sender: Sender<String>,
        input_receiver: Receiver<String>,
    ) {
        use std::io::{BufRead, BufReader, Write};
        
        // Stdout reading thread
        if let Some(stdout) = child.stdout.take() {
            std::thread::spawn(move || {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    match line {
                        Ok(content) => {
                            if output_sender.send(content).is_err() {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
            });
        }
        
        // Stderr reading thread
        if let Some(stderr) = child.stderr.take() {
            std::thread::spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines() {
                    match line {
                        Ok(content) => {
                            if error_sender.send(content).is_err() {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
            });
        }
        
        // Stdin writing thread
        if let Some(mut stdin) = child.stdin.take() {
            std::thread::spawn(move || {
                while let Ok(input) = input_receiver.recv() {
                    if stdin.write_all(input.as_bytes()).is_err() {
                        break;
                    }
                    if stdin.flush().is_err() {
                        break;
                    }
                }
            });
        }
    }
    
    /// Update terminal (read output, check process status)
    pub fn update(&mut self, event_sender: &Option<Sender<TerminalEvent>>) {
        if let Some(process) = &mut self.process {
            // Read stdout
            while let Ok(output) = process.output_receiver.try_recv() {
                self.add_output_line(output, LineType::Normal);
                self.last_activity = Instant::now();
                
                if let Some(sender) = event_sender {
                    let _ = sender.send(TerminalEvent::OutputReceived {
                        terminal_id: self.id,
                        content: output,
                        line_type: LineType::Normal,
                    });
                }
            }
            
            // Read stderr
            while let Ok(error) = process.error_receiver.try_recv() {
                self.add_output_line(error, LineType::Error);
                self.last_activity = Instant::now();
                
                if let Some(sender) = event_sender {
                    let _ = sender.send(TerminalEvent::OutputReceived {
                        terminal_id: self.id,
                        content: error,
                        line_type: LineType::Error,
                    });
                }
            }
            
            // Check process status
            match process.child.try_wait() {
                Ok(Some(exit_status)) => {
                    let exit_code = exit_status.code().unwrap_or(-1);
                    let old_state = self.state;
                    self.state = TerminalState::Exited(exit_code);
                    
                    self.add_system_message(&format!("Process exited with code: {}", exit_code));
                    
                    if let Some(sender) = event_sender {
                        let _ = sender.send(TerminalEvent::StateChanged {
                            terminal_id: self.id,
                            old_state,
                            new_state: self.state,
                        });
                        let _ = sender.send(TerminalEvent::ProcessExited {
                            terminal_id: self.id,
                            exit_code,
                        });
                    }
                }
                Ok(None) => {
                    // Process still running
                    if self.state == TerminalState::Initializing {
                        self.state = TerminalState::Ready;
                    }
                }
                Err(e) => {
                    crate::log_error!("Error checking process status: {}", e);
                    self.state = TerminalState::Error;
                }
            }
        }
    }
    
    /// Send input to terminal
    pub fn send_input(&mut self, input: &str) -> Result<(), String> {
        if let Some(process) = &self.process {
            // Add to history if it's a complete command (ends with newline)
            if input.ends_with('\n') {
                let command = input.trim_end().to_string();
                if !command.is_empty() {
                    self.input_history.add_command(command.clone());
                    self.add_input_line(command);
                }
            }
            
            // Send to process
            process.input_sender.send(input.to_string())
                .map_err(|e| format!("Failed to send input: {}", e))?;
            
            self.last_activity = Instant::now();
            Ok(())
        } else {
            Err("No active process".to_string())
        }
    }
    
    /// Add output line to buffer
    fn add_output_line(&mut self, content: String, line_type: LineType) {
        let line = TerminalLine {
            content,
            line_type,
            timestamp: Instant::now(),
            ends_with_newline: true,
            ansi_metadata: None, // TODO: Parse ANSI sequences
        };
        
        self.output_buffer.add_line(line);
    }
    
    /// Add input line to buffer
    fn add_input_line(&mut self, content: String) {
        self.add_output_line(format!("$ {}", content), LineType::Input);
    }
    
    /// Add system message to buffer
    fn add_system_message(&mut self, content: &str) {
        self.add_output_line(content.to_string(), LineType::System);
    }
    
    /// Add error message to buffer
    fn add_error_message(&mut self, content: &str) {
        self.add_output_line(content.to_string(), LineType::Error);
    }
    
    /// Clean up terminal resources
    pub fn cleanup(&mut self) {
        if let Some(mut process) = self.process.take() {
            let _ = process.child.kill();
            let _ = process.child.wait();
        }
    }
}

impl TerminalBuffer {
    /// Create a new terminal buffer
    pub fn new(max_lines: usize, max_memory_bytes: usize) -> Self {
        Self {
            lines: VecDeque::with_capacity(std::cmp::min(max_lines, 1000)),
            max_lines,
            scroll_offset: 0,
            total_lines_added: 0,
            estimated_memory_bytes: 0,
        }
    }
    
    /// Add a line to the buffer
    pub fn add_line(&mut self, line: TerminalLine) {
        let line_memory = line.content.len() + std::mem::size_of::<TerminalLine>();
        
        self.lines.push_back(line);
        self.estimated_memory_bytes += line_memory;
        self.total_lines_added += 1;
        
        // Enforce memory and line limits
        while self.lines.len() > self.max_lines || self.estimated_memory_bytes > (self.max_memory_bytes * 2) {
            if let Some(removed_line) = self.lines.pop_front() {
                self.estimated_memory_bytes -= removed_line.content.len() + std::mem::size_of::<TerminalLine>();
            } else {
                break;
            }
        }
    }
    
    /// Clear the buffer
    pub fn clear(&mut self) {
        self.lines.clear();
        self.estimated_memory_bytes = 0;
        self.scroll_offset = 0;
    }
    
    /// Get lines in visible range
    pub fn get_visible_lines(&self, viewport_height: usize) -> &[TerminalLine] {
        let total_lines = self.lines.len();
        if total_lines == 0 {
            return &[];
        }
        
        let start_index = if self.scroll_offset >= total_lines {
            0
        } else {
            total_lines - self.scroll_offset - viewport_height
        }.max(0);
        
        let end_index = (start_index + viewport_height).min(total_lines);
        
        // Convert VecDeque slice to regular slice
        let lines_vec: Vec<&TerminalLine> = self.lines.iter().collect();
        unsafe {
            std::slice::from_raw_parts(
                lines_vec.as_ptr().add(start_index) as *const TerminalLine,
                end_index - start_index
            )
        }
    }
}

impl InputHistory {
    /// Create new input history
    pub fn new(max_size: usize) -> Self {
        Self {
            commands: VecDeque::with_capacity(std::cmp::min(max_size, 1000)),
            max_size,
            navigation_index: None,
            temp_input: None,
        }
    }
    
    /// Add a command to history
    pub fn add_command(&mut self, command: String) {
        // Don't add empty commands or duplicates
        if command.trim().is_empty() || self.commands.back() == Some(&command) {
            return;
        }
        
        self.commands.push_back(command);
        
        // Enforce size limit
        while self.commands.len() > self.max_size {
            self.commands.pop_front();
        }
        
        // Reset navigation
        self.navigation_index = None;
        self.temp_input = None;
    }
    
    /// Navigate to previous command in history
    pub fn previous(&mut self, current_input: &str) -> Option<String> {
        if self.commands.is_empty() {
            return None;
        }
        
        match self.navigation_index {
            None => {
                // Start navigation
                self.temp_input = Some(current_input.to_string());
                self.navigation_index = Some(self.commands.len() - 1);
                self.commands.back().cloned()
            }
            Some(index) => {
                if index > 0 {
                    self.navigation_index = Some(index - 1);
                    self.commands.get(index - 1).cloned()
                } else {
                    None
                }
            }
        }
    }
    
    /// Navigate to next command in history
    pub fn next(&mut self) -> Option<String> {
        match self.navigation_index {
            Some(index) => {
                if index < self.commands.len() - 1 {
                    self.navigation_index = Some(index + 1);
                    self.commands.get(index + 1).cloned()
                } else {
                    // Return to current input
                    let temp = self.temp_input.take();
                    self.navigation_index = None;
                    temp
                }
            }
            None => None,
        }
    }
    
    /// Cancel navigation and return to original input
    pub fn cancel_navigation(&mut self) -> Option<String> {
        let temp = self.temp_input.take();
        self.navigation_index = None;
        temp
    }
}

/// Terminal usage statistics
#[derive(Debug, Default)]
pub struct TerminalStatistics {
    pub total_terminals: usize,
    pub running_terminals: usize,
    pub exited_terminals: usize,
    pub error_terminals: usize,
    pub total_lines: usize,
    pub total_memory_bytes: usize,
}

impl Default for TerminalManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.cleanup();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_manager_creation() {
        let manager = TerminalManager::new();
        assert_eq!(manager.terminals.len(), 0);
        assert!(manager.active_terminal.is_none());
    }

    #[test]
    fn test_terminal_creation() {
        let mut manager = TerminalManager::new();
        let terminal_id = manager.create_terminal(Some("Test Terminal".to_string()));
        
        assert_eq!(manager.terminals.len(), 1);
        assert_eq!(manager.active_terminal, Some(terminal_id));
        
        let terminal = manager.terminals.get(&terminal_id).unwrap();
        assert_eq!(terminal.name, "Test Terminal");
        assert_eq!(terminal.id, terminal_id);
    }

    #[test]
    fn test_terminal_buffer() {
        let mut buffer = TerminalBuffer::new(100, 1024);
        
        let line = TerminalLine {
            content: "Test output".to_string(),
            line_type: LineType::Normal,
            timestamp: Instant::now(),
            ends_with_newline: true,
            ansi_metadata: None,
        };
        
        buffer.add_line(line);
        assert_eq!(buffer.lines.len(), 1);
        assert_eq!(buffer.total_lines_added, 1);
    }

    #[test]
    fn test_input_history() {
        let mut history = InputHistory::new(10);
        
        history.add_command("ls -la".to_string());
        history.add_command("cd /tmp".to_string());
        
        assert_eq!(history.commands.len(), 2);
        
        // Test navigation
        let prev = history.previous("current");
        assert_eq!(prev, Some("cd /tmp".to_string()));
        
        let prev2 = history.previous("current");
        assert_eq!(prev2, Some("ls -la".to_string()));
    }

    #[test]
    fn test_shell_type_detection() {
        let terminal = Terminal::new(
            Uuid::new_v4(),
            "Test".to_string(),
            &TerminalSettings::default()
        );
        
        let (cmd, _) = terminal.get_shell_command();
        
        if cfg!(windows) {
            assert_eq!(cmd, "powershell.exe");
        } else {
            assert_eq!(cmd, "bash");
        }
    }
}