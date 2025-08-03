/// Undo/redo and design history logic with operation batching
#[derive(Clone)]
pub struct DesignHistory {
    pub operations: Vec<OperationBatch>,
    pub current_index: usize,
    pub max_size: usize,
    /// Current batch being built (for operation batching)
    pub current_batch: Option<OperationBatch>,
    /// Auto-batch timeout in milliseconds
    pub batch_timeout_ms: u64,
}

/// A batch of operations that should be undone/redone together
#[derive(Clone)]
pub struct OperationBatch {
    pub operations: Vec<DesignOperation>,
    pub description: String,
    pub timestamp: std::time::Instant,
}

impl OperationBatch {
    pub fn new(description: String) -> Self {
        Self {
            operations: Vec::new(),
            description,
            timestamp: std::time::Instant::now(),
        }
    }
    
    pub fn add_operation(&mut self, operation: DesignOperation) {
        self.operations.push(operation);
    }
    
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }
    
    pub fn len(&self) -> usize {
        self.operations.len()
    }
}

impl Default for DesignHistory {
    fn default() -> Self {
        Self {
            operations: Vec::new(),
            current_index: 0,
            max_size: 100,
            current_batch: None,
            batch_timeout_ms: 1000, // 1 second
        }
    }
}

/// Design operation for undo/redo
#[derive(Clone, Debug)]
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
        old_position: Option<egui::Pos2>, // For resize operations that also change position
        new_position: Option<egui::Pos2>,
    },
    Add {
        component_id: usize,
        position: egui::Pos2,
        size: egui::Vec2,
        component_type: String,
        component_data: String, // Serialized component state
    },
    Delete {
        component_id: usize,
        position: egui::Pos2,
        size: egui::Vec2,
        component_type: String,
        component_data: String, // Serialized component state for restoration
    },
    PropertyChange {
        component_id: usize,
        property_name: String,
        old_value: String,
        new_value: String,
    },
    Group {
        group_id: usize,
        component_ids: Vec<usize>,
        old_positions: Vec<egui::Pos2>, // Relative to their previous positions
        new_positions: Vec<egui::Pos2>, // Relative to group
    },
    Ungroup {
        group_id: usize,
        component_ids: Vec<usize>,
        old_positions: Vec<egui::Pos2>,
        new_positions: Vec<egui::Pos2>,
    },
    LayerChange {
        component_id: usize,
        old_layer: i32,
        new_layer: i32,
    },
    Duplicate {
        original_ids: Vec<usize>,
        new_ids: Vec<usize>,
        positions: Vec<egui::Pos2>,
        sizes: Vec<egui::Vec2>,
    },
    Cut {
        component_ids: Vec<usize>,
        positions: Vec<egui::Pos2>,
        sizes: Vec<egui::Vec2>,
        component_data: Vec<String>,
    },
    Paste {
        component_ids: Vec<usize>,
        positions: Vec<egui::Pos2>,
        sizes: Vec<egui::Vec2>,
        component_data: Vec<String>,
    },
}
impl DesignHistory {
    pub fn new() -> Self {
        Self::default()
    }

    /// Start a new operation batch
    pub fn begin_batch(&mut self, description: String) {
        // Finish current batch if any
        self.end_batch();
        self.current_batch = Some(OperationBatch::new(description));
    }

    /// End the current operation batch and add it to history
    pub fn end_batch(&mut self) {
        if let Some(batch) = self.current_batch.take() {
            if !batch.is_empty() {
                self.add_batch_to_history(batch);
            }
        }
    }

    /// Add an operation to the current batch, or create a single-operation batch
    pub fn add_operation(&mut self, operation: DesignOperation) {
        if let Some(ref mut batch) = self.current_batch {
            batch.add_operation(operation);
        } else {
            // Create a single-operation batch
            let description = self.operation_description(&operation);
            let mut batch = OperationBatch::new(description);
            batch.add_operation(operation);
            self.add_batch_to_history(batch);
        }
    }

    /// Add an operation to history (legacy method for compatibility)
    pub fn add_to_history(&mut self, operation: DesignOperation) {
        self.add_operation(operation);
    }

    /// Add a batch to the history
    fn add_batch_to_history(&mut self, batch: OperationBatch) {
        // Clear any future operations (we're creating a new branch)
        self.operations.truncate(self.current_index);
        
        self.operations.push(batch);
        self.current_index = self.operations.len();
        
        // Maintain max size
        if self.operations.len() > self.max_size {
            self.operations.remove(0);
            self.current_index -= 1;
        }
    }

    /// Check if we should auto-end the current batch due to timeout
    pub fn maybe_end_batch_on_timeout(&mut self) {
        if let Some(ref batch) = self.current_batch {
            if batch.timestamp.elapsed().as_millis() > self.batch_timeout_ms as u128 {
                self.end_batch();
            }
        }
    }

