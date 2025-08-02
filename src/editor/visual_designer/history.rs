/// Undo/redo and design history logic
#[derive(Clone)]
pub struct DesignHistory {
    pub operations: Vec<DesignOperation>,
    pub current_index: usize,
    pub max_size: usize,
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

/// Design operation for undo/redo
#[derive(Clone)]
pub enum DesignOperation {
    Move {
        component_ids: Vec<usize>,
        old_positions: Vec<egui::Pos2>,
        new_positions: Vec<egui::Pos2>,
    },
    Resize {
        component_id: usize,
        old_size: egui::Vec2,
        new_size: egui::Vec2,
    },
    Add {
        component_id: usize,
        position: egui::Pos2,
    },
    Delete {
        component_id: usize,
        component_data: String,
    },
    PropertyChange {
        component_id: usize,
        property_name: String,
        old_value: String,
        new_value: String,
    },
}
//! Undo/redo and design history logic
//!
//! Maintains operation history and state snapshots.

// TODO: Move all history-related structs and logic here.

pub struct DesignHistory {
    // ...fields...
}

impl DesignHistory {
    pub fn new() -> Self {
        Self {
            // ...
        }
    }

    pub fn add_to_history(&mut self, operation: DesignOperation) {
        self.operations.truncate(self.current_index);
        self.operations.push(operation);
        self.current_index = self.operations.len();
        if self.operations.len() > self.max_size {
            self.operations.remove(0);
            self.current_index -= 1;
        }
    }

    pub fn undo(&mut self, layout: &mut crate::editor::visual_designer::LayoutManager) -> bool {
        if self.current_index == 0 {
            return false;
        }
        self.current_index -= 1;
        let operation = &self.operations[self.current_index].clone();
        match operation {
            DesignOperation::Move { component_ids, old_positions, .. } => {
                for (i, &component_id) in component_ids.iter().enumerate() {
                    if let Some(old_pos) = old_positions.get(i) {
                        layout.positions.insert(component_id, *old_pos);
                    }
                }
            }
            // Other operation reversals would be implemented here
            _ => {}
        }
        true
    }

    pub fn redo(&mut self, layout: &mut crate::editor::visual_designer::LayoutManager) -> bool {
        if self.current_index >= self.operations.len() {
            return false;
        }
        let operation = &self.operations[self.current_index].clone();
        self.current_index += 1;
        match operation {
            DesignOperation::Move { component_ids, new_positions, .. } => {
                for (i, &component_id) in component_ids.iter().enumerate() {
                    if let Some(new_pos) = new_positions.get(i) {
                        layout.positions.insert(component_id, *new_pos);
                    }
                }
            }
            // Other operation applications would be implemented here
            _ => {}
        }
        true
    }
}
