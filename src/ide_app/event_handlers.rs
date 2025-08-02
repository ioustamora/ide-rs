//! # Event Handlers
//! 
//! Manages global event handling, keyboard shortcuts, and command processing
//! for the IDE application.

use eframe::egui;
use super::app_state::IdeAppState;

/// # Event Handlers
/// 
/// Centralized event handling system for the IDE. This manages global
/// keyboard shortcuts, menu commands, and other application-wide events.
#[derive(Default)]
pub struct EventHandlers {
    /// Queue of pending commands to be processed
    command_queue: Vec<IdeCommand>,
    
    /// Last known modifier state for complex shortcuts
    last_modifiers: egui::Modifiers,
    
    /// Tracks if we're in the middle of a multi-key sequence
    in_key_sequence: bool,
    
    /// Current key sequence being built
    key_sequence: Vec<egui::Key>,
}

/// # IDE Commands
/// 
/// Represents commands that can be executed in the IDE.
/// These can be triggered by keyboard shortcuts, menu items, or programmatically.
#[derive(Debug, Clone)]
pub enum IdeCommand {
    // File operations
    NewProject,
    OpenProject,
    SaveProject,
    SaveProjectAs,
    CloseProject,
    
    // Edit operations
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    Delete,
    SelectAll,
    
    // View operations
    ToggleDesignMode,
    ToggleCodeMode,
    ToggleComponentPalette,
    TogglePropertiesPanel,
    ToggleProjectPanel,
    ToggleOutputPanel,
    ToggleAiPanel,
    ToggleHierarchyPanel,
    ToggleModernIdePanel,
    
    // Design operations
    ToggleGrid,
    ToggleRulers,
    ToggleSnapToGrid,
    AlignLeft,
    AlignRight,
    AlignTop,
    AlignBottom,
    AlignCenter,
    AlignMiddle,
    
    // Build operations
    BuildDebug,
    BuildRelease,
    RunDebug,
    RunRelease,
    Clean,
    
    // AI operations
    AiAssist,
    AiGenerate,
    AiFix,
    
    // Component operations
    DuplicateComponent,
    DeleteComponent,
    GroupComponents,
    UngroupComponents,
    BringToFront,
    SendToBack,
    
    // Window operations
    ToggleFullscreen,
    Quit,
}

