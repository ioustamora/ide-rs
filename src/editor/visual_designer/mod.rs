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
}

impl VisualDesigner {
    pub fn new() -> Self {
        let mut designer = Self::default();
        designer.movement_manager = MovementManager::new();
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
    pub fn undo(&mut self) -> bool {
        self.history.undo(&mut self.layout)
    }

    /// Redo last undone operation
    pub fn redo(&mut self) -> bool {
        self.history.redo(&mut self.layout)
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

        // Update movement animations
        self.movement_manager.update_all(ui.ctx());
        
        // Render components on top of the form
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
            
            if response.clicked() {
                // Select component
                if !ui.input(|i| i.modifiers.ctrl) {
                    self.selection.selected.clear();
                }
                self.selection.selected.insert(idx);
                self.selection.primary = Some(idx);
                clicked_component = Some(idx);
            }
            
            if response.dragged() && is_selected {
                // Get current component rect for smart guides
                let current_rect = egui::Rect::from_min_size(
                    form_rect.min + animated_pos.to_vec2(), 
                    size
                );
                
                // Collect other component rects for smart guides
                let mut other_rects = Vec::new();
                for (other_idx, _) in components.iter().enumerate() {
                    if other_idx != idx && !self.selection.selected.contains(&other_idx) {
                        let other_pos = self.layout.get_or_init_position(other_idx);
                        let other_size = self.layout.get_or_init_size(other_idx, components[other_idx].name());
                        let other_rect = egui::Rect::from_min_size(
                            form_rect.min + other_pos.to_vec2(), 
                            other_size
                        );
                        other_rects.push((other_idx, other_rect));
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
                            let component_size = self.layout.get_or_init_size(selected_idx, 
                                if selected_idx < components.len() { components[selected_idx].name() } else { "Unknown" });
                            let component_rect = egui::Rect::from_min_size(form_rect.min + new_pos.to_vec2(), component_size);
                            
                            // First try smart guide snapping
                            let guide_snapped = self.guides.get_snap_position(new_pos, component_size);
                            
                            // Then apply grid snapping if no guide snap occurred
                            if guide_snapped == new_pos && self.grid.snap_enabled {
                                self.snap_to_grid(new_pos)
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
            } else {
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
}
