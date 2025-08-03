//! Tests for the Property Inspector system
//! 
//! This module contains comprehensive tests for the property inspector,
//! covering property definitions, value editors, validation, and UI rendering.

use ide_rs::editor::inspector::*;
use ide_rs::rcl::ui::component::Component;
use ide_rs::rcl::ui::basic::button::Button;

#[cfg(test)]
mod property_inspector_tests {
    use super::*;

    #[test]
    fn test_property_inspector_creation() {
        let inspector = PropertyInspector::new();
        
        // Should have default property definitions for built-in components
        assert!(inspector.property_definitions.contains_key("Button"));
        assert!(inspector.property_definitions.contains_key("Label"));
        
        // Should be initialized with default settings
        assert!(!inspector.show_advanced);
        assert!(inspector.live_preview);
        assert!(inspector.search_filter.is_empty());
    }

    #[test]
    fn test_button_property_definitions() {
        let inspector = PropertyInspector::new();
        let button_props = inspector.property_definitions.get("Button").unwrap();
        
        // Button should have standard properties
        let prop_names: Vec<&str> = button_props.iter().map(|p| p.name.as_str()).collect();
        assert!(prop_names.contains(&"label"));
        assert!(prop_names.contains(&"enabled"));
        assert!(prop_names.contains(&"width"));
        assert!(prop_names.contains(&"height"));
        assert!(prop_names.contains(&"background_color"));
    }

    #[test]
    fn test_property_value_types() {
        use ide_rs::editor::inspector::PropertyValue;
        
        // Test different property value types
        let string_val = PropertyValue::String("test".to_string());
        let number_val = PropertyValue::Number(42.0);
        let bool_val = PropertyValue::Boolean(true);
        let color_val = PropertyValue::Color([255, 0, 0, 255]);
        
        // Test equality
        assert_eq!(string_val, PropertyValue::String("test".to_string()));
        assert_eq!(number_val, PropertyValue::Number(42.0));
        assert_eq!(bool_val, PropertyValue::Boolean(true));
        assert_eq!(color_val, PropertyValue::Color([255, 0, 0, 255]));
        
        // Test inequality
        assert_ne!(string_val, PropertyValue::String("different".to_string()));
        assert_ne!(number_val, PropertyValue::Number(43.0));
        assert_ne!(bool_val, PropertyValue::Boolean(false));
    }

    #[test]
    fn test_property_type_validation() {
        use ide_rs::editor::inspector::{PropertyType, PropertyValue};
        
        let string_type = PropertyType::String { max_length: Some(10) };
        let number_type = PropertyType::Number { min: 0.0, max: 100.0, step: 1.0 };
        let bool_type = PropertyType::Boolean;
        
        // Test type matching
        assert!(matches!(string_type, PropertyType::String { .. }));
        assert!(matches!(number_type, PropertyType::Number { .. }));
        assert!(matches!(bool_type, PropertyType::Boolean));
    }

    #[test]
    fn test_property_groups() {
        use ide_rs::editor::inspector::PropertyGroup;
        
        let appearance = PropertyGroup::Appearance;
        let layout = PropertyGroup::Layout;
        let behavior = PropertyGroup::Behavior;
        
        assert_eq!(appearance.name(), "Appearance");
        assert_eq!(layout.name(), "Layout");
        assert_eq!(behavior.name(), "Behavior");
        
        assert_eq!(appearance.icon(), "🎨");
        assert_eq!(layout.icon(), "📐");
        assert_eq!(behavior.icon(), "⚡");
    }

    #[test]
    fn test_property_value_setting() {
        let mut inspector = PropertyInspector::new();
        
        // Set a property value
        inspector.set_property_value(0, "test_prop", PropertyValue::String("test_value".to_string()));
        
        // Check if the value was set
        let key = format!("{}_{}", 0, "test_prop");
        assert!(inspector.current_values.contains_key(&key));
        assert_eq!(
            inspector.current_values.get(&key).unwrap(),
            &PropertyValue::String("test_value".to_string())
        );
    }

    #[test]
    fn test_search_filter() {
        let mut inspector = PropertyInspector::new();
        
        // Initially empty
        assert!(inspector.search_filter.is_empty());
        
        // Set search filter
        inspector.search_filter = "label".to_string();
        assert_eq!(inspector.search_filter, "label");
    }

    #[test]
    fn test_advanced_properties_toggle() {
        let mut inspector = PropertyInspector::new();
        
        // Initially not showing advanced
        assert!(!inspector.show_advanced);
        
        // Toggle advanced properties
        inspector.show_advanced = true;
        assert!(inspector.show_advanced);
    }

    #[test]
    fn test_validation_errors() {
        let mut inspector = PropertyInspector::new();
        
        // Set a validation error
        inspector.validation_errors.insert("test_error".to_string(), "Invalid value".to_string());
        
        // Check if error was set
        assert!(inspector.validation_errors.contains_key("test_error"));
        assert_eq!(
            inspector.validation_errors.get("test_error").unwrap(),
            "Invalid value"
        );
    }

    #[test]
    fn test_property_change_history() {
        let mut inspector = PropertyInspector::new();
        
        // Initially no history
        assert!(inspector.change_history.is_empty());
        assert_eq!(inspector.history_index, 0);
        
        // Set a property to create history
        let old_value = PropertyValue::String("old".to_string());
        let new_value = PropertyValue::String("new".to_string());
        
        inspector.set_property_value(0, "test", new_value.clone());
        
        // History should still be empty as we didn't explicitly add to history
        assert!(inspector.change_history.is_empty());
    }

    #[test]
    fn test_expanded_groups_state() {
        let mut inspector = PropertyInspector::new();
        
        // Initially no expanded groups
        assert!(inspector.expanded_groups.is_empty());
        
        // Set a group as expanded
        inspector.expanded_groups.insert("Appearance".to_string(), true);
        inspector.expanded_groups.insert("Layout".to_string(), false);
        
        assert_eq!(inspector.expanded_groups.get("Appearance"), Some(&true));
        assert_eq!(inspector.expanded_groups.get("Layout"), Some(&false));
    }

    #[test] 
    fn test_property_definition_structure() {
        use ide_rs::editor::inspector::{PropertyDefinition, PropertyType, PropertyValue};
        
        let prop_def = PropertyDefinition {
            name: "test_prop".to_string(),
            label: "Test Property".to_string(),
            description: "A test property for validation".to_string(),
            property_type: PropertyType::String { max_length: Some(50) },
            group: "Test".to_string(),
            advanced: false,
            default_value: PropertyValue::String("default".to_string()),
            read_only: false,
        };
        
        assert_eq!(prop_def.name, "test_prop");
        assert_eq!(prop_def.label, "Test Property");
        assert!(!prop_def.advanced);
        assert!(!prop_def.read_only);
        assert_eq!(prop_def.group, "Test");
    }

    #[test]
    fn test_adding_custom_component_properties() {
        let mut inspector = PropertyInspector::new();
        
        let custom_props = vec![
            PropertyDefinition {
                name: "custom_prop".to_string(),
                label: "Custom Property".to_string(),
                description: "A custom property".to_string(),
                property_type: PropertyType::Number { min: 0.0, max: 10.0, step: 0.1 },
                group: "Custom".to_string(),
                advanced: false,
                default_value: PropertyValue::Number(5.0),
                read_only: false,
            }
        ];
        
        inspector.add_component_properties("CustomComponent", custom_props);
        
        assert!(inspector.property_definitions.contains_key("CustomComponent"));
        let props = inspector.property_definitions.get("CustomComponent").unwrap();
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].name, "custom_prop");
    }
}