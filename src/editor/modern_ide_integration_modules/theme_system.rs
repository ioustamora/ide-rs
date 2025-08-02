//! Theme System for Design Consistency
//!
//! This module provides comprehensive theming capabilities including
//! light/dark modes, custom themes, and design system integration.

use egui::*;
use std::collections::HashMap;

/// Theme management system
#[derive(Clone, Debug)]
pub struct ThemeSystem {
    /// Available themes
    pub themes: HashMap<String, Theme>,
    /// Currently active theme
    pub active_theme: String,
    /// Theme customization settings
    pub customization: ThemeCustomization,
    /// Auto theme switching settings
    pub auto_switching: AutoThemeSwitching,
}

/// Theme definition
#[derive(Clone, Debug)]
pub struct Theme {
    /// Theme name
    pub name: String,
    /// Theme description
    pub description: String,
    /// Theme type (light, dark, auto)
    pub theme_type: ThemeType,
    /// Color palette
    pub colors: ColorPalette,
    /// Typography settings
    pub typography: TypographyTheme,
    /// Spacing scale
    pub spacing: SpacingTheme,
    /// Border radius scale
    pub border_radius: BorderRadiusTheme,
    /// Shadow definitions
    pub shadows: ShadowTheme,
    /// Animation settings
    pub animations: AnimationTheme,
}

/// Theme customization settings
#[derive(Clone, Debug)]
pub struct ThemeCustomization {
    /// Custom color overrides
    pub color_overrides: HashMap<String, Color32>,
    /// Custom spacing overrides
    pub spacing_overrides: HashMap<String, f32>,
    /// Custom font family
    pub font_family: Option<String>,
    /// Custom font size scale
    pub font_size_scale: f32,
    /// Custom border radius scale
    pub border_radius_scale: f32,
}

/// Auto theme switching settings
#[derive(Clone, Debug)]
pub struct AutoThemeSwitching {
    /// Whether auto switching is enabled
    pub enabled: bool,
    /// Light theme for day mode
    pub light_theme: String,
    /// Dark theme for night mode
    pub dark_theme: String,
    /// Time to switch to light theme (24h format)
    pub light_time: String,
    /// Time to switch to dark theme (24h format)
    pub dark_time: String,
}

/// Theme type enumeration
#[derive(Clone, Debug)]
pub enum ThemeType {
    Light,
    Dark,
    Auto,
    Custom,
}

/// Color palette for themes
#[derive(Clone, Debug)]
pub struct ColorPalette {
    /// Primary colors
    pub primary: ColorScale,
    /// Secondary colors
    pub secondary: ColorScale,
    /// Accent colors
    pub accent: ColorScale,
    /// Neutral colors
    pub neutral: ColorScale,
    /// Semantic colors
    pub semantic: SemanticColors,
    /// Background colors
    pub background: BackgroundColors,
    /// Text colors
    pub text: TextColors,
    /// Border colors
    pub border: BorderColors,
}

/// Color scale (50, 100, 200, ..., 900)
#[derive(Clone, Debug)]
pub struct ColorScale {
    pub c50: Color32,
    pub c100: Color32,
    pub c200: Color32,
    pub c300: Color32,
    pub c400: Color32,
    pub c500: Color32,
    pub c600: Color32,
    pub c700: Color32,
    pub c800: Color32,
    pub c900: Color32,
}

/// Semantic color definitions
#[derive(Clone, Debug)]
pub struct SemanticColors {
    /// Success color
    pub success: Color32,
    /// Warning color
    pub warning: Color32,
    /// Error color
    pub error: Color32,
    /// Info color
    pub info: Color32,
}

/// Background color definitions
#[derive(Clone, Debug)]
pub struct BackgroundColors {
    /// Primary background
    pub primary: Color32,
    /// Secondary background
    pub secondary: Color32,
    /// Tertiary background
    pub tertiary: Color32,
    /// Overlay background
    pub overlay: Color32,
}

/// Text color definitions
#[derive(Clone, Debug)]
pub struct TextColors {
    /// Primary text
    pub primary: Color32,
    /// Secondary text
    pub secondary: Color32,
    /// Tertiary text
    pub tertiary: Color32,
    /// Disabled text
    pub disabled: Color32,
    /// Inverse text
    pub inverse: Color32,
}

