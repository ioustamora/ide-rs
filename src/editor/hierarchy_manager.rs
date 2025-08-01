//! Component Hierarchy and Z-Order Management
//!
//! This module provides comprehensive hierarchy management including:
//! - Component layering and z-order
//! - Parent-child relationships
//! - Visual hierarchy tree
//! - Layer management tools

use egui::*;
use crate::rcl::ui::component::Component;
use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};

/// Component hierarchy manager for organizing and layering components
pub struct HierarchyManager {
    /// Component hierarchy tree
    pub hierarchy: ComponentHierarchy,
    /// Z-order for components (higher values render on top)
    pub z_order: HashMap<usize, i32>,
    /// Selected components in hierarchy view
    pub selected_in_hierarchy: HashSet<usize>,
    /// Expanded nodes in hierarchy tree
    pub expanded_nodes: HashSet<usize>,
    /// Layer management system
    pub layer_manager: LayerManager,
    /// Whether the hierarchy panel is visible
    pub show_hierarchy_panel: bool,
}

/// Component hierarchy tree structure
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComponentHierarchy {
    /// Root components (no parent)
    pub root_components: Vec<usize>,
    /// Parent-child relationships
    pub parent_child: HashMap<usize, Vec<usize>>,
    /// Child-parent relationships (reverse lookup)
    pub child_parent: HashMap<usize, usize>,
    /// Component metadata for hierarchy
    pub component_metadata: HashMap<usize, ComponentMetadata>,
}

/// Metadata for components in the hierarchy
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComponentMetadata {
    /// Component display name
    pub name: String,
    /// Component type
    pub component_type: String,
    /// Whether component is visible
    pub visible: bool,
    /// Whether component is locked (cannot be selected/moved)
    pub locked: bool,
    /// Custom icon for the component
    pub icon: String,
    /// Layer assignment
    pub layer: String,
}

/// Layer management system
#[derive(Clone, Debug)]
pub struct LayerManager {
    /// Available layers
    pub layers: Vec<Layer>,
    /// Currently active layer
    pub active_layer: usize,
    /// Layer visibility states
    pub layer_visibility: HashMap<String, bool>,
    /// Layer lock states
    pub layer_locks: HashMap<String, bool>,
}

/// Layer definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Layer {
    /// Layer name
    pub name: String,
    /// Layer color for visual identification
    pub color: [u8; 4], // RGBA
    /// Layer opacity
    pub opacity: f32,
    /// Whether layer is visible
    pub visible: bool,
    /// Whether layer is locked
    pub locked: bool,
    /// Layer z-index base (components on this layer start from this value)
    pub z_base: i32,
}

/// Hierarchy operation for undo/redo
#[derive(Clone, Debug)]
pub enum HierarchyOperation {
    /// Move component to new parent
    Reparent { component: usize, old_parent: Option<usize>, new_parent: Option<usize> },
    /// Change component z-order
    ReorderZ { component: usize, old_z: i32, new_z: i32 },
    /// Create new layer
    CreateLayer { layer: Layer },
    /// Delete layer
    DeleteLayer { layer: Layer, affected_components: Vec<usize> },
    /// Move component to different layer
    MoveToLayer { component: usize, old_layer: String, new_layer: String },
}

impl Default for HierarchyManager {
    fn default() -> Self {
        let mut manager = Self {
            hierarchy: ComponentHierarchy::default(),
            z_order: HashMap::new(),
            selected_in_hierarchy: HashSet::new(),
            expanded_nodes: HashSet::new(),
            layer_manager: LayerManager::default(),
            show_hierarchy_panel: true,
        };
        
        // Initialize with default layers
        manager.layer_manager.create_default_layers();
        manager
    }
}

impl Default for ComponentHierarchy {
    fn default() -> Self {
        Self {
            root_components: Vec::new(),
            parent_child: HashMap::new(),
            child_parent: HashMap::new(),
            component_metadata: HashMap::new(),
        }
    }
}

impl Default for LayerManager {
    fn default() -> Self {
        Self {
            layers: Vec::new(),
            active_layer: 0,
            layer_visibility: HashMap::new(),
            layer_locks: HashMap::new(),
        }
    }
}

