//! # Enhanced Component System
//!
//! This module provides an enhanced component system that builds upon the basic Component trait
//! with standardized properties, theming, layout management, and better IDE integration.
//!
//! ## Design Goals
//!
//! - **Backward Compatibility**: Works alongside existing Component trait
//! - **Standardization**: All components follow consistent property patterns
//! - **Extensibility**: Components can add custom properties while maintaining standards
//! - **Performance**: Efficient property access and minimal overhead
//! - **IDE Integration**: Rich property metadata for advanced property inspectors

use egui::{Ui, Color32, Vec2, Pos2, Rect};
use std::collections::HashMap;
use super::properties::{PropertyInfo, PropertyValue, PropertyRegistry, PropertyCategory, standard_properties};
use super::component::Component;

/// Standard properties that all components should have
#[derive(Debug, Clone)]
pub struct StandardProperties {
    // Layout properties
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub margin: f32,
    pub padding: f32,
    
    // Style properties
    pub color: Color32,
    pub background_color: Color32,
    pub border_color: Color32,
    pub border_width: f32,
    pub corner_radius: f32,
    pub font_size: f32,
    pub opacity: f32,
    
    // Behavior properties
    pub visible: bool,
    pub enabled: bool,
    pub editable: bool,
    pub tooltip: String,
    pub tab_index: i32,
    
    // Accessibility properties
    pub aria_label: String,
    pub aria_describedby: String,
    pub role: String,
}

impl Default for StandardProperties {
    fn default() -> Self {
        Self {
            // Layout defaults
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 30.0,
            margin: 0.0,
            padding: 5.0,
            
            // Style defaults
            color: Color32::BLACK,
            background_color: Color32::WHITE,
            border_color: Color32::LIGHT_GRAY,
            border_width: 1.0,
            corner_radius: 3.0,
            font_size: 14.0,
            opacity: 1.0,
            
            // Behavior defaults
            visible: true,
            enabled: true,
            editable: false,
            tooltip: String::new(),
            tab_index: 0,
            
            // Accessibility defaults
            aria_label: String::new(),
            aria_describedby: String::new(),
            role: String::new(),
        }
    }
}

/// Enhanced component trait that provides standardized properties and behavior
pub trait EnhancedComponent: Component {
    /// Get the standard properties for this component
    fn standard_properties(&self) -> &StandardProperties;
    
    /// Get mutable access to standard properties
    fn standard_properties_mut(&mut self) -> &mut StandardProperties;
    
    /// Get the property registry for this component (standard + custom properties)
    fn property_registry(&self) -> PropertyRegistry {
        let mut registry = PropertyRegistry::new();
        
        // Add all standard properties
        for property in standard_properties::all_standard_properties() {
            registry.register(property);
        }
        
        // Add custom properties
        for property in self.custom_properties() {
            registry.register(property);
        }
        
        registry
    }
    
    /// Get custom properties specific to this component type
    fn custom_properties(&self) -> Vec<PropertyInfo> {
        Vec::new() // Default: no custom properties
    }
    
    /// Get the current bounds of the component
    fn bounds(&self) -> Rect {
        let props = self.standard_properties();
        Rect::from_min_size(
            Pos2::new(props.x, props.y),
            Vec2::new(props.width, props.height)
        )
    }
    
    /// Set the position of the component
    fn set_position(&mut self, pos: Pos2) {
        let props = self.standard_properties_mut();
        props.x = pos.x;
        props.y = pos.y;
    }
    
    /// Set the size of the component
    fn set_size(&mut self, size: Vec2) {
        let props = self.standard_properties_mut();
        props.width = size.x;
        props.height = size.y;
    }
    
    /// Check if a point is inside the component
    fn contains_point(&self, point: Pos2) -> bool {
        self.bounds().contains(point)
    }
    
