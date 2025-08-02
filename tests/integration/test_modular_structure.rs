//! Integration tests for the modular code structure
//!
//! These tests verify that the modularized architecture works correctly
//! and that all modules can be imported and used together.

use ide_rs::editor::{
    smart_editing_modules::*,
    enhanced_property_inspector::*,
    modern_ide_integration_modules::*,
};
use ide_rs::shared::*;

#[cfg(test)]
mod smart_editing_tests {
    use super::*;
    use egui::Pos2;

    #[test]
    fn test_alignment_guide_manager_creation() {
        let mut manager = AlignmentGuideManager::new();
        assert!(manager.guides.is_empty());
        assert!(!manager.enabled);
    }

    #[test]
    fn test_alignment_guide_operations() {
        let mut manager = AlignmentGuideManager::new();
        manager.enabled = true;
        
        // Add a vertical guide
        manager.add_guide(GuideDirection::Vertical, 100.0);
        assert_eq!(manager.guides.len(), 1);
        
        // Check snap functionality
        let snap_result = manager.snap_to_guides(Pos2::new(102.0, 50.0), 5.0);
        assert!(snap_result.is_some());
        let snapped = snap_result.unwrap();
        assert_eq!(snapped.x, 100.0);
        assert_eq!(snapped.y, 50.0);
    }

    #[test]
    fn test_magnetism_manager() {
        let mut manager = MagnetismManager::new();
        assert!(manager.zones.is_empty());
        assert!(!manager.enabled);
        
        manager.enabled = true;
        manager.add_zone(MagnetismZone {
            position: Pos2::new(50.0, 50.0),
            magnetism_type: MagnetismType::EdgeToEdge,
            strength: 10.0,
            active: true,
        });
        
        assert_eq!(manager.zones.len(), 1);
    }

    #[test]
    fn test_spacing_guide_manager() {
        let mut manager = SpacingGuideManager::new();
        assert!(manager.guides.is_empty());
        assert!(!manager.enabled);
        
        manager.enabled = true;
        manager.add_guide(SpacingGuide {
            position: Pos2::new(25.0, 25.0),
            spacing_type: SpacingType::Margin,
            value: 8.0,
            visible: true,
        });
        
        assert_eq!(manager.guides.len(), 1);
        assert!(manager.guides[0].visible);
    }

    #[test]
    fn test_learning_system() {
        let mut system = LearningSystem::new();
        assert!(system.patterns.detected_patterns.is_empty());
        assert!(system.behavior_tracker.action_history.is_empty());
        
        // Test that the system can track actions
        system.behavior_tracker.track_action("component_moved".to_string(), 
            std::collections::HashMap::new());
        assert_eq!(system.behavior_tracker.action_history.len(), 1);
    }
}

#[cfg(test)]
mod enhanced_property_inspector_tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_multi_selection_manager() {
        let mut manager = MultiSelectionManager::new();
        assert!(manager.selected_components.is_empty());
        assert!(manager.common_properties.is_empty());
        
        // Add a component selection
        let component = ComponentSelection {
            component_id: 1,
            component_type: "button".to_string(),
            properties: HashMap::from([
                ("width".to_string(), PropertyValue::Number(100.0)),
                ("height".to_string(), PropertyValue::Number(30.0)),
            ]),
        };
        
        manager.add_to_selection(vec![component]);
        assert_eq!(manager.selected_components.len(), 1);
    }

    #[test]
    fn test_design_system_integration() {
        let integration = DesignSystemIntegration::new();
        assert!(!integration.design_tokens.colors.is_empty()); // Should have default tokens
        assert!(integration.validator.validation_rules.is_empty()); // No custom rules initially
    }

    #[test]
    fn test_ai_property_suggestions() {
        let mut suggestions = AiPropertySuggestions::new();
        assert!(suggestions.suggestions.is_empty());
        
        // Test suggestion generation
        let context = SuggestionContext {
            component_type: "button".to_string(),
            nearby_components: vec![],
            container_context: HashMap::new(),
            recent_actions: vec![],
            design_phase: DesignPhase::Styling,
            target_platform: vec!["web".to_string()],
        };
        
        let property_value = PropertyValue::Number(12.0); // Small font size
        suggestions.generate_suggestions(&context, "font_size", &property_value);
        
        // Should generate accessibility suggestion for small font size
        assert!(!suggestions.suggestions.is_empty());
        let accessibility_suggestions: Vec<_> = suggestions.suggestions.iter()
            .filter(|s| s.category == SuggestionCategory::Accessibility)
            .collect();
        assert!(!accessibility_suggestions.is_empty());
    }
}

