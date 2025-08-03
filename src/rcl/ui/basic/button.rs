//! Basic Button Component
//! 
//! Provides a simple, clickable button component with text label.
//! Supports both display and edit modes for design-time interaction.

use crate::rcl::ui::component::Component;

/// A basic button component with a text label
/// 
/// The button can operate in two modes:
/// - **Display mode**: Shows as a clickable button
/// - **Edit mode**: Shows as a text input for label editing
#[derive(Debug, Clone)]
pub struct Button {
    /// The text displayed on the button
    label: String,
    /// Whether the button is in edit mode (for design-time editing)
    editable: bool,
}

impl Button {
    /// Creates a new button with the specified label
    /// 
    /// # Arguments
    /// 
    /// * `label` - The text to display on the button
    /// 
    /// # Returns
    /// 
    /// A new `Button` instance with the specified label
    /// 
    /// # Examples
    /// 
    /// ```ignore
    /// let button = Button::new("Submit".to_string());
    /// assert_eq!(button.label(), "Submit");
    /// assert!(!button.is_editable());
    /// ```
    pub fn new(label: String) -> Self {
        Self {
            label,
            editable: false,
        }
    }
    
    /// Sets the editable state of the button
    /// 
    /// # Arguments
    /// 
    /// * `editable` - `true` to enable label editing, `false` for normal button mode
    pub fn set_editable(&mut self, editable: bool) {
        self.editable = editable;
    }
    
    /// Returns whether the button is currently in editable mode
    /// 
    /// # Returns
    /// 
    /// `true` if the button is editable, `false` otherwise
    pub fn is_editable(&self) -> bool {
        self.editable
    }
    
    /// Gets the current button label
    /// 
    /// # Returns
    /// 
    /// A string slice containing the button's label text
    pub fn label(&self) -> &str {
        &self.label
    }
    
    /// Sets the button label
    /// 
    /// # Arguments
    /// 
    /// * `label` - The new label text for the button
    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }
}

impl Component for Button {
    /// Returns the component type name
    /// 
    /// # Returns
    /// 
    /// The string "Button" identifying this component type
    fn name(&self) -> &str {
        "Button"
    }
    
    /// Renders the button component in either display or edit mode
    /// 
    /// The rendering behavior depends on the current editable state:
    /// - **Display mode**: Shows a clickable button with the current label
    /// - **Edit mode**: Shows a text input field for modifying the label
    /// 
    /// # Arguments
    /// 
    /// * `ui` - The egui UI context for rendering
    fn render(&mut self, ui: &mut egui::Ui) {
        if self.editable {
            // Edit mode - show text input
            ui.text_edit_singleline(&mut self.label);
        } else {
            // Display mode - show button
            ui.button(&self.label);
        }
    }
    
    fn get_property(&self, name: &str) -> Option<String> {
        match name {
            "label" => Some(self.label.clone()),
            "editable" => Some(self.editable.to_string()),
            _ => None,
        }
    }
    
    fn set_property(&mut self, name: &str, value: &str) -> bool {
        match name {
            "label" => {
                self.label = value.to_string();
                true
            }
            "editable" => {
                if let Ok(editable) = value.parse::<bool>() {
                    self.editable = editable;
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
    
    fn get_property_names(&self) -> Vec<String> {
        vec!["label".to_string(), "editable".to_string()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_creation() {
        let button = Button::new("Test Button".to_string());
        assert_eq!(button.label(), "Test Button");
        assert_eq!(button.name(), "Button");
        assert!(!button.is_editable());
    }

    #[test]
    fn test_button_label_modification() {
        let mut button = Button::new("Initial".to_string());
        assert_eq!(button.label(), "Initial");
        
        button.set_label("Modified".to_string());
        assert_eq!(button.label(), "Modified");
    }

    #[test]
    fn test_button_editable_state() {
        let mut button = Button::new("Test".to_string());
        assert!(!button.is_editable());
        
        button.set_editable(true);
        assert!(button.is_editable());
        
        button.set_editable(false);
        assert!(!button.is_editable());
    }

    #[test]
    fn test_button_clone() {
        let button = Button::new("Clone Test".to_string());
        let cloned = button.clone();
        
        assert_eq!(button.label(), cloned.label());
        assert_eq!(button.is_editable(), cloned.is_editable());
        assert_eq!(button.name(), cloned.name());
    }

    #[test]
    fn test_button_component_interface() {
        let button = Button::new("Interface Test".to_string());
        assert_eq!(button.name(), "Button");
        
        // Test that it implements Component trait
        let component: &dyn Component = &button;
        assert_eq!(component.name(), "Button");
    }
}