impl HierarchyManager {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a component to the hierarchy
    pub fn add_component(&mut self, component_id: usize, component: &dyn Component, parent: Option<usize>) {
        // Create metadata for the component
        let metadata = ComponentMetadata {
            name: format!("{} {}", component.name(), component_id),
            component_type: component.name().to_string(),
            visible: true,
            locked: false,
            icon: self.get_component_icon(component.name()),
            layer: self.layer_manager.get_active_layer_name(),
        };
        
        self.hierarchy.component_metadata.insert(component_id, metadata);
        
        // Set initial z-order
        let base_z = self.layer_manager.get_active_layer_z_base();
        let component_z = base_z + self.hierarchy.root_components.len() as i32;
        self.z_order.insert(component_id, component_z);
        
        // Add to hierarchy
        if let Some(parent_id) = parent {
            // Add as child
            self.hierarchy.parent_child.entry(parent_id).or_default().push(component_id);
            self.hierarchy.child_parent.insert(component_id, parent_id);
        } else {
            // Add as root component
            self.hierarchy.root_components.push(component_id);
        }
    }
    
    /// Remove a component from the hierarchy
    pub fn remove_component(&mut self, component_id: usize) {
        // Remove from metadata
        self.hierarchy.component_metadata.remove(&component_id);
        self.z_order.remove(&component_id);
        self.selected_in_hierarchy.remove(&component_id);
        self.expanded_nodes.remove(&component_id);
        
        // Remove from parent-child relationships
        if let Some(parent_id) = self.hierarchy.child_parent.remove(&component_id) {
            if let Some(children) = self.hierarchy.parent_child.get_mut(&parent_id) {
                children.retain(|&child_id| child_id != component_id);
            }
        }
        
        // Remove from root components if it's a root
        self.hierarchy.root_components.retain(|&id| id != component_id);
        
        // Reparent children to this component's parent
        if let Some(children) = self.hierarchy.parent_child.remove(&component_id) {
            let parent = self.hierarchy.child_parent.get(&component_id).copied();
            
            for child_id in children {
                if let Some(parent_id) = parent {
                    self.hierarchy.parent_child.entry(parent_id).or_default().push(child_id);
                    self.hierarchy.child_parent.insert(child_id, parent_id);
                } else {
                    self.hierarchy.root_components.push(child_id);
                    self.hierarchy.child_parent.remove(&child_id);
                }
            }
        }
    }
    
