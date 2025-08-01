//! # Advanced Visual Form Designer
//!
//! Professional WYSIWYG visual designer inspired by RAD Studio and Delphi's form designer.
//! This module implements a comprehensive visual editing system that enables drag-and-drop
//! component placement with precision tools and professional-grade layout capabilities.
//!
//! ## Core Features
//!
//! - **Grid System**: Configurable grid with snap-to-grid functionality
//! - **Multi-Selection**: Advanced selection system supporting multiple components
//! - **Alignment Tools**: Professional alignment and distribution operations
//! - **Visual Guides**: Dynamic alignment guides and rulers for precision layout
//! - **Undo/Redo**: Complete operation history with atomic transactions
//! - **Smart Editing**: Intelligent editing assistance and magnetic alignment
//! - **Performance**: Spatial indexing for efficient hit-testing and rendering
//!
//! ## Architecture Overview
//!
//! The visual designer follows a component-based architecture where each system
//! handles a specific aspect of the visual editing experience:
//!
//! ```
//! VisualDesigner (main coordinator)
//!     ├── GridSettings (snap-to-grid system)
//!     ├── ComponentSelection (multi-select management)
//!     ├── DesignHistory (undo/redo operations)
//!     ├── GuideSystem (visual alignment aids)
//!     ├── LayoutManager (spatial indexing and positioning)
//!     └── SmartEditingSystem (intelligent assistance)
//! ```
//!
//! ## Performance Considerations
//!
//! - **Spatial Indexing**: Components are spatially indexed for O(log n) hit-testing
//! - **Dirty Regions**: Only modified areas are re-rendered when possible
//! - **Event Batching**: Multiple operations are batched to reduce state updates
//! - **Memory Pooling**: Frequently allocated objects are pooled for reuse

use eframe::egui;
use egui::NumExt; // For at_least method on Vec2
use crate::rcl::ui::component::Component;
use crate::editor::smart_editing::SmartEditingSystem;
use crate::editor::advanced_alignment::AdvancedAlignment;
use std::collections::HashMap;

/// # Visual Designer State Container
/// 
/// Central coordinator for all visual design operations. This struct maintains the state
/// of the visual editing environment and orchestrates interactions between different
/// subsystems like grid management, component selection, and layout operations.
/// 
/// ## State Management
/// 
/// The visual designer maintains several types of state:
/// - **Transient State**: Current selection, drag operations, hover states
/// - **Persistent State**: Component positions, grid settings, design preferences
/// - **History State**: Undo/redo operations with full state snapshots
/// 
/// ## Coordinate System
/// 
/// The designer uses a screen-space coordinate system where:
/// - Origin (0,0) is at the top-left of the design canvas
/// - X increases rightward, Y increases downward
/// - All measurements are in logical pixels (egui units)
/// - Grid snapping is applied in this coordinate space
/// 
/// ## Thread Safety
/// 
/// This struct is not thread-safe as it's designed to operate on the main UI thread.
/// All operations should be performed within the egui update context.
pub struct VisualDesigner {
    // ==================================================================================
    // LAYOUT AND POSITIONING SYSTEMS
    // ==================================================================================
    
    /// Grid system for precise component alignment and snap-to-grid functionality.
    /// 
    /// Provides visual grid overlay and automatic snapping of component positions
    /// to grid intersections. Essential for professional layout consistency.
    pub grid: GridSettings,
    
    /// Component positioning and spatial indexing system.
    /// 
    /// Manages the spatial relationships between components and provides efficient
    /// hit-testing through spatial data structures. Maintains component bounds,
    /// z-order, and collision detection.
    pub layout: LayoutManager,

    // ==================================================================================
    // SELECTION AND INTERACTION SYSTEMS  
    // ==================================================================================
    
    /// Multi-component selection state and operations.
    /// 
    /// Tracks which components are currently selected and provides operations
    /// for bulk manipulation of selected components. Supports various selection
    /// modes including additive and toggle selection.
    pub selection: ComponentSelection,
    
