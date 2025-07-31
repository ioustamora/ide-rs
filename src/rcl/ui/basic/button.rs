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
pub struct Button {
    /// The text displayed on the button face
    pub label: String,
    /// Flag indicating if the button is in edit mode (label can be modified)
    pub editable: bool,
}

impl Component for Button {
    fn name(&self) -> &str {
        "Button"
    }
    
    /// Renders the button component in either display or edit mode
    /// 
    /// In display mode, shows a clickable button. In edit mode, shows a text input
    /// field for modifying the button's label. Always includes an "Edit" toggle button.
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            // Edit mode - show text input for label modification
            ui.text_edit_singleline(&mut self.label);
        } else {
            // Display mode - show clickable button
            if ui.button(&self.label).clicked() {
                // TODO: Add callback/action logic for button clicks
                // This could trigger custom user-defined actions
            }
        }
        
        // Toggle button to switch between edit and display modes
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
}
