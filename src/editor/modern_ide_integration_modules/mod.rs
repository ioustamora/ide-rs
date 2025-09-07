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
pub use design_tokens::DesignTokenSystem;
pub use component_library::ComponentLibrary;
pub use framework_export::FrameworkExportManager;
pub use theme_system::ThemeSystem;
pub use code_generation::CodeGenerator;
// Note: ExportFormat import disabled due to path resolution issue
// pub use super::super::shared::serialization::ExportFormat;