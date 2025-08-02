//! Learning System for Smart Editing
//!
//! This module provides machine learning capabilities to improve
//! smart editing suggestions based on user behavior and preferences.

use egui::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Guide activation history for learning
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuideActivation {
    /// Timestamp of activation
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Type of guide activated
    pub guide_type: GuideType,
    /// User action taken
    pub user_action: UserAction,
    /// Context information
    pub context: ActivationContext,
}

/// Type of guide that was activated
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum GuideType {
    AlignmentGuide,
    SpacingGuide,
    MagnetismZone,
    GridSnap,
}

/// User action in response to guide
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum UserAction {
    /// User followed the guide suggestion
    Accepted,
    /// User ignored the guide
    Ignored,
    /// User explicitly rejected/disabled the guide
    Rejected,
    /// User modified the suggestion
    Modified,
}

/// Context information for guide activation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActivationContext {
    /// Number of components on canvas
    pub component_count: usize,
    /// Canvas size
    pub canvas_size: Vec2,
    /// Current zoom level
    pub zoom_level: f32,
    /// Time spent on the action
    pub action_duration_ms: u64,
    /// Component types involved
    pub component_types: Vec<String>,
}

/// Learning system for smart editing behavior
pub struct SmartEditingLearningSystem {
    /// History of guide activations
    pub activation_history: Vec<GuideActivation>,
    /// Learned preferences
    pub preferences: LearnedPreferences,
    /// Learning configuration
    pub config: LearningConfig,
    /// User behavior patterns
    pub patterns: BehaviorPatterns,
}

/// Learned user preferences
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LearnedPreferences {
    /// Preferred guide types by frequency of use
    pub preferred_guide_types: HashMap<GuideType, f32>,
    /// Preferred spacing values
    pub preferred_spacings: Vec<f32>,
    /// Preferred alignment tolerances
    pub preferred_tolerances: HashMap<GuideType, f32>,
    /// Context-specific preferences
    pub context_preferences: HashMap<String, ContextualPreference>,
}

/// Context-specific preferences
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContextualPreference {
    /// Component count range this applies to
    pub component_count_range: (usize, usize),
    /// Preferred guide sensitivity
    pub guide_sensitivity: f32,
    /// Preferred guide types for this context
    pub preferred_types: Vec<GuideType>,
}

/// Learning system configuration
#[derive(Clone, Debug)]
pub struct LearningConfig {
    /// Maximum history entries to keep
    pub max_history_entries: usize,
    /// Learning rate for preference updates
    pub learning_rate: f32,
    /// Minimum activations before learning kicks in
    pub min_activations_threshold: usize,
    /// Weight decay for old preferences
    pub preference_decay: f32,
    /// Enable automatic preference updates
    pub auto_update: bool,
}

/// Detected user behavior patterns
#[derive(Clone, Debug, Default)]
pub struct BehaviorPatterns {
    /// Frequently used spacing values
    pub common_spacings: Vec<(f32, f32)>, // (spacing, frequency)
    /// Preferred workflow patterns
    pub workflow_patterns: Vec<WorkflowPattern>,
    /// Time-based usage patterns
    pub usage_patterns: TimeBasedPatterns,
    /// Component arrangement preferences
    pub arrangement_preferences: ArrangementPreferences,
}

/// Detected workflow pattern
#[derive(Clone, Debug)]
pub struct WorkflowPattern {
    /// Sequence of guide types used
    pub guide_sequence: Vec<GuideType>,
    /// Frequency of this pattern
    pub frequency: f32,
    /// Average time for this workflow
    pub average_duration_ms: u64,
    /// Success rate (accepted vs rejected)
    pub success_rate: f32,
}

/// Time-based usage patterns
#[derive(Clone, Debug, Default)]
pub struct TimeBasedPatterns {
    /// Guide usage by hour of day
    pub hourly_usage: HashMap<u32, f32>,
    /// Preferred guide types by time
    pub time_preferences: HashMap<u32, Vec<GuideType>>,
    /// Productivity patterns
    pub productivity_windows: Vec<ProductivityWindow>,
}

/// Productivity time window
#[derive(Clone, Debug)]
pub struct ProductivityWindow {
    /// Start hour (0-23)
    pub start_hour: u32,
    /// End hour (0-23)
    pub end_hour: u32,
    /// Guide acceptance rate during this window
    pub acceptance_rate: f32,
    /// Average action speed
    pub average_speed: f32,
}

