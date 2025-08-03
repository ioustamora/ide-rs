/// Modular Visual Designer - Orchestrator
///
/// This module re-exports all submodules for the visual designer.

pub mod layout;
pub mod selection;
pub mod render;
pub mod history;
pub mod smart_editing;
pub mod performance;
pub mod accessibility;
pub mod state;
pub mod context_menu;

// Re-export key types for easier access
pub use layout::{
    LayoutManager, AlignmentTools, AlignmentOperation, ComponentBounds,
    ConstraintSystem, AutoLayoutMode, ComponentId,
    GridColumns, GridRows, GridTrack, Gap, EdgeInsets,
    StackDirection, StackAlignment, WrapDirection, WrapAlignment,
    GridAlignment, SizeConstraint, HorizontalConstraint, VerticalConstraint,
};
use crate::ide_app::animated_ui::MovementManager;

/// Resize direction for interactive component resizing
#[derive(Debug, Clone, Copy, PartialEq)]
enum ResizeDirection {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}
pub use selection::{
    ComponentSelection, DragOperation, DragOperationType, ResizeHandle,
};
pub use render::{
    GuideSystem, GridSettings,
};
pub use history::{
    DesignHistory, DesignOperation,
};
pub use performance::{
    PerformanceMetrics, MemoryUsage, PerformanceReport,
};
pub use state::{
    DesignerState, DesignTimeProperties,
};
pub use smart_editing::SmartEditingSystem;
pub use accessibility::{AccessibilityValidator, AccessibilityReport};
pub use context_menu::{ContextMenuManager, ContextMenuAction};

/// Main Visual Designer struct that orchestrates all subsystems
#[derive(Default)]
pub struct VisualDesigner {
    /// Layout management system
    pub layout: LayoutManager,
    /// Component selection system
    pub selection: ComponentSelection,
    /// Design history for undo/redo
    pub history: DesignHistory,
    /// Performance metrics
    pub performance: PerformanceMetrics,
    /// Grid settings for display
    pub grid: GridSettings,
    /// Guide system for visual aids
    pub guides: GuideSystem,
    /// Smart editing system
    pub smart_editing: SmartEditingSystem,
    /// Accessibility validator
    pub accessibility: AccessibilityValidator,
    /// Drag and drop state
    pub drag_state: crate::ide_app::drag_drop::DragState,
    /// Movement animation manager for smooth component transitions
    pub movement_manager: MovementManager,
    /// Context menu manager for right-click operations
    pub context_menu: ContextMenuManager,
}

impl VisualDesigner {
    pub fn new() -> Self {
        let mut designer = Self::default();
        designer.movement_manager = MovementManager::new();
        designer.context_menu = ContextMenuManager::new();
        designer
    }

    /// Clear current selection
    pub fn clear_selection(&mut self) {
        self.selection.selected.clear();
        self.selection.primary = None;
    }

    /// Add operation to history for undo/redo
    pub fn add_to_history(&mut self, operation: DesignOperation) {
        self.history.add_to_history(operation);
    }

    /// Undo last operation
    pub fn undo(&mut self, components: &mut Vec<Box<dyn crate::rcl::ui::component::Component>>) -> bool {
        self.history.undo(&mut self.layout, components)
    }

    /// Redo last undone operation
    pub fn redo(&mut self, components: &mut Vec<Box<dyn crate::rcl::ui::component::Component>>) -> bool {
        self.history.redo(&mut self.layout, components)
    }
    
    /// Start a batch of operations for undo/redo
    pub fn begin_operation_batch(&mut self, description: String) {
        self.history.begin_batch(description);
    }
    
    /// End the current batch of operations
    pub fn end_operation_batch(&mut self) {
        self.history.end_batch();
    }
    
    /// Check for batch timeout and auto-end if needed
    pub fn update_history(&mut self) {
        self.history.maybe_end_batch_on_timeout();
    }
    
    /// Get undo/redo information for UI display
    pub fn get_history_info(&self) -> (usize, usize, Option<&str>, Option<&str>) {
        (
            self.history.undo_count(),
            self.history.redo_count(),
            self.history.next_undo_description(),
            self.history.next_redo_description(),
        )
    }

