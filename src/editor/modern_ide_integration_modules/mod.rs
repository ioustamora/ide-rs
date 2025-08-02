//! Modern IDE Integration - Modular Architecture
//!
//! This module provides modern IDE integration features:
//! - Design token system with comprehensive token management
//! - Component library with reusable design patterns
//! - Framework-specific export capabilities
//! - Theme system with advanced customization
//! - Code generation for multiple frameworks

pub mod design_tokens;
pub mod component_library;
pub mod framework_export;
pub mod theme_system;
pub mod code_generation;

// Re-export main types for convenience
pub use design_tokens::{DesignTokenSystem, TokenCategory, DesignToken};
pub use component_library::{ComponentLibrary, ComponentTemplate, LibraryCategory};
pub use framework_export::{FrameworkExporter, ExportTarget, ExportOptions};
pub use theme_system::{ThemeSystem, Theme, ThemeVariant};
pub use code_generation::{CodeGenerator, CodeStyle, ExportFormat};