//! Integrated Terminal System
//!
//! Provides embedded terminal/console functionality within the IDE,
//! supporting multiple shells, command history, and process management.

use egui::*;
use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

/// Terminal manager handling multiple terminal instances
pub struct TerminalManager {
    /// Active terminal instances
    pub terminals: HashMap<usize, Terminal>,
    /// Currently active terminal
    pub active_terminal: Option<usize>,
    /// Next terminal ID
    pub next_terminal_id: usize,
    /// Terminal settings
    pub settings: TerminalSettings,
    /// UI state
    pub ui_state: TerminalUIState,
}

/// Individual terminal instance
pub struct Terminal {
    /// Terminal ID
    pub id: usize,
    /// Terminal name/title
    pub name: String,
    /// Working directory
    pub working_dir: PathBuf,
    /// Shell process
    pub process: Option<Child>,
    /// Output buffer
    pub output_buffer: VecDeque<TerminalLine>,
    /// Input history
    pub input_history: VecDeque<String>,
    /// Current input
    pub current_input: String,
    /// History navigation index
    pub history_index: Option<usize>,
    /// Terminal state
    pub state: TerminalState,
    /// Shell type
    pub shell_type: ShellType,
    /// Process output receiver
    pub output_receiver: Option<Receiver<String>>,
    /// Process input sender
    pub input_sender: Option<Sender<String>>,
    /// Scrollback limit
    pub scrollback_limit: usize,
    /// Auto-scroll to bottom
    pub auto_scroll: bool,
}

/// Terminal line with styling information
#[derive(Debug, Clone)]
pub struct TerminalLine {
    /// Line content
    pub content: String,
    /// Line type for coloring
    pub line_type: TerminalLineType,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
    /// Whether this is user input or output
    pub is_input: bool,
}

/// Type of terminal line for styling
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalLineType {
    /// Regular output
    Output,
    /// Error output
    Error,
    /// Warning
    Warning,
    /// User input/command
    Input,
    /// System message
    System,
    /// Success message
    Success,
}

/// Terminal execution state
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalState {
    /// Terminal is ready for input
    Ready,
    /// Command is running
    Running,
    /// Terminal is starting up
    Starting,
    /// Terminal encountered an error
    Error(String),
    /// Terminal is being closed
    Closing,
}

/// Supported shell types
#[derive(Debug, Clone, PartialEq)]
pub enum ShellType {
    /// Windows Command Prompt
    Cmd,
    /// Windows PowerShell
    PowerShell,
    /// PowerShell Core (cross-platform)
    PowerShellCore,
    /// Bash (Unix/Linux/WSL)
    Bash,
    /// Zsh (Unix/Linux/macOS)
    Zsh,
    /// Fish shell
    Fish,
    /// Custom shell command
    Custom(String),
}

/// Terminal configuration settings
#[derive(Debug, Clone)]
pub struct TerminalSettings {
    /// Default shell
    pub default_shell: ShellType,
    /// Font size
    pub font_size: f32,
    /// Background color
    pub background_color: Color32,
    /// Text color
    pub text_color: Color32,
    /// Error text color
    pub error_color: Color32,
    /// Success text color
    pub success_color: Color32,
    /// Input text color
    pub input_color: Color32,
    /// Maximum scrollback lines
    pub max_scrollback: usize,
    /// Command history size
    pub max_history: usize,
    /// Auto-complete commands
    pub enable_autocomplete: bool,
    /// Show timestamps
    pub show_timestamps: bool,
    /// Enable ANSI color codes
    pub enable_ansi_colors: bool,
}

/// Terminal UI state
pub struct TerminalUIState {
    /// Show new terminal dialog
    pub show_new_terminal_dialog: bool,
    /// Selected shell for new terminal
    pub new_terminal_shell: ShellType,
    /// New terminal working directory
    pub new_terminal_dir: String,
    /// Show terminal settings
    pub show_settings: bool,
    /// Show command palette
    pub show_command_palette: bool,
    /// Command palette input
    pub command_palette_input: String,
    /// Available commands for palette
    pub available_commands: Vec<TerminalCommand>,
}

/// Terminal command for command palette
#[derive(Debug, Clone)]
pub struct TerminalCommand {
    /// Command name
    pub name: String,
    /// Command description
    pub description: String,
    /// Command to execute
    pub command: String,
    /// Working directory (optional)
    pub working_dir: Option<PathBuf>,
    /// Shell type (optional, uses default if None)
    pub shell: Option<ShellType>,
}

