//! Validation Utilities
//!
//! Common validation functions, error handling, and data integrity
//! checking utilities used throughout the IDE.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use thiserror::Error;

/// Result of a validation operation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Whether validation passed
    pub is_valid: bool,
    /// Validation errors found
    pub errors: Vec<ValidationError>,
    /// Validation warnings
    pub warnings: Vec<ValidationWarning>,
    /// Additional context information
    pub context: HashMap<String, String>,
}

/// Validation error with detailed information
#[derive(Clone, Debug, Serialize, Deserialize, Error)]
#[error("{message}")]
pub struct ValidationError {
    /// Error code for programmatic handling
    pub code: String,
    /// Human-readable error message
    pub message: String,
    /// Field or property that failed validation
    pub field: Option<String>,
    /// Expected value or format
    pub expected: Option<String>,
    /// Actual value that was provided
    pub actual: Option<String>,
    /// Severity level
    pub severity: ValidationSeverity,
}

/// Validation warning
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidationWarning {
    /// Warning code
    pub code: String,
    /// Warning message
    pub message: String,
    /// Field that triggered the warning
    pub field: Option<String>,
    /// Suggested fix
    pub suggestion: Option<String>,
}

/// Severity levels for validation issues
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValidationSeverity {
    /// Critical error that prevents operation
    Critical,
    /// Error that should be fixed
    Error,
    /// Warning that should be reviewed
    Warning,
    /// Information for user awareness
    Info,
}

/// Generic validator trait
pub trait Validator<T> {
    /// Validate an item and return results
    fn validate(&self, item: &T) -> ValidationResult;
    
    /// Get validator name
    fn name(&self) -> &str;
    
    /// Get validator description
    fn description(&self) -> &str;
}

/// Component property validator
pub struct PropertyValidator {
    /// Validator name
    pub name: String,
    /// Property validation rules
    pub rules: Vec<PropertyValidationRule>,
    /// Required properties
    pub required_properties: Vec<String>,
    /// Property type constraints
    pub type_constraints: HashMap<String, PropertyTypeConstraint>,
}

/// Property validation rule
#[derive(Clone, Debug)]
pub struct PropertyValidationRule {
    /// Rule name
    pub name: String,
    /// Property this rule applies to
    pub property: String,
    /// Rule type
    pub rule_type: PropertyRuleType,
    /// Error message template
    pub error_message: String,
    /// Rule severity
    pub severity: ValidationSeverity,
}

/// Type of property validation rule
#[derive(Clone, Debug)]
pub enum PropertyRuleType {
    /// Value must be within range
    Range { min: f64, max: f64 },
    /// Value must match regex pattern
    Pattern(String),
    /// Value must be one of the specified options
    Enum(Vec<String>),
    /// Value must meet minimum length
    MinLength(usize),
    /// Value must not exceed maximum length
    MaxLength(usize),
    /// Custom validation function
    Custom(String),
    /// Value must be unique within collection
    Unique,
    /// Value must not be empty
    NotEmpty,
    /// Value must be a valid URL
    Url,
    /// Value must be a valid email
    Email,
    /// Value must be a valid hex color
    HexColor,
    /// Value must be a valid CSS unit
    CssUnit,
}

/// Property type constraint
#[derive(Clone, Debug)]
pub struct PropertyTypeConstraint {
    /// Expected property type
    pub expected_type: String,
    /// Whether the property is nullable
    pub nullable: bool,
    /// Default value if not provided
    pub default_value: Option<String>,
    /// Additional type-specific constraints
    pub constraints: HashMap<String, String>,
}

/// Layout validation rules
pub struct LayoutValidator {
    /// Minimum component dimensions
    pub min_dimensions: (f32, f32),
    /// Maximum component dimensions
    pub max_dimensions: (f32, f32),
    /// Overlap detection enabled
    pub check_overlaps: bool,
    /// Boundary checking enabled
    pub check_boundaries: bool,
}

/// Accessibility validator
pub struct AccessibilityValidator {
    /// WCAG compliance level
    pub wcag_level: String,
    /// Color contrast requirements
    pub contrast_requirements: HashMap<String, f32>,
    /// Minimum touch target sizes
    pub touch_target_sizes: HashMap<String, f32>,
    /// Required ARIA attributes
    pub required_aria: Vec<String>,
}