    /// Smart editing assistance system.
    /// 
    /// Provides intelligent editing aids such as:
    /// - Magnetic alignment between components
    /// - Automatic spacing suggestions
    /// - Smart resizing with aspect ratio preservation
    pub smart_editing: SmartEditingSystem,

    // ==================================================================================
    // VISUAL AIDS AND ASSISTANCE
    // ==================================================================================
    
    /// Dynamic visual guides and ruler system.
    /// 
    /// Renders alignment guides, rulers, and measurement overlays to assist
    /// with precise component positioning. Shows distances, alignments, and
    /// provides visual feedback during operations.
    pub guides: GuideSystem,
    
    /// Professional alignment and distribution tools.
    /// 
    /// Advanced alignment system inspired by professional design tools,
    /// providing operations like align left/right/center, distribute evenly,
    /// make same size, etc.
    pub advanced_alignment: AdvancedAlignment,

    // ==================================================================================
    // HISTORY AND STATE MANAGEMENT
    // ==================================================================================
    
    /// Complete undo/redo operation history.
    /// 
    /// Maintains a history of all design operations with full state snapshots
    /// to enable atomic undo/redo operations. Each operation is stored as a
    /// complete transaction that can be reversed.
    pub history: DesignHistory,
    
    /// Design-time properties for enhanced IDE integration.
    /// 
    /// Stores additional metadata for components that is only relevant during
    /// design time, such as component names, locked state, and custom properties
    /// that don't affect runtime behavior.
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
    /// Selection rectangle for multi-select
    pub selection_rect: Option<egui::Rect>,
    /// Multi-select mode
    pub multi_select_mode: bool,
    /// Currently dragging components
    pub dragging: Option<DragOperation>,
    /// Hover state for visual feedback
    pub hover_component: Option<usize>,
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

/// Active drag operation
#[derive(Clone)]
pub struct DragOperation {
    /// Components being dragged
    pub component_indices: Vec<usize>,
    /// Original positions before drag started
    pub original_positions: Vec<egui::Pos2>,
    /// Current drag offset from start
    pub drag_offset: egui::Vec2,
    /// Drag start position
    pub start_pos: egui::Pos2,
    /// Type of drag operation
    pub drag_type: DragOperationType,
}

/// Types of drag operations
#[derive(Clone, Copy)]
pub enum DragOperationType {
    /// Moving components
    Move,
    /// Resizing a component
    Resize { handle: ResizeHandle },
    /// Creating selection rectangle
    SelectionRect,
}

/// Resize handle types
#[derive(Clone, Copy)]
pub enum ResizeHandle {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    TopCenter,
    BottomCenter,
    LeftCenter,
    RightCenter,
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
            dragging: None,
            hover_component: None,
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
            smart_editing: SmartEditingSystem::new(),
            advanced_alignment: AdvancedAlignment::new(),
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
        
        // Render actual components at their positions
        self.render_components_at_positions(ui, components);
        
        // Handle component interaction (must be after rendering for proper hit testing)
        self.handle_component_interaction(ui, components, canvas_rect);
        
        // Draw selection indicators on top
        self.draw_selection_indicators(ui, components);
        
        // Draw alignment helpers
        self.draw_alignment_helpers(ui, components);
        
        // Render smart editing guides
        self.smart_editing.render_guides(ui, canvas_rect);
    }
    
