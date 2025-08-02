//! # Enhanced Property Inspector for IDE Integration
//!
//! This module provides a comprehensive property inspector that integrates with
//! the enhanced RCL property system, offering categorized properties, validation,
//! theming support, and advanced IDE features.

use egui::*;
use std::collections::HashMap;
use crate::rcl::ui::{
    Component, EnhancedComponent, StandardProperties,
    PropertyInfo, PropertyValue, PropertyCategory, PropertyRegistry,
    Theme, ThemeManager
};
use crate::rcl::ui::basic::EnhancedButton;

/// Enhanced property inspector for IDE integration
pub struct EnhancedPropertyInspector {
    /// Current theme for styling
    pub theme: Theme,
    /// Search filter for properties
    pub search_filter: String,
    /// Expanded property categories
    pub expanded_categories: HashMap<PropertyCategory, bool>,
    /// Show advanced properties
    pub show_advanced: bool,
    /// Live preview mode (updates in real-time)
    pub live_preview: bool,
    /// Validation errors for properties
    pub validation_errors: HashMap<String, String>,
    /// Multi-selection support
    pub multi_selection_mode: bool,
    /// Property edit history for undo/redo
    pub edit_history: Vec<PropertyEdit>,
    /// Current history position
    pub history_position: usize,
    /// Theme manager for theme switching
    pub theme_manager: ThemeManager,
}

/// Property edit for undo/redo support
#[derive(Debug, Clone)]
pub struct PropertyEdit {
    pub component_indices: Vec<usize>,
    pub property_name: String,
    pub old_values: Vec<PropertyValue>,
    pub new_values: Vec<PropertyValue>,
    pub timestamp: std::time::Instant,
}

impl EnhancedPropertyInspector {
    /// Create a new enhanced property inspector
    pub fn new() -> Self {
        let mut expanded_categories = HashMap::new();
        // Expand Content and Layout by default
        expanded_categories.insert(PropertyCategory::Content, true);
        expanded_categories.insert(PropertyCategory::Layout, true);
        expanded_categories.insert(PropertyCategory::Style, false);
        expanded_categories.insert(PropertyCategory::Behavior, false);
        expanded_categories.insert(PropertyCategory::Accessibility, false);

        Self {
            theme: Theme::light_theme(),
            search_filter: String::new(),
            expanded_categories,
            show_advanced: false,
            live_preview: true,
            validation_errors: HashMap::new(),
            multi_selection_mode: false,
            edit_history: Vec::new(),
            history_position: 0,
            theme_manager: ThemeManager::new(),
        }
    }

    /// Render the property inspector UI
    pub fn render_ui(&mut self, ui: &mut Ui, selected_components: &mut [Box<dyn Component>]) {
        if selected_components.is_empty() {
            self.render_no_selection(ui);
            return;
        }

        // Header with component info and controls
        self.render_header(ui, selected_components);
        
        ui.separator();
        
        // Search and filter controls
        self.render_search_controls(ui);
        
        ui.separator();
        
        // Property categories
        if selected_components.len() == 1 {
            self.render_single_component_properties(ui, &mut selected_components[0]);
        } else {
            self.render_multi_component_properties(ui, selected_components);
        }
    }

