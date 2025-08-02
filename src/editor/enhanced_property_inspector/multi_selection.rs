//! Multi-Selection Property Editing
//!
//! This module handles editing properties across multiple selected components,
//! including common property detection and batch editing capabilities.

use egui::*;
use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
use crate::editor::inspector::{PropertyValue, PropertyType};

/// Multi-selection property editing system
pub struct MultiSelectionManager {
    /// Selected components
    pub selected_components: Vec<ComponentSelection>,
    /// Common properties across selection
    pub common_properties: Vec<CommonProperty>,
    /// Mixed value indicators
    pub mixed_values: HashSet<String>,
    /// Batch edit mode
    pub batch_edit_enabled: bool,
    /// Selection history for undo/redo
    pub selection_history: Vec<SelectionState>,
    /// Current history index
    pub history_index: usize,
}

/// Component selection info
#[derive(Clone, Debug)]
pub struct ComponentSelection {
    pub component_id: usize,
    pub component_type: String,
    pub properties: HashMap<String, PropertyValue>,
}

/// Common property across multiple components
#[derive(Clone, Debug)]
pub struct CommonProperty {
    pub name: String,
    pub property_type: PropertyType,
    /// Values from all selected components
    pub values: Vec<PropertyValue>,
    /// Whether all components have the same value
    pub is_consistent: bool,
    /// The common value if consistent, or a representative value
    pub representative_value: PropertyValue,
    /// Components that support this property
    pub supporting_components: Vec<usize>,
}

/// Selection state for history
#[derive(Clone, Debug)]
pub struct SelectionState {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub selected_components: Vec<ComponentSelection>,
    pub operation: SelectionOperation,
}

/// Type of selection operation
#[derive(Clone, Debug)]
pub enum SelectionOperation {
    Added(Vec<usize>),
    Removed(Vec<usize>),
    Cleared,
    PropertyChanged { property: String, old_values: Vec<PropertyValue>, new_values: Vec<PropertyValue> },
}

/// Multi-selection editing result
#[derive(Debug, Clone)]
pub struct MultiEditResult {
    /// Number of components affected
    pub affected_count: usize,
    /// Properties that were changed
    pub changed_properties: Vec<String>,
    /// Whether the operation was successful
    pub success: bool,
    /// Error message if operation failed
    pub error: Option<String>,
}

impl Default for MultiSelectionManager {
    fn default() -> Self {
        Self {
            selected_components: Vec::new(),
            common_properties: Vec::new(),
            mixed_values: HashSet::new(),
            batch_edit_enabled: true,
            selection_history: Vec::new(),
            history_index: 0,
        }
    }
}

impl MultiSelectionManager {
    /// Create a new multi-selection manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add components to selection
    pub fn add_to_selection(&mut self, components: Vec<ComponentSelection>) {
        let added_ids: Vec<usize> = components.iter().map(|c| c.component_id).collect();
        
        // Add components that aren't already selected
        for component in components {
            if !self.selected_components.iter().any(|c| c.component_id == component.component_id) {
                self.selected_components.push(component);
            }
        }
        
        // Record operation in history
        self.add_to_history(SelectionOperation::Added(added_ids));
        
        // Update common properties
        self.update_common_properties();
    }
    
    /// Remove components from selection
    pub fn remove_from_selection(&mut self, component_ids: &[usize]) {
        let removed_ids = component_ids.to_vec();
        
        self.selected_components.retain(|c| !component_ids.contains(&c.component_id));
        
        // Record operation in history
        self.add_to_history(SelectionOperation::Removed(removed_ids));
        
        // Update common properties
        self.update_common_properties();
    }
    
    /// Clear all selection
    pub fn clear_selection(&mut self) {
        if !self.selected_components.is_empty() {
            self.selected_components.clear();
            self.common_properties.clear();
            self.mixed_values.clear();
            
            // Record operation in history
            self.add_to_history(SelectionOperation::Cleared);
        }
    }
    
