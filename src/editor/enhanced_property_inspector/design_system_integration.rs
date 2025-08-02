//! Design System Integration
//!
//! This module provides integration with design systems, including design tokens,
//! accessibility validation, and automatic style application.

use egui::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::editor::inspector::PropertyValue;

/// Design system integration manager
pub struct DesignSystemIntegration {
    /// Design tokens system
    pub design_tokens: DesignTokens,
    /// Token suggestion engine
    pub token_suggestions: TokenSuggestionEngine,
    /// Design system validator
    pub validator: DesignSystemValidator,
    /// Auto-apply system for tokens
    pub auto_apply: AutoApplySystem,
}

/// Design tokens collection
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DesignTokens {
    /// Color tokens
    pub colors: HashMap<String, ColorToken>,
    /// Typography tokens
    pub typography: HashMap<String, TypographyToken>,
    /// Spacing tokens
    pub spacing: HashMap<String, SpacingToken>,
    /// Shadow tokens
    pub shadows: HashMap<String, ShadowToken>,
    /// Animation tokens
    pub animations: HashMap<String, AnimationToken>,
}

/// Color token with accessibility information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColorToken {
    /// Token name
    pub name: String,
    /// Color value
    pub value: [u8; 4], // RGBA
    /// Color description
    pub description: String,
    /// Color category (primary, secondary, etc.)
    pub category: String,
    /// Accessibility information
    pub accessibility: AccessibilityInfo,
    /// Usage contexts where this color is appropriate
    pub usage_contexts: Vec<String>,
    /// Related color tokens
    pub related_tokens: Vec<String>,
    /// Color variants (light, dark modes)
    pub variants: HashMap<String, [u8; 4]>,
}

/// Accessibility information for colors
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccessibilityInfo {
    /// WCAG contrast ratio when used as foreground
    pub contrast_ratio: f32,
    /// WCAG compliance level (AA, AAA)
    pub wcag_level: String,
    /// Whether suitable for text
    pub text_suitable: bool,
    /// Whether suitable for UI elements
    pub ui_suitable: bool,
}

/// Typography token
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TypographyToken {
    /// Token name
    pub name: String,
    /// Font family
    pub font_family: String,
    /// Font size in pixels
    pub font_size: f32,
    /// Font weight
    pub font_weight: FontWeight,
    /// Line height multiplier
    pub line_height: f32,
    /// Letter spacing
    pub letter_spacing: f32,
    /// Text transform
    pub text_transform: TextTransform,
    /// Usage contexts
    pub usage_contexts: Vec<String>,
    /// Responsive sizing
    pub responsive_sizes: HashMap<String, f32>, // breakpoint -> size
}

/// Font weight enumeration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
    Custom(u32),
}

/// Text transform enumeration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TextTransform {
    None,
    Uppercase,
    Lowercase,
    Capitalize,
}

/// Spacing token
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpacingToken {
    /// Token name
    pub name: String,
    /// Spacing value in pixels
    pub value: f32,
    /// Relative scale factor
    pub scale: f32,
    /// Usage contexts
    pub usage_contexts: Vec<String>,
    /// Responsive values
    pub responsive_values: HashMap<String, f32>,
}

/// Shadow token
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShadowToken {
    /// Token name
    pub name: String,
    /// Shadow offset X
    pub offset_x: f32,
    /// Shadow offset Y
    pub offset_y: f32,
    /// Shadow blur radius
    pub blur_radius: f32,
    /// Shadow spread radius
    pub spread_radius: f32,
    /// Shadow color
    pub color: [u8; 4],
    /// Whether shadow is inset
    pub inset: bool,
    /// Elevation level this shadow represents
    pub elevation: u32,
}

/// Animation token
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnimationToken {
    /// Token name
    pub name: String,
    /// Animation duration in milliseconds
    pub duration: u32,
    /// Animation easing function
    pub easing: String,
    /// Animation delay
    pub delay: u32,
    /// Usage contexts
    pub usage_contexts: Vec<String>,
}