impl EventHandlers {
    /// Create a new event handlers instance
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Handle global events and keyboard shortcuts
    pub fn handle_global_events(&mut self, _app_state: &mut IdeAppState, ctx: &egui::Context) {
        ctx.input(|i| {
            self.last_modifiers = i.modifiers;
            
            // Handle file operations
            if i.modifiers.ctrl {
                if i.key_pressed(egui::Key::N) {
                    self.queue_command(IdeCommand::NewProject);
                }
                if i.key_pressed(egui::Key::O) {
                    self.queue_command(IdeCommand::OpenProject);
                }
                if i.key_pressed(egui::Key::S) {
                    if i.modifiers.shift {
                        self.queue_command(IdeCommand::SaveProjectAs);
                    } else {
                        self.queue_command(IdeCommand::SaveProject);
                    }
                }
            }
            
            // Handle edit operations
            if i.modifiers.ctrl {
                if i.key_pressed(egui::Key::Z) {
                    if i.modifiers.shift {
                        self.queue_command(IdeCommand::Redo);
                    } else {
                        self.queue_command(IdeCommand::Undo);
                    }
                }
                if i.key_pressed(egui::Key::Y) {
                    self.queue_command(IdeCommand::Redo);
                }
                if i.key_pressed(egui::Key::X) {
                    self.queue_command(IdeCommand::Cut);
                }
                if i.key_pressed(egui::Key::C) {
                    self.queue_command(IdeCommand::Copy);
                }
                if i.key_pressed(egui::Key::V) {
                    self.queue_command(IdeCommand::Paste);
                }
                if i.key_pressed(egui::Key::A) {
                    self.queue_command(IdeCommand::SelectAll);
                }
                if i.key_pressed(egui::Key::D) {
                    self.queue_command(IdeCommand::DuplicateComponent);
                }
            }
            
            // Handle function keys
            if i.key_pressed(egui::Key::F5) {
                if i.modifiers.ctrl {
                    self.queue_command(IdeCommand::RunDebug);
                } else {
                    self.queue_command(IdeCommand::ToggleDesignMode);
                }
            }
            if i.key_pressed(egui::Key::F6) {
                self.queue_command(IdeCommand::ToggleCodeMode);
            }
            if i.key_pressed(egui::Key::F7) {
                self.queue_command(IdeCommand::BuildDebug);
            }
            if i.key_pressed(egui::Key::F11) {
                self.queue_command(IdeCommand::ToggleFullscreen);
            }
            
            // Handle delete key
            if i.key_pressed(egui::Key::Delete) {
                self.queue_command(IdeCommand::DeleteComponent);
            }
            
            // Handle escape key
            if i.key_pressed(egui::Key::Escape) {
                // Cancel current operation or close active dialog
                // This is handled by individual components
            }
            
            // Handle alignment shortcuts (Ctrl+Alt combinations)
            if i.modifiers.ctrl && i.modifiers.alt {
                if i.key_pressed(egui::Key::L) {
                    self.queue_command(IdeCommand::AlignLeft);
                }
                if i.key_pressed(egui::Key::R) {
                    self.queue_command(IdeCommand::AlignRight);
                }
                if i.key_pressed(egui::Key::T) {
                    self.queue_command(IdeCommand::AlignTop);
                }
                if i.key_pressed(egui::Key::B) {
                    self.queue_command(IdeCommand::AlignBottom);
                }
                if i.key_pressed(egui::Key::C) {
                    self.queue_command(IdeCommand::AlignCenter);
                }
                if i.key_pressed(egui::Key::M) {
                    self.queue_command(IdeCommand::AlignMiddle);
                }
            }
            
            // Handle AI shortcuts
            if i.modifiers.alt {
                if i.key_pressed(egui::Key::A) {
                    self.queue_command(IdeCommand::AiAssist);
                }
                if i.key_pressed(egui::Key::G) {
                    self.queue_command(IdeCommand::AiGenerate);
                }
                if i.key_pressed(egui::Key::F) {
                    self.queue_command(IdeCommand::AiFix);
                }
            }
        });
    }
    
    /// Queue a command for processing
    pub fn queue_command(&mut self, command: IdeCommand) {
        self.command_queue.push(command);
    }
    
    /// Process all pending commands
    pub fn process_pending_events(&mut self, app_state: &mut IdeAppState) {
        let commands = std::mem::take(&mut self.command_queue);
        
        for command in commands {
            self.execute_command(command, app_state);
        }
    }
    
