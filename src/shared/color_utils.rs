//! Color Utilities
//!
//! Common color manipulation, palette generation, and accessibility
//! checking utilities used throughout the IDE.

use egui::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Color palette management system
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColorPalette {
    /// Palette name
    pub name: String,
    /// Primary colors
    pub primary_colors: Vec<Color32>,
    /// Secondary colors
    pub secondary_colors: Vec<Color32>,
    /// Accent colors
    pub accent_colors: Vec<Color32>,
    /// Neutral colors
    pub neutral_colors: Vec<Color32>,
    /// Semantic colors (success, warning, error, info)
    pub semantic_colors: HashMap<String, Color32>,
}

/// Color harmony generator
pub struct ColorHarmony {
    /// Base color for harmony calculations
    pub base_color: Color32,
    /// Generated harmony schemes
    pub schemes: HashMap<String, Vec<Color32>>,
}

/// Accessibility color checker
pub struct AccessibilityChecker {
    /// WCAG compliance levels
    pub compliance_levels: HashMap<String, f32>,
    /// Color blindness simulation
    pub colorblind_simulation: ColorblindSimulator,
}

/// Color blindness simulator
pub struct ColorblindSimulator {
    /// Protanopia simulation
    pub protanopia: bool,
    /// Deuteranopia simulation
    pub deuteranopia: bool,
    /// Tritanopia simulation
    pub tritanopia: bool,
}

/// HSL color representation
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HSL {
    /// Hue (0-360)
    pub h: f32,
    /// Saturation (0-1)
    pub s: f32,
    /// Lightness (0-1)
    pub l: f32,
}

/// HSV color representation
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HSV {
    /// Hue (0-360)
    pub h: f32,
    /// Saturation (0-1)
    pub s: f32,
    /// Value (0-1)
    pub v: f32,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            primary_colors: vec![
                Color32::from_rgb(59, 130, 246),   // Blue
                Color32::from_rgb(16, 185, 129),   // Green
                Color32::from_rgb(239, 68, 68),    // Red
            ],
            secondary_colors: vec![
                Color32::from_rgb(139, 92, 246),   // Purple
                Color32::from_rgb(245, 158, 11),   // Orange
                Color32::from_rgb(236, 72, 153),   // Pink
            ],
            accent_colors: vec![
                Color32::from_rgb(255, 193, 7),    // Yellow
                Color32::from_rgb(6, 182, 212),    // Cyan
            ],
            neutral_colors: vec![
                Color32::from_rgb(15, 23, 42),     // Slate 900
                Color32::from_rgb(51, 65, 85),     // Slate 700
                Color32::from_rgb(100, 116, 139),  // Slate 500
                Color32::from_rgb(148, 163, 184),  // Slate 400
                Color32::from_rgb(203, 213, 225),  // Slate 300
                Color32::from_rgb(241, 245, 249),  // Slate 100
                Color32::from_rgb(255, 255, 255),  // White
            ],
            semantic_colors: HashMap::from([
                ("success".to_string(), Color32::from_rgb(34, 197, 94)),
                ("warning".to_string(), Color32::from_rgb(251, 191, 36)),
                ("error".to_string(), Color32::from_rgb(239, 68, 68)),
                ("info".to_string(), Color32::from_rgb(59, 130, 246)),
            ]),
        }
    }
}

impl Default for ColorHarmony {
    fn default() -> Self {
        Self {
            base_color: Color32::from_rgb(59, 130, 246),
            schemes: HashMap::new(),
        }
    }
}

impl Default for AccessibilityChecker {
    fn default() -> Self {
        Self {
            compliance_levels: HashMap::from([
                ("AA".to_string(), 4.5),
                ("AAA".to_string(), 7.0),
            ]),
            colorblind_simulation: ColorblindSimulator::default(),
        }
    }
}

impl Default for ColorblindSimulator {
    fn default() -> Self {
        Self {
            protanopia: false,
            deuteranopia: false,
            tritanopia: false,
        }
    }
}

impl ColorPalette {
    /// Create a new color palette
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
    
    /// Add a primary color
    pub fn add_primary(&mut self, color: Color32) {
        self.primary_colors.push(color);
    }
    
    /// Add a semantic color
    pub fn add_semantic(&mut self, name: String, color: Color32) {
        self.semantic_colors.insert(name, color);
    }
    