/// Token suggestion engine
pub struct TokenSuggestionEngine {
    /// Current suggestions
    pub suggestions: Vec<TokenSuggestion>,
    /// Suggestion preferences
    pub preferences: SuggestionPreferences,
}

/// Token suggestion
#[derive(Clone, Debug)]
pub struct TokenSuggestion {
    /// Property name this suggestion applies to
    pub property_name: String,
    /// Suggested token name
    pub token_name: String,
    /// Token value
    pub token_value: PropertyValue,
    /// Reason for suggestion
    pub reason: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,
    /// Suggestion type
    pub suggestion_type: SuggestionType,
}

/// Type of token suggestion
#[derive(Clone, Debug, PartialEq)]
pub enum SuggestionType {
    /// Exact token match
    ExactMatch,
    /// Similar value found
    SimilarValue,
    /// Accessibility improvement
    AccessibilityImprovement,
    /// Consistency with other components
    ConsistencyImprovement,
    /// Design system best practice
    BestPractice,
}

/// Suggestion preferences
#[derive(Clone, Debug)]
pub struct SuggestionPreferences {
    /// Enable automatic suggestions
    pub auto_suggest: bool,
    /// Minimum confidence threshold for showing suggestions
    pub confidence_threshold: f32,
    /// Preferred suggestion types
    pub preferred_types: Vec<SuggestionType>,
    /// Context-aware suggestions
    pub context_aware: bool,
}

/// Design system validator
pub struct DesignSystemValidator {
    /// Validation rules
    pub rules: Vec<ValidationRule>,
    /// Current violations
    pub violations: Vec<DesignViolation>,
}

/// Design system validation rule
#[derive(Clone, Debug)]
pub struct ValidationRule {
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Properties this rule applies to
    pub applicable_properties: Vec<String>,
    /// Component types this rule applies to
    pub applicable_components: Vec<String>,
    /// Rule severity
    pub severity: RuleSeverity,
    /// Rule validation function (simplified)
    pub rule_type: RuleType,
}

/// Rule severity levels
#[derive(Clone, Debug, PartialEq)]
pub enum RuleSeverity {
    Error,
    Warning,
    Info,
}

/// Rule type for validation
#[derive(Clone, Debug)]
pub enum RuleType {
    /// Must use design tokens
    RequireDesignTokens,
    /// Accessibility requirements
    AccessibilityCompliance,
    /// Consistency requirements
    ConsistencyCheck,
    /// Best practice recommendations
    BestPractice,
}

/// Design system violation
#[derive(Clone, Debug)]
pub struct DesignViolation {
    /// Component ID where violation occurs
    pub component_id: usize,
    /// Property name with violation
    pub property_name: String,
    /// Current value
    pub current_value: PropertyValue,
    /// Rule that was violated
    pub violated_rule: String,
    /// Severity of violation
    pub severity: RuleSeverity,
    /// Suggested fix
    pub suggested_fix: Option<PropertyValue>,
    /// Explanation of the violation
    pub explanation: String,
}

/// Auto-apply system for design tokens
pub struct AutoApplySystem {
    /// Whether auto-apply is enabled
    pub enabled: bool,
    /// Auto-apply preferences
    pub preferences: AutoApplyPreferences,
    /// History of auto-applied changes
    pub application_history: Vec<AutoApplicationRecord>,
}

/// Auto-apply preferences
#[derive(Clone, Debug)]
pub struct AutoApplyPreferences {
    /// Automatically apply exact token matches
    pub apply_exact_matches: bool,
    /// Automatically apply accessibility improvements
    pub apply_accessibility_fixes: bool,
    /// Minimum confidence for auto-apply
    pub min_confidence: f32,
    /// Properties to auto-apply
    pub auto_apply_properties: Vec<String>,
}