    /// Execute a specific command
    fn execute_command(&self, command: IdeCommand, app_state: &mut IdeAppState) {
        match command {
            // File operations
            IdeCommand::NewProject => {
                app_state.project_manager.create_new_project();
            }
            IdeCommand::OpenProject => {
                app_state.project_manager.open_project_dialog();
            }
            IdeCommand::SaveProject => {
                app_state.project_manager.save_current_project();
            }
            IdeCommand::SaveProjectAs => {
                app_state.project_manager.save_project_as_dialog();
            }
            IdeCommand::CloseProject => {
                app_state.project_manager.close_current_project();
            }
            
            // Edit operations
            IdeCommand::Undo => {
                if app_state.design_mode {
                    app_state.visual_designer.undo();
                } else {
                    app_state.code_editor.undo();
                }
            }
            IdeCommand::Redo => {
                if app_state.design_mode {
                    app_state.visual_designer.redo();
                } else {
                    app_state.code_editor.redo();
                }
            }
            IdeCommand::Cut => {
                if app_state.design_mode {
                    self.cut_selected_components(app_state);
                } else {
                    app_state.code_editor.cut();
                }
            }
            IdeCommand::Copy => {
                if app_state.design_mode {
                    self.copy_selected_components(app_state);
                } else {
                    app_state.code_editor.copy();
                }
            }
            IdeCommand::Paste => {
                if app_state.design_mode {
                    self.paste_components(app_state);
                } else {
                    app_state.code_editor.paste();
                }
            }
            IdeCommand::Delete => {
                if app_state.design_mode {
                    self.delete_selected_components(app_state);
                }
            }
            IdeCommand::SelectAll => {
                if app_state.design_mode {
                    self.select_all_components(app_state);
                } else {
                    app_state.code_editor.select_all();
                }
            }
            
            // View operations
            IdeCommand::ToggleDesignMode => {
                app_state.design_mode = true;
            }
            IdeCommand::ToggleCodeMode => {
                app_state.design_mode = false;
            }
            IdeCommand::ToggleComponentPalette => {
                app_state.show_component_palette = !app_state.show_component_palette;
            }
            IdeCommand::TogglePropertiesPanel => {
                app_state.show_properties_inspector = !app_state.show_properties_inspector;
            }
            IdeCommand::ToggleProjectPanel => {
                app_state.show_project_panel = !app_state.show_project_panel;
            }
            IdeCommand::ToggleOutputPanel => {
                app_state.show_output_panel = !app_state.show_output_panel;
            }
            IdeCommand::ToggleAiPanel => {
                app_state.show_ai_panel = !app_state.show_ai_panel;
            }
            IdeCommand::ToggleHierarchyPanel => {
                app_state.hierarchy_manager.show_hierarchy_panel = !app_state.hierarchy_manager.show_hierarchy_panel;
            }
            IdeCommand::ToggleModernIdePanel => {
                app_state.show_modern_ide_panel = !app_state.show_modern_ide_panel;
            }
            
            // Design operations
            IdeCommand::ToggleGrid => {
                app_state.visual_designer.grid.visible = !app_state.visual_designer.grid.visible;
            }
            IdeCommand::ToggleRulers => {
                app_state.visual_designer.guides.rulers_visible = !app_state.visual_designer.guides.rulers_visible;
            }
            IdeCommand::ToggleSnapToGrid => {
                app_state.visual_designer.grid.snap_enabled = !app_state.visual_designer.grid.snap_enabled;
            }
            IdeCommand::AlignLeft => {
                self.align_selected_components(app_state, crate::editor::visual_designer::layout::AlignmentOperation::AlignLeft);
            }
            IdeCommand::AlignRight => {
                self.align_selected_components(app_state, crate::editor::visual_designer::layout::AlignmentOperation::AlignRight);
            }
            IdeCommand::AlignTop => {
                self.align_selected_components(app_state, crate::editor::visual_designer::layout::AlignmentOperation::AlignTop);
            }
            IdeCommand::AlignBottom => {
                self.align_selected_components(app_state, crate::editor::visual_designer::layout::AlignmentOperation::AlignBottom);
            }
            IdeCommand::AlignCenter => {
                self.align_selected_components(app_state, crate::editor::visual_designer::layout::AlignmentOperation::AlignCenterHorizontal);
            }
            IdeCommand::AlignMiddle => {
                self.align_selected_components(app_state, crate::editor::visual_designer::layout::AlignmentOperation::AlignCenterVertical);
            }
            
            // Build operations
            IdeCommand::BuildDebug => {
                crate::editor::actions::get_actions().build_debug(&mut app_state.menu.output_panel);
            }
            IdeCommand::BuildRelease => {
                crate::editor::actions::get_actions().build_release(&mut app_state.menu.output_panel);
            }
            IdeCommand::RunDebug => {
                crate::editor::actions::get_actions().run_debug(&mut app_state.menu.output_panel);
            }
            IdeCommand::RunRelease => {
                crate::editor::actions::get_actions().run_release(&mut app_state.menu.output_panel);
            }
            IdeCommand::Clean => {
                // TODO: Implement clean command
            }
            
            // AI operations
            IdeCommand::AiAssist => {
                app_state.show_ai_panel = true;
                // TODO: Focus AI input
            }
            IdeCommand::AiGenerate => {
                // TODO: Trigger AI code generation
            }
            IdeCommand::AiFix => {
                // TODO: Trigger AI error fixing
            }
            
            // Component operations
            IdeCommand::DuplicateComponent => {
                self.duplicate_selected_components(app_state);
            }
            IdeCommand::DeleteComponent => {
                self.delete_selected_components(app_state);
            }
            IdeCommand::GroupComponents => {
                self.group_selected_components(app_state);
            }
            IdeCommand::UngroupComponents => {
                self.ungroup_selected_components(app_state);
            }
            IdeCommand::BringToFront => {
                self.bring_to_front(app_state);
            }
            IdeCommand::SendToBack => {
                self.send_to_back(app_state);
            }
            
            // Window operations
            IdeCommand::ToggleFullscreen => {
                // TODO: Implement fullscreen toggle
            }
            IdeCommand::Quit => {
                // TODO: Implement quit with save prompt
            }
        }
    }
    