    /// Render the design canvas with the root form and all components
    /// The Form is always rendered as the bottom-most layer (root component)
    pub fn render_design_canvas(
        &mut self,
        ui: &mut egui::Ui,
        root_form: &mut crate::rcl::ui::basic::form::Form,
        components: &mut Vec<Box<dyn crate::rcl::ui::component::Component>>,
        canvas_size: egui::Vec2,
    ) -> Option<usize> {
        // Get the available rect for the canvas
        let canvas_rect = ui.available_rect_before_wrap();
        
        // Calculate the form rect based on form size and center it in the canvas
        let form_size = root_form.size();
        let form_rect = egui::Rect::from_center_size(
            canvas_rect.center(),
            form_size
        );
        
        // CRITICAL: Render the root form background FIRST to ensure it's always at the bottom
        root_form.render_background(ui, form_rect);
        
        // Check if form is selected (form is index -1 or a special value)
        let form_selected = self.selection.selected.contains(&usize::MAX); // Use MAX as form indicator
        if form_selected {
            // Draw selection highlight for the form
            ui.painter().rect_stroke(
                form_rect,
                root_form.corner_radius(),
                egui::Stroke::new(2.0, egui::Color32::BLUE)
            );
            // Draw resize handles for the form
            self.draw_resize_handles(ui, form_rect);
        }

        // If no components exist on the form, show a helpful message
        if components.is_empty() {
            self.render_empty_form_message(ui, form_rect);
        }

        // Draw grid if enabled
        if self.grid.visible {
            self.draw_grid_in_rect(ui, canvas_rect);
        }

        // Draw rulers if enabled
        if self.guides.rulers_visible {
            self.guides.draw_rulers(ui, canvas_rect);
        }

        // Track if any component was clicked
        let mut clicked_component = None;
        
        // Handle form interaction (clicking on empty form area)
        let form_response = ui.interact(form_rect, egui::Id::new("root_form"), egui::Sense::click());
        if form_response.clicked() {
            // Select the form
            if !ui.input(|i| i.modifiers.ctrl) {
                self.selection.selected.clear();
            }
            self.selection.selected.insert(usize::MAX); // Use MAX as form indicator
            self.selection.primary = Some(usize::MAX);
            clicked_component = Some(usize::MAX);
        }
        
        // Handle right-click on form for context menu
        if form_response.secondary_clicked() {
            if let Some(pointer_pos) = ui.ctx().pointer_latest_pos() {
                self.context_menu.show_at(pointer_pos, Some(usize::MAX));
            }
        }

        // Update movement animations
        self.movement_manager.update_all(ui.ctx());
        
        // Update history (check for batch timeout)
        self.update_history();
        
        // Collect component data and handle interactions separately to avoid borrowing conflicts
        let mut component_data = Vec::new();
        let mut component_responses = Vec::new();
        
        // First pass: collect component data and render
        for (idx, component) in components.iter_mut().enumerate() {
            let layout_pos = self.layout.get_or_init_position(idx);
            let size = self.layout.get_or_init_size(idx, component.name());
            
            // Get animated position from movement manager
            let movement_anim = self.movement_manager.get_or_create(idx, layout_pos);
            let animated_pos = movement_anim.current_pos;
            
            // Create a rect for this component relative to the form using animated position
            let component_rect = egui::Rect::from_min_size(
                form_rect.min + animated_pos.to_vec2(), 
                size
            );
            
            // Check if component is selected
            let is_selected = self.selection.selected.contains(&idx);
            
            // Draw selection highlight
            if is_selected {
                ui.painter().rect_stroke(
                    component_rect,
                    2.0,
                    egui::Stroke::new(2.0, egui::Color32::BLUE)
                );
                
                // Draw resize handles for selected components
                self.draw_resize_handles(ui, component_rect);
            }
            
            // Render the component in its allocated space
            ui.allocate_ui_at_rect(component_rect, |ui| {
                component.render(ui);
            });
            
            // Handle interaction
            let response = ui.interact(component_rect, egui::Id::new(format!("component_{}", idx)), egui::Sense::click_and_drag());
            
            component_data.push((idx, layout_pos, size, animated_pos, component_rect, component.name().to_string()));
            component_responses.push((idx, response, is_selected));
        }
        
        // Second pass: handle interactions without borrowing conflicts
        for (idx, response, is_selected) in component_responses {
            if response.clicked() {
                // Select component
                if !ui.input(|i| i.modifiers.ctrl) {
                    self.selection.selected.clear();
                }
                self.selection.selected.insert(idx);
                self.selection.primary = Some(idx);
                clicked_component = Some(idx);
            }
            
            // Handle right-click for context menu
            if response.secondary_clicked() {
                // Ensure component is selected when right-clicked
                if !self.selection.selected.contains(&idx) {
                    self.selection.selected.clear();
                    self.selection.selected.insert(idx);
                    self.selection.primary = Some(idx);
                }
                
                if let Some(pointer_pos) = ui.ctx().pointer_latest_pos() {
                    self.context_menu.show_at(pointer_pos, Some(idx));
                }
            }
            
            if response.drag_started() && is_selected {
                // Start a move operation batch
                let description = if self.selection.selected.len() == 1 {
                    "Move Component".to_string()
                } else {
                    format!("Move {} Components", self.selection.selected.len())
                };
                self.begin_operation_batch(description);
            }
            
            if response.dragged() && is_selected {
                // Find component data for this index
                let (_, _, size, animated_pos, _, _) = component_data.iter()
                    .find(|(i, _, _, _, _, _)| *i == idx)
                    .unwrap();
                
                // Get current component rect for smart guides
                let current_rect = egui::Rect::from_min_size(
                    form_rect.min + animated_pos.to_vec2(), 
                    *size
                );
                
                // Collect other component rects for smart guides
                let mut other_rects = Vec::new();
                for (other_idx, other_pos, other_size, _, _, _) in &component_data {
                    if *other_idx != idx && !self.selection.selected.contains(other_idx) {
                        let other_rect = egui::Rect::from_min_size(
                            form_rect.min + other_pos.to_vec2(), 
                            *other_size
                        );
                        other_rects.push((*other_idx, other_rect));
                    }
                }
                
                // Generate smart alignment guides
                self.guides.generate_smart_guides(current_rect, &other_rects, form_rect);
                
                // Move selected components with smart snapping
                let delta = response.drag_delta();
                for &selected_idx in &self.selection.selected {
                    if let Some(pos) = self.layout.positions.get_mut(&selected_idx) {
                        // Calculate new position with delta
                        let new_pos = *pos + delta;
                        
                        // Apply smart snapping if enabled
                        let snapped_pos = if self.grid.snap_enabled {
                            // Find component size
                            let component_size = component_data.iter()
                                .find(|(i, _, _, _, _, _)| *i == selected_idx)
                                .map(|(_, _, size, _, _, _)| *size)
                                .unwrap_or_else(|| egui::Vec2::new(100.0, 32.0));
                            
                            // First try smart guide snapping
                            let guide_snapped = self.guides.get_snap_position(new_pos, component_size);
                            
                            // Then apply grid snapping if no guide snap occurred
                            if guide_snapped == new_pos && self.grid.snap_enabled {
                                // Calculate grid snapping without borrowing self
                                let grid_size = self.grid.size;
                                egui::Pos2::new(
                                    (new_pos.x / grid_size).round() * grid_size,
                                    (new_pos.y / grid_size).round() * grid_size
                                )
                            } else {
                                guide_snapped
                            }
                        } else {
                            new_pos
                        };
                        
                        // Update layout position
                        *pos = snapped_pos;
                        
                        // Animate to the new position
                        let movement_anim = self.movement_manager.get_or_create(selected_idx, snapped_pos);
                        movement_anim.move_to(snapped_pos);
                    }
                }
            }
            
            if response.drag_stopped() && is_selected {
                // End the move operation batch
                self.end_operation_batch();
                // Clear smart guides when drag stops
                self.guides.clear_smart_guides();
            } else if !response.dragged() {
                // Clear smart guides when not dragging
                self.guides.clear_smart_guides();
            }
        }
        
        // Handle drag and drop operations
        self.handle_drag_and_drop(ui, form_rect, components);
        
        // Draw guides and smart alignment aids
        let canvas_rect = egui::Rect::from_min_size(ui.cursor().left_top(), canvas_size);
        self.guides.draw_guides(ui, canvas_rect);
        self.guides.draw_smart_guides(ui, canvas_rect);
        
        // Handle keyboard shortcuts
        if let Some(action) = self.context_menu.handle_keyboard_shortcuts(ui, &self.selection.selected) {
            self.handle_context_menu_action(action, components);
        }
        
        // Render context menu and handle actions
        if let Some(action) = self.context_menu.render_context_menu(ui, &self.selection.selected) {
            self.handle_context_menu_action(action, components);
        }
        
        // Return the clicked component for selection synchronization
        clicked_component
    }

