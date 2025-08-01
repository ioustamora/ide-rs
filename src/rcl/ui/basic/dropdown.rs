//! # Dropdown Component
//!
//! A selection component that presents a list of options in a collapsible menu.
//! Dropdowns provide an efficient way to present multiple choices while conserving
//! screen space, showing only the selected option until expanded.
//!
//! This component supports both interactive selection mode and label editing mode,
//! making it suitable for forms, settings panels, and dynamic option lists.

use egui::Ui;
use crate::rcl::ui::component::Component;

/// A dropdown selection component with customizable options and editable label
/// 
/// The Dropdown component provides a space-efficient way to present multiple
/// selection options. Users can choose from a list of predefined options,
/// and the component can switch between selection mode and label editing mode.
/// 
/// # Features
/// 
/// - **Option Selection**: Choose from a list of predefined options
/// - **Space Efficient**: Collapsible interface that shows only selected value
/// - **Editable Label**: Label text can be modified in edit mode
/// - **Index Tracking**: Maintains selected option index for easy access
/// - **Dynamic Options**: Support for runtime option list modifications
/// 
/// # Use Cases
/// 
/// - Form field selections (country, category, etc.)
/// - Settings and preferences with predefined values
/// - Filter controls and sorting options
/// - Language and theme selectors
/// - Any scenario requiring single selection from multiple options
/// 
/// # Examples
/// 
/// ```ignore
/// use crate::rcl::ui::basic::dropdown::Dropdown;
/// use crate::rcl::ui::component::Component;
/// 
/// let options = vec!["Option 1".to_string(), "Option 2".to_string(), "Option 3".to_string()];
/// let mut dropdown = Dropdown::new("Choose an option".to_string(), options, 0);
/// 
/// // Render in UI context
/// dropdown.render(&mut ui);
/// 
/// // Change selection programmatically
/// dropdown.set_selected(1);
/// ```
pub struct Dropdown {
    /// The label text displayed with the dropdown
    /// 
    /// This label provides context for what the dropdown represents.
    /// In edit mode, users can modify this text to customize the label.
    pub label: String,
    
    /// The list of available options for selection
    /// 
    /// Each string in this vector represents a selectable option that will
    /// be displayed in the dropdown menu when expanded.
    pub options: Vec<String>,
    
    /// The index of the currently selected option
    /// 
    /// This index corresponds to the position in the `options` vector.
    /// Must be a valid index within the bounds of the options vector.
    pub selected: usize,
    
    /// Whether the component is in label-editing mode
    /// 
    /// When `true`, the label can be edited via a text input field.
    /// When `false`, the component displays as an interactive dropdown.
    pub editable: bool,
}

impl Dropdown {
    /// Creates a new dropdown with the specified label, options, and selection
    /// 
    /// The dropdown is created in interactive mode by default. Use `set_editable(true)`
    /// or click the "Edit" button to enable label editing.
    /// 
    /// # Arguments
    /// 
    /// * `label` - The text label for the dropdown
    /// * `options` - Vector of option strings to display in the dropdown
    /// * `selected` - Index of the initially selected option
    /// 
    /// # Returns
    /// 
    /// A new `Dropdown` instance with the specified parameters
    /// 
    /// # Panics
    /// 
    /// Panics if `selected` is out of bounds for the options vector,
    /// or if the options vector is empty.
    /// 
    /// # Examples
    /// 
    /// ```ignore
    /// let options = vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()];
    /// let dropdown = Dropdown::new("Color".to_string(), options, 0);
    /// assert_eq!(dropdown.label(), "Color");
    /// assert_eq!(dropdown.selected_index(), 0);
    /// assert_eq!(dropdown.selected_text(), "Red");
    /// ```
    pub fn new(label: String, options: Vec<String>, selected: usize) -> Self {
        assert!(!options.is_empty(), "Dropdown options cannot be empty");
        assert!(selected < options.len(), "Selected index out of bounds");
        
        Self {
            label,
            options,
            selected,
            editable: false,
        }
    }
    
    /// Sets the editable state of the dropdown label
    /// 
    /// # Arguments
    /// 
    /// * `editable` - `true` to enable label editing, `false` for normal dropdown mode
    pub fn set_editable(&mut self, editable: bool) {
        self.editable = editable;
    }
    
