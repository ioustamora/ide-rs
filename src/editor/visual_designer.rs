//! Advanced Visual Form Designer - RAD Studio inspired WYSIWYG designer
//!
//! This module provides a professional visual form designer with:
//! - Grid-based layout system with snap-to-grid
//! - Multi-select and group operations
//! - Alignment and distribution tools
//! - Visual guides and rulers
//! - Undo/Redo system
//! - Real-time property preview

use eframe::egui;
use crate::rcl::ui::component::Component;
use std::collections::HashMap;

/// Visual designer state and operations
pub struct VisualDesigner {
    /// Grid settings for alignment
    pub grid: GridSettings,
    /// Currently selected components
    pub selection: ComponentSelection,
    /// Undo/Redo history
    pub history: DesignHistory,
    /// Visual guides and rulers
    pub guides: GuideSystem,
    /// Component positioning and layout
    pub layout: LayoutManager,
    /// Design-time properties
    pub design_props: HashMap<usize, DesignTimeProperties>,
}

/// Grid system for component alignment
#[derive(Clone)]
pub struct GridSettings {
    /// Grid size in pixels
    pub size: f32,
    /// Whether grid is visible
    pub visible: bool,
    /// Whether snap-to-grid is enabled
    pub snap_enabled: bool,
    /// Grid color
    pub color: egui::Color32,
    /// Major grid lines (every N lines)
    pub major_lines: u32,
}

/// Component selection management
pub struct ComponentSelection {
    /// Selected component indices
    pub selected: Vec<usize>,
    /// Primary selection (for properties editing)
    pub primary: Option<usize>,
    /// Selection rectangle
    pub selection_rect: Option<egui::Rect>,
    /// Multi-select mode
    pub multi_select_mode: bool,
}

/// Undo/Redo system for design operations
pub struct DesignHistory {
    /// History of design operations
    pub operations: Vec<DesignOperation>,
    /// Current position in history
    pub current_index: usize,
    /// Maximum history size
    pub max_size: usize,
}

/// Types of design operations that can be undone
#[derive(Clone)]
pub enum DesignOperation {
    /// Component moved
    Move { 
        component_ids: Vec<usize>, 
        old_positions: Vec<egui::Pos2>, 
        new_positions: Vec<egui::Pos2> 
    },
    /// Component resized
    Resize { 
        component_id: usize, 
        old_size: egui::Vec2, 
        new_size: egui::Vec2 
    },
    /// Component added
    Add { 
        component_id: usize, 
        position: egui::Pos2 
    },
    /// Component deleted
    Delete { 
        component_id: usize, 
        component_data: String // Serialized component
    },
    /// Property changed
    PropertyChange { 
        component_id: usize, 
        property_name: String, 
        old_value: String, 
        new_value: String 
    },
}

/// Visual guides and rulers system
pub struct GuideSystem {
    /// Horizontal guides
    pub horizontal_guides: Vec<f32>,
    /// Vertical guides
    pub vertical_guides: Vec<f32>,
    /// Whether rulers are visible
    pub rulers_visible: bool,
    /// Ruler color
    pub ruler_color: egui::Color32,
    /// Guide color
    pub guide_color: egui::Color32,
}

/// Layout management for components
pub struct LayoutManager {
    /// Component positions
    pub positions: HashMap<usize, egui::Pos2>,
    /// Component sizes
    pub sizes: HashMap<usize, egui::Vec2>,
    /// Z-order (layering)
    pub z_order: Vec<usize>,
    /// Alignment helpers
    pub alignment: AlignmentTools,
}

/// Alignment and distribution tools
pub struct AlignmentTools {
    /// Last alignment operation
    pub last_operation: Option<AlignmentOperation>,
}

