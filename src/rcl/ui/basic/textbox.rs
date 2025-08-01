//! # TextBox Component
//!
//! A multi-line text input component with read-only and editable modes.
//! TextBoxes are essential UI elements for entering and displaying larger
//! amounts of text content, supporting both read-only display and interactive editing.
//!
//! This component is ideal for text areas, content editors, notes fields,
//! and any scenario requiring multi-line text input with optional editing controls.

use egui::Ui;
use crate::rcl::ui::component::Component;

/// A multi-line text input component with toggleable editing mode
/// 
/// The TextBox component provides a flexible text area that can switch between
/// read-only display mode and interactive editing mode. It supports multi-line
/// text content and provides built-in editing controls.
/// 
/// # Features
/// 
/// - **Multi-line Support**: Handles text content with line breaks
/// - **Dual Modes**: Read-only display and interactive editing
/// - **Toggle Control**: Built-in edit button for mode switching
/// - **Text Preservation**: Maintains content when switching modes
/// 
/// # Use Cases
/// 
/// - Multi-line text input forms
/// - Code snippets and documentation display
/// - Notes and comments sections
/// - Content editing interfaces
/// - Configuration text areas
/// 
/// # Examples
/// 
/// ```ignore
/// use crate::rcl::ui::basic::textbox::TextBox;
/// use crate::rcl::ui::component::Component;
/// 
/// let mut textbox = TextBox::new("Hello\nWorld".to_string());
/// 
/// // Render in UI context
/// textbox.render(&mut ui);
/// 
/// // Make editable
/// textbox.set_editable(true);
/// ```
pub struct TextBox {
    /// The text content displayed or edited in the text box
    /// 
    /// This field contains the full text content including line breaks.
    /// In read-only mode, this text is displayed as-is. In editable mode,
    /// users can modify this content through the text area interface.
    pub value: String,
    
    /// Whether the text box is currently in editable mode
    /// 
    /// When `true`, the text box displays as an interactive multi-line
    /// text area that allows content modification. When `false`, the
    /// content is displayed as read-only text.
    pub editable: bool,
}

impl TextBox {
    /// Creates a new text box with the specified content
    /// 
    /// The text box is created in read-only mode by default. Use `set_editable(true)`
    /// or click the "Edit" button to enable text editing.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The initial text content for the text box
    /// 
    /// # Returns
    /// 
    /// A new `TextBox` instance with the specified content
    /// 
    /// # Examples
    /// 
    /// ```ignore
    /// let textbox = TextBox::new("Line 1\nLine 2\nLine 3".to_string());
    /// assert_eq!(textbox.value(), "Line 1\nLine 2\nLine 3");
    /// assert!(!textbox.is_editable());
    /// ```
    pub fn new(value: String) -> Self {
        Self {
            value,
            editable: false,
        }
    }
    
    /// Sets the editable state of the text box
    /// 
    /// # Arguments
    /// 
    /// * `editable` - `true` to enable text editing, `false` for read-only mode
    pub fn set_editable(&mut self, editable: bool) {
        self.editable = editable;
    }
    
    /// Returns whether the text box is currently editable
    /// 
    /// # Returns
    /// 
    /// `true` if the text box is in editable mode, `false` if read-only
    pub fn is_editable(&self) -> bool {
        self.editable
    }
    
    /// Gets the current text content
    /// 
    /// # Returns
    /// 
    /// A reference to the current text content
    pub fn value(&self) -> &str {
        &self.value
    }
    
    /// Sets the text content
    /// 
    /// # Arguments
    /// 
    /// * `value` - The new text content for the text box
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
    
    /// Appends text to the current content
    /// 
    /// # Arguments
    /// 
    /// * `text` - The text to append to the current content
    pub fn append_text(&mut self, text: &str) {
        self.value.push_str(text);
    }
    
    /// Clears all text content
    pub fn clear(&mut self) {
        self.value.clear();
    }
    
    /// Returns the number of lines in the text content
    /// 
    /// # Returns
    /// 
    /// The number of lines (including empty lines) in the text
    pub fn line_count(&self) -> usize {
        self.value.lines().count().max(1)
    }
    
    /// Checks if the text box is empty
    /// 
    /// # Returns
    /// 
    /// `true` if the text content is empty, `false` otherwise
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

impl Component for TextBox {
    /// Returns the component type name
    /// 
    /// # Returns
    /// 
    /// The string "TextBox" identifying this component type
    fn name(&self) -> &str {
        "TextBox"
    }
    
    /// Renders the text box component in either read-only or editable mode
    /// 
    /// The rendering behavior depends on the current editable state:
    /// - **Read-only mode**: Displays the text content as a multi-line label
    /// - **Editable mode**: Shows an interactive multi-line text area
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
    /// 1. The main content area (text display or text editor)
    /// 2. An "Edit" toggle button
    /// 
    /// # User Interaction
    /// 
    /// - In read-only mode, text is displayed but cannot be modified
    /// - In editable mode, users can modify text content with full text editing capabilities
    /// - The "Edit" button toggles between read-only and editable modes
    /// - Text changes are immediately reflected in the component state
    /// - Supports standard text editing operations (select, copy, paste, etc.)
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            // Editable mode - show multi-line text editor
            // Provides full text editing capabilities including:
            // - Text selection and highlighting
            // - Copy/paste operations
            // - Multi-line editing with line breaks
            // - Cursor positioning and navigation
            ui.text_edit_multiline(&mut self.value);
        } else {
            // Read-only mode - display text content as label
            // Preserves formatting and line breaks but prevents editing
            ui.label(&self.value);
        }
        
        // Toggle button to switch between modes
        // Button text changes to provide clear state indication
        let edit_button_text = if self.editable { "Done" } else { "Edit" };
        if ui.button(edit_button_text).clicked() {
            self.editable = !self.editable;
        }
    }
}
