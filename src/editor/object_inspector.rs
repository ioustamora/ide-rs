/// Object Inspector - Delphi/C++ Builder inspired component hierarchy tree
/// 
/// This module provides a hierarchical view of all components in the form,
/// similar to the Object Inspector in Delphi/C++ Builder. It allows users to:
/// - View component hierarchy in a tree structure
/// - Select components from the tree
/// - Navigate parent-child relationships
/// - Rename components
/// - See component types and names

use egui::*;
use std::collections::HashMap;
use crate::rcl::ui::component::Component;

/// Object Inspector UI component
pub struct ObjectInspector {
    /// Whether the inspector is visible
    pub visible: bool,
    /// Currently selected component in the tree
    pub selected_component: Option<usize>,
    /// Expanded nodes in the tree
    pub expanded_nodes: HashMap<usize, bool>,
    /// Component names (user-defined names)
    pub component_names: HashMap<usize, String>,
    /// Search filter for components
    pub search_filter: String,
    /// Show only visible components
    pub show_visible_only: bool,
    /// Tree node icons
    pub node_icons: HashMap<String, &'static str>,
}

/// Component hierarchy node
#[derive(Debug, Clone)]
pub struct ComponentNode {
    /// Component index
    pub index: usize,
    /// Component name (user-defined)
    pub name: String,
    /// Component type (Button, Label, etc.)
    pub component_type: String,
    /// Parent component index (None for root form)
    pub parent: Option<usize>,
    /// Child component indices
    pub children: Vec<usize>,
    /// Whether component is visible
    pub visible: bool,
    /// Whether component is locked
    pub locked: bool,
    /// Nesting level in hierarchy
    pub level: usize,
}

impl Default for ObjectInspector {
    fn default() -> Self {
        let mut node_icons = HashMap::new();
        node_icons.insert("Form".to_string(), "📋");
        node_icons.insert("Button".to_string(), "🔲");
        node_icons.insert("Label".to_string(), "🏷");
        node_icons.insert("TextBox".to_string(), "📝");
        node_icons.insert("Checkbox".to_string(), "☑");
        node_icons.insert("Slider".to_string(), "🎚");
        node_icons.insert("Panel".to_string(), "📦");
        node_icons.insert("GroupBox".to_string(), "📋");
        node_icons.insert("TabControl".to_string(), "📑");
        node_icons.insert("ListView".to_string(), "📋");
        node_icons.insert("TreeView".to_string(), "🌳");
        
        Self {
            visible: true,
            selected_component: None,
            expanded_nodes: HashMap::new(),
            component_names: HashMap::new(),
            search_filter: String::new(),
            show_visible_only: false,
            node_icons,
        }
    }
}

impl ObjectInspector {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Render the Object Inspector UI
    pub fn render_ui(
        &mut self,
        ui: &mut Ui,
        components: &mut Vec<Box<dyn Component>>,
        root_form: &crate::rcl::ui::basic::form::Form,
        selected_components: &std::collections::HashSet<usize>,
    ) -> Option<usize> {
        if !self.visible {
            return None;
        }
        
        let mut clicked_component = None;
        
        ui.vertical(|ui| {
            // Header
            self.render_header(ui);
            
            ui.separator();
            
            // Search and filters
            self.render_search_controls(ui);
            
            ui.separator();
            
            // Component tree
            ScrollArea::vertical()
                .id_source("object_inspector_scroll")
                .show(ui, |ui| {
                    clicked_component = self.render_component_tree(
                        ui, 
                        components, 
                        root_form, 
                        selected_components
                    );
                });
        });
        
        clicked_component
    }
    
    /// Render the header with title and controls
    fn render_header(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("🔍 Object Inspector").strong().size(16.0));
            
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                // Toggle visibility button
                if ui.button("👁").on_hover_text("Toggle visibility filter").clicked() {
                    self.show_visible_only = !self.show_visible_only;
                }
                
                // Expand all button
                if ui.button("⬇").on_hover_text("Expand all nodes").clicked() {
                    self.expand_all_nodes();
                }
                