    /// Get color by semantic name
    pub fn get_semantic(&self, name: &str) -> Option<Color32> {
        self.semantic_colors.get(name).copied()
    }
    
    /// Generate shades of a color
    pub fn generate_shades(&self, base_color: Color32, count: usize) -> Vec<Color32> {
        let mut shades = Vec::new();
        let hsl = rgb_to_hsl(base_color);
        
        for i in 0..count {
            let lightness = 0.1 + (0.8 * i as f32 / (count - 1) as f32);
            let shade_hsl = HSL {
                h: hsl.h,
                s: hsl.s,
                l: lightness,
            };
            shades.push(hsl_to_rgb(shade_hsl));
        }
        
        shades
    }
    
    /// Generate tints of a color
    pub fn generate_tints(&self, base_color: Color32, count: usize) -> Vec<Color32> {
        let mut tints = Vec::new();
        let hsl = rgb_to_hsl(base_color);
        
        for i in 0..count {
            let lightness = hsl.l + ((1.0 - hsl.l) * i as f32 / (count - 1) as f32);
            let tint_hsl = HSL {
                h: hsl.h,
                s: hsl.s,
                l: lightness,
            };
            tints.push(hsl_to_rgb(tint_hsl));
        }
        
        tints
    }
}

impl ColorHarmony {
    /// Create color harmony from base color
    pub fn new(base_color: Color32) -> Self {
        let mut harmony = Self {
            base_color,
            schemes: HashMap::new(),
        };
        harmony.generate_all_schemes();
        harmony
    }
    
    /// Generate all harmony schemes
    pub fn generate_all_schemes(&mut self) {
        self.schemes.insert("complementary".to_string(), self.complementary());
        self.schemes.insert("analogous".to_string(), self.analogous());
        self.schemes.insert("triadic".to_string(), self.triadic());
        self.schemes.insert("split_complementary".to_string(), self.split_complementary());
        self.schemes.insert("tetradic".to_string(), self.tetradic());
        self.schemes.insert("monochromatic".to_string(), self.monochromatic());
    }
    
    /// Generate complementary colors
    pub fn complementary(&self) -> Vec<Color32> {
        let hsl = rgb_to_hsl(self.base_color);
        let complement_hue = (hsl.h + 180.0) % 360.0;
        
        vec![
            self.base_color,
            hsl_to_rgb(HSL { h: complement_hue, s: hsl.s, l: hsl.l }),
        ]
    }
    
    /// Generate analogous colors
    pub fn analogous(&self) -> Vec<Color32> {
        let hsl = rgb_to_hsl(self.base_color);
        
        vec![
            hsl_to_rgb(HSL { h: (hsl.h - 30.0 + 360.0) % 360.0, s: hsl.s, l: hsl.l }),
            self.base_color,
            hsl_to_rgb(HSL { h: (hsl.h + 30.0) % 360.0, s: hsl.s, l: hsl.l }),
        ]
    }
    
    /// Generate triadic colors
    pub fn triadic(&self) -> Vec<Color32> {
        let hsl = rgb_to_hsl(self.base_color);
        
        vec![
            self.base_color,
            hsl_to_rgb(HSL { h: (hsl.h + 120.0) % 360.0, s: hsl.s, l: hsl.l }),
            hsl_to_rgb(HSL { h: (hsl.h + 240.0) % 360.0, s: hsl.s, l: hsl.l }),
        ]
    }
    
    /// Generate split complementary colors
    pub fn split_complementary(&self) -> Vec<Color32> {
        let hsl = rgb_to_hsl(self.base_color);
        let complement = (hsl.h + 180.0) % 360.0;
        
        vec![
            self.base_color,
            hsl_to_rgb(HSL { h: (complement - 30.0 + 360.0) % 360.0, s: hsl.s, l: hsl.l }),
            hsl_to_rgb(HSL { h: (complement + 30.0) % 360.0, s: hsl.s, l: hsl.l }),
        ]
    }
    
    /// Generate tetradic colors
    pub fn tetradic(&self) -> Vec<Color32> {
        let hsl = rgb_to_hsl(self.base_color);
        
        vec![
            self.base_color,
            hsl_to_rgb(HSL { h: (hsl.h + 90.0) % 360.0, s: hsl.s, l: hsl.l }),
            hsl_to_rgb(HSL { h: (hsl.h + 180.0) % 360.0, s: hsl.s, l: hsl.l }),
            hsl_to_rgb(HSL { h: (hsl.h + 270.0) % 360.0, s: hsl.s, l: hsl.l }),
        ]
    }
    
