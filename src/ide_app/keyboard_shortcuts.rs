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

/// Shortcut execution result
#[derive(Debug, Clone)]
pub enum ShortcutResult {
    Executed(String),
    NotFound,
    Disabled,
    WrongContext,
    SequenceInProgress,
}

/// Predefined keyboard shortcuts following industry standards
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
            ("file.save_all", "Save All", Key::S, ModifierSet::ctrl_alt(), ShortcutContext::Global),
            ("file.close", "Close File", Key::W, ModifierSet::ctrl(), ShortcutContext::Global),
            ("file.close_all", "Close All", Key::W, ModifierSet::ctrl_shift(), ShortcutContext::Global),
            
            // Edit operations
            ("edit.undo", "Undo", Key::Z, ModifierSet::ctrl(), ShortcutContext::Editor),
            ("edit.redo", "Redo", Key::Y, ModifierSet::ctrl(), ShortcutContext::Editor),
            ("edit.cut", "Cut", Key::X, ModifierSet::ctrl(), ShortcutContext::Editor),
            ("edit.copy", "Copy", Key::C, ModifierSet::ctrl(), ShortcutContext::Editor),
            ("edit.paste", "Paste", Key::V, ModifierSet::ctrl(), ShortcutContext::Editor),
            ("edit.select_all", "Select All", Key::A, ModifierSet::ctrl(), ShortcutContext::Editor),
            ("edit.find", "Find", Key::F, ModifierSet::ctrl(), ShortcutContext::Editor),
            ("edit.replace", "Find and Replace", Key::H, ModifierSet::ctrl(), ShortcutContext::Editor),
            ("edit.duplicate_line", "Duplicate Line", Key::D, ModifierSet::ctrl_shift(), ShortcutContext::Editor),
            ("edit.delete_line", "Delete Line", Key::K, ModifierSet::ctrl_shift(), ShortcutContext::Editor),
            ("edit.move_line_up", "Move Line Up", Key::ArrowUp, ModifierSet::alt(), ShortcutContext::Editor),
            ("edit.move_line_down", "Move Line Down", Key::ArrowDown, ModifierSet::alt(), ShortcutContext::Editor),
            
            // Navigation
            ("nav.go_to_line", "Go to Line", Key::G, ModifierSet::ctrl(), ShortcutContext::Editor),
            ("nav.go_to_definition", "Go to Definition", Key::F12, ModifierSet::none(), ShortcutContext::Editor),
            ("nav.go_back", "Go Back", Key::ArrowLeft, ModifierSet::alt(), ShortcutContext::Global),
            ("nav.go_forward", "Go Forward", Key::ArrowRight, ModifierSet::alt(), ShortcutContext::Global),
            ("nav.quick_open", "Quick Open", Key::P, ModifierSet::ctrl(), ShortcutContext::Global),
            ("nav.command_palette", "Command Palette", Key::P, ModifierSet::ctrl_shift(), ShortcutContext::Global),
            
            // View operations
            ("view.toggle_sidebar", "Toggle Sidebar", Key::B, ModifierSet::ctrl(), ShortcutContext::Global),
            ("view.toggle_terminal", "Toggle Terminal", Key::Backtick, ModifierSet::ctrl(), ShortcutContext::Global),
            ("view.toggle_explorer", "Toggle Explorer", Key::E, ModifierSet::ctrl_shift(), ShortcutContext::Global),
            ("view.zoom_in", "Zoom In", Key::Equals, ModifierSet::ctrl(), ShortcutContext::Global),
            ("view.zoom_out", "Zoom Out", Key::Minus, ModifierSet::ctrl(), ShortcutContext::Global),
            ("view.zoom_reset", "Reset Zoom", Key::Num0, ModifierSet::ctrl(), ShortcutContext::Global),
            
            // Build and run
            ("build.run", "Run Project", Key::F5, ModifierSet::none(), ShortcutContext::Global),
            ("build.debug", "Debug Project", Key::F5, ModifierSet::shift(), ShortcutContext::Global),
            ("build.build", "Build Project", Key::B, ModifierSet::ctrl_shift(), ShortcutContext::Global),
            ("build.clean", "Clean Project", Key::K, ModifierSet::ctrl_alt(), ShortcutContext::Global),
            
            // Visual Designer specific
            ("designer.align_left", "Align Left", Key::L, ModifierSet::ctrl_alt(), ShortcutContext::VisualDesigner),
            ("designer.align_center", "Align Center", Key::E, ModifierSet::ctrl_alt(), ShortcutContext::VisualDesigner),
            ("designer.align_right", "Align Right", Key::R, ModifierSet::ctrl_alt(), ShortcutContext::VisualDesigner),
            ("designer.distribute_horizontal", "Distribute Horizontally", Key::H, ModifierSet::ctrl_shift(), ShortcutContext::VisualDesigner),
            ("designer.distribute_vertical", "Distribute Vertically", Key::V, ModifierSet::ctrl_shift(), ShortcutContext::VisualDesigner),
            ("designer.bring_to_front", "Bring to Front", Key::F, ModifierSet::ctrl_shift(), ShortcutContext::VisualDesigner),
            ("designer.send_to_back", "Send to Back", Key::B, ModifierSet::ctrl_shift(), ShortcutContext::VisualDesigner),
            
            // Multi-key sequences (inspired by Emacs/Vim)
            ("workspace.switch_file", "Switch Between Files", Key::Tab, ModifierSet::ctrl(), ShortcutContext::Global),
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
            ("edit.format_document", "Format Document", "Format the entire document", CommandCategory::Edit),
            ("view.toggle_word_wrap", "Toggle Word Wrap", "Toggle word wrapping in editor", CommandCategory::View),
            ("nav.go_to_symbol", "Go to Symbol", "Navigate to a symbol in the current file", CommandCategory::Navigate),
            ("build.run_task", "Run Task", "Run a build task", CommandCategory::Build),
            ("debug.start", "Start Debugging", "Start a debugging session", CommandCategory::Debug),
            ("tools.settings", "Open Settings", "Open IDE settings", CommandCategory::Tools),
            ("help.show_all_commands", "Show All Commands", "Display all available commands", CommandCategory::Help),
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
    pub fn handle_input(&mut self, ctx: &Context) -> Vec<String> {
        if !self.enabled {
            return Vec::new();
        }

        let mut executed_commands = Vec::new();

        ctx.input(|i| {
            for event in &i.events {
                if let Event::Key { key, pressed, modifiers, .. } = event {
                    if *pressed {
                        let key_press = KeyPress {
                            key: *key,
                            modifiers: ModifierSet::from_egui_modifiers(*modifiers),
                        };

                        // Check for direct shortcuts first
                        if let Some(command) = self.find_matching_shortcut(&key_press) {
                            if self.can_execute_in_context(&command.context) {
                                executed_commands.push(command.command_id.clone());
                                self.sequence_buffer.clear();
                                continue;
                            }
                        }

                        // Handle multi-key sequences
                        self.handle_sequence_input(key_press, &mut executed_commands);
                    }
                }
            }
        });

        // Clear sequence buffer if timeout exceeded
        if self.last_key_time.elapsed().as_millis() > self.sequence_timeout as u128 {
            self.sequence_buffer.clear();
        }

        executed_commands
    }

    /// Find matching shortcut for a key press
    fn find_matching_shortcut(&self, key_press: &KeyPress) -> Option<&KeyboardShortcut> {
        self.shortcuts.values().find(|shortcut| {
            shortcut.enabled &&
            shortcut.key_combination.key == key_press.key &&
            shortcut.key_combination.modifiers == key_press.modifiers &&
            shortcut.key_combination.sequence.is_empty()
        })
    }

    /// Handle multi-key sequence input
    fn handle_sequence_input(&mut self, key_press: KeyPress, executed_commands: &mut Vec<String>) {
        self.sequence_buffer.push(key_press);
        self.last_key_time = std::time::Instant::now();

        // Check for matching sequences
        for shortcut in self.shortcuts.values() {
            if shortcut.enabled && 
               !shortcut.key_combination.sequence.is_empty() &&
               self.sequence_matches(&shortcut.key_combination) {
                if self.can_execute_in_context(&shortcut.context) {
                    executed_commands.push(shortcut.command_id.clone());
                    self.sequence_buffer.clear();
                    return;
                }
            }
        }

        // Keep buffer reasonable size
        if self.sequence_buffer.len() > 5 {
            self.sequence_buffer.remove(0);
        }
    }

    /// Check if current sequence matches a shortcut
    fn sequence_matches(&self, combination: &KeyCombination) -> bool {
        if self.sequence_buffer.len() < combination.sequence.len() + 1 {
            return false;
        }

        // Check main key first
        let main_key = KeyPress {
            key: combination.key,
            modifiers: combination.modifiers.clone(),
        };

        let buffer_end = self.sequence_buffer.len();
        let sequence_start = buffer_end - combination.sequence.len() - 1;

        if self.sequence_buffer[buffer_end - 1] != main_key {
            return false;
        }

        // Check sequence keys
        for (i, expected_key) in combination.sequence.iter().enumerate() {
            if self.sequence_buffer[sequence_start + i] != *expected_key {
                return false;
            }
        }

        true
    }

    /// Check if shortcut can execute in current context
    fn can_execute_in_context(&self, context: &ShortcutContext) -> bool {
        match context {
            ShortcutContext::Global => true,
            _ => context == &self.active_context,
        }
    }

    /// Set the active context
    pub fn set_context(&mut self, context: ShortcutContext) {
        self.active_context = context;
    }

    /// Toggle command palette
    pub fn toggle_command_palette(&mut self) {
        self.command_palette.is_open = !self.command_palette.is_open;
        if self.command_palette.is_open {
            self.command_palette.search_query.clear();
            self.command_palette.selected_index = 0;
            self.command_palette.update_filtered_commands();
        }
    }

    /// Render command palette UI
    pub fn render_command_palette(&mut self, ctx: &Context) -> Option<String> {
        if !self.command_palette.is_open {
            return None;
        }

        let mut executed_command = None;

        egui::Window::new("Command Palette")
            .anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 100.0))
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                // Search input
                let response = ui.text_edit_singleline(&mut self.command_palette.search_query);
                if response.changed() {
                    self.command_palette.update_filtered_commands();
                    self.command_palette.selected_index = 0;
                }

                // Auto-focus search input
                if self.command_palette.search_query.is_empty() {
                    response.request_focus();
                }

                ui.separator();

                // Command list
                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        for (index, command) in self.command_palette.filtered_commands.iter().enumerate() {
                            let is_selected = index == self.command_palette.selected_index;
                            
                            let response = ui.selectable_label(is_selected, format!(
                                "{} - {}",
                                command.name,
                                command.description
                            ));

                            if response.clicked() {
                                executed_command = Some(command.id.clone());
                                self.command_palette.is_open = false;
                            }

                            if is_selected && response.hovered() {
                                // Show shortcut hint if available
                                if let Some(shortcut) = &command.shortcut {
                                    ui.label(format!("Shortcut: {}", self.format_shortcut(shortcut)));
                                }
                            }
                        }
                    });

                // Handle keyboard navigation
                ui.input(|i| {
                    if i.key_pressed(Key::Escape) {
                        self.command_palette.is_open = false;
                    } else if i.key_pressed(Key::Enter) {
                        if let Some(command) = self.command_palette.filtered_commands.get(self.command_palette.selected_index) {
                            executed_command = Some(command.id.clone());
                            self.command_palette.is_open = false;
                        }
                    } else if i.key_pressed(Key::ArrowDown) {
                        self.command_palette.selected_index = 
                            (self.command_palette.selected_index + 1).min(self.command_palette.filtered_commands.len().saturating_sub(1));
                    } else if i.key_pressed(Key::ArrowUp) {
                        self.command_palette.selected_index = self.command_palette.selected_index.saturating_sub(1);
                    }
                });
            });

        executed_command
    }

    /// Format shortcut for display
    fn format_shortcut(&self, combination: &KeyCombination) -> String {
        let mut parts = Vec::new();

        if combination.modifiers.ctrl {
            parts.push("Ctrl");
        }
        if combination.modifiers.alt {
            parts.push("Alt");
        }
        if combination.modifiers.shift {
            parts.push("Shift");
        }
        if combination.modifiers.cmd {
            parts.push("Cmd");
        }

        parts.push(&format!("{:?}", combination.key));

        if !combination.sequence.is_empty() {
            parts.push("then");
            for key_press in &combination.sequence {
                parts.push(&format!("{:?}", key_press.key));
            }
        }

        parts.join("+")
    }

    /// Add custom shortcut
    pub fn add_custom_shortcut(&mut self, shortcut: KeyboardShortcut) -> Result<(), String> {
        // Check for conflicts
        if self.has_conflict(&shortcut.key_combination, &shortcut.context) {
            return Err("Shortcut conflict detected".to_string());
        }

        let id = shortcut.command_id.clone();
        self.user_shortcuts.insert(id.clone(), shortcut.clone());
        self.shortcuts.insert(id, shortcut);
        Ok(())
    }

    /// Check for shortcut conflicts
    fn has_conflict(&self, combination: &KeyCombination, context: &ShortcutContext) -> bool {
        self.shortcuts.values().any(|existing| {
            existing.key_combination == *combination &&
            (existing.context == *context || 
             existing.context == ShortcutContext::Global || 
             *context == ShortcutContext::Global)
        })
    }

    /// Export shortcuts to JSON
    pub fn export_shortcuts(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.user_shortcuts)
    }

    /// Import shortcuts from JSON
    pub fn import_shortcuts(&mut self, json: &str) -> Result<(), Box<dyn std::error::Error>> {
        let imported: HashMap<String, KeyboardShortcut> = serde_json::from_str(json)?;
        
        for (id, shortcut) in imported {
            if !self.has_conflict(&shortcut.key_combination, &shortcut.context) {
                self.user_shortcuts.insert(id.clone(), shortcut.clone());
                self.shortcuts.insert(id, shortcut);
            }
        }
        
        Ok(())
    }

    /// Get all shortcuts for a context
    pub fn get_shortcuts_for_context(&self, context: &ShortcutContext) -> Vec<&KeyboardShortcut> {
        self.shortcuts.values()
            .filter(|s| s.context == *context || s.context == ShortcutContext::Global)
            .collect()
    }

    /// Enable/disable shortcuts
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
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

    /// Update filtered commands based on search query
    fn update_filtered_commands(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_commands = self.all_commands.clone();
        } else {
            let query = self.search_query.to_lowercase();
            self.filtered_commands = self.all_commands
                .iter()
                .filter(|cmd| {
                    cmd.name.to_lowercase().contains(&query) ||
                    cmd.description.to_lowercase().contains(&query) ||
                    cmd.tags.iter().any(|tag| tag.contains(&query))
                })
                .cloned()
                .collect();
        }

        // Sort by usage count and relevance
        self.filtered_commands.sort_by(|a, b| {
            let a_score = self.calculate_relevance_score(a);
            let b_score = self.calculate_relevance_score(b);
            b_score.partial_cmp(&a_score).unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    /// Calculate relevance score for command ranking
    fn calculate_relevance_score(&self, command: &Command) -> f32 {
        let mut score = command.usage_count as f32 * 0.1;

        if !self.search_query.is_empty() {
            let query = self.search_query.to_lowercase();
            
            // Exact name match gets highest score
            if command.name.to_lowercase() == query {
                score += 100.0;
            } else if command.name.to_lowercase().starts_with(&query) {
                score += 50.0;
            } else if command.name.to_lowercase().contains(&query) {
                score += 25.0;
            }

            // Description matches
            if command.description.to_lowercase().contains(&query) {
                score += 10.0;
            }

            // Tag matches
            for tag in &command.tags {
                if tag.contains(&query) {
                    score += 5.0;
                }
            }
        }

        // Recent commands get bonus
        if self.recent_commands.contains(&command.id) {
            score += 20.0;
        }

        score
    }
}

impl ModifierSet {
    fn none() -> Self {
        Self { ctrl: false, alt: false, shift: false, cmd: false }
    }

    fn ctrl() -> Self {
        Self { ctrl: true, alt: false, shift: false, cmd: false }
    }

    fn ctrl_shift() -> Self {
        Self { ctrl: true, alt: false, shift: true, cmd: false }
    }

    fn ctrl_alt() -> Self {
        Self { ctrl: true, alt: true, shift: false, cmd: false }
    }

    fn alt() -> Self {
        Self { ctrl: false, alt: true, shift: false, cmd: false }
    }

    fn from_egui_modifiers(modifiers: Modifiers) -> Self {
        Self {
            ctrl: modifiers.ctrl,
            alt: modifiers.alt,
            shift: modifiers.shift,
            cmd: modifiers.command,
        }
    }
}

/// Shortcut builder for easy shortcut creation
pub struct ShortcutBuilder {
    shortcut: KeyboardShortcut,
}

impl ShortcutBuilder {
    pub fn new(command_id: &str, description: &str) -> Self {
        Self {
            shortcut: KeyboardShortcut {
                command_id: command_id.to_string(),
                description: description.to_string(),
                key_combination: KeyCombination {
                    key: Key::A, // Default, will be overridden
                    modifiers: ModifierSet::none(),
                    sequence: Vec::new(),
                },
                context: ShortcutContext::Global,
                enabled: true,
                is_custom: true,
            },
        }
    }

    pub fn key(mut self, key: Key) -> Self {
        self.shortcut.key_combination.key = key;
        self
    }

    pub fn ctrl(mut self) -> Self {
        self.shortcut.key_combination.modifiers.ctrl = true;
        self
    }

    pub fn alt(mut self) -> Self {
        self.shortcut.key_combination.modifiers.alt = true;
        self
    }

    pub fn shift(mut self) -> Self {
        self.shortcut.key_combination.modifiers.shift = true;
        self
    }

    pub fn context(mut self, context: ShortcutContext) -> Self {
        self.shortcut.context = context;
        self
    }

    pub fn sequence(mut self, sequence: Vec<KeyPress>) -> Self {
        self.shortcut.key_combination.sequence = sequence;
        self
    }

    pub fn build(self) -> KeyboardShortcut {
        self.shortcut
    }
}