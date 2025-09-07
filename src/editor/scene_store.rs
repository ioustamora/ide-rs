//! SceneStore with canonical scene graph and command-based undo/redo
//!
//! Provides stable UUID IDs, diffing, and command pattern undo/redo for the visual designer
//! as specified in the improvement plan Phase P0.

use std::collections::{HashMap, VecDeque};
use std::time::Instant;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::rcl::component_registry::{PropertyValue, ComponentMetadata};

/// Central scene store managing component hierarchy and state
pub struct SceneStore {
    /// Root component of the scene
    pub root_component: Option<ComponentId>,
    /// All components in the scene by ID
    pub components: HashMap<ComponentId, SceneComponent>,
    /// Component hierarchy (parent -> children mapping)
    pub hierarchy: HashMap<ComponentId, Vec<ComponentId>>,
    /// Reverse hierarchy (child -> parent mapping)  
    pub parent_map: HashMap<ComponentId, ComponentId>,
    /// Command history for undo/redo
    pub command_history: CommandHistory,
    /// Selection state
    pub selection: SelectionState,
    /// Scene metadata
    pub metadata: SceneMetadata,
    /// Change tracking for diffing
    pub change_tracker: SceneChangeTracker,
}

/// Unique component identifier using UUIDs for stability
pub type ComponentId = Uuid;

/// Component in the scene with stable ID and properties
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SceneComponent {
    /// Unique component ID
    pub id: ComponentId,
    /// Component type name
    pub component_type: String,
    /// Component display name
    pub name: String,
    /// Component properties
    pub properties: HashMap<String, PropertyValue>,
    /// Layout metadata
    pub layout_meta: LayoutMetadata,
    /// Component visibility
    pub visible: bool,
    /// Component locked state
    pub locked: bool,
    /// Creation timestamp
    pub created_at: Instant,
    /// Last modified timestamp
    pub modified_at: Instant,
}

/// Layout metadata for positioning and sizing
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LayoutMetadata {
    /// Position in parent coordinate system
    pub position: Position,
    /// Component size
    pub size: Size,
    /// Z-index for layering
    pub z_index: i32,
    /// Layout constraints
    pub constraints: LayoutConstraints,
    /// Transform matrix
    pub transform: Transform,
    /// Layout type
    pub layout_type: LayoutType,
}

/// 2D position
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

/// 2D size  
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

/// Layout constraints for responsive design
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LayoutConstraints {
    /// Minimum size
    pub min_size: Option<Size>,
    /// Maximum size
    pub max_size: Option<Size>,
    /// Aspect ratio constraint
    pub aspect_ratio: Option<f32>,
    /// Anchor points for relative positioning
    pub anchors: Anchors,
}

/// Anchor points for relative positioning
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Anchors {
    pub left: Option<f32>,
    pub top: Option<f32>,
    pub right: Option<f32>,
    pub bottom: Option<f32>,
}

/// Transform matrix for rotation, scale, skew
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transform {
    /// Translation
    pub translation: Position,
    /// Rotation in radians
    pub rotation: f32,
    /// Scale factors
    pub scale: Position,
    /// Skew factors
    pub skew: Position,
}

/// Layout types
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum LayoutType {
    /// Absolute positioning
    Absolute,
    /// Flexbox layout
    Flex(FlexLayout),
    /// Grid layout
    Grid(GridLayout),
    /// Relative positioning
    Relative,
    /// Fixed positioning
    Fixed,
}

/// Flexbox layout configuration
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FlexLayout {
    /// Flex direction
    pub direction: FlexDirection,
    /// Justify content
    pub justify_content: JustifyContent,
    /// Align items
    pub align_items: AlignItems,
    /// Flex wrap
    pub wrap: FlexWrap,
    /// Gap between items
    pub gap: f32,
}

/// Grid layout configuration
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GridLayout {
    /// Column definitions
    pub columns: Vec<GridTrack>,
    /// Row definitions
    pub rows: Vec<GridTrack>,
    /// Grid gaps
    pub gap: GridGap,
    /// Grid item placement
    pub auto_flow: GridAutoFlow,
}