/// Border color definitions
#[derive(Clone, Debug)]
pub struct BorderColors {
    /// Default border
    pub default: Color32,
    /// Subtle border
    pub subtle: Color32,
    /// Strong border
    pub strong: Color32,
    /// Focus border
    pub focus: Color32,
}

/// Typography theme settings
#[derive(Clone, Debug)]
pub struct TypographyTheme {
    /// Font families
    pub font_families: HashMap<String, String>,
    /// Font sizes
    pub font_sizes: HashMap<String, f32>,
    /// Font weights
    pub font_weights: HashMap<String, FontWeight>,
    /// Line heights
    pub line_heights: HashMap<String, f32>,
    /// Letter spacing
    pub letter_spacing: HashMap<String, f32>,
}

/// Spacing theme settings
#[derive(Clone, Debug)]
pub struct SpacingTheme {
    /// Base spacing unit
    pub base_unit: f32,
    /// Spacing scale multipliers
    pub scale: HashMap<String, f32>,
}

/// Border radius theme settings
#[derive(Clone, Debug)]
pub struct BorderRadiusTheme {
    /// None (0px)
    pub none: f32,
    /// Small
    pub sm: f32,
    /// Medium
    pub md: f32,
    /// Large
    pub lg: f32,
    /// Extra large
    pub xl: f32,
    /// Full (pill shape)
    pub full: f32,
}

/// Shadow theme settings
#[derive(Clone, Debug)]
pub struct ShadowTheme {
    /// Shadow definitions by elevation
    pub elevations: HashMap<String, ShadowDefinition>,
}

/// Shadow definition
#[derive(Clone, Debug)]
pub struct ShadowDefinition {
    /// Shadow offset X
    pub offset_x: f32,
    /// Shadow offset Y
    pub offset_y: f32,
    /// Shadow blur
    pub blur: f32,
    /// Shadow spread
    pub spread: f32,
    /// Shadow color
    pub color: Color32,
}

/// Animation theme settings
#[derive(Clone, Debug)]
pub struct AnimationTheme {
    /// Animation durations
    pub durations: HashMap<String, u32>,
    /// Easing functions
    pub easings: HashMap<String, String>,
}

/// Font weight enumeration
#[derive(Clone, Debug)]
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
}

impl Default for ThemeSystem {
    fn default() -> Self {
        let mut system = Self {
            themes: HashMap::new(),
            active_theme: "default_light".to_string(),
            customization: ThemeCustomization::default(),
            auto_switching: AutoThemeSwitching::default(),
        };
        
        system.initialize_default_themes();
        system
    }
}

impl Default for ThemeCustomization {
    fn default() -> Self {
        Self {
            color_overrides: HashMap::new(),
            spacing_overrides: HashMap::new(),
            font_family: None,
            font_size_scale: 1.0,
            border_radius_scale: 1.0,
        }
    }
}

impl Default for AutoThemeSwitching {
    fn default() -> Self {
        Self {
            enabled: false,
            light_theme: "default_light".to_string(),
            dark_theme: "default_dark".to_string(),
            light_time: "08:00".to_string(),
            dark_time: "20:00".to_string(),
        }
    }
}

impl ThemeSystem {
    /// Create a new theme system
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Initialize default themes
    fn initialize_default_themes(&mut self) {
        // Default light theme
        self.add_theme(Theme {
            name: "default_light".to_string(),
            description: "Default light theme".to_string(),
            theme_type: ThemeType::Light,
            colors: ColorPalette::default_light(),
            typography: TypographyTheme::default(),
            spacing: SpacingTheme::default(),
            border_radius: BorderRadiusTheme::default(),
            shadows: ShadowTheme::default(),
            animations: AnimationTheme::default(),
        });
        
        // Default dark theme
        self.add_theme(Theme {
            name: "default_dark".to_string(),
            description: "Default dark theme".to_string(),
            theme_type: ThemeType::Dark,
            colors: ColorPalette::default_dark(),
            typography: TypographyTheme::default(),
            spacing: SpacingTheme::default(),
            border_radius: BorderRadiusTheme::default(),
            shadows: ShadowTheme::default(),
            animations: AnimationTheme::default(),
        });
    }
    
    /// Add a theme
    pub fn add_theme(&mut self, theme: Theme) {
        self.themes.insert(theme.name.clone(), theme);
    }
    
