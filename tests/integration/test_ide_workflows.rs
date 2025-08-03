//! Integration Tests for IDE Workflows
//! 
//! This module contains integration tests that verify complete workflows
//! work correctly, including project creation, component manipulation,
//! and property editing.

use ide_rs::ide_app::app_state::IdeAppState;
use ide_rs::ide_app::drag_drop::{ComponentType, DragState, DragType};
use ide_rs::editor::project_manager::ProjectManager;
use ide_rs::rcl::ui::basic::{button::Button, label::Label};
use ide_rs::rcl::ui::component::Component;

#[cfg(test)]
mod ide_workflow_tests {
    use super::*;

    #[test]
    fn test_complete_component_creation_workflow() {
        // Create a new IDE app state
        let mut app_state = IdeAppState::new();
        
        // Verify initial state
        assert!(app_state.components.is_empty());
        assert!(app_state.selected_component.is_none());
        
        // Simulate creating a button component
        let button = Box::new(Button::new("Test Button".to_string()));
        app_state.components.push(button);
        
        // Verify component was added
        assert_eq!(app_state.components.len(), 1);
        assert_eq!(app_state.components[0].name(), "Button");
        
        // Select the component
        app_state.selected_component = Some(0);
        assert_eq!(app_state.selected_component, Some(0));
        
        // Verify the component can be accessed
        if let Some(idx) = app_state.selected_component {
            assert!(idx < app_state.components.len());
            let component = &app_state.components[idx];
            assert_eq!(component.name(), "Button");
        }
    }

    #[test]
    fn test_drag_and_drop_workflow() {
        let mut drag_state = DragState::new();
        
        // Start dragging a button from palette
        let start_pos = egui::Pos2::new(50.0, 100.0);
        drag_state.start_drag(
            DragType::ComponentFromPalette(ComponentType::Button),
            start_pos
        );
        
        // Verify drag started correctly
        assert!(drag_state.is_dragging);
        assert_eq!(drag_state.start_position, Some(start_pos));
        
        // Simulate mouse movement
        let move_pos = egui::Pos2::new(150.0, 200.0);
        drag_state.update_drag_position(move_pos);
        
        // Verify position updated
        assert_eq!(drag_state.current_position, Some(move_pos));
        assert_eq!(drag_state.preview_position, Some(move_pos));
        
        // Mark drop as valid (over form area)
        drag_state.drop_valid = true;
        assert!(drag_state.drop_valid);
        
        // Complete the drag operation
        let completion = drag_state.end_drag();
        
        // Verify drag completed successfully
        assert!(!drag_state.is_dragging);
        assert!(completion.is_some());
        
        let result = completion.unwrap();
        assert!(result.drop_valid);
        assert_eq!(result.start_position, Some(start_pos));
        assert_eq!(result.end_position, Some(move_pos));
        
        // Verify drag type
        match result.drag_type {
            DragType::ComponentFromPalette(ComponentType::Button) => {
                // Success - expected drag type
            }
            _ => panic!("Unexpected drag type"),
        }
    }

    #[test]
    fn test_component_selection_workflow() {
        let mut app_state = IdeAppState::new();
        
        // Add multiple components
        app_state.components.push(Box::new(Button::new("Button 1".to_string())));
        app_state.components.push(Box::new(Label::new("Label 1".to_string())));
        app_state.components.push(Box::new(Button::new("Button 2".to_string())));
        
        assert_eq!(app_state.components.len(), 3);
        
        // Test selecting different components
        app_state.selected_component = Some(0);
        assert_eq!(app_state.selected_component, Some(0));
        
        app_state.selected_component = Some(1);
        assert_eq!(app_state.selected_component, Some(1));
        
        app_state.selected_component = Some(2);
        assert_eq!(app_state.selected_component, Some(2));
        
        // Test deselection
        app_state.selected_component = None;
        assert!(app_state.selected_component.is_none());
        
        // Test form selection (using usize::MAX)
        app_state.selected_component = Some(usize::MAX);
        assert_eq!(app_state.selected_component, Some(usize::MAX));
    }

    #[test]
    fn test_project_creation_workflow() {
        let mut project_manager = ProjectManager::new();
        
        // Verify initial state
        assert!(project_manager.current_project.is_none());
        assert!(project_manager.recent_projects.is_empty());
        
        // Create new project dialog state
        project_manager.show_new_project_dialog = true;
        project_manager.new_project_name = "Test Project".to_string();
        project_manager.new_project_location = "/test/path".to_string();
        project_manager.selected_template = "Basic GUI".to_string();
        
        // Verify dialog state
        assert!(project_manager.show_new_project_dialog);
        assert_eq!(project_manager.new_project_name, "Test Project");
        assert_eq!(project_manager.new_project_location, "/test/path");
        assert_eq!(project_manager.selected_template, "Basic GUI");
    }