    /// Undo the last batch of operations
    pub fn undo(&mut self, layout: &mut super::LayoutManager, components: &mut Vec<Box<dyn crate::rcl::ui::component::Component>>) -> bool {
        // End current batch before undoing
        self.end_batch();
        
        if self.current_index == 0 {
            return false;
        }
        
        self.current_index -= 1;
        let batch = &self.operations[self.current_index].clone();
        
        // Apply operations in reverse order
        for operation in batch.operations.iter().rev() {
            self.undo_operation(operation, layout, components);
        }
        
        true
    }

    /// Redo the next batch of operations
    pub fn redo(&mut self, layout: &mut super::LayoutManager, components: &mut Vec<Box<dyn crate::rcl::ui::component::Component>>) -> bool {
        // End current batch before redoing
        self.end_batch();
        
        if self.current_index >= self.operations.len() {
            return false;
        }
        
        let batch = &self.operations[self.current_index].clone();
        self.current_index += 1;
        
        // Apply operations in forward order
        for operation in &batch.operations {
            self.redo_operation(operation, layout, components);
        }
        
        true
    }

    /// Undo a single operation
    fn undo_operation(
        &self,
        operation: &DesignOperation,
        layout: &mut super::LayoutManager,
        components: &mut Vec<Box<dyn crate::rcl::ui::component::Component>>,
    ) {
        match operation {
            DesignOperation::Move { component_ids, old_positions, .. } => {
                for (i, &component_id) in component_ids.iter().enumerate() {
                    if let Some(old_pos) = old_positions.get(i) {
                        layout.positions.insert(component_id, *old_pos);
                    }
                }
            }
            DesignOperation::Resize { component_id, old_size, old_position, .. } => {
                layout.sizes.insert(*component_id, *old_size);
                if let Some(old_pos) = old_position {
                    layout.positions.insert(*component_id, *old_pos);
                }
            }
            DesignOperation::Add { component_id, .. } => {
                // Remove the component
                if *component_id < components.len() {
                    components.remove(*component_id);
                    layout.positions.remove(component_id);
                    layout.sizes.remove(component_id);
                    // TODO: Shift other component indices
                }
            }
            DesignOperation::Delete { component_id, position, size, component_type, component_data } => {
                // Restore the component
                let restored_component = self.deserialize_component(component_type, component_data);
                if let Some(component) = restored_component {
                    components.insert(*component_id, component);
                    layout.positions.insert(*component_id, *position);
                    layout.sizes.insert(*component_id, *size);
                    // TODO: Shift other component indices
                }
            }
            DesignOperation::PropertyChange { component_id, property_name, old_value, .. } => {
                if let Some(component) = components.get_mut(*component_id) {
                    component.set_property(property_name, old_value);
                }
            }
            DesignOperation::Duplicate { new_ids, .. } => {
                // Remove the duplicated components
                let mut sorted_ids: Vec<usize> = new_ids.clone();
                sorted_ids.sort_by(|a, b| b.cmp(a)); // Reverse order to avoid index shifts
                
                for &id in &sorted_ids {
                    if id < components.len() {
                        components.remove(id);
                        layout.positions.remove(&id);
                        layout.sizes.remove(&id);
                    }
                }
            }
            // TODO: Implement other operation undos
            _ => {
                println!("Undo for operation {:?} not implemented", operation);
            }
        }
    }

    /// Redo a single operation
    fn redo_operation(
        &self,
        operation: &DesignOperation,
        layout: &mut super::LayoutManager,
        components: &mut Vec<Box<dyn crate::rcl::ui::component::Component>>,
    ) {
        match operation {
            DesignOperation::Move { component_ids, new_positions, .. } => {
                for (i, &component_id) in component_ids.iter().enumerate() {
                    if let Some(new_pos) = new_positions.get(i) {
                        layout.positions.insert(component_id, *new_pos);
                    }
                }
            }
            DesignOperation::Resize { component_id, new_size, new_position, .. } => {
                layout.sizes.insert(*component_id, *new_size);
                if let Some(new_pos) = new_position {
                    layout.positions.insert(*component_id, *new_pos);
                }
            }
            DesignOperation::Add { component_id, position, size, component_type, component_data } => {
                // Re-add the component
                let restored_component = self.deserialize_component(component_type, component_data);
                if let Some(component) = restored_component {
                    components.insert(*component_id, component);
                    layout.positions.insert(*component_id, *position);
                    layout.sizes.insert(*component_id, *size);
                }
            }
            DesignOperation::Delete { component_id, .. } => {
                // Re-delete the component
                if *component_id < components.len() {
                    components.remove(*component_id);
                    layout.positions.remove(component_id);
                    layout.sizes.remove(component_id);
                }
            }
            DesignOperation::PropertyChange { component_id, property_name, new_value, .. } => {
                if let Some(component) = components.get_mut(*component_id) {
                    component.set_property(property_name, new_value);
                }
            }
            DesignOperation::Duplicate { new_ids, positions, sizes, .. } => {
                // Re-create the duplicated components
                for (i, &new_id) in new_ids.iter().enumerate() {
                    if let (Some(pos), Some(size)) = (
                        positions.get(i),
                        sizes.get(i)
                    ) {
                        // This is simplified - create default button for now
                        let component = Box::new(crate::rcl::ui::basic::button::Button::new("Duplicate".to_string()));
                        components.insert(new_id, component);
                        layout.positions.insert(new_id, *pos);
                        layout.sizes.insert(new_id, *size);
                    }
                }
            }
            // TODO: Implement other operation redos
            _ => {
                println!("Redo for operation {:?} not implemented", operation);
            }
        }
    }