/// Types of alignment operations
#[derive(Clone, Copy)]
pub enum AlignmentOperation {
    AlignLeft,
    AlignRight,
    AlignTop,
    AlignBottom,
    AlignCenterHorizontal,
    AlignCenterVertical,
    DistributeHorizontal,
    DistributeVertical,
    SameWidth,
    SameHeight,
    SameSize,
}

/// Design-time properties for components
#[derive(Clone)]
pub struct DesignTimeProperties {
    /// Component name (for code generation)
    pub name: String,
    /// Whether component is locked (cannot be moved/resized)
    pub locked: bool,
    /// Whether component is visible at design time
    pub visible: bool,
    /// Custom properties
    pub custom_props: HashMap<String, String>,
}

impl Default for GridSettings {
    fn default() -> Self {
        Self {
            size: 8.0,
            visible: true,
            snap_enabled: true,
            color: egui::Color32::from_rgba_unmultiplied(200, 200, 200, 50),
            major_lines: 5,
        }
    }
}

impl Default for ComponentSelection {
    fn default() -> Self {
        Self {
            selected: Vec::new(),
            primary: None,
            selection_rect: None,
            multi_select_mode: false,
        }
    }
}

impl Default for DesignHistory {
    fn default() -> Self {
        Self {
            operations: Vec::new(),
            current_index: 0,
            max_size: 100,
        }
    }
}

impl Default for GuideSystem {
    fn default() -> Self {
        Self {
            horizontal_guides: Vec::new(),
            vertical_guides: Vec::new(),
            rulers_visible: true,
            ruler_color: egui::Color32::from_rgb(100, 100, 100),
            guide_color: egui::Color32::from_rgb(0, 150, 255),
        }
    }
}

impl Default for LayoutManager {
    fn default() -> Self {
        Self {
            positions: HashMap::new(),
            sizes: HashMap::new(),
            z_order: Vec::new(),
            alignment: AlignmentTools::default(),
        }
    }
}

impl Default for AlignmentTools {
    fn default() -> Self {
        Self {
            last_operation: None,
        }
    }
}

impl VisualDesigner {
    /// Create a new visual designer
    pub fn new() -> Self {
        Self {
            grid: GridSettings::default(),
            selection: ComponentSelection::default(),
            history: DesignHistory::default(),
            guides: GuideSystem::default(),
            layout: LayoutManager::default(),
            design_props: HashMap::new(),
        }
    }

    /// Render the design canvas with grid, guides, and components
    pub fn render_design_canvas(&mut self, ui: &mut egui::Ui, components: &mut [Box<dyn Component>], canvas_size: egui::Vec2) {
        let canvas_rect = egui::Rect::from_min_size(ui.cursor().min, canvas_size);
        
        // Draw grid if visible
        if self.grid.visible {
            self.draw_grid(ui, canvas_rect);
        }
        
        // Draw rulers if visible
        if self.guides.rulers_visible {
            self.draw_rulers(ui, canvas_rect);
        }
        
        // Draw guides
        self.draw_guides(ui, canvas_rect);
        
        // Handle component interaction
        self.handle_component_interaction(ui, components, canvas_rect);
        
        // Draw selection indicators
        self.draw_selection_indicators(ui, components);
        
        // Draw alignment helpers
        self.draw_alignment_helpers(ui, components);
    }

    /// Draw the grid system
    fn draw_grid(&self, ui: &mut egui::Ui, canvas_rect: egui::Rect) {
        let painter = ui.painter();
        let grid_size = self.grid.size;
        
        // Vertical grid lines
        let mut x = canvas_rect.min.x;
        let mut line_count = 0;
        while x <= canvas_rect.max.x {
            let is_major = line_count % self.grid.major_lines == 0;
            let color = if is_major {
                self.grid.color.gamma_multiply(2.0)
            } else {
                self.grid.color
            };
            
            painter.line_segment(
                [egui::pos2(x, canvas_rect.min.y), egui::pos2(x, canvas_rect.max.y)],
                egui::Stroke::new(if is_major { 1.0 } else { 0.5 }, color)
            );
            
            x += grid_size;
            line_count += 1;
        }
        
        // Horizontal grid lines
        let mut y = canvas_rect.min.y;
        line_count = 0;
        while y <= canvas_rect.max.y {
            let is_major = line_count % self.grid.major_lines == 0;
            let color = if is_major {
                self.grid.color.gamma_multiply(2.0)
            } else {
                self.grid.color
            };
            
            painter.line_segment(
                [egui::pos2(canvas_rect.min.x, y), egui::pos2(canvas_rect.max.x, y)],
                egui::Stroke::new(if is_major { 1.0 } else { 0.5 }, color)
            );
            
            y += grid_size;
            line_count += 1;
        }
    }