/// Record of an auto-applied change
#[derive(Clone, Debug)]
pub struct AutoApplicationRecord {
    /// Timestamp of application
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Component ID
    pub component_id: usize,
    /// Property name
    pub property_name: String,
    /// Old value
    pub old_value: PropertyValue,
    /// New value
    pub new_value: PropertyValue,
    /// Token name that was applied
    pub applied_token: String,
    /// Reason for application
    pub reason: String,
}

impl Default for DesignSystemIntegration {
    fn default() -> Self {
        Self {
            design_tokens: DesignTokens::default(),
            token_suggestions: TokenSuggestionEngine::default(),
            validator: DesignSystemValidator::default(),
            auto_apply: AutoApplySystem::default(),
        }
    }
}

impl Default for DesignTokens {
    fn default() -> Self {
        let mut tokens = Self {
            colors: HashMap::new(),
            typography: HashMap::new(),
            spacing: HashMap::new(),
            shadows: HashMap::new(),
            animations: HashMap::new(),
        };
        
        tokens.initialize_default_tokens();
        tokens
    }
}

impl Default for TokenSuggestionEngine {
    fn default() -> Self {
        Self {
            suggestions: Vec::new(),
            preferences: SuggestionPreferences::default(),
        }
    }
}

impl Default for SuggestionPreferences {
    fn default() -> Self {
        Self {
            auto_suggest: true,
            confidence_threshold: 0.7,
            preferred_types: vec![
                SuggestionType::ExactMatch,
                SuggestionType::AccessibilityImprovement,
                SuggestionType::ConsistencyImprovement,
            ],
            context_aware: true,
        }
    }
}

impl Default for DesignSystemValidator {
    fn default() -> Self {
        let mut validator = Self {
            rules: Vec::new(),
            violations: Vec::new(),
        };
        
        validator.initialize_default_rules();
        validator
    }
}

impl Default for AutoApplySystem {
    fn default() -> Self {
        Self {
            enabled: false, // Conservative default
            preferences: AutoApplyPreferences::default(),
            application_history: Vec::new(),
        }
    }
}

impl Default for AutoApplyPreferences {
    fn default() -> Self {
        Self {
            apply_exact_matches: true,
            apply_accessibility_fixes: false, // User should review these
            min_confidence: 0.9,
            auto_apply_properties: vec![
                "color".to_string(),
                "background_color".to_string(),
                "font_size".to_string(),
                "margin".to_string(),
                "padding".to_string(),
            ],
        }
    }
}

impl DesignTokens {
    /// Initialize default design tokens
    fn initialize_default_tokens(&mut self) {
        // Default color tokens
        self.colors.insert("primary".to_string(), ColorToken {
            name: "primary".to_string(),
            value: [70, 130, 200, 255],
            description: "Primary brand color".to_string(),
            category: "brand".to_string(),
            accessibility: AccessibilityInfo {
                contrast_ratio: 4.5,
                wcag_level: "AA".to_string(),
                text_suitable: true,
                ui_suitable: true,
            },
            usage_contexts: vec!["buttons".to_string(), "links".to_string()],
            related_tokens: vec!["primary-light".to_string(), "primary-dark".to_string()],
            variants: HashMap::from([
                ("light".to_string(), [120, 170, 230, 255]),
                ("dark".to_string(), [50, 100, 160, 255]),
            ]),
        });
        
        // Default typography tokens
        self.typography.insert("heading-1".to_string(), TypographyToken {
            name: "heading-1".to_string(),
            font_family: "system-ui".to_string(),
            font_size: 32.0,
            font_weight: FontWeight::Bold,
            line_height: 1.2,
            letter_spacing: -0.02,
            text_transform: TextTransform::None,
            usage_contexts: vec!["page-titles".to_string(), "section-headers".to_string()],
            responsive_sizes: HashMap::from([
                ("mobile".to_string(), 24.0),
                ("tablet".to_string(), 28.0),
                ("desktop".to_string(), 32.0),
            ]),
        });
        
        // Default spacing tokens
        self.spacing.insert("xs".to_string(), SpacingToken {
            name: "xs".to_string(),
            value: 4.0,
            scale: 0.25,
            usage_contexts: vec!["tight-spacing".to_string(), "icon-margins".to_string()],
            responsive_values: HashMap::from([
                ("mobile".to_string(), 2.0),
                ("desktop".to_string(), 4.0),
            ]),
        });
        
        // Default shadow tokens
        self.shadows.insert("elevation-1".to_string(), ShadowToken {
            name: "elevation-1".to_string(),
            offset_x: 0.0,
            offset_y: 2.0,
            blur_radius: 4.0,
            spread_radius: 0.0,
            color: [0, 0, 0, 26], // 10% opacity black
            inset: false,
            elevation: 1,
        });
        
        // Default animation tokens
        self.animations.insert("fast".to_string(), AnimationToken {
            name: "fast".to_string(),
            duration: 150,
            easing: "ease-out".to_string(),
            delay: 0,
            usage_contexts: vec!["hover-states".to_string(), "button-feedback".to_string()],
        });
    }
    
