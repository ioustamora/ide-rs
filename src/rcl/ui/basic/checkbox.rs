//! # Checkbox Component
//!
//! A boolean input component with an editable label. Checkboxes are fundamental
//! UI elements for binary choices, settings toggles, and form selections.
//!
//! This component supports both interactive checkbox functionality and in-place
//! label editing, making it suitable for both static forms and dynamic
//! configuration interfaces.

use egui::Ui;
use crate::rcl::ui::component::Component;

/// A checkbox component with toggleable state and editable label
/// 
/// The Checkbox component provides a standard checkbox interface with the ability
/// to toggle between checked/unchecked states and edit the associated label text.
/// 
/// # Features
/// 
/// - **Boolean State**: Toggleable checked/unchecked state
/// - **Editable Label**: Label text can be modified in edit mode
/// - **Visual Feedback**: Clear indication of current state
/// - **Mode Switching**: Toggle between interaction and editing modes
/// 
/// # Use Cases
/// 
/// - Form controls for boolean options
/// - Settings toggles in configuration panels
/// - Feature enable/disable switches
/// - Multi-selection lists and filters
/// 
/// # Examples
/// 
/// ```ignore
/// use crate::rcl::ui::basic::checkbox::Checkbox;
/// use crate::rcl::ui::component::Component;
/// 
/// let mut checkbox = Checkbox::new("Enable notifications".to_string(), false);
/// 
/// // Render in UI context
/// checkbox.render(&mut ui);
/// 
/// // Check programmatically
/// checkbox.set_checked(true);
/// ```
pub struct Checkbox {
    /// The text label displayed next to the checkbox
    /// 
    /// This label provides context for what the checkbox controls.
    /// In edit mode, users can modify this text to customize the label.
    pub label: String,
    
    /// The current checked state of the checkbox
    /// 
    /// `true` indicates the checkbox is checked (selected), `false` indicates
    /// it is unchecked (not selected). Users can toggle this by clicking
    /// the checkbox in normal mode.
    pub checked: bool,
    
    /// Whether the component is in label-editing mode
    /// 
    /// When `true`, the label can be edited via a text input field.
    /// When `false`, the component displays as a normal interactive checkbox.
    pub editable: bool,
}

impl Checkbox {
    /// Creates a new checkbox with the specified label and initial state
    /// 
    /// The checkbox is created in interactive mode by default. Use `set_editable(true)`
    /// or click the "Edit" button to enable label editing.
    /// 
    /// # Arguments
    /// 
    /// * `label` - The text to display next to the checkbox
    /// * `checked` - The initial checked state (`true` for checked, `false` for unchecked)
    /// 
    /// # Returns
    /// 
    /// A new `Checkbox` instance with the specified label and state
    /// 
    /// # Examples
    /// 
    /// ```ignore
    /// let checkbox = Checkbox::new("Accept terms".to_string(), false);
    /// assert_eq!(checkbox.label(), "Accept terms");
    /// assert!(!checkbox.is_checked());
    /// assert!(!checkbox.is_editable());
    /// ```
    pub fn new(label: String, checked: bool) -> Self {
        Self {
            label,
            checked,
            editable: false,
        }
    }
    
    /// Sets the checked state of the checkbox
    /// 
    /// # Arguments
    /// 
    /// * `checked` - `true` to check the checkbox, `false` to uncheck it
    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }
    
    /// Returns the current checked state
    /// 
    /// # Returns
    /// 
    /// `true` if the checkbox is checked, `false` if unchecked
    pub fn is_checked(&self) -> bool {
        self.checked
    }
    
    /// Sets the editable state of the checkbox label
    /// 
    /// # Arguments
    /// 
    /// * `editable` - `true` to enable label editing, `false` for normal mode
    pub fn set_editable(&mut self, editable: bool) {
        self.editable = editable;
    }
    
    /// Returns whether the checkbox is in label-editing mode
    /// 
    /// # Returns
    /// 
    /// `true` if label editing is enabled, `false` if in normal mode
    pub fn is_editable(&self) -> bool {
        self.editable
    }
    
    /// Gets the current label text
    /// 
    /// # Returns
    /// 
    /// A reference to the current label text
    pub fn label(&self) -> &str {
        &self.label
    }
    
    /// Sets the label text
    /// 
    /// # Arguments
    /// 
    /// * `label` - The new label text for the checkbox
    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }
}

impl Component for Checkbox {
    /// Returns the component type name
    /// 
    /// # Returns
    /// 
    /// The string "Checkbox" identifying this component type
    fn name(&self) -> &str {
        "Checkbox"
    }
    
    /// Renders the checkbox component in either normal or edit mode
    /// 
    /// The rendering behavior depends on the current editable state:
    /// - **Normal mode**: Shows an interactive checkbox with label
    /// - **Edit mode**: Shows a text input field for label modification
    /// 
    /// An "Edit" toggle button is provided to switch between modes.
    /// 
    /// # Arguments
    /// 
    /// * `ui` - Mutable reference to the egui UI context for rendering
    /// 
    /// # Layout
    /// 
    /// The component renders vertically with:
    /// 1. The main content (checkbox or text input)
    /// 2. An "Edit" toggle button
    /// 
    /// # User Interaction
    /// 
    /// - In normal mode, clicking the checkbox toggles the checked state
    /// - In edit mode, users can modify the label text
    /// - The "Edit" button switches between interaction and editing modes
    /// - State changes are immediately reflected in the component
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            // Edit mode - show text input for label modification
            ui.text_edit_singleline(&mut self.label);
        } else {
            // Display mode - show interactive checkbox
            ui.checkbox(&mut self.checked, &self.label);
        }
        
        // Toggle button to switch between edit and normal modes
        // Button text provides clear indication of current state
        let edit_button_text = if self.editable { "Done" } else { "Edit" };
        if ui.button(edit_button_text).clicked() {
            self.editable = !self.editable;
        }
    }
    
    fn get_property(&self, name: &str) -> Option<String> {
        match name {
            "label" => Some(self.label.clone()),
            "checked" => Some(self.checked.to_string()),
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
            "checked" => {
                if let Ok(checked) = value.parse::<bool>() {
                    self.checked = checked;
                    true
                } else {
                    false
                }
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
        vec!["label".to_string(), "checked".to_string(), "editable".to_string()]
    }
}