    /// Draw rulers on the canvas
    fn draw_rulers(&self, ui: &mut egui::Ui, canvas_rect: egui::Rect) {
        let painter = ui.painter();
        let ruler_size = 20.0;
        
        // Horizontal ruler
        let h_ruler_rect = egui::Rect::from_min_size(
            egui::pos2(canvas_rect.min.x, canvas_rect.min.y - ruler_size),
            egui::vec2(canvas_rect.width(), ruler_size)
        );
        painter.rect_filled(h_ruler_rect, 0.0, self.guides.ruler_color);
        
        // Vertical ruler
        let v_ruler_rect = egui::Rect::from_min_size(
            egui::pos2(canvas_rect.min.x - ruler_size, canvas_rect.min.y),
            egui::vec2(ruler_size, canvas_rect.height())
        );
        painter.rect_filled(v_ruler_rect, 0.0, self.guides.ruler_color);
        
        // Add measurement marks
        self.draw_ruler_marks(ui, h_ruler_rect, v_ruler_rect);
    }

    /// Draw measurement marks on rulers
    fn draw_ruler_marks(&self, ui: &mut egui::Ui, h_ruler: egui::Rect, v_ruler: egui::Rect) {
        let painter = ui.painter();
        let mark_interval = 50.0; // pixels between major marks
        
        // Horizontal ruler marks
        let mut x = h_ruler.min.x;
        let mut pixel_count = 0;
        while x <= h_ruler.max.x {
            let mark_height = if pixel_count % 100 == 0 { 8.0 } else { 4.0 };
            painter.line_segment(
                [egui::pos2(x, h_ruler.max.y - mark_height), egui::pos2(x, h_ruler.max.y)],
                egui::Stroke::new(1.0, egui::Color32::WHITE)
            );
            
            if pixel_count % 100 == 0 {
                painter.text(
                    egui::pos2(x + 2.0, h_ruler.max.y - 12.0),
                    egui::Align2::LEFT_BOTTOM,
                    pixel_count.to_string(),
                    egui::FontId::monospace(8.0),
                    egui::Color32::WHITE
                );
            }
            
            x += mark_interval;
            pixel_count += mark_interval as i32;
        }
        
        // Vertical ruler marks
        let mut y = v_ruler.min.y;
        pixel_count = 0;
        while y <= v_ruler.max.y {
            let mark_width = if pixel_count % 100 == 0 { 8.0 } else { 4.0 };
            painter.line_segment(
                [egui::pos2(v_ruler.max.x - mark_width, y), egui::pos2(v_ruler.max.x, y)],
                egui::Stroke::new(1.0, egui::Color32::WHITE)
            );
            
            if pixel_count % 100 == 0 {
                painter.text(
                    egui::pos2(v_ruler.max.x - 2.0, y - 2.0),
                    egui::Align2::RIGHT_BOTTOM,
                    pixel_count.to_string(),
                    egui::FontId::monospace(8.0),
                    egui::Color32::WHITE
                );
            }
            
            y += mark_interval;
            pixel_count += mark_interval as i32;
        }
    }