    /// Find matching color token for a value
    pub fn find_color_token(&self, color: &[u8; 4]) -> Option<&ColorToken> {
        self.colors.values().find(|token| {
            // Check main value and variants
            if token.value == *color {
                return true;
            }
            
            token.variants.values().any(|variant| variant == color)
        })
    }
    
    /// Find closest color token for a value
    pub fn find_closest_color_token(&self, color: &[u8; 4]) -> Option<(&ColorToken, f32)> {
        let mut closest: Option<(&ColorToken, f32)> = None;
        
        for token in self.colors.values() {
            let distance = self.color_distance(&token.value, color);
            
            if let Some((_, current_distance)) = &closest {
                if distance < *current_distance {
                    closest = Some((token, distance));
                }
            } else {
                closest = Some((token, distance));
            }
            
            // Also check variants
            for variant in token.variants.values() {
                let variant_distance = self.color_distance(variant, color);
                if let Some((_, current_distance)) = &closest {
                    if variant_distance < *current_distance {
                        closest = Some((token, variant_distance));
                    }
                }
            }
        }
        
        closest
    }
    
    /// Calculate color distance (simplified RGB distance)
    fn color_distance(&self, a: &[u8; 4], b: &[u8; 4]) -> f32 {
        let dr = (a[0] as f32 - b[0] as f32).powi(2);
        let dg = (a[1] as f32 - b[1] as f32).powi(2);
        let db = (a[2] as f32 - b[2] as f32).powi(2);
        let da = (a[3] as f32 - b[3] as f32).powi(2);
        
        (dr + dg + db + da).sqrt()
    }
}

impl TokenSuggestionEngine {
    /// Generate suggestions for a property value
    pub fn generate_suggestions(&mut self, property_name: &str, current_value: &PropertyValue, tokens: &DesignTokens) {
        self.suggestions.clear();
        
        match current_value {
            PropertyValue::Color(color) => {
                self.generate_color_suggestions(property_name, color, tokens);
            }
            PropertyValue::Number(number) => {
                self.generate_spacing_suggestions(property_name, *number, tokens);
            }
            PropertyValue::String(text) => {
                self.generate_typography_suggestions(property_name, text, tokens);
            }
            _ => {}
        }
        
        // Sort suggestions by confidence
        self.suggestions.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
    }
    