/// Grid track definition
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum GridTrack {
    /// Fixed size in pixels
    Px(f32),
    /// Fractional unit
    Fr(f32),
    /// Auto-sizing
    Auto,
    /// Min-content
    MinContent,
    /// Max-content
    MaxContent,
    /// Minmax constraint
    Minmax(Box<GridTrack>, Box<GridTrack>),
}

/// Grid gap configuration
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GridGap {
    pub row: f32,
    pub column: f32,
}

/// Grid auto flow direction
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum GridAutoFlow {
    Row,
    Column,
    RowDense,
    ColumnDense,
}

/// Flex direction
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

/// Justify content alignment
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

/// Align items alignment
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    Baseline,
}

/// Flex wrap behavior
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

/// Command history for undo/redo operations
pub struct CommandHistory {
    /// Undo stack
    pub undo_stack: VecDeque<Box<dyn Command>>,
    /// Redo stack
    pub redo_stack: VecDeque<Box<dyn Command>>,
    /// Maximum history size
    pub max_history_size: usize,
    /// Current command group (for macro commands)
    pub current_group: Option<CommandGroup>,
}

/// Command trait for undo/redo operations
pub trait Command: Send + Sync {
    /// Execute the command
    fn execute(&self, store: &mut SceneStore) -> Result<(), CommandError>;
    /// Undo the command
    fn undo(&self, store: &mut SceneStore) -> Result<(), CommandError>;
    /// Get command description for UI
    fn description(&self) -> String;
    /// Check if command can be merged with another
    fn can_merge(&self, other: &dyn Command) -> bool;
    /// Merge with another command
    fn merge(&self, other: Box<dyn Command>) -> Box<dyn Command>;
}

/// Command group for atomic operations
pub struct CommandGroup {
    /// Commands in the group
    pub commands: Vec<Box<dyn Command>>,
    /// Group description
    pub description: String,
    /// Group start time
    pub start_time: Instant,
}

/// Selection state management
#[derive(Clone, Debug, Default)]
pub struct SelectionState {
    /// Currently selected components
    pub selected: Vec<ComponentId>,
    /// Primary selection (for multi-select)
    pub primary: Option<ComponentId>,
    /// Selection bounds cache
    pub bounds: Option<SelectionBounds>,
}

/// Bounding box for selection
#[derive(Clone, Debug)]
pub struct SelectionBounds {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

/// Scene metadata
#[derive(Clone, Debug)]
pub struct SceneMetadata {
    /// Scene name
    pub name: String,
    /// Scene version
    pub version: u32,
    /// Created timestamp
    pub created_at: Instant,
    /// Modified timestamp
    pub modified_at: Instant,
    /// Scene author
    pub author: Option<String>,
    /// Scene description
    pub description: Option<String>,
}

/// Change tracking for scene diffing
pub struct SceneChangeTracker {
    /// Recent changes for diffing
    pub changes: VecDeque<SceneChange>,
    /// Last snapshot hash
    pub last_snapshot_hash: u64,
    /// Change counter for versions
    pub version: u64,
}

/// Individual scene change
#[derive(Clone, Debug)]
pub struct SceneChange {
    /// Change type
    pub change_type: ChangeType,
    /// Component ID affected
    pub component_id: ComponentId,
    /// Property name (for property changes)
    pub property: Option<String>,
    /// Old value
    pub old_value: Option<PropertyValue>,
    /// New value
    pub new_value: Option<PropertyValue>,
    /// Timestamp
    pub timestamp: Instant,
    /// Version number
    pub version: u64,
}

/// Types of scene changes
#[derive(Clone, Debug, PartialEq)]
pub enum ChangeType {
    ComponentAdded,
    ComponentRemoved,
    ComponentMoved,
    PropertyChanged,
    HierarchyChanged,
    SelectionChanged,
}

/// Scene commands
#[derive(Clone, Debug)]
pub struct AddComponentCommand {
    pub component: SceneComponent,
    pub parent_id: Option<ComponentId>,
    pub index: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct RemoveComponentCommand {
    pub component_id: ComponentId,
    pub component_data: Option<SceneComponent>,
    pub parent_id: Option<ComponentId>,
    pub index: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct UpdatePropertyCommand {
    pub component_id: ComponentId,
    pub property: String,
    pub old_value: PropertyValue,
    pub new_value: PropertyValue,
}

#[derive(Clone, Debug)]
pub struct MoveComponentCommand {
    pub component_id: ComponentId,
    pub old_parent: Option<ComponentId>,
    pub new_parent: Option<ComponentId>,
    pub old_index: usize,
    pub new_index: usize,
}

/// Command errors
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("Component not found: {0}")]
    ComponentNotFound(ComponentId),
    #[error("Invalid parent: {0}")]
    InvalidParent(ComponentId),
    #[error("Property not found: {0}")]
    PropertyNotFound(String),
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

impl SceneStore {
    /// Create new empty scene store
    pub fn new() -> Self {
        Self {
            root_component: None,
            components: HashMap::new(),
            hierarchy: HashMap::new(),
            parent_map: HashMap::new(),
            command_history: CommandHistory::new(),
            selection: SelectionState::default(),
            metadata: SceneMetadata {
                name: "Untitled Scene".to_string(),
                version: 1,
                created_at: Instant::now(),
                modified_at: Instant::now(),
                author: None,
                description: None,
            },
            change_tracker: SceneChangeTracker::new(),
        }
    }