                // Collapse all button
                if ui.button("⬆").on_hover_text("Collapse all nodes").clicked() {
                    self.collapse_all_nodes();
                }
            });
        });
    }
    
    /// Render search and filter controls
    fn render_search_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Search:");
            ui.text_edit_singleline(&mut self.search_filter);
            
            if ui.button("✖").on_hover_text("Clear search").clicked() {
                self.search_filter.clear();
            }
        });
        
        ui.checkbox(&mut self.show_visible_only, "Show visible only");
    }
    
    /// Render the component hierarchy tree
    fn render_component_tree(
        &mut self,
        ui: &mut Ui,
        components: &mut Vec<Box<dyn Component>>,
        root_form: &crate::rcl::ui::basic::form::Form,
        selected_components: &std::collections::HashSet<usize>,
    ) -> Option<usize> {
        let mut clicked_component = None;
        
        // Build component hierarchy
        let hierarchy = self.build_component_hierarchy(components, root_form);
        
        // Render root form first
        if let Some(form_response) = self.render_tree_node(
            ui,
            &ComponentNode {
                index: usize::MAX,
                name: self.get_component_name(usize::MAX, "Form1"),
                component_type: "Form".to_string(),
                parent: None,
                children: (0..components.len()).collect(),
                visible: true,
                locked: false,
                level: 0,
            },
            selected_components.contains(&usize::MAX),
            &hierarchy,
        ) {
            clicked_component = Some(form_response);
        }
        
        // Render component nodes
        for node in hierarchy.values() {
            if node.parent.is_none() && node.index != usize::MAX {
                if let Some(response) = self.render_tree_node_recursive(
                    ui,
                    node,
                    selected_components,
                    &hierarchy,
                ) {
                    clicked_component = Some(response);
                }
            }
        }
        
        clicked_component
    }
    
    /// Render a tree node recursively
    fn render_tree_node_recursive(
        &mut self,
        ui: &mut Ui,
        node: &ComponentNode,
        selected_components: &std::collections::HashSet<usize>,
        hierarchy: &HashMap<usize, ComponentNode>,
    ) -> Option<usize> {
        let mut clicked_component = None;
        
        // Check if node should be shown based on filters
        if !self.should_show_node(node) {
            return None;
        }
        
        // Render this node
        if let Some(response) = self.render_tree_node(
            ui,
            node,
            selected_components.contains(&node.index),
            hierarchy,
        ) {
            clicked_component = Some(response);
        }
        
        // Render children if expanded
        if self.is_node_expanded(node.index) {
            ui.indent(format!("children_{}", node.index), |ui| {
                for &child_index in &node.children {
                    if let Some(child_node) = hierarchy.get(&child_index) {
                        if let Some(response) = self.render_tree_node_recursive(
                            ui,
                            child_node,
                            selected_components,
                            hierarchy,
                        ) {
                            clicked_component = Some(response);
                        }
                    }
                }
            });
        }
        
        clicked_component
    }
    
    /// Render a single tree node
    fn render_tree_node(
        &mut self,
        ui: &mut Ui,
        node: &ComponentNode,
        is_selected: bool,
        hierarchy: &HashMap<usize, ComponentNode>,
    ) -> Option<usize> {
        let mut clicked_component = None;
        
        // Node background color
        let bg_color = if is_selected {
            ui.style().visuals.selection.bg_fill
        } else {
            Color32::TRANSPARENT
        };
        
        // Calculate indentation based on level
        let indent = node.level as f32 * 16.0;
        
        ui.horizontal(|ui| {
            // Indentation
            ui.add_space(indent);
            
            // Expand/collapse button for nodes with children
            if !node.children.is_empty() {
                let expanded = self.is_node_expanded(node.index);
                let icon = if expanded { "▼" } else { "▶" };
                
                if ui.small_button(icon).clicked() {
                    self.toggle_node_expansion(node.index);
                }
            } else {
                ui.add_space(20.0); // Space for alignment
            }
            
            // Component icon
            let icon = self.node_icons.get(&node.component_type)
                .unwrap_or(&"📄");
            ui.label(*icon);
            
            // Component name and type
            let name_text = if node.name.is_empty() {
                format!("{}{}", node.component_type, node.index)
            } else {
                node.name.clone()
            };
            
            let response = ui.selectable_label(
                is_selected,
                RichText::new(format!("{} : {}", name_text, node.component_type))
                    .color(if node.visible { ui.style().visuals.text_color() } else { Color32::GRAY })
            );
            
            if response.clicked() {
                clicked_component = Some(node.index);
            }
            
            // Context menu
            response.context_menu(|ui| {
                self.render_node_context_menu(ui, node);
            });
            
            // Status indicators
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if node.locked {
                    ui.label("🔒");
                }
                if !node.visible {
                    ui.label("👁‍🗨");
                }
            });
        });
        
        clicked_component
    }
    
    /// Render context menu for a tree node
    fn render_node_context_menu(&mut self, ui: &mut Ui, _node: &ComponentNode) {
        if ui.button("📝 Rename").clicked() {
            // TODO: Implement rename dialog
            ui.close_menu();
        }
        
        if ui.button("👁 Toggle Visibility").clicked() {
            // TODO: Toggle component visibility
            ui.close_menu();
        }
        
        if ui.button("🔒 Toggle Lock").clicked() {
            // TODO: Toggle component lock
            ui.close_menu();
        }
        
        ui.separator();
        
        if ui.button("📋 Copy").clicked() {
            // TODO: Copy component
            ui.close_menu();
        }
        
        if ui.button("🗑 Delete").clicked() {
            // TODO: Delete component
            ui.close_menu();
        }
        
        ui.separator();
        
        if ui.button("⬆ Bring to Front").clicked() {
            // TODO: Bring to front
            ui.close_menu();
        }
        
        if ui.button("⬇ Send to Back").clicked() {
            // TODO: Send to back
            ui.close_menu();
        }
    }
    
    /// Build component hierarchy from flat component list
    fn build_component_hierarchy(
        &self,
        components: &[Box<dyn Component>],
        _root_form: &crate::rcl::ui::basic::form::Form,
    ) -> HashMap<usize, ComponentNode> {
        let mut hierarchy = HashMap::new();
        
        // For now, all components are direct children of the form
        // In a more advanced implementation, we'd parse parent-child relationships
        for (index, component) in components.iter().enumerate() {
            let node = ComponentNode {
                index,
                name: self.get_component_name(index, ""),
                component_type: component.name().to_string(),
                parent: Some(usize::MAX), // Form is parent
                children: Vec::new(),
                visible: true, // TODO: Get from component properties
                locked: false, // TODO: Get from component properties
                level: 1, // Direct children of form
            };
            
            hierarchy.insert(index, node);
        }
        
        hierarchy
    }
    
    /// Get component name (user-defined or default)
    fn get_component_name(&self, index: usize, default: &str) -> String {
        self.component_names.get(&index)
            .cloned()
            .or_else(|| if default.is_empty() { None } else { Some(default.to_string()) })
            .unwrap_or_else(|| format!("Component{}", index))
    }
    
    /// Check if a node should be shown based on current filters
    fn should_show_node(&self, node: &ComponentNode) -> bool {
        // Visibility filter
        if self.show_visible_only && !node.visible {
            return false;
        }
        
        // Search filter
        if !self.search_filter.is_empty() {
            let search_lower = self.search_filter.to_lowercase();
            let name_match = node.name.to_lowercase().contains(&search_lower);
            let type_match = node.component_type.to_lowercase().contains(&search_lower);
            
            if !name_match && !type_match {
                return false;
            }
        }
        
        true
    }
    
    /// Check if a node is expanded
    fn is_node_expanded(&self, index: usize) -> bool {
        self.expanded_nodes.get(&index).copied().unwrap_or(true)
    }
    
    /// Toggle node expansion
    fn toggle_node_expansion(&mut self, index: usize) {
        let current = self.is_node_expanded(index);
        self.expanded_nodes.insert(index, !current);
    }
    
    /// Expand all nodes
    fn expand_all_nodes(&mut self) {
        // This would need to iterate through all known component indices
        // For now, we'll just clear the expanded_nodes map since default is true
        self.expanded_nodes.clear();
    }
    
    /// Collapse all nodes
    fn collapse_all_nodes(&mut self) {
        // Set all known nodes to collapsed
        // In a full implementation, we'd iterate through all component indices
        for i in 0..100 { // Arbitrary limit for demo
            self.expanded_nodes.insert(i, false);
        }
        self.expanded_nodes.insert(usize::MAX, false); // Form
    }
    
    /// Set component name
    pub fn set_component_name(&mut self, index: usize, name: String) {
        self.component_names.insert(index, name);
    }
    
    /// Get component name
    pub fn get_component_name_by_index(&self, index: usize) -> Option<&String> {
        self.component_names.get(&index)
    }
    
    /// Select component in tree
    pub fn select_component(&mut self, index: Option<usize>) {
        self.selected_component = index;
    }
    
    /// Get selected component
    pub fn get_selected_component(&self) -> Option<usize> {
        self.selected_component
    }
    
    /// Toggle inspector visibility
    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }
    
    /// Set inspector visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    
    /// Check if inspector is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }
}