    /// Generate color token suggestions
    fn generate_color_suggestions(&mut self, property_name: &str, color: &[u8; 4], tokens: &DesignTokens) {
        // Exact match
        if let Some(token) = tokens.find_color_token(color) {
            self.suggestions.push(TokenSuggestion {
                property_name: property_name.to_string(),
                token_name: token.name.clone(),
                token_value: PropertyValue::Color(*color),
                reason: "Exact token match found".to_string(),
                confidence: 1.0,
                suggestion_type: SuggestionType::ExactMatch,
            });
        }
        
        // Closest match
        if let Some((closest_token, distance)) = tokens.find_closest_color_token(color) {
            if distance > 0.0 && distance < 50.0 { // Reasonable similarity threshold
                let confidence = 1.0 - (distance / 50.0);
                self.suggestions.push(TokenSuggestion {
                    property_name: property_name.to_string(),
                    token_name: closest_token.name.clone(),
                    token_value: PropertyValue::Color(closest_token.value),
                    reason: format!("Similar color found (distance: {:.1})", distance),
                    confidence,
                    suggestion_type: SuggestionType::SimilarValue,
                });
            }
        }
        
        // Accessibility suggestions
        for token in tokens.colors.values() {
            if token.accessibility.wcag_level == "AAA" && property_name.contains("text") {
                self.suggestions.push(TokenSuggestion {
                    property_name: property_name.to_string(),
                    token_name: token.name.clone(),
                    token_value: PropertyValue::Color(token.value),
                    reason: "High accessibility compliance".to_string(),
                    confidence: 0.8,
                    suggestion_type: SuggestionType::AccessibilityImprovement,
                });
            }
        }
    }
    
    /// Generate spacing token suggestions
    fn generate_spacing_suggestions(&mut self, property_name: &str, value: f32, tokens: &DesignTokens) {
        for (token_name, token) in &tokens.spacing {
            let distance = (token.value - value).abs();
            
            if distance < 0.1 {
                // Exact match
                self.suggestions.push(TokenSuggestion {
                    property_name: property_name.to_string(),
                    token_name: token_name.clone(),
                    token_value: PropertyValue::Number(token.value),
                    reason: "Exact spacing token match".to_string(),
                    confidence: 1.0,
                    suggestion_type: SuggestionType::ExactMatch,
                });
            } else if distance < 8.0 {
                // Close match
                let confidence = 1.0 - (distance / 8.0);
                self.suggestions.push(TokenSuggestion {
                    property_name: property_name.to_string(),
                    token_name: token_name.clone(),
                    token_value: PropertyValue::Number(token.value),
                    reason: format!("Similar spacing value (diff: {:.1}px)", distance),
                    confidence,
                    suggestion_type: SuggestionType::SimilarValue,
                });
            }
        }
    }
    
    /// Generate typography token suggestions
    fn generate_typography_suggestions(&mut self, property_name: &str, _text: &str, tokens: &DesignTokens) {
        if property_name.contains("font") {
            for (token_name, token) in &tokens.typography {
                if property_name.contains("family") {
                    self.suggestions.push(TokenSuggestion {
                        property_name: property_name.to_string(),
                        token_name: token_name.clone(),
                        token_value: PropertyValue::String(token.font_family.clone()),
                        reason: "Design system font family".to_string(),
                        confidence: 0.8,
                        suggestion_type: SuggestionType::BestPractice,
                    });
                }
            }
        }
    }
}

impl DesignSystemValidator {
    /// Initialize default validation rules
    fn initialize_default_rules(&mut self) {
        self.rules.push(ValidationRule {
            name: "Use Design Tokens for Colors".to_string(),
            description: "Colors should use design tokens instead of hardcoded values".to_string(),
            applicable_properties: vec!["color".to_string(), "background_color".to_string()],
            applicable_components: vec![], // Apply to all components
            severity: RuleSeverity::Warning,
            rule_type: RuleType::RequireDesignTokens,
        });
        
        self.rules.push(ValidationRule {
            name: "Text Color Accessibility".to_string(),
            description: "Text colors must meet WCAG AA contrast requirements".to_string(),
            applicable_properties: vec!["color".to_string()],
            applicable_components: vec!["Label".to_string(), "Text".to_string()],
            severity: RuleSeverity::Error,
            rule_type: RuleType::AccessibilityCompliance,
        });
        
        self.rules.push(ValidationRule {
            name: "Consistent Spacing".to_string(),
            description: "Spacing should use design token values".to_string(),
            applicable_properties: vec!["margin".to_string(), "padding".to_string()],
            applicable_components: vec![],
            severity: RuleSeverity::Info,
            rule_type: RuleType::ConsistencyCheck,
        });
    }
    