    // Helper methods for component operations
    
    fn cut_selected_components(&self, app_state: &mut IdeAppState) {
        self.copy_selected_components(app_state);
        self.delete_selected_components(app_state);
    }
    
    fn copy_selected_components(&self, _app_state: &mut IdeAppState) {
        // TODO: Implement component copying to clipboard
    }
    
    fn paste_components(&self, _app_state: &mut IdeAppState) {
        // TODO: Implement component pasting from clipboard
    }
    
    fn delete_selected_components(&self, app_state: &mut IdeAppState) {
        if let Some(selected_idx) = app_state.selected_component {
            if selected_idx < app_state.components.len() {
                app_state.components.remove(selected_idx);
                app_state.selected_component = None;
                app_state.visual_designer.clear_selection();
            }
        }
        
        // Also delete any multi-selected components
        let selected_indices: Vec<usize> = app_state.visual_designer.selection.selected_components().collect();
        for &idx in selected_indices.iter().rev() { // Reverse order to maintain indices
            if idx < app_state.components.len() {
                app_state.components.remove(idx);
            }
        }
        app_state.visual_designer.clear_selection();
    }
    
    fn select_all_components(&self, app_state: &mut IdeAppState) {
        app_state.visual_designer.clear_selection();
        for i in 0..app_state.components.len() {
            app_state.visual_designer.selection.select(i);
        }
    }
    
    fn duplicate_selected_components(&self, app_state: &mut IdeAppState) {
        if let Some(selected_idx) = app_state.selected_component {
            // TODO: Implement component duplication
            let _ = selected_idx; // Suppress unused warning
        }
    }
    
    fn group_selected_components(&self, _app_state: &mut IdeAppState) {
        // TODO: Implement component grouping
    }
    
    fn ungroup_selected_components(&self, _app_state: &mut IdeAppState) {
        // TODO: Implement component ungrouping
    }
    
    fn bring_to_front(&self, _app_state: &mut IdeAppState) {
        // TODO: Implement z-order manipulation
    }
    
    fn send_to_back(&self, _app_state: &mut IdeAppState) {
        // TODO: Implement z-order manipulation
    }
    
    fn align_selected_components(&self, app_state: &mut IdeAppState, operation: crate::editor::visual_designer::layout::AlignmentOperation) {
        let selected_indices: Vec<usize> = app_state.visual_designer.selection.selected_components().collect();
        if selected_indices.len() > 1 {
            // TODO: Implement component alignment
            let _ = operation; // Suppress unused warning
        }
    }
    
    /// Get a description of the available keyboard shortcuts
    pub fn get_shortcuts_help(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("Ctrl+N", "New Project"),
            ("Ctrl+O", "Open Project"),
            ("Ctrl+S", "Save Project"),
            ("Ctrl+Shift+S", "Save Project As"),
            ("Ctrl+Z", "Undo"),
            ("Ctrl+Y / Ctrl+Shift+Z", "Redo"),
            ("Ctrl+X", "Cut"),
            ("Ctrl+C", "Copy"),
            ("Ctrl+V", "Paste"),
            ("Ctrl+A", "Select All"),
            ("Ctrl+D", "Duplicate Component"),
            ("Delete", "Delete Selected"),
            ("F5", "Toggle Design Mode / Run Debug"),
            ("F6", "Toggle Code Mode"),
            ("F7", "Build Debug"),
            ("F11", "Toggle Fullscreen"),
            ("Ctrl+Alt+L", "Align Left"),
            ("Ctrl+Alt+R", "Align Right"),
            ("Ctrl+Alt+T", "Align Top"),
            ("Ctrl+Alt+B", "Align Bottom"),
            ("Ctrl+Alt+C", "Align Center"),
            ("Ctrl+Alt+M", "Align Middle"),
            ("Alt+A", "AI Assist"),
            ("Alt+G", "AI Generate"),
            ("Alt+F", "AI Fix"),
            ("Esc", "Cancel Operation"),
        ]
    }
}