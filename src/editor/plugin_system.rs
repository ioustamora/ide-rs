//! Plugin/Extension System
//!
//! Provides a flexible plugin architecture for extending IDE functionality
//! with custom components, commands, language support, and integrations.

use egui::*;
use std::collections::{HashMap, BTreeMap};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

/// Plugin system manager
pub struct PluginManager {
    /// Loaded plugins
    pub plugins: HashMap<String, Plugin>,
    /// Plugin registry (installed but not necessarily loaded)
    pub registry: PluginRegistry,
    /// Plugin hooks and event handlers
    pub hooks: PluginHooks,
    /// Plugin settings
    pub settings: PluginSettings,
    /// Plugin marketplace connection
    pub marketplace: PluginMarketplace,
}

/// Individual plugin instance
pub struct Plugin {
    /// Plugin metadata
    pub metadata: PluginMetadata,
    /// Plugin state
    pub state: PluginState,
    /// Plugin configuration
    pub config: PluginConfig,
    /// Extension points provided by this plugin
    pub extension_points: Vec<ExtensionPoint>,
    /// Commands registered by this plugin
    pub commands: Vec<PluginCommand>,
    /// Plugin API interface
    pub api: Box<dyn PluginAPI>,
}

/// Plugin metadata from manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    /// Plugin unique identifier
    pub id: String,
    /// Plugin name
    pub name: String,
    /// Plugin version
    pub version: String,
    /// Plugin description
    pub description: String,
    /// Plugin author
    pub author: String,
    /// Plugin website/repository
    pub website: Option<String>,
    /// Plugin license
    pub license: String,
    /// Plugin keywords/tags
    pub keywords: Vec<String>,
    /// Minimum IDE version required
    pub min_ide_version: String,
    /// Plugin dependencies
    pub dependencies: Vec<PluginDependency>,
    /// Plugin entry point
    pub entry_point: String,
    /// Plugin icon
    pub icon: Option<String>,
    /// Plugin category
    pub category: PluginCategory,
}

/// Plugin dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    /// Dependency plugin ID
    pub id: String,
    /// Required version (semver)
    pub version: String,
    /// Whether dependency is optional
    pub optional: bool,
}

/// Plugin categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginCategory {
    /// Language support
    Language,
    /// Code editing enhancements
    Editor,
    /// UI/UX themes and customization
    Theme,
    /// Version control integration
    VersionControl,
    /// Build and deployment tools
    Build,
    /// Debugging tools
    Debugging,
    /// File management
    FileManagement,
    /// Code analysis and linting
    Analysis,
    /// Documentation tools
    Documentation,
    /// Testing frameworks
    Testing,
    /// Productivity tools
    Productivity,
    /// Integration with external services
    Integration,
    /// Utility and miscellaneous
    Utility,
}

/// Plugin execution state
#[derive(Debug, Clone, PartialEq)]
pub enum PluginState {
    /// Plugin is not loaded
    Unloaded,
    /// Plugin is loading
    Loading,
    /// Plugin is active and running
    Active,
    /// Plugin is paused/disabled
    Paused,
    /// Plugin encountered an error
    Error(String),
    /// Plugin is being unloaded
    Unloading,
}

/// Plugin configuration
#[derive(Debug, Clone)]
pub struct PluginConfig {
    /// Plugin-specific settings
    pub settings: BTreeMap<String, PluginSettingValue>,
    /// Plugin enabled state
    pub enabled: bool,
    /// Auto-load on IDE startup
    pub auto_load: bool,
    /// Plugin priority (for load order)
    pub priority: i32,
}

/// Plugin setting value types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginSettingValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<PluginSettingValue>),
    Object(BTreeMap<String, PluginSettingValue>),
}

/// Extension point for plugin functionality
pub struct ExtensionPoint {
    /// Extension point ID
    pub id: String,
    /// Extension point type
    pub point_type: ExtensionPointType,
    /// Extension implementation
    pub implementation: Box<dyn Extension>,
}