    /// Generate monochromatic colors
    pub fn monochromatic(&self) -> Vec<Color32> {
        let hsl = rgb_to_hsl(self.base_color);
        
        vec![
            hsl_to_rgb(HSL { h: hsl.h, s: hsl.s, l: 0.2 }),
            hsl_to_rgb(HSL { h: hsl.h, s: hsl.s, l: 0.4 }),
            hsl_to_rgb(HSL { h: hsl.h, s: hsl.s, l: 0.6 }),
            hsl_to_rgb(HSL { h: hsl.h, s: hsl.s, l: 0.8 }),
        ]
    }
}

impl AccessibilityChecker {
    /// Check color contrast ratio
    pub fn contrast_ratio(&self, foreground: Color32, background: Color32) -> f32 {
        let fg_luminance = self.relative_luminance(foreground);
        let bg_luminance = self.relative_luminance(background);
        
        let lighter = fg_luminance.max(bg_luminance);
        let darker = fg_luminance.min(bg_luminance);
        
        (lighter + 0.05) / (darker + 0.05)
    }
    
    /// Check WCAG compliance
    pub fn check_wcag_compliance(&self, foreground: Color32, background: Color32, level: &str) -> bool {
        let ratio = self.contrast_ratio(foreground, background);
        let threshold = self.compliance_levels.get(level).unwrap_or(&4.5);
        ratio >= *threshold
    }
    
    /// Calculate relative luminance
    fn relative_luminance(&self, color: Color32) -> f32 {
        let r = self.linearize_rgb_component(color.r() as f32 / 255.0);
        let g = self.linearize_rgb_component(color.g() as f32 / 255.0);
        let b = self.linearize_rgb_component(color.b() as f32 / 255.0);
        
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }
    
    /// Linearize RGB component for luminance calculation
    fn linearize_rgb_component(&self, component: f32) -> f32 {
        if component <= 0.03928 {
            component / 12.92
        } else {
            ((component + 0.055) / 1.055).powf(2.4)
        }
    }
    
    /// Simulate color blindness
    pub fn simulate_colorblindness(&self, color: Color32, blindness_type: &str) -> Color32 {
        match blindness_type {
            "protanopia" => self.simulate_protanopia(color),
            "deuteranopia" => self.simulate_deuteranopia(color),
            "tritanopia" => self.simulate_tritanopia(color),
            _ => color,
        }
    }
    
    /// Simulate protanopia (red-blind)
    fn simulate_protanopia(&self, color: Color32) -> Color32 {
        let r = color.r() as f32 / 255.0;
        let g = color.g() as f32 / 255.0;
        let b = color.b() as f32 / 255.0;
        
        let new_r = 0.567 * r + 0.433 * g;
        let new_g = 0.558 * r + 0.442 * g;
        let new_b = 0.242 * g + 0.758 * b;
        
        Color32::from_rgb(
            (new_r * 255.0) as u8,
            (new_g * 255.0) as u8,
            (new_b * 255.0) as u8,
        )
    }
    
    /// Simulate deuteranopia (green-blind)
    fn simulate_deuteranopia(&self, color: Color32) -> Color32 {
        let r = color.r() as f32 / 255.0;
        let g = color.g() as f32 / 255.0;
        let b = color.b() as f32 / 255.0;
        
        let new_r = 0.625 * r + 0.375 * g;
        let new_g = 0.7 * r + 0.3 * g;
        let new_b = 0.3 * g + 0.7 * b;
        
        Color32::from_rgb(
            (new_r * 255.0) as u8,
            (new_g * 255.0) as u8,
            (new_b * 255.0) as u8,
        )
    }
    
    /// Simulate tritanopia (blue-blind)
    fn simulate_tritanopia(&self, color: Color32) -> Color32 {
        let r = color.r() as f32 / 255.0;
        let g = color.g() as f32 / 255.0;
        let b = color.b() as f32 / 255.0;
        
        let new_r = 0.95 * r + 0.05 * g;
        let new_g = 0.433 * g + 0.567 * b;
        let new_b = 0.475 * g + 0.525 * b;
        
        Color32::from_rgb(
            (new_r * 255.0) as u8,
            (new_g * 255.0) as u8,
            (new_b * 255.0) as u8,
        )
    }
}