#[cfg(test)]
mod modern_ide_integration_tests {
    use super::*;

    #[test]
    fn test_design_token_system() {
        let mut system = DesignTokenSystem::new();
        assert!(!system.colors.is_empty()); // Should have default colors
        assert!(!system.typography.is_empty()); // Should have default typography
        
        // Test adding a new color token
        system.add_color_token("custom-blue".to_string(), [59, 130, 246, 255]);
        assert!(system.colors.contains_key("custom-blue"));
    }

    #[test]
    fn test_component_library() {
        let mut library = ComponentLibrary::new();
        assert!(library.templates.is_empty());
        
        // Test adding a template
        let template = ComponentTemplate {
            name: "Test Button".to_string(),
            description: "A test button component".to_string(),
            component_type: "button".to_string(),
            properties: HashMap::from([
                ("width".to_string(), PropertyValue::Number(120.0)),
                ("height".to_string(), PropertyValue::Number(40.0)),
            ]),
            preview_data: None,
            tags: vec!["ui".to_string(), "button".to_string()],
            category: LibraryCategory::Basic,
        };
        
        library.add_template(template);
        assert_eq!(library.templates.len(), 1);
        assert_eq!(library.templates[0].name, "Test Button");
    }

    #[test]
    fn test_theme_system() {
        let mut system = ThemeSystem::new();
        assert!(!system.themes.is_empty()); // Should have default themes
        
        // Test theme switching
        if let Some(dark_theme) = system.themes.iter().find(|t| t.name == "Dark") {
            system.set_active_theme(dark_theme.id.clone());
            assert_eq!(system.active_theme_id, Some(dark_theme.id.clone()));
        }
    }

    #[test]
    fn test_code_generator() {
        let mut generator = CodeGenerator::default();
        assert!(!generator.templates.is_empty()); // Should have default templates
        
        // Test basic code generation capability
        let context = GenerationContext {
            component_name: "TestButton".to_string(),
            component_type: "button".to_string(),
            properties: HashMap::from([
                ("text".to_string(), PropertyValue::String("Click me".to_string())),
                ("width".to_string(), PropertyValue::Number(100.0)),
            ]),
            children: vec![],
            framework: "react".to_string(),
        };
        
        let result = generator.generate_component(&context);
        assert!(result.is_ok());
        let generated = result.unwrap();
        assert!(!generated.code.is_empty());
        assert!(generated.code.contains("TestButton"));
    }
}

#[cfg(test)]
mod shared_utilities_tests {
    use super::*;
    use egui::{Pos2, Vec2};

    #[test]
    fn test_geometry_bounds() {
        let bounds = Bounds::new(Pos2::new(10.0, 20.0), Pos2::new(50.0, 60.0));
        assert_eq!(bounds.width(), 40.0);
        assert_eq!(bounds.height(), 40.0);
        assert_eq!(bounds.center(), Pos2::new(30.0, 40.0));
        
        // Test containment
        assert!(bounds.contains_point(Pos2::new(30.0, 40.0)));
        assert!(!bounds.contains_point(Pos2::new(5.0, 15.0)));
    }

    #[test]
    fn test_geometry_transform() {
        let transform = Transform2D::translation(Vec2::new(10.0, 20.0));
        let point = Pos2::new(5.0, 5.0);
        let transformed = transform.transform_point(point);
        assert_eq!(transformed, Pos2::new(15.0, 25.0));
    }

    #[test]
    fn test_spatial_index() {
        let mut index = SpatialIndex::new(10.0);
        let bounds = Bounds::new(Pos2::new(0.0, 0.0), Pos2::new(5.0, 5.0));
        
        index.insert(bounds.clone(), "test_item");
        
        let query_bounds = Bounds::new(Pos2::new(2.0, 2.0), Pos2::new(7.0, 7.0));
        let results = index.query(&query_bounds);
        assert_eq!(results.len(), 1);
        assert_eq!(*results[0], "test_item");
    }

    #[test]
    fn test_color_palette() {
        let mut palette = ColorPalette::default();
        assert!(!palette.primary_colors.is_empty());
        assert!(!palette.semantic_colors.is_empty());
        
        // Test adding custom color
        palette.add_semantic("custom".to_string(), egui::Color32::from_rgb(255, 0, 0));
        assert!(palette.get_semantic("custom").is_some());
    }