    /// Validate component against design system rules
    pub fn validate_component(
        &mut self,
        component_id: usize,
        component_type: &str,
        properties: &HashMap<String, PropertyValue>,
        tokens: &DesignTokens,
    ) {
        self.violations.retain(|v| v.component_id != component_id);
        
        for rule in &self.rules {
            // Check if rule applies to this component type
            if !rule.applicable_components.is_empty() && !rule.applicable_components.contains(&component_type.to_string()) {
                continue;
            }
            
            // Check each applicable property
            for property_name in &rule.applicable_properties {
                if let Some(value) = properties.get(property_name) {
                    if let Some(violation) = self.check_rule_violation(component_id, property_name, value, rule, tokens) {
                        self.violations.push(violation);
                    }
                }
            }
        }
    }
    
    /// Check if a specific rule is violated
    fn check_rule_violation(
        &self,
        component_id: usize,
        property_name: &str,
        value: &PropertyValue,
        rule: &ValidationRule,
        tokens: &DesignTokens,
    ) -> Option<DesignViolation> {
        match &rule.rule_type {
            RuleType::RequireDesignTokens => {
                match value {
                    PropertyValue::Color(color) => {
                        if tokens.find_color_token(color).is_none() {
                            let suggested_fix = tokens.find_closest_color_token(color)
                                .map(|(token, _)| PropertyValue::Color(token.value));
                            
                            return Some(DesignViolation {
                                component_id,
                                property_name: property_name.to_string(),
                                current_value: value.clone(),
                                violated_rule: rule.name.clone(),
                                severity: rule.severity.clone(),
                                suggested_fix,
                                explanation: "This color is not from the design token system".to_string(),
                            });
                        }
                    }
                    PropertyValue::Number(num) => {
                        if property_name.contains("margin") || property_name.contains("padding") {
                            let has_matching_token = tokens.spacing.values().any(|token| (token.value - num).abs() < 0.1);
                            if !has_matching_token {
                                return Some(DesignViolation {
                                    component_id,
                                    property_name: property_name.to_string(),
                                    current_value: value.clone(),
                                    violated_rule: rule.name.clone(),
                                    severity: rule.severity.clone(),
                                    suggested_fix: None,
                                    explanation: "This spacing value is not from the design token system".to_string(),
                                });
                            }
                        }
                    }
                    _ => {}
                }
            }
            RuleType::AccessibilityCompliance => {
                if let PropertyValue::Color(color) = value {
                    if let Some(token) = tokens.find_color_token(color) {
                        if !token.accessibility.text_suitable && property_name == "color" {
                            return Some(DesignViolation {
                                component_id,
                                property_name: property_name.to_string(),
                                current_value: value.clone(),
                                violated_rule: rule.name.clone(),
                                severity: rule.severity.clone(),
                                suggested_fix: None,
                                explanation: format!("Color has insufficient contrast ratio: {}", token.accessibility.contrast_ratio),
                            });
                        }
                    }
                }
            }
            _ => {}
        }
        
        None
    }
    
    /// Get violations by severity
    pub fn get_violations_by_severity(&self, severity: RuleSeverity) -> Vec<&DesignViolation> {
        self.violations.iter().filter(|v| v.severity == severity).collect()
    }
}

impl DesignSystemIntegration {
    /// Create a new design system integration
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Generate suggestions for a property
    pub fn generate_suggestions(&mut self, property_name: &str, value: &PropertyValue) {
        self.token_suggestions.generate_suggestions(property_name, value, &self.design_tokens);
    }
    
