use std::collections::HashMap;

/// Design-time properties for components
#[derive(Clone)]
pub struct DesignTimeProperties {
    /// Component name (for code generation)
    pub name: String,
    /// Whether component is locked (cannot be moved/resized)
    pub locked: bool,
    /// Whether component is visible at design time
    pub visible: bool,
    /// Custom properties
    pub custom_props: HashMap<String, String>,
}
//! Centralized state management for Visual Designer
//!
//! Coordinates state between layout, selection, history, etc.

// TODO: Move state management logic here.

pub struct DesignerState {
    // ...fields...
}

impl DesignerState {
    pub fn new() -> Self {
        Self {
            // ...
        }
    }
    // ...state methods...
}
