//! Advanced UI components for RCL
//! 
//! This module contains sophisticated UI components that provide
//! enhanced functionality beyond basic widgets.

pub mod floating_panel;
pub mod layout_manager;
pub mod rich_text_editor;
pub mod code_editor;
pub mod progress_bar;
pub mod tabs;
pub mod table;
pub mod tree;
pub mod modal;
pub mod notification;
pub mod calendar;
pub mod color_picker;
pub mod file_picker;
pub mod image;
pub mod menu;
pub mod split;
pub mod status_bar;
pub mod toolbar;
pub mod chart;

// Re-export commonly used components
pub use layout_manager::{LayoutManager, LayoutType};
pub use rich_text_editor::RichTextEditor;
pub use code_editor::CodeEditor;
