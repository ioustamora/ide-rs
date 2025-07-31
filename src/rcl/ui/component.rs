//! Shared Component trait for all UI components in the Rust Component Library (RCL)
//!
//! This module defines the core Component trait that all RCL UI components must implement.
//! It provides a consistent interface for rendering components and accessing their metadata.

use egui::Ui;

/// Core trait that all RCL UI components must implement
/// 
/// This trait provides the fundamental interface for:
/// - Component identification via the `name()` method
/// - Component rendering via the `render()` method
/// 
/// All UI components in the RCL (buttons, labels, textboxes, etc.) implement this trait
/// to ensure consistent behavior and integration with the IDE's component system.
#[allow(dead_code)]
pub trait Component {
    /// Returns the human-readable name of this component type
    /// 
    /// Used for display in the Properties Inspector and Component Palette
    fn name(&self) -> &str;
    
    /// Renders this component within the provided egui UI context
    /// 
    /// This method is called by the IDE's rendering system to display
    /// the component in the design canvas or property editor.
    /// 
    /// # Arguments
    /// * `ui` - The egui UI context for rendering
    fn render(&mut self, ui: &mut Ui);
}
