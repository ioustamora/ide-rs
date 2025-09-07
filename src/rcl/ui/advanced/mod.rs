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
pub mod badge;
pub mod avatar;
pub mod breadcrumb;
pub mod virtual_list;
pub mod toast_notification;
pub mod data_grid;

// Re-export commonly used components
pub use virtual_list::VirtualList;
pub use toast_notification::{ToastManager, Toast, ToastType};
pub use data_grid::{DataGrid, ColumnDefinition, CellValue};
