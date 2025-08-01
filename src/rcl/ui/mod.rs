//! # RCL User Interface Components
//!
//! This module provides a comprehensive library of UI components for building
//! modern desktop applications. The component system is built on a trait-based
//! architecture that promotes code reuse, consistency, and extensibility.
//!
//! ## Architecture
//!
//! The UI module is organized into three main categories:
//!
//! ### Component Framework
//! - [`component`] - Core component trait and shared functionality
//!
//! ### Component Libraries
//! - [`basic`] - Fundamental UI elements (buttons, labels, inputs, etc.)
//! - [`advanced`] - Complex components (charts, editors, panels, etc.)
//!
//! ## Component Trait System
//!
//! All UI components implement the [`Component`] trait, which provides:
//! - **Consistent Rendering**: Uniform render method signature
//! - **Type Identification**: Component name and type information  
//! - **Composability**: Components can contain other components
//! - **Event Handling**: Integrated interaction and state management
//!
//! ## Design Philosophy
//!
//! The RCL UI system follows these core principles:
//!
//! - **Modularity**: Components are self-contained and reusable
//! - **Consistency**: Uniform behavior and styling across all components
//! - **Flexibility**: Support for both simple and complex use cases
//! - **Performance**: Efficient rendering and minimal memory overhead
//! - **Developer Experience**: Intuitive APIs with comprehensive documentation
//!
//! ## Usage Patterns
//!
//! Components can be used in several ways:
//!
//! ### Direct Usage
//! Create and render individual components in immediate mode GUI contexts.
//!
//! ### Composition
//! Combine multiple components to create complex interfaces and custom widgets.
//!
//! ### Inheritance
//! Extend existing components to add specialized behavior or styling.
//!
//! ## Integration
//!
//! The UI components integrate seamlessly with:
//! - **egui**: Modern immediate mode GUI framework
//! - **RCL System**: System services (clipboard, file system, etc.)
//! - **RCL Network**: Network components for data-driven interfaces
//! - **Visual Designer**: Drag-and-drop interface builder
//!
//! # Examples
//!
//! ```ignore
//! use crate::rcl::ui::{basic::Button, component::Component};
//! 
//! // Create a simple button
//! let mut button = Button::new("Click Me".to_string());
//! 
//! // Render in egui context
//! button.render(&mut ui);
//! 
//! // Components can be composed
//! struct MyPanel {
//!     submit_button: Button,
//!     cancel_button: Button,
//! }
//! 
//! impl Component for MyPanel {
//!     fn name(&self) -> &str { "MyPanel" }
//!     
//!     fn render(&mut self, ui: &mut egui::Ui) {
//!         ui.horizontal(|ui| {
//!             self.submit_button.render(ui);
//!             self.cancel_button.render(ui);
//!         });
//!     }
//! }
//! ```

/// Core component trait and framework
/// 
/// Defines the fundamental [`Component`] trait that all UI elements implement,
/// providing the foundation for the RCL component system.
pub mod component;

/// Basic UI components
/// 
/// Fundamental interactive elements like buttons, labels, text inputs, and
/// form controls that serve as building blocks for application interfaces.
pub mod basic;

/// Advanced UI components
/// 
/// Complex components like charts, editors, file pickers, and specialized
/// panels that provide rich functionality for sophisticated applications.
pub mod advanced;

// Re-export the core component trait for convenience
pub use component::Component;