/// Component arrangement preferences
#[derive(Clone, Debug, Default)]
pub struct ArrangementPreferences {
    /// Preferred layout patterns
    pub layout_patterns: HashMap<String, f32>,
    /// Preferred component groupings
    pub grouping_preferences: Vec<GroupingPreference>,
    /// Alignment style preferences
    pub alignment_styles: HashMap<String, f32>,
}

/// Component grouping preference
#[derive(Clone, Debug)]
pub struct GroupingPreference {
    /// Component types that are often grouped
    pub component_types: Vec<String>,
    /// Preferred spacing within groups
    pub internal_spacing: f32,
    /// Preferred spacing between groups
    pub external_spacing: f32,
    /// Frequency of this grouping
    pub frequency: f32,
}

impl Default for SmartEditingLearningSystem {
    fn default() -> Self {
        Self {
            activation_history: Vec::new(),
            preferences: LearnedPreferences::default(),
            config: LearningConfig::default(),
            patterns: BehaviorPatterns::default(),
        }
    }
}

impl Default for LearnedPreferences {
    fn default() -> Self {
        Self {
            preferred_guide_types: HashMap::new(),
            preferred_spacings: vec![8.0, 16.0, 24.0, 32.0],
            preferred_tolerances: HashMap::new(),
            context_preferences: HashMap::new(),
        }
    }
}

impl Default for LearningConfig {
    fn default() -> Self {
        Self {
            max_history_entries: 10000,
            learning_rate: 0.1,
            min_activations_threshold: 10,
            preference_decay: 0.95,
            auto_update: true,
        }
    }
}

impl SmartEditingLearningSystem {
    /// Create a new learning system
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Record a guide activation
    pub fn record_activation(&mut self, activation: GuideActivation) {
        self.activation_history.push(activation);
        
        // Limit history size
        if self.activation_history.len() > self.config.max_history_entries {
            self.activation_history.remove(0);
        }
        
        // Update preferences if auto-update is enabled
        if self.config.auto_update && self.activation_history.len() >= self.config.min_activations_threshold {
            self.update_preferences();
        }
    }
    
    /// Update learned preferences based on history
    pub fn update_preferences(&mut self) {
        self.update_guide_type_preferences();
        self.update_spacing_preferences();
        self.update_tolerance_preferences();
        self.update_contextual_preferences();
        self.analyze_behavior_patterns();
    }
    
    /// Update guide type preferences
    fn update_guide_type_preferences(&mut self) {
        let mut type_scores: HashMap<GuideType, (f32, f32)> = HashMap::new(); // (positive, total)
        
        for activation in &self.activation_history {
            let entry = type_scores.entry(activation.guide_type.clone()).or_default();
            entry.1 += 1.0; // Total count
            
            match activation.user_action {
                UserAction::Accepted => entry.0 += 1.0,
                UserAction::Modified => entry.0 += 0.7, // Partial credit
                UserAction::Ignored => entry.0 += 0.1,  // Very little credit
                UserAction::Rejected => {} // No credit
            }
        }
        
        // Update preferences with learning rate
        for (guide_type, (positive, total)) in type_scores {
            let acceptance_rate = positive / total;
            let current_pref = self.preferences.preferred_guide_types.get(&guide_type).copied().unwrap_or(0.5);
            let new_pref = current_pref * (1.0 - self.config.learning_rate) + acceptance_rate * self.config.learning_rate;
            
            self.preferences.preferred_guide_types.insert(guide_type, new_pref);
        }
        
        // Apply decay to existing preferences
        for (_, pref) in self.preferences.preferred_guide_types.iter_mut() {
            *pref *= self.config.preference_decay;
        }
    }
    
    /// Update spacing preferences based on accepted suggestions
    fn update_spacing_preferences(&mut self) {
        let mut spacing_frequency: HashMap<i32, f32> = HashMap::new();
        
        // Extract spacing values from accepted activations
        for activation in &self.activation_history {
            if activation.user_action == UserAction::Accepted && activation.guide_type == GuideType::SpacingGuide {
                // In a real implementation, we'd extract actual spacing values from context
                // For now, we'll simulate with common values
                let simulated_spacing = match activation.context.component_count {
                    1..=3 => 16.0,
                    4..=8 => 24.0,
                    _ => 32.0,
                };
                
                let spacing_key = (simulated_spacing / 4.0) as i32 * 4; // Round to nearest 4px
                *spacing_frequency.entry(spacing_key).or_default() += 1.0;
            }
        }
        
        // Update preferred spacings based on frequency
        let mut spacing_vec: Vec<(f32, f32)> = spacing_frequency
            .into_iter()
            .map(|(spacing, freq)| (spacing as f32, freq))
            .collect();
        
        spacing_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Keep top 10 most frequent spacings
        self.preferences.preferred_spacings = spacing_vec
            .into_iter()
            .take(10)
            .map(|(spacing, _)| spacing)
            .collect();
    }
    