    /// Get property value by name (enhanced version with type safety)
    fn get_property_value(&self, name: &str) -> Option<PropertyValue> {
        let props = self.standard_properties();
        
        match name {
            // Layout properties
            "x" => Some(PropertyValue::Float(props.x as f64)),
            "y" => Some(PropertyValue::Float(props.y as f64)),
            "width" => Some(PropertyValue::Float(props.width as f64)),
            "height" => Some(PropertyValue::Float(props.height as f64)),
            "margin" => Some(PropertyValue::Float(props.margin as f64)),
            "padding" => Some(PropertyValue::Float(props.padding as f64)),
            
            // Style properties
            "color" => Some(PropertyValue::Color(props.color)),
            "background_color" => Some(PropertyValue::Color(props.background_color)),
            "border_color" => Some(PropertyValue::Color(props.border_color)),
            "border_width" => Some(PropertyValue::Float(props.border_width as f64)),
            "corner_radius" => Some(PropertyValue::Float(props.corner_radius as f64)),
            "font_size" => Some(PropertyValue::Float(props.font_size as f64)),
            "opacity" => Some(PropertyValue::Float(props.opacity as f64)),
            
            // Behavior properties
            "visible" => Some(PropertyValue::Boolean(props.visible)),
            "enabled" => Some(PropertyValue::Boolean(props.enabled)),
            "editable" => Some(PropertyValue::Boolean(props.editable)),
            "tooltip" => Some(PropertyValue::Text(props.tooltip.clone())),
            "tab_index" => Some(PropertyValue::Integer(props.tab_index as i64)),
            
            // Accessibility properties
            "aria_label" => Some(PropertyValue::Text(props.aria_label.clone())),
            "aria_describedby" => Some(PropertyValue::Text(props.aria_describedby.clone())),
            "role" => Some(PropertyValue::Text(props.role.clone())),
            
            // Custom properties handled by component
            _ => self.get_custom_property_value(name),
        }
    }
    