/// Types of extension points
#[derive(Debug, Clone, PartialEq)]
pub enum ExtensionPointType {
    /// Language syntax provider
    LanguageProvider,
    /// Code completion provider
    CompletionProvider,
    /// Diagnostics provider
    DiagnosticsProvider,
    /// Formatter provider
    FormatterProvider,
    /// Code action provider
    CodeActionProvider,
    /// Theme provider
    ThemeProvider,
    /// Command provider
    CommandProvider,
    /// File system provider
    FileSystemProvider,
    /// Build task provider
    BuildTaskProvider,
    /// Debug adapter
    DebugAdapter,
    /// Git integration
    GitIntegration,
    /// UI widget provider
    WidgetProvider,
    /// Settings panel provider
    SettingsProvider,
}

/// Extension trait for plugin functionality
pub trait Extension: Send + Sync {
    /// Initialize the extension
    fn initialize(&mut self, context: &ExtensionContext) -> Result<(), PluginError>;
    
    /// Activate the extension
    fn activate(&mut self) -> Result<(), PluginError>;
    
    /// Deactivate the extension
    fn deactivate(&mut self) -> Result<(), PluginError>;
    
    /// Handle extension-specific requests
    fn handle_request(&mut self, request: ExtensionRequest) -> Result<ExtensionResponse, PluginError>;
}

/// Extension context provided to plugins
pub struct ExtensionContext {
    /// Plugin ID
    pub plugin_id: String,
    /// IDE version
    pub ide_version: String,
    /// Plugin configuration
    pub config: PluginConfig,
    /// Available APIs
    pub apis: HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
}

/// Extension request/response system
#[derive(Debug, Clone)]
pub enum ExtensionRequest {
    /// Get completion items
    GetCompletions {
        file_path: PathBuf,
        position: Position,
        context: String,
    },
    /// Format code
    FormatCode {
        file_path: PathBuf,
        content: String,
        options: FormatOptions,
    },
    /// Get diagnostics
    GetDiagnostics {
        file_path: PathBuf,
        content: String,
    },
    /// Execute command
    ExecuteCommand {
        command: String,
        args: Vec<String>,
    },
    /// Custom request with plugin-specific data
    Custom {
        request_type: String,
        data: serde_json::Value,
    },
}

/// Extension response
#[derive(Debug, Clone)]
pub enum ExtensionResponse {
    /// Completion items response
    Completions(Vec<CompletionItem>),
    /// Formatted code response
    FormattedCode(String),
    /// Diagnostics response
    Diagnostics(Vec<Diagnostic>),
    /// Command execution result
    CommandResult(String),
    /// Custom response
    Custom(serde_json::Value),
}

/// Plugin command definition
pub struct PluginCommand {
    /// Command ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Command description
    pub description: String,
    /// Keyboard shortcut (optional)
    pub shortcut: Option<String>,
    /// Command handler
    pub handler: Box<dyn CommandHandler>,
}

/// Command handler trait
pub trait CommandHandler: Send + Sync {
    /// Execute the command
    fn execute(&mut self, context: &CommandContext) -> Result<(), PluginError>;
    
    /// Check if command can be executed
    fn can_execute(&self, context: &CommandContext) -> bool;
}

/// Command execution context
pub struct CommandContext {
    /// Active file path
    pub active_file: Option<PathBuf>,
    /// Current selection
    pub selection: Option<String>,
    /// Cursor position
    pub cursor_position: Option<Position>,
    /// Plugin configuration
    pub config: PluginConfig,
}

/// Plugin registry for managing installed plugins
pub struct PluginRegistry {
    /// Installed plugins
    pub installed: HashMap<String, PluginMetadata>,
    /// Plugin directory
    pub plugin_directory: PathBuf,
    /// Registry cache
    pub cache: RegistryCache,
}

/// Registry cache for performance
#[derive(Default)]
pub struct RegistryCache {
    /// Last scan time
    pub last_scan: Option<std::time::SystemTime>,
    /// Cached plugin list
    pub cached_plugins: Vec<PluginMetadata>,
    /// Cache validity duration in seconds
    pub cache_duration: u64,
}

