//! # Advanced Visual Form Designer
//!
//! Professional WYSIWYG visual designer inspired by RAD Studio and Delphi's form designer.
//! This module implements a comprehensive visual editing system that enables drag-and-drop
//! component placement with precision tools and professional-grade layout capabilities.
//!
//! ## Core Features
//!
//! - **Grid System**: Configurable grid with snap-to-grid functionality
//! - **Multi-Selection**: Advanced selection system supporting multiple components
//! - **Alignment Tools**: Professional alignment and distribution operations
//! - **Visual Guides**: Dynamic alignment guides and rulers for precision layout
//! - **Undo/Redo**: Complete operation history with atomic transactions
//! - **Smart Editing**: Intelligent editing assistance and magnetic alignment
//! - **Performance**: Spatial indexing for efficient hit-testing and rendering
//!
//! ## Architecture Overview
//!
//! The visual designer follows a component-based architecture where each system
//! handles a specific aspect of the visual editing experience:
//!
//! ```
//! VisualDesigner (main coordinator)
//!     ├── GridSettings (snap-to-grid system)
//!     ├── ComponentSelection (multi-select management)
//!     ├── DesignHistory (undo/redo operations)
//!     ├── GuideSystem (visual alignment aids)
//!     ├── LayoutManager (spatial indexing and positioning)
//!     ├── SmartEditingSystem (intelligent assistance)
//!     ├── PerformanceMetrics (performance and caching)
//!     ├── AccessibilityValidator (accessibility and WCAG)
//!     └── DesignerState (centralized state)
//! ```
//!
//! ## Modular Architecture
//!
//! All logic is now implemented in submodules:
//! - layout.rs: layout management
//! - selection.rs: selection and interaction
//! - render.rs: rendering logic
//! - history.rs: undo/redo
//! - smart_editing.rs: smart editing features
//! - performance.rs: performance metrics and caching
//! - accessibility.rs: accessibility validation
//! - state.rs: centralized state management
//!
//! This file only re-exports submodules and serves as the entry point for the designer.

pub use visual_designer::layout::*;
pub use visual_designer::selection::*;
pub use visual_designer::render::*;
pub use visual_designer::history::*;
pub use visual_designer::smart_editing::*;
pub use visual_designer::performance::*;
pub use visual_designer::accessibility::*;
pub use visual_designer::state::*;