    /// Returns whether the dropdown is in label-editing mode
    /// 
    /// # Returns
    /// 
    /// `true` if label editing is enabled, `false` if in normal dropdown mode
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
    /// * `label` - The new label text for the dropdown
    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }
    
    /// Gets the currently selected option index
    /// 
    /// # Returns
    /// 
    /// The index of the selected option in the options vector
    pub fn selected_index(&self) -> usize {
        self.selected
    }
    
    /// Gets the text of the currently selected option
    /// 
    /// # Returns
    /// 
    /// A reference to the text of the currently selected option
    /// 
    /// # Panics
    /// 
    /// Panics if the selected index is out of bounds (should not happen
    /// if the component is used correctly)
    pub fn selected_text(&self) -> &str {
        &self.options[self.selected]
    }
    
    /// Sets the selected option by index
    /// 
    /// # Arguments
    /// 
    /// * `index` - The index of the option to select
    /// 
    /// # Panics
    /// 
    /// Panics if the index is out of bounds for the options vector
    pub fn set_selected(&mut self, index: usize) {
        assert!(index < self.options.len(), "Selected index out of bounds");
        self.selected = index;
    }
    
    /// Sets the selected option by matching text value
    /// 
    /// # Arguments
    /// 
    /// * `text` - The text of the option to select
    /// 
    /// # Returns
    /// 
    /// `true` if the option was found and selected, `false` otherwise
    pub fn set_selected_by_text(&mut self, text: &str) -> bool {
        if let Some(index) = self.options.iter().position(|opt| opt == text) {
            self.selected = index;
            true
        } else {
            false
        }
    }
    
    /// Gets a reference to the options vector
    /// 
    /// # Returns
    /// 
    /// A reference to the vector containing all available options
    pub fn options(&self) -> &Vec<String> {
        &self.options
    }
    
    /// Sets new options for the dropdown, resetting selection to first option
    /// 
    /// # Arguments
    /// 
    /// * `options` - Vector of new option strings
    /// 
    /// # Panics
    /// 
    /// Panics if the options vector is empty
    pub fn set_options(&mut self, options: Vec<String>) {
        assert!(!options.is_empty(), "Dropdown options cannot be empty");
        self.options = options;
        self.selected = 0; // Reset to first option
    }
    
    /// Adds a new option to the dropdown
    /// 
    /// # Arguments
    /// 
    /// * `option` - The new option text to add
    pub fn add_option(&mut self, option: String) {
        self.options.push(option);
    }
    
    /// Removes an option by index
    /// 
    /// # Arguments
    /// 
    /// * `index` - The index of the option to remove
    /// 
    /// # Returns
    /// 
    /// The removed option text
    /// 
    /// # Panics
    /// 
    /// Panics if the index is out of bounds or if removing the option
    /// would leave the dropdown empty
    pub fn remove_option(&mut self, index: usize) -> String {
        assert!(index < self.options.len(), "Remove index out of bounds");
        assert!(self.options.len() > 1, "Cannot remove last option");
        
        let removed = self.options.remove(index);
        
        // Adjust selected index if necessary
        if self.selected >= index && self.selected > 0 {
            self.selected -= 1;
        } else if self.selected >= self.options.len() {
            self.selected = self.options.len() - 1;
        }
        
        removed
    }
    
    /// Returns the number of available options
    /// 
    /// # Returns
    /// 
    /// The count of options in the dropdown
    pub fn option_count(&self) -> usize {
        self.options.len()
    }
}

impl Component for Dropdown {
    /// Returns the component type name
    /// 
    /// # Returns
    /// 
    /// The string "Dropdown" identifying this component type
    fn name(&self) -> &str {
        "Dropdown"
    }
    
    /// Renders the dropdown component in either selection or edit mode
    /// 
    /// The rendering behavior depends on the current editable state:
    /// - **Selection mode**: Shows an interactive dropdown with selectable options
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
    /// 1. The main control (dropdown or text input)
    /// 2. An "Edit" toggle button
    /// 
    /// # User Interaction
    /// 
    /// - In selection mode, clicking the dropdown opens the option list
    /// - Users can select any option from the expanded list
    /// - Selected option becomes the displayed value
    /// - The "Edit" button switches between selection and label editing modes
    /// - Changes are immediately reflected in the component state
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            // Edit mode - show text input for label modification
            ui.text_edit_singleline(&mut self.label);
            // TODO: Future enhancement could allow editing the options themselves
            // This would require a more complex UI for adding/removing/editing individual options
        } else {
            // Selection mode - show interactive dropdown
            // The ComboBox provides:
            // - Collapsible interface showing current selection
            // - Expandable list of selectable options
            // - Automatic selection state management
            // - Keyboard navigation support
            if !self.options.is_empty() && self.selected < self.options.len() {
                egui::ComboBox::from_label(&self.label)
                    .selected_text(&self.options[self.selected])
                    .show_ui(ui, |ui| {
                        // Render all options as selectable items
                        for (i, option) in self.options.iter().enumerate() {
                            ui.selectable_value(&mut self.selected, i, option);
                        }
                    });
            } else {
                // Fallback for invalid state - should not normally occur
                ui.label(format!("Invalid dropdown state: {} options, selected {}", 
                    self.options.len(), self.selected));
            }
        }
        
        // Toggle button to switch between modes
        // Button text changes to provide clear state indication
        let edit_button_text = if self.editable { "Done" } else { "Edit" };
        if ui.button(edit_button_text).clicked() {
            self.editable = !self.editable;
        }
    }
}
