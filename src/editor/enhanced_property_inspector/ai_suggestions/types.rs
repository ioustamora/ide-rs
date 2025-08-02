//! Core types for AI property suggestions
//!
//! This module contains the fundamental data structures and enums
//! used throughout the AI suggestions system.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::editor::inspector::PropertyValue;

/// AI property suggestion
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropertySuggestion {
    /// Property name
    pub property_name: String,
    /// Suggested value
    pub suggested_value: PropertyValue,
    /// Current value
    pub current_value: PropertyValue,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,
    /// Reasoning behind the suggestion
    pub reasoning: String,
    /// Suggestion category
    pub category: SuggestionCategory,
    /// Context that triggered this suggestion
    pub context: SuggestionContext,
    /// Supporting evidence
    pub evidence: Vec<String>,
}

/// Category of AI suggestion
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SuggestionCategory {
    /// Performance optimization
    Performance,
    /// Accessibility improvement
    Accessibility,
    /// Visual consistency
    Consistency,
    /// User experience enhancement
    UserExperience,
    /// Best practice recommendation
    BestPractice,
    /// Responsive design
    ResponsiveDesign,
    /// Brand guideline compliance
    BrandCompliance,
}

/// Context for suggestion generation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuggestionContext {
    /// Component type
    pub component_type: String,
    /// Surrounding components
    pub nearby_components: Vec<String>,
    /// Container properties
    pub container_context: HashMap<String, PropertyValue>,
    /// User's recent actions
    pub recent_actions: Vec<UserAction>,
    /// Current design phase
    pub design_phase: DesignPhase,
    /// Target audience/platform
    pub target_platform: Vec<String>,
}

/// User action for context
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserAction {
    /// Action timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Property that was changed
    pub property_changed: String,
    /// Old value
    pub old_value: PropertyValue,
    /// New value
    pub new_value: PropertyValue,
    /// Component type
    pub component_type: String,
}

/// Design phase indicator
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum DesignPhase {
    /// Initial layout and structure
    Layout,
    /// Visual styling and theming
    Styling,
    /// Fine-tuning and polish
    Polish,
    /// Responsive adaptation
    Responsive,
    /// Accessibility review
    Accessibility,
}