    /// Render when no components are selected
    fn render_no_selection(&self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            
            // Icon and message
            ui.label(RichText::new("📋").size(48.0));
            ui.add_space(10.0);
            ui.label(RichText::new("No Selection").size(18.0).strong());
            ui.label("Select one or more components to edit their properties");
            
            ui.add_space(20.0);
            
            // Tips
            ui.group(|ui| {
                ui.label(RichText::new("💡 Tips:").strong());
                ui.label("• Click a component to select it");
                ui.label("• Hold Ctrl to select multiple components");
                ui.label("• Use the hierarchy panel for precise selection");
            });
        });
    }

    /// Render the header with component information
    fn render_header(&mut self, ui: &mut Ui, components: &[Box<dyn Component>]) {
        ui.horizontal(|ui| {
            // Component info
            if components.len() == 1 {
                ui.label(RichText::new(format!("📦 {}", components[0].name())).strong());
            } else {
                ui.label(RichText::new(format!("📦 {} Components Selected", components.len())).strong());
            }
            
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                // Settings menu
                if ui.button("⚙").on_hover_text("Inspector Settings").clicked() {
                    // Toggle settings menu
                }
                
                // Theme selector
                ComboBox::from_id_source("theme_selector")
                    .selected_text(&self.theme.name)
                    .show_ui(ui, |ui| {
                        for theme_name in self.theme_manager.get_theme_names() {
                            if ui.selectable_value(&mut self.theme.name, theme_name.clone(), &theme_name).clicked() {
                                if let Some(theme) = self.theme_manager.get_theme(&theme_name) {
                                    self.theme = theme.clone();
                                }
                            }
                        }
                    });
            });
        });
        
        // Multi-selection controls
        if components.len() > 1 {
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.multi_selection_mode, "Multi-edit mode");
                ui.label(format!("Editing {} components", components.len()));
                
                if ui.button("🔄 Reset All").on_hover_text("Reset all selected components to default values").clicked() {
                    // TODO: Implement reset functionality
                }
            });
        }
    }

    /// Render search and filter controls
    fn render_search_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("🔍");
            ui.text_edit_singleline(&mut self.search_filter);
            if ui.button("✖").on_hover_text("Clear search").clicked() {
                self.search_filter.clear();
            }
            
            ui.separator();
            
            ui.checkbox(&mut self.show_advanced, "Advanced");
            ui.checkbox(&mut self.live_preview, "Live Preview");
        });
    }

    /// Render properties for a single component
    fn render_single_component_properties(&mut self, ui: &mut Ui, component: &mut Box<dyn Component>) {
        // Try to cast to EnhancedComponent for better property support
        if let Some(enhanced) = self.try_get_enhanced_component(component) {
            self.render_enhanced_component_properties(ui, enhanced);
        } else {
            self.render_basic_component_properties(ui, component);
        }
    }

    /// Render properties for multiple components
    fn render_multi_component_properties(&mut self, ui: &mut Ui, components: &mut [Box<dyn Component>]) {
        ui.label("Multi-component editing");
        
        // Find common properties
        let common_properties = self.find_common_properties(components);
        
        if common_properties.is_empty() {
            ui.label("Selected components have no common properties");
            return;
        }
        
        ui.label(format!("Editing {} common properties", common_properties.len()));
        
        // Render common properties
        for property_name in common_properties {
            self.render_multi_component_property(ui, components, &property_name);
        }
    }

    /// Render properties for an enhanced component
    fn render_enhanced_component_properties(&mut self, ui: &mut Ui, component: &mut dyn EnhancedComponent) {
        let registry = component.property_registry();
        
        // Group properties by category
        let mut categories: HashMap<PropertyCategory, Vec<PropertyInfo>> = HashMap::new();
        for property_name in registry.get_names() {
            if let Some(property) = registry.get(&property_name) {
                if self.should_show_property(property) {
                    categories.entry(property.category.clone()).or_default().push(property.clone());
                }
            }
        }
        
        // Render each category
        for (category, properties) in categories {
            self.render_property_category(ui, &category, &properties, component);
        }
    }

    /// Render properties for a basic component
    fn render_basic_component_properties(&mut self, ui: &mut Ui, component: &mut Box<dyn Component>) {
        ui.group(|ui| {
            ui.label(RichText::new("Basic Properties").strong());
            
            // Get all property names
            let property_names = component.get_property_names();
            
            for property_name in property_names {
                if let Some(current_value) = component.get_property(&property_name) {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}:", property_name));
                        
                        let mut value = current_value.clone();
                        if ui.text_edit_singleline(&mut value).changed() {
                            component.set_property(&property_name, &value);
                        }
                    });
                }
            }
        });
    }

    /// Render a property category with collapsible header
    fn render_property_category(
        &mut self,
        ui: &mut Ui,
        category: &PropertyCategory,
        properties: &[PropertyInfo],
        component: &mut dyn EnhancedComponent,
    ) {
        let category_name = format!("{:?}", category);
        let is_expanded = self.expanded_categories.get(category).unwrap_or(&false);
        
        let header_response = ui.collapsing(
            RichText::new(format!("{} {} ({})", self.get_category_icon(category), category_name, properties.len())).strong(),
            *is_expanded,
            |ui| {
                for property in properties {
                    self.render_property_editor(ui, property, component);
                }
            },
        );
        
        // Update expanded state
        self.expanded_categories.insert(category.clone(), header_response.header_response.clicked());
    }

    /// Render a single property editor
    fn render_property_editor(
        &mut self,
        ui: &mut Ui,
        property: &PropertyInfo,
        component: &mut dyn EnhancedComponent,
    ) {
        ui.horizontal(|ui| {
            // Property label with tooltip
            let label_response = ui.label(&property.display_name);
            if !property.description.is_empty() {
                label_response.on_hover_text(&property.description);
            }
            
            // Property editor based on type
            if let Some(current_value) = component.get_property_value(&property.name) {
                let mut changed = false;
                let mut new_value = current_value.clone();
                
                match &property.property_type {
                    crate::rcl::ui::properties::PropertyType::Text => {
                        if let PropertyValue::Text(mut text) = current_value {
                            if ui.text_edit_singleline(&mut text).changed() {
                                new_value = PropertyValue::Text(text);
                                changed = true;
                            }
                        }
                    }
                    crate::rcl::ui::properties::PropertyType::Float => {
                        if let PropertyValue::Float(mut value) = current_value {
                            if let Some(constraints) = &property.constraints {
                                let min = constraints.min.unwrap_or(f64::MIN) as f32;
                                let max = constraints.max.unwrap_or(f64::MAX) as f32;
                                if ui.add(Slider::new(&mut (value as f32), min..=max)).changed() {
                                    new_value = PropertyValue::Float(value);
                                    changed = true;
                                }
                            } else {
                                let mut value_f32 = value as f32;
                                if ui.add(DragValue::new(&mut value_f32)).changed() {
                                    new_value = PropertyValue::Float(value_f32 as f64);
                                    changed = true;
                                }
                            }
                        }
                    }
                    crate::rcl::ui::properties::PropertyType::Boolean => {
                        if let PropertyValue::Boolean(mut value) = current_value {
                            if ui.checkbox(&mut value, "").changed() {
                                new_value = PropertyValue::Boolean(value);
                                changed = true;
                            }
                        }
                    }
                    crate::rcl::ui::properties::PropertyType::Color => {
                        if let PropertyValue::Color(mut color) = current_value {
                            let mut color_array = [color.r(), color.g(), color.b(), color.a()];
                            if ui.color_edit_button_rgba_unmultiplied(&mut color_array).changed() {
                                new_value = PropertyValue::Color(Color32::from_rgba_unmultiplied(
                                    color_array[0], color_array[1], color_array[2], color_array[3]
                                ));
                                changed = true;
                            }
                        }
                    }
                    _ => {
                        // Fallback to string editing for other types
                        let mut text = current_value.to_string();
                        if ui.text_edit_singleline(&mut text).changed() {
                            if let Ok(parsed_value) = PropertyValue::from_string(&text, &property.property_type) {
                                new_value = parsed_value;
                                changed = true;
                            }
                        }
                    }
                }
                
                // Apply changes
                if changed {
                    if let Err(error) = component.set_property_value(&property.name, new_value) {
                        self.validation_errors.insert(property.name.clone(), error);
                    } else {
                        self.validation_errors.remove(&property.name);
                    }
                }
            }
            
            // Show validation error if any
            if let Some(error) = self.validation_errors.get(&property.name) {
                ui.colored_label(Color32::RED, "⚠");
                if ui.response().hovered() {
                    show_tooltip_text(ui.ctx(), "validation_error".into(), error);
                }
            }
        });
    }

    /// Try to get enhanced component interface
    fn try_get_enhanced_component(&self, component: &mut Box<dyn Component>) -> Option<&mut dyn EnhancedComponent> {
        // This is a simplified approach - in a real implementation, you'd use proper downcasting
        // For now, return None to use basic property handling
        None
    }

    /// Find common properties between multiple components
    fn find_common_properties(&self, components: &[Box<dyn Component>]) -> Vec<String> {
        if components.is_empty() {
            return Vec::new();
        }
        
        let first_properties: std::collections::HashSet<String> = 
            components[0].get_property_names().into_iter().collect();
        
        let mut common = first_properties;
        for component in components.iter().skip(1) {
            let component_properties: std::collections::HashSet<String> = 
                component.get_property_names().into_iter().collect();
            common.retain(|prop| component_properties.contains(prop));
        }
        
        common.into_iter().collect()
    }

    /// Render property editor for multiple components
    fn render_multi_component_property(&mut self, ui: &mut Ui, components: &mut [Box<dyn Component>], property_name: &str) {
        ui.horizontal(|ui| {
            ui.label(format!("{}:", property_name));
            
            // Get values from all components
            let values: Vec<Option<String>> = components.iter()
                .map(|comp| comp.get_property(property_name))
                .collect();
            
            // Check if all values are the same
            let first_value = &values[0];
            let all_same = values.iter().all(|v| v == first_value);
            
            if all_same {
                // Edit single value that applies to all
                if let Some(mut value) = first_value.clone() {
                    if ui.text_edit_singleline(&mut value).changed() {
                        // Apply to all components
                        for component in components.iter_mut() {
                            component.set_property(property_name, &value);
                        }
                    }
                }
            } else {
                // Multiple different values
                ui.label(RichText::new("(Multiple values)").italics());
                
                if ui.button("Unify").on_hover_text("Set all to the same value").clicked() {
                    // Set all to first component's value
                    if let Some(first_val) = &values[0] {
                        for component in components.iter_mut() {
                            component.set_property(property_name, first_val);
                        }
                    }
                }
            }
        });
    }

    /// Check if a property should be shown based on filters
    fn should_show_property(&self, property: &PropertyInfo) -> bool {
        // Filter by search
        if !self.search_filter.is_empty() {
            let search_lower = self.search_filter.to_lowercase();
            if !property.name.to_lowercase().contains(&search_lower) &&
               !property.display_name.to_lowercase().contains(&search_lower) &&
               !property.description.to_lowercase().contains(&search_lower) {
                return false;
            }
        }
        
        // Filter by advanced
        if !self.show_advanced && property.name.starts_with("_") {
            return false;
        }
        
        true
    }

    /// Get icon for property category
    fn get_category_icon(&self, category: &PropertyCategory) -> &'static str {
        match category {
            PropertyCategory::Content => "📝",
            PropertyCategory::Layout => "📐",
            PropertyCategory::Style => "🎨",
            PropertyCategory::Behavior => "⚡",
            PropertyCategory::Accessibility => "♿",
            PropertyCategory::Custom(_) => "🔧",
        }
    }

    /// Apply theme to a component
    pub fn apply_theme_to_component(&self, component: &mut dyn EnhancedComponent) {
        self.theme.apply_to_component(component, component.name());
    }

    /// Undo last property change
    pub fn undo(&mut self) -> bool {
        if self.history_position > 0 {
            self.history_position -= 1;
            // TODO: Apply undo
            true
        } else {
            false
        }
    }

    /// Redo next property change
    pub fn redo(&mut self) -> bool {
        if self.history_position < self.edit_history.len() {
            self.history_position += 1;
            // TODO: Apply redo
            true
        } else {
            false
        }
    }
}

impl Default for EnhancedPropertyInspector {
    fn default() -> Self {
        Self::new()
    }
}