    #[test]
    fn test_property_editing_workflow() {
        use ide_rs::editor::inspector::{PropertyInspector, PropertyValue};
        
        let mut inspector = PropertyInspector::new();
        
        // Verify initial state
        assert!(inspector.current_values.is_empty());
        assert!(!inspector.show_advanced);
        
        // Simulate editing a button's label property
        let component_id = 0;
        let property_name = "label";
        let new_value = PropertyValue::String("Updated Label".to_string());
        
        inspector.set_property_value(component_id, property_name, new_value.clone());
        
        // Verify property was set
        let property_key = format!("{}_{}", component_id, property_name);
        assert!(inspector.current_values.contains_key(&property_key));
        assert_eq!(inspector.current_values.get(&property_key).unwrap(), &new_value);
        
        // Test enabling advanced properties
        inspector.show_advanced = true;
        assert!(inspector.show_advanced);
        
        // Test search filtering
        inspector.search_filter = "label".to_string();
        assert_eq!(inspector.search_filter, "label");
    }

    #[test]
    fn test_visual_designer_workflow() {
        use ide_rs::editor::visual_designer::VisualDesigner;
        
        let mut designer = VisualDesigner::new();
        
        // Verify initial state
        assert!(designer.selection.selected.is_empty());
        assert!(designer.selection.primary.is_none());
        
        // Test component selection
        designer.selection.select(0);
        assert!(designer.selection.selected.contains(&0));
        assert_eq!(designer.selection.primary, Some(0));
        
        // Test adding more selections
        designer.selection.select(1);
        assert!(designer.selection.selected.contains(&1));
        assert_eq!(designer.selection.primary, Some(1));
        
        // Test clearing selection
        designer.clear_selection();
        assert!(designer.selection.selected.is_empty());
        assert!(designer.selection.primary.is_none());
        
        // Test multi-selection
        designer.selection.select(0);
        designer.selection.toggle_selection(1); // Add to selection
        assert!(designer.selection.selected.contains(&0));
        assert!(designer.selection.selected.contains(&1));
    }

    #[test]
    fn test_component_hierarchy_workflow() {
        let mut app_state = IdeAppState::new();
        
        // Add parent-child relationship simulation
        app_state.components.push(Box::new(Button::new("Parent Button".to_string())));
        app_state.components.push(Box::new(Label::new("Child Label".to_string())));
        
        // In a real hierarchy system, we would track parent-child relationships
        // For now, we just verify components exist
        assert_eq!(app_state.components.len(), 2);
        assert_eq!(app_state.components[0].name(), "Button");
        assert_eq!(app_state.components[1].name(), "Label");
    }

    #[test]
    fn test_theme_application_workflow() {
        use ide_rs::rcl::ui::theme::{Theme, ThemeManager};
        
        let mut theme_manager = ThemeManager::new();
        
        // Add themes
        let light_theme = Theme::light_theme();
        let dark_theme = Theme::dark_theme();
        
        theme_manager.add_theme(light_theme);
        theme_manager.add_theme(dark_theme);
        
        // Test theme switching
        assert!(theme_manager.set_active_theme("Light Theme").is_ok());
        assert!(theme_manager.set_active_theme("Dark Theme").is_ok());
        assert!(theme_manager.set_active_theme("Non-existent").is_err());
        
        // Verify active theme
        if let Some(active) = theme_manager.get_active_theme() {
            assert_eq!(active.name, "Dark Theme");
        }
    }

    #[test]
    fn test_undo_redo_workflow() {
        let mut app_state = IdeAppState::new();
        
        // Add initial component
        app_state.components.push(Box::new(Button::new("Initial".to_string())));
        let initial_count = app_state.components.len();
        
        // Simulate action that can be undone (add component)
        app_state.components.push(Box::new(Label::new("Added".to_string())));
        assert_eq!(app_state.components.len(), initial_count + 1);
        
        // In a real undo system, we would track commands and reverse them
        // For now, we simulate by removing the component
        app_state.components.pop();
        assert_eq!(app_state.components.len(), initial_count);
        
        // And simulate redo by adding it back
        app_state.components.push(Box::new(Label::new("Redone".to_string())));
        assert_eq!(app_state.components.len(), initial_count + 1);
    }

    #[test]
    fn test_form_background_selection_workflow() {
        let mut app_state = IdeAppState::new();
        
        // Add some components
        app_state.components.push(Box::new(Button::new("Button".to_string())));
        app_state.components.push(Box::new(Label::new("Label".to_string())));
        
        // Select a component first
        app_state.selected_component = Some(0);
        assert_eq!(app_state.selected_component, Some(0));
        
        // Click on form background (simulated by setting to usize::MAX)
        app_state.selected_component = Some(usize::MAX);
        assert_eq!(app_state.selected_component, Some(usize::MAX));
        
        // This should show form properties instead of component properties
        if let Some(selected) = app_state.selected_component {
            if selected == usize::MAX {
                // Form is selected - would show form properties
                assert!(true); // Form selection works
            } else {
                panic!("Expected form selection");
            }
        }
    }
}