/// Plugin hooks for IDE events
pub struct PluginHooks {
    /// File open hooks
    pub file_opened: Vec<Box<dyn FileEventHandler>>,
    /// File save hooks
    pub file_saved: Vec<Box<dyn FileEventHandler>>,
    /// File close hooks
    pub file_closed: Vec<Box<dyn FileEventHandler>>,
    /// Editor selection changed hooks
    pub selection_changed: Vec<Box<dyn SelectionEventHandler>>,
    /// Build started hooks
    pub build_started: Vec<Box<dyn BuildEventHandler>>,
    /// Build completed hooks
    pub build_completed: Vec<Box<dyn BuildEventHandler>>,
    /// Git commit hooks
    pub git_commit: Vec<Box<dyn GitEventHandler>>,
}

/// File event handler trait
pub trait FileEventHandler: Send + Sync {
    fn handle_file_event(&mut self, event: FileEvent) -> Result<(), PluginError>;
}

/// Selection event handler trait
pub trait SelectionEventHandler: Send + Sync {
    fn handle_selection_event(&mut self, event: SelectionEvent) -> Result<(), PluginError>;
}

/// Build event handler trait
pub trait BuildEventHandler: Send + Sync {
    fn handle_build_event(&mut self, event: BuildEvent) -> Result<(), PluginError>;
}

/// Git event handler trait
pub trait GitEventHandler: Send + Sync {
    fn handle_git_event(&mut self, event: GitEvent) -> Result<(), PluginError>;
}

/// File events
#[derive(Debug, Clone)]
pub struct FileEvent {
    pub file_path: PathBuf,
    pub event_type: FileEventType,
    pub timestamp: std::time::SystemTime,
}

/// File event types
#[derive(Debug, Clone, PartialEq)]
pub enum FileEventType {
    Opened,
    Saved,
    Closed,
    Modified,
    Renamed { old_path: PathBuf },
    Deleted,
}

/// Selection events
#[derive(Debug, Clone)]
pub struct SelectionEvent {
    pub file_path: PathBuf,
    pub selection: String,
    pub range: Option<TextRange>,
}

/// Build events
#[derive(Debug, Clone)]
pub struct BuildEvent {
    pub project_path: PathBuf,
    pub event_type: BuildEventType,
    pub details: String,
}

/// Build event types
#[derive(Debug, Clone, PartialEq)]
pub enum BuildEventType {
    Started,
    Completed { success: bool },
    Failed { error: String },
}

/// Git events
#[derive(Debug, Clone)]
pub struct GitEvent {
    pub repository_path: PathBuf,
    pub event_type: GitEventType,
    pub details: String,
}

/// Git event types
#[derive(Debug, Clone, PartialEq)]
pub enum GitEventType {
    Commit { hash: String },
    Branch { name: String },
    Push,
    Pull,
    Merge { branch: String },
}

/// Plugin settings management
#[derive(Default)]
pub struct PluginSettings {
    /// Global plugin settings
    pub global_settings: BTreeMap<String, PluginSettingValue>,
    /// Per-plugin settings
    pub plugin_settings: HashMap<String, BTreeMap<String, PluginSettingValue>>,
    /// Auto-load enabled plugins
    pub auto_load_plugins: Vec<String>,
}

/// Plugin marketplace integration
pub struct PluginMarketplace {
    /// Marketplace URL
    pub marketplace_url: String,
    /// Available plugins from marketplace
    pub available_plugins: Vec<MarketplacePlugin>,
    /// Update cache
    pub update_cache: HashMap<String, String>, // plugin_id -> latest_version
}

/// Marketplace plugin information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplacePlugin {
    /// Plugin metadata
    pub metadata: PluginMetadata,
    /// Download URL
    pub download_url: String,
    /// Plugin size in bytes
    pub size: u64,
    /// Download count
    pub downloads: u64,
    /// User rating (0.0 - 5.0)
    pub rating: f32,
    /// Number of ratings
    pub rating_count: u32,
    /// Last updated timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Plugin API trait for runtime interaction
