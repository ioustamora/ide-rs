//! AI-Powered Property Suggestions - Legacy Compatibility Layer
//!
//! This module provides backward compatibility for the old monolithic structure.
//! New code should use the modular structure in the `ai_suggestions` submodule.

// Re-export the new modular structure for backward compatibility
pub use self::ai_suggestions::*;

// Include the new modular structure
mod ai_suggestions;