    /// Set property value by name (enhanced version with type safety and validation)
    fn set_property_value(&mut self, name: &str, value: PropertyValue) -> Result<(), String> {
        let props = self.standard_properties_mut();
        
        match name {
            // Layout properties
            "x" => {
                if let PropertyValue::Float(v) = value {
                    props.x = v as f32;
                    Ok(())
                } else {
                    Err("Invalid type for x property".to_string())
                }
            }
            "y" => {
                if let PropertyValue::Float(v) = value {
                    props.y = v as f32;
                    Ok(())
                } else {
                    Err("Invalid type for y property".to_string())
                }
            }
            "width" => {
                if let PropertyValue::Float(v) = value {
                    if v >= 1.0 {
                        props.width = v as f32;
                        Ok(())
                    } else {
                        Err("Width must be at least 1".to_string())
                    }
                } else {
                    Err("Invalid type for width property".to_string())
                }
            }
            "height" => {
                if let PropertyValue::Float(v) = value {
                    if v >= 1.0 {
                        props.height = v as f32;
                        Ok(())
                    } else {
                        Err("Height must be at least 1".to_string())
                    }
                } else {
                    Err("Invalid type for height property".to_string())
                }
            }
            "margin" => {
                if let PropertyValue::Float(v) = value {
                    if v >= 0.0 {
                        props.margin = v as f32;
                        Ok(())
                    } else {
                        Err("Margin cannot be negative".to_string())
                    }
                } else {
                    Err("Invalid type for margin property".to_string())
                }
            }
            "padding" => {
                if let PropertyValue::Float(v) = value {
                    if v >= 0.0 {
                        props.padding = v as f32;
                        Ok(())
                    } else {
                        Err("Padding cannot be negative".to_string())
                    }
                } else {
                    Err("Invalid type for padding property".to_string())
                }
            }
            
            // Style properties
            "color" => {
                if let PropertyValue::Color(c) = value {
                    props.color = c;
                    Ok(())
                } else {
                    Err("Invalid type for color property".to_string())
                }
            }
            "background_color" => {
                if let PropertyValue::Color(c) = value {
                    props.background_color = c;
                    Ok(())
                } else {
                    Err("Invalid type for background_color property".to_string())
                }
            }
            "border_color" => {
                if let PropertyValue::Color(c) = value {
                    props.border_color = c;
                    Ok(())
                } else {
                    Err("Invalid type for border_color property".to_string())
                }
            }
            "border_width" => {
                if let PropertyValue::Float(v) = value {
                    if v >= 0.0 {
                        props.border_width = v as f32;
                        Ok(())
                    } else {
                        Err("Border width cannot be negative".to_string())
                    }
                } else {
                    Err("Invalid type for border_width property".to_string())
                }
            }
            "corner_radius" => {
                if let PropertyValue::Float(v) = value {
                    if v >= 0.0 {
                        props.corner_radius = v as f32;
                        Ok(())
                    } else {
                        Err("Corner radius cannot be negative".to_string())
                    }
                } else {
                    Err("Invalid type for corner_radius property".to_string())
                }
            }
            "font_size" => {
                if let PropertyValue::Float(v) = value {
                    if v >= 6.0 && v <= 72.0 {
                        props.font_size = v as f32;
                        Ok(())
                    } else {
                        Err("Font size must be between 6 and 72".to_string())
                    }
                } else {
                    Err("Invalid type for font_size property".to_string())
                }
            }
            "opacity" => {
                if let PropertyValue::Float(v) = value {
                    if v >= 0.0 && v <= 1.0 {
                        props.opacity = v as f32;
                        Ok(())
                    } else {
                        Err("Opacity must be between 0.0 and 1.0".to_string())
                    }
                } else {
                    Err("Invalid type for opacity property".to_string())
                }
            }
            
            // Behavior properties
            "visible" => {
                if let PropertyValue::Boolean(v) = value {
                    props.visible = v;
                    Ok(())
                } else {
                    Err("Invalid type for visible property".to_string())
                }
            }
            "enabled" => {
                if let PropertyValue::Boolean(v) = value {
                    props.enabled = v;
                    Ok(())
                } else {
                    Err("Invalid type for enabled property".to_string())
                }
            }
            "editable" => {
                if let PropertyValue::Boolean(v) = value {
                    props.editable = v;
                    Ok(())
                } else {
                    Err("Invalid type for editable property".to_string())
                }
            }
            "tooltip" => {
                if let PropertyValue::Text(v) = value {
                    props.tooltip = v;
                    Ok(())
                } else {
                    Err("Invalid type for tooltip property".to_string())
                }
            }
            "tab_index" => {
                if let PropertyValue::Integer(v) = value {
                    if v >= 0 {
                        props.tab_index = v as i32;
                        Ok(())
                    } else {
                        Err("Tab index cannot be negative".to_string())
                    }
                } else {
                    Err("Invalid type for tab_index property".to_string())
                }
            }
            
            // Accessibility properties
            "aria_label" => {
                if let PropertyValue::Text(v) = value {
                    props.aria_label = v;
                    Ok(())
                } else {
                    Err("Invalid type for aria_label property".to_string())
                }
            }
            "aria_describedby" => {
                if let PropertyValue::Text(v) = value {
                    props.aria_describedby = v;
                    Ok(())
                } else {
                    Err("Invalid type for aria_describedby property".to_string())
                }
            }
            "role" => {
                if let PropertyValue::Text(v) = value {
                    props.role = v;
                    Ok(())
                } else {
                    Err("Invalid type for role property".to_string())
                }
            }
            
            // Custom properties handled by component
            _ => self.set_custom_property_value(name, value),
        }
    }
    
    /// Get custom property value (override in components that have custom properties)
    fn get_custom_property_value(&self, _name: &str) -> Option<PropertyValue> {
        None // Default: no custom properties
    }
    
    /// Set custom property value (override in components that have custom properties)
    fn set_custom_property_value(&mut self, _name: &str, _value: PropertyValue) -> Result<(), String> {
        Err("Unknown property".to_string()) // Default: reject unknown properties
    }
    
    /// Enhanced render method that applies standard styling
    fn render_enhanced(&mut self, ui: &mut Ui) {
        let props = self.standard_properties();
        
        // Don't render if not visible
        if !props.visible {
            return;
        }
        
        // Apply position and size constraints
        let available_rect = ui.available_rect_before_wrap();
        let desired_rect = Rect::from_min_size(
            Pos2::new(props.x, props.y),
            Vec2::new(props.width, props.height)
        );
        
        // Ensure the component is within bounds
        let final_rect = available_rect.intersect(desired_rect);
        
        if final_rect.width() > 0.0 && final_rect.height() > 0.0 {
            // Create a child UI with the component's bounds
            let mut child_ui = ui.child_ui(final_rect, *ui.layout());
            
            // Apply styling (background, border, etc.)
            self.apply_standard_styling(&mut child_ui);
            
            // Apply opacity
            if props.opacity < 1.0 {
                child_ui.multiply_opacity(props.opacity);
            }
            
            // Render the component content
            if props.enabled {
                self.render_content(&mut child_ui);
            } else {
                // Render disabled state
                child_ui.disable();
                self.render_content(&mut child_ui);
            }
            
            // Handle tooltip
            if !props.tooltip.is_empty() && child_ui.rect_contains_pointer(final_rect) {
                child_ui.on_hover_text(&props.tooltip);
            }
        }
    }
    