pub trait PluginAPI: Send + Sync {
    /// Get plugin metadata
    fn get_metadata(&self) -> &PluginMetadata;
    
    /// Initialize plugin
    fn initialize(&mut self, context: ExtensionContext) -> Result<(), PluginError>;
    
    /// Activate plugin
    fn activate(&mut self) -> Result<(), PluginError>;
    
    /// Deactivate plugin
    fn deactivate(&mut self) -> Result<(), PluginError>;
    
    /// Handle plugin requests
    fn handle_request(&mut self, request: serde_json::Value) -> Result<serde_json::Value, PluginError>;
    
    /// Render plugin UI (if applicable)
    fn render_ui(&mut self, ui: &mut Ui) -> Result<(), PluginError>;
    
    /// Get plugin settings schema
    fn get_settings_schema(&self) -> Result<serde_json::Value, PluginError>;
    
    /// Update plugin settings
    fn update_settings(&mut self, settings: BTreeMap<String, PluginSettingValue>) -> Result<(), PluginError>;
}

// Helper types from other modules
use crate::editor::lsp_integration::{CompletionItem, Diagnostic, Position};

/// Text range for selections
#[derive(Debug, Clone)]
pub struct TextRange {
    pub start: Position,
    pub end: Position,
}

/// Format options for code formatting
#[derive(Debug, Clone)]
pub struct FormatOptions {
    pub tab_size: u32,
    pub insert_spaces: bool,
    pub trim_trailing_whitespace: bool,
}

impl PluginManager {
    /// Create new plugin manager
    pub fn new() -> Self {
        let plugin_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ide-plugins");

        Self {
            plugins: HashMap::new(),
            registry: PluginRegistry {
                installed: HashMap::new(),
                plugin_directory: plugin_dir,
                cache: RegistryCache {
                    last_scan: None,
                    cached_plugins: Vec::new(),
                    cache_duration: 300, // 5 minutes
                },
            },
            hooks: PluginHooks::default(),
            settings: PluginSettings::default(),
            marketplace: PluginMarketplace {
                marketplace_url: "https://ide-plugins.example.com".to_string(),
                available_plugins: Vec::new(),
                update_cache: HashMap::new(),
            },
        }
    }

    /// Scan for installed plugins
    pub fn scan_plugins(&mut self) -> Result<(), PluginError> {
        if !self.registry.plugin_directory.exists() {
            std::fs::create_dir_all(&self.registry.plugin_directory)
                .map_err(|e| PluginError::FileSystemError(e.to_string()))?;
        }

        self.registry.installed.clear();

        for entry in std::fs::read_dir(&self.registry.plugin_directory)
            .map_err(|e| PluginError::FileSystemError(e.to_string()))? 
        {
            let entry = entry.map_err(|e| PluginError::FileSystemError(e.to_string()))?;
            let path = entry.path();

            if path.is_dir() {
                if let Ok(metadata) = self.load_plugin_metadata(&path) {
                    self.registry.installed.insert(metadata.id.clone(), metadata);
                }
            }
        }

        self.registry.cache.last_scan = Some(std::time::SystemTime::now());
        Ok(())
    }

    /// Load plugin metadata from manifest
    fn load_plugin_metadata(&self, plugin_path: &Path) -> Result<PluginMetadata, PluginError> {
        let manifest_path = plugin_path.join("plugin.json");
        let manifest_content = std::fs::read_to_string(manifest_path)
            .map_err(|e| PluginError::FileSystemError(e.to_string()))?;
            
        let metadata: PluginMetadata = serde_json::from_str(&manifest_content)
            .map_err(|e| PluginError::SerializationError(e.to_string()))?;
            
        Ok(metadata)
    }

