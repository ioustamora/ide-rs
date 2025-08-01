//! # Label Component
//!
//! A basic text display component that can optionally be made editable.
//! Labels are fundamental UI elements used to display static text or
//! provide editable text fields when interactive functionality is needed.
//!
//! This component supports both read-only text display and in-place editing,
//! making it versatile for various UI scenarios from simple labels to
//! editable captions and form fields.

use egui::Ui;
use crate::rcl::ui::component::Component;

/// A text label component with optional editing capabilities
/// 
/// The Label component displays text content and can be toggled between
/// read-only and editable modes. In read-only mode, it displays static text.
/// In editable mode, it provides a text input field for user interaction.
/// 
/// # Features
/// 
/// - **Static Display**: Shows read-only text content
/// - **In-place Editing**: Can be switched to editable mode
/// - **Toggle Control**: Built-in edit button to switch modes
/// - **Responsive**: Adapts to available space
/// 
/// # Use Cases
/// 
/// - Static text labels in forms and interfaces
/// - Editable captions and titles
/// - Form field labels that can be customized
/// - Dynamic content that may need occasional editing
/// 
/// # Examples
/// 
/// ```ignore
/// use crate::rcl::ui::basic::label::Label;
/// use crate::rcl::ui::component::Component;
/// 
/// let mut label = Label::new("Hello, World!".to_string());
/// 
/// // Render in UI context
/// label.render(&mut ui);
/// 
/// // Make editable
/// label.set_editable(true);
/// ```
pub struct Label {
    /// The text content to display or edit
    /// 
    /// In read-only mode, this text is displayed as-is. In editable mode,
    /// this field can be modified by the user through the text input interface.
    pub text: String,
    
    /// Whether the label is currently in editable mode
    /// 
    /// When `true`, the label displays as a text input field that allows
    /// the user to modify the content. When `false`, the text is displayed
    /// as a read-only label.
    pub editable: bool,
}

impl Label {
    /// Creates a new label with the specified text
    /// 
    /// The label is created in read-only mode by default. Use `set_editable(true)`
    /// or click the "Edit" button to enable editing mode.
    /// 
    /// # Arguments
    /// 
    /// * `text` - The initial text content for the label
    /// 
    /// # Returns
    /// 
    /// A new `Label` instance with the specified text content
    /// 
    /// # Examples
    /// 
    /// ```ignore
    /// let label = Label::new("My Label".to_string());
    /// assert_eq!(label.text(), "My Label");
    /// assert!(!label.is_editable());
    /// ```
    pub fn new(text: String) -> Self {
        Self {
            text,
            editable: false,
        }
    }
    
    /// Sets the editable state of the label
    /// 
    /// # Arguments
    /// 
    /// * `editable` - `true` to enable editing, `false` for read-only mode
    pub fn set_editable(&mut self, editable: bool) {
        self.editable = editable;
    }
    
    /// Returns whether the label is currently editable
    /// 
    /// # Returns
    /// 
    /// `true` if the label is in editable mode, `false` if read-only
    pub fn is_editable(&self) -> bool {
        self.editable
    }
    
    /// Gets the current text content
    /// 
    /// # Returns
    /// 
    /// A reference to the current text content
    pub fn text(&self) -> &str {
        &self.text
    }
    
    /// Sets the text content
    /// 
    /// # Arguments
    /// 
    /// * `text` - The new text content for the label
    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

impl Component for Label {
    /// Returns the component name for identification
    /// 
    /// # Returns
    /// 
    /// The string "Label" identifying this component type
    fn name(&self) -> &str {
        "Label"
    }
    
    /// Renders the label component in the provided UI context
    /// 
    /// The rendering behavior changes based on the `editable` state:
    /// - **Read-only mode**: Displays the text as a static label
    /// - **Editable mode**: Shows a single-line text input field
    /// 
    /// Additionally, an "Edit" button is provided to toggle between modes.
    /// 
    /// # Arguments
    /// 
    /// * `ui` - Mutable reference to the egui UI context for rendering
    /// 
    /// # Layout
    /// 
    /// The component renders vertically with:
    /// 1. The text content (label or text input)
    /// 2. An "Edit" toggle button
    /// 
    /// # User Interaction
    /// 
    /// - Clicking the "Edit" button toggles between read-only and editable modes
    /// - In editable mode, users can modify the text content directly
    /// - Changes are immediately reflected in the component state
    fn render(&mut self, ui: &mut Ui) {
        // Render the main content based on editable state
        if self.editable {
            // Editable mode: show text input field
            ui.text_edit_singleline(&mut self.text);
        } else {
            // Read-only mode: show static label
            ui.label(&self.text);
        }
        
        // Toggle button to switch between modes
        // Button text changes based on current state for clarity
        let button_text = if self.editable { "Done" } else { "Edit" };
        if ui.button(button_text).clicked() {
            // Toggle the editable state
            self.editable = !self.editable;
        }
    }
}