/// Design system validator
pub struct DesignSystemValidator {
    /// Allowed color values
    pub allowed_colors: Vec<String>,
    /// Spacing scale values
    pub spacing_scale: Vec<f32>,
    /// Typography scale
    pub typography_scale: Vec<f32>,
    /// Brand compliance rules
    pub brand_rules: Vec<BrandRule>,
}

/// Brand compliance rule
#[derive(Clone, Debug)]
pub struct BrandRule {
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Properties this rule affects
    pub properties: Vec<String>,
    /// Rule implementation
    pub rule_type: BrandRuleType,
}

/// Type of brand rule
#[derive(Clone, Debug)]
pub enum BrandRuleType {
    /// Must use approved colors only
    ApprovedColorsOnly,
    /// Must follow spacing guidelines
    SpacingGuidelines,
    /// Must use brand typography
    BrandTypography,
    /// Logo usage guidelines
    LogoUsage,
    /// Imagery guidelines
    ImageryGuidelines,
}

/// Performance validator
pub struct PerformanceValidator {
    /// Maximum component count warnings
    pub max_components: usize,
    /// Memory usage thresholds
    pub memory_thresholds: HashMap<String, usize>,
    /// Render performance checks
    pub performance_checks: Vec<PerformanceCheck>,
}

/// Performance check definition
#[derive(Clone, Debug)]
pub struct PerformanceCheck {
    /// Check name
    pub name: String,
    /// What metric to check
    pub metric: String,
    /// Threshold value
    pub threshold: f64,
    /// Warning message
    pub warning_message: String,
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            context: HashMap::new(),
        }
    }
}

impl ValidationResult {
    /// Create a successful validation result
    pub fn success() -> Self {
        Self::default()
    }
    
    /// Create a failed validation result with error
    pub fn error(error: ValidationError) -> Self {
        Self {
            is_valid: false,
            errors: vec![error],
            warnings: Vec::new(),
            context: HashMap::new(),
        }
    }
    
    /// Create a validation result with warnings
    pub fn warning(warning: ValidationWarning) -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: vec![warning],
            context: HashMap::new(),
        }
    }
    
    /// Add an error to the result
    pub fn add_error(&mut self, error: ValidationError) {
        self.errors.push(error);
        self.is_valid = false;
    }
    
    /// Add a warning to the result
    pub fn add_warning(&mut self, warning: ValidationWarning) {
        self.warnings.push(warning);
    }
    
    /// Add context information
    pub fn add_context(&mut self, key: String, value: String) {
        self.context.insert(key, value);
    }
    
    /// Merge another validation result
    pub fn merge(&mut self, other: ValidationResult) {
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
        self.context.extend(other.context);
        self.is_valid = self.is_valid && other.is_valid;
    }
    
    /// Get total issue count
    pub fn total_issues(&self) -> usize {
        self.errors.len() + self.warnings.len()
    }
    
    /// Check if has critical errors
    pub fn has_critical_errors(&self) -> bool {
        self.errors.iter().any(|e| e.severity == ValidationSeverity::Critical)
    }
}

impl ValidationError {
    /// Create a new validation error
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            field: None,
            expected: None,
            actual: None,
            severity: ValidationSeverity::Error,
        }
    }
    
    /// Set the field that failed validation
    pub fn with_field(mut self, field: String) -> Self {
        self.field = Some(field);
        self
    }
    
    /// Set expected and actual values
    pub fn with_values(mut self, expected: String, actual: String) -> Self {
        self.expected = Some(expected);
        self.actual = Some(actual);
        self
    }
    
    /// Set severity level
    pub fn with_severity(mut self, severity: ValidationSeverity) -> Self {
        self.severity = severity;
        self
    }
}

impl ValidationWarning {
    /// Create a new validation warning
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            field: None,
            suggestion: None,
        }
    }
    
    /// Set the field that triggered the warning
    pub fn with_field(mut self, field: String) -> Self {
        self.field = Some(field);
        self
    }
    
    /// Set a suggestion for fixing the issue
    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }
}

impl Default for PropertyValidator {
    fn default() -> Self {
        Self {
            name: "Default Property Validator".to_string(),
            rules: Vec::new(),
            required_properties: Vec::new(),
            type_constraints: HashMap::new(),
        }
    }
}

