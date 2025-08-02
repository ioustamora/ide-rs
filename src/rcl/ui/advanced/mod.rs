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
pub use floating_panel::FloatingPanel;
pub use progress_bar::ProgressBar;
pub use tabs::Tabs;
pub use table::Table;
pub use tree::{Tree, TreeNode};
pub use modal::Modal;
pub use notification::{Notification, NotificationKind};
pub use calendar::Calendar;
pub use color_picker::ColorPicker;
pub use file_picker::FilePicker;
pub use image::Image;
pub use menu::Menu;
pub use split::Split;
pub use status_bar::StatusBar;
pub use toolbar::Toolbar;
pub use chart::{Chart, ChartType, ChartConfig, ChartData, DataSeries, SeriesStyle, PointShape, ChartAnimation};