    /// Execute a command with undo/redo support
    pub fn execute_command(&mut self, command: Box<dyn Command>) -> Result<(), CommandError> {
        // Execute the command
        command.execute(self)?;
        
        // Clear redo stack when new command is executed
        self.command_history.redo_stack.clear();
        
        // Add to undo stack
        self.command_history.push_undo(command);
        
        // Update metadata
        self.metadata.modified_at = Instant::now();
        self.metadata.version += 1;
        
        Ok(())
    }

    /// Undo last command
    pub fn undo(&mut self) -> Result<(), CommandError> {
        if let Some(command) = self.command_history.pop_undo() {
            command.undo(self)?;
            self.command_history.push_redo(command);
            self.metadata.modified_at = Instant::now();
            Ok(())
        } else {
            Err(CommandError::InvalidOperation("Nothing to undo".to_string()))
        }
    }

    /// Redo last undone command
    pub fn redo(&mut self) -> Result<(), CommandError> {
        if let Some(command) = self.command_history.pop_redo() {
            command.execute(self)?;
            self.command_history.push_undo(command);
            self.metadata.modified_at = Instant::now();
            Ok(())
        } else {
            Err(CommandError::InvalidOperation("Nothing to redo".to_string()))
        }
    }

    /// Add component to scene
    pub fn add_component(&mut self, component_type: &str, parent_id: Option<ComponentId>) -> Result<ComponentId, CommandError> {
        let component = SceneComponent::new(component_type);
        let component_id = component.id;
        
        let command = AddComponentCommand {
            component,
            parent_id,
            index: None,
        };
        
        self.execute_command(Box::new(command))?;
        Ok(component_id)
    }

    /// Remove component from scene
    pub fn remove_component(&mut self, component_id: ComponentId) -> Result<(), CommandError> {
        if !self.components.contains_key(&component_id) {
            return Err(CommandError::ComponentNotFound(component_id));
        }
        
        let command = RemoveComponentCommand {
            component_id,
            component_data: None,
            parent_id: self.parent_map.get(&component_id).copied(),
            index: None,
        };
        
        self.execute_command(Box::new(command))
    }

    /// Update component property
    pub fn update_property(&mut self, component_id: ComponentId, property: &str, value: PropertyValue) -> Result<(), CommandError> {
        let component = self.components.get(&component_id)
            .ok_or(CommandError::ComponentNotFound(component_id))?;
        
        let old_value = component.properties.get(property)
            .ok_or_else(|| CommandError::PropertyNotFound(property.to_string()))?
            .clone();
        
        let command = UpdatePropertyCommand {
            component_id,
            property: property.to_string(),
            old_value,
            new_value: value,
        };
        
        self.execute_command(Box::new(command))
    }

    /// Move component in hierarchy
    pub fn move_component(&mut self, component_id: ComponentId, new_parent: Option<ComponentId>, index: usize) -> Result<(), CommandError> {
        let old_parent = self.parent_map.get(&component_id).copied();
        let old_index = self.get_component_index(component_id)?;
        
        let command = MoveComponentCommand {
            component_id,
            old_parent,
            new_parent,
            old_index,
            new_index: index,
        };
        
        self.execute_command(Box::new(command))
    }