impl PropertyValidator {
    /// Create a new property validator
    pub fn new(name: String) -> Self {
        Self {
            name,
            rules: Vec::new(),
            required_properties: Vec::new(),
            type_constraints: HashMap::new(),
        }
    }
    
    /// Add a validation rule
    pub fn add_rule(&mut self, rule: PropertyValidationRule) {
        self.rules.push(rule);
    }
    
    /// Add a required property
    pub fn add_required_property(&mut self, property: String) {
        self.required_properties.push(property);
    }
    
    /// Add type constraint
    pub fn add_type_constraint(&mut self, property: String, constraint: PropertyTypeConstraint) {
        self.type_constraints.insert(property, constraint);
    }
    
    /// Validate property value
    pub fn validate_property(&self, property: &str, value: &str) -> ValidationResult {
        let mut result = ValidationResult::success();
        
        // Check rules for this property
        for rule in &self.rules {
            if rule.property == property {
                if !self.check_rule(rule, value) {
                    result.add_error(ValidationError::new(
                        format!("{}_{}", rule.name, property),
                        rule.error_message.clone(),
                    ).with_field(property.to_string())
                     .with_severity(rule.severity.clone()));
                }
            }
        }
        
        // Check type constraints
        if let Some(constraint) = self.type_constraints.get(property) {
            if let Some(type_error) = self.check_type_constraint(property, value, constraint) {
                result.add_error(type_error);
            }
        }
        
        result
    }
    
    /// Check if a rule passes for a value
    fn check_rule(&self, rule: &PropertyValidationRule, value: &str) -> bool {
        match &rule.rule_type {
            PropertyRuleType::Range { min, max } => {
                value.parse::<f64>().map_or(false, |v| v >= *min && v <= *max)
            }
            PropertyRuleType::Pattern(pattern) => {
                // Simple pattern matching (in real implementation, use regex crate)
                value.contains(pattern)
            }
            PropertyRuleType::Enum(options) => {
                options.contains(&value.to_string())
            }
            PropertyRuleType::MinLength(min_len) => {
                value.len() >= *min_len
            }
            PropertyRuleType::MaxLength(max_len) => {
                value.len() <= *max_len
            }
            PropertyRuleType::NotEmpty => {
                !value.trim().is_empty()
            }
            PropertyRuleType::Url => {
                value.starts_with("http://") || value.starts_with("https://")
            }
            PropertyRuleType::Email => {
                value.contains('@') && value.contains('.')
            }
            PropertyRuleType::HexColor => {
                value.starts_with('#') && value.len() == 7
            }
            PropertyRuleType::CssUnit => {
                value.ends_with("px") || value.ends_with("em") || value.ends_with("rem") || value.ends_with("%")
            }
            _ => true, // For other types, assume valid
        }
    }
    
    /// Check type constraint
    fn check_type_constraint(&self, property: &str, value: &str, constraint: &PropertyTypeConstraint) -> Option<ValidationError> {
        // Basic type checking based on constraint
        match constraint.expected_type.as_str() {
            "number" => {
                if value.parse::<f64>().is_err() {
                    Some(ValidationError::new(
                        "type_mismatch".to_string(),
                        format!("Expected number for property '{}'", property),
                    ).with_field(property.to_string())
                     .with_values("number".to_string(), value.to_string()))
                } else {
                    None
                }
            }
            "boolean" => {
                if !matches!(value, "true" | "false") {
                    Some(ValidationError::new(
                        "type_mismatch".to_string(),
                        format!("Expected boolean for property '{}'", property),
                    ).with_field(property.to_string())
                     .with_values("boolean".to_string(), value.to_string()))
                } else {
                    None
                }
            }
            _ => None, // For other types, assume valid
        }
    }
}

impl Default for LayoutValidator {
    fn default() -> Self {
        Self {
            min_dimensions: (1.0, 1.0),
            max_dimensions: (2000.0, 2000.0),
            check_overlaps: true,
            check_boundaries: true,
        }
    }
}

