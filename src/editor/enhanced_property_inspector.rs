//! Enhanced Property Inspector - Legacy Compatibility Layer
//!
//! This module provides backward compatibility for the old monolithic structure.
//! New code should use the modular structure in the `enhanced_property_inspector` submodule.

// Re-export the new modular structure for backward compatibility
pub use self::enhanced_property_inspector::*;

// Include the new modular structure
mod enhanced_property_inspector;