    /// Draw visual guides
    fn draw_guides(&self, ui: &mut egui::Ui, canvas_rect: egui::Rect) {
        let painter = ui.painter();
        
        // Draw horizontal guides
        for &y in &self.guides.horizontal_guides {
            painter.line_segment(
                [egui::pos2(canvas_rect.min.x, y), egui::pos2(canvas_rect.max.x, y)],
                egui::Stroke::new(1.0, self.guides.guide_color)
            );
        }
        
        // Draw vertical guides
        for &x in &self.guides.vertical_guides {
            painter.line_segment(
                [egui::pos2(x, canvas_rect.min.y), egui::pos2(x, canvas_rect.max.y)],
                egui::Stroke::new(1.0, self.guides.guide_color)
            );
        }
    }

    /// Handle component interaction (selection, moving, resizing)
    fn handle_component_interaction(&mut self, ui: &mut egui::Ui, components: &mut [Box<dyn Component>], _canvas_rect: egui::Rect) {
        // Component interaction logic will be implemented here
        // This includes:
        // - Click to select
        // - Drag to move
        // - Resize handles
        // - Multi-select with Ctrl+click or drag rectangle
        
        // For now, placeholder implementation
        for (idx, _component) in components.iter().enumerate() {
            if let Some(pos) = self.layout.positions.get(&idx) {
                let size = self.layout.sizes.get(&idx).cloned().unwrap_or(egui::vec2(100.0, 30.0));
                let component_rect = egui::Rect::from_min_size(*pos, size);
                
                // Simple click to select for now
                let response = ui.allocate_rect(component_rect, egui::Sense::click_and_drag());
                if response.clicked() {
                    self.select_component(idx);
                }
            }
        }
    }