    /// Set active theme
    pub fn set_active_theme(&mut self, theme_name: &str) -> Result<(), ThemeError> {
        if self.themes.contains_key(theme_name) {
            self.active_theme = theme_name.to_string();
            Ok(())
        } else {
            Err(ThemeError::ThemeNotFound(theme_name.to_string()))
        }
    }
    
    /// Get active theme
    pub fn get_active_theme(&self) -> Option<&Theme> {
        self.themes.get(&self.active_theme)
    }
    
    /// Apply theme to egui context
    pub fn apply_to_egui(&self, ctx: &Context) {
        if let Some(theme) = self.get_active_theme() {
            let mut visuals = ctx.style().visuals.clone();
            
            // Apply color scheme
            match theme.theme_type {
                ThemeType::Light => {
                    visuals.dark_mode = false;
                    visuals.window_fill = theme.colors.background.primary;
                    visuals.panel_fill = theme.colors.background.secondary;
                }
                ThemeType::Dark => {
                    visuals.dark_mode = true;
                    visuals.window_fill = theme.colors.background.primary;
                    visuals.panel_fill = theme.colors.background.secondary;
                }
                _ => {}
            }
            
            // Apply text colors (text color is now in fg_stroke.color in egui 0.27)
            visuals.widgets.noninteractive.fg_stroke.color = theme.colors.text.primary;
            
            // Apply custom overrides
            for (name, color) in &self.customization.color_overrides {
                match name.as_str() {
                    "window_fill" => visuals.window_fill = *color,
                    "panel_fill" => visuals.panel_fill = *color,
                    "text_color" => visuals.widgets.noninteractive.fg_stroke.color = *color,
                    _ => {}
                }
            }
            
            ctx.set_visuals(visuals);
        }
    }
    
    /// Render theme system UI
    pub fn render_ui(&mut self, ui: &mut Ui) {
        ui.heading("Theme System");
        
        ui.horizontal(|ui| {
            ui.label("Active Theme:");
            egui::ComboBox::from_label("")
                .selected_text(&self.active_theme)
                .show_ui(ui, |ui| {
                    for (name, theme) in &self.themes {
                        if ui.selectable_value(&mut self.active_theme, name.clone(), &theme.name).clicked() {
                            // Theme changed
                        }
                    }
                });
        });
        
        ui.separator();
        
        ui.collapsing("Auto Theme Switching", |ui| {
            ui.checkbox(&mut self.auto_switching.enabled, "Enable auto switching");
            
            if self.auto_switching.enabled {
                ui.horizontal(|ui| {
                    ui.label("Light theme:");
                    ui.text_edit_singleline(&mut self.auto_switching.light_theme);
                });
                
                ui.horizontal(|ui| {
                    ui.label("Dark theme:");
                    ui.text_edit_singleline(&mut self.auto_switching.dark_theme);
                });
                
                ui.horizontal(|ui| {
                    ui.label("Switch to light at:");
                    ui.text_edit_singleline(&mut self.auto_switching.light_time);
                });
                
                ui.horizontal(|ui| {
                    ui.label("Switch to dark at:");
                    ui.text_edit_singleline(&mut self.auto_switching.dark_time);
                });
            }
        });
        
        ui.separator();
        
        ui.collapsing("Theme Customization", |ui| {
            ui.horizontal(|ui| {
                ui.label("Font size scale:");
                ui.add(egui::Slider::new(&mut self.customization.font_size_scale, 0.5..=2.0).step_by(0.1));
            });
            
            ui.horizontal(|ui| {
                ui.label("Border radius scale:");
                ui.add(egui::Slider::new(&mut self.customization.border_radius_scale, 0.0..=2.0).step_by(0.1));
            });
        });
    }
}

impl ColorPalette {
    /// Create default light color palette
    pub fn default_light() -> Self {
        Self {
            primary: ColorScale::blue(),
            secondary: ColorScale::gray(),
            accent: ColorScale::purple(),
            neutral: ColorScale::gray(),
            semantic: SemanticColors::default(),
            background: BackgroundColors::light(),
            text: TextColors::light(),
            border: BorderColors::light(),
        }
    }
    
    /// Create default dark color palette
    pub fn default_dark() -> Self {
        Self {
            primary: ColorScale::blue(),
            secondary: ColorScale::gray(),
            accent: ColorScale::purple(),
            neutral: ColorScale::gray(),
            semantic: SemanticColors::default(),
            background: BackgroundColors::dark(),
            text: TextColors::dark(),
            border: BorderColors::dark(),
        }
    }
}