    /// Set property value for all selected components
    pub fn set_property_for_all(&mut self, property_name: &str, value: PropertyValue) -> MultiEditResult {
        if self.selected_components.is_empty() {
            return MultiEditResult {
                affected_count: 0,
                changed_properties: vec![],
                success: false,
                error: Some("No components selected".to_string()),
            };
        }
        
        let mut old_values = Vec::new();
        let mut new_values = Vec::new();
        let mut affected_count = 0;
        
        // Apply the property change to all selected components
        for component in &mut self.selected_components {
            if let Some(old_value) = component.properties.get(property_name) {
                old_values.push(old_value.clone());
                component.properties.insert(property_name.to_string(), value.clone());
                new_values.push(value.clone());
                affected_count += 1;
            }
        }
        
        if affected_count > 0 {
            // Record property change in history
            self.add_to_history(SelectionOperation::PropertyChanged {
                property: property_name.to_string(),
                old_values,
                new_values,
            });
            
            // Update common properties
            self.update_common_properties();
            
            MultiEditResult {
                affected_count,
                changed_properties: vec![property_name.to_string()],
                success: true,
                error: None,
            }
        } else {
            MultiEditResult {
                affected_count: 0,
                changed_properties: vec![],
                success: false,
                error: Some(format!("Property '{}' not found in selected components", property_name)),
            }
        }
    }
    
    /// Set multiple properties for all selected components
    pub fn set_properties_for_all(&mut self, properties: HashMap<String, PropertyValue>) -> MultiEditResult {
        if self.selected_components.is_empty() {
            return MultiEditResult {
                affected_count: 0,
                changed_properties: vec![],
                success: false,
                error: Some("No components selected".to_string()),
            };
        }
        
        let mut affected_count = 0;
        let mut changed_properties = Vec::new();
        
        for (property_name, value) in &properties {
            let result = self.set_property_for_all(property_name, value.clone());
            if result.success {
                affected_count = result.affected_count;
                changed_properties.extend(result.changed_properties);
            }
        }
        
        MultiEditResult {
            affected_count,
            changed_properties,
            success: !changed_properties.is_empty(),
            error: if changed_properties.is_empty() { 
                Some("No properties could be set".to_string()) 
            } else { 
                None 
            },
        }
    }
    
    /// Update common properties analysis
    fn update_common_properties(&mut self) {
        self.common_properties.clear();
        self.mixed_values.clear();
        
        if self.selected_components.is_empty() {
            return;
        }
        
        // Find all unique property names across selected components
        let mut all_property_names: HashSet<String> = HashSet::new();
        for component in &self.selected_components {
            all_property_names.extend(component.properties.keys().cloned());
        }
        
        // Analyze each property
        for property_name in all_property_names {
            let mut values = Vec::new();
            let mut supporting_components = Vec::new();
            
            // Collect values from all components that have this property
            for component in &self.selected_components {
                if let Some(value) = component.properties.get(&property_name) {
                    values.push(value.clone());
                    supporting_components.push(component.component_id);
                }
            }
            
            if values.is_empty() {
                continue;
            }
            
            // Check if all values are the same
            let first_value = &values[0];
            let is_consistent = values.iter().all(|v| self.are_values_equal(v, first_value));
            
            if !is_consistent {
                self.mixed_values.insert(property_name.clone());
            }
            
            // Determine representative value
            let representative_value = if is_consistent {
                first_value.clone()
            } else {
                // For mixed values, use the first value or a default
                self.get_mixed_value_representative(&values)
            };
            
            // Infer property type from the first value
            let property_type = self.infer_property_type(&representative_value);
            
            self.common_properties.push(CommonProperty {
                name: property_name,
                property_type,
                values,
                is_consistent,
                representative_value,
                supporting_components,
            });
        }
        
        // Sort common properties by name for consistent UI
        self.common_properties.sort_by(|a, b| a.name.cmp(&b.name));
    }
    
