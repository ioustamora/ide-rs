//! Design Token System for Consistent Styling
//!
//! This module provides a comprehensive design token system that enables
//! consistent styling across components and frameworks.

use egui::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Design token system for consistent styling
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DesignTokenSystem {
    /// Color tokens
    pub colors: HashMap<String, ColorToken>,
    /// Typography tokens
    pub typography: HashMap<String, TypographyToken>,
    /// Spacing tokens
    pub spacing: HashMap<String, SpacingToken>,
    /// Shadow tokens
    pub shadows: HashMap<String, ShadowToken>,
    /// Border radius tokens
    pub border_radius: HashMap<String, f32>,
    /// Animation tokens
    pub animations: HashMap<String, AnimationToken>,
    /// Custom tokens
    pub custom_tokens: HashMap<String, TokenValue>,
}

/// Color token definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColorToken {
    /// Token name
    pub name: String,
    /// Primary color value
    pub value: [u8; 4], // RGBA
    /// Color variants (light, dark, etc.)
    pub variants: HashMap<String, [u8; 4]>,
    /// Description or usage notes
    pub description: String,
    /// Token category
    pub category: String,
}

/// Typography token definition
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
}

/// Spacing token definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpacingToken {
    /// Token name
    pub name: String,
    /// Spacing value in pixels
    pub value: f32,
    /// Relative scale factor
    pub scale: f32,
}

/// Shadow token definition
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
    pub color: [u8; 4], // RGBA
    /// Whether shadow is inset
    pub inset: bool,
}

/// Animation token definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnimationToken {
    /// Token name
    pub name: String,
    /// Animation duration in milliseconds
    pub duration: u32,
    /// Animation easing function
    pub easing: EasingFunction,
    /// Animation delay in milliseconds
    pub delay: u32,
    /// Animation iteration count
    pub iteration_count: AnimationIteration,
    /// Animation direction
    pub direction: AnimationDirection,
    /// Animation fill mode
    pub fill_mode: AnimationFillMode,
}

/// Token value for custom tokens
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TokenValue {
    String(String),
    Number(f32),
    Boolean(bool),
    Color([u8; 4]),
    Array(Vec<TokenValue>),
    Object(HashMap<String, TokenValue>),
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

/// Animation easing function
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EasingFunction {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier(f32, f32, f32, f32),
}

/// Animation iteration count
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AnimationIteration {
    Count(u32),
    Infinite,
}

/// Animation direction
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

/// Animation fill mode
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AnimationFillMode {
    None,
    Forwards,
    Backwards,
    Both,
}

impl Default for DesignTokenSystem {
    fn default() -> Self {
        let mut system = Self {
            colors: HashMap::new(),
            typography: HashMap::new(),
            spacing: HashMap::new(),
            shadows: HashMap::new(),
            border_radius: HashMap::new(),
            animations: HashMap::new(),
            custom_tokens: HashMap::new(),
        };
        
        system.initialize_default_tokens();
        system
    }
}

impl DesignTokenSystem {
    /// Create a new design token system
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Initialize default design tokens
    fn initialize_default_tokens(&mut self) {
        // Default color tokens
        self.add_color_token(ColorToken {
            name: "primary".to_string(),
            value: [70, 130, 200, 255],
            variants: HashMap::from([
                ("light".to_string(), [120, 170, 230, 255]),
                ("dark".to_string(), [50, 100, 160, 255]),
            ]),
            description: "Primary brand color".to_string(),
            category: "brand".to_string(),
        });
        
        // Default typography tokens
        self.add_typography_token(TypographyToken {
            name: "heading-1".to_string(),
            font_family: "system-ui".to_string(),
            font_size: 32.0,
            font_weight: FontWeight::Bold,
            line_height: 1.2,
            letter_spacing: -0.02,
            text_transform: TextTransform::None,
        });
        
        // Default spacing tokens
        self.add_spacing_token(SpacingToken {
            name: "xs".to_string(),
            value: 4.0,
            scale: 0.25,
        });
        
        // Default shadow tokens
        self.add_shadow_token(ShadowToken {
            name: "elevation-1".to_string(),
            offset_x: 0.0,
            offset_y: 2.0,
            blur_radius: 4.0,
            spread_radius: 0.0,
            color: [0, 0, 0, 26], // 10% opacity black
            inset: false,
        });
    }
    
    /// Add a color token
    pub fn add_color_token(&mut self, token: ColorToken) {
        self.colors.insert(token.name.clone(), token);
    }
    
    /// Add a typography token
    pub fn add_typography_token(&mut self, token: TypographyToken) {
        self.typography.insert(token.name.clone(), token);
    }
    
    /// Add a spacing token
    pub fn add_spacing_token(&mut self, token: SpacingToken) {
        self.spacing.insert(token.name.clone(), token);
    }
    
    /// Add a shadow token
    pub fn add_shadow_token(&mut self, token: ShadowToken) {
        self.shadows.insert(token.name.clone(), token);
    }
    
    /// Get color by token name
    pub fn get_color(&self, name: &str) -> Option<Color32> {
        self.colors.get(name).map(|token| {
            Color32::from_rgba_premultiplied(
                token.value[0],
                token.value[1], 
                token.value[2],
                token.value[3]
            )
        })
    }
    
    /// Get spacing value by token name
    pub fn get_spacing(&self, name: &str) -> Option<f32> {
        self.spacing.get(name).map(|token| token.value)
    }
    
    /// Export tokens as JSON
    pub fn export_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
    
    /// Import tokens from JSON
    pub fn import_json(&mut self, json: &str) -> serde_json::Result<()> {
        let imported: DesignTokenSystem = serde_json::from_str(json)?;
        *self = imported;
        Ok(())
    }
}