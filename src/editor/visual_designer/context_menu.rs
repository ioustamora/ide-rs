/// Context Menu System for Visual Designer
/// 
/// Provides right-click context menus for components with operations like
/// copy, paste, delete, bring to front, send to back, group, etc.

use egui::*;
use std::collections::HashSet;
use crate::rcl::ui::component::Component;

/// Context menu actions that can be performed on components
#[derive(Debug, Clone, PartialEq)]
pub enum ContextMenuAction {
    Cut,
    Copy,
    Paste,
    Delete,
    Duplicate,
    BringToFront,
    SendToBack,
    BringForward,
    SendBackward,
    Group,
    Ungroup,
    Lock,
    Unlock,
    Hide,
    Show,
    AlignLeft,
    AlignCenter,
    AlignRight,
    AlignTop,
    AlignMiddle,
    AlignBottom,
    DistributeHorizontally,
    DistributeVertically,
    MakeSameWidth,
    MakeSameHeight,
    MakeSameSize,
    ResetToDefault,
    Properties,
}

impl ContextMenuAction {
    /// Get the display name for the action
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Cut => "Cut",
            Self::Copy => "Copy",
            Self::Paste => "Paste",
            Self::Delete => "Delete",
            Self::Duplicate => "Duplicate",
            Self::BringToFront => "Bring to Front",
            Self::SendToBack => "Send to Back",
            Self::BringForward => "Bring Forward",
            Self::SendBackward => "Send Backward", 
            Self::Group => "Group",
            Self::Ungroup => "Ungroup",
            Self::Lock => "Lock",
            Self::Unlock => "Unlock",
            Self::Hide => "Hide",
            Self::Show => "Show",
            Self::AlignLeft => "Align Left",
            Self::AlignCenter => "Align Center",
            Self::AlignRight => "Align Right",
            Self::AlignTop => "Align Top",
            Self::AlignMiddle => "Align Middle",
            Self::AlignBottom => "Align Bottom",
            Self::DistributeHorizontally => "Distribute Horizontally",
            Self::DistributeVertically => "Distribute Vertically",
            Self::MakeSameWidth => "Make Same Width",
            Self::MakeSameHeight => "Make Same Height",
            Self::MakeSameSize => "Make Same Size",
            Self::ResetToDefault => "Reset to Default",
            Self::Properties => "Properties",
        }
    }

    /// Get the keyboard shortcut for the action
    pub fn shortcut(&self) -> Option<&'static str> {
        match self {
            Self::Cut => Some("Ctrl+X"),
            Self::Copy => Some("Ctrl+C"),
            Self::Paste => Some("Ctrl+V"),
            Self::Delete => Some("Del"),
            Self::Duplicate => Some("Ctrl+D"),
            Self::BringToFront => Some("Ctrl+Shift+]"),
            Self::SendToBack => Some("Ctrl+Shift+["),
            Self::BringForward => Some("Ctrl+]"),
            Self::SendBackward => Some("Ctrl+["),
            Self::Group => Some("Ctrl+G"),
            Self::Ungroup => Some("Ctrl+Shift+G"),
            Self::Lock => Some("Ctrl+L"),
            Self::Properties => Some("F4"),
            _ => None,
        }
    }

    /// Get the icon for the action
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Cut => "✂",
            Self::Copy => "📋",
            Self::Paste => "📄",
            Self::Delete => "🗑",
            Self::Duplicate => "📋",
            Self::BringToFront => "⬆",
            Self::SendToBack => "⬇",
            Self::BringForward => "↗",
            Self::SendBackward => "↙",
            Self::Group => "📦",
            Self::Ungroup => "📤",
            Self::Lock => "🔒",
            Self::Unlock => "🔓",
            Self::Hide => "👁‍🗨",
            Self::Show => "👁",
            Self::AlignLeft => "◀",
            Self::AlignCenter => "⬛",
            Self::AlignRight => "▶",
            Self::AlignTop => "▲",
            Self::AlignMiddle => "⬛",
            Self::AlignBottom => "▼",
            Self::DistributeHorizontally => "↔",
            Self::DistributeVertically => "↕",
            Self::MakeSameWidth => "↔",
            Self::MakeSameHeight => "↕",
            Self::MakeSameSize => "⬛",
            Self::ResetToDefault => "↺",
            Self::Properties => "⚙",
        }
    }

    /// Check if this action requires multiple selected components
    pub fn requires_multiple_selection(&self) -> bool {
        matches!(self, 
            Self::Group | 
            Self::DistributeHorizontally | 
            Self::DistributeVertically |
            Self::MakeSameWidth |
            Self::MakeSameHeight |
            Self::MakeSameSize
        )
    }

    /// Check if this action is disabled for the given selection
    pub fn is_disabled(&self, selected_components: &HashSet<usize>, has_clipboard: bool) -> bool {
        match self {
            Self::Paste => !has_clipboard,
            Self::Cut | Self::Copy | Self::Delete | Self::Duplicate => selected_components.is_empty(),
            action if action.requires_multiple_selection() => selected_components.len() < 2,
            Self::Ungroup => {
                // Would need to check if any selected components are groups
                false // For now, always enabled
            }
            _ => false,
        }
    }
}