    /// Load a plugin
    pub fn load_plugin(&mut self, plugin_id: &str) -> Result<(), PluginError> {
        if self.plugins.contains_key(plugin_id) {
            return Err(PluginError::PluginAlreadyLoaded(plugin_id.to_string()));
        }

        let metadata = self.registry.installed.get(plugin_id)
            .ok_or_else(|| PluginError::PluginNotFound(plugin_id.to_string()))?
            .clone();

        // Check dependencies
        self.check_dependencies(&metadata)?;

        // Create plugin instance
        let plugin = self.create_plugin_instance(metadata)?;
        
        // Initialize plugin
        // Note: This would typically involve loading dynamic libraries or WASM modules
        // For now, we'll use a placeholder implementation

        self.plugins.insert(plugin_id.to_string(), plugin);
        Ok(())
    }

    /// Create plugin instance (placeholder implementation)
    fn create_plugin_instance(&self, metadata: PluginMetadata) -> Result<Plugin, PluginError> {
        // This would typically load the plugin from a dynamic library or WASM module
        // For now, create a basic placeholder
        
        Ok(Plugin {
            metadata: metadata.clone(),
            state: PluginState::Unloaded,
            config: PluginConfig {
                settings: BTreeMap::new(),
                enabled: true,
                auto_load: false,
                priority: 0,
            },
            extension_points: Vec::new(),
            commands: Vec::new(),
            api: Box::new(PlaceholderPluginAPI { metadata }),
        })
    }

    /// Check plugin dependencies
    fn check_dependencies(&self, metadata: &PluginMetadata) -> Result<(), PluginError> {
        for dependency in &metadata.dependencies {
            if !dependency.optional {
                if !self.registry.installed.contains_key(&dependency.id) {
                    return Err(PluginError::DependencyNotFound(dependency.id.clone()));
                }
                
                // TODO: Check version compatibility
            }
        }
        Ok(())
    }

    /// Unload a plugin
    pub fn unload_plugin(&mut self, plugin_id: &str) -> Result<(), PluginError> {
        if let Some(mut plugin) = self.plugins.remove(plugin_id) {
            plugin.state = PluginState::Unloading;
            let _ = plugin.api.deactivate();
            Ok(())
        } else {
            Err(PluginError::PluginNotFound(plugin_id.to_string()))
        }
    }

    /// Activate a plugin
    pub fn activate_plugin(&mut self, plugin_id: &str) -> Result<(), PluginError> {
        if let Some(plugin) = self.plugins.get_mut(plugin_id) {
            plugin.api.activate()?;
            plugin.state = PluginState::Active;
            Ok(())
        } else {
            Err(PluginError::PluginNotFound(plugin_id.to_string()))
        }
    }

    /// Deactivate a plugin
    pub fn deactivate_plugin(&mut self, plugin_id: &str) -> Result<(), PluginError> {
        if let Some(plugin) = self.plugins.get_mut(plugin_id) {
            plugin.api.deactivate()?;
            plugin.state = PluginState::Paused;
            Ok(())
        } else {
            Err(PluginError::PluginNotFound(plugin_id.to_string()))
        }
    }

    /// Execute plugin command
    pub fn execute_plugin_command(&mut self, command_id: &str) -> Result<(), PluginError> {
        // Find plugin that owns this command
        for plugin in self.plugins.values_mut() {
            for command in &mut plugin.commands {
                if command.id == command_id {
                    let context = CommandContext {
                        active_file: None,
                        selection: None,
                        cursor_position: None,
                        config: plugin.config.clone(),
                    };
                    return command.handler.execute(&context);
                }
            }
        }
        Err(PluginError::CommandNotFound(command_id.to_string()))
    }

