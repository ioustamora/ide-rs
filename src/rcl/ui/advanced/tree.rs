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
}