    /// Check if two property values are equal
    fn are_values_equal(&self, a: &PropertyValue, b: &PropertyValue) -> bool {
        match (a, b) {
            (PropertyValue::String(a), PropertyValue::String(b)) => a == b,
            (PropertyValue::Number(a), PropertyValue::Number(b)) => (a - b).abs() < f32::EPSILON,
            (PropertyValue::Boolean(a), PropertyValue::Boolean(b)) => a == b,
            (PropertyValue::Color(a), PropertyValue::Color(b)) => a == b,
            (PropertyValue::Enum(a), PropertyValue::Enum(b)) => a == b,
            _ => false,
        }
    }
    
    /// Get a representative value for mixed values
    fn get_mixed_value_representative(&self, values: &[PropertyValue]) -> PropertyValue {
        // For mixed values, we'll use the most common value, or the first one
        let mut value_counts: HashMap<String, usize> = HashMap::new();
        
        for value in values {
            let value_str = format!("{:?}", value); // Simple serialization
            *value_counts.entry(value_str).or_default() += 1;
        }
        
        // Find the most common value
        if let Some((_, most_common_index)) = value_counts
            .iter()
            .enumerate()
            .max_by_key(|(_, count)| *count) {
            
            values.get(most_common_index).cloned().unwrap_or_else(|| values[0].clone())
        } else {
            values[0].clone()
        }
    }
    
    /// Infer property type from a value
    fn infer_property_type(&self, value: &PropertyValue) -> PropertyType {
        match value {
            PropertyValue::String(_) => PropertyType::String { max_length: None },
            PropertyValue::Number(_) => PropertyType::Number { min: f32::NEG_INFINITY, max: f32::INFINITY, step: 1.0 },
            PropertyValue::Boolean(_) => PropertyType::Boolean,
            PropertyValue::Color(_) => PropertyType::Color,
            PropertyValue::Enum(_) => PropertyType::Enum { options: vec![] },
        }
    }
    
    /// Add operation to history
    fn add_to_history(&mut self, operation: SelectionOperation) {
        // Remove any operations after current index (when redoing after undo)
        self.selection_history.truncate(self.history_index);
        
        self.selection_history.push(SelectionState {
            timestamp: chrono::Utc::now(),
            selected_components: self.selected_components.clone(),
            operation,
        });
        
        self.history_index = self.selection_history.len();
        
        // Limit history size
        const MAX_HISTORY: usize = 100;
        if self.selection_history.len() > MAX_HISTORY {
            self.selection_history.remove(0);
            self.history_index = self.selection_history.len();
        }
    }
    
    /// Undo last operation
    pub fn undo(&mut self) -> bool {
        if self.history_index > 0 {
            self.history_index -= 1;
            if let Some(state) = self.selection_history.get(self.history_index) {
                self.selected_components = state.selected_components.clone();
                self.update_common_properties();
                return true;
            }
        }
        false
    }
    
    /// Redo last undone operation
    pub fn redo(&mut self) -> bool {
        if self.history_index < self.selection_history.len() {
            if let Some(state) = self.selection_history.get(self.history_index) {
                self.selected_components = state.selected_components.clone();
                self.history_index += 1;
                self.update_common_properties();
                return true;
            }
        }
        false
    }
    
    /// Get common property by name
    pub fn get_common_property(&self, name: &str) -> Option<&CommonProperty> {
        self.common_properties.iter().find(|p| p.name == name)
    }
    
    /// Check if a property has mixed values
    pub fn has_mixed_values(&self, property_name: &str) -> bool {
        self.mixed_values.contains(property_name)
    }
    
    /// Get selection statistics
    pub fn get_statistics(&self) -> SelectionStatistics {
        let component_types: HashSet<String> = self.selected_components
            .iter()
            .map(|c| c.component_type.clone())
            .collect();
        
        SelectionStatistics {
            selected_count: self.selected_components.len(),
            component_types: component_types.len(),
            common_properties: self.common_properties.len(),
            mixed_properties: self.mixed_values.len(),
            history_depth: self.selection_history.len(),
        }
    }
    