    /// Get component by ID
    pub fn get_component(&self, component_id: ComponentId) -> Option<&SceneComponent> {
        self.components.get(&component_id)
    }

    /// Get component mutably
    pub fn get_component_mut(&mut self, component_id: ComponentId) -> Option<&mut SceneComponent> {
        self.components.get_mut(&component_id)
    }

    /// Get component children
    pub fn get_children(&self, component_id: ComponentId) -> Vec<ComponentId> {
        self.hierarchy.get(&component_id).cloned().unwrap_or_default()
    }

    /// Get component parent
    pub fn get_parent(&self, component_id: ComponentId) -> Option<ComponentId> {
        self.parent_map.get(&component_id).copied()
    }

    /// Set selection
    pub fn set_selection(&mut self, selection: Vec<ComponentId>) {
        let old_selection = self.selection.selected.clone();
        self.selection.selected = selection;
        self.selection.primary = self.selection.selected.first().copied();
        self.update_selection_bounds();
        
        // Record change
        self.change_tracker.record_change(SceneChange {
            change_type: ChangeType::SelectionChanged,
            component_id: self.selection.primary.unwrap_or_default(),
            property: None,
            old_value: None,
            new_value: None,
            timestamp: Instant::now(),
            version: self.change_tracker.next_version(),
        });
    }

    /// Get current selection
    pub fn get_selection(&self) -> &[ComponentId] {
        &self.selection.selected
    }

    /// Start command group for atomic operations
    pub fn begin_command_group(&mut self, description: String) {
        self.command_history.current_group = Some(CommandGroup {
            commands: Vec::new(),
            description,
            start_time: Instant::now(),
        });
    }

    /// End command group
    pub fn end_command_group(&mut self) -> Result<(), CommandError> {
        if let Some(group) = self.command_history.current_group.take() {
            if !group.commands.is_empty() {
                let group_command = GroupCommand { group };
                self.execute_command(Box::new(group_command))?;
            }
        }
        Ok(())
    }

    /// Create scene snapshot for diffing
    pub fn create_snapshot(&self) -> SceneSnapshot {
        SceneSnapshot {
            components: self.components.clone(),
            hierarchy: self.hierarchy.clone(),
            parent_map: self.parent_map.clone(),
            metadata: self.metadata.clone(),
            version: self.change_tracker.version,
        }
    }

    /// Compute diff between snapshots
    pub fn compute_diff(&self, old_snapshot: &SceneSnapshot) -> SceneDiff {
        let mut changes = Vec::new();
        
        // Find added/removed components
        for (id, component) in &self.components {
            if !old_snapshot.components.contains_key(id) {
                changes.push(SceneChange {
                    change_type: ChangeType::ComponentAdded,
                    component_id: *id,
                    property: None,
                    old_value: None,
                    new_value: None,
                    timestamp: Instant::now(),
                    version: self.change_tracker.version,
                });
            }
        }
        
        for (id, _) in &old_snapshot.components {
            if !self.components.contains_key(id) {
                changes.push(SceneChange {
                    change_type: ChangeType::ComponentRemoved,
                    component_id: *id,
                    property: None,
                    old_value: None,
                    new_value: None,
                    timestamp: Instant::now(),
                    version: self.change_tracker.version,
                });
            }
        }
        
        // Find property changes
        for (id, component) in &self.components {
            if let Some(old_component) = old_snapshot.components.get(id) {
                for (prop_name, prop_value) in &component.properties {
                    if let Some(old_value) = old_component.properties.get(prop_name) {
                        if old_value != prop_value {
                            changes.push(SceneChange {
                                change_type: ChangeType::PropertyChanged,
                                component_id: *id,
                                property: Some(prop_name.clone()),
                                old_value: Some(old_value.clone()),
                                new_value: Some(prop_value.clone()),
                                timestamp: Instant::now(),
                                version: self.change_tracker.version,
                            });
                        }
                    }
                }
            }
        }
        
        SceneDiff { changes }
    }

    // Private helper methods
    fn get_component_index(&self, component_id: ComponentId) -> Result<usize, CommandError> {
        let parent_id = self.parent_map.get(&component_id).copied();
        
        if let Some(parent) = parent_id {
            let children = self.hierarchy.get(&parent).ok_or(CommandError::InvalidParent(parent))?;
            children.iter().position(|&id| id == component_id)
                .ok_or(CommandError::ComponentNotFound(component_id))
        } else {
            Ok(0) // Root component
        }
    }