    /// Move a component by a delta vector with smooth animation
    pub fn move_component(&mut self, component_idx: usize, delta: egui::Vec2) {
        if let Some(pos) = self.layout.positions.get_mut(&component_idx) {
            *pos += delta;
            
            // Animate to the new position
            let movement_anim = self.movement_manager.get_or_create(component_idx, *pos);
            movement_anim.move_to(*pos);
        }
    }

    /// Resize a component to a new size
    pub fn resize_component(&mut self, component_idx: usize, new_size: egui::Vec2) {
        self.layout.sizes.insert(component_idx, new_size);
    }

    /// Move all selected components by a delta vector with smooth animation
    pub fn move_selected_components(&mut self, delta: egui::Vec2) {
        for &component_idx in &self.selection.selected {
            if let Some(pos) = self.layout.positions.get_mut(&component_idx) {
                *pos += delta;
                
                // Animate to the new position
                let movement_anim = self.movement_manager.get_or_create(component_idx, *pos);
                movement_anim.move_to(*pos);
            }
        }
    }

    /// Set the position of a component with smooth animation
    pub fn set_component_position(&mut self, component_idx: usize, position: egui::Pos2) {
        self.layout.positions.insert(component_idx, position);
        
        // Animate to the new position
        let movement_anim = self.movement_manager.get_or_create(component_idx, position);
        movement_anim.move_to(position);
    }

    /// Get the position of a component
    pub fn get_component_position(&mut self, component_idx: usize) -> egui::Pos2 {
        self.layout.positions.get(&component_idx).copied()
            .unwrap_or(egui::Pos2::ZERO)
    }
    
    /// Snap position to grid
    pub fn snap_to_grid(&self, pos: egui::Pos2) -> egui::Pos2 {
        if !self.grid.snap_enabled {
            return pos;
        }
        
        let grid_size = self.grid.size;
        egui::Pos2::new(
            (pos.x / grid_size).round() * grid_size,
            (pos.y / grid_size).round() * grid_size
        )
    }

    /// Draw grid on the canvas
    fn draw_grid(&self, ui: &mut egui::Ui, canvas_size: egui::Vec2) {
        let painter = ui.painter();
        let grid_size = self.grid.size;
        let canvas_rect = egui::Rect::from_min_size(ui.cursor().left_top(), canvas_size);
        
        // Draw vertical lines
        let mut x = canvas_rect.min.x;
        while x <= canvas_rect.max.x {
            painter.line_segment(
                [egui::pos2(x, canvas_rect.min.y), egui::pos2(x, canvas_rect.max.y)],
                egui::Stroke::new(1.0, self.grid.color)
            );
            x += grid_size;
        }
        
        // Draw horizontal lines
        let mut y = canvas_rect.min.y;
        while y <= canvas_rect.max.y {
            painter.line_segment(
                [egui::pos2(canvas_rect.min.x, y), egui::pos2(canvas_rect.max.x, y)],
                egui::Stroke::new(1.0, self.grid.color)
            );
            y += grid_size;
        }
    }

