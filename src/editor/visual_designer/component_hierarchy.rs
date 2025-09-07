//! Component Hierarchy Management
//!
//! Provides a tree-based view of the component hierarchy in the visual designer,
//! similar to the component inspector in modern design tools like Figma, Sketch, or XD.

use egui::*;
use std::collections::HashMap;

/// Hierarchical component tree node
#[derive(Debug, Clone)]
pub struct ComponentHierarchyNode {
    /// Component ID
    pub id: usize,
    /// Component name/type
    pub name: String,
    /// Parent component ID (None for root)
    pub parent: Option<usize>,
    /// Child component IDs
    pub children: Vec<usize>,
    /// Whether this node is expanded in the tree view
    pub expanded: bool,
    /// Whether this node is visible
    pub visible: bool,
    /// Whether this node is locked from editing
    pub locked: bool,
    /// Component depth in hierarchy
    pub depth: usize,
}

/// Component hierarchy manager for tree-based component organization
pub struct ComponentHierarchy {
    /// Tree nodes by component ID
    pub nodes: HashMap<usize, ComponentHierarchyNode>,
    /// Root component IDs (components with no parent)
    pub root_nodes: Vec<usize>,
    /// Currently selected node
    pub selected_node: Option<usize>,
    /// Drag state for hierarchy reordering
    pub drag_state: Option<HierarchyDragState>,
    /// Search filter for components
    pub search_filter: String,
    /// Show only visible components
    pub show_only_visible: bool,
}

/// Drag state for reordering components in hierarchy
#[derive(Debug, Clone)]
pub struct HierarchyDragState {
    /// Component being dragged
    pub dragging_id: usize,
    /// Drop target (component to drop onto)
    pub drop_target: Option<usize>,
    /// Drop position (before, after, or child)
    pub drop_position: DropPosition,
}

/// Position for dropping in hierarchy
#[derive(Debug, Clone, PartialEq)]
pub enum DropPosition {
    /// Drop as previous sibling
    Before,
    /// Drop as next sibling  
    After,
    /// Drop as child
    Child,
}

