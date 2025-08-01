//! # Drag and Drop System
//! 
//! Manages drag and drop operations within the IDE, including component placement
//! from the palette and repositioning of existing components.

use eframe::egui;

/// # Drag and Drop State Management
/// 
/// Manages the state of drag and drop operations within the IDE. This system
/// supports multiple types of drag operations including component placement
/// from the palette and repositioning of existing components.
/// 
/// ## State Lifecycle
/// 
/// 1. **Drag Start**: User begins dragging (from palette or existing component)
/// 2. **Drag Continue**: Track mouse movement and provide visual feedback
/// 3. **Drag End**: Complete the operation (place component or update position)
/// 4. **Drag Cancel**: User cancels the operation (ESC key or invalid drop)
/// 
/// ## Performance Considerations
/// 
/// The drag state is updated every frame during drag operations, so it's kept
/// minimal to reduce allocation overhead. Visual feedback is handled separately
/// by the live feedback system.
#[derive(Default)]
pub struct DragState {
    /// Index of the component being dragged, if any.
    /// 
    /// For existing components being repositioned, this refers to their index
    /// in the main `components` vector. None when dragging from palette.
    pub dragging_component: Option<usize>,
    
    /// Type of drag operation currently in progress.
    /// 
    /// Determines how the drag operation should be handled and what visual
    /// feedback should be provided to the user.
    pub drag_type: DragType,
    
    /// Screen position where the drag operation started.
    /// 
    /// Used for calculating drag deltas and determining drag thresholds.
    /// Relative to the IDE's coordinate system (usually canvas coordinates).
    pub drag_start_pos: egui::Pos2,
    
    /// Current mouse position during drag operation.
    /// 
    /// Updated continuously during the drag operation to provide real-time
    /// feedback and determine drop locations.
    pub current_drag_pos: egui::Pos2,
    
    /// Flag indicating if a drag operation is currently active.
    /// 
    /// Used to determine whether to process drag-related input events
    /// and render drag-specific visual feedback.
    pub is_dragging: bool,
    
    /// Preview position for the component being dragged.
    /// 
    /// Shows where the component would be placed if dropped at the current
    /// cursor position. Accounts for snapping and alignment constraints.
    pub preview_position: Option<egui::Pos2>,
    
    /// Drop target validation result.
    /// 
    /// Indicates whether the current drop location is valid for the component
    /// being dragged. Used to provide visual feedback (e.g., green vs red highlight).
    pub drop_valid: bool,
}

/// # Drag Operation Types
/// 
/// Defines the different types of drag operations supported by the IDE.
/// Each type has different behavior for initiation, feedback, and completion.
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum DragType {
    /// No drag operation in progress.
    #[default]
    None,
    
    /// Dragging a new component from the component palette.
    /// 
    /// Creates a new component instance when dropped on a valid target.
    /// The component type is determined by the palette selection.
    ComponentFromPalette(ComponentType),
    
    /// Moving an existing component within the designer.
    /// 
    /// Updates the position of an existing component. Supports both
    /// absolute positioning and constraint-based positioning.
    ComponentMove,
    
    /// Resizing a component by dragging its handles.
    /// 
    /// Modifies the size of a component while maintaining aspect ratio
    /// constraints and minimum/maximum size limits.
    ComponentResize {
        /// Which resize handle is being dragged
        handle: ResizeHandle,
        /// Original size when resize started
        original_size: egui::Vec2,
    },
    
    /// Multi-selection drag operation.
    /// 
    /// Moves multiple selected components as a group while maintaining
    /// their relative positions and alignment relationships.
    MultiComponentMove {
        /// Number of components in the selection
        selection_count: usize,
    },
}

/// # Component Types for Palette Dragging
/// 
/// Defines the types of components that can be dragged from the palette.
/// Each type corresponds to a specific component implementation.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ComponentType {
    Button,
    Label,
    TextBox,
    Checkbox,
    Slider,
    Dropdown,
    Panel,
    Image,
    Chart,
    Table,
    Tree,
    Custom(u32), // Custom component ID
}

/// # Resize Handle Types
/// 
/// Defines the different resize handles available on selected components.
/// Each handle allows resizing in specific directions with appropriate constraints.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ResizeHandle {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Top,
    Bottom,
    Left,
    Right,
}

impl DragState {
    /// Create a new drag state with default values
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Start a new drag operation
    pub fn start_drag(&mut self, drag_type: DragType, start_pos: egui::Pos2) {
        self.drag_type = drag_type;
        self.drag_start_pos = start_pos;
        self.current_drag_pos = start_pos;
        self.is_dragging = true;
        self.preview_position = None;
        self.drop_valid = false;
    }
    
    /// Update the current drag position
    pub fn update_drag_position(&mut self, pos: egui::Pos2) {
        if self.is_dragging {
            self.current_drag_pos = pos;
            // Update preview position based on drag type and constraints
            self.update_preview_position();
        }
    }
    
    /// End the current drag operation
    pub fn end_drag(&mut self) -> Option<DragCompletionResult> {
        if !self.is_dragging {
            return None;
        }
        
        let result = if self.drop_valid {
            Some(DragCompletionResult {
                drag_type: self.drag_type,
                start_position: self.drag_start_pos,
                end_position: self.current_drag_pos,
                preview_position: self.preview_position,
                component_index: self.dragging_component,
            })
        } else {
            None
        };
        
        self.reset();
        result
    }
    
