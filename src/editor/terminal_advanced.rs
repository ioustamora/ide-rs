//! Advanced Terminal Shell Integration
//!
//! Provides intelligent shell features including auto-completion, environment variable management,
//! integrated package managers, and advanced command history with contextual suggestions.

use egui::*;
use std::collections::{HashMap, VecDeque, HashSet};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{Receiver, Sender};
use serde::{Serialize, Deserialize};

/// Advanced terminal shell integration
pub struct AdvancedTerminal {
    /// Base terminal functionality
    pub terminal_id: usize,
    pub name: String,
    pub working_dir: PathBuf,
    pub current_input: String,
    
    /// Advanced shell features
    pub shell_integration: ShellIntegration,
    pub auto_completion: AutoCompletion,
    pub environment_manager: EnvironmentManager,
    pub command_intelligence: CommandIntelligence,
    pub package_integration: PackageIntegration,
    pub git_integration: GitIntegration,
    
    /// Advanced UI features
    pub split_panes: Vec<TerminalPane>,
    pub active_pane: usize,
    pub terminal_themes: Vec<TerminalTheme>,
    pub current_theme: String,
    
    /// Session management
    pub session_manager: SessionManager,
    pub bookmark_manager: BookmarkManager,
}

/// Enhanced shell integration with OS-specific features
#[derive(Clone, Serialize, Deserialize)]
pub struct ShellIntegration {
    pub shell_type: AdvancedShellType,
    pub shell_version: String,
    pub supported_features: HashSet<ShellFeature>,
    pub environment_variables: HashMap<String, String>,
    pub aliases: HashMap<String, String>,
    pub functions: HashMap<String, ShellFunction>,
    pub path_directories: Vec<PathBuf>,
    
