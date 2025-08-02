//! # Basic UI Components
//!
//! This module provides fundamental UI components that form the building blocks
//! of modern application interfaces. These components offer essential interactive
//! elements with consistent styling, behavior, and extensibility.
//!
//! ## Component Categories
//!
//! ### Text Components
//! - [`label`] - Static and editable text display
//! - [`textbox`] - Single and multi-line text input
//!
//! ### Interactive Controls
//! - [`button`] - Clickable action triggers
//! - [`checkbox`] - Boolean state toggles
//! - [`radio_button`] - Single-choice selection from groups
//! - [`slider`] - Numeric value selection with ranges
//! - [`dropdown`] - List-based option selection
//!
//! ## Design Principles
//!
//! All basic components follow these design principles:
//!
//! - **Consistency**: Uniform styling and behavior patterns
//! - **Accessibility**: Support for keyboard navigation and screen readers
//! - **Responsiveness**: Adaptive layout and sizing
//! - **Editability**: Built-in editing modes for dynamic configuration
//! - **Type Safety**: Compile-time guarantees for component state
//!
//! ## Usage Patterns
//!
//! Basic components are designed to be composed together to create complex
//! interfaces. They integrate seamlessly with the RCL component system and
//! can be used in forms, dialogs, panels, and custom layouts.
//!
//! # Examples
//!
//! ```ignore
//! use crate::rcl::ui::basic::{button::Button, label::Label, checkbox::Checkbox};
//! use crate::rcl::ui::component::Component;
//!
//! // Create basic form components
//! let mut submit_button = Button::new("Submit".to_string());
//! let mut title_label = Label::new("User Settings".to_string());
//! let mut enable_notifications = Checkbox::new("Enable notifications".to_string(), false);
//!
//! // Render in UI context
//! title_label.render(&mut ui);
//! enable_notifications.render(&mut ui);
//! submit_button.render(&mut ui);
//! ```

// Re-export all basic UI components for convenient access
pub mod label;
pub mod button;
pub mod textbox;
pub mod checkbox;
pub mod slider;
pub mod dropdown;
pub mod radio_button;
pub mod form;

// Re-export component types for easier importing
pub use button::Button;
pub use checkbox::Checkbox;
pub use dropdown::Dropdown;
pub use form::Form;
pub use label::Label;
pub use radio_button::RadioButton;
pub use slider::Slider;
pub use textbox::TextBox;