    /// Render plugin management UI
    pub fn render_plugin_manager_ui(&mut self, ui: &mut Ui) {
        ui.heading("Plugin Manager");

        ui.horizontal(|ui| {
            if ui.button("🔄 Refresh").clicked() {
                let _ = self.scan_plugins();
            }
            if ui.button("🌐 Marketplace").clicked() {
                // TODO: Open marketplace
            }
            if ui.button("📁 Open Plugin Folder").clicked() {
                // TODO: Open plugin directory in file manager
            }
        });

        ui.separator();

        // Installed plugins
        ui.collapsing("Installed Plugins", |ui| {
            let plugins_data: Vec<_> = self.plugins.iter().map(|(id, plugin)| {
                (id.clone(), plugin.metadata.name.clone(), plugin.metadata.version.clone(), plugin.state.clone())
            }).collect();
            
            for (plugin_id, name, version, state) in plugins_data {
                ui.horizontal(|ui| {
                    ui.label(&name);
                    ui.label(&version);
                    
                    match state {
                        PluginState::Active => {
                            ui.colored_label(Color32::GREEN, "Active");
                            if ui.small_button("Deactivate").clicked() {
                                let _ = self.deactivate_plugin(&plugin_id);
                            }
                        }
                        PluginState::Paused => {
                            ui.colored_label(Color32::YELLOW, "Paused");
                            if ui.small_button("Activate").clicked() {
                                let _ = self.activate_plugin(&plugin_id);
                            }
                        }
                        PluginState::Error(ref error) => {
                            ui.colored_label(Color32::RED, format!("Error: {}", error));
                        }
                        _ => {
                            ui.colored_label(Color32::GRAY, format!("{:?}", state));
                        }
                    }
                    
                    if ui.small_button("Unload").clicked() {
                        let _ = self.unload_plugin(&plugin_id);
                    }
                });
            }
        });

        // Available plugins
        ui.collapsing("Available Plugins", |ui| {
            let loaded_plugin_ids: std::collections::HashSet<_> = self.plugins.keys().collect();
            let available_plugins: Vec<_> = self.registry.installed.values()
                .filter(|metadata| !loaded_plugin_ids.contains(&metadata.id))
                .map(|metadata| (metadata.id.clone(), metadata.name.clone(), metadata.version.clone(), metadata.description.clone()))
                .collect();
                
            for (id, name, version, description) in available_plugins {
                ui.horizontal(|ui| {
                    ui.label(&name);
                    ui.label(&version);
                    ui.label(&description);
                    
                    if ui.small_button("Load").clicked() {
                        let _ = self.load_plugin(&id);
                    }
                });
            }
        });
    }
}

impl Default for PluginHooks {
    fn default() -> Self {
        Self {
            file_opened: Vec::new(),
            file_saved: Vec::new(),
            file_closed: Vec::new(),
            selection_changed: Vec::new(),
            build_started: Vec::new(),
            build_completed: Vec::new(),
            git_commit: Vec::new(),
        }
    }
}

/// Placeholder plugin API implementation
pub struct PlaceholderPluginAPI {
    metadata: PluginMetadata,
}

impl PluginAPI for PlaceholderPluginAPI {
    fn get_metadata(&self) -> &PluginMetadata {
        &self.metadata
    }
    
    fn initialize(&mut self, _context: ExtensionContext) -> Result<(), PluginError> {
        Ok(())
    }
    
    fn activate(&mut self) -> Result<(), PluginError> {
        Ok(())
    }
    
    fn deactivate(&mut self) -> Result<(), PluginError> {
        Ok(())
    }
    
    fn handle_request(&mut self, _request: serde_json::Value) -> Result<serde_json::Value, PluginError> {
        Ok(serde_json::Value::Null)
    }
    
    fn render_ui(&mut self, _ui: &mut Ui) -> Result<(), PluginError> {
        Ok(())
    }
    
    fn get_settings_schema(&self) -> Result<serde_json::Value, PluginError> {
        Ok(serde_json::Value::Object(serde_json::Map::new()))
    }
    
    fn update_settings(&mut self, _settings: BTreeMap<String, PluginSettingValue>) -> Result<(), PluginError> {
        Ok(())
    }
}

/// Plugin system errors
#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("Plugin not found: {0}")]
    PluginNotFound(String),
    #[error("Plugin already loaded: {0}")]
    PluginAlreadyLoaded(String),
    #[error("Dependency not found: {0}")]
    DependencyNotFound(String),
    #[error("Command not found: {0}")]
    CommandNotFound(String),
    #[error("File system error: {0}")]
    FileSystemError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Plugin initialization failed: {0}")]
    InitializationFailed(String),
    #[error("Plugin execution error: {0}")]
    ExecutionError(String),
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}