impl LayoutValidator {
    /// Validate component dimensions
    pub fn validate_dimensions(&self, width: f32, height: f32) -> ValidationResult {
        let mut result = ValidationResult::success();
        
        if width < self.min_dimensions.0 {
            result.add_error(ValidationError::new(
                "min_width".to_string(),
                format!("Width {} is below minimum {}", width, self.min_dimensions.0),
            ).with_field("width".to_string()));
        }
        
        if height < self.min_dimensions.1 {
            result.add_error(ValidationError::new(
                "min_height".to_string(),
                format!("Height {} is below minimum {}", height, self.min_dimensions.1),
            ).with_field("height".to_string()));
        }
        
        if width > self.max_dimensions.0 {
            result.add_warning(ValidationWarning::new(
                "max_width".to_string(),
                format!("Width {} exceeds recommended maximum {}", width, self.max_dimensions.0),
            ).with_field("width".to_string()));
        }
        
        if height > self.max_dimensions.1 {
            result.add_warning(ValidationWarning::new(
                "max_height".to_string(),
                format!("Height {} exceeds recommended maximum {}", height, self.max_dimensions.1),
            ).with_field("height".to_string()));
        }
        
        result
    }
}

impl Default for AccessibilityValidator {
    fn default() -> Self {
        Self {
            wcag_level: "AA".to_string(),
            contrast_requirements: HashMap::from([
                ("normal_text".to_string(), 4.5),
                ("large_text".to_string(), 3.0),
                ("ui_components".to_string(), 3.0),
            ]),
            touch_target_sizes: HashMap::from([
                ("button".to_string(), 44.0),
                ("link".to_string(), 44.0),
                ("input".to_string(), 44.0),
            ]),
            required_aria: vec![
                "role".to_string(),
                "aria-label".to_string(),
                "aria-describedby".to_string(),
            ],
        }
    }
}

impl AccessibilityValidator {
    /// Validate color contrast
    pub fn validate_contrast(&self, contrast_ratio: f32, text_type: &str) -> ValidationResult {
        let mut result = ValidationResult::success();
        
        if let Some(&required_ratio) = self.contrast_requirements.get(text_type) {
            if contrast_ratio < required_ratio {
                result.add_error(ValidationError::new(
                    "insufficient_contrast".to_string(),
                    format!("Contrast ratio {:.1} is below required {:.1} for {}", 
                           contrast_ratio, required_ratio, text_type),
                ).with_field("color_contrast".to_string())
                 .with_severity(ValidationSeverity::Critical));
            }
        }
        
        result
    }
    
    /// Validate touch target size
    pub fn validate_touch_target(&self, component_type: &str, width: f32, height: f32) -> ValidationResult {
        let mut result = ValidationResult::success();
        
        if let Some(&min_size) = self.touch_target_sizes.get(component_type) {
            if width < min_size || height < min_size {
                result.add_error(ValidationError::new(
                    "touch_target_too_small".to_string(),
                    format!("Touch target {}x{} is below minimum {}x{} for {}", 
                           width, height, min_size, min_size, component_type),
                ).with_field("dimensions".to_string())
                 .with_severity(ValidationSeverity::Error));
            }
        }
        
        result
    }
}

/// Validate a string is not empty
pub fn validate_not_empty(value: &str, field_name: &str) -> ValidationResult {
    if value.trim().is_empty() {
        ValidationResult::error(ValidationError::new(
            "empty_value".to_string(),
            format!("{} cannot be empty", field_name),
        ).with_field(field_name.to_string()))
    } else {
        ValidationResult::success()
    }
}

/// Validate a number is within range
pub fn validate_number_range(value: f64, min: f64, max: f64, field_name: &str) -> ValidationResult {
    if value < min || value > max {
        ValidationResult::error(ValidationError::new(
            "out_of_range".to_string(),
            format!("{} must be between {} and {}", field_name, min, max),
        ).with_field(field_name.to_string())
         .with_values(format!("{}-{}", min, max), value.to_string()))
    } else {
        ValidationResult::success()
    }
}

/// Validate an email address format
pub fn validate_email(email: &str, field_name: &str) -> ValidationResult {
    if email.contains('@') && email.contains('.') && email.len() > 5 {
        ValidationResult::success()
    } else {
        ValidationResult::error(ValidationError::new(
            "invalid_email".to_string(),
            format!("{} is not a valid email address", field_name),
        ).with_field(field_name.to_string()))
    }
}

/// Validate a URL format
pub fn validate_url(url: &str, field_name: &str) -> ValidationResult {
    if url.starts_with("http://") || url.starts_with("https://") {
        ValidationResult::success()
    } else {
        ValidationResult::error(ValidationError::new(
            "invalid_url".to_string(),
            format!("{} must be a valid URL", field_name),
        ).with_field(field_name.to_string()))
    }
}