    #[test]
    fn test_color_harmony() {
        let base_color = egui::Color32::from_rgb(59, 130, 246);
        let harmony = ColorHarmony::new(base_color);
        
        assert!(harmony.schemes.contains_key("complementary"));
        assert!(harmony.schemes.contains_key("analogous"));
        assert!(harmony.schemes.contains_key("triadic"));
        
        let complementary = harmony.complementary();
        assert_eq!(complementary.len(), 2);
        assert_eq!(complementary[0], base_color);
    }

    #[test]
    fn test_accessibility_checker() {
        let checker = AccessibilityChecker::default();
        
        // Test contrast ratio calculation
        let white = egui::Color32::WHITE;
        let black = egui::Color32::BLACK;
        let ratio = checker.contrast_ratio(black, white);
        assert!(ratio > 20.0); // Should be very high contrast
        
        // Test WCAG compliance
        assert!(checker.check_wcag_compliance(black, white, "AA"));
        assert!(checker.check_wcag_compliance(black, white, "AAA"));
    }

    #[test]
    fn test_validation_system() {
        let mut validator = PropertyValidator::new("test_validator".to_string());
        
        // Add a validation rule
        validator.add_rule(PropertyValidationRule {
            name: "min_width".to_string(),
            property: "width".to_string(),
            rule_type: PropertyRuleType::Range { min: 10.0, max: 1000.0 },
            error_message: "Width must be between 10 and 1000".to_string(),
            severity: ValidationSeverity::Error,
        });
        
        // Test validation
        let result = validator.validate_property("width", "5");
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
        
        let valid_result = validator.validate_property("width", "100");
        assert!(valid_result.is_valid);
        assert!(valid_result.errors.is_empty());
    }

    #[test]
    fn test_serialization_utils() {
        use crate::shared::serialization::*;
        
        let component_data = ComponentData {
            component_type: "button".to_string(),
            id: "btn_1".to_string(),
            properties: HashMap::from([
                ("text".to_string(), PropertyValue::String("Click me".to_string())),
                ("width".to_string(), PropertyValue::Number(100.0)),
            ]),
            children: vec![],
            metadata: ComponentMetadata::default(),
            schema_version: "1.0.0".to_string(),
        };
        
        // Test serialization
        let json = SerializationUtils::serialize_component(&component_data);
        assert!(json.is_ok());
        
        // Test deserialization
        let json_str = json.unwrap();
        let deserialized = SerializationUtils::deserialize_component(&json_str);
        assert!(deserialized.is_ok());
        
        let restored = deserialized.unwrap();
        assert_eq!(restored.component_type, "button");
        assert_eq!(restored.id, "btn_1");
    }