/// Context menu state and management
pub struct ContextMenuManager {
    /// Whether the context menu is currently open
    pub is_open: bool,
    /// Position where the context menu should appear
    pub menu_position: Option<Pos2>,
    /// Component index that was right-clicked (if any)
    pub target_component: Option<usize>,
    /// Whether we have content in clipboard
    pub has_clipboard: bool,
    /// Clipboard content (simplified - would store actual component data)
    pub clipboard_data: Vec<String>,
}

impl Default for ContextMenuManager {
    fn default() -> Self {
        Self {
            is_open: false,
            menu_position: None,
            target_component: None,
            has_clipboard: false,
            clipboard_data: Vec::new(),
        }
    }
}

impl ContextMenuManager {
    /// Create a new context menu manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Show context menu at the given position
    pub fn show_at(&mut self, position: Pos2, target_component: Option<usize>) {
        self.is_open = true;
        self.menu_position = Some(position);
        self.target_component = target_component;
    }

    /// Hide the context menu
    pub fn hide(&mut self) {
        self.is_open = false;
        self.menu_position = None;
        self.target_component = None;
    }

    /// Render the context menu and return any selected action
    pub fn render_context_menu(
        &mut self,
        ui: &mut Ui,
        selected_components: &HashSet<usize>,
    ) -> Option<ContextMenuAction> {
        if !self.is_open {
            return None;
        }

        let mut action_taken = None;

        if let Some(menu_pos) = self.menu_position {
            let menu_id = Id::new("component_context_menu");
            
            Area::new(menu_id)
                .fixed_pos(menu_pos)
                .order(Order::Foreground)
                .show(ui.ctx(), |ui| {
                    Frame::popup(ui.style())
                        .inner_margin(Margin::same(8.0))
                        .show(ui, |ui| {
                            ui.set_min_width(200.0);
                            
                            // Basic operations
                            ui.label(RichText::new("Edit").strong().small());
                            ui.separator();
                            
                            for action in [
                                ContextMenuAction::Cut,
                                ContextMenuAction::Copy,
                                ContextMenuAction::Paste,
                                ContextMenuAction::Delete,
                                ContextMenuAction::Duplicate,
                            ] {
                                if self.render_menu_item(ui, &action, selected_components) {
                                    action_taken = Some(action);
                                }
                            }
                            
                            ui.add_space(4.0);
                            
                            // Layer operations (only show if components are selected)
                            if !selected_components.is_empty() {
                                ui.label(RichText::new("Layer").strong().small());
                                ui.separator();
                                
                                for action in [
                                    ContextMenuAction::BringToFront,
                                    ContextMenuAction::BringForward,
                                    ContextMenuAction::SendBackward,
                                    ContextMenuAction::SendToBack,
                                ] {
                                    if self.render_menu_item(ui, &action, selected_components) {
                                        action_taken = Some(action);
                                    }
                                }
                                
                                ui.add_space(4.0);
                            }
                            
                            // Grouping operations (only show if multiple components selected)
                            if selected_components.len() > 1 {
                                ui.label(RichText::new("Group").strong().small());
                                ui.separator();
                                
                                for action in [
                                    ContextMenuAction::Group,
                                    ContextMenuAction::Ungroup,
                                ] {
                                    if self.render_menu_item(ui, &action, selected_components) {
                                        action_taken = Some(action);
                                    }
                                }
                                
                                ui.add_space(4.0);
                            }
                            
                            // Alignment operations (only show if multiple components selected)
                            if selected_components.len() > 1 {
                                ui.label(RichText::new("Align").strong().small());
                                ui.separator();
                                
                                ui.horizontal(|ui| {
                                    // Horizontal alignment
                                    for action in [
                                        ContextMenuAction::AlignLeft,
                                        ContextMenuAction::AlignCenter,
                                        ContextMenuAction::AlignRight,
                                    ] {
                                        if ui.small_button(action.icon()).on_hover_text(action.display_name()).clicked() {
                                            action_taken = Some(action);
                                        }
                                    }
                                });
                                
                                ui.horizontal(|ui| {
                                    // Vertical alignment
                                    for action in [
                                        ContextMenuAction::AlignTop,
                                        ContextMenuAction::AlignMiddle,
                                        ContextMenuAction::AlignBottom,
                                    ] {
                                        if ui.small_button(action.icon()).on_hover_text(action.display_name()).clicked() {
                                            action_taken = Some(action);
                                        }
                                    }
                                });
                                
                                // Distribution and sizing
                                for action in [
                                    ContextMenuAction::DistributeHorizontally,
                                    ContextMenuAction::DistributeVertically,
                                    ContextMenuAction::MakeSameWidth,
                                    ContextMenuAction::MakeSameHeight,
                                    ContextMenuAction::MakeSameSize,
                                ] {
                                    if self.render_menu_item(ui, &action, selected_components) {
                                        action_taken = Some(action);
                                    }
                                }
                                
                                ui.add_space(4.0);
                            }
                            
                            // Component-specific operations
                            if !selected_components.is_empty() {
                                ui.label(RichText::new("Component").strong().small());
                                ui.separator();
                                
                                for action in [
                                    ContextMenuAction::Lock,
                                    ContextMenuAction::Hide,
                                    ContextMenuAction::ResetToDefault,
                                ] {
                                    if self.render_menu_item(ui, &action, selected_components) {
                                        action_taken = Some(action);
                                    }
                                }
                                
                                ui.add_space(4.0);
                                
                                // Properties (separator line above)
                                ui.separator();
                                if self.render_menu_item(ui, &ContextMenuAction::Properties, selected_components) {
                                    action_taken = Some(ContextMenuAction::Properties);
                                }
                            }
                        });
                });

            // Close menu if clicked outside or action was taken
            if ui.input(|i| i.pointer.any_click()) || action_taken.is_some() {
                // Check if click was outside the menu
                if let Some(pointer_pos) = ui.ctx().pointer_latest_pos() {
                    let menu_rect = Rect::from_min_size(menu_pos, Vec2::new(200.0, 300.0)); // Approximate menu size
                    if !menu_rect.contains(pointer_pos) || action_taken.is_some() {
                        self.hide();
                    }
                }
            }
        }

        action_taken
    }

