//! Smart Editing - Modular Architecture
//!
//! This module provides intelligent editing assistance with modern UX features:
//! - Intelligent alignment guides with automatic grid detection
//! - Component magnetism for precise positioning  
//! - Smart spacing guides with visual feedback
//! - Machine learning-based pattern recognition and suggestions

pub mod alignment_guides;
pub mod magnetism;
pub mod spacing_guides;
pub mod learning_system;

// Re-export main types for convenience
pub use alignment_guides::{AlignmentGuideManager, AlignmentGuide, GuideDirection};
pub use magnetism::{MagnetismManager, MagnetZone, MagnetType};
pub use spacing_guides::{SpacingGuideManager, SpacingGuide, SpacingType};
pub use learning_system::{SmartEditingLearningSystem, BehaviorPatterns, GuideType};