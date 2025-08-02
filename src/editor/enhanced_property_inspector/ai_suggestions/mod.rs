//! AI-Powered Property Suggestions - Modular Architecture
//!
//! This module provides AI-driven suggestions for component properties
//! based on context, user behavior, and design best practices.

pub mod types;
pub mod suggestion_engine;
pub mod pattern_recognition;
pub mod context_analysis;
pub mod user_learning;

// Re-export main types for convenience
pub use types::{PropertySuggestion, SuggestionCategory, SuggestionContext, UserAction, DesignPhase};
pub use suggestion_engine::{PropertySuggestionEngine, SuggestionModels, ColorHarmonyModel, PerformanceAnalyzer};
pub use pattern_recognition::{PropertyPatternRecognition, DetectedPattern, PatternMatcher, MatcherType};
pub use context_analysis::{PropertyContextAnalyzer, ContextPredictionModel, PredictedAction};
pub use user_learning::{UserPreferenceLearning, UserPreference};

use egui::*;
use std::collections::HashMap;
use crate::editor::inspector::PropertyValue;

/// AI-powered property suggestion system
pub struct AiPropertySuggestions {
    /// Current suggestions
    pub suggestions: Vec<PropertySuggestion>,
    /// Suggestion engine
    pub engine: PropertySuggestionEngine,
    /// Pattern recognition system
    pub pattern_recognition: PropertyPatternRecognition,
    /// Context analyzer
    pub context_analyzer: PropertyContextAnalyzer,
    /// User preference learning
    pub preference_learning: UserPreferenceLearning,
}

impl Default for AiPropertySuggestions {
    fn default() -> Self {
        Self {
            suggestions: Vec::new(),
            engine: PropertySuggestionEngine::default(),
            pattern_recognition: PropertyPatternRecognition::default(),
            context_analyzer: PropertyContextAnalyzer::default(),
            preference_learning: UserPreferenceLearning::default(),
        }
    }
}

impl AiPropertySuggestions {
    /// Create new AI suggestions system
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Generate suggestions for a property
    pub fn generate_suggestions(&mut self, context: &SuggestionContext, property_name: &str, current_value: &PropertyValue) {
        self.suggestions.clear();
        
        // Generate suggestions based on different models
        match current_value {
            PropertyValue::Color(color) => {
                let color_suggestions = self.engine.models.color_harmony.suggest_harmonious_colors(color);
                self.suggestions.extend(color_suggestions);
            }
            PropertyValue::Number(number) => {
                self.generate_spacing_suggestions(context, property_name, *number);
            }
            PropertyValue::String(text) => {
                self.generate_typography_suggestions(context, property_name, text);
            }
            _ => {}
        }
        
        // Add accessibility suggestions
        self.add_accessibility_suggestions(context, property_name, current_value);
        
        // Add performance suggestions
        self.add_performance_suggestions(property_name, current_value);
        
        // Sort by confidence
        self.suggestions.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
    }
    
    /// Generate spacing suggestions
    fn generate_spacing_suggestions(&mut self, context: &SuggestionContext, property_name: &str, current_value: f32) {
        if property_name.contains("margin") || property_name.contains("padding") {
            // Golden ratio suggestion
            let golden_ratio = 1.618;
            let suggested_value = (current_value * golden_ratio).round();
            
            self.suggestions.push(PropertySuggestion {
                property_name: property_name.to_string(),
                suggested_value: PropertyValue::Number(suggested_value as f64),
                current_value: PropertyValue::Number(current_value as f64),
                confidence: 0.7,
                reasoning: "Golden ratio spacing for visual harmony".to_string(),
                category: SuggestionCategory::BestPractice,
                context: context.clone(),
                evidence: vec!["Mathematical design principles".to_string()],
            });
        }
    }
    
    /// Generate typography suggestions
    fn generate_typography_suggestions(&mut self, context: &SuggestionContext, property_name: &str, _current_value: &str) {
        if property_name.contains("font") {
            // Suggest web-safe fonts
            let web_safe_fonts = vec!["system-ui", "Georgia", "Times New Roman"];
            
            for font in web_safe_fonts {
                self.suggestions.push(PropertySuggestion {
                    property_name: property_name.to_string(),
                    suggested_value: PropertyValue::String(font.to_string()),
                    current_value: PropertyValue::String("current".to_string()),
                    confidence: 0.6,
                    reasoning: "Web-safe font for better compatibility".to_string(),
                    category: SuggestionCategory::Performance,
                    context: context.clone(),
                    evidence: vec!["Cross-platform compatibility".to_string()],
                });
            }
        }
    }
    