    /// Render a single menu item
    fn render_menu_item(
        &self,
        ui: &mut Ui,
        action: &ContextMenuAction,
        selected_components: &HashSet<usize>,
    ) -> bool {
        let is_disabled = action.is_disabled(selected_components, self.has_clipboard);
        
        ui.add_enabled_ui(!is_disabled, |ui| {
            ui.horizontal(|ui| {
                // Icon
                ui.label(action.icon());
                
                // Action name
                ui.label(action.display_name());
                
                // Keyboard shortcut (right-aligned)
                if let Some(shortcut) = action.shortcut() {
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(RichText::new(shortcut).small().weak());
                    });
                }
            }).response.clicked()
        }).inner
    }

    /// Handle keyboard shortcuts for context menu actions
    pub fn handle_keyboard_shortcuts(
        &mut self,
        ui: &mut Ui,
        selected_components: &HashSet<usize>,
    ) -> Option<ContextMenuAction> {
        let input = ui.input(|i| i.clone());
        
        // Check for keyboard shortcuts
        if input.modifiers.ctrl {
            if input.key_pressed(Key::X) && !selected_components.is_empty() {
                return Some(ContextMenuAction::Cut);
            }
            if input.key_pressed(Key::C) && !selected_components.is_empty() {
                return Some(ContextMenuAction::Copy);
            }
            if input.key_pressed(Key::V) && self.has_clipboard {
                return Some(ContextMenuAction::Paste);
            }
            if input.key_pressed(Key::D) && !selected_components.is_empty() {
                return Some(ContextMenuAction::Duplicate);
            }
            if input.key_pressed(Key::G) {
                if input.modifiers.shift {
                    return Some(ContextMenuAction::Ungroup);
                } else if selected_components.len() > 1 {
                    return Some(ContextMenuAction::Group);
                }
            }
            if input.key_pressed(Key::L) && !selected_components.is_empty() {
                return Some(ContextMenuAction::Lock);
            }
        }
        
        if input.key_pressed(Key::Delete) && !selected_components.is_empty() {
            return Some(ContextMenuAction::Delete);
        }
        
        if input.key_pressed(Key::F4) && !selected_components.is_empty() {
            return Some(ContextMenuAction::Properties);
        }
        
        None
    }

    /// Copy selected components to clipboard
    pub fn copy_to_clipboard(&mut self, components: &[Box<dyn Component>], selected: &HashSet<usize>) {
        self.clipboard_data.clear();
        
        for &idx in selected {
            if let Some(component) = components.get(idx) {
                // Serialize component data (simplified)
                self.clipboard_data.push(format!("{}:{}", component.name(), idx));
            }
        }
        
        self.has_clipboard = !self.clipboard_data.is_empty();
    }

    /// Cut selected components (copy + mark for deletion)
    pub fn cut_to_clipboard(&mut self, components: &[Box<dyn Component>], selected: &HashSet<usize>) {
        self.copy_to_clipboard(components, selected);
        // The actual deletion would be handled by the caller
    }

    /// Check if clipboard has content that can be pasted
    pub fn can_paste(&self) -> bool {
        self.has_clipboard && !self.clipboard_data.is_empty()
    }

    /// Clear clipboard
    pub fn clear_clipboard(&mut self) {
        self.clipboard_data.clear();
        self.has_clipboard = false;
    }
}