    /// Get a description for an operation
    fn operation_description(&self, operation: &DesignOperation) -> String {
        match operation {
            DesignOperation::Move { component_ids, .. } => {
                if component_ids.len() == 1 {
                    "Move Component".to_string()
                } else {
                    format!("Move {} Components", component_ids.len())
                }
            }
            DesignOperation::Resize { .. } => "Resize Component".to_string(),
            DesignOperation::Add { .. } => "Add Component".to_string(),
            DesignOperation::Delete { .. } => "Delete Component".to_string(),
            DesignOperation::PropertyChange { property_name, .. } => {
                format!("Change {}", property_name)
            }
            DesignOperation::Group { .. } => "Group Components".to_string(),
            DesignOperation::Ungroup { .. } => "Ungroup Components".to_string(),
            DesignOperation::LayerChange { .. } => "Change Layer".to_string(),
            DesignOperation::Duplicate { .. } => "Duplicate Components".to_string(),
            DesignOperation::Cut { .. } => "Cut Components".to_string(),
            DesignOperation::Paste { .. } => "Paste Components".to_string(),
        }
    }

    /// Deserialize a component from string data (simplified)
    fn deserialize_component(
        &self,
        component_type: &str,
        _component_data: &str,
    ) -> Option<Box<dyn crate::rcl::ui::component::Component>> {
        use crate::rcl::ui::component::Component;
        
        match component_type {
            "Button" => Some(Box::new(crate::rcl::ui::basic::button::Button::new("Button".to_string()))),
            "Label" => Some(Box::new(crate::rcl::ui::basic::label::Label::new("Label".to_string()))),
            "TextBox" => Some(Box::new(crate::rcl::ui::basic::textbox::TextBox::new("".to_string()))),
            "Checkbox" => Some(Box::new(crate::rcl::ui::basic::checkbox::Checkbox::new("Checkbox".to_string(), false))),
            "Slider" => Some(Box::new(crate::rcl::ui::basic::slider::Slider::new(0.0, 0.0, 100.0))),
            _ => None,
        }
    }

    /// Create a default component from serialized data (simplified)
    fn create_default_component_from_data(
        &self,
        data: &str,
    ) -> Option<Box<dyn crate::rcl::ui::component::Component>> {
        // This is a very simplified approach - would need proper serialization
        if data.contains("Button") {
            Some(Box::new(crate::rcl::ui::basic::button::Button::new("Button Copy".to_string())))
        } else if data.contains("Label") {
            Some(Box::new(crate::rcl::ui::basic::label::Label::new("Label Copy".to_string())))
        } else if data.contains("TextBox") {
            Some(Box::new(crate::rcl::ui::basic::textbox::TextBox::new("".to_string())))
        } else if data.contains("Checkbox") {
            Some(Box::new(crate::rcl::ui::basic::checkbox::Checkbox::new("Checkbox Copy".to_string(), false)))
        } else if data.contains("Slider") {
            Some(Box::new(crate::rcl::ui::basic::slider::Slider::new(0.0, 0.0, 100.0)))
        } else {
            Some(Box::new(crate::rcl::ui::basic::button::Button::new("Unknown".to_string())))
        }
    }

    /// Get the number of operations that can be undone
    pub fn undo_count(&self) -> usize {
        self.current_index
    }

    /// Get the number of operations that can be redone
    pub fn redo_count(&self) -> usize {
        self.operations.len() - self.current_index
    }

    /// Get a description of the next undo operation
    pub fn next_undo_description(&self) -> Option<&str> {
        if self.current_index > 0 {
            Some(&self.operations[self.current_index - 1].description)
        } else {
            None
        }
    }

    /// Get a description of the next redo operation
    pub fn next_redo_description(&self) -> Option<&str> {
        if self.current_index < self.operations.len() {
            Some(&self.operations[self.current_index].description)
        } else {
            None
        }
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.operations.clear();
        self.current_index = 0;
        self.current_batch = None;
    }
}
