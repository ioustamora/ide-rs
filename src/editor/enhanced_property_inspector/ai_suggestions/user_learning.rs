//! User preference learning system
//!
//! This module handles learning from user behavior and preferences
//! to provide personalized suggestions.

use std::collections::HashMap;
use crate::editor::inspector::PropertyValue;

/// User preference learning system
pub struct UserPreferenceLearning {
    /// Learned preferences
    pub preferences: HashMap<String, UserPreference>,
    /// Learning rate
    pub learning_rate: f32,
    /// Preference confidence
    pub confidence_threshold: f32,
}

/// Learned user preference
#[derive(Clone, Debug)]
pub struct UserPreference {
    /// Preference category
    pub category: String,
    /// Preferred values
    pub preferred_values: Vec<PropertyValue>,
    /// Usage frequency
    pub frequency: f32,
    /// Context where this preference applies
    pub contexts: Vec<String>,
}

impl Default for UserPreferenceLearning {
    fn default() -> Self {
        Self {
            preferences: HashMap::new(),
            learning_rate: 0.1,
            confidence_threshold: 0.7,
        }
    }
}