    /// Advanced shell features
    pub prompt_customization: PromptCustomization,
    pub shell_hooks: ShellHooks,
    pub performance_monitoring: ShellPerformance,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AdvancedShellType {
    Bash { version: String, features: Vec<String> },
    Zsh { version: String, plugins: Vec<String> },
    PowerShell { version: String, modules: Vec<String> },
    Fish { version: String, functions: Vec<String> },
    Cmd { version: String },
    Nushell { version: String },
    Custom { name: String, command: String },
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ShellFeature {
    AutoCompletion,
    SyntaxHighlighting,
    HistorySearch,
    GitIntegration,
    EnvironmentVariables,
    Aliases,
    Functions,
    JobControl,
    PipeCompletion,
    CommandSubstitution,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ShellFunction {
    pub name: String,
    pub definition: String,
    pub description: String,
    pub parameters: Vec<String>,
}

/// Advanced auto-completion system
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct AutoCompletion {
    pub enabled: bool,
    pub completion_providers: Vec<CompletionProvider>,
    pub custom_completions: HashMap<String, Vec<String>>,
    pub fuzzy_matching: bool,
    pub context_aware: bool,
    
    /// Dynamic completion sources
    pub file_system_completion: bool,
    pub command_history_completion: bool,
    pub environment_variable_completion: bool,
    pub git_completion: bool,
    pub package_manager_completion: bool,
    
    /// Performance settings
    pub max_completion_results: usize,
    pub completion_timeout_ms: u32,
    pub cache_completions: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CompletionProvider {
    pub name: String,
    pub command_patterns: Vec<String>,
    pub completion_script: String,
    pub priority: u32,
}

/// Environment variable management
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct EnvironmentManager {
    /// Current environment variables
    pub variables: HashMap<String, EnvironmentVariable>,
    /// Environment profiles
    pub profiles: HashMap<String, EnvironmentProfile>,
    pub active_profile: Option<String>,
    
    /// Advanced features
    pub variable_validation: bool,
    pub auto_export: bool,
    pub variable_suggestions: Vec<VariableSuggestion>,
    pub secure_variables: HashSet<String>, // Variables that should be masked
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EnvironmentVariable {
    pub name: String,
    pub value: String,
    pub description: Option<String>,
    pub variable_type: VariableType,
    pub source: VariableSource,
    pub is_exported: bool,
    pub is_secure: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum VariableType {
    String,
    Path,
    Number,
    Boolean,
    List,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum VariableSource {
    System,
    User,
    Profile,
    Temporary,
    Project,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EnvironmentProfile {
    pub name: String,
    pub description: String,
    pub variables: HashMap<String, String>,
    pub path_additions: Vec<PathBuf>,
    pub aliases: HashMap<String, String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VariableSuggestion {
    pub variable_name: String,
    pub suggested_value: String,
    pub reason: String,
}

/// Intelligent command analysis and suggestions
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct CommandIntelligence {
    pub command_history: VecDeque<CommandHistoryEntry>,
    pub usage_statistics: HashMap<String, CommandUsageStats>,
    pub command_suggestions: Vec<CommandSuggestion>,
    pub error_analysis: Vec<CommandError>,
    
    /// AI-powered features
    pub smart_suggestions: bool,
    pub command_learning: bool,
    pub error_correction: bool,
    pub performance_warnings: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CommandHistoryEntry {
    pub command: String,
    pub working_directory: PathBuf,
    pub timestamp: String,
    pub exit_code: i32,
    pub execution_time_ms: u64,
    pub environment_snapshot: HashMap<String, String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CommandUsageStats {
    pub command: String,
    pub usage_count: u64,
    pub average_execution_time: f64,
    pub success_rate: f64,
    pub common_arguments: Vec<String>,
    pub common_contexts: Vec<PathBuf>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CommandSuggestion {
    pub suggested_command: String,
    pub reason: SuggestionReason,
    pub confidence: f32,
    pub context: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum SuggestionReason {
    FrequentlyUsed,
    SimilarContext,
    ErrorCorrection,
    BestPractice,
    PerformanceOptimization,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CommandError {
    pub command: String,
    pub error_message: String,
    pub suggested_fix: Option<String>,
    pub error_category: ErrorCategory,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ErrorCategory {
    SyntaxError,
    FileNotFound,
    PermissionDenied,
    CommandNotFound,
    InvalidArgument,
    NetworkError,
}

/// Package manager integration
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct PackageIntegration {
    pub detected_managers: Vec<PackageManager>,
    pub package_suggestions: Vec<PackageSuggestion>,
    pub dependency_tracking: bool,
    pub auto_update_check: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PackageManager {
    pub name: String,
    pub manager_type: PackageManagerType,
    pub config_file: Option<PathBuf>,
    pub packages: Vec<InstalledPackage>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum PackageManagerType {
    Npm,
    Yarn,
    Pip,
    Cargo,
    Apt,
    Yum,
    Brew,
    Chocolatey,
    Custom(String),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InstalledPackage {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub dependencies: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PackageSuggestion {
    pub package_name: String,
    pub reason: String,
    pub package_manager: PackageManagerType,
}

/// Git integration for terminals
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct GitIntegration {
    pub git_status: Option<GitStatus>,
    pub branch_info: Option<BranchInfo>,
    pub git_aliases: HashMap<String, String>,
    pub commit_templates: Vec<CommitTemplate>,
    pub auto_git_status: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GitStatus {
    pub current_branch: String,
    pub staged_files: Vec<String>,
    pub unstaged_files: Vec<String>,
    pub untracked_files: Vec<String>,
    pub commits_ahead: u32,
    pub commits_behind: u32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BranchInfo {
    pub current_branch: String,
    pub all_branches: Vec<String>,
    pub remote_branches: Vec<String>,
    pub upstream_branch: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CommitTemplate {
    pub name: String,
    pub template: String,
    pub description: String,
}

/// Terminal pane management for split views
#[derive(Clone, Serialize, Deserialize)]
pub struct TerminalPane {
    pub pane_id: usize,
    pub name: String,
    pub split_type: SplitType,
    pub size_ratio: f32,
    pub terminal_id: usize,
    pub is_active: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum SplitType {
    Horizontal,
    Vertical,
    Tab,
}

/// Terminal theming system
#[derive(Clone, Serialize, Deserialize)]
pub struct TerminalTheme {
    pub name: String,
    pub background_color: String,
    pub foreground_color: String,
    pub cursor_color: String,
    pub selection_color: String,
    pub ansi_colors: [String; 16],
    pub font_family: String,
    pub font_size: f32,
    pub line_height: f32,
}

/// Session management for terminal persistence
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct SessionManager {
    pub sessions: HashMap<String, TerminalSession>,
    pub auto_save: bool,
    pub session_timeout_hours: u32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TerminalSession {
    pub session_id: String,
    pub name: String,
    pub created_at: String,
    pub last_accessed: String,
    pub working_directory: PathBuf,
    pub environment_variables: HashMap<String, String>,
    pub command_history: Vec<String>,
    pub open_files: Vec<PathBuf>,
}

/// Bookmark management for quick directory navigation
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct BookmarkManager {
    pub bookmarks: HashMap<String, DirectoryBookmark>,
    pub recent_directories: VecDeque<PathBuf>,
    pub auto_bookmark_frequent: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DirectoryBookmark {
    pub name: String,
    pub path: PathBuf,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub access_count: u64,
    pub created_at: String,
}

/// Prompt customization
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct PromptCustomization {
    pub custom_prompt: Option<String>,
    pub show_git_status: bool,
    pub show_current_time: bool,
    pub show_execution_time: bool,
    pub show_exit_code: bool,
    pub prompt_segments: Vec<PromptSegment>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PromptSegment {
    pub segment_type: PromptSegmentType,
    pub format: String,
    pub color: String,
    pub enabled: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum PromptSegmentType {
    CurrentDirectory,
    GitBranch,
    GitStatus,
    Time,
    ExitCode,
    UserName,
    HostName,
    Custom(String),
}

/// Shell hooks for advanced integration
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct ShellHooks {
    pub pre_command_hooks: Vec<Hook>,
    pub post_command_hooks: Vec<Hook>,
    pub directory_change_hooks: Vec<Hook>,
    pub environment_change_hooks: Vec<Hook>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Hook {
    pub name: String,
    pub command: String,
    pub conditions: Vec<HookCondition>,
    pub enabled: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct HookCondition {
    pub condition_type: HookConditionType,
    pub pattern: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum HookConditionType {
    CommandMatch,
    DirectoryMatch,
    EnvironmentVariable,
    FileExists,
}

/// Performance monitoring for shell operations
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct ShellPerformance {
    pub monitor_enabled: bool,
    pub command_timings: HashMap<String, Vec<f64>>,
    pub slow_command_threshold_ms: f64,
    pub memory_usage_tracking: bool,
    pub performance_alerts: Vec<PerformanceAlert>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    pub alert_type: PerformanceAlertType,
    pub command: String,
    pub metric_value: f64,
    pub threshold: f64,
    pub timestamp: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum PerformanceAlertType {
    SlowCommand,
    HighMemoryUsage,
    FrequentErrors,
    LongRunningProcess,
}

impl AdvancedTerminal {
    /// Create a new advanced terminal instance
    pub fn new(id: usize, name: String, working_dir: PathBuf) -> Self {
        Self {
            terminal_id: id,
            name,
            working_dir,
            current_input: String::new(),
            
            shell_integration: ShellIntegration::default(),
            auto_completion: AutoCompletion::default(),
            environment_manager: EnvironmentManager::default(),
            command_intelligence: CommandIntelligence::default(),
            package_integration: PackageIntegration::default(),
            git_integration: GitIntegration::default(),
            
            split_panes: Vec::new(),
            active_pane: 0,
            terminal_themes: Self::create_default_themes(),
            current_theme: "default".to_string(),
            
            session_manager: SessionManager::default(),
            bookmark_manager: BookmarkManager::default(),
        }
    }
    
    /// Initialize shell integration based on detected shell
    pub fn initialize_shell_integration(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Detect shell type and version
        self.detect_shell_capabilities()?;
        
        // Load environment variables
        self.load_environment_variables()?;
        
        // Initialize auto-completion
        self.initialize_auto_completion()?;
        
        // Set up git integration
        self.initialize_git_integration()?;
        
        // Detect package managers
        self.detect_package_managers()?;
        
        Ok(())
    }
    
    /// Detect shell capabilities and features
    fn detect_shell_capabilities(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // This would detect the actual shell and its capabilities
        // For now, using placeholder implementation
        self.shell_integration.supported_features.insert(ShellFeature::AutoCompletion);
        self.shell_integration.supported_features.insert(ShellFeature::HistorySearch);
        self.shell_integration.supported_features.insert(ShellFeature::GitIntegration);
        
        Ok(())
    }
    
    /// Load environment variables with intelligent categorization
    fn load_environment_variables(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Load system environment variables
        for (key, value) in std::env::vars() {
            let variable = EnvironmentVariable {
                name: key.clone(),
                value,
                description: None,
                variable_type: VariableType::String,
                source: VariableSource::System,
                is_exported: true,
                is_secure: Self::is_secure_variable(&key),
            };
            
            self.environment_manager.variables.insert(key, variable);
        }
        
        Ok(())
    }
    
    /// Check if a variable should be treated as secure
    fn is_secure_variable(var_name: &str) -> bool {
        let secure_patterns = vec![
            "PASSWORD", "SECRET", "KEY", "TOKEN", "AUTH", "CREDENTIAL",
            "PRIVATE", "CONFIDENTIAL", "SECURE"
        ];
        
        secure_patterns.iter().any(|pattern| {
            var_name.to_uppercase().contains(pattern)
        })
    }
    
    /// Initialize auto-completion system
    fn initialize_auto_completion(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.auto_completion.enabled = true;
        self.auto_completion.file_system_completion = true;
        self.auto_completion.command_history_completion = true;
        self.auto_completion.environment_variable_completion = true;
        self.auto_completion.git_completion = true;
        
        Ok(())
    }
    
    /// Initialize git integration
    fn initialize_git_integration(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.git_integration.auto_git_status = true;
        
        // Add common git aliases
        self.git_integration.git_aliases.insert("st".to_string(), "status".to_string());
        self.git_integration.git_aliases.insert("co".to_string(), "checkout".to_string());
        self.git_integration.git_aliases.insert("br".to_string(), "branch".to_string());
        self.git_integration.git_aliases.insert("ci".to_string(), "commit".to_string());
        
        Ok(())
    }
    
    /// Detect available package managers
    fn detect_package_managers(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // This would detect actual package managers in the system
        // Placeholder implementation
        Ok(())
    }
    
    /// Get intelligent command suggestions based on context
    pub fn get_command_suggestions(&self, partial_command: &str) -> Vec<CommandSuggestion> {
        let mut suggestions = Vec::new();
        
        // Add suggestions based on command history
        // Add suggestions based on current directory context
        // Add suggestions based on git status
        // Add suggestions based on package.json/Cargo.toml/etc.
        
        suggestions
    }
    
    /// Create default terminal themes
    fn create_default_themes() -> Vec<TerminalTheme> {
        vec![
            TerminalTheme {
                name: "default".to_string(),
                background_color: "#1e1e1e".to_string(),
                foreground_color: "#d4d4d4".to_string(),
                cursor_color: "#ffffff".to_string(),
                selection_color: "#264f78".to_string(),
                ansi_colors: [
                    "#000000".to_string(), "#cd3131".to_string(), "#0dbc79".to_string(), "#e5e510".to_string(),
                    "#2472c8".to_string(), "#bc3fbc".to_string(), "#11a8cd".to_string(), "#e5e5e5".to_string(),
                    "#666666".to_string(), "#f14c4c".to_string(), "#23d18b".to_string(), "#f5f543".to_string(),
                    "#3b8eea".to_string(), "#d670d6".to_string(), "#29b8db".to_string(), "#e5e5e5".to_string(),
                ],
                font_family: "Cascadia Code".to_string(),
                font_size: 14.0,
                line_height: 1.2,
            },
            TerminalTheme {
                name: "dracula".to_string(),
                background_color: "#282a36".to_string(),
                foreground_color: "#f8f8f2".to_string(),
                cursor_color: "#f8f8f0".to_string(),
                selection_color: "#44475a".to_string(),
                ansi_colors: [
                    "#000000".to_string(), "#ff5555".to_string(), "#50fa7b".to_string(), "#f1fa8c".to_string(),
                    "#bd93f9".to_string(), "#ff79c6".to_string(), "#8be9fd".to_string(), "#bfbfbf".to_string(),
                    "#4d4d4d".to_string(), "#ff6e67".to_string(), "#5af78e".to_string(), "#f4f99d".to_string(),
                    "#caa9fa".to_string(), "#ff92d0".to_string(), "#9aedfe".to_string(), "#e6e6e6".to_string(),
                ],
                font_family: "Fira Code".to_string(),
                font_size: 14.0,
                line_height: 1.2,
            },
        ]
    }
    
    /// Render advanced terminal UI
    pub fn render_advanced_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Terminal tabs and panes
            self.render_terminal_tabs(ui);
            
            // Main terminal area with potential splits
            ui.separator();
            self.render_terminal_panes(ui);
            
            // Advanced input area with auto-completion
            ui.separator();
            self.render_advanced_input(ui);
            
            // Status bar with git and environment info
            ui.separator();
            self.render_status_bar(ui);
        });
    }
    
    /// Render terminal tabs
    fn render_terminal_tabs(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Terminal:");
            
            // Tab for each pane
            for (i, pane) in self.split_panes.iter().enumerate() {
                let is_active = i == self.active_pane;
                if ui.selectable_label(is_active, &pane.name).clicked() {
                    self.active_pane = i;
                }
            }
            
            // Add new pane button
            if ui.button("➕").clicked() {
                // Add new terminal pane
                self.add_terminal_pane();
            }
        });
    }
    
    /// Add a new terminal pane
    fn add_terminal_pane(&mut self) {
        let pane = TerminalPane {
            pane_id: self.split_panes.len(),
            name: format!("Terminal {}", self.split_panes.len() + 1),
            split_type: SplitType::Tab,
            size_ratio: 1.0,
            terminal_id: self.terminal_id,
            is_active: false,
        };
        
        self.split_panes.push(pane);
    }
    
    /// Render terminal panes
    fn render_terminal_panes(&mut self, ui: &mut egui::Ui) {
        // This would render the actual terminal content
        // For now, placeholder
        ui.group(|ui| {
            ui.heading("Advanced Terminal Output");
            ui.label("Terminal content would be rendered here with:");
            ui.label("• Syntax highlighting");
            ui.label("• Clickable links and file paths");
            ui.label("• Integrated file preview");
            ui.label("• Command execution status");
            ui.label("• Git status indicators");
        });
    }
    
    /// Render advanced input with auto-completion
    fn render_advanced_input(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("$");
            
            let input_response = ui.text_edit_singleline(&mut self.current_input);
            
            // Show auto-completion popup if enabled and input has content
            if self.auto_completion.enabled && !self.current_input.is_empty() {
                self.show_completion_popup(ui);
            }
            
            if ui.button("Execute").clicked() || input_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                self.execute_command();
            }
        });
        
        // Command suggestions
        if !self.current_input.is_empty() {
            let suggestions = self.get_command_suggestions(&self.current_input);
            if !suggestions.is_empty() {
                ui.group(|ui| {
                    ui.label("💡 Suggestions:");
                    for suggestion in suggestions.iter().take(3) {
                        ui.horizontal(|ui| {
                            ui.label(&suggestion.suggested_command);
                            ui.small(format!("({})", suggestion.reason as u8));
                        });
                    }
                });
            }
        }
    }
    
    /// Show auto-completion popup
    fn show_completion_popup(&mut self, ui: &mut egui::Ui) {
        // This would show an intelligent completion popup
        // Placeholder implementation
    }
    
    /// Execute command with advanced intelligence
    fn execute_command(&mut self) {
        if !self.current_input.trim().is_empty() {
            // Add to command history with context
            let history_entry = CommandHistoryEntry {
                command: self.current_input.clone(),
                working_directory: self.working_dir.clone(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                exit_code: 0, // Would be set after execution
                execution_time_ms: 0, // Would be measured
                environment_snapshot: std::env::vars().collect(),
            };
            
            self.command_intelligence.command_history.push_back(history_entry);
            
            // Clear input
            self.current_input.clear();
        }
    }
    
    /// Render status bar with advanced information
    fn render_status_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Current directory
            ui.label(format!("📁 {}", self.working_dir.display()));
            
            ui.separator();
            
            // Git status
            if let Some(ref git_status) = self.git_integration.git_status {
                ui.label(format!("🌿 {}", git_status.current_branch));
                if git_status.staged_files.len() > 0 {
                    ui.label(format!("📝 {}", git_status.staged_files.len()));
                }
            }
            
            ui.separator();
            
            // Environment profile
            if let Some(ref profile) = self.environment_manager.active_profile {
                ui.label(format!("🔧 {}", profile));
            }
            
            ui.separator();
            
            // Theme selector
            egui::ComboBox::from_label("Theme")
                .selected_text(&self.current_theme)
                .show_ui(ui, |ui| {
                    for theme in &self.terminal_themes {
                        ui.selectable_value(&mut self.current_theme, theme.name.clone(), &theme.name);
                    }
                });
        });
    }
}

impl Default for ShellIntegration {
    fn default() -> Self {
        Self {
            shell_type: AdvancedShellType::Custom { 
                name: "default".to_string(), 
                command: "cmd".to_string() 
            },
            shell_version: "1.0".to_string(),
            supported_features: HashSet::new(),
            environment_variables: HashMap::new(),
            aliases: HashMap::new(),
            functions: HashMap::new(),
            path_directories: Vec::new(),
            prompt_customization: PromptCustomization::default(),
            shell_hooks: ShellHooks::default(),
            performance_monitoring: ShellPerformance::default(),
        }
    }
}