    /// Render multi-selection UI
    pub fn render_ui(&mut self, ui: &mut Ui) {
        ui.heading("Multi-Selection Editor");
        
        if self.selected_components.is_empty() {
            ui.label("No components selected");
            return;
        }
        
        // Selection summary
        let stats = self.get_statistics();
        ui.horizontal(|ui| {
            ui.label(format!("{} components selected", stats.selected_count));
            ui.label(format!("({} types)", stats.component_types));
        });
        
        ui.separator();
        
        // Batch edit toggle
        ui.checkbox(&mut self.batch_edit_enabled, "Enable batch editing");
        
        ui.separator();
        
        // Common properties
        ui.heading("Common Properties");
        
        if self.common_properties.is_empty() {
            ui.label("No common properties found");
            return;
        }
        
        egui::ScrollArea::vertical().show(ui, |ui| {
            for common_property in &self.common_properties {
                self.render_common_property_editor(ui, common_property);
            }
        });
        
        ui.separator();
        
        // Bulk operations
        ui.horizontal(|ui| {
            if ui.button("Reset All").clicked() {
                // Reset all properties to default values
            }
            
            if ui.button("Copy Properties").clicked() {
                // Copy properties from first selected component
            }
            
            if ui.button("Paste Properties").clicked() {
                // Paste properties to all selected components
            }
        });
    }
    
    /// Render editor for a common property
    fn render_common_property_editor(&mut self, ui: &mut Ui, property: &CommonProperty) {
        ui.horizontal(|ui| {
            // Property name
            ui.label(&property.name);
            
            // Mixed value indicator
            if !property.is_consistent {
                ui.colored_label(Color32::YELLOW, "Mixed");
            }
            
            // Value editor
            match &property.property_type {
                PropertyType::String { .. } => {
                    if let PropertyValue::String(value) = &property.representative_value {
                        let mut text = value.clone();
                        if ui.text_edit_singleline(&mut text).changed() && self.batch_edit_enabled {
                            let _ = self.set_property_for_all(&property.name, PropertyValue::String(text));
                        }
                    }
                }
                PropertyType::Number { min, max, step } => {
                    if let PropertyValue::Number(value) = &property.representative_value {
                        let mut number = *value;
                        if ui.add(egui::Slider::new(&mut number, *min..=*max).step_by(*step as f64)).changed() && self.batch_edit_enabled {
                            let _ = self.set_property_for_all(&property.name, PropertyValue::Number(number));
                        }
                    }
                }
                PropertyType::Boolean => {
                    if let PropertyValue::Boolean(value) = &property.representative_value {
                        let mut boolean = *value;
                        if ui.checkbox(&mut boolean, "").changed() && self.batch_edit_enabled {
                            let _ = self.set_property_for_all(&property.name, PropertyValue::Boolean(boolean));
                        }
                    }
                }
                PropertyType::Color => {
                    if let PropertyValue::Color(color) = &property.representative_value {
                        let mut color32 = Color32::from_rgba_premultiplied(color[0], color[1], color[2], color[3]);
                        if ui.color_edit_button_srgba(&mut color32).changed() && self.batch_edit_enabled {
                            let color_array = [color32.r(), color32.g(), color32.b(), color32.a()];
                            let _ = self.set_property_for_all(&property.name, PropertyValue::Color(color_array));
                        }
                    }
                }
                PropertyType::Enum { options } => {
                    if let PropertyValue::Enum(value) = &property.representative_value {
                        egui::ComboBox::from_id_source(&property.name)
                            .selected_text(value)
                            .show_ui(ui, |ui| {
                                for option in options {
                                    if ui.selectable_label(option == value, option).clicked() && self.batch_edit_enabled {
                                        let _ = self.set_property_for_all(&property.name, PropertyValue::Enum(option.clone()));
                                    }
                                }
                            });
                    }
                }
            }
            
            // Component count for this property
            ui.label(format!("({}/{})", property.supporting_components.len(), self.selected_components.len()));
        });
    }
}

/// Statistics for multi-selection
#[derive(Debug, Clone)]
pub struct SelectionStatistics {
    pub selected_count: usize,
    pub component_types: usize,
    pub common_properties: usize,
    pub mixed_properties: usize,
    pub history_depth: usize,
}