    /// Add accessibility suggestions
    fn add_accessibility_suggestions(&mut self, context: &SuggestionContext, property_name: &str, current_value: &PropertyValue) {
        if property_name == "font_size" {
            if let PropertyValue::Number(size) = current_value {
                if *size < 16.0 {
                    self.suggestions.push(PropertySuggestion {
                        property_name: property_name.to_string(),
                        suggested_value: PropertyValue::Number(16.0),
                        current_value: current_value.clone(),
                        confidence: 0.9,
                        reasoning: "Minimum font size for accessibility".to_string(),
                        category: SuggestionCategory::Accessibility,
                        context: context.clone(),
                        evidence: vec!["WCAG 2.1 guidelines".to_string()],
                    });
                }
            }
        }
        
        if property_name.contains("width") || property_name.contains("height") {
            if let PropertyValue::Number(size) = current_value {
                if context.component_type.contains("button") && *size < 44.0 {
                    self.suggestions.push(PropertySuggestion {
                        property_name: property_name.to_string(),
                        suggested_value: PropertyValue::Number(44.0),
                        current_value: current_value.clone(),
                        confidence: 0.85,
                        reasoning: "Minimum touch target size for accessibility".to_string(),
                        category: SuggestionCategory::Accessibility,
                        context: context.clone(),
                        evidence: vec!["iOS Human Interface Guidelines".to_string(), "Material Design Guidelines".to_string()],
                    });
                }
            }
        }
    }
    
    /// Add performance suggestions
    fn add_performance_suggestions(&mut self, property_name: &str, current_value: &PropertyValue) {
        // Suggest transform over position changes for animations
        if property_name == "position" {
            self.suggestions.push(PropertySuggestion {
                property_name: "transform".to_string(),
                suggested_value: PropertyValue::String("translateX(0px)".to_string()),
                current_value: current_value.clone(),
                confidence: 0.6,
                reasoning: "Use transform for better animation performance".to_string(),
                category: SuggestionCategory::Performance,
                context: SuggestionContext {
                    component_type: "unknown".to_string(),
                    nearby_components: vec![],
                    container_context: HashMap::new(),
                    recent_actions: vec![],
                    design_phase: DesignPhase::Polish,
                    target_platform: vec!["web".to_string()],
                },
                evidence: vec!["GPU acceleration benefits".to_string()],
            });
        }
    }
    
    /// Update user preferences based on actions
    pub fn learn_from_action(&mut self, action: &UserAction) {
        let preference_key = format!("{}_{}", action.component_type, action.property_changed);
        
        let preference = self.preference_learning.preferences
            .entry(preference_key)
            .or_insert(UserPreference {
                category: action.property_changed.clone(),
                preferred_values: vec![],
                frequency: 0.0,
                contexts: vec![action.component_type.clone()],
            });
        
        // Update preference
        preference.preferred_values.push(action.new_value.clone());
        preference.frequency += self.preference_learning.learning_rate;
        
        // Limit history size
        if preference.preferred_values.len() > 10 {
            preference.preferred_values.remove(0);
        }
    }
    
    /// Render AI suggestions UI
    pub fn render_ui(&mut self, ui: &mut Ui) {
        ui.heading("AI Suggestions");
        
        if self.suggestions.is_empty() {
            ui.label("No suggestions available");
            return;
        }
        
        egui::ScrollArea::vertical().show(ui, |ui| {
            for suggestion in &self.suggestions {
                self.render_suggestion_ui(ui, suggestion);
            }
        });
    }
    
    /// Render individual suggestion
    fn render_suggestion_ui(&self, ui: &mut Ui, suggestion: &PropertySuggestion) {
        egui::Frame::group(ui.style())
            .inner_margin(8.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Category icon
                    let icon = match suggestion.category {
                        SuggestionCategory::Performance => "⚡",
                        SuggestionCategory::Accessibility => "♿",
                        SuggestionCategory::Consistency => "🎯",
                        SuggestionCategory::UserExperience => "😊",
                        SuggestionCategory::BestPractice => "✨",
                        SuggestionCategory::ResponsiveDesign => "📱",
                        SuggestionCategory::BrandCompliance => "🎨",
                    };
                    ui.label(icon);
                    
                    // Confidence indicator
                    let confidence_color = if suggestion.confidence > 0.8 {
                        Color32::GREEN
                    } else if suggestion.confidence > 0.6 {
                        Color32::YELLOW
                    } else {
                        Color32::GRAY
                    };
                    ui.colored_label(confidence_color, format!("{:.0}%", suggestion.confidence * 100.0));
                });
                
                ui.label(&suggestion.reasoning);
                
                ui.horizontal(|ui| {
                    ui.label(format!("Property: {}", suggestion.property_name));
                    ui.label(format!("Suggested: {:?}", suggestion.suggested_value));
                });
                
                if !suggestion.evidence.is_empty() {
                    ui.collapsing("Evidence", |ui| {
                        for evidence in &suggestion.evidence {
                            ui.label(format!("• {}", evidence));
                        }
                    });
                }
                
                ui.horizontal(|ui| {
                    if ui.button("Apply").clicked() {
                        // Apply suggestion
                    }
                    
                    if ui.button("Dismiss").clicked() {
                        // Dismiss suggestion
                    }
                });
            });
    }
}