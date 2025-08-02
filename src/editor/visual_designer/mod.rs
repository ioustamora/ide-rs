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
}

impl VisualDesigner {
    pub fn new() -> Self {
        Self::default()
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

    /// Render the design canvas with all components
    pub fn render_design_canvas(
        &mut self,
        ui: &mut egui::Ui,
        components: &mut Vec<Box<dyn crate::rcl::ui::component::Component>>,
        canvas_size: egui::Vec2,
    ) {
        // Draw grid if enabled
        if self.grid.visible {
            self.draw_grid(ui, canvas_size);
        }

        // Draw rulers if enabled
        if self.guides.rulers_visible {
            let canvas_rect = egui::Rect::from_min_size(ui.cursor().left_top(), canvas_size);
            self.guides.draw_rulers(ui, canvas_rect);
        }

        // Render components
        for (idx, component) in components.iter_mut().enumerate() {
            let pos = self.layout.get_or_init_position(idx);
            let size = self.layout.get_or_init_size(idx, component.name());
            
            // Create a rect for this component
            let component_rect = egui::Rect::from_min_size(pos, size);
            
            // Check if component is selected
            let is_selected = self.selection.selected.contains(&idx);
            
            // Draw selection highlight
            if is_selected {
                ui.painter().rect_stroke(
                    component_rect,
                    2.0,
                    egui::Stroke::new(2.0, egui::Color32::BLUE)
                );
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
            }
            
            if response.dragged() && is_selected {
                // Move selected components
                let delta = response.drag_delta();
                for &selected_idx in &self.selection.selected {
                    if let Some(pos) = self.layout.positions.get_mut(&selected_idx) {
                        *pos += delta;
                    }
                }
            }
        }
        
        // Draw guides
        let canvas_rect = egui::Rect::from_min_size(ui.cursor().left_top(), canvas_size);
        self.guides.draw_guides(ui, canvas_rect);
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
}