    /// Draw selection indicators around selected components
    fn draw_selection_indicators(&self, ui: &mut egui::Ui, _components: &[Box<dyn Component>]) {
        let painter = ui.painter();
        
        for &component_idx in &self.selection.selected {
            if let Some(pos) = self.layout.positions.get(&component_idx) {
                let size = self.layout.sizes.get(&component_idx).cloned().unwrap_or(egui::vec2(100.0, 30.0));
                let rect = egui::Rect::from_min_size(*pos, size);
                
                // Draw selection rectangle
                painter.rect_stroke(
                    rect.expand(2.0),
                    2.0,
                    egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 120, 255))
                );
                
                // Draw resize handles without borrowing ui mutably
                self.draw_resize_handles(&painter, rect);
            }
        }
    }

    /// Draw resize handles for selected components
    fn draw_resize_handles(&self, painter: &egui::Painter, rect: egui::Rect) {
        let handle_size = 6.0;
        let handle_color = egui::Color32::from_rgb(0, 120, 255);
        
        // Corner handles
        let handles = [
            rect.min,                                    // Top-left
            egui::pos2(rect.max.x, rect.min.y),         // Top-right
            rect.max,                                    // Bottom-right
            egui::pos2(rect.min.x, rect.max.y),         // Bottom-left
            // Edge handles
            egui::pos2(rect.center().x, rect.min.y),    // Top-center
            egui::pos2(rect.max.x, rect.center().y),    // Right-center
            egui::pos2(rect.center().x, rect.max.y),    // Bottom-center
            egui::pos2(rect.min.x, rect.center().y),    // Left-center
        ];
        
        for handle_pos in handles {
            let handle_rect = egui::Rect::from_center_size(handle_pos, egui::vec2(handle_size, handle_size));
            painter.rect_filled(handle_rect, 1.0, handle_color);
            painter.rect_stroke(handle_rect, 1.0, egui::Stroke::new(1.0, egui::Color32::WHITE));
        }
    }

    /// Draw alignment helpers (snap lines, etc.)
    fn draw_alignment_helpers(&self, _ui: &mut egui::Ui, _components: &[Box<dyn Component>]) {
        // This will show temporary alignment lines when dragging components
        // to help with visual alignment
        
        // Placeholder for now
    }

    /// Select a component
    pub fn select_component(&mut self, component_idx: usize) {
        if !self.selection.multi_select_mode {
            self.selection.selected.clear();
        }
        
        if !self.selection.selected.contains(&component_idx) {
            self.selection.selected.push(component_idx);
        }
        
        self.selection.primary = Some(component_idx);
    }

    /// Apply alignment operation to selected components
    pub fn apply_alignment(&mut self, operation: AlignmentOperation) {
        if self.selection.selected.len() < 2 {
            return; // Need at least 2 components for alignment
        }
        
        let mut positions: Vec<(usize, egui::Pos2)> = self.selection.selected
            .iter()
            .filter_map(|&idx| self.layout.positions.get(&idx).map(|pos| (idx, *pos)))
            .collect();
        
        match operation {
            AlignmentOperation::AlignLeft => {
                if let Some(min_x) = positions.iter().map(|(_, pos)| pos.x).min_by(|a, b| a.partial_cmp(b).unwrap()) {
                    for (idx, pos) in &mut positions {
                        pos.x = min_x;
                        self.layout.positions.insert(*idx, *pos);
                    }
                }
            }
            AlignmentOperation::AlignRight => {
                if let Some(max_x) = positions.iter().map(|(_, pos)| pos.x).max_by(|a, b| a.partial_cmp(b).unwrap()) {
                    for (idx, pos) in &mut positions {
                        pos.x = max_x;
                        self.layout.positions.insert(*idx, *pos);
                    }
                }
            }
            // Additional alignment operations would be implemented here
            _ => {} // Placeholder for other operations
        }
        
        self.layout.alignment.last_operation = Some(operation);
    }

    /// Snap position to grid if enabled
    pub fn snap_to_grid(&self, pos: egui::Pos2) -> egui::Pos2 {
        if !self.grid.snap_enabled {
            return pos;
        }
        
        let grid_size = self.grid.size;
        egui::pos2(
            (pos.x / grid_size).round() * grid_size,
            (pos.y / grid_size).round() * grid_size
        )
    }

    /// Add operation to history for undo/redo
    pub fn add_to_history(&mut self, operation: DesignOperation) {
        // Remove any operations after current index (if we're not at the end)
        self.history.operations.truncate(self.history.current_index);
        
        // Add new operation
        self.history.operations.push(operation);
        self.history.current_index = self.history.operations.len();
        
        // Limit history size
        if self.history.operations.len() > self.history.max_size {
            self.history.operations.remove(0);
            self.history.current_index -= 1;
        }
    }

    /// Undo last operation
    pub fn undo(&mut self) -> bool {
        if self.history.current_index == 0 {
            return false;
        }
        
        self.history.current_index -= 1;
        let operation = &self.history.operations[self.history.current_index].clone();
        
        // Apply reverse operation
        match operation {
            DesignOperation::Move { component_ids, old_positions, .. } => {
                for (i, &component_id) in component_ids.iter().enumerate() {
                    if let Some(old_pos) = old_positions.get(i) {
                        self.layout.positions.insert(component_id, *old_pos);
                    }
                }
            }
            // Other operation reversals would be implemented here
            _ => {}
        }
        
        true
    }

    /// Redo next operation
    pub fn redo(&mut self) -> bool {
        if self.history.current_index >= self.history.operations.len() {
            return false;
        }
        
        let operation = &self.history.operations[self.history.current_index].clone();
        self.history.current_index += 1;
        
        // Apply operation
        match operation {
            DesignOperation::Move { component_ids, new_positions, .. } => {
                for (i, &component_id) in component_ids.iter().enumerate() {
                    if let Some(new_pos) = new_positions.get(i) {
                        self.layout.positions.insert(component_id, *new_pos);
                    }
                }
            }
            // Other operation applications would be implemented here
            _ => {}
        }
        
        true
    }
}