impl Default for ComponentHierarchy {
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentHierarchy {
    /// Create a new component hierarchy manager
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            root_nodes: Vec::new(),
            selected_node: None,
            drag_state: None,
            search_filter: String::new(),
            show_only_visible: true,
        }
    }

    /// Add a component to the hierarchy
    pub fn add_component(&mut self, id: usize, name: String, parent: Option<usize>) {
        let depth = if let Some(parent_id) = parent {
            self.nodes.get(&parent_id)
                .map(|p| p.depth + 1)
                .unwrap_or(0)
        } else {
            0
        };

        let node = ComponentHierarchyNode {
            id,
            name,
            parent,
            children: Vec::new(),
            expanded: true,
            visible: true,
            locked: false,
            depth,
        };

        // Add to parent's children
        if let Some(parent_id) = parent {
            if let Some(parent_node) = self.nodes.get_mut(&parent_id) {
                parent_node.children.push(id);
            }
        } else {
            self.root_nodes.push(id);
        }

        self.nodes.insert(id, node);
    }

    /// Remove a component from the hierarchy
    pub fn remove_component(&mut self, id: usize) {
        let (children, parent) = if let Some(node) = self.nodes.get(&id) {
            (node.children.clone(), node.parent)
        } else {
            return;
        };

        // Remove from parent's children
        if let Some(parent_id) = parent {
            if let Some(parent_node) = self.nodes.get_mut(&parent_id) {
                parent_node.children.retain(|&child_id| child_id != id);
            }
        } else {
            self.root_nodes.retain(|&root_id| root_id != id);
        }

        // Move children to parent or make them root nodes
        for child_id in children {
            if let Some(child_node) = self.nodes.get_mut(&child_id) {
                child_node.parent = parent;
                if let Some(parent_id) = parent {
                    if let Some(parent_node) = self.nodes.get_mut(&parent_id) {
                        parent_node.children.push(child_id);
                    }
                } else {
                    self.root_nodes.push(child_id);
                }
            }
        }

        self.nodes.remove(&id);
        
        // Clear selection if deleted component was selected
        if self.selected_node == Some(id) {
            self.selected_node = None;
        }
    }

    /// Render the hierarchy tree view
    pub fn render(&mut self, ui: &mut Ui) -> Option<usize> {
        let mut selected_component = None;

        // Search filter
        ui.horizontal(|ui| {
            ui.label("🔍");
            ui.text_edit_singleline(&mut self.search_filter);
            if ui.button("❌").clicked() {
                self.search_filter.clear();
            }
        });

        ui.separator();

        // Options
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.show_only_visible, "Show only visible");
        });

        ui.separator();

        // Hierarchy tree
        ScrollArea::vertical()
            .max_height(300.0)
            .show(ui, |ui| {
                for &root_id in &self.root_nodes.clone() {
                    if let Some(clicked_component) = self.render_node(ui, root_id, 0) {
                        selected_component = Some(clicked_component);
                    }
                }

                // Handle drop zones
                self.handle_drop_zones(ui);
            });

        selected_component
    }

    /// Render a single node in the hierarchy
    fn render_node(&mut self, ui: &mut Ui, node_id: usize, indent_level: usize) -> Option<usize> {
        let node = match self.nodes.get(&node_id) {
            Some(node) => node.clone(),
            None => return None,
        };

        // Apply search filter
        if !self.search_filter.is_empty() &&
           !node.name.to_lowercase().contains(&self.search_filter.to_lowercase()) {
            return None;
        }

        // Apply visibility filter
        if self.show_only_visible && !node.visible {
            return None;
        }

        let mut selected_component = None;
        let indent = indent_level as f32 * 20.0;

        ui.horizontal(|ui| {
            ui.add_space(indent);

            // Expand/collapse button
            if !node.children.is_empty() {
                let expand_button = if node.expanded { "▼" } else { "▶" };
                if ui.small_button(expand_button).clicked() {
                    if let Some(node_mut) = self.nodes.get_mut(&node_id) {
                        node_mut.expanded = !node_mut.expanded;
                    }
                }
            } else {
                ui.add_space(20.0);
            }

            // Visibility toggle
            let visibility_icon = if node.visible { "👁" } else { "👁‍🗨" };
            if ui.small_button(visibility_icon).clicked() {
                if let Some(node_mut) = self.nodes.get_mut(&node_id) {
                    node_mut.visible = !node_mut.visible;
                }
            }

            // Lock toggle  
            let lock_icon = if node.locked { "🔒" } else { "🔓" };
            if ui.small_button(lock_icon).clicked() {
                if let Some(node_mut) = self.nodes.get_mut(&node_id) {
                    node_mut.locked = !node_mut.locked;
                }
            }

            // Component name and selection
            let is_selected = self.selected_node == Some(node_id);
            let text_color = if is_selected { 
                ui.visuals().selection.bg_fill 
            } else { 
                ui.visuals().text_color() 
            };

            if ui.colored_label(text_color, &node.name).clicked() {
                self.selected_node = Some(node_id);
                selected_component = Some(node_id);
            }

            // Context menu
            ui.menu_button("⋮", |ui| {
                if ui.button("Duplicate").clicked() {
                    // TODO: Implement component duplication
                    ui.close_menu();
                }
                if ui.button("Delete").clicked() {
                    self.remove_component(node_id);
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Move to Top").clicked() {
                    // TODO: Implement hierarchy reordering
                    ui.close_menu();
                }
                if ui.button("Move to Bottom").clicked() {
                    // TODO: Implement hierarchy reordering
                    ui.close_menu();
                }
            });
        });

        // Render children if expanded
        if node.expanded {
            for &child_id in &node.children {
                if let Some(clicked_component) = self.render_node(ui, child_id, indent_level + 1) {
                    selected_component = Some(clicked_component);
                }
            }
        }

        selected_component
    }

    /// Handle drag and drop zones for hierarchy reordering
    fn handle_drop_zones(&mut self, _ui: &mut Ui) {
        // TODO: Implement drag and drop reordering
        // This would allow users to drag components to reorder them in the hierarchy
    }

    /// Get the path to a component (for breadcrumbs or navigation)
    pub fn get_component_path(&self, component_id: usize) -> Vec<String> {
        let mut path = Vec::new();
        let mut current_id = Some(component_id);

        while let Some(id) = current_id {
            if let Some(node) = self.nodes.get(&id) {
                path.insert(0, node.name.clone());
                current_id = node.parent;
            } else {
                break;
            }
        }

        path
    }

    /// Find components by name or type
    pub fn find_components(&self, query: &str) -> Vec<usize> {
        let query_lower = query.to_lowercase();
        self.nodes
            .iter()
            .filter(|(_, node)| {
                node.name.to_lowercase().contains(&query_lower)
            })
            .map(|(&id, _)| id)
            .collect()
    }

    /// Get component statistics
    pub fn get_statistics(&self) -> HierarchyStatistics {
        let total_components = self.nodes.len();
        let visible_components = self.nodes.values()
            .filter(|node| node.visible)
            .count();
        let locked_components = self.nodes.values()
            .filter(|node| node.locked)
            .count();
        
        let max_depth = self.nodes.values()
            .map(|node| node.depth)
            .max()
            .unwrap_or(0);

        HierarchyStatistics {
            total_components,
            visible_components,
            locked_components,
            max_depth,
        }
    }
}

/// Component hierarchy statistics
pub struct HierarchyStatistics {
    pub total_components: usize,
    pub visible_components: usize,
    pub locked_components: usize,
    pub max_depth: usize,
}