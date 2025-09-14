//! Concrete Command Implementations for Visual Designer
//!
//! This module contains specific command implementations for all visual designer
//! operations including component manipulation, layout changes, and property edits.

use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use super::command_system::{Command, CommandContext, CommandResult, CommandMetadata};
use crate::rcl::ui::component::Component;
use crate::core::event_bus::IdeEvent;

/// Command to move one or more components
#[derive(Debug)]
pub struct MoveComponentCommand {
    id: Uuid,
    component_indices: Vec<usize>,
    old_positions: Vec<egui::Pos2>,
    new_positions: Vec<egui::Pos2>,
    metadata: CommandMetadata,
}

impl MoveComponentCommand {
    pub fn new(component_indices: Vec<usize>, old_positions: Vec<egui::Pos2>, new_positions: Vec<egui::Pos2>) -> Self {
        Self {
            id: Uuid::new_v4(),
            metadata: CommandMetadata {
                category: "Layout".to_string(),
                affected_components: component_indices.clone(),
                should_log: true,
                tags: vec!["movement".to_string(), "layout".to_string()],
                ..Default::default()
            },
            component_indices,
            old_positions,
            new_positions,
        }
    }
}

impl Command for MoveComponentCommand {
    fn execute(&mut self, context: &mut CommandContext) -> CommandResult {
        for (i, &component_idx) in self.component_indices.iter().enumerate() {
            if let Some(new_pos) = self.new_positions.get(i) {
                context.layout.positions.insert(component_idx, *new_pos);
            }
        }
        
        context.emit_event(IdeEvent::CommandExecuted {
            command_id: self.id,
            description: self.description(),
        });
        
        CommandResult::Success
    }
    
    fn undo(&mut self, context: &mut CommandContext) -> CommandResult {
        for (i, &component_idx) in self.component_indices.iter().enumerate() {
            if let Some(old_pos) = self.old_positions.get(i) {
                context.layout.positions.insert(component_idx, *old_pos);
            }
        }
        
        context.emit_event(IdeEvent::CommandUndone {
            command_id: self.id,
            description: self.description(),
        });
        
        CommandResult::Success
    }
    
    fn description(&self) -> String {
        if self.component_indices.len() == 1 {
            "Move Component".to_string()
        } else {
            format!("Move {} Components", self.component_indices.len())
        }
    }
    
    fn id(&self) -> Uuid {
        self.id
    }
    
    fn can_merge_with(&self, other: &dyn Command) -> bool {
        // Try to downcast to MoveComponentCommand
        if let Some(other_move) = other.as_any().downcast_ref::<MoveComponentCommand>() {
            // Can merge if affecting the same components
            self.component_indices == other_move.component_indices
        } else {
            false
        }
    }
    
    fn merge_with(&mut self, other: Box<dyn Command>) -> Result<(), Box<dyn Command>> {
        // Simplified: disable merging for now due to type system complexity
        Err(other)
    }
    
    fn is_valid(&self, context: &CommandContext) -> bool {
        self.component_indices.iter().all(|&idx| context.is_valid_component(idx))
    }
    
    fn metadata(&self) -> CommandMetadata {
        self.metadata.clone()
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}


/// Command to resize a component
#[derive(Debug)]
pub struct ResizeComponentCommand {
    id: Uuid,
    component_index: usize,
    old_size: egui::Vec2,
    new_size: egui::Vec2,
    old_position: Option<egui::Pos2>, // For resize operations that also change position
    new_position: Option<egui::Pos2>,
    metadata: CommandMetadata,
}

impl ResizeComponentCommand {
    pub fn new(
        component_index: usize, 
        old_size: egui::Vec2, 
        new_size: egui::Vec2,
        old_position: Option<egui::Pos2>,
        new_position: Option<egui::Pos2>
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            metadata: CommandMetadata {
                category: "Layout".to_string(),
                affected_components: vec![component_index],
                should_log: true,
                tags: vec!["resize".to_string(), "layout".to_string()],
                ..Default::default()
            },
            component_index,
            old_size,
            new_size,
            old_position,
            new_position,
        }
    }
}

impl Command for ResizeComponentCommand {
    fn execute(&mut self, context: &mut CommandContext) -> CommandResult {
        context.layout.sizes.insert(self.component_index, self.new_size);
        if let Some(new_pos) = self.new_position {
            context.layout.positions.insert(self.component_index, new_pos);
        }
        
        context.emit_event(IdeEvent::CommandExecuted {
            command_id: self.id,
            description: self.description(),
        });
        
        CommandResult::Success
    }
    
