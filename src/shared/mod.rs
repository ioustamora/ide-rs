//! Shared Utilities Module
//!
//! This module contains common utilities, data structures, and helper functions
//! that are used across multiple modules in the IDE.

pub mod geometry;
pub mod color_utils;
pub mod validation;
pub mod serialization;
pub mod performance;

// Re-export commonly used types
pub use geometry::{Bounds, Transform2D, SpatialIndex};
pub use color_utils::{ColorPalette, ColorHarmony, AccessibilityChecker};
pub use validation::{ValidationResult, ValidationError, Validator};
pub use serialization::{SerializableComponent, ExportFormat};
pub use performance::{PerformanceProfiler, RenderMetrics};