/// Convert RGB to HSL
pub fn rgb_to_hsl(color: Color32) -> HSL {
    let r = color.r() as f32 / 255.0;
    let g = color.g() as f32 / 255.0;
    let b = color.b() as f32 / 255.0;
    
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    
    let l = (max + min) / 2.0;
    
    if delta == 0.0 {
        return HSL { h: 0.0, s: 0.0, l };
    }
    
    let s = if l < 0.5 {
        delta / (max + min)
    } else {
        delta / (2.0 - max - min)
    };
    
    let h = if max == r {
        ((g - b) / delta + if g < b { 6.0 } else { 0.0 }) * 60.0
    } else if max == g {
        ((b - r) / delta + 2.0) * 60.0
    } else {
        ((r - g) / delta + 4.0) * 60.0
    };
    
    HSL { h, s, l }
}

/// Convert HSL to RGB
pub fn hsl_to_rgb(hsl: HSL) -> Color32 {
    let c = (1.0 - (2.0 * hsl.l - 1.0).abs()) * hsl.s;
    let x = c * (1.0 - ((hsl.h / 60.0) % 2.0 - 1.0).abs());
    let m = hsl.l - c / 2.0;
    
    let (r_prime, g_prime, b_prime) = if hsl.h < 60.0 {
        (c, x, 0.0)
    } else if hsl.h < 120.0 {
        (x, c, 0.0)
    } else if hsl.h < 180.0 {
        (0.0, c, x)
    } else if hsl.h < 240.0 {
        (0.0, x, c)
    } else if hsl.h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    
    Color32::from_rgb(
        ((r_prime + m) * 255.0) as u8,
        ((g_prime + m) * 255.0) as u8,
        ((b_prime + m) * 255.0) as u8,
    )
}

/// Convert RGB to HSV
pub fn rgb_to_hsv(color: Color32) -> HSV {
    let r = color.r() as f32 / 255.0;
    let g = color.g() as f32 / 255.0;
    let b = color.b() as f32 / 255.0;
    
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    
    let v = max;
    let s = if max == 0.0 { 0.0 } else { delta / max };
    
    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        ((g - b) / delta + if g < b { 6.0 } else { 0.0 }) * 60.0
    } else if max == g {
        ((b - r) / delta + 2.0) * 60.0
    } else {
        ((r - g) / delta + 4.0) * 60.0
    };
    
    HSV { h, s, v }
}

/// Convert HSV to RGB
pub fn hsv_to_rgb(hsv: HSV) -> Color32 {
    let c = hsv.v * hsv.s;
    let x = c * (1.0 - ((hsv.h / 60.0) % 2.0 - 1.0).abs());
    let m = hsv.v - c;
    
    let (r_prime, g_prime, b_prime) = if hsv.h < 60.0 {
        (c, x, 0.0)
    } else if hsv.h < 120.0 {
        (x, c, 0.0)
    } else if hsv.h < 180.0 {
        (0.0, c, x)
    } else if hsv.h < 240.0 {
        (0.0, x, c)
    } else if hsv.h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    
    Color32::from_rgb(
        ((r_prime + m) * 255.0) as u8,
        ((g_prime + m) * 255.0) as u8,
        ((b_prime + m) * 255.0) as u8,
    )
}

/// Generate a color palette from a base color
pub fn generate_palette_from_base(base_color: Color32) -> ColorPalette {
    let harmony = ColorHarmony::new(base_color);
    
    ColorPalette {
        name: "Generated".to_string(),
        primary_colors: harmony.schemes.get("triadic").unwrap_or(&vec![base_color]).clone(),
        secondary_colors: harmony.schemes.get("analogous").unwrap_or(&vec![]).clone(),
        accent_colors: harmony.schemes.get("complementary").unwrap_or(&vec![]).clone(),
        neutral_colors: ColorPalette::default().neutral_colors,
        semantic_colors: HashMap::from([
            ("success".to_string(), Color32::from_rgb(34, 197, 94)),
            ("warning".to_string(), Color32::from_rgb(251, 191, 36)),
            ("error".to_string(), Color32::from_rgb(239, 68, 68)),
            ("info".to_string(), base_color),
        ]),
    }
}