    fn undo(&mut self, context: &mut CommandContext) -> CommandResult {
        context.layout.sizes.insert(self.component_index, self.old_size);
        if let Some(old_pos) = self.old_position {
            context.layout.positions.insert(self.component_index, old_pos);
        }
        
        context.emit_event(IdeEvent::CommandUndone {
            command_id: self.id,
            description: self.description(),
        });
        
        CommandResult::Success
    }
    
    fn description(&self) -> String {
        "Resize Component".to_string()
    }
    
    fn id(&self) -> Uuid {
        self.id
    }
    
    fn can_merge_with(&self, other: &dyn Command) -> bool {
        if let Some(other_resize) = other.as_any().downcast_ref::<ResizeComponentCommand>() {
            self.component_index == other_resize.component_index
        } else {
            false
        }
    }
    
    fn merge_with(&mut self, other: Box<dyn Command>) -> Result<(), Box<dyn Command>> {
        // Simplified: disable merging for now due to type system complexity
        Err(other)
    }
    
    fn is_valid(&self, context: &CommandContext) -> bool {
        context.is_valid_component(self.component_index)
    }
    
    fn metadata(&self) -> CommandMetadata {
        self.metadata.clone()
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

/// Command to add a new component
#[derive(Debug)]
pub struct AddComponentCommand {
    id: Uuid,
    component_index: usize,
    position: egui::Pos2,
    size: egui::Vec2,
    component_data: ComponentData,
    added: bool, // Track if component has been added
    metadata: CommandMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentData {
    pub component_type: String,
    pub properties: HashMap<String, String>,
}

impl AddComponentCommand {
    pub fn new(
        component_index: usize,
        position: egui::Pos2,
        size: egui::Vec2,
        component_data: ComponentData,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            metadata: CommandMetadata {
                category: "Components".to_string(),
                affected_components: vec![component_index],
                should_log: true,
                tags: vec!["add".to_string(), "component".to_string()],
                ..Default::default()
            },
            component_index,
            position,
            size,
            component_data,
            added: false,
        }
    }
}

impl Command for AddComponentCommand {
    fn execute(&mut self, context: &mut CommandContext) -> CommandResult {
        if !self.added {
            // Create the component based on component_data
            let component = self.create_component_from_data();
            
            // Insert at the specified index
            if self.component_index <= context.components.len() {
                context.components.insert(self.component_index, component);
                context.layout.positions.insert(self.component_index, self.position);
                context.layout.sizes.insert(self.component_index, self.size);
                
                // Update indices of components after this one
                self.shift_indices_after_insertion(context);
                
                self.added = true;
                
                context.emit_event(IdeEvent::CommandExecuted {
                    command_id: self.id,
                    description: self.description(),
                });
                
                CommandResult::Success
            } else {
                CommandResult::Error("Invalid component index".to_string())
            }
        } else {
            CommandResult::Warning("Component already added".to_string())
        }
    }
    
    fn undo(&mut self, context: &mut CommandContext) -> CommandResult {
        if self.added && self.component_index < context.components.len() {
            context.components.remove(self.component_index);
            context.layout.positions.remove(&self.component_index);
            context.layout.sizes.remove(&self.component_index);
            
            // Update indices of components after this one
            self.shift_indices_after_removal(context);
            
            self.added = false;
            
            context.emit_event(IdeEvent::CommandUndone {
                command_id: self.id,
                description: self.description(),
            });
            
            CommandResult::Success
        } else {
            CommandResult::Warning("Component not found or not added".to_string())
        }
    }
    
    fn description(&self) -> String {
        format!("Add {}", self.component_data.component_type)
    }
    
    fn id(&self) -> Uuid {
        self.id
    }
    
    fn is_valid(&self, _context: &CommandContext) -> bool {
        // Add command is always valid as it creates new components
        true
    }
    
    fn metadata(&self) -> CommandMetadata {
        self.metadata.clone()
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

impl AddComponentCommand {
    fn create_component_from_data(&self) -> Box<dyn Component> {
        match self.component_data.component_type.as_str() {
            "Button" => {
                let text = self.component_data.properties
                    .get("text")
                    .cloned()
                    .unwrap_or_else(|| "Button".to_string());
                Box::new(crate::rcl::ui::basic::button::Button::new(text))
            }
            "Label" => {
                let text = self.component_data.properties
                    .get("text")
                    .cloned()
                    .unwrap_or_else(|| "Label".to_string());
                Box::new(crate::rcl::ui::basic::label::Label::new(text))
            }
            "TextBox" => {
                let text = self.component_data.properties
                    .get("text")
                    .cloned()
                    .unwrap_or_default();
                Box::new(crate::rcl::ui::basic::textbox::TextBox::new(text))
            }
            "Checkbox" => {
                let text = self.component_data.properties
                    .get("text")
                    .cloned()
                    .unwrap_or_else(|| "Checkbox".to_string());
                let checked = self.component_data.properties
                    .get("checked")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(false);
                Box::new(crate::rcl::ui::basic::checkbox::Checkbox::new(text, checked))
            }
            "Slider" => {
                let value = self.component_data.properties
                    .get("value")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
                let min = self.component_data.properties
                    .get("min")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
                let max = self.component_data.properties
                    .get("max")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(100.0);
                Box::new(crate::rcl::ui::basic::slider::Slider::new(value, min, max))
            }
            _ => {
                // Default to button for unknown types
                Box::new(crate::rcl::ui::basic::button::Button::new("Unknown".to_string()))
            }
        }
    }
    
    fn shift_indices_after_insertion(&self, context: &mut CommandContext) {
        // Update layout data for components with indices >= component_index
        let mut new_positions = HashMap::new();
        let mut new_sizes = HashMap::new();
        
        for (&idx, &pos) in &context.layout.positions {
            if idx > self.component_index {
                new_positions.insert(idx + 1, pos);
            } else {
                new_positions.insert(idx, pos);
            }
        }
        
        for (&idx, &size) in &context.layout.sizes {
            if idx > self.component_index {
                new_sizes.insert(idx + 1, size);
            } else {
                new_sizes.insert(idx, size);
            }
        }
        
        context.layout.positions = new_positions;
        context.layout.sizes = new_sizes;
        
        // Update selection indices
        let mut new_selection = std::collections::HashSet::new();
        for &idx in &context.selection {
            if idx > self.component_index {
                new_selection.insert(idx + 1);
            } else {
                new_selection.insert(idx);
            }
        }
        context.selection = new_selection;
        
        // Update primary selection
        if let Some(primary) = context.primary_selection {
            if primary > self.component_index {
                context.primary_selection = Some(primary + 1);
            }
        }
    }
    
    fn shift_indices_after_removal(&self, context: &mut CommandContext) {
        // Update layout data for components with indices > component_index
        let mut new_positions = HashMap::new();
        let mut new_sizes = HashMap::new();
        
        for (&idx, &pos) in &context.layout.positions {
            if idx > self.component_index {
                new_positions.insert(idx - 1, pos);
            } else if idx < self.component_index {
                new_positions.insert(idx, pos);
            }
        }
        
        for (&idx, &size) in &context.layout.sizes {
            if idx > self.component_index {
                new_sizes.insert(idx - 1, size);
            } else if idx < self.component_index {
                new_sizes.insert(idx, size);
            }
        }
        
        context.layout.positions = new_positions;
        context.layout.sizes = new_sizes;
        
        // Update selection indices
        let mut new_selection = std::collections::HashSet::new();
        for &idx in &context.selection {
            if idx > self.component_index {
                new_selection.insert(idx - 1);
            } else if idx < self.component_index {
                new_selection.insert(idx);
            }
        }
        context.selection = new_selection;
        
        // Update primary selection
        if let Some(primary) = context.primary_selection {
            if primary > self.component_index {
                context.primary_selection = Some(primary - 1);
            } else if primary == self.component_index {
                context.primary_selection = None;
            }
        }
    }
}

/// Command to delete one or more components
#[derive(Debug)]
pub struct DeleteComponentCommand {
    id: Uuid,
    component_data: Vec<DeletedComponentData>,
    metadata: CommandMetadata,
}

#[derive(Debug)]
struct DeletedComponentData {
    index: usize,
    position: egui::Pos2,
    size: egui::Vec2,
    component_data: ComponentData,
    // Store the actual component for restoration
    component: Option<Box<dyn Component>>,
}

impl DeleteComponentCommand {
    pub fn new(indices: Vec<usize>) -> Self {
        Self {
            id: Uuid::new_v4(),
            metadata: CommandMetadata {
                category: "Components".to_string(),
                affected_components: indices.clone(),
                should_log: true,
                tags: vec!["delete".to_string(), "component".to_string()],
                ..Default::default()
            },
            component_data: indices.into_iter().map(|idx| DeletedComponentData {
                index: idx,
                position: egui::Pos2::ZERO,
                size: egui::Vec2::ZERO,
                component_data: ComponentData {
                    component_type: "Unknown".to_string(),
                    properties: HashMap::new(),
                },
                component: None,
            }).collect(),
        }
    }
}

impl Command for DeleteComponentCommand {
    fn execute(&mut self, context: &mut CommandContext) -> CommandResult {
        // Sort indices in descending order to avoid index shifting issues
        self.component_data.sort_by(|a, b| b.index.cmp(&a.index));
        
        for data in &mut self.component_data {
            if data.index < context.components.len() {
                // Store component data before deletion
                if let Some(pos) = context.layout.positions.get(&data.index) {
                    data.position = *pos;
                }
                if let Some(size) = context.layout.sizes.get(&data.index) {
                    data.size = *size;
                }
                
                // Remove the component and store it for potential restoration
                let component = context.components.remove(data.index);
                data.component_data.component_type = component.name().to_string();
                data.component = Some(component);
                
                // Remove from layout data
                context.layout.positions.remove(&data.index);
                context.layout.sizes.remove(&data.index);
                
                // Update selection
                context.selection.remove(&data.index);
                if context.primary_selection == Some(data.index) {
                    context.primary_selection = None;
                }
            }
        }
        
        context.emit_event(IdeEvent::CommandExecuted {
            command_id: self.id,
            description: self.description(),
        });
        
        CommandResult::Success
    }
    
    fn undo(&mut self, context: &mut CommandContext) -> CommandResult {
        // Sort indices in ascending order for restoration
        self.component_data.sort_by(|a, b| a.index.cmp(&b.index));
        
        for data in &mut self.component_data {
            if let Some(component) = data.component.take() {
                // Insert component back
                context.components.insert(data.index, component);
                context.layout.positions.insert(data.index, data.position);
                context.layout.sizes.insert(data.index, data.size);
                
                // Restore to selection if it was the primary
                context.selection.insert(data.index);
                if context.primary_selection.is_none() {
                    context.primary_selection = Some(data.index);
                }
            }
        }
        
        context.emit_event(IdeEvent::CommandUndone {
            command_id: self.id,
            description: self.description(),
        });
        
        CommandResult::Success
    }
    
    fn description(&self) -> String {
        if self.component_data.len() == 1 {
            "Delete Component".to_string()
        } else {
            format!("Delete {} Components", self.component_data.len())
        }
    }
    
    fn id(&self) -> Uuid {
        self.id
    }
    
    fn is_valid(&self, context: &CommandContext) -> bool {
        self.component_data.iter().all(|data| context.is_valid_component(data.index))
    }
    
    fn metadata(&self) -> CommandMetadata {
        self.metadata.clone()
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

/// Command to change a component property
#[derive(Debug)]
pub struct PropertyChangeCommand {
    id: Uuid,
    component_index: usize,
    property_name: String,
    old_value: String,
    new_value: String,
    metadata: CommandMetadata,
}

impl PropertyChangeCommand {
    pub fn new(
        component_index: usize,
        property_name: String,
        old_value: String,
        new_value: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            metadata: CommandMetadata {
                category: "Properties".to_string(),
                affected_components: vec![component_index],
                should_log: true,
                tags: vec!["property".to_string(), "change".to_string()],
                ..Default::default()
            },
            component_index,
            property_name,
            old_value,
            new_value,
        }
    }
}

impl Command for PropertyChangeCommand {
    fn execute(&mut self, context: &mut CommandContext) -> CommandResult {
        if let Some(component) = context.get_component_mut(self.component_index) {
            component.set_property(&self.property_name, &self.new_value);
            
            context.emit_event(IdeEvent::CommandExecuted {
                command_id: self.id,
                description: self.description(),
            });
            
            CommandResult::Success
        } else {
            CommandResult::Error("Component not found".to_string())
        }
    }
    
    fn undo(&mut self, context: &mut CommandContext) -> CommandResult {
        if let Some(component) = context.get_component_mut(self.component_index) {
            component.set_property(&self.property_name, &self.old_value);
            
            context.emit_event(IdeEvent::CommandUndone {
                command_id: self.id,
                description: self.description(),
            });
            
            CommandResult::Success
        } else {
            CommandResult::Error("Component not found".to_string())
        }
    }
    
    fn description(&self) -> String {
        format!("Change {} Property", self.property_name)
    }
    
    fn id(&self) -> Uuid {
        self.id
    }
    
    fn can_merge_with(&self, other: &dyn Command) -> bool {
        if let Some(other_prop) = other.as_any().downcast_ref::<PropertyChangeCommand>() {
            self.component_index == other_prop.component_index 
                && self.property_name == other_prop.property_name
        } else {
            false
        }
    }
    
    fn merge_with(&mut self, other: Box<dyn Command>) -> Result<(), Box<dyn Command>> {
        // Simplified: disable merging for now due to type system complexity
        Err(other)
    }
    
    fn is_valid(&self, context: &CommandContext) -> bool {
        context.is_valid_component(self.component_index)
    }
    
    fn metadata(&self) -> CommandMetadata {
        self.metadata.clone()
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rcl::ui::basic::button::Button;
    use super::super::layout::LayoutManager;
    
    fn create_test_context() -> CommandContext {
        let layout = LayoutManager::new();
        let components: Vec<Box<dyn Component>> = vec![
            Box::new(Button::new("Test Button".to_string())),
        ];
        let selection = std::collections::HashSet::new();
        
        CommandContext::new(layout, components, selection, None)
    }
    
    #[test]
    fn test_move_command() {
        let mut context = create_test_context();
        let old_pos = egui::Pos2::new(10.0, 10.0);
        let new_pos = egui::Pos2::new(20.0, 20.0);
        
        context.layout.positions.insert(0, old_pos);
        
        let mut cmd = MoveComponentCommand::new(
            vec![0],
            vec![old_pos],
            vec![new_pos]
        );
        
        // Execute
        let result = cmd.execute(&mut context);
        assert!(matches!(result, CommandResult::Success));
        assert_eq!(context.layout.positions.get(&0), Some(&new_pos));
        
        // Undo
        let result = cmd.undo(&mut context);
        assert!(matches!(result, CommandResult::Success));
        assert_eq!(context.layout.positions.get(&0), Some(&old_pos));
    }
    
    #[test]
    fn test_add_component_command() {
        let mut context = create_test_context();
        let initial_count = context.components.len();
        
        let component_data = ComponentData {
            component_type: "Button".to_string(),
            properties: {
                let mut props = HashMap::new();
                props.insert("text".to_string(), "New Button".to_string());
                props
            }
        };
        
        let mut cmd = AddComponentCommand::new(
            1, // Insert at index 1
            egui::Pos2::new(50.0, 50.0),
            egui::Vec2::new(100.0, 30.0),
            component_data
        );
        
        // Execute
        let result = cmd.execute(&mut context);
        assert!(matches!(result, CommandResult::Success));
        assert_eq!(context.components.len(), initial_count + 1);
        
        // Undo
        let result = cmd.undo(&mut context);
        assert!(matches!(result, CommandResult::Success));
        assert_eq!(context.components.len(), initial_count);
    }
    
    #[test]
    fn test_command_merging() {
        let old_pos = egui::Pos2::new(10.0, 10.0);
        let pos1 = egui::Pos2::new(20.0, 20.0);
        let pos2 = egui::Pos2::new(30.0, 30.0);
        
        let mut cmd1 = MoveComponentCommand::new(
            vec![0],
            vec![old_pos],
            vec![pos1]
        );
        
        let cmd2 = MoveComponentCommand::new(
            vec![0],
            vec![pos1],
            vec![pos2]
        );
        
        // Test that commands can merge
        assert!(cmd1.can_merge_with(&cmd2));
        
        // Perform merge
        let result = cmd1.merge_with(Box::new(cmd2));
        assert!(result.is_ok());
        
        // Check that new positions are from cmd2
        assert_eq!(cmd1.new_positions[0], pos2);
        assert_eq!(cmd1.old_positions[0], old_pos); // Should keep original old position
    }
}