    /// Apply a token suggestion
    pub fn apply_suggestion(&mut self, suggestion: &TokenSuggestion, component_id: usize) -> bool {
        if self.auto_apply.enabled && 
           suggestion.confidence >= self.auto_apply.preferences.min_confidence &&
           self.auto_apply.preferences.auto_apply_properties.contains(&suggestion.property_name) {
            
            // Record the application
            self.auto_apply.application_history.push(AutoApplicationRecord {
                timestamp: chrono::Utc::now(),
                component_id,
                property_name: suggestion.property_name.clone(),
                old_value: suggestion.token_value.clone(), // Would be actual old value
                new_value: suggestion.token_value.clone(),
                applied_token: suggestion.token_name.clone(),
                reason: suggestion.reason.clone(),
            });
            
            true
        } else {
            false
        }
    }
    
    /// Render design system integration UI
    pub fn render_ui(&mut self, ui: &mut Ui) {
        ui.heading("Design System Integration");
        
        // Token suggestions
        if !self.token_suggestions.suggestions.is_empty() {
            ui.collapsing("Token Suggestions", |ui| {
                for suggestion in &self.token_suggestions.suggestions {
                    self.render_suggestion_ui(ui, suggestion);
                }
            });
        }
        
        // Validation results
        if !self.validator.violations.is_empty() {
            ui.collapsing("Design Violations", |ui| {
                let errors = self.validator.get_violations_by_severity(RuleSeverity::Error);
                let warnings = self.validator.get_violations_by_severity(RuleSeverity::Warning);
                
                if !errors.is_empty() {
                    ui.colored_label(Color32::RED, format!("Errors: {}", errors.len()));
                    for violation in errors {
                        self.render_violation_ui(ui, violation);
                    }
                }
                
                if !warnings.is_empty() {
                    ui.colored_label(Color32::YELLOW, format!("Warnings: {}", warnings.len()));
                    for violation in warnings {
                        self.render_violation_ui(ui, violation);
                    }
                }
            });
        }
        
        // Auto-apply settings
        ui.collapsing("Auto-Apply Settings", |ui| {
            ui.checkbox(&mut self.auto_apply.enabled, "Enable auto-apply");
            
            if self.auto_apply.enabled {
                ui.horizontal(|ui| {
                    ui.label("Min confidence:");
                    ui.add(egui::Slider::new(&mut self.auto_apply.preferences.min_confidence, 0.0..=1.0));
                });
                
                ui.checkbox(&mut self.auto_apply.preferences.apply_exact_matches, "Apply exact matches");
                ui.checkbox(&mut self.auto_apply.preferences.apply_accessibility_fixes, "Apply accessibility fixes");
            }
        });
    }
    
    /// Render suggestion UI
    fn render_suggestion_ui(&self, ui: &mut Ui, suggestion: &TokenSuggestion) {
        ui.horizontal(|ui| {
            // Confidence indicator
            let confidence_color = if suggestion.confidence > 0.8 {
                Color32::GREEN
            } else if suggestion.confidence > 0.6 {
                Color32::YELLOW
            } else {
                Color32::GRAY
            };
            
            ui.colored_label(confidence_color, format!("{:.0}%", suggestion.confidence * 100.0));
            
            // Suggestion details
            ui.label(&suggestion.token_name);
            ui.label(&suggestion.reason);
            
            // Apply button
            if ui.small_button("Apply").clicked() {
                // Apply suggestion
            }
        });
    }
    
    /// Render violation UI
    fn render_violation_ui(&self, ui: &mut Ui, violation: &DesignViolation) {
        ui.horizontal(|ui| {
            // Severity indicator
            let severity_color = match violation.severity {
                RuleSeverity::Error => Color32::RED,
                RuleSeverity::Warning => Color32::YELLOW,
                RuleSeverity::Info => Color32::BLUE,
            };
            
            ui.colored_label(severity_color, format!("{:?}", violation.severity));
            
            // Violation details
            ui.label(&violation.property_name);
            ui.label(&violation.explanation);
            
            // Fix button if suggestion available
            if violation.suggested_fix.is_some() {
                if ui.small_button("Fix").clicked() {
                    // Apply suggested fix
                }
            }
        });
    }
}