    /// Update tolerance preferences for different guide types
    fn update_tolerance_preferences(&mut self) {
        for guide_type in [GuideType::AlignmentGuide, GuideType::SpacingGuide, GuideType::MagnetismZone] {
            let relevant_activations: Vec<_> = self.activation_history
                .iter()
                .filter(|a| a.guide_type == guide_type)
                .collect();
            
            if relevant_activations.is_empty() {
                continue;
            }
            
            // Calculate acceptance rate for this guide type
            let accepted_count = relevant_activations
                .iter()
                .filter(|a| a.user_action == UserAction::Accepted)
                .count();
            
            let acceptance_rate = accepted_count as f32 / relevant_activations.len() as f32;
            
            // Adjust tolerance based on acceptance rate
            // Lower acceptance rate = increase tolerance (less sensitive)
            // Higher acceptance rate = decrease tolerance (more sensitive)
            let base_tolerance = 5.0;
            let new_tolerance = base_tolerance * (2.0 - acceptance_rate).max(0.5).min(2.0);
            
            self.preferences.preferred_tolerances.insert(guide_type, new_tolerance);
        }
    }
    
    /// Update contextual preferences
    fn update_contextual_preferences(&mut self) {
        // Group activations by component count ranges
        let ranges = [(1, 3), (4, 8), (9, 15), (16, usize::MAX)];
        
        for &(min_count, max_count) in &ranges {
            let context_key = format!("components_{}_{}", min_count, max_count);
            
            let relevant_activations: Vec<_> = self.activation_history
                .iter()
                .filter(|a| {
                    a.context.component_count >= min_count && 
                    a.context.component_count <= max_count
                })
                .collect();
            
            if relevant_activations.len() < 5 {
                continue; // Not enough data
            }
            
            // Calculate guide type preferences for this context
            let mut type_acceptance: HashMap<GuideType, f32> = HashMap::new();
            
            for activation in &relevant_activations {
                let score = match activation.user_action {
                    UserAction::Accepted => 1.0,
                    UserAction::Modified => 0.7,
                    UserAction::Ignored => 0.1,
                    UserAction::Rejected => 0.0,
                };
                
                *type_acceptance.entry(activation.guide_type.clone()).or_default() += score;
            }
            
            // Find preferred guide types
            let mut preferred_types: Vec<_> = type_acceptance
                .into_iter()
                .filter(|(_, score)| *score > 0.5)
                .map(|(guide_type, _)| guide_type)
                .collect();
            
            preferred_types.sort();
            
            // Calculate average guide sensitivity
            let total_activations = relevant_activations.len() as f32;
            let accepted_activations = relevant_activations
                .iter()
                .filter(|a| a.user_action == UserAction::Accepted)
                .count() as f32;
            
            let sensitivity = (accepted_activations / total_activations).max(0.1).min(1.0);
            
            self.preferences.context_preferences.insert(
                context_key,
                ContextualPreference {
                    component_count_range: (min_count, max_count),
                    guide_sensitivity: sensitivity,
                    preferred_types,
                },
            );
        }
    }
    
    /// Analyze and extract behavior patterns
    fn analyze_behavior_patterns(&mut self) {
        self.analyze_spacing_patterns();
        self.analyze_workflow_patterns();
        self.analyze_time_patterns();
        self.analyze_arrangement_patterns();
    }
    
    /// Analyze common spacing patterns
    fn analyze_spacing_patterns(&mut self) {
        let mut spacing_counts: HashMap<i32, f32> = HashMap::new();
        
        for activation in &self.activation_history {
            if activation.guide_type == GuideType::SpacingGuide && activation.user_action == UserAction::Accepted {
                // Simulate spacing extraction from context
                let spacing = match activation.context.component_count {
                    1..=3 => 16,
                    4..=8 => 24,
                    _ => 32,
                };
                
                *spacing_counts.entry(spacing).or_default() += 1.0;
            }
        }
        
        // Convert to frequency-sorted list
        let total_count: f32 = spacing_counts.values().sum();
        self.patterns.common_spacings = spacing_counts
            .into_iter()
            .map(|(spacing, count)| (spacing as f32, count / total_count))
            .collect();
        
        self.patterns.common_spacings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    }
    