    fn update_selection_bounds(&mut self) {
        if self.selection.selected.is_empty() {
            self.selection.bounds = None;
            return;
        }
        
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;
        
        for &component_id in &self.selection.selected {
            if let Some(component) = self.components.get(&component_id) {
                let pos = &component.layout_meta.position;
                let size = &component.layout_meta.size;
                
                min_x = min_x.min(pos.x);
                min_y = min_y.min(pos.y);
                max_x = max_x.max(pos.x + size.width);
                max_y = max_y.max(pos.y + size.height);
            }
        }
        
        self.selection.bounds = Some(SelectionBounds {
            min_x, min_y, max_x, max_y,
        });
    }
}

impl SceneComponent {
    /// Create new scene component
    pub fn new(component_type: &str) -> Self {
        let now = Instant::now();
        Self {
            id: Uuid::new_v4(),
            component_type: component_type.to_string(),
            name: format!("{} {}", component_type, now.elapsed().as_millis()),
            properties: HashMap::new(),
            layout_meta: LayoutMetadata::default(),
            visible: true,
            locked: false,
            created_at: now,
            modified_at: now,
        }
    }
}

impl Default for LayoutMetadata {
    fn default() -> Self {
        Self {
            position: Position { x: 0.0, y: 0.0 },
            size: Size { width: 100.0, height: 100.0 },
            z_index: 0,
            constraints: LayoutConstraints::default(),
            transform: Transform::default(),
            layout_type: LayoutType::Absolute,
        }
    }
}

impl Default for LayoutConstraints {
    fn default() -> Self {
        Self {
            min_size: None,
            max_size: None,
            aspect_ratio: None,
            anchors: Anchors {
                left: None,
                top: None,
                right: None,
                bottom: None,
            },
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Position { x: 0.0, y: 0.0 },
            rotation: 0.0,
            scale: Position { x: 1.0, y: 1.0 },
            skew: Position { x: 0.0, y: 0.0 },
        }
    }
}

impl CommandHistory {
    pub fn new() -> Self {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
            max_history_size: 100,
            current_group: None,
        }
    }
    
    pub fn push_undo(&mut self, command: Box<dyn Command>) {
        if let Some(ref mut group) = self.current_group {
            group.commands.push(command);
        } else {
            self.undo_stack.push_back(command);
            
            // Limit stack size
            if self.undo_stack.len() > self.max_history_size {
                self.undo_stack.pop_front();
            }
        }
    }
    
    pub fn pop_undo(&mut self) -> Option<Box<dyn Command>> {
        self.undo_stack.pop_back()
    }
    
    pub fn push_redo(&mut self, command: Box<dyn Command>) {
        self.redo_stack.push_back(command);
        
        // Limit stack size
        if self.redo_stack.len() > self.max_history_size {
            self.redo_stack.pop_front();
        }
    }
    
    pub fn pop_redo(&mut self) -> Option<Box<dyn Command>> {
        self.redo_stack.pop_back()
    }
}

impl SceneChangeTracker {
    pub fn new() -> Self {
        Self {
            changes: VecDeque::new(),
            last_snapshot_hash: 0,
            version: 0,
        }
    }
    
    pub fn record_change(&mut self, change: SceneChange) {
        self.changes.push_back(change);
        
        // Limit change history
        if self.changes.len() > 1000 {
            self.changes.pop_front();
        }
    }
    
