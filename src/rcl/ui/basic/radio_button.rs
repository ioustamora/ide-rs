//! # RadioButton Component
//!
//! A single radio button component for mutually exclusive selections.
//! Radio buttons are typically used in groups where only one option can be
//! selected at a time, providing clear visual feedback for exclusive choices.
//!
//! This component supports both interactive selection mode and label editing mode,
//! making it suitable for forms, settings panels, and option selection interfaces.

use egui::Ui;
use crate::rcl::ui::component::Component;

/// A radio button component with selectable state and editable label
/// 
/// The RadioButton component provides a standard radio button interface that
/// can be selected or deselected. It's designed to work as part of a group
/// where only one radio button should be selected at a time.
/// 
/// # Features
/// 
/// - **Exclusive Selection**: Designed for mutually exclusive option groups
/// - **Visual State**: Clear indication of selected/deselected state
/// - **Editable Label**: Label text can be modified in edit mode
/// - **Toggle Behavior**: Can be selected and deselected programmatically
/// 
/// # Use Cases
/// 
/// - Form controls for single-choice questions
/// - Settings with mutually exclusive options
/// - Preference selections (theme, language, etc.)
/// - Survey and questionnaire interfaces
/// - Configuration panels with exclusive choices
/// 
/// # Group Management
/// 
/// For proper radio button group behavior, manage multiple RadioButton
/// components together, ensuring only one is selected at a time.
/// 
/// # Examples
/// 
/// ```ignore
/// use crate::rcl::ui::basic::radio_button::RadioButton;
/// use crate::rcl::ui::component::Component;
/// 
/// let mut radio1 = RadioButton::new("Option A".to_string(), true);
/// let mut radio2 = RadioButton::new("Option B".to_string(), false);
/// 
/// // Render in UI context
/// radio1.render(&mut ui);
/// radio2.render(&mut ui);
/// 
/// // Select programmatically
/// radio2.set_selected(true);
/// radio1.set_selected(false); // Deselect others in group
/// ```
pub struct RadioButton {
    /// The text label displayed next to the radio button
    /// 
    /// This label provides context for what selecting this radio button means.
    /// In edit mode, users can modify this text to customize the label.
    pub label: String,
    
    /// Whether this radio button is currently selected
    /// 
    /// In a properly managed radio button group, only one button should
    /// have `selected = true` at any given time.
    pub selected: bool,
    
    /// Whether the component is in label-editing mode
    /// 
    /// When `true`, the label can be edited via a text input field.
    /// When `false`, the component displays as an interactive radio button.
    pub editable: bool,
}

impl RadioButton {
    /// Creates a new radio button with the specified label and selection state
    /// 
    /// The radio button is created in interactive mode by default. Use `set_editable(true)`
    /// or click the "Edit" button to enable label editing.
    /// 
    /// # Arguments
    /// 
    /// * `label` - The text to display next to the radio button
    /// * `selected` - The initial selection state (`true` for selected, `false` for deselected)
    /// 
    /// # Returns
    /// 
    /// A new `RadioButton` instance with the specified label and selection state
    /// 
    /// # Examples
    /// 
    /// ```ignore
    /// let radio = RadioButton::new("Enable feature".to_string(), false);
    /// assert_eq!(radio.label(), "Enable feature");
    /// assert!(!radio.is_selected());
    /// assert!(!radio.is_editable());
    /// ```
    pub fn new(label: String, selected: bool) -> Self {
        Self {
            label,
            selected,
            editable: false,
        }
    }
    
    /// Sets the selection state of the radio button
    /// 
    /// # Arguments
    /// 
    /// * `selected` - `true` to select the radio button, `false` to deselect it
    /// 
    /// # Note
    /// 
    /// When implementing radio button groups, ensure that selecting one button
    /// deselects all others in the same group.
    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
    
    /// Returns the current selection state
    /// 
    /// # Returns
    /// 
    /// `true` if the radio button is selected, `false` if deselected
    pub fn is_selected(&self) -> bool {
        self.selected
    }
    
    /// Toggles the selection state
    /// 
    /// # Note
    /// 
    /// This method is provided for convenience but should be used carefully
    /// in radio button groups to maintain exclusive selection behavior.
    pub fn toggle(&mut self) {
        self.selected = !self.selected;
    }
    
    /// Sets the editable state of the radio button label
    /// 
    /// # Arguments
    /// 
    /// * `editable` - `true` to enable label editing, `false` for normal mode
    pub fn set_editable(&mut self, editable: bool) {
        self.editable = editable;
    }
    
    /// Returns whether the radio button is in label-editing mode
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
    /// * `label` - The new label text for the radio button
    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }
}

impl Component for RadioButton {
    /// Returns the component type name
    /// 
    /// # Returns
    /// 
    /// The string "RadioButton" identifying this component type
    fn name(&self) -> &str {
        "RadioButton"
    }
    
    /// Renders the radio button component in either selection or edit mode
    /// 
    /// The rendering behavior depends on the current editable state:
    /// - **Selection mode**: Shows an interactive radio button with label
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
    /// 1. The main control (radio button or text input)
    /// 2. An "Edit" toggle button
    /// 
    /// # User Interaction
    /// 
    /// - In selection mode, clicking the radio button toggles its selected state
    /// - The radio button provides visual feedback for its current state
    /// - In edit mode, users can modify the label text
    /// - The "Edit" button switches between selection and editing modes
    /// - State changes are immediately reflected in the component
    /// 
    /// # Radio Button Group Behavior
    /// 
    /// The component uses `radio_value` which compares the current state
    /// with the target value (true). This allows for proper radio button
    /// group behavior when managed externally.
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            // Edit mode - show text input for label modification
            ui.text_edit_singleline(&mut self.label);
        } else {
            // Selection mode - show interactive radio button
            // Using radio_value provides standard radio button behavior:
            // - Visual indication of selected/deselected state
            // - Click handling for state toggling
            // - Consistent styling with other radio buttons
            // - Keyboard accessibility support
            ui.radio_value(&mut self.selected, true, &self.label);
        }
        
        // Toggle button to switch between modes
        // Button text changes to provide clear state indication
        let edit_button_text = if self.editable { "Done" } else { "Edit" };
        if ui.button(edit_button_text).clicked() {
            self.editable = !self.editable;
        }
    }
    
    fn get_property(&self, name: &str) -> Option<String> {
        match name {
            "label" => Some(self.label.clone()),
            "selected" => Some(self.selected.to_string()),
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
            "selected" => {
                if let Ok(selected) = value.parse::<bool>() {
                    self.selected = selected;
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
        vec!["label".to_string(), "selected".to_string(), "editable".to_string()]
    }
}