    /// Analyze workflow patterns (sequences of guide usage)
    fn analyze_workflow_patterns(&mut self) {
        let window_size = 5; // Look at sequences of 5 actions
        
        if self.activation_history.len() < window_size {
            return;
        }
        
        let mut sequence_counts: HashMap<Vec<GuideType>, f32> = HashMap::new();
        
        for window in self.activation_history.windows(window_size) {
            let sequence: Vec<GuideType> = window.iter().map(|a| a.guide_type.clone()).collect();
            *sequence_counts.entry(sequence).or_default() += 1.0;
        }
        
        // Convert to workflow patterns
        let total_sequences = sequence_counts.values().sum::<f32>();
        
        self.patterns.workflow_patterns = sequence_counts
            .into_iter()
            .filter(|(_, count)| *count >= 2.0) // At least 2 occurrences
            .map(|(sequence, count)| {
                let frequency = count / total_sequences;
                
                // Calculate success rate for this pattern
                let success_rate = 0.8; // Simplified - would analyze actual outcomes
                
                // Calculate average duration
                let average_duration_ms = 5000; // Simplified - would analyze actual durations
                
                WorkflowPattern {
                    guide_sequence: sequence,
                    frequency,
                    average_duration_ms,
                    success_rate,
                }
            })
            .collect();
        
        // Sort by frequency
        self.patterns.workflow_patterns.sort_by(|a, b| b.frequency.partial_cmp(&a.frequency).unwrap());
    }
    
    /// Analyze time-based usage patterns
    fn analyze_time_patterns(&mut self) {
        let mut hourly_counts: HashMap<u32, f32> = HashMap::new();
        
        for activation in &self.activation_history {
            let hour = activation.timestamp.hour();
            *hourly_counts.entry(hour).or_default() += 1.0;
        }
        
        // Normalize to frequencies
        let total_activations = hourly_counts.values().sum::<f32>();
        self.patterns.usage_patterns.hourly_usage = hourly_counts
            .into_iter()
            .map(|(hour, count)| (hour, count / total_activations))
            .collect();
        
        // Identify productivity windows (hours with high acceptance rates)
        let mut productivity_data: Vec<(u32, f32, f32)> = Vec::new(); // (hour, acceptance_rate, count)
        
        for hour in 0..24 {
            let hour_activations: Vec<_> = self.activation_history
                .iter()
                .filter(|a| a.timestamp.hour() == hour)
                .collect();
            
            if hour_activations.len() < 3 {
                continue;
            }
            
            let accepted = hour_activations
                .iter()
                .filter(|a| a.user_action == UserAction::Accepted)
                .count() as f32;
            
            let acceptance_rate = accepted / hour_activations.len() as f32;
            productivity_data.push((hour, acceptance_rate, hour_activations.len() as f32));
        }
        
        // Find windows with above-average acceptance rates
        let avg_acceptance: f32 = productivity_data.iter().map(|(_, rate, _)| *rate).sum::<f32>() / productivity_data.len() as f32;
        
        let mut windows = Vec::new();
        let mut current_window: Option<(u32, u32, Vec<f32>)> = None;
        
        for (hour, acceptance_rate, _) in productivity_data {
            if acceptance_rate > avg_acceptance * 1.2 {
                match current_window {
                    Some((start, _, ref mut rates)) => {
                        rates.push(acceptance_rate);
                        current_window = Some((start, hour, rates.clone()));
                    }
                    None => {
                        current_window = Some((hour, hour, vec![acceptance_rate]));
                    }
                }
            } else if let Some((start, end, rates)) = current_window.take() {
                let avg_rate = rates.iter().sum::<f32>() / rates.len() as f32;
                windows.push(ProductivityWindow {
                    start_hour: start,
                    end_hour: end,
                    acceptance_rate: avg_rate,
                    average_speed: 1.0, // Simplified
                });
            }
        }
        
        self.patterns.usage_patterns.productivity_windows = windows;
    }
    
    /// Analyze component arrangement patterns
    fn analyze_arrangement_patterns(&mut self) {
        // Simplified implementation - would analyze actual component layouts
        let mut layout_counts: HashMap<String, f32> = HashMap::new();
        
        for activation in &self.activation_history {
            let layout_type = match activation.context.component_count {
                1..=2 => "simple",
                3..=5 => "medium",
                6..=10 => "complex",
                _ => "large",
            };
            
            *layout_counts.entry(layout_type.to_string()).or_default() += 1.0;
        }
        
        // Normalize to frequencies
        let total_layouts = layout_counts.values().sum::<f32>();
        self.patterns.arrangement_preferences.layout_patterns = layout_counts
            .into_iter()
            .map(|(layout, count)| (layout, count / total_layouts))
            .collect();
    }
    