    /// Cancel the current drag operation
    pub fn cancel_drag(&mut self) {
        self.reset();
    }
    
    /// Reset the drag state to default values
    fn reset(&mut self) {
        self.dragging_component = None;
        self.drag_type = DragType::None;
        self.is_dragging = false;
        self.preview_position = None;
        self.drop_valid = false;
    }
    
    /// Update the preview position based on current constraints
    fn update_preview_position(&mut self) {
        match self.drag_type {
            DragType::ComponentFromPalette(_) | DragType::ComponentMove => {
                // For component placement/movement, preview position is the snap-adjusted position
                self.preview_position = Some(self.current_drag_pos);
                self.drop_valid = true; // TODO: Add actual validation logic
            }
            DragType::ComponentResize { .. } => {
                // For resize operations, preview shows the new size
                self.preview_position = Some(self.current_drag_pos);
                self.drop_valid = true; // TODO: Add size constraint validation
            }
            DragType::MultiComponentMove { .. } => {
                // For multi-selection, preview shows the group movement
                self.preview_position = Some(self.current_drag_pos);
                self.drop_valid = true; // TODO: Add group movement validation
            }
            DragType::None => {
                self.preview_position = None;
                self.drop_valid = false;
            }
        }
    }
    
    /// Get the drag delta from start to current position
    pub fn drag_delta(&self) -> egui::Vec2 {
        self.current_drag_pos - self.drag_start_pos
    }
    
    /// Check if the drag distance exceeds the threshold for starting a drag
    pub fn exceeds_drag_threshold(&self, threshold: f32) -> bool {
        self.drag_delta().length() > threshold
    }
    
    /// Get a description of the current drag operation for UI feedback
    pub fn drag_description(&self) -> String {
        match self.drag_type {
            DragType::None => "No drag operation".to_string(),
            DragType::ComponentFromPalette(component_type) => {
                format!("Adding {:?} component", component_type)
            }
            DragType::ComponentMove => "Moving component".to_string(),
            DragType::ComponentResize { handle, .. } => {
                format!("Resizing component from {:?} handle", handle)
            }
            DragType::MultiComponentMove { selection_count } => {
                format!("Moving {} components", selection_count)
            }
        }
    }
}

/// # Drag Completion Result
/// 
/// Contains the results of a completed drag operation, including all the
/// information needed to apply the operation to the component system.
pub struct DragCompletionResult {
    pub drag_type: DragType,
    pub start_position: egui::Pos2,
    pub end_position: egui::Pos2,
    pub preview_position: Option<egui::Pos2>,
    pub component_index: Option<usize>,
}

impl ComponentType {
    /// Get the display name for this component type
    pub fn display_name(&self) -> &'static str {
        match self {
            ComponentType::Button => "Button",
            ComponentType::Label => "Label",
            ComponentType::TextBox => "Text Box",
            ComponentType::Checkbox => "Checkbox",
            ComponentType::Slider => "Slider",
            ComponentType::Dropdown => "Dropdown",
            ComponentType::Panel => "Panel",
            ComponentType::Image => "Image",
            ComponentType::Chart => "Chart",
            ComponentType::Table => "Table",
            ComponentType::Tree => "Tree",
            ComponentType::Custom(_) => "Custom",
        }
    }
    
    /// Get the icon for this component type
    pub fn icon(&self) -> &'static str {
        match self {
            ComponentType::Button => "🔘",
            ComponentType::Label => "🏷️",
            ComponentType::TextBox => "📝",
            ComponentType::Checkbox => "☑️",
            ComponentType::Slider => "🎚️",
            ComponentType::Dropdown => "📋",
            ComponentType::Panel => "🖼️",
            ComponentType::Image => "🖼️",
            ComponentType::Chart => "📊",
            ComponentType::Table => "🗂️",
            ComponentType::Tree => "🌳",
            ComponentType::Custom(_) => "🔧",
        }
    }
}

impl ResizeHandle {
    /// Get the cursor style for this resize handle
    pub fn cursor_icon(&self) -> egui::CursorIcon {
        match self {
            ResizeHandle::TopLeft | ResizeHandle::BottomRight => egui::CursorIcon::ResizeNwSe,
            ResizeHandle::TopRight | ResizeHandle::BottomLeft => egui::CursorIcon::ResizeNeSw,
            ResizeHandle::Top | ResizeHandle::Bottom => egui::CursorIcon::ResizeVertical,
            ResizeHandle::Left | ResizeHandle::Right => egui::CursorIcon::ResizeHorizontal,
        }
    }
    
    /// Check if this handle allows horizontal resizing
    pub fn allows_horizontal_resize(&self) -> bool {
        matches!(self, 
            ResizeHandle::Left | ResizeHandle::Right | 
            ResizeHandle::TopLeft | ResizeHandle::TopRight |
            ResizeHandle::BottomLeft | ResizeHandle::BottomRight
        )
    }
    
    /// Check if this handle allows vertical resizing
    pub fn allows_vertical_resize(&self) -> bool {
        matches!(self, 
            ResizeHandle::Top | ResizeHandle::Bottom | 
            ResizeHandle::TopLeft | ResizeHandle::TopRight |
            ResizeHandle::BottomLeft | ResizeHandle::BottomRight
        )
    }
}