    #[test]
    fn test_performance_profiler() {
        let mut profiler = PerformanceProfiler::new();
        assert!(profiler.sessions.is_empty());
        assert!(profiler.metrics_history.is_empty());
        
        // Test session management
        profiler.start_session("test_session".to_string());
        assert!(profiler.sessions.contains_key("test_session"));
        
        // Record a measurement
        profiler.record_measurement("test_session", "test_operation".to_string(), 
            std::time::Duration::from_millis(10));
        
        let session = profiler.sessions.get("test_session").unwrap();
        assert_eq!(session.measurements.len(), 1);
        assert_eq!(session.measurements[0].name, "test_operation");
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_end_to_end_component_workflow() {
        // This test simulates a complete workflow using the modularized components
        
        // 1. Create a component using design tokens
        let mut design_tokens = DesignTokenSystem::new();
        let button_color = design_tokens.colors.get("primary").unwrap().value;
        
        // 2. Use smart editing for positioning
        let mut alignment_manager = AlignmentGuideManager::new();
        alignment_manager.enabled = true;
        alignment_manager.add_guide(GuideDirection::Vertical, 100.0);
        
        let component_pos = Pos2::new(102.0, 50.0);
        let snapped_pos = alignment_manager.snap_to_guides(component_pos, 5.0).unwrap();
        
        // 3. Create component selection for property editing
        let component = ComponentSelection {
            component_id: 1,
            component_type: "button".to_string(),
            properties: HashMap::from([
                ("x".to_string(), PropertyValue::Number(snapped_pos.x as f64)),
                ("y".to_string(), PropertyValue::Number(snapped_pos.y as f64)),
                ("background_color".to_string(), PropertyValue::Color(button_color)),
            ]),
        };
        
        // 4. Use multi-selection manager
        let mut multi_selection = MultiSelectionManager::new();
        multi_selection.add_to_selection(vec![component]);
        
        // 5. Generate AI suggestions
        let mut ai_suggestions = AiPropertySuggestions::new();
        let context = SuggestionContext {
            component_type: "button".to_string(),
            nearby_components: vec![],
            container_context: HashMap::new(),
            recent_actions: vec![],
            design_phase: DesignPhase::Styling,
            target_platform: vec!["web".to_string()],
        };
        
        ai_suggestions.generate_suggestions(&context, "width", &PropertyValue::Number(20.0));
        
        // 6. Validate the component
        let mut validator = PropertyValidator::new("button_validator".to_string());
        validator.add_rule(PropertyValidationRule {
            name: "min_width".to_string(),
            property: "width".to_string(),
            rule_type: PropertyRuleType::Range { min: 44.0, max: 500.0 },
            error_message: "Button width must meet accessibility requirements".to_string(),
            severity: ValidationSeverity::Error,
        });
        
        let validation_result = validator.validate_property("width", "20");
        
        // Assertions
        assert_eq!(snapped_pos.x, 100.0); // Should snap to guide
        assert_eq!(multi_selection.selected_components.len(), 1);
        assert!(!ai_suggestions.suggestions.is_empty()); // Should have suggestions
        assert!(!validation_result.is_valid); // Should fail validation for accessibility
        
        // The workflow demonstrates that all modules work together correctly
    }

    #[test]
    fn test_theme_and_code_generation_integration() {
        // Test that themes can be applied and code generated with those theme values
        
        let mut theme_system = ThemeSystem::new();
        let mut code_generator = CodeGenerator::default();
        
        // Apply a theme
        if let Some(theme) = theme_system.themes.first() {
            theme_system.set_active_theme(theme.id.clone());
            
            // Generate code with theme values
            let context = GenerationContext {
                component_name: "ThemedButton".to_string(),
                component_type: "button".to_string(),
                properties: HashMap::from([
                    ("background_color".to_string(), 
                     PropertyValue::Color(theme.colors.primary)),
                    ("text_color".to_string(), 
                     PropertyValue::Color(theme.colors.on_primary)),
                ]),
                children: vec![],
                framework: "react".to_string(),
            };
            
            let result = code_generator.generate_component(&context);
            assert!(result.is_ok());
            
            let generated = result.unwrap();
            assert!(generated.code.contains("ThemedButton"));
            // Should contain color values from the theme
            assert!(generated.code.contains("rgba(") || generated.code.contains("#"));
        }
    }

    #[test]
    fn test_serialization_with_all_modules() {
        // Test that components created with all modules can be serialized and deserialized
        
        use crate::shared::serialization::*;
        
        // Create a complex component using multiple modules
        let mut design_tokens = DesignTokenSystem::new();
        let primary_color = design_tokens.colors.get("primary").unwrap().value;
        
        let component_data = ComponentData {
            component_type: "enhanced_button".to_string(),
            id: "btn_enhanced_1".to_string(),
            properties: HashMap::from([
                ("text".to_string(), PropertyValue::String("Enhanced Button".to_string())),
                ("width".to_string(), PropertyValue::Number(120.0)),
                ("height".to_string(), PropertyValue::Number(44.0)), // Accessibility compliant
                ("background_color".to_string(), PropertyValue::Color(primary_color)),
                ("border_radius".to_string(), PropertyValue::Number(8.0)),
            ]),
            children: vec![],
            metadata: ComponentMetadata::default(),
            schema_version: "1.0.0".to_string(),
        };
        
        // Test serialization to different formats
        let native_json = SerializationUtils::serialize_component(&component_data);
        assert!(native_json.is_ok());
        
        let react_jsx = SerializationUtils::export_component(&component_data, &ExportFormat::ReactJsx);
        assert!(react_jsx.is_ok());
        assert!(react_jsx.unwrap().contains("enhanced_button") || 
                react_jsx.as_ref().unwrap().contains("EnhancedButton"));
        
        let html_css = SerializationUtils::export_component(&component_data, &ExportFormat::HtmlCss);
        assert!(html_css.is_ok());
        assert!(html_css.unwrap().contains("Enhanced Button"));
    }
}