    /// Draw grid within a specific rectangle
    fn draw_grid_in_rect(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let painter = ui.painter();
        let grid_size = self.grid.size;
        
        // Draw vertical lines
        let mut x = rect.min.x + (grid_size - (rect.min.x % grid_size));
        while x <= rect.max.x {
            painter.line_segment(
                [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
                egui::Stroke::new(1.0, self.grid.color)
            );
            x += grid_size;
        }
        
        // Draw horizontal lines
        let mut y = rect.min.y + (grid_size - (rect.min.y % grid_size));
        while y <= rect.max.y {
            painter.line_segment(
                [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                egui::Stroke::new(1.0, self.grid.color)
            );
            y += grid_size;
        }
    }

    /// Render message for empty form
    fn render_empty_form_message(&self, ui: &mut egui::Ui, canvas_rect: egui::Rect) {
        let center = canvas_rect.center();
        
        // Draw welcome message
        ui.painter().text(
            center,
            egui::Align2::CENTER_CENTER,
            "Drop components here to start designing",
            egui::FontId::proportional(16.0),
            egui::Color32::from_gray(128)
        );
        
        // Draw a subtle dashed border to indicate drop area
        let inner_rect = canvas_rect.shrink(20.0);
        self.draw_dashed_border(ui, inner_rect);
    }

    /// Draw dashed border for drop zones
    fn draw_dashed_border(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let painter = ui.painter();
        let dash_length = 8.0;
        let gap_length = 4.0;
        let stroke = egui::Stroke::new(2.0, egui::Color32::from_gray(180));
        
        // Top edge
        let mut x = rect.min.x;
        while x < rect.max.x {
            let end_x = (x + dash_length).min(rect.max.x);
            painter.line_segment(
                [egui::pos2(x, rect.min.y), egui::pos2(end_x, rect.min.y)],
                stroke
            );
            x += dash_length + gap_length;
        }
        
        // Bottom edge
        x = rect.min.x;
        while x < rect.max.x {
            let end_x = (x + dash_length).min(rect.max.x);
            painter.line_segment(
                [egui::pos2(x, rect.max.y), egui::pos2(end_x, rect.max.y)],
                stroke
            );
            x += dash_length + gap_length;
        }
        
        // Left edge
        let mut y = rect.min.y;
        while y < rect.max.y {
            let end_y = (y + dash_length).min(rect.max.y);
            painter.line_segment(
                [egui::pos2(rect.min.x, y), egui::pos2(rect.min.x, end_y)],
                stroke
            );
            y += dash_length + gap_length;
        }
        
        // Right edge
        y = rect.min.y;
        while y < rect.max.y {
            let end_y = (y + dash_length).min(rect.max.y);
            painter.line_segment(
                [egui::pos2(rect.max.x, y), egui::pos2(rect.max.x, end_y)],
                stroke
            );
            y += dash_length + gap_length;
        }
    }

    /// Draw and handle interactive resize handles for components
    fn draw_and_handle_resize_handles(&mut self, ui: &mut egui::Ui, rect: egui::Rect, component_idx: usize, components: &mut Vec<Box<dyn crate::rcl::ui::component::Component>>) {
        let painter = ui.painter();
        let handle_size = 8.0;
        let handle_color = egui::Color32::BLUE;
        
        // Define resize handles with their resize directions
        let handles = [
            (egui::pos2(rect.min.x, rect.min.y), ResizeDirection::TopLeft),     // Top-left
            (egui::pos2(rect.max.x, rect.min.y), ResizeDirection::TopRight),    // Top-right
            (egui::pos2(rect.min.x, rect.max.y), ResizeDirection::BottomLeft),  // Bottom-left
            (egui::pos2(rect.max.x, rect.max.y), ResizeDirection::BottomRight), // Bottom-right
            (egui::pos2(rect.center().x, rect.min.y), ResizeDirection::Top),    // Top
            (egui::pos2(rect.center().x, rect.max.y), ResizeDirection::Bottom), // Bottom
            (egui::pos2(rect.min.x, rect.center().y), ResizeDirection::Left),   // Left
            (egui::pos2(rect.max.x, rect.center().y), ResizeDirection::Right),  // Right
        ];
        
        for (handle_pos, resize_direction) in handles {
            let handle_rect = egui::Rect::from_center_size(handle_pos, egui::Vec2::splat(handle_size));
            
            // Draw handle
            painter.rect_filled(handle_rect, 1.0, handle_color);
            painter.rect_stroke(handle_rect, 1.0, egui::Stroke::new(1.0, egui::Color32::WHITE));
            
            // Handle interaction
            let handle_id = egui::Id::new(format!("resize_handle_{}_{:?}", component_idx, resize_direction));
            let response = ui.interact(handle_rect, handle_id, egui::Sense::click_and_drag());
            
            // Change cursor based on resize direction
            if response.hovered() {
                match resize_direction {
                    ResizeDirection::TopLeft | ResizeDirection::BottomRight => {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeNwSe);
                    }
                    ResizeDirection::TopRight | ResizeDirection::BottomLeft => {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeNeSw);
                    }
                    ResizeDirection::Top | ResizeDirection::Bottom => {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeVertical);
                    }
                    ResizeDirection::Left | ResizeDirection::Right => {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
                    }
                }
            }
            
            // Handle dragging for resizing
            if response.dragged() {
                let delta = response.drag_delta();
                self.resize_component_interactive(component_idx, resize_direction, delta, components);
            }
        }
    }
    
    /// Draw and handle interactive resize handles for the form
    fn draw_and_handle_form_resize_handles(&mut self, ui: &mut egui::Ui, rect: egui::Rect) {
        let painter = ui.painter();
        let handle_size = 8.0;
        let handle_color = egui::Color32::from_rgb(100, 150, 255);
        
        // Form only gets corner and edge handles (simpler than components)
        let handles = [
            (egui::pos2(rect.max.x, rect.max.y), ResizeDirection::BottomRight), // Primary resize handle
            (egui::pos2(rect.max.x, rect.center().y), ResizeDirection::Right),  // Width only
            (egui::pos2(rect.center().x, rect.max.y), ResizeDirection::Bottom), // Height only
        ];
        
        for (handle_pos, resize_direction) in handles {
            let handle_rect = egui::Rect::from_center_size(handle_pos, egui::Vec2::splat(handle_size));
            
            // Draw handle with different color for form
            painter.rect_filled(handle_rect, 1.0, handle_color);
            painter.rect_stroke(handle_rect, 1.0, egui::Stroke::new(1.0, egui::Color32::WHITE));
            
            // Handle interaction
            let handle_id = egui::Id::new(format!("form_resize_handle_{:?}", resize_direction));
            let response = ui.interact(handle_rect, handle_id, egui::Sense::click_and_drag());
            
            // Change cursor
            if response.hovered() {
                match resize_direction {
                    ResizeDirection::BottomRight => {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeNwSe);
                    }
                    ResizeDirection::Right => {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
                    }
                    ResizeDirection::Bottom => {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeVertical);
                    }
                    _ => {}
                }
            }
            
            // Handle form resizing (this would need to be connected to the form state)
            if response.dragged() {
                let delta = response.drag_delta();
                // TODO: Resize form - this needs integration with the form state in app_state
                // For now, just show visual feedback
                println!("Form resize: {:?} by {:?}", resize_direction, delta);
            }
        }
    }
    
    /// Resize component interactively based on handle drag
    fn resize_component_interactive(&mut self, component_idx: usize, resize_direction: ResizeDirection, delta: egui::Vec2, components: &mut Vec<Box<dyn crate::rcl::ui::component::Component>>) {
        let current_size = self.layout.get_or_init_size(component_idx, 
            if component_idx < components.len() { components[component_idx].name() } else { "Unknown" });
        let current_pos = self.layout.get_or_init_position(component_idx);
        
        let (new_size, new_pos) = match resize_direction {
            ResizeDirection::Right => {
                (egui::Vec2::new((current_size.x + delta.x).max(20.0), current_size.y), current_pos)
            }
            ResizeDirection::Left => {
                let new_width = (current_size.x - delta.x).max(20.0);
                let width_diff = current_size.x - new_width;
                (egui::Vec2::new(new_width, current_size.y), 
                 egui::Pos2::new(current_pos.x + width_diff, current_pos.y))
            }
            ResizeDirection::Bottom => {
                (egui::Vec2::new(current_size.x, (current_size.y + delta.y).max(20.0)), current_pos)
            }
            ResizeDirection::Top => {
                let new_height = (current_size.y - delta.y).max(20.0);
                let height_diff = current_size.y - new_height;
                (egui::Vec2::new(current_size.x, new_height),
                 egui::Pos2::new(current_pos.x, current_pos.y + height_diff))
            }
            ResizeDirection::BottomRight => {
                (egui::Vec2::new((current_size.x + delta.x).max(20.0), (current_size.y + delta.y).max(20.0)), current_pos)
            }
            ResizeDirection::BottomLeft => {
                let new_width = (current_size.x - delta.x).max(20.0);
                let width_diff = current_size.x - new_width;
                (egui::Vec2::new(new_width, (current_size.y + delta.y).max(20.0)),
                 egui::Pos2::new(current_pos.x + width_diff, current_pos.y))
            }
            ResizeDirection::TopRight => {
                let new_height = (current_size.y - delta.y).max(20.0);
                let height_diff = current_size.y - new_height;
                (egui::Vec2::new((current_size.x + delta.x).max(20.0), new_height),
                 egui::Pos2::new(current_pos.x, current_pos.y + height_diff))
            }
            ResizeDirection::TopLeft => {
                let new_width = (current_size.x - delta.x).max(20.0);
                let new_height = (current_size.y - delta.y).max(20.0);
                let width_diff = current_size.x - new_width;
                let height_diff = current_size.y - new_height;
                (egui::Vec2::new(new_width, new_height),
                 egui::Pos2::new(current_pos.x + width_diff, current_pos.y + height_diff))
            }
        };
        
        // Apply constraints and snapping if enabled
        let constrained_size = self.apply_size_constraints(new_size);
        let snapped_pos = if self.grid.snap_enabled {
            self.snap_to_grid(new_pos)
        } else {
            new_pos
        };
        
        // Update layout
        self.layout.sizes.insert(component_idx, constrained_size);
        self.layout.positions.insert(component_idx, snapped_pos);
        
        // Animate to new position if position changed
        if snapped_pos != current_pos {
            let movement_anim = self.movement_manager.get_or_create(component_idx, snapped_pos);
            movement_anim.move_to(snapped_pos);
        }
    }
    
    /// Apply size constraints to prevent components from becoming too small or large
    fn apply_size_constraints(&self, size: egui::Vec2) -> egui::Vec2 {
        egui::Vec2::new(
            size.x.clamp(20.0, 800.0), // Min 20px, max 800px width
            size.y.clamp(20.0, 600.0)  // Min 20px, max 600px height
        )
    }
    
    /// Draw resize handles for selected components (simple version)
    fn draw_resize_handles(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let painter = ui.painter();
        let handle_size = 6.0;
        let handle_color = egui::Color32::BLUE;
        
        let handles = [
            // Corner handles
            egui::pos2(rect.min.x, rect.min.y), // Top-left
            egui::pos2(rect.max.x, rect.min.y), // Top-right
            egui::pos2(rect.min.x, rect.max.y), // Bottom-left
            egui::pos2(rect.max.x, rect.max.y), // Bottom-right
            // Edge handles
            egui::pos2(rect.center().x, rect.min.y), // Top
            egui::pos2(rect.center().x, rect.max.y), // Bottom
            egui::pos2(rect.min.x, rect.center().y), // Left
            egui::pos2(rect.max.x, rect.center().y), // Right
        ];
        
        for handle_pos in handles {
            let handle_rect = egui::Rect::from_center_size(handle_pos, egui::Vec2::splat(handle_size));
            painter.rect_filled(handle_rect, 0.0, handle_color);
            painter.rect_stroke(handle_rect, 0.0, egui::Stroke::new(1.0, egui::Color32::WHITE));
        }
    }
    
    /// Handle drag and drop operations
    fn handle_drag_and_drop(
        &mut self, 
        ui: &mut egui::Ui, 
        form_rect: egui::Rect,
        components: &mut Vec<Box<dyn crate::rcl::ui::component::Component>>
    ) {
        
        
        // Update drag position if dragging
        if self.drag_state.is_dragging {
            if let Some(pointer_pos) = ui.ctx().pointer_latest_pos() {
                self.drag_state.update_drag_position(pointer_pos);
                
                // Check if drop is valid (over the form)
                self.drag_state.drop_valid = form_rect.contains(pointer_pos);
                
                // Draw drag preview
                if let Some(preview_pos) = self.drag_state.preview_position {
                    self.draw_drag_preview(ui, preview_pos);
                }
            }
            
            // Check for drag end (mouse released)
            if ui.input(|i| i.pointer.any_released()) {
                if let Some(completion) = self.drag_state.end_drag() {
                    self.complete_drag_operation(completion, form_rect, components);
                }
            }
            
            // Check for drag cancel (ESC key)
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.drag_state.cancel_drag();
            }
        }
    }
    
    /// Draw drag preview
    fn draw_drag_preview(&self, ui: &mut egui::Ui, position: egui::Pos2) {
        use crate::ide_app::drag_drop::DragType;
        
        let painter = ui.painter();
        let preview_size = egui::Vec2::new(80.0, 30.0); // Default preview size
        let preview_rect = egui::Rect::from_center_size(position, preview_size);
        
        // Draw preview based on drag type
        match self.drag_state.drag_type {
            DragType::ComponentFromPalette(component_type) => {
                // Draw preview of the component being dragged
                let color = if self.drag_state.drop_valid {
                    egui::Color32::from_rgba_premultiplied(0, 255, 0, 100) // Green for valid drop
                } else {
                    egui::Color32::from_rgba_premultiplied(255, 0, 0, 100) // Red for invalid drop
                };
                
                painter.rect_filled(preview_rect, 5.0, color);
                painter.rect_stroke(preview_rect, 5.0, egui::Stroke::new(2.0, egui::Color32::WHITE));
                
                // Draw component type icon
                painter.text(
                    preview_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    component_type.icon(),
                    egui::FontId::proportional(16.0),
                    egui::Color32::WHITE
                );
            }
            _ => {
                // Default preview for other drag types
                painter.rect_filled(preview_rect, 5.0, egui::Color32::from_rgba_premultiplied(100, 100, 255, 100));
            }
        }
    }
    
    /// Complete a drag operation
    fn complete_drag_operation(
        &mut self,
        completion: crate::ide_app::drag_drop::DragCompletionResult,
        form_rect: egui::Rect,
        components: &mut Vec<Box<dyn crate::rcl::ui::component::Component>>
    ) {
        use crate::ide_app::drag_drop::DragType;
        
        
        match completion.drag_type {
            DragType::ComponentFromPalette(component_type) => {
                if let Some(drop_pos) = completion.preview_position {
                    if form_rect.contains(drop_pos) {
                        // Convert screen position to form-relative position
                        let relative_pos = drop_pos - form_rect.min.to_vec2();
                        
                        // Create and add the new component
                        self.create_and_add_component(component_type, egui::Pos2::new(relative_pos.x, relative_pos.y), components);
                    }
                }
            }
            DragType::ComponentMove => {
                // Component movement is already handled in the main render loop
                // This is just for cleanup
            }
            _ => {
                // Handle other drag types as needed
            }
        }
    }
    
    /// Create and add a new component
    fn create_and_add_component(
        &mut self,
        component_type: crate::ide_app::drag_drop::ComponentType,
        position: egui::Pos2,
        components: &mut Vec<Box<dyn crate::rcl::ui::component::Component>>
    ) {
        use crate::rcl::ui::component::Component;
        use crate::ide_app::drag_drop::ComponentType;
        
        let component: Box<dyn Component> = match component_type {
            ComponentType::Button => {
                Box::new(crate::rcl::ui::basic::button::Button::new("Button".to_string()))
            }
            ComponentType::Label => {
                Box::new(crate::rcl::ui::basic::label::Label::new("Label".to_string()))
            }
            ComponentType::TextBox => {
                Box::new(crate::rcl::ui::basic::textbox::TextBox::new("".to_string()))
            }
            ComponentType::Checkbox => {
                Box::new(crate::rcl::ui::basic::checkbox::Checkbox::new("Checkbox".to_string(), false))
            }
            ComponentType::Slider => {
                Box::new(crate::rcl::ui::basic::slider::Slider::new(0.0, 0.0, 100.0))
            }
            _ => {
                // Default to button for unsupported types
                Box::new(crate::rcl::ui::basic::button::Button::new("New Component".to_string()))
            }
        };
        
        let component_idx = components.len();
        components.push(component);
        
        // Set the layout position for the new component
        self.layout.positions.insert(component_idx, position);
        
        // Select the new component
        self.selection.selected.clear();
        self.selection.selected.insert(component_idx);
        self.selection.primary = Some(component_idx);
    }
    
    /// Handle context menu actions
    fn handle_context_menu_action(
        &mut self,
        action: ContextMenuAction,
        components: &mut Vec<Box<dyn crate::rcl::ui::component::Component>>
    ) {
        match action {
            ContextMenuAction::Copy => {
                self.context_menu.copy_to_clipboard(components, &self.selection.selected);
            }
            ContextMenuAction::Cut => {
                self.context_menu.cut_to_clipboard(components, &self.selection.selected);
                // Mark selected components for deletion (handled below)
                self.delete_selected_components(components);
            }
            ContextMenuAction::Paste => {
                if self.context_menu.can_paste() {
                    // Implementation would depend on clipboard format
                    // For now, just clear clipboard
                    self.context_menu.clear_clipboard();
                }
            }
            ContextMenuAction::Delete => {
                self.delete_selected_components(components);
            }
            ContextMenuAction::Duplicate => {
                self.duplicate_selected_components(components);
            }
            ContextMenuAction::BringToFront => {
                self.bring_selected_to_front(components);
            }
            ContextMenuAction::SendToBack => {
                self.send_selected_to_back(components);
            }
            ContextMenuAction::BringForward => {
                self.bring_selected_forward(components);
            }
            ContextMenuAction::SendBackward => {
                self.send_selected_backward(components);
            }
            ContextMenuAction::AlignLeft => {
                self.align_selected_components(AlignmentOperation::AlignLeft);
            }
            ContextMenuAction::AlignCenter => {
                self.align_selected_components(AlignmentOperation::AlignCenterHorizontal);
            }
            ContextMenuAction::AlignRight => {
                self.align_selected_components(AlignmentOperation::AlignRight);
            }
            ContextMenuAction::AlignTop => {
                self.align_selected_components(AlignmentOperation::AlignTop);
            }
            ContextMenuAction::AlignMiddle => {
                self.align_selected_components(AlignmentOperation::AlignCenterVertical);
            }
            ContextMenuAction::AlignBottom => {
                self.align_selected_components(AlignmentOperation::AlignBottom);
            }
            ContextMenuAction::DistributeHorizontally => {
                self.distribute_selected_components(true);
            }
            ContextMenuAction::DistributeVertically => {
                self.distribute_selected_components(false);
            }
            ContextMenuAction::MakeSameWidth => {
                self.make_selected_same_width(components);
            }
            ContextMenuAction::MakeSameHeight => {
                self.make_selected_same_height(components);
            }
            ContextMenuAction::MakeSameSize => {
                self.make_selected_same_size(components);
            }
            ContextMenuAction::ResetToDefault => {
                self.reset_selected_to_default(components);
            }
            _ => {
                // Handle other actions or show not implemented message
                println!("Context menu action {:?} not yet implemented", action);
            }
        }
    }
    
    /// Delete selected components
    fn delete_selected_components(&mut self, components: &mut Vec<Box<dyn crate::rcl::ui::component::Component>>) {
        let mut indices_to_remove: Vec<usize> = self.selection.selected.iter().copied().collect();
        indices_to_remove.sort_by(|a, b| b.cmp(a)); // Sort in reverse order to avoid index shifting
        
        for &idx in &indices_to_remove {
            if idx != usize::MAX && idx < components.len() { // Don't delete the form
                components.remove(idx);
                // Remove from layout data
                self.layout.positions.remove(&idx);
                self.layout.sizes.remove(&idx);
                // Shift indices for remaining components
                self.shift_component_indices_after_removal(idx);
            }
        }
        
        self.selection.selected.clear();
        self.selection.primary = None;
    }
    
    /// Duplicate selected components
    fn duplicate_selected_components(&mut self, components: &mut Vec<Box<dyn crate::rcl::ui::component::Component>>) {
        let selected: Vec<usize> = self.selection.selected.iter().copied().collect();
        let offset = egui::Vec2::new(20.0, 20.0); // Offset for duplicated components
        
        self.selection.selected.clear();
        
        for &idx in &selected {
            if idx != usize::MAX && idx < components.len() {
                // Clone component (this is a simplified approach)
                let original_component = &components[idx];
                let new_component = self.clone_component(original_component);
                
                let new_idx = components.len();
                components.push(new_component);
                
                // Copy layout data with offset
                if let Some(&pos) = self.layout.positions.get(&idx) {
                    self.layout.positions.insert(new_idx, pos + offset);
                }
                if let Some(&size) = self.layout.sizes.get(&idx) {
                    self.layout.sizes.insert(new_idx, size);
                }
                
                // Select the new component
                self.selection.selected.insert(new_idx);
                self.selection.primary = Some(new_idx);
            }
        }
    }
    
    /// Clone a component (simplified implementation)
    fn clone_component(&self, component: &Box<dyn crate::rcl::ui::component::Component>) -> Box<dyn crate::rcl::ui::component::Component> {
        // This is a simplified approach - in a real implementation you'd need proper component cloning
        use crate::rcl::ui::component::Component;
        
        match component.name() {
            "Button" => Box::new(crate::rcl::ui::basic::button::Button::new("Button Copy".to_string())),
            "Label" => Box::new(crate::rcl::ui::basic::label::Label::new("Label Copy".to_string())),
            "TextBox" => Box::new(crate::rcl::ui::basic::textbox::TextBox::new("".to_string())),
            "Checkbox" => Box::new(crate::rcl::ui::basic::checkbox::Checkbox::new("Checkbox Copy".to_string(), false)),
            "Slider" => Box::new(crate::rcl::ui::basic::slider::Slider::new(0.0, 0.0, 100.0)),
            _ => Box::new(crate::rcl::ui::basic::button::Button::new("Copy".to_string())),
        }
    }
    
    /// Shift component indices after a component is removed
    fn shift_component_indices_after_removal(&mut self, removed_idx: usize) {
        // Update layout positions and sizes
        let mut new_positions = std::collections::HashMap::new();
        let mut new_sizes = std::collections::HashMap::new();
        
        for (&idx, &pos) in &self.layout.positions {
            if idx > removed_idx {
                new_positions.insert(idx - 1, pos);
            } else if idx < removed_idx {
                new_positions.insert(idx, pos);
            }
        }
        
        for (&idx, &size) in &self.layout.sizes {
            if idx > removed_idx {
                new_sizes.insert(idx - 1, size);
            } else if idx < removed_idx {
                new_sizes.insert(idx, size);
            }
        }
        
        self.layout.positions = new_positions;
        self.layout.sizes = new_sizes;
        
        // Update selection indices
        let mut new_selection = std::collections::HashSet::new();
        for &idx in &self.selection.selected {
            if idx > removed_idx {
                new_selection.insert(idx - 1);
            } else if idx < removed_idx {
                new_selection.insert(idx);
            }
        }
        self.selection.selected = new_selection;
        
        // Update primary selection
        if let Some(primary) = self.selection.primary {
            if primary > removed_idx {
                self.selection.primary = Some(primary - 1);
            } else if primary == removed_idx {
                self.selection.primary = None;
            }
        }
    }
    
    /// Bring selected components to front (layer management placeholder)
    fn bring_selected_to_front(&mut self, _components: &mut Vec<Box<dyn crate::rcl::ui::component::Component>>) {
        // Layer management would be implemented here
        println!("Bring to front - layer management not yet implemented");
    }
    
    /// Send selected components to back (layer management placeholder)
    fn send_selected_to_back(&mut self, _components: &mut Vec<Box<dyn crate::rcl::ui::component::Component>>) {
        // Layer management would be implemented here
        println!("Send to back - layer management not yet implemented");
    }
    
    /// Bring selected components forward (layer management placeholder)
    fn bring_selected_forward(&mut self, _components: &mut Vec<Box<dyn crate::rcl::ui::component::Component>>) {
        // Layer management would be implemented here
        println!("Bring forward - layer management not yet implemented");
    }
    
    /// Send selected components backward (layer management placeholder)
    fn send_selected_backward(&mut self, _components: &mut Vec<Box<dyn crate::rcl::ui::component::Component>>) {
        // Layer management would be implemented here
        println!("Send backward - layer management not yet implemented");
    }
    
    /// Align selected components using existing alignment system
    fn align_selected_components(&mut self, alignment: AlignmentOperation) {
        if self.selection.selected.len() < 2 {
            return;
        }
        
        let selected: Vec<usize> = self.selection.selected.iter().copied().collect();
        self.layout.align_components(&selected, alignment);
        
        // Animate components to their new positions
        for &idx in &selected {
            if let Some(&new_pos) = self.layout.positions.get(&idx) {
                let movement_anim = self.movement_manager.get_or_create(idx, new_pos);
                movement_anim.move_to(new_pos);
            }
        }
    }
    
    /// Distribute selected components horizontally or vertically
    fn distribute_selected_components(&mut self, horizontal: bool) {
        if self.selection.selected.len() < 3 {
            return;
        }
        
        let selected: Vec<usize> = self.selection.selected.iter().copied().collect();
        self.layout.distribute_components(&selected, horizontal);
        
        // Animate components to their new positions
        for &idx in &selected {
            if let Some(&new_pos) = self.layout.positions.get(&idx) {
                let movement_anim = self.movement_manager.get_or_create(idx, new_pos);
                movement_anim.move_to(new_pos);
            }
        }
    }
    
    /// Make selected components the same width
    fn make_selected_same_width(&mut self, components: &[Box<dyn crate::rcl::ui::component::Component>]) {
        if self.selection.selected.len() < 2 {
            return;
        }
        
        // Find the reference width (from primary selection or first component)
        let reference_idx = self.selection.primary.unwrap_or_else(|| *self.selection.selected.iter().next().unwrap());
        let reference_width = self.layout.get_or_init_size(
            reference_idx,
            if reference_idx < components.len() { components[reference_idx].name() } else { "Unknown" }
        ).x;
        
        // Apply the width to all selected components
        for &idx in &self.selection.selected {
            if let Some(current_size) = self.layout.sizes.get_mut(&idx) {
                current_size.x = reference_width;
            }
        }
    }
    
    /// Make selected components the same height
    fn make_selected_same_height(&mut self, components: &[Box<dyn crate::rcl::ui::component::Component>]) {
        if self.selection.selected.len() < 2 {
            return;
        }
        
        // Find the reference height (from primary selection or first component)
        let reference_idx = self.selection.primary.unwrap_or_else(|| *self.selection.selected.iter().next().unwrap());
        let reference_height = self.layout.get_or_init_size(
            reference_idx,
            if reference_idx < components.len() { components[reference_idx].name() } else { "Unknown" }
        ).y;
        
        // Apply the height to all selected components
        for &idx in &self.selection.selected {
            if let Some(current_size) = self.layout.sizes.get_mut(&idx) {
                current_size.y = reference_height;
            }
        }
    }
    
    /// Make selected components the same size
    fn make_selected_same_size(&mut self, components: &[Box<dyn crate::rcl::ui::component::Component>]) {
        if self.selection.selected.len() < 2 {
            return;
        }
        
        // Find the reference size (from primary selection or first component)
        let reference_idx = self.selection.primary.unwrap_or_else(|| *self.selection.selected.iter().next().unwrap());
        let reference_size = self.layout.get_or_init_size(
            reference_idx,
            if reference_idx < components.len() { components[reference_idx].name() } else { "Unknown" }
        );
        
        // Apply the size to all selected components
        for &idx in &self.selection.selected {
            self.layout.sizes.insert(idx, reference_size);
        }
    }
    
    /// Reset selected components to default properties
    fn reset_selected_to_default(&mut self, components: &mut [Box<dyn crate::rcl::ui::component::Component>]) {
        for &idx in &self.selection.selected {
            if idx != usize::MAX && idx < components.len() {
                // Reset component properties to default
                // This would depend on having a reset method on components
                println!("Reset to default for component {} - not yet implemented", idx);
            }
        }
    }
}