    pub fn next_version(&mut self) -> u64 {
        self.version += 1;
        self.version
    }
}

/// Scene snapshot for diffing
#[derive(Clone, Debug)]
pub struct SceneSnapshot {
    pub components: HashMap<ComponentId, SceneComponent>,
    pub hierarchy: HashMap<ComponentId, Vec<ComponentId>>,
    pub parent_map: HashMap<ComponentId, ComponentId>,
    pub metadata: SceneMetadata,
    pub version: u64,
}

/// Scene diff result
#[derive(Clone, Debug)]
pub struct SceneDiff {
    pub changes: Vec<SceneChange>,
}

/// Group command for atomic operations
pub struct GroupCommand {
    pub group: CommandGroup,
}

// Command implementations
impl Command for AddComponentCommand {
    fn execute(&self, store: &mut SceneStore) -> Result<(), CommandError> {
        let component_id = self.component.id;
        
        // Add component to store
        store.components.insert(component_id, self.component.clone());
        
        // Update hierarchy
        if let Some(parent_id) = self.parent_id {
            store.hierarchy.entry(parent_id).or_default().push(component_id);
            store.parent_map.insert(component_id, parent_id);
        } else {
            // Root component
            store.root_component = Some(component_id);
        }
        
        // Record change
        let version = store.change_tracker.next_version();
        store.change_tracker.record_change(SceneChange {
            change_type: ChangeType::ComponentAdded,
            component_id,
            property: None,
            old_value: None,
            new_value: None,
            timestamp: Instant::now(),
            version,
        });
        
        Ok(())
    }
    
    fn undo(&self, store: &mut SceneStore) -> Result<(), CommandError> {
        let component_id = self.component.id;
        
        // Remove from hierarchy
        if let Some(parent_id) = self.parent_id {
            if let Some(children) = store.hierarchy.get_mut(&parent_id) {
                children.retain(|&id| id != component_id);
            }
            store.parent_map.remove(&component_id);
        } else {
            store.root_component = None;
        }
        
        // Remove component
        store.components.remove(&component_id);
        
        Ok(())
    }
    
    fn description(&self) -> String {
        format!("Add {}", self.component.component_type)
    }
    
    fn can_merge(&self, _other: &dyn Command) -> bool {
        false
    }
    
    fn merge(&self, _other: Box<dyn Command>) -> Box<dyn Command> {
        unimplemented!()
    }
}

impl Command for RemoveComponentCommand {
    fn execute(&self, store: &mut SceneStore) -> Result<(), CommandError> {
        // Store component data for undo
        let component = store.components.get(&self.component_id)
            .ok_or(CommandError::ComponentNotFound(self.component_id))?
            .clone();
        
        // Remove from hierarchy
        if let Some(parent_id) = store.parent_map.get(&self.component_id).copied() {
            if let Some(children) = store.hierarchy.get_mut(&parent_id) {
                children.retain(|&id| id != self.component_id);
            }
            store.parent_map.remove(&self.component_id);
        } else if store.root_component == Some(self.component_id) {
            store.root_component = None;
        }
        
        // Remove component
        store.components.remove(&self.component_id);
        
        Ok(())
    }
    
    fn undo(&self, store: &mut SceneStore) -> Result<(), CommandError> {
        if let Some(component) = &self.component_data {
            // Restore component
            store.components.insert(self.component_id, component.clone());
            
            // Restore hierarchy
            if let Some(parent_id) = self.parent_id {
                store.hierarchy.entry(parent_id).or_default().push(self.component_id);
                store.parent_map.insert(self.component_id, parent_id);
            } else {
                store.root_component = Some(self.component_id);
            }
        }
        Ok(())
    }
    
    fn description(&self) -> String {
        "Remove component".to_string()
    }
    
    fn can_merge(&self, _other: &dyn Command) -> bool {
        false
    }
    
    fn merge(&self, _other: Box<dyn Command>) -> Box<dyn Command> {
        unimplemented!()
    }
}

impl Command for UpdatePropertyCommand {
    fn execute(&self, store: &mut SceneStore) -> Result<(), CommandError> {
        let component = store.components.get_mut(&self.component_id)
            .ok_or(CommandError::ComponentNotFound(self.component_id))?;
        
        component.properties.insert(self.property.clone(), self.new_value.clone());
        component.modified_at = Instant::now();
        
        // Record change
        let version = store.change_tracker.next_version();
        store.change_tracker.record_change(SceneChange {
            change_type: ChangeType::PropertyChanged,
            component_id: self.component_id,
            property: Some(self.property.clone()),
            old_value: Some(self.old_value.clone()),
            new_value: Some(self.new_value.clone()),
            timestamp: Instant::now(),
            version,
        });
        
        Ok(())
    }
    
