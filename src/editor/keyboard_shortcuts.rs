//! Enhanced Keyboard Shortcuts and Hotkey System
//!
//! This module provides a comprehensive keyboard shortcut system with customizable
//! key bindings, command palettes, and context-aware hotkeys inspired by modern
//! IDEs like VS Code and JetBrains products.

use egui::{Key, Modifiers, Context, Event};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Main keyboard shortcut manager
#[derive(Debug, Clone)]
pub struct KeyboardShortcutManager {
    /// Registered shortcuts
    shortcuts: HashMap<String, KeyboardShortcut>,
    /// Key sequence buffer for multi-key shortcuts
    sequence_buffer: Vec<KeyPress>,
    /// Sequence timeout in milliseconds
    sequence_timeout: u64,
    /// Last key press timestamp
    last_key_time: std::time::Instant,
    /// Currently active context
    active_context: ShortcutContext,
    /// Command palette state
    command_palette: CommandPalette,
    /// Custom user shortcuts
    user_shortcuts: HashMap<String, KeyboardShortcut>,
    /// Shortcut enabled state
    enabled: bool,
}

/// Individual keyboard shortcut definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardShortcut {
    /// Unique command identifier
    pub command_id: String,
    /// Human-readable description
    pub description: String,
    /// Key combination
    pub key_combination: KeyCombination,
    /// Context where this shortcut is active
    pub context: ShortcutContext,
    /// Whether shortcut is enabled
    pub enabled: bool,
    /// Custom user-defined shortcut
    pub is_custom: bool,
}

/// Key combination definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct KeyCombination {
    /// Primary key
    pub key: Key,
    /// Required modifiers
    pub modifiers: ModifierSet,
    /// Optional sequence keys for multi-key shortcuts
    pub sequence: Vec<KeyPress>,
}

/// Modifier key set
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ModifierSet {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub cmd: bool, // Command key on macOS
}

/// Individual key press
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct KeyPress {
    pub key: Key,
    pub modifiers: ModifierSet,
}

/// Shortcut context for context-sensitive shortcuts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ShortcutContext {
    Global,
    Editor,
    VisualDesigner,
    ProjectBrowser,
    ToolWindow,
    DebugMode,
    CommandPalette,
    Modal,
    Custom(String),
}

/// Command palette for discovering and executing commands
#[derive(Debug, Clone)]
pub struct CommandPalette {
    /// Whether command palette is open
    pub is_open: bool,
    /// Current search query
    pub search_query: String,
    /// Filtered commands
    pub filtered_commands: Vec<Command>,
    /// Selected command index
    pub selected_index: usize,
    /// All available commands
    pub all_commands: Vec<Command>,
    /// Recently used commands
    pub recent_commands: Vec<String>,
}

/// Executable command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    /// Unique command identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Command description
    pub description: String,
    /// Command category
    pub category: CommandCategory,
    /// Associated keyboard shortcut
    pub shortcut: Option<KeyCombination>,
    /// Command tags for better searching
    pub tags: Vec<String>,
    /// Usage frequency for ranking
    pub usage_count: usize,
}

/// Command categories for organization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CommandCategory {
    File,
    Edit,
    View,
    Navigate,
    Build,
    Debug,
    Tools,
    Help,
    Custom(String),
}

impl Default for KeyboardShortcutManager {
    fn default() -> Self {
        let mut manager = Self {
            shortcuts: HashMap::new(),
            sequence_buffer: Vec::new(),
            sequence_timeout: 1000, // 1 second
            last_key_time: std::time::Instant::now(),
            active_context: ShortcutContext::Global,
            command_palette: CommandPalette::new(),
            user_shortcuts: HashMap::new(),
            enabled: true,
        };
        
        manager.initialize_default_shortcuts();
        manager.initialize_default_commands();
        manager
    }
}

impl KeyboardShortcutManager {
    /// Create a new keyboard shortcut manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Initialize default keyboard shortcuts following VS Code conventions
    fn initialize_default_shortcuts(&mut self) {
        let shortcuts = vec![
            // File operations
            ("file.new", "New File", Key::N, ModifierSet::ctrl(), ShortcutContext::Global),
            ("file.open", "Open File", Key::O, ModifierSet::ctrl(), ShortcutContext::Global),
            ("file.save", "Save File", Key::S, ModifierSet::ctrl(), ShortcutContext::Global),
            ("file.save_as", "Save As", Key::S, ModifierSet::ctrl_shift(), ShortcutContext::Global),
            ("file.close", "Close File", Key::W, ModifierSet::ctrl(), ShortcutContext::Global),
        ];

        for (id, desc, key, modifiers, context) in shortcuts {
            let shortcut = KeyboardShortcut {
                command_id: id.to_string(),
                description: desc.to_string(),
                key_combination: KeyCombination {
                    key,
                    modifiers,
                    sequence: Vec::new(),
                },
                context,
                enabled: true,
                is_custom: false,
            };
            self.shortcuts.insert(id.to_string(), shortcut);
        }
    }

    /// Initialize default commands for command palette
    fn initialize_default_commands(&mut self) {
        let commands = vec![
            ("file.new", "New File", "Create a new file", CommandCategory::File),
            ("file.open", "Open File", "Open an existing file", CommandCategory::File),
            ("file.save", "Save File", "Save the current file", CommandCategory::File),
        ];

        for (id, name, desc, category) in commands {
            let command = Command {
                id: id.to_string(),
                name: name.to_string(),
                description: desc.to_string(),
                category,
                shortcut: self.shortcuts.get(id).map(|s| s.key_combination.clone()),
                tags: vec![name.to_lowercase(), desc.to_lowercase()],
                usage_count: 0,
            };
            self.command_palette.all_commands.push(command);
        }
    }

    /// Handle keyboard input and execute shortcuts
    pub fn handle_input(&mut self, _ctx: &Context) -> Vec<String> {
        // Simplified implementation
        Vec::new()
    }

    /// Toggle command palette
    pub fn toggle_command_palette(&mut self) {
        self.command_palette.is_open = !self.command_palette.is_open;
    }

    /// Set the active context
    pub fn set_context(&mut self, context: ShortcutContext) {
        self.active_context = context;
    }
}

impl CommandPalette {
    fn new() -> Self {
        Self {
            is_open: false,
            search_query: String::new(),
            filtered_commands: Vec::new(),
            selected_index: 0,
            all_commands: Vec::new(),
            recent_commands: Vec::new(),
        }
    }
}

impl ModifierSet {
    fn ctrl() -> Self {
        Self { ctrl: true, alt: false, shift: false, cmd: false }
    }

    fn ctrl_shift() -> Self {
        Self { ctrl: true, alt: false, shift: true, cmd: false }
    }
}