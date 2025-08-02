//! # Content Manager
//! 
//! Manages the main content area of the IDE, including the visual designer,
//! code editor, and central workspace functionality.

use eframe::egui;
use super::app_state::IdeAppState;
use super::drag_drop::DragState;

/// # Content Manager
/// 
/// Handles the main content area rendering and interaction management.
/// This includes the visual designer, code editor, and workspace coordination.
pub struct ContentManager;

impl ContentManager {
    /// Render the main central content area
    pub fn render_central_panel(app_state: &mut IdeAppState, drag_state: &mut DragState, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if app_state.design_mode {
                Self::render_design_mode(app_state, drag_state, ui);
            } else {
                Self::render_code_mode(app_state, ui);
            }
        });
    }
    
    /// Render the design mode interface
    fn render_design_mode(app_state: &mut IdeAppState, drag_state: &mut DragState, ui: &mut egui::Ui) {
        // Add padding around the design area for a better visual separation
        ui.add_space(8.0);
        
        // Create a frame for the design canvas
        egui::Frame::none()
            .fill(egui::Color32::from_gray(245)) // Light gray background
            .inner_margin(egui::Margin::same(16.0))
            .show(ui, |ui| {
                // Add a status bar with design information
                Self::render_design_status_bar(app_state, ui);
                ui.separator();
                ui.add_space(8.0);
                
                // Check if multi-device preview is enabled
                let preview_enabled = app_state.multi_device_preview.enabled;
                
                if preview_enabled {
                    // For now, render normally without the preview frame to avoid borrowing issues
                    // TODO: Refactor to properly handle the preview frame without borrow conflicts
                    Self::render_visual_designer(app_state, drag_state, ui);
                } else {
                    Self::render_visual_designer(app_state, drag_state, ui);
                }
            });
        
        // Render live feedback overlays
        app_state.live_feedback.render_overlays(ui, &app_state.visual_designer);
    }
    
    /// Render the visual designer
    fn render_visual_designer(app_state: &mut IdeAppState, drag_state: &mut DragState, ui: &mut egui::Ui) {
        let canvas_size = ui.available_size();
        
        // Update live feedback system with current selection
        if let Some(selected_idx) = app_state.selected_component {
            if selected_idx == usize::MAX {
                // Form is selected - no component update needed
            } else if selected_idx < app_state.components.len() {
                app_state.live_feedback.update_selection(selected_idx, &app_state.components);
            }
        }
        
        // Render the visual designer canvas with root form
        app_state.visual_designer.render_design_canvas(ui, &mut app_state.root_form, &mut app_state.components, canvas_size);
        
        // Handle drag and drop interactions
        Self::handle_drag_drop_interactions(app_state, drag_state, ui);
        
        // Handle component selection
        Self::handle_component_selection(app_state, ui);
    }
    
    /// Handle drag and drop interactions in the design canvas
    fn handle_drag_drop_interactions(app_state: &mut IdeAppState, drag_state: &mut DragState, ui: &mut egui::Ui) {
        let response = ui.interact(ui.available_rect_before_wrap(), egui::Id::new("design_canvas"), egui::Sense::click_and_drag());
        
        // Handle drag start
        if response.drag_started() {
            if let Some(component_idx) = Self::find_component_at_position(app_state, response.interact_pointer_pos().unwrap_or_default()) {
                drag_state.start_drag(
                    super::drag_drop::DragType::ComponentMove,
                    response.interact_pointer_pos().unwrap_or_default()
                );
                drag_state.dragging_component = Some(component_idx);
            }
        }
        
        // Handle ongoing drag
        if response.dragged() && drag_state.is_dragging {
            if let Some(pointer_pos) = response.interact_pointer_pos() {
                drag_state.update_drag_position(pointer_pos);
            }
        }
        
        // Handle drag end
        if response.drag_stopped() && drag_state.is_dragging {
            if let Some(result) = drag_state.end_drag() {
                Self::apply_drag_result(app_state, result);
            }
        }
        
        // Handle drag cancellation (ESC key)
        if ui.input(|i| i.key_pressed(egui::Key::Escape)) && drag_state.is_dragging {
            drag_state.cancel_drag();
        }
    }
    
    /// Find component at the given position
    fn find_component_at_position(app_state: &IdeAppState, pos: egui::Pos2) -> Option<usize> {
        // TODO: Implement spatial indexing for efficient hit testing
        // For now, use simple iteration (not efficient for large numbers of components)
        for (idx, _component) in app_state.components.iter().enumerate() {
            // Check if position is within component bounds
            // This would use the visual designer's layout information
            if Self::point_in_component_bounds(pos, idx, app_state) {
                return Some(idx);
            }
        }
        None
    }
    
    /// Check if a point is within component bounds
    fn point_in_component_bounds(_pos: egui::Pos2, _component_idx: usize, _app_state: &IdeAppState) -> bool {
        // TODO: Implement actual bounds checking using visual designer layout
        false
    }
    
    /// Apply the result of a completed drag operation
    fn apply_drag_result(app_state: &mut IdeAppState, result: super::drag_drop::DragCompletionResult) {
        use super::drag_drop::DragType;
        
        match result.drag_type {
            DragType::ComponentMove => {
                if let Some(component_idx) = result.component_index {
                    // Update component position through visual designer
                    let delta = result.end_position - result.start_position;
                    app_state.visual_designer.move_component(component_idx, delta);
                    
                    // Add to history for undo/redo
                    app_state.visual_designer.history.add_to_history(
                        crate::editor::visual_designer::history::DesignOperation::Move {
                            component_ids: vec![component_idx],
                            old_positions: vec![result.start_position],
                            new_positions: vec![result.end_position],
                        }
                    );
                }
            }
            DragType::ComponentFromPalette(component_type) => {
                // Create new component at drop position
                Self::create_component_from_palette(app_state, component_type, result.end_position);
            }
            DragType::ComponentResize { .. } => {
                if let Some(component_idx) = result.component_index {
                    // Update component size through visual designer
                    let new_size = result.end_position - result.start_position;
                    app_state.visual_designer.resize_component(component_idx, new_size);
                }
            }
            DragType::MultiComponentMove { .. } => {
                // Handle multi-component movement
                let delta = result.end_position - result.start_position;
                app_state.visual_designer.move_selected_components(delta);
            }
            DragType::None => {
                // Should not happen, but handle gracefully
            }
        }
    }
    
    /// Create a new component from palette at the specified position
    fn create_component_from_palette(app_state: &mut IdeAppState, component_type: super::drag_drop::ComponentType, position: egui::Pos2) {
        use super::drag_drop::ComponentType;
        use crate::rcl::ui::basic::*;
        
        let new_component: Box<dyn crate::rcl::ui::component::Component> = match component_type {
            ComponentType::Button => Box::new(button::Button::new("New Button".to_string())),
            ComponentType::Label => Box::new(label::Label::new("New Label".to_string())),
            ComponentType::TextBox => Box::new(textbox::TextBox::new("New TextBox".to_string())),
            ComponentType::Checkbox => Box::new(checkbox::Checkbox::new("New Checkbox".to_string(), false)),
            ComponentType::Slider => Box::new(slider::Slider::new(50.0, 0.0, 100.0)),
            ComponentType::Dropdown => Box::new(dropdown::Dropdown::new(
                "New Dropdown".to_string(),
                vec!["Option 1".to_string(), "Option 2".to_string()],
                0
            )),
            _ => {
                // For other component types, create a label as placeholder
                Box::new(label::Label::new(format!("New {}", component_type.display_name())))
            }
        };
        
        // Add component to the collection
        app_state.components.push(new_component);
        let new_component_idx = app_state.components.len() - 1;
        
        // Set component position in visual designer
        app_state.visual_designer.set_component_position(new_component_idx, position);
        
        // Select the new component
        app_state.selected_component = Some(new_component_idx);
        
        // Add to history
        app_state.visual_designer.history.add_to_history(
            crate::editor::visual_designer::history::DesignOperation::Add {
                component_id: new_component_idx,
                position,
            }
        );
    }
    
    /// Handle component selection logic
    fn handle_component_selection(app_state: &mut IdeAppState, ui: &mut egui::Ui) {
        let response = ui.interact(ui.available_rect_before_wrap(), egui::Id::new("selection_handler"), egui::Sense::click());
        
        if response.clicked() {
            if let Some(pointer_pos) = response.interact_pointer_pos() {
                // Find component at click position
                if let Some(component_idx) = Self::find_component_at_position(app_state, pointer_pos) {
                    // Handle multi-selection with Ctrl key
                    if ui.input(|i| i.modifiers.ctrl) {
                        app_state.visual_designer.selection.toggle_selection(component_idx);
                    } else {
                        // Single selection
                        app_state.selected_component = Some(component_idx);
                        app_state.visual_designer.clear_selection();
                        app_state.visual_designer.selection.select(component_idx);
                        
                        // Basic property inspector doesn't need explicit selection updates
                    }
                } else {
                    // Clicked on empty space (form background) - select the form
                    app_state.selected_component = Some(usize::MAX); // Use MAX to represent form selection
                    app_state.visual_designer.clear_selection();
                    
                    // Clear property inspector selection when form is selected (no method needed for basic inspector)
                }
            }
        }
    }
    
    /// Render the code editor mode
    fn render_code_mode(app_state: &mut IdeAppState, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Code editor toolbar
            if ui.button("💾").on_hover_text("Save File").clicked() {
                // TODO: Save current file
            }
            if ui.button("🔍").on_hover_text("Find/Replace").clicked() {
                // TODO: Open find/replace dialog
            }
            if ui.button("🚀").on_hover_text("Run Code").clicked() {
                // TODO: Run/compile code
            }
            
            ui.separator();
            
            // LSP status
            let lsp_status = if app_state.lsp_client.is_connected() {
                "🟢 LSP Connected"
            } else {
                "🔴 LSP Disconnected"
            };
            ui.label(lsp_status);
        });
        
        ui.separator();
        
        // Main code editor area
        app_state.code_editor.render(ui);
        
        // Render LSP diagnostics if available
        if app_state.lsp_client.has_diagnostics() {
            ui.separator();
            ui.label("Diagnostics:");
            app_state.lsp_client.render_diagnostics(ui);
        }
    }
    
    /// Handle keyboard shortcuts for the content area
    pub fn handle_shortcuts(app_state: &mut IdeAppState, ctx: &egui::Context) {
        ctx.input(|i| {
            // Global shortcuts
            if i.key_pressed(egui::Key::F5) {
                // Switch to design mode
                app_state.design_mode = true;
            }
            if i.key_pressed(egui::Key::F6) {
                // Switch to code mode  
                app_state.design_mode = false;
            }
            
            // Design mode shortcuts
            if app_state.design_mode {
                if i.modifiers.ctrl && i.key_pressed(egui::Key::Z) {
                    // Undo
                    app_state.visual_designer.undo();
                }
                if i.modifiers.ctrl && i.key_pressed(egui::Key::Y) {
                    // Redo
                    app_state.visual_designer.redo();
                }
                if i.key_pressed(egui::Key::Delete) {
                    // Delete selected component
                    if let Some(selected_idx) = app_state.selected_component {
                        Self::delete_component(app_state, selected_idx);
                    }
                }
                if i.modifiers.ctrl && i.key_pressed(egui::Key::D) {
                    // Duplicate selected component
                    if let Some(selected_idx) = app_state.selected_component {
                        Self::duplicate_component(app_state, selected_idx);
                    }
                }
            }
        });
    }
    
    /// Delete a component
    fn delete_component(app_state: &mut IdeAppState, component_idx: usize) {
        if component_idx < app_state.components.len() {
            app_state.components.remove(component_idx);
            app_state.selected_component = None;
            app_state.visual_designer.clear_selection();
            
            // Add to history
            app_state.visual_designer.history.add_to_history(
                crate::editor::visual_designer::history::DesignOperation::Delete {
                    component_id: component_idx,
                    component_data: format!("Deleted component {}", component_idx),
                }
            );
        }
    }
    
    /// Duplicate a component
    fn duplicate_component(app_state: &mut IdeAppState, component_idx: usize) {
        if component_idx < app_state.components.len() {
            // TODO: Implement component cloning
            // For now, create a simple duplicate based on component type
            let new_component = Self::create_duplicate_component(&app_state.components[component_idx]);
            app_state.components.push(new_component);
            
            let new_idx = app_state.components.len() - 1;
            app_state.selected_component = Some(new_idx);
            
            // Position slightly offset from original
            let original_pos = app_state.visual_designer.get_component_position(component_idx);
            let new_pos = original_pos + egui::Vec2::new(20.0, 20.0);
            app_state.visual_designer.set_component_position(new_idx, new_pos);
        }
    }
    
    /// Create a duplicate of the given component
    fn create_duplicate_component(original: &Box<dyn crate::rcl::ui::component::Component>) -> Box<dyn crate::rcl::ui::component::Component> {
        // TODO: Implement proper component cloning
        // For now, create a new component based on the name
        match original.name() {
            "Button" => Box::new(crate::rcl::ui::basic::button::Button::new("Copy of Button".to_string())),
            "Label" => Box::new(crate::rcl::ui::basic::label::Label::new("Copy of Label".to_string())),
            "TextBox" => Box::new(crate::rcl::ui::basic::textbox::TextBox::new("Copy of TextBox".to_string())),
            "Checkbox" => Box::new(crate::rcl::ui::basic::checkbox::Checkbox::new("Copy of Checkbox".to_string(), false)),
            "Slider" => Box::new(crate::rcl::ui::basic::slider::Slider::new(50.0, 0.0, 100.0)),
            "Dropdown" => Box::new(crate::rcl::ui::basic::dropdown::Dropdown::new(
                "Copy of Dropdown".to_string(),
                vec!["Option 1".to_string(), "Option 2".to_string()],
                0
            )),
            _ => Box::new(crate::rcl::ui::basic::label::Label::new("Copy of Component".to_string())),
        }
    }

    /// Render design status bar showing current form information
    fn render_design_status_bar(app_state: &IdeAppState, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Form name and info
            ui.label("📄");
            ui.strong("Form1"); // TODO: Make this configurable
            ui.separator();
            
            // Component count
            ui.label("🧩");
            ui.label(format!("{} components", app_state.components.len()));
            ui.separator();
            
            // Selection info
            if let Some(selected_idx) = app_state.selected_component {
                if selected_idx == usize::MAX {
                    ui.label("🎯");
                    ui.label("Selected: Form (Root)");
                } else if selected_idx < app_state.components.len() {
                    ui.label("🎯");
                    ui.label(format!("Selected: {}", app_state.components[selected_idx].name()));
                } else {
                    ui.label("❌");
                    ui.label("Invalid selection");
                }
            } else {
                ui.label("⭕");
                ui.label("No selection");
            }
            ui.separator();
            
            // Grid status
            if app_state.visual_designer.grid.visible {
                ui.label("📐");
                ui.label(format!("Grid: {}px", app_state.visual_designer.grid.size));
                if app_state.visual_designer.grid.snap_enabled {
                    ui.label("🔗 Snap");
                }
            }
            
            // Fill remaining space
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Zoom level (placeholder)
                ui.label("🔍 100%");
            });
        });
    }
}