    /// Render the hierarchy panel
    pub fn render_hierarchy_panel(&mut self, ui: &mut Ui, components: &[Box<dyn Component>]) {
        if !self.show_hierarchy_panel {
            return;
        }
        
        ui.vertical(|ui| {
            // Header
            ui.horizontal(|ui| {
                ui.label("🗂");
                ui.heading("Hierarchy");
                
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("🔧").on_hover_text("Layer Management").clicked() {
                        // Toggle layer management panel
                    }
                    
                    if ui.button("📋").on_hover_text("Hierarchy Options").clicked() {
                        // Show hierarchy options menu
                    }
                });
            });
            
            ui.separator();
            
            // Layer management
            self.render_layer_management(ui);
            
            ui.separator();
            
            // Component hierarchy tree
            ScrollArea::vertical().show(ui, |ui| {
                self.render_hierarchy_tree(ui, components);
            });
            
            ui.separator();
            
            // Hierarchy actions
            self.render_hierarchy_actions(ui);
        });
    }
    
    /// Render layer management section
    fn render_layer_management(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Layers:");
            
            ComboBox::from_id_source("active_layer")
                .selected_text(&self.layer_manager.get_active_layer_name())
                .show_ui(ui, |ui| {
                    for (i, layer) in self.layer_manager.layers.iter().enumerate() {
                        if ui.selectable_value(&mut self.layer_manager.active_layer, i, &layer.name).clicked() {
                            // Layer changed
                        }
                    }
                });
            
            if ui.button("➕").on_hover_text("Add Layer").clicked() {
                self.create_new_layer();
            }
        });
        
        // Layer visibility toggles
        ui.horizontal_wrapped(|ui| {
            for layer in &mut self.layer_manager.layers {
                let color = Color32::from_rgba_unmultiplied(layer.color[0], layer.color[1], layer.color[2], layer.color[3]);
                
                ui.horizontal(|ui| {
                    // Color indicator
                    let (color_rect, _) = ui.allocate_exact_size(Vec2::new(12.0, 12.0), egui::Sense::click());
                    ui.painter().rect_filled(color_rect, 2.0, color);
                    
                    // Visibility toggle
                    if ui.checkbox(&mut layer.visible, "").changed() {
                        self.layer_manager.layer_visibility.insert(layer.name.clone(), layer.visible);
                    }
                    
                    // Layer name
                    ui.label(&layer.name);
                    
                    // Lock toggle
                    let lock_icon = if layer.locked { "🔒" } else { "🔓" };
                    if ui.button(lock_icon).clicked() {
                        layer.locked = !layer.locked;
                        self.layer_manager.layer_locks.insert(layer.name.clone(), layer.locked);
                    }
                });
            }
        });
    }
    
    /// Render the component hierarchy tree
    fn render_hierarchy_tree(&mut self, ui: &mut Ui, components: &[Box<dyn Component>]) {
        // Render root components
        for &component_id in &self.hierarchy.root_components.clone() {
            self.render_hierarchy_node(ui, component_id, 0, components);
        }
        
        // Handle drag and drop for hierarchy reorganization
        self.handle_hierarchy_drag_drop(ui);
    }
    
    /// Render a single hierarchy node
    fn render_hierarchy_node(&mut self, ui: &mut Ui, component_id: usize, depth: usize, components: &[Box<dyn Component>]) {
        let metadata = if let Some(meta) = self.hierarchy.component_metadata.get(&component_id) {
            meta.clone()
        } else {
            return;
        };
        
        let has_children = self.hierarchy.parent_child.get(&component_id).map_or(false, |children| !children.is_empty());
        let is_expanded = self.expanded_nodes.contains(&component_id);
        let is_selected = self.selected_in_hierarchy.contains(&component_id);
        
        // Indentation
        let indent = depth as f32 * 16.0;
        ui.horizontal(|ui| {
            ui.add_space(indent);
            
            // Expand/collapse button
            if has_children {
                let expand_icon = if is_expanded { "▼" } else { "▶" };
                if ui.small_button(expand_icon).clicked() {
                    if is_expanded {
                        self.expanded_nodes.remove(&component_id);
                    } else {
                        self.expanded_nodes.insert(component_id);
                    }
                }
            } else {
                ui.add_space(20.0); // Space for alignment
            }
            
            // Component icon
            ui.label(&metadata.icon);
            
            // Component name
            let name_response = ui.selectable_label(is_selected, &metadata.name);
            
            if name_response.clicked() {
                if ui.input(|i| i.modifiers.ctrl) {
                    // Multi-select
                    if is_selected {
                        self.selected_in_hierarchy.remove(&component_id);
                    } else {
                        self.selected_in_hierarchy.insert(component_id);
                    }
                } else {
                    // Single select
                    self.selected_in_hierarchy.clear();
                    self.selected_in_hierarchy.insert(component_id);
                }
            }
            
            // Component visibility toggle
            let mut visible = metadata.visible;
            if ui.checkbox(&mut visible, "").changed() {
                if let Some(meta) = self.hierarchy.component_metadata.get_mut(&component_id) {
                    meta.visible = visible;
                }
            }
            
            // Component lock toggle
            let lock_icon = if metadata.locked { "🔒" } else { "🔓" };
            if ui.small_button(lock_icon).clicked() {
                if let Some(meta) = self.hierarchy.component_metadata.get_mut(&component_id) {
                    meta.locked = !meta.locked;
                }
            }
            
            // Z-order controls
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui.small_button("▲").on_hover_text("Move Up").clicked() {
                    self.move_component_up(component_id);
                }
                if ui.small_button("▼").on_hover_text("Move Down").clicked() {
                    self.move_component_down(component_id);
                }
                
                // Show z-order value
                if let Some(&z_order) = self.z_order.get(&component_id) {
                    ui.label(format!("Z:{}", z_order));
                }
            });
            
            // Context menu
            name_response.context_menu(|ui| {
                self.render_component_context_menu(ui, component_id);
            });
        });
        
        // Render children if expanded
        if is_expanded && has_children {
            let children: Vec<usize> = self.hierarchy.parent_child.get(&component_id)
                .map(|children| children.clone())
                .unwrap_or_default();
            for child_id in children {
                self.render_hierarchy_node(ui, child_id, depth + 1, components);
            }
        }
    }
    
    /// Render context menu for a component
    fn render_component_context_menu(&mut self, ui: &mut Ui, component_id: usize) {
        if ui.button("🔝 Bring to Front").clicked() {
            self.bring_to_front(component_id);
            ui.close_menu();
        }
        
        if ui.button("🔃 Send to Back").clicked() {
            self.send_to_back(component_id);
            ui.close_menu();
        }
        
        ui.separator();
        
        if ui.button("📋 Duplicate").clicked() {
            // TODO: Implement component duplication
            ui.close_menu();
        }
        
        if ui.button("🗑 Delete").clicked() {
            // TODO: Implement component deletion
            ui.close_menu();
        }
        
        ui.separator();
        
        if ui.button("✏ Rename").clicked() {
            // TODO: Implement component renaming
            ui.close_menu();
        }
        
        // Layer assignment submenu
        let layers: Vec<String> = self.layer_manager.layers.iter().map(|l| l.name.clone()).collect();
        ui.menu_button("📁 Move to Layer", |ui| {
            for layer_name in layers {
                if ui.button(&layer_name).clicked() {
                    self.move_component_to_layer(component_id, &layer_name);
                    ui.close_menu();
                }
            }
        });
    }
    
    /// Render hierarchy actions
    fn render_hierarchy_actions(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("🔝 Front").on_hover_text("Bring selected to front").clicked() {
                for &component_id in &self.selected_in_hierarchy.clone() {
                    self.bring_to_front(component_id);
                }
            }
            
            if ui.button("🔃 Back").on_hover_text("Send selected to back").clicked() {
                for &component_id in &self.selected_in_hierarchy.clone() {
                    self.send_to_back(component_id);
                }
            }
            
            ui.separator();
            
            if ui.button("📋 Group").on_hover_text("Group selected components").clicked() {
                self.group_selected_components();
            }
            
            if ui.button("📤 Ungroup").on_hover_text("Ungroup selected components").clicked() {
                self.ungroup_selected_components();
            }
        });
    }
    
    /// Handle drag and drop operations in hierarchy
    fn handle_hierarchy_drag_drop(&mut self, _ui: &mut Ui) {
        // TODO: Implement drag and drop for hierarchy reorganization
        // This would allow dragging components to reorder or reparent them
    }
    
    /// Move component up in z-order
    pub fn move_component_up(&mut self, component_id: usize) {
        if let Some(&current_z) = self.z_order.get(&component_id) {
            self.z_order.insert(component_id, current_z + 1);
        }
    }
    
    /// Move component down in z-order
    pub fn move_component_down(&mut self, component_id: usize) {
        if let Some(&current_z) = self.z_order.get(&component_id) {
            self.z_order.insert(component_id, (current_z - 1).max(0));
        }
    }
    
    /// Bring component to front (highest z-order)
    pub fn bring_to_front(&mut self, component_id: usize) {
        let max_z = self.z_order.values().max().copied().unwrap_or(0);
        self.z_order.insert(component_id, max_z + 1);
    }
    
    /// Send component to back (lowest z-order)
    pub fn send_to_back(&mut self, component_id: usize) {
        let min_z = self.z_order.values().min().copied().unwrap_or(0);
        self.z_order.insert(component_id, min_z - 1);
    }
    
    /// Move component to a different layer
    pub fn move_component_to_layer(&mut self, component_id: usize, layer_name: &str) {
        if let Some(metadata) = self.hierarchy.component_metadata.get_mut(&component_id) {
            metadata.layer = layer_name.to_string();
            
            // Update z-order based on new layer
            if let Some(layer) = self.layer_manager.layers.iter().find(|l| l.name == layer_name) {
                let new_z = layer.z_base + self.get_layer_component_count(layer_name) as i32;
                self.z_order.insert(component_id, new_z);
            }
        }
    }
    
    /// Group selected components
    fn group_selected_components(&mut self) {
        // TODO: Implement component grouping
        // This would create a logical group that can be moved/manipulated together
    }
    
    /// Ungroup selected components
    fn ungroup_selected_components(&mut self) {
        // TODO: Implement component ungrouping
    }
    
    /// Get component icon based on type
    fn get_component_icon(&self, component_type: &str) -> String {
        match component_type {
            "Button" => "🔘".to_string(),
            "Label" => "🏷".to_string(),
            "TextBox" => "📝".to_string(),
            "Checkbox" => "☑".to_string(),
            "Slider" => "🎚".to_string(),
            "Dropdown" => "📋".to_string(),
            "Chart" => "📊".to_string(),
            _ => "📦".to_string(),
        }
    }
    
    /// Create a new layer
    fn create_new_layer(&mut self) {
        let layer_count = self.layer_manager.layers.len();
        let new_layer = Layer {
            name: format!("Layer {}", layer_count + 1),
            color: [100 + (layer_count * 50) as u8 % 255, 150, 200, 255],
            opacity: 1.0,
            visible: true,
            locked: false,
            z_base: (layer_count as i32 + 1) * 100,
        };
        
        self.layer_manager.layers.push(new_layer);
    }
    
    /// Get number of components in a layer
    fn get_layer_component_count(&self, layer_name: &str) -> usize {
        self.hierarchy.component_metadata.values()
            .filter(|meta| meta.layer == layer_name)
            .count()
    }
    
    /// Get components sorted by z-order
    pub fn get_components_by_z_order(&self) -> Vec<usize> {
        let mut components: Vec<(usize, i32)> = self.z_order.iter()
            .map(|(&id, &z)| (id, z))
            .collect();
        
        components.sort_by_key(|(_, z)| *z);
        components.into_iter().map(|(id, _)| id).collect()
    }
    
    /// Get z-order for a component
    pub fn get_z_order(&self, component_id: usize) -> i32 {
        self.z_order.get(&component_id).copied().unwrap_or(0)
    }
    
    /// Check if component is visible
    pub fn is_component_visible(&self, component_id: usize) -> bool {
        if let Some(metadata) = self.hierarchy.component_metadata.get(&component_id) {
            if !metadata.visible {
                return false;
            }
            
            // Check layer visibility
            if let Some(layer) = self.layer_manager.layers.iter().find(|l| l.name == metadata.layer) {
                return layer.visible;
            }
        }
        true
    }
    
    /// Check if component is locked
    pub fn is_component_locked(&self, component_id: usize) -> bool {
        if let Some(metadata) = self.hierarchy.component_metadata.get(&component_id) {
            if metadata.locked {
                return true;
            }
            
            // Check layer lock
            if let Some(layer) = self.layer_manager.layers.iter().find(|l| l.name == metadata.layer) {
                return layer.locked;
            }
        }
        false
    }
    
    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selected_in_hierarchy.clear();
    }
    
    /// Get selected components
    pub fn get_selected_components(&self) -> Vec<usize> {
        self.selected_in_hierarchy.iter().copied().collect()
    }
}

impl LayerManager {
    /// Create default layers
    fn create_default_layers(&mut self) {
        self.layers = vec![
            Layer {
                name: "Background".to_string(),
                color: [100, 100, 100, 255],
                opacity: 1.0,
                visible: true,
                locked: false,
                z_base: 0,
            },
            Layer {
                name: "Main".to_string(),
                color: [150, 150, 200, 255],
                opacity: 1.0,
                visible: true,
                locked: false,
                z_base: 100,
            },
            Layer {
                name: "UI".to_string(),
                color: [200, 150, 100, 255],
                opacity: 1.0,
                visible: true,
                locked: false,
                z_base: 200,
            },
        ];
        
        self.active_layer = 1; // Start with "Main" layer
    }
    
    /// Get active layer name
    fn get_active_layer_name(&self) -> String {
        self.layers.get(self.active_layer)
            .map(|layer| layer.name.clone())
            .unwrap_or("Main".to_string())
    }
    
    /// Get active layer z-base
    fn get_active_layer_z_base(&self) -> i32 {
        self.layers.get(self.active_layer)
            .map(|layer| layer.z_base)
            .unwrap_or(100)
    }
}