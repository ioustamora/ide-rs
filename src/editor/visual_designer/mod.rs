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
                // Move selected components with smooth animation
                let delta = response.drag_delta();
                for &selected_idx in &self.selection.selected {
                    if let Some(pos) = self.layout.positions.get_mut(&selected_idx) {
                        // Update layout position immediately for responsiveness
                        *pos += delta;
                        
                        // Animate to the new position
                        let movement_anim = self.movement_manager.get_or_create(selected_idx, *pos);
                        movement_anim.move_to(*pos);
                    }
                }
            }
        }
        
        // Handle drag and drop operations
        self.handle_drag_and_drop(ui, form_rect, components);
        
        // Draw guides
        let canvas_rect = egui::Rect::from_min_size(ui.cursor().left_top(), canvas_size);
        self.guides.draw_guides(ui, canvas_rect);
        
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

    /// Draw resize handles for selected components
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