    /// Apply standard styling (background, border, etc.)
    fn apply_standard_styling(&self, ui: &mut Ui) {
        let props = self.standard_properties();
        let rect = ui.available_rect_before_wrap();
        
        // Draw background
        if props.background_color != Color32::TRANSPARENT {
            ui.painter().rect(
                rect,
                props.corner_radius,
                props.background_color,
                egui::Stroke::NONE,
            );
        }
        
        // Draw border
        if props.border_width > 0.0 {
            ui.painter().rect_stroke(
                rect,
                props.corner_radius,
                egui::Stroke::new(props.border_width, props.border_color),
            );
        }
    }
    
    /// Render the component's content (to be implemented by specific components)
    fn render_content(&mut self, ui: &mut Ui);
}

/// Backward compatibility: Implement Component trait for EnhancedComponent
impl<T: EnhancedComponent> Component for T {
    fn name(&self) -> &str {
        self.name()
    }
    
    fn render(&mut self, ui: &mut Ui) {
        self.render_enhanced(ui);
    }
    
    fn get_property(&self, name: &str) -> Option<String> {
        self.get_property_value(name).map(|v| v.to_string())
    }
    
    fn set_property(&mut self, name: &str, value: &str) -> bool {
        // Try to get property info to determine type
        let registry = self.property_registry();
        if let Some(property_info) = registry.get(name) {
            match PropertyValue::from_string(value, &property_info.property_type) {
                Ok(property_value) => {
                    self.set_property_value(name, property_value).is_ok()
                }
                Err(_) => false,
            }
        } else {
            false
        }
    }
    
    fn get_property_names(&self) -> Vec<String> {
        self.property_registry().get_names()
    }
}

/// Helper trait for components that want to use the standard properties system
/// without implementing the full EnhancedComponent trait
pub trait StandardPropertiesComponent {
    /// Get the standard properties
    fn standard_properties(&self) -> &StandardProperties;
    
    /// Get mutable standard properties
    fn standard_properties_mut(&mut self) -> &mut StandardProperties;
}

/// Macro to easily implement standard properties for a component
#[macro_export]
macro_rules! impl_standard_properties {
    ($type:ty) => {
        impl StandardPropertiesComponent for $type {
            fn standard_properties(&self) -> &StandardProperties {
                &self.standard_props
            }
            
            fn standard_properties_mut(&mut self) -> &mut StandardProperties {
                &mut self.standard_props
            }
        }
    };
}

/// Utility functions for property management
pub mod property_utils {
    use super::*;
    
    /// Create a property registry with all standard properties
    pub fn create_standard_registry() -> PropertyRegistry {
        let mut registry = PropertyRegistry::new();
        for property in standard_properties::all_standard_properties() {
            registry.register(property);
        }
        registry
    }
    
    /// Validate all properties in a component
    pub fn validate_component_properties<T: EnhancedComponent>(component: &T) -> Vec<String> {
        let mut errors = Vec::new();
        let registry = component.property_registry();
        
        for property_name in registry.get_names() {
            if let Some(property_info) = registry.get(&property_name) {
                if let Some(property_value) = component.get_property_value(&property_name) {
                    if let Some(constraints) = &property_info.constraints {
                        if let Err(error) = property_value.validate(constraints) {
                            errors.push(format!("{}: {}", property_name, error));
                        }
                    }
                }
            }
        }
        
        errors
    }
    
    /// Copy standard properties from one component to another
    pub fn copy_standard_properties<T1: EnhancedComponent, T2: EnhancedComponent>(
        from: &T1,
        to: &mut T2,
    ) {
        let from_props = from.standard_properties();
        let to_props = to.standard_properties_mut();
        *to_props = from_props.clone();
    }
}