impl ColorScale {
    /// Create blue color scale
    pub fn blue() -> Self {
        Self {
            c50: Color32::from_rgb(239, 246, 255),
            c100: Color32::from_rgb(219, 234, 254),
            c200: Color32::from_rgb(191, 219, 254),
            c300: Color32::from_rgb(147, 197, 253),
            c400: Color32::from_rgb(96, 165, 250),
            c500: Color32::from_rgb(59, 130, 246),
            c600: Color32::from_rgb(37, 99, 235),
            c700: Color32::from_rgb(29, 78, 216),
            c800: Color32::from_rgb(30, 64, 175),
            c900: Color32::from_rgb(30, 58, 138),
        }
    }
    
    /// Create gray color scale
    pub fn gray() -> Self {
        Self {
            c50: Color32::from_rgb(249, 250, 251),
            c100: Color32::from_rgb(243, 244, 246),
            c200: Color32::from_rgb(229, 231, 235),
            c300: Color32::from_rgb(209, 213, 219),
            c400: Color32::from_rgb(156, 163, 175),
            c500: Color32::from_rgb(107, 114, 128),
            c600: Color32::from_rgb(75, 85, 99),
            c700: Color32::from_rgb(55, 65, 81),
            c800: Color32::from_rgb(31, 41, 55),
            c900: Color32::from_rgb(17, 24, 39),
        }
    }
    
    /// Create purple color scale
    pub fn purple() -> Self {
        Self {
            c50: Color32::from_rgb(245, 243, 255),
            c100: Color32::from_rgb(237, 233, 254),
            c200: Color32::from_rgb(221, 214, 254),
            c300: Color32::from_rgb(196, 181, 253),
            c400: Color32::from_rgb(167, 139, 250),
            c500: Color32::from_rgb(139, 92, 246),
            c600: Color32::from_rgb(124, 58, 237),
            c700: Color32::from_rgb(109, 40, 217),
            c800: Color32::from_rgb(91, 33, 182),
            c900: Color32::from_rgb(76, 29, 149),
        }
    }
}

impl Default for SemanticColors {
    fn default() -> Self {
        Self {
            success: Color32::from_rgb(34, 197, 94),
            warning: Color32::from_rgb(251, 191, 36),
            error: Color32::from_rgb(239, 68, 68),
            info: Color32::from_rgb(59, 130, 246),
        }
    }
}

impl BackgroundColors {
    /// Light mode backgrounds
    pub fn light() -> Self {
        Self {
            primary: Color32::WHITE,
            secondary: Color32::from_rgb(249, 250, 251),
            tertiary: Color32::from_rgb(243, 244, 246),
            overlay: Color32::from_rgba_premultiplied(0, 0, 0, 80),
        }
    }
    
    /// Dark mode backgrounds
    pub fn dark() -> Self {
        Self {
            primary: Color32::from_rgb(17, 24, 39),
            secondary: Color32::from_rgb(31, 41, 55),
            tertiary: Color32::from_rgb(55, 65, 81),
            overlay: Color32::from_rgba_premultiplied(0, 0, 0, 160),
        }
    }
}

impl TextColors {
    /// Light mode text colors
    pub fn light() -> Self {
        Self {
            primary: Color32::from_rgb(17, 24, 39),
            secondary: Color32::from_rgb(75, 85, 99),
            tertiary: Color32::from_rgb(156, 163, 175),
            disabled: Color32::from_rgb(209, 213, 219),
            inverse: Color32::WHITE,
        }
    }
    
    /// Dark mode text colors
    pub fn dark() -> Self {
        Self {
            primary: Color32::WHITE,
            secondary: Color32::from_rgb(209, 213, 219),
            tertiary: Color32::from_rgb(156, 163, 175),
            disabled: Color32::from_rgb(75, 85, 99),
            inverse: Color32::from_rgb(17, 24, 39),
        }
    }
}

impl BorderColors {
    /// Light mode border colors
    pub fn light() -> Self {
        Self {
            default: Color32::from_rgb(229, 231, 235),
            subtle: Color32::from_rgb(243, 244, 246),
            strong: Color32::from_rgb(156, 163, 175),
            focus: Color32::from_rgb(59, 130, 246),
        }
    }
    
