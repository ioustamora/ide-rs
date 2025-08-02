// Moved from rcl/tree.rs
//! Tree component for RCL advanced UI
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct TreeNode {
    pub label: String,
    pub children: Vec<TreeNode>,
    pub editable: bool,
}

pub struct Tree {
    pub root: TreeNode,
    pub editable: bool,
}

impl Component for Tree {
    fn name(&self) -> &str {
        "Tree"
    }
    fn render(&mut self, ui: &mut Ui) {
        fn render_node(node: &mut TreeNode, ui: &mut Ui, editable: bool) {
            if editable {
                ui.text_edit_singleline(&mut node.label);
            } else {
                ui.collapsing(&node.label, |ui| {
                    for child in &mut node.children {
                        render_node(child, ui, editable);
                    }
                });
            }
            if ui.button("Edit").clicked() {
                node.editable = !node.editable;
            }
        }
        render_node(&mut self.root, ui, self.editable);
        if ui.button("Edit Tree").clicked() {
            self.editable = !self.editable;
        }
    }
    
    fn get_property(&self, name: &str) -> Option<String> {
        match name {
            "root_label" => Some(self.root.label.clone()),
            "editable" => Some(self.editable.to_string()),
            "child_count" => Some(self.root.children.len().to_string()),
            "total_nodes" => Some(self.count_total_nodes().to_string()),
            _ => None,
        }
    }
    
    fn set_property(&mut self, name: &str, value: &str) -> bool {
        match name {
            "root_label" => {
                self.root.label = value.to_string();
                true
            }
            "editable" => {
                if let Ok(editable) = value.parse::<bool>() {
                    self.editable = editable;
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
    
    fn get_property_names(&self) -> Vec<String> {
        vec![
            "root_label".to_string(),
            "editable".to_string(),
            "child_count".to_string(),
            "total_nodes".to_string(),
        ]
    }
}

impl Tree {
    fn count_total_nodes(&self) -> usize {
        fn count_node(node: &TreeNode) -> usize {
            1 + node.children.iter().map(count_node).sum::<usize>()
        }
        count_node(&self.root)
    }
}