impl Default for ShellType {
    fn default() -> Self {
        #[cfg(windows)]
        return ShellType::PowerShell;
        
        #[cfg(not(windows))]
        return ShellType::Bash;
    }
}

impl Default for TerminalSettings {
    fn default() -> Self {
        Self {
            default_shell: ShellType::default(),
            font_size: 12.0,
            background_color: Color32::from_gray(20),
            text_color: Color32::from_gray(240),
            error_color: Color32::from_rgb(255, 100, 100),
            success_color: Color32::from_rgb(100, 255, 100),
            input_color: Color32::from_rgb(100, 200, 255),
            max_scrollback: 10000,
            max_history: 1000,
            enable_autocomplete: true,
            show_timestamps: false,
            enable_ansi_colors: true,
        }
    }
}

impl TerminalManager {
    /// Create new terminal manager
    pub fn new() -> Self {
        let mut manager = Self {
            terminals: HashMap::new(),
            active_terminal: None,
            next_terminal_id: 1,
            settings: TerminalSettings::default(),
            ui_state: TerminalUIState::default(),
        };

        // Initialize with common commands
        manager.ui_state.available_commands = vec![
            TerminalCommand {
                name: "Build Project".to_string(),
                description: "Build the current Rust project".to_string(),
                command: "cargo build".to_string(),
                working_dir: None,
                shell: None,
            },
            TerminalCommand {
                name: "Run Project".to_string(),
                description: "Run the current Rust project".to_string(),
                command: "cargo run".to_string(),
                working_dir: None,
                shell: None,
            },
            TerminalCommand {
                name: "Run Tests".to_string(),
                description: "Run all tests in the project".to_string(),
                command: "cargo test".to_string(),
                working_dir: None,
                shell: None,
            },
            TerminalCommand {
                name: "Git Status".to_string(),
                description: "Show Git repository status".to_string(),
                command: "git status".to_string(),
                working_dir: None,
                shell: None,
            },
        ];

        manager
    }

    /// Create a new terminal
    pub fn create_terminal(&mut self, name: String, working_dir: PathBuf, shell_type: ShellType) -> Result<usize, TerminalError> {
        let terminal_id = self.next_terminal_id;
        self.next_terminal_id += 1;

        let mut terminal = Terminal {
            id: terminal_id,
            name,
            working_dir: working_dir.clone(),
            process: None,
            output_buffer: VecDeque::new(),
            input_history: VecDeque::new(),
            current_input: String::new(),
            history_index: None,
            state: TerminalState::Starting,
            shell_type: shell_type.clone(),
            output_receiver: None,
            input_sender: None,
            scrollback_limit: self.settings.max_scrollback,
            auto_scroll: true,
        };

        // Start the shell process
        if let Err(e) = self.start_shell_process(&mut terminal) {
            return Err(e);
        }

        self.terminals.insert(terminal_id, terminal);
        self.active_terminal = Some(terminal_id);

        Ok(terminal_id)
    }