    /// Dark mode border colors
    pub fn dark() -> Self {
        Self {
            default: Color32::from_rgb(55, 65, 81),
            subtle: Color32::from_rgb(31, 41, 55),
            strong: Color32::from_rgb(156, 163, 175),
            focus: Color32::from_rgb(59, 130, 246),
        }
    }
}

impl Default for TypographyTheme {
    fn default() -> Self {
        let mut font_families = HashMap::new();
        font_families.insert("sans".to_string(), "system-ui, sans-serif".to_string());
        font_families.insert("serif".to_string(), "Georgia, serif".to_string());
        font_families.insert("mono".to_string(), "Consolas, monospace".to_string());
        
        let mut font_sizes = HashMap::new();
        font_sizes.insert("xs".to_string(), 12.0);
        font_sizes.insert("sm".to_string(), 14.0);
        font_sizes.insert("base".to_string(), 16.0);
        font_sizes.insert("lg".to_string(), 18.0);
        font_sizes.insert("xl".to_string(), 20.0);
        font_sizes.insert("2xl".to_string(), 24.0);
        font_sizes.insert("3xl".to_string(), 30.0);
        font_sizes.insert("4xl".to_string(), 36.0);
        
        Self {
            font_families,
            font_sizes,
            font_weights: HashMap::new(),
            line_heights: HashMap::new(),
            letter_spacing: HashMap::new(),
        }
    }
}

impl Default for SpacingTheme {
    fn default() -> Self {
        let mut scale = HashMap::new();
        scale.insert("0".to_string(), 0.0);
        scale.insert("1".to_string(), 0.25);
        scale.insert("2".to_string(), 0.5);
        scale.insert("3".to_string(), 0.75);
        scale.insert("4".to_string(), 1.0);
        scale.insert("5".to_string(), 1.25);
        scale.insert("6".to_string(), 1.5);
        scale.insert("8".to_string(), 2.0);
        scale.insert("10".to_string(), 2.5);
        scale.insert("12".to_string(), 3.0);
        scale.insert("16".to_string(), 4.0);
        scale.insert("20".to_string(), 5.0);
        scale.insert("24".to_string(), 6.0);
        scale.insert("32".to_string(), 8.0);
        
        Self {
            base_unit: 16.0,
            scale,
        }
    }
}

impl Default for BorderRadiusTheme {
    fn default() -> Self {
        Self {
            none: 0.0,
            sm: 2.0,
            md: 4.0,
            lg: 8.0,
            xl: 12.0,
            full: 9999.0,
        }
    }
}

impl Default for ShadowTheme {
    fn default() -> Self {
        let mut elevations = HashMap::new();
        
        elevations.insert("1".to_string(), ShadowDefinition {
            offset_x: 0.0,
            offset_y: 1.0,
            blur: 3.0,
            spread: 0.0,
            color: Color32::from_rgba_premultiplied(0, 0, 0, 26), // 10% opacity
        });
        
        elevations.insert("2".to_string(), ShadowDefinition {
            offset_x: 0.0,
            offset_y: 4.0,
            blur: 6.0,
            spread: -1.0,
            color: Color32::from_rgba_premultiplied(0, 0, 0, 26), // 10% opacity
        });
        
        Self { elevations }
    }
}

impl Default for AnimationTheme {
    fn default() -> Self {
        let mut durations = HashMap::new();
        durations.insert("fast".to_string(), 150);
        durations.insert("normal".to_string(), 300);
        durations.insert("slow".to_string(), 500);
        
        let mut easings = HashMap::new();
        easings.insert("ease".to_string(), "cubic-bezier(0.25, 0.1, 0.25, 1)".to_string());
        easings.insert("ease-in".to_string(), "cubic-bezier(0.42, 0, 1, 1)".to_string());
        easings.insert("ease-out".to_string(), "cubic-bezier(0, 0, 0.58, 1)".to_string());
        easings.insert("ease-in-out".to_string(), "cubic-bezier(0.42, 0, 0.58, 1)".to_string());
        
        Self {
            durations,
            easings,
        }
    }
}

/// Theme error types
#[derive(Debug, thiserror::Error)]
pub enum ThemeError {
    #[error("Theme not found: {0}")]
    ThemeNotFound(String),
    #[error("Invalid theme configuration: {0}")]
    InvalidConfiguration(String),
}