//! Button component for the Rust Component Library (RCL)
//!
//! This module provides a basic button component that supports both display and edit modes.
//! The button can be clicked to trigger actions or edited to change its label text.

use egui::Ui;
use crate::rcl::ui::component::Component;

/// A clickable button component with editable label
/// 
/// The Button component supports two modes:
/// - Display mode: Shows a clickable button with the specified label
/// - Edit mode: Allows in-place editing of the button's label text
/// 
/// # Fields
/// * `label` - The text displayed on the button
/// * `editable` - Whether the button is currently in edit mode
/// 
/// # Examples
/// 
/// ```ignore
/// use crate::rcl::ui::basic::button::Button;
/// use crate::rcl::ui::component::Component;
/// 
/// let mut button = Button::new("Click Me".to_string());
/// 
/// // Render in UI context
/// button.render(&mut ui);
/// 
/// // Make editable
/// button.set_editable(true);
/// ```
pub struct Button {
    /// The text displayed on the button face
    pub label: String,
    /// Flag indicating if the button is in edit mode (label can be modified)
    pub editable: bool,
}

impl Button {
    /// Creates a new button with the specified label
    /// 
    /// The button is created in display mode by default. Use `set_editable(true)`
    /// or click the "Edit" button to enable label editing.
    /// 
    /// # Arguments
    /// 
    /// * `label` - The initial text to display on the button
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
    /// `true` if the button label can be edited, `false` if in normal mode
    pub fn is_editable(&self) -> bool {
        self.editable
    }
    
    /// Gets the current button label
    /// 
    /// # Returns
    /// 
    /// A reference to the current button label text
    pub fn label(&self) -> &str {
        &self.label
    }
    
    /// Sets the button label
    /// 
    /// # Arguments
    /// 
    /// * `label` - The new text to display on the button
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
    /// An "Edit" toggle button is always shown to switch between modes.
    /// 
    /// # Arguments
    /// 
    /// * `ui` - Mutable reference to the egui UI context for rendering
    /// 
    /// # Layout
    /// 
    /// The component renders vertically with:
    /// 1. The main button (clickable) or text input (editable)
    /// 2. An "Edit" toggle button
    /// 
    /// # User Interaction
    /// 
    /// - In display mode, clicking the main button generates a click event
    /// - In edit mode, users can modify the label text directly
    /// - The "Edit" button toggles between the two modes
    /// - Button text updates dynamically based on current state
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            // Edit mode - show text input for label modification
            ui.text_edit_singleline(&mut self.label);
        } else {
            // Display mode - show clickable button
            if ui.button(&self.label).clicked() {
                // TODO: Add callback/action logic for button clicks
                // This could trigger custom user-defined actions
                // Future enhancement: support for click handlers
            }
        }
        
        // Toggle button to switch between edit and display modes
        // Button text changes to provide clear state indication
        let edit_button_text = if self.editable { "Done" } else { "Edit" };
        if ui.button(edit_button_text).clicked() {
            self.editable = !self.editable;
        }
    }
}
