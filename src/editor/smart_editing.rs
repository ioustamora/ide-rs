//! Smart Editing Features - Legacy Compatibility Layer
//!
//! This module provides backward compatibility for the old monolithic structure.
//! New code should use the modular structure in the `smart_editing` submodule.

// Re-export the new modular structure for backward compatibility
pub use self::smart_editing::*;

// Include the new modular structure
mod smart_editing;