    fn undo(&self, store: &mut SceneStore) -> Result<(), CommandError> {
        let component = store.components.get_mut(&self.component_id)
            .ok_or(CommandError::ComponentNotFound(self.component_id))?;
        
        component.properties.insert(self.property.clone(), self.old_value.clone());
        component.modified_at = Instant::now();
        
        Ok(())
    }
    
    fn description(&self) -> String {
        format!("Update {}", self.property)
    }
    
    fn can_merge(&self, other: &dyn Command) -> bool {
        // Can merge consecutive property updates to the same property
        if let Some(other_cmd) = other.as_any().downcast_ref::<UpdatePropertyCommand>() {
            self.component_id == other_cmd.component_id && 
            self.property == other_cmd.property
        } else {
            false
        }
    }
    
    fn merge(&self, other: Box<dyn Command>) -> Box<dyn Command> {
        if let Ok(other_cmd) = other.into_any().downcast::<UpdatePropertyCommand>() {
            Box::new(UpdatePropertyCommand {
                component_id: self.component_id,
                property: self.property.clone(),
                old_value: self.old_value.clone(),
                new_value: other_cmd.new_value,
            })
        } else {
            panic!("Cannot merge incompatible commands");
        }
    }
}

impl Command for GroupCommand {
    fn execute(&self, store: &mut SceneStore) -> Result<(), CommandError> {
        for command in &self.group.commands {
            command.execute(store)?;
        }
        Ok(())
    }
    
    fn undo(&self, store: &mut SceneStore) -> Result<(), CommandError> {
        // Undo in reverse order
        for command in self.group.commands.iter().rev() {
            command.undo(store)?;
        }
        Ok(())
    }
    
    fn description(&self) -> String {
        self.group.description.clone()
    }
    
    fn can_merge(&self, _other: &dyn Command) -> bool {
        false
    }
    
    fn merge(&self, _other: Box<dyn Command>) -> Box<dyn Command> {
        unimplemented!()
    }
}

// Helper trait for command downcasting
trait CommandAny {
    fn as_any(&self) -> &dyn std::any::Any;
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any>;
}

impl<T: Command + 'static> CommandAny for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

impl Default for SceneStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_store_creation() {
        let store = SceneStore::new();
        assert_eq!(store.components.len(), 0);
        assert_eq!(store.metadata.version, 1);
    }

    #[test]
    fn test_add_component() {
        let mut store = SceneStore::new();
        
        let component_id = store.add_component("Button", None).unwrap();
        assert!(store.components.contains_key(&component_id));
        assert_eq!(store.root_component, Some(component_id));
    }

    #[test]
    fn test_undo_redo() {
        let mut store = SceneStore::new();
        
        // Add component
        let component_id = store.add_component("Button", None).unwrap();
        assert!(store.components.contains_key(&component_id));
        
        // Undo
        store.undo().unwrap();
        assert!(!store.components.contains_key(&component_id));
        assert_eq!(store.root_component, None);
        
        // Redo
        store.redo().unwrap();
        assert!(store.components.contains_key(&component_id));
        assert_eq!(store.root_component, Some(component_id));
    }

    #[test]
    fn test_property_update() {
        let mut store = SceneStore::new();
        
        let component_id = store.add_component("Button", None).unwrap();
        
        // Update property
        store.update_property(
            component_id,
            "text",
            PropertyValue::String("Hello".to_string())
        ).unwrap();
        
        let component = store.get_component(component_id).unwrap();
        assert_eq!(
            component.properties.get("text"),
            Some(&PropertyValue::String("Hello".to_string()))
        );
    }

    #[test]
    fn test_selection() {
        let mut store = SceneStore::new();
        
        let component_id = store.add_component("Button", None).unwrap();
        
        store.set_selection(vec![component_id]);
        assert_eq!(store.get_selection(), &[component_id]);
        assert_eq!(store.selection.primary, Some(component_id));
    }

    #[test]
    fn test_scene_diffing() {
        let mut store = SceneStore::new();
        let snapshot1 = store.create_snapshot();
        
        // Add component
        let component_id = store.add_component("Button", None).unwrap();
        
        // Compute diff
        let diff = store.compute_diff(&snapshot1);
        assert_eq!(diff.changes.len(), 1);
        assert_eq!(diff.changes[0].change_type, ChangeType::ComponentAdded);
        assert_eq!(diff.changes[0].component_id, component_id);
    }
}