    /// Start shell process for terminal
    fn start_shell_process(&self, terminal: &mut Terminal) -> Result<(), TerminalError> {
        let (shell_cmd, shell_args) = self.get_shell_command(&terminal.shell_type)?;
        
        let mut process = Command::new(&shell_cmd)
            .args(&shell_args)
            .current_dir(&terminal.working_dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| TerminalError::ProcessStartFailed(e.to_string()))?;

        // Set up communication channels
        let (output_tx, output_rx) = std::sync::mpsc::channel();
        let (input_tx, input_rx) = std::sync::mpsc::channel();

        terminal.output_receiver = Some(output_rx);
        terminal.input_sender = Some(input_tx);

        // Spawn thread to handle process I/O
        if let Some(stdout) = process.stdout.take() {
            let output_tx_clone = output_tx.clone();
            thread::spawn(move || {
                use std::io::{BufRead, BufReader};
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    match line {
                        Ok(line_content) => {
                            if output_tx_clone.send(line_content).is_err() {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
            });
        }

        if let Some(stderr) = process.stderr.take() {
            let output_tx_clone = output_tx;
            thread::spawn(move || {
                use std::io::{BufRead, BufReader};
                let reader = BufReader::new(stderr);
                for line in reader.lines() {
                    match line {
                        Ok(line_content) => {
                            let error_line = format!("[ERROR] {}", line_content);
                            if output_tx_clone.send(error_line).is_err() {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
            });
        }

        // Handle input
        if let Some(mut stdin) = process.stdin.take() {
            thread::spawn(move || {
                use std::io::Write;
                while let Ok(input) = input_rx.recv() {
                    if stdin.write_all(input.as_bytes()).is_err() {
                        break;
                    }
                    if stdin.write_all(b"\n").is_err() {
                        break;
                    }
                    if stdin.flush().is_err() {
                        break;
                    }
                }
            });
        }

        terminal.process = Some(process);
        terminal.state = TerminalState::Ready;

        Ok(())
    }

    /// Get shell command and arguments
    fn get_shell_command(&self, shell_type: &ShellType) -> Result<(String, Vec<String>), TerminalError> {
        match shell_type {
            ShellType::Cmd => Ok(("cmd".to_string(), vec!["/k".to_string()])),
            ShellType::PowerShell => Ok(("powershell".to_string(), vec!["-NoExit".to_string()])),
            ShellType::PowerShellCore => Ok(("pwsh".to_string(), vec!["-NoExit".to_string()])),
            ShellType::Bash => Ok(("bash".to_string(), vec!["--login".to_string()])),
            ShellType::Zsh => Ok(("zsh".to_string(), vec!["-l".to_string()])),
            ShellType::Fish => Ok(("fish".to_string(), vec![])),
            ShellType::Custom(cmd) => {
                let parts: Vec<String> = cmd.split_whitespace().map(|s| s.to_string()).collect();
                if parts.is_empty() {
                    return Err(TerminalError::InvalidShellCommand(cmd.clone()));
                }
                Ok((parts[0].clone(), parts[1..].to_vec()))
            }
        }
    }

    /// Execute command in active terminal
    pub fn execute_command(&mut self, command: String) -> Result<(), TerminalError> {
        let terminal_id = self.active_terminal
            .ok_or(TerminalError::NoActiveTerminal)?;

        if let Some(terminal) = self.terminals.get_mut(&terminal_id) {
            // Add to history
            if !command.trim().is_empty() {
                terminal.input_history.push_back(command.clone());
                if terminal.input_history.len() > self.settings.max_history {
                    terminal.input_history.pop_front();
                }
            }

            // Add command to output buffer
            terminal.output_buffer.push_back(TerminalLine {
                content: format!("> {}", command),
                line_type: TerminalLineType::Input,
                timestamp: std::time::SystemTime::now(),
                is_input: true,
            });

            // Send command to process
            if let Some(ref input_sender) = terminal.input_sender {
                input_sender.send(command)
                    .map_err(|_| TerminalError::ProcessCommunicationError)?;
            }

            terminal.state = TerminalState::Running;
            Ok(())
        } else {
            Err(TerminalError::TerminalNotFound(terminal_id))
        }
    }

    /// Update terminal outputs
    pub fn update_terminals(&mut self) {
        let terminal_ids: Vec<_> = self.terminals.keys().cloned().collect();
        
        for terminal_id in terminal_ids {
            if let Some(terminal) = self.terminals.get_mut(&terminal_id) {
                // Check for new output
                if let Some(ref output_receiver) = terminal.output_receiver {
                    while let Ok(output) = output_receiver.try_recv() {
                        let line_type = if output.starts_with("[ERROR]") {
                            TerminalLineType::Error
                        } else {
                            TerminalLineType::Output
                        };

                        terminal.output_buffer.push_back(TerminalLine {
                            content: output,
                            line_type,
                            timestamp: std::time::SystemTime::now(),
                            is_input: false,
                        });

                        // Limit scrollback
                        if terminal.output_buffer.len() > terminal.scrollback_limit {
                            terminal.output_buffer.pop_front();
                        }
                    }
                }

                // Update terminal state
                if terminal.state == TerminalState::Running {
                    if let Some(ref mut process) = terminal.process {
                        match process.try_wait() {
                            Ok(Some(_)) => {
                                terminal.state = TerminalState::Ready;
                            }
                            Ok(None) => {
                                // Process still running
                            }
                            Err(_) => {
                                terminal.state = TerminalState::Error("Process error".to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    /// Close terminal
    pub fn close_terminal(&mut self, terminal_id: usize) -> Result<(), TerminalError> {
        if let Some(mut terminal) = self.terminals.remove(&terminal_id) {
            terminal.state = TerminalState::Closing;
            
            // Kill process if still running
            if let Some(ref mut process) = terminal.process {
                let _ = process.kill();
            }

            // Update active terminal
            if self.active_terminal == Some(terminal_id) {
                self.active_terminal = self.terminals.keys().next().cloned();
            }

            Ok(())
        } else {
            Err(TerminalError::TerminalNotFound(terminal_id))
        }
    }

    /// Render terminal UI
    pub fn render_terminal_ui(&mut self, ui: &mut Ui) {
        // Terminal tabs
        ui.horizontal(|ui| {
            let terminal_ids: Vec<_> = self.terminals.keys().cloned().collect();
            for terminal_id in terminal_ids {
                if let Some(terminal) = self.terminals.get(&terminal_id) {
                    let _is_active = self.active_terminal == Some(terminal_id);
                    let _tab_open = true;
                    
                    ui.selectable_value(&mut self.active_terminal, Some(terminal_id), &terminal.name);
                    
                    // Close button
                    if ui.small_button("×").clicked() {
                        let _ = self.close_terminal(terminal_id);
                        return;
                    }
                }
            }

            if ui.button("+ New Terminal").clicked() {
                self.ui_state.show_new_terminal_dialog = true;
            }
        });

        ui.separator();

        // Active terminal content
        if let Some(terminal_id) = self.active_terminal {
            // Extract the data we need without cloning the entire terminal
            let (output_lines, current_input) = if let Some(terminal) = self.terminals.get(&terminal_id) {
                (terminal.output_buffer.iter().collect::<Vec<_>>(), terminal.current_input.clone())
            } else {
                (vec![], String::new())
            };
            
            // Display terminal output
            ui.vertical(|ui| {
                ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for line in output_lines {
                            let color = match line.line_type {
                                TerminalLineType::Output => Color32::WHITE,
                                TerminalLineType::Input => Color32::LIGHT_BLUE,
                                TerminalLineType::Error => Color32::RED,
                                TerminalLineType::Warning => Color32::YELLOW,
                                TerminalLineType::System => Color32::GRAY,
                                TerminalLineType::Success => Color32::GREEN,
                            };
                            ui.colored_label(color, &line.content);
                        }
                    });
            });
        } else {
            ui.centered_and_justified(|ui| {
                ui.heading("No terminal active");
                if ui.button("Create New Terminal").clicked() {
                    self.ui_state.show_new_terminal_dialog = true;
                }
            });
        }

        // New terminal dialog
        if self.ui_state.show_new_terminal_dialog {
            let mut show_dialog = true;
            let mut new_terminal_dir = self.ui_state.new_terminal_dir.clone();
            let mut new_terminal_shell = self.ui_state.new_terminal_shell.clone();
            let mut should_create = false;
            let mut should_cancel = false;
            
            egui::Window::new("New Terminal")
                .open(&mut show_dialog)
                .show(ui.ctx(), |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Working directory:");
                        ui.text_edit_singleline(&mut new_terminal_dir);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Shell:");
                        egui::ComboBox::from_label("")
                            .selected_text(format!("{:?}", new_terminal_shell))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut new_terminal_shell, ShellType::Bash, "Bash");
                                ui.selectable_value(&mut new_terminal_shell, ShellType::PowerShell, "PowerShell");
                                ui.selectable_value(&mut new_terminal_shell, ShellType::Cmd, "Command Prompt");
                            });
                    });

                    ui.horizontal(|ui| {
                        if ui.button("Create").clicked() {
                            should_create = true;
                        }
                        if ui.button("Cancel").clicked() {
                            should_cancel = true;
                        }
                    });
                });
                
            // Handle actions outside closure
            if should_create {
                let working_dir = if new_terminal_dir.is_empty() {
                    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
                } else {
                    PathBuf::from(&new_terminal_dir)
                };

                let name = format!("Terminal {}", self.next_terminal_id);
                if let Ok(_) = self.create_terminal(name, working_dir, new_terminal_shell.clone()) {
                    self.ui_state.show_new_terminal_dialog = false;
                    self.ui_state.new_terminal_dir.clear();
                }
            } else if should_cancel || !show_dialog {
                self.ui_state.show_new_terminal_dialog = false;
            } else {
                self.ui_state.new_terminal_dir = new_terminal_dir;
                self.ui_state.new_terminal_shell = new_terminal_shell;
            }
        }
    }

    /// Render individual terminal content
    fn render_terminal_content(&mut self, ui: &mut Ui, terminal: &mut Terminal) {
        // Terminal output area
        let available_height = ui.available_height() - 30.0; // Reserve space for input
        
        ScrollArea::vertical()
            .max_height(available_height)
            .stick_to_bottom(terminal.auto_scroll)
            .show(ui, |ui| {
                for line in &terminal.output_buffer {
                    let color = match line.line_type {
                        TerminalLineType::Output => self.settings.text_color,
                        TerminalLineType::Error => self.settings.error_color,
                        TerminalLineType::Input => self.settings.input_color,
                        TerminalLineType::Success => self.settings.success_color,
                        TerminalLineType::Warning => Color32::YELLOW,
                        TerminalLineType::System => Color32::GRAY,
                    };

                    ui.horizontal(|ui| {
                        if self.settings.show_timestamps {
                            if let Ok(elapsed) = line.timestamp.elapsed() {
                                ui.label(format!("[{:02}:{:02}]", elapsed.as_secs() / 60, elapsed.as_secs() % 60));
                            }
                        }
                        ui.colored_label(color, &line.content);
                    });
                }
            });

        // Input area
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("$");
            let input_response = ui.text_edit_singleline(&mut terminal.current_input);
            
            if input_response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                let command = terminal.current_input.clone();
                terminal.current_input.clear();
                if let Err(_) = self.execute_command(command) {
                    // Handle error
                }
                ui.memory_mut(|m| m.request_focus(input_response.id));
            }

            // Handle history navigation
            if input_response.has_focus() {
                if ui.input(|i| i.key_pressed(Key::ArrowUp)) {
                    if let Some(index) = terminal.history_index {
                        if index > 0 {
                            terminal.history_index = Some(index - 1);
                        }
                    } else {
                        terminal.history_index = Some(terminal.input_history.len().saturating_sub(1));
                    }
                    
                    if let Some(index) = terminal.history_index {
                        if let Some(cmd) = terminal.input_history.get(index) {
                            terminal.current_input = cmd.clone();
                        }
                    }
                }
                
                if ui.input(|i| i.key_pressed(Key::ArrowDown)) {
                    if let Some(index) = terminal.history_index {
                        if index < terminal.input_history.len() - 1 {
                            terminal.history_index = Some(index + 1);
                            if let Some(cmd) = terminal.input_history.get(index + 1) {
                                terminal.current_input = cmd.clone();
                            }
                        } else {
                            terminal.history_index = None;
                            terminal.current_input.clear();
                        }
                    }
                }
            }

            if ui.button("Execute").clicked() {
                let command = terminal.current_input.clone();
                terminal.current_input.clear();
                let _ = self.execute_command(command);
            }

            if ui.button("Clear").clicked() {
                terminal.output_buffer.clear();
            }
        });
    }

    /// Quick execute command without creating persistent terminal
    pub fn quick_execute(&self, command: &str, working_dir: &Path) -> Result<String, TerminalError> {
        let output = Command::new(self.get_default_shell_command())
            .arg("-c")
            .arg(command)
            .current_dir(working_dir)
            .output()
            .map_err(|e| TerminalError::ProcessStartFailed(e.to_string()))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(TerminalError::CommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string()
            ))
        }
    }

    /// Get default shell command for quick execution
    fn get_default_shell_command(&self) -> &str {
        match self.settings.default_shell {
            ShellType::Bash => "bash",
            ShellType::Zsh => "zsh", 
            ShellType::Fish => "fish",
            ShellType::PowerShell | ShellType::PowerShellCore => "powershell",
            ShellType::Cmd => "cmd",
            ShellType::Custom(ref cmd) => cmd.split_whitespace().next().unwrap_or("sh"),
        }
    }
}

/// Terminal system errors
#[derive(Debug, thiserror::Error)]
pub enum TerminalError {
    #[error("No active terminal")]
    NoActiveTerminal,
    #[error("Terminal not found: {0}")]
    TerminalNotFound(usize),
    #[error("Failed to start process: {0}")]
    ProcessStartFailed(String),
    #[error("Process communication error")]
    ProcessCommunicationError,
    #[error("Invalid shell command: {0}")]
    InvalidShellCommand(String),
    #[error("Command failed: {0}")]
    CommandFailed(String),
}

impl Default for TerminalManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TerminalUIState {
    fn default() -> Self {
        Self {
            show_new_terminal_dialog: false,
            new_terminal_shell: ShellType::default(),
            new_terminal_dir: String::new(),
            show_settings: false,
            show_command_palette: false,
            command_palette_input: String::new(),
            available_commands: Vec::new(),
        }
    }
}