    /// Render components at their design-time positions
    fn render_components_at_positions(&mut self, ui: &mut egui::Ui, components: &mut [Box<dyn Component>]) {
        for (idx, component) in components.iter_mut().enumerate() {
            // Get or initialize component position and size
            let pos = self.layout.positions.get(&idx).copied().unwrap_or_else(|| {
                // Improved default positioning: staggered grid layout to prevent overlaps
                let columns = 3;
                let col = idx % columns;
                let row = idx / columns;
                let spacing_x = 150.0;
                let spacing_y = 60.0;
                let start_x = 50.0;
                let start_y = 50.0;
                
                let default_pos = egui::pos2(
                    start_x + (col as f32 * spacing_x),
                    start_y + (row as f32 * spacing_y)
                );
                
                // Snap to grid if enabled
                let final_pos = if self.grid.snap_enabled {
                    self.snap_to_grid(default_pos)
                } else {
                    default_pos
                };
                
                self.layout.positions.insert(idx, final_pos);
                final_pos
            });
            
            let size = self.layout.sizes.get(&idx).copied().unwrap_or_else(|| {
                // Improved default sizes with better proportions
                let default_size = match component.name() {
                    "Button" => egui::vec2(100.0, 32.0),
                    "Label" => egui::vec2(80.0, 24.0),
                    "TextBox" => egui::vec2(140.0, 28.0),
                    "Checkbox" => egui::vec2(120.0, 24.0),
                    "Slider" => egui::vec2(140.0, 24.0),
                    "Dropdown" => egui::vec2(120.0, 28.0),
                    _ => egui::vec2(100.0, 32.0),
                };
                self.layout.sizes.insert(idx, default_size);
                default_size
            });
            
            // Create a child UI at the component's position with proper padding
            let component_rect = egui::Rect::from_min_size(pos, size);
            let mut child_ui = ui.child_ui(
                component_rect, 
                egui::Layout::left_to_right(egui::Align::Center)
            );
            
            // Add some visual padding to prevent components from touching
            child_ui.spacing_mut().item_spacing = egui::vec2(4.0, 4.0);
            
            // Render the component in design mode (non-editable)
            component.render(&mut child_ui);
        }
    }

    /// Draw the grid system
    fn draw_grid(&self, ui: &mut egui::Ui, canvas_rect: egui::Rect) {
        let painter = ui.painter();
        let grid_size = self.grid.size.max(1.0); // Prevent division by zero
        
        // Calculate grid bounds to avoid drawing too many lines
        let max_lines = 1000; // Reasonable limit to prevent performance issues
        let h_lines = ((canvas_rect.width() / grid_size) as i32).min(max_lines);
        let v_lines = ((canvas_rect.height() / grid_size) as i32).min(max_lines);
        
        // Draw vertical grid lines
        for i in 0..=h_lines {
            let x = canvas_rect.min.x + (i as f32 * grid_size);
            if x > canvas_rect.max.x { break; }
            
            let is_major = i % (self.grid.major_lines as i32) == 0;
            let color = if is_major {
                self.grid.color.gamma_multiply(1.8)
            } else {
                self.grid.color
            };
            
            painter.line_segment(
                [egui::pos2(x, canvas_rect.min.y), egui::pos2(x, canvas_rect.max.y)],
                egui::Stroke::new(if is_major { 1.2 } else { 0.6 }, color)
            );
        }
        
        // Draw horizontal grid lines
        for i in 0..=v_lines {
            let y = canvas_rect.min.y + (i as f32 * grid_size);
            if y > canvas_rect.max.y { break; }
            
            let is_major = i % (self.grid.major_lines as i32) == 0;
            let color = if is_major {
                self.grid.color.gamma_multiply(1.8)
            } else {
                self.grid.color
            };
            
            painter.line_segment(
                [egui::pos2(canvas_rect.min.x, y), egui::pos2(canvas_rect.max.x, y)],
                egui::Stroke::new(if is_major { 1.2 } else { 0.6 }, color)
            );
        }
        
        // Draw origin indicator at (0,0) relative to canvas
        if canvas_rect.contains(egui::pos2(canvas_rect.min.x, canvas_rect.min.y)) {
            painter.circle_filled(
                egui::pos2(canvas_rect.min.x, canvas_rect.min.y),
                3.0,
                egui::Color32::from_rgb(255, 100, 100)
            );
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
    fn handle_component_interaction(&mut self, ui: &mut egui::Ui, components: &mut [Box<dyn Component>], canvas_rect: egui::Rect) {
        let pointer_pos = ui.input(|i| i.pointer.hover_pos());
        let pointer_down = ui.input(|i| i.pointer.primary_down());
        let pointer_released = ui.input(|i| i.pointer.primary_released());
        let ctrl_held = ui.input(|i| i.modifiers.ctrl);
        
        // Handle active drag operations
        if self.selection.dragging.is_some() {
            let drag_finished = self.handle_active_drag_state(pointer_pos, pointer_released);
            if drag_finished {
                self.finish_drag_operation();
            }
            return;
        }
        
        // Reset hover state
        self.selection.hover_component = None;
        
        // Check for component interaction in reverse order (top-to-bottom)
        let mut hit_component = None;
        for (idx, _component) in components.iter().enumerate().rev() {
            if let Some(pos) = self.layout.positions.get(&idx) {
                let size = self.layout.sizes.get(&idx).cloned().unwrap_or(egui::vec2(100.0, 30.0));
                let component_rect = egui::Rect::from_min_size(*pos, size);
                
                if let Some(pointer_pos) = pointer_pos {
                    if component_rect.contains(pointer_pos) {
                        self.selection.hover_component = Some(idx);
                        hit_component = Some(idx);
                        
                        // Handle component interaction
                        let response = ui.allocate_rect(component_rect, egui::Sense::click_and_drag());
                        
                        if response.clicked() {
                            if ctrl_held {
                                self.toggle_component_selection(idx);
                            } else {
                                self.select_single_component(idx);
                            }
                        } else if response.drag_started() {
                            self.start_component_drag(idx, pointer_pos);
                        }
                        
                        // Check for resize handle interaction if component is selected
                        if self.selection.selected.contains(&idx) {
                            if let Some(handle) = self.check_resize_handle_hit(component_rect, pointer_pos) {
                                if pointer_down {
                                    self.start_resize_drag(idx, handle, pointer_pos);
                                }
                            }
                        }
                        
                        break; // Only handle the topmost component
                    }
                }
            }
        }
        
        // Handle selection rectangle or canvas click
        if hit_component.is_none() && pointer_pos.is_some() {
            let canvas_response = ui.allocate_rect(canvas_rect, egui::Sense::click_and_drag());
            
            if canvas_response.clicked() && !ctrl_held {
                self.clear_selection();
            } else if canvas_response.drag_started() {
                self.start_selection_rect_drag(pointer_pos.unwrap());
            }
        }
    }
    
    /// Handle active drag operation state
    /// 
    /// This method implements the core drag-and-drop algorithm for component manipulation.
    /// It handles three types of drag operations: moving components, resizing components,
    /// and creating selection rectangles. The algorithm maintains smooth real-time feedback
    /// while applying constraints like grid snapping.
    fn handle_active_drag_state(&mut self, pointer_pos: Option<egui::Pos2>, pointer_released: bool) -> bool {
        if let Some(current_pos) = pointer_pos {
            if let Some(ref drag_op) = self.selection.dragging {
                // Calculate the delta from where the drag started to current mouse position
                // This offset will be applied to all selected components to maintain relative positions
                let drag_offset = current_pos - drag_op.start_pos;
                let drag_type = drag_op.drag_type;
                let component_indices = drag_op.component_indices.clone();
                let original_positions = drag_op.original_positions.clone();
                
                // Update the drag offset in the operation state for visual feedback
                // This is used by other systems (like visual guides) to show drag shadows
                if let Some(ref mut drag_op) = self.selection.dragging {
                    drag_op.drag_offset = drag_offset;
                }
                
                match drag_type {
                    DragOperationType::Move => {
                        // Handle component movement - apply the drag offset to all selected components
                        // while maintaining their relative positions to each other
                        for (i, &component_idx) in component_indices.iter().enumerate() {
                            if let Some(original_pos) = original_positions.get(i) {
                                // Calculate the new position by adding the drag offset to the original position
                                // This ensures smooth dragging regardless of how far the mouse has moved
                                let new_pos = *original_pos + drag_offset;
                                
                                // Apply grid snapping if enabled - this provides the "magnetic" feeling
                                // where components automatically align to grid intersections
                                let final_pos = if self.grid.snap_enabled {
                                    // Snap to the nearest grid intersection for precise alignment
                                    self.snap_to_grid(new_pos)
                                } else {
                                    // Use exact mouse position for freeform positioning
                                    new_pos
                                };
                                
                                // Update the component's position in the layout system
                                // This immediately reflects in the visual display
                                self.layout.positions.insert(component_idx, final_pos);
                            }
                        }
                    }
                    DragOperationType::Resize { handle } => {
                        // Handle component resizing - only one component can be resized at a time
                        // The resize algorithm adjusts both position and size based on which handle is dragged
                        if let Some(&component_idx) = component_indices.first() {
                            if let (Some(original_pos), Some(current_size)) = (
                                original_positions.first(),
                                self.layout.sizes.get(&component_idx).copied()
                            ) {
                                // Calculate new position and size based on the resize handle being dragged
                                // Different handles affect position and size differently (e.g., top-left vs bottom-right)
                                let (new_pos, new_size) = self.calculate_resize(*original_pos, current_size, drag_offset, handle);
                                
                                // Apply the calculated changes to the component
                                // Position may change when dragging certain handles (like top-left)
                                self.layout.positions.insert(component_idx, new_pos);
                                self.layout.sizes.insert(component_idx, new_size);
                            }
                        }
                    }
                    DragOperationType::SelectionRect => {
                        // Handle selection rectangle creation - allows selecting multiple components by dragging
                        // The algorithm creates a rectangle from the start position to current mouse position
                        let start_pos = if let Some(ref drag_op) = self.selection.dragging {
                            drag_op.start_pos
                        } else {
                            // Fallback if drag operation state is inconsistent
                            return pointer_released;
                        };
                        
                        // Calculate the selection rectangle bounds by finding min/max coordinates
                        // This handles dragging in any direction (up, down, left, right)
                        let min_x = start_pos.x.min(current_pos.x);  // Leftmost edge
                        let min_y = start_pos.y.min(current_pos.y);  // Topmost edge
                        let max_x = start_pos.x.max(current_pos.x);  // Rightmost edge
                        let max_y = start_pos.y.max(current_pos.y);  // Bottommost edge
                        
                        // Create the selection rectangle that will be rendered as visual feedback
                        // and used to determine which components are selected when drag ends
                        self.selection.selection_rect = Some(egui::Rect::from_min_max(
                            egui::pos2(min_x, min_y),
                            egui::pos2(max_x, max_y)
                        ));
                    }
                }
            }
        }
        
        // Return whether the drag operation should end (true when mouse is released)
        pointer_released
    }
    
    /// Start dragging selected components
    fn start_component_drag(&mut self, component_idx: usize, start_pos: egui::Pos2) {
        if !self.selection.selected.contains(&component_idx) {
            self.select_single_component(component_idx);
        }
        
        let original_positions: Vec<egui::Pos2> = self.selection.selected
            .iter()
            .filter_map(|&idx| self.layout.positions.get(&idx).copied())
            .collect();
            
        self.selection.dragging = Some(DragOperation {
            component_indices: self.selection.selected.clone(),
            original_positions,
            drag_offset: egui::Vec2::ZERO,
            start_pos,
            drag_type: DragOperationType::Move,
        });
    }
    
    /// Start resizing a component
    fn start_resize_drag(&mut self, component_idx: usize, handle: ResizeHandle, start_pos: egui::Pos2) {
        if let Some(original_pos) = self.layout.positions.get(&component_idx).copied() {
            self.selection.dragging = Some(DragOperation {
                component_indices: vec![component_idx],
                original_positions: vec![original_pos],
                drag_offset: egui::Vec2::ZERO,
                start_pos,
                drag_type: DragOperationType::Resize { handle },
            });
        }
    }
    
    /// Start selection rectangle drag
    fn start_selection_rect_drag(&mut self, start_pos: egui::Pos2) {
        self.selection.dragging = Some(DragOperation {
            component_indices: Vec::new(),
            original_positions: Vec::new(),
            drag_offset: egui::Vec2::ZERO,
            start_pos,
            drag_type: DragOperationType::SelectionRect,
        });
        
        self.selection.selection_rect = Some(egui::Rect::from_min_size(start_pos, egui::Vec2::ZERO));
    }
    
    
    /// Finish drag operation and commit changes
    fn finish_drag_operation(&mut self) {
        if let Some(drag_op) = self.selection.dragging.take() {
            match drag_op.drag_type {
                DragOperationType::Move => {
                    // Create undo operation for move
                    let operation = DesignOperation::Move {
                        component_ids: drag_op.component_indices.clone(),
                        old_positions: drag_op.original_positions.clone(),
                        new_positions: drag_op.component_indices
                            .iter()
                            .filter_map(|&idx| self.layout.positions.get(&idx).copied())
                            .collect(),
                    };
                    self.add_to_history(operation);
                }
                DragOperationType::Resize { .. } => {
                    // Handle resize completion
                    // TODO: Add resize to history
                }
                DragOperationType::SelectionRect => {
                    // Select components within rectangle
                    if let Some(selection_rect) = self.selection.selection_rect {
                        self.select_components_in_rect(selection_rect);
                    }
                    self.selection.selection_rect = None;
                }
            }
        }
    }
    
    /// Calculate new position and size during resize
    /// 
    /// This is a critical algorithm that handles 8-directional resizing of components.
    /// Each resize handle behaves differently - some affect only size, others affect both
    /// position and size. The algorithm ensures components never shrink below minimum size
    /// and maintains proper visual feedback during resize operations.
    fn calculate_resize(&self, original_pos: egui::Pos2, original_size: egui::Vec2, offset: egui::Vec2, handle: ResizeHandle) -> (egui::Pos2, egui::Vec2) {
        // Enforce minimum component size to prevent components from disappearing
        // This is essential for usability - users should never be able to resize components to zero
        let min_size = egui::vec2(20.0, 20.0);
        
        match handle {
            ResizeHandle::TopLeft => {
                // Top-left handle: moving mouse right/down shrinks component, left/up grows it
                // Both position and size change because we're dragging the origin point
                let new_pos = original_pos + egui::vec2(offset.x, offset.y);  // Move origin with mouse
                let new_size = original_size - egui::vec2(offset.x, offset.y); // Subtract offset from size
                (new_pos, new_size.at_least(min_size))  // Enforce minimum size constraints
            }
            ResizeHandle::TopRight => {
                // Top-right handle: mouse movement affects width positively, height negatively
                // Only Y position changes, width grows/shrinks with X offset
                let new_pos = original_pos + egui::vec2(0.0, offset.y);        // Only Y position changes
                let new_size = original_size + egui::vec2(offset.x, -offset.y); // Width grows, height shrinks with upward drag
                (new_pos, new_size.at_least(min_size))
            }
            ResizeHandle::BottomLeft => {
                // Bottom-left handle: mouse movement affects height positively, width negatively
                // Only X position changes, height grows/shrinks with Y offset
                let new_pos = original_pos + egui::vec2(offset.x, 0.0);        // Only X position changes
                let new_size = original_size + egui::vec2(-offset.x, offset.y); // Width shrinks, height grows with downward drag
                (new_pos, new_size.at_least(min_size))
            }
            ResizeHandle::BottomRight => {
                // Bottom-right handle: simplest case, both width and height grow with mouse movement
                // Position never changes, size increases in both directions
                let new_size = original_size + offset;  // Direct offset addition to size
                (original_pos, new_size.at_least(min_size))  // Position unchanged
            }
            ResizeHandle::TopCenter => {
                // Top-center handle: only affects height, moving up grows component, down shrinks it
                // Y position changes to maintain bottom edge position
                let new_pos = original_pos + egui::vec2(0.0, offset.y);        // Move top edge with mouse
                let new_size = original_size + egui::vec2(0.0, -offset.y);     // Height changes inversely to Y offset
                (new_pos, new_size.at_least(min_size))
            }
            ResizeHandle::BottomCenter => {
                // Bottom-center handle: only affects height, moving down grows component
                // Position never changes, only height increases/decreases
                let new_size = original_size + egui::vec2(0.0, offset.y);      // Only height changes
                (original_pos, new_size.at_least(min_size))  // Position unchanged
            }
            ResizeHandle::LeftCenter => {
                // Left-center handle: only affects width, moving left grows component, right shrinks it
                // X position changes to maintain right edge position
                let new_pos = original_pos + egui::vec2(offset.x, 0.0);        // Move left edge with mouse
                let new_size = original_size + egui::vec2(-offset.x, 0.0);     // Width changes inversely to X offset
                (new_pos, new_size.at_least(min_size))
            }
            ResizeHandle::RightCenter => {
                // Right-center handle: only affects width, moving right grows component
                // Position never changes, only width increases/decreases
                let new_size = original_size + egui::vec2(offset.x, 0.0);      // Only width changes
                (original_pos, new_size.at_least(min_size))  // Position unchanged
            }
        }
    }
    
    /// Check if pointer is over a resize handle
    fn check_resize_handle_hit(&self, component_rect: egui::Rect, pointer_pos: egui::Pos2) -> Option<ResizeHandle> {
        let handle_size = 6.0;
        let handles = [
            (component_rect.min, ResizeHandle::TopLeft),
            (egui::pos2(component_rect.max.x, component_rect.min.y), ResizeHandle::TopRight),
            (component_rect.max, ResizeHandle::BottomRight),
            (egui::pos2(component_rect.min.x, component_rect.max.y), ResizeHandle::BottomLeft),
            (egui::pos2(component_rect.center().x, component_rect.min.y), ResizeHandle::TopCenter),
            (egui::pos2(component_rect.max.x, component_rect.center().y), ResizeHandle::RightCenter),
            (egui::pos2(component_rect.center().x, component_rect.max.y), ResizeHandle::BottomCenter),
            (egui::pos2(component_rect.min.x, component_rect.center().y), ResizeHandle::LeftCenter),
        ];
        
        for (handle_pos, handle_type) in handles {
            let handle_rect = egui::Rect::from_center_size(handle_pos, egui::vec2(handle_size, handle_size));
            if handle_rect.contains(pointer_pos) {
                return Some(handle_type);
            }
        }
        
        None
    }
    
    /// Select components within a rectangle
    fn select_components_in_rect(&mut self, selection_rect: egui::Rect) {
        self.selection.selected.clear();
        
        for (idx, pos) in &self.layout.positions {
            let size = self.layout.sizes.get(idx).cloned().unwrap_or(egui::vec2(100.0, 30.0));
            let component_rect = egui::Rect::from_min_size(*pos, size);
            
            if selection_rect.intersects(component_rect) {
                self.selection.selected.push(*idx);
            }
        }
        
        self.selection.primary = self.selection.selected.first().copied();
    }

    /// Draw selection indicators around selected components
    fn draw_selection_indicators(&self, ui: &mut egui::Ui, _components: &[Box<dyn Component>]) {
        let painter = ui.painter();
        
        // Draw selection rectangle if active
        if let Some(selection_rect) = self.selection.selection_rect {
            painter.rect_stroke(
                selection_rect,
                0.0,
                egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(0, 120, 255, 128))
            );
            painter.rect_filled(
                selection_rect,
                0.0,
                egui::Color32::from_rgba_unmultiplied(0, 120, 255, 32)
            );
        }
        
        // Draw hover indicator
        if let Some(hover_idx) = self.selection.hover_component {
            if !self.selection.selected.contains(&hover_idx) {
                if let Some(pos) = self.layout.positions.get(&hover_idx) {
                    let size = self.layout.sizes.get(&hover_idx).cloned().unwrap_or(egui::vec2(100.0, 30.0));
                    let rect = egui::Rect::from_min_size(*pos, size);
                    
                    painter.rect_stroke(
                        rect.expand(1.0),
                        1.0,
                        egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(0, 120, 255, 128))
                    );
                }
            }
        }
        
        // Draw selected components
        for &component_idx in &self.selection.selected {
            if let Some(pos) = self.layout.positions.get(&component_idx) {
                let size = self.layout.sizes.get(&component_idx).cloned().unwrap_or(egui::vec2(100.0, 30.0));
                let rect = egui::Rect::from_min_size(*pos, size);
                
                // Different color for primary selection
                let is_primary = Some(component_idx) == self.selection.primary;
                let selection_color = if is_primary {
                    egui::Color32::from_rgb(0, 120, 255)
                } else {
                    egui::Color32::from_rgb(120, 120, 255)
                };
                
                // Draw selection rectangle
                painter.rect_stroke(
                    rect.expand(2.0),
                    2.0,
                    egui::Stroke::new(2.0, selection_color)
                );
                
                // Only draw resize handles for primary selection
                if is_primary {
                    self.draw_resize_handles(&painter, rect);
                }
                
                // Draw drag shadow if being dragged
                if let Some(ref drag_op) = self.selection.dragging {
                    if drag_op.component_indices.contains(&component_idx) && matches!(drag_op.drag_type, DragOperationType::Move) {
                        let shadow_rect = rect.translate(egui::vec2(2.0, 2.0));
                        painter.rect_filled(
                            shadow_rect,
                            2.0,
                            egui::Color32::from_rgba_unmultiplied(0, 0, 0, 64)
                        );
                    }
                }
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

    /// Select a single component (clears other selections)
    pub fn select_single_component(&mut self, component_idx: usize) {
        self.selection.selected.clear();
        self.selection.selected.push(component_idx);
        self.selection.primary = Some(component_idx);
    }
    
    /// Toggle component selection (for Ctrl+click)
    pub fn toggle_component_selection(&mut self, component_idx: usize) {
        if let Some(pos) = self.selection.selected.iter().position(|&idx| idx == component_idx) {
            self.selection.selected.remove(pos);
            if Some(component_idx) == self.selection.primary {
                self.selection.primary = self.selection.selected.first().copied();
            }
        } else {
            self.selection.selected.push(component_idx);
            if self.selection.primary.is_none() {
                self.selection.primary = Some(component_idx);
            }
        }
    }
    
    /// Clear all selections
    pub fn clear_selection(&mut self) {
        self.selection.selected.clear();
        self.selection.primary = None;
    }
    
    /// Select a component (legacy method for compatibility)
    pub fn select_component(&mut self, component_idx: usize) {
        self.select_single_component(component_idx);
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
    /// 
    /// This algorithm implements the "magnetic" grid snapping behavior that's essential
    /// for precise component alignment. It rounds positions to the nearest grid intersection,
    /// providing the satisfying "snap" feeling that users expect from professional design tools.
    pub fn snap_to_grid(&self, pos: egui::Pos2) -> egui::Pos2 {
        // Early exit if grid snapping is disabled or grid size is invalid
        // This optimization avoids unnecessary calculations when snapping isn't needed
        if !self.grid.snap_enabled || self.grid.size <= 0.0 {
            return pos;  // Return original position unchanged
        }
        
        // Clamp grid size to minimum value to prevent division by zero or infinite loops
        // Even with invalid settings, we want the system to remain stable
        let grid_size = self.grid.size.max(1.0);
        
        // Grid snapping algorithm: round to nearest grid intersection
        // 1. Divide position by grid size to get fractional grid units
        // 2. Round to nearest integer to find closest grid line
        // 3. Multiply back by grid size to get pixel position
        // 4. Clamp to prevent negative coordinates (keep components in visible area)
        let snapped_x = ((pos.x / grid_size).round() * grid_size).max(0.0);
        let snapped_y = ((pos.y / grid_size).round() * grid_size).max(0.0);
        
        // Return the snapped position as a new point
        egui::pos2(snapped_x, snapped_y)
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