    /// Get recommendation for guide sensitivity based on learned preferences
    pub fn get_guide_sensitivity_recommendation(&self, guide_type: &GuideType, context: &ActivationContext) -> f32 {
        // Start with base preference for this guide type
        let base_preference = self.preferences.preferred_guide_types.get(guide_type).copied().unwrap_or(0.5);
        
        // Check for contextual preferences
        let context_key = format!("components_{}_{}", 
            if context.component_count <= 3 { 1 } else if context.component_count <= 8 { 4 } else if context.component_count <= 15 { 9 } else { 16 },
            if context.component_count <= 3 { 3 } else if context.component_count <= 8 { 8 } else if context.component_count <= 15 { 15 } else { usize::MAX }
        );
        
        let contextual_sensitivity = self.preferences.context_preferences
            .get(&context_key)
            .map(|pref| pref.guide_sensitivity)
            .unwrap_or(1.0);
        
        // Combine base preference with contextual sensitivity
        (base_preference * contextual_sensitivity).clamp(0.1, 2.0)
    }
    
    /// Get recommended spacing values based on learned preferences
    pub fn get_spacing_recommendations(&self, context: &ActivationContext) -> Vec<f32> {
        let mut recommendations = self.preferences.preferred_spacings.clone();
        
        // Add context-specific adjustments
        match context.component_count {
            1..=3 => {
                recommendations.retain(|&spacing| spacing <= 32.0);
                if !recommendations.contains(&16.0) {
                    recommendations.push(16.0);
                }
            }
            4..=8 => {
                if !recommendations.contains(&24.0) {
                    recommendations.push(24.0);
                }
            }
            _ => {
                if !recommendations.contains(&32.0) {
                    recommendations.push(32.0);
                }
            }
        }
        
        recommendations.sort_by(|a, b| a.partial_cmp(b).unwrap());
        recommendations.dedup();
        recommendations
    }
    
    /// Export learning data for analysis or backup
    pub fn export_learning_data(&self) -> serde_json::Result<String> {
        #[derive(Serialize)]
        struct LearningData<'a> {
            activation_history: &'a [GuideActivation],
            preferences: &'a LearnedPreferences,
            patterns: &'a BehaviorPatterns,
        }
        
        let data = LearningData {
            activation_history: &self.activation_history,
            preferences: &self.preferences,
            patterns: &self.patterns,
        };
        
        serde_json::to_string_pretty(&data)
    }
    
    /// Import learning data from backup
    pub fn import_learning_data(&mut self, json: &str) -> serde_json::Result<()> {
        #[derive(Deserialize)]
        struct LearningData {
            activation_history: Vec<GuideActivation>,
            preferences: LearnedPreferences,
        }
        
        let data: LearningData = serde_json::from_str(json)?;
        self.activation_history = data.activation_history;
        self.preferences = data.preferences;
        
        // Regenerate patterns from imported data
        self.analyze_behavior_patterns();
        
        Ok(())
    }
    
    /// Get learning statistics
    pub fn get_statistics(&self) -> LearningStatistics {
        let total_activations = self.activation_history.len();
        let accepted_count = self.activation_history.iter().filter(|a| a.user_action == UserAction::Accepted).count();
        let rejected_count = self.activation_history.iter().filter(|a| a.user_action == UserAction::Rejected).count();
        
        let acceptance_rate = if total_activations > 0 {
            accepted_count as f32 / total_activations as f32
        } else {
            0.0
        };
        
        LearningStatistics {
            total_activations,
            acceptance_rate,
            rejection_rate: rejected_count as f32 / total_activations.max(1) as f32,
            learned_guide_types: self.preferences.preferred_guide_types.len(),
            learned_spacings: self.preferences.preferred_spacings.len(),
            workflow_patterns: self.patterns.workflow_patterns.len(),
        }
    }
}

/// Learning system statistics
#[derive(Debug, Clone)]
pub struct LearningStatistics {
    pub total_activations: usize,
    pub acceptance_rate: f32,
    pub rejection_rate: f32,
    pub learned_guide_types: usize,
    pub learned_spacings: usize,
    pub workflow_patterns: usize,
}