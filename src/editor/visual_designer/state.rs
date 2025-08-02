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
/// Centralized state management for Visual Designer
///
/// Coordinates state between layout, selection, history, etc.

pub struct DesignerState {
    /// Layout management
    pub layout: super::LayoutManager,
    /// Component selection
    pub selection: super::ComponentSelection,
    /// Design history for undo/redo
    pub history: super::DesignHistory,
    /// Performance metrics
    pub performance: super::PerformanceMetrics,
}

impl DesignerState {
    pub fn new() -> Self {
        Self {
            layout: super::LayoutManager::default(),
            selection: super::ComponentSelection::default(),
            history: super::DesignHistory::default(),
            performance: super::PerformanceMetrics::new(),
        }
    }
}
