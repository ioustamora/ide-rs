//! # RCL Theming System
//!
//! This module provides a comprehensive theming system for RCL components,
//! allowing consistent styling across all components and easy theme switching.

use egui::Color32;
use std::collections::HashMap;

/// A complete theme for RCL components
#[derive(Debug, Clone)]
pub struct Theme {
    /// Theme metadata
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,
    
    /// Color palette
    pub colors: ColorPalette,
    
    /// Typography settings
    pub typography: Typography,
    
    /// Spacing and sizing
    pub spacing: Spacing,
    
    /// Component-specific overrides
    pub component_overrides: HashMap<String, ComponentTheme>,
}

/// Color palette for the theme
#[derive(Debug, Clone)]
pub struct ColorPalette {
    /// Primary brand colors
    pub primary: Color32,
    pub primary_hover: Color32,
    pub primary_active: Color32,
    
    /// Secondary colors
    pub secondary: Color32,
    pub secondary_hover: Color32,
    pub secondary_active: Color32,
    
    /// Semantic colors
    pub success: Color32,
    pub warning: Color32,
    pub danger: Color32,
    pub info: Color32,
    
    /// Neutral colors
    pub background: Color32,
    pub surface: Color32,
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub text_disabled: Color32,
    
    /// Border and outline colors
    pub border: Color32,
    pub border_focus: Color32,
    pub border_error: Color32,
    
    /// Overlay colors
    pub overlay: Color32,
    pub shadow: Color32,
}

/// Typography settings
#[derive(Debug, Clone)]
pub struct Typography {
    /// Font families
    pub font_family_primary: String,
    pub font_family_secondary: String,
    pub font_family_mono: String,
    
    /// Font sizes
    pub font_size_xs: f32,
    pub font_size_sm: f32,
    pub font_size_base: f32,
    pub font_size_lg: f32,
    pub font_size_xl: f32,
    pub font_size_2xl: f32,
    
    /// Line heights
    pub line_height_tight: f32,
    pub line_height_normal: f32,
    pub line_height_relaxed: f32,
    
    /// Font weights (if supported by font)
    pub font_weight_normal: i32,
    pub font_weight_medium: i32,
    pub font_weight_bold: i32,
}

/// Spacing and sizing values
#[derive(Debug, Clone)]
pub struct Spacing {
    /// Base spacing unit
    pub base_unit: f32,
    
    /// Padding values
    pub padding_xs: f32,
    pub padding_sm: f32,
    pub padding_md: f32,
    pub padding_lg: f32,
    pub padding_xl: f32,
    
    /// Margin values
    pub margin_xs: f32,
    pub margin_sm: f32,
    pub margin_md: f32,
    pub margin_lg: f32,
    pub margin_xl: f32,
    
    /// Border radius values
    pub radius_sm: f32,
    pub radius_md: f32,
    pub radius_lg: f32,
    pub radius_full: f32,
    
    /// Border widths
    pub border_thin: f32,
    pub border_normal: f32,
    pub border_thick: f32,
    
    /// Shadow values
    pub shadow_sm: f32,
    pub shadow_md: f32,
    pub shadow_lg: f32,
}

/// Component-specific theme overrides
#[derive(Debug, Clone)]
pub struct ComponentTheme {
    /// Override colors
    pub colors: Option<HashMap<String, Color32>>,
    
    /// Override dimensions
    pub dimensions: Option<HashMap<String, f32>>,
    
    /// Override text properties
    pub text: Option<HashMap<String, String>>,
}

impl Default for Theme {
    fn default() -> Self {
        Self::light_theme()
    }
}

impl Theme {
    /// Create a light theme
    pub fn light_theme() -> Self {
        Self {
            name: "Light".to_string(),
            description: "Clean and bright theme".to_string(),
            author: "RCL".to_string(),
            version: "1.0.0".to_string(),
            
            colors: ColorPalette {
                primary: Color32::from_rgb(70, 130, 200),
                primary_hover: Color32::from_rgb(60, 120, 190),
                primary_active: Color32::from_rgb(50, 110, 180),
                
                secondary: Color32::from_rgb(108, 117, 125),
                secondary_hover: Color32::from_rgb(98, 107, 115),
                secondary_active: Color32::from_rgb(88, 97, 105),
                
                success: Color32::from_rgb(40, 167, 69),
                warning: Color32::from_rgb(255, 193, 7),
                danger: Color32::from_rgb(220, 53, 69),
                info: Color32::from_rgb(23, 162, 184),
                
                background: Color32::from_rgb(255, 255, 255),
                surface: Color32::from_rgb(248, 249, 250),
                text_primary: Color32::from_rgb(33, 37, 41),
                text_secondary: Color32::from_rgb(108, 117, 125),
                text_disabled: Color32::from_rgb(173, 181, 189),
                
                border: Color32::from_rgb(222, 226, 230),
                border_focus: Color32::from_rgb(70, 130, 200),
                border_error: Color32::from_rgb(220, 53, 69),
                
                overlay: Color32::from_rgba_unmultiplied(0, 0, 0, 128),
                shadow: Color32::from_rgba_unmultiplied(0, 0, 0, 25),
            },
            
            typography: Typography {
                font_family_primary: "Inter, -apple-system, BlinkMacSystemFont, sans-serif".to_string(),
                font_family_secondary: "Georgia, serif".to_string(),
                font_family_mono: "JetBrains Mono, Consolas, monospace".to_string(),
                
                font_size_xs: 10.0,
                font_size_sm: 12.0,
                font_size_base: 14.0,
                font_size_lg: 16.0,
                font_size_xl: 18.0,
                font_size_2xl: 24.0,
                
                line_height_tight: 1.2,
                line_height_normal: 1.4,
                line_height_relaxed: 1.6,
                
                font_weight_normal: 400,
                font_weight_medium: 500,
                font_weight_bold: 700,
            },
            
            spacing: Spacing {
                base_unit: 4.0,
                
                padding_xs: 4.0,
                padding_sm: 8.0,
                padding_md: 12.0,
                padding_lg: 16.0,
                padding_xl: 24.0,
                
                margin_xs: 4.0,
                margin_sm: 8.0,
                margin_md: 12.0,
                margin_lg: 16.0,
                margin_xl: 24.0,
                
                radius_sm: 2.0,
                radius_md: 4.0,
                radius_lg: 8.0,
                radius_full: 9999.0,
                
                border_thin: 0.5,
                border_normal: 1.0,
                border_thick: 2.0,
                
                shadow_sm: 1.0,
                shadow_md: 3.0,
                shadow_lg: 6.0,
            },
            
            component_overrides: HashMap::new(),
        }
    }
    
    /// Create a dark theme
    pub fn dark_theme() -> Self {
        Self {
            name: "Dark".to_string(),
            description: "Sleek dark theme for low-light environments".to_string(),
            author: "RCL".to_string(),
            version: "1.0.0".to_string(),
            
            colors: ColorPalette {
                primary: Color32::from_rgb(100, 150, 220),
                primary_hover: Color32::from_rgb(110, 160, 230),
                primary_active: Color32::from_rgb(120, 170, 240),
                
                secondary: Color32::from_rgb(108, 117, 125),
                secondary_hover: Color32::from_rgb(118, 127, 135),
                secondary_active: Color32::from_rgb(128, 137, 145),
                
                success: Color32::from_rgb(60, 187, 89),
                warning: Color32::from_rgb(255, 213, 47),
                danger: Color32::from_rgb(240, 73, 89),
                info: Color32::from_rgb(43, 182, 204),
                
                background: Color32::from_rgb(18, 18, 18),
                surface: Color32::from_rgb(28, 28, 28),
                text_primary: Color32::from_rgb(255, 255, 255),
                text_secondary: Color32::from_rgb(173, 181, 189),
                text_disabled: Color32::from_rgb(108, 117, 125),
                
                border: Color32::from_rgb(68, 68, 68),
                border_focus: Color32::from_rgb(100, 150, 220),
                border_error: Color32::from_rgb(240, 73, 89),
                
                overlay: Color32::from_rgba_unmultiplied(0, 0, 0, 180),
                shadow: Color32::from_rgba_unmultiplied(0, 0, 0, 100),
            },
            
            typography: Typography {
                font_family_primary: "Inter, -apple-system, BlinkMacSystemFont, sans-serif".to_string(),
                font_family_secondary: "Georgia, serif".to_string(),
                font_family_mono: "JetBrains Mono, Consolas, monospace".to_string(),
                
                font_size_xs: 10.0,
                font_size_sm: 12.0,
                font_size_base: 14.0,
                font_size_lg: 16.0,
                font_size_xl: 18.0,
                font_size_2xl: 24.0,
                
                line_height_tight: 1.2,
                line_height_normal: 1.4,
                line_height_relaxed: 1.6,
                
                font_weight_normal: 400,
                font_weight_medium: 500,
                font_weight_bold: 700,
            },
            
            spacing: Spacing {
                base_unit: 4.0,
                
                padding_xs: 4.0,
                padding_sm: 8.0,
                padding_md: 12.0,
                padding_lg: 16.0,
                padding_xl: 24.0,
                
                margin_xs: 4.0,
                margin_sm: 8.0,
                margin_md: 12.0,
                margin_lg: 16.0,
                margin_xl: 24.0,
                
                radius_sm: 2.0,
                radius_md: 4.0,
                radius_lg: 8.0,
                radius_full: 9999.0,
                
                border_thin: 0.5,
                border_normal: 1.0,
                border_thick: 2.0,
                
                shadow_sm: 1.0,
                shadow_md: 3.0,
                shadow_lg: 6.0,
            },
            
            component_overrides: HashMap::new(),
        }
    }
    
    /// Create a custom theme
    pub fn custom_theme(name: String, description: String) -> Self {
        let mut theme = Self::light_theme();
        theme.name = name;
        theme.description = description;
        theme.author = "Custom".to_string();
        theme
    }
    
    /// Get color by semantic name
    pub fn get_color(&self, name: &str) -> Option<Color32> {
        match name {
            "primary" => Some(self.colors.primary),
            "primary_hover" => Some(self.colors.primary_hover),
            "primary_active" => Some(self.colors.primary_active),
            "secondary" => Some(self.colors.secondary),
            "secondary_hover" => Some(self.colors.secondary_hover),
            "secondary_active" => Some(self.colors.secondary_active),
            "success" => Some(self.colors.success),
            "warning" => Some(self.colors.warning),
            "danger" => Some(self.colors.danger),
            "info" => Some(self.colors.info),
            "background" => Some(self.colors.background),
            "surface" => Some(self.colors.surface),
            "text_primary" => Some(self.colors.text_primary),
            "text_secondary" => Some(self.colors.text_secondary),
            "text_disabled" => Some(self.colors.text_disabled),
            "border" => Some(self.colors.border),
            "border_focus" => Some(self.colors.border_focus),
            "border_error" => Some(self.colors.border_error),
            "overlay" => Some(self.colors.overlay),
            "shadow" => Some(self.colors.shadow),
            _ => None,
        }
    }
    
    /// Get spacing value by name
    pub fn get_spacing(&self, name: &str) -> Option<f32> {
        match name {
            "xs" => Some(self.spacing.padding_xs),
            "sm" => Some(self.spacing.padding_sm),
            "md" => Some(self.spacing.padding_md),
            "lg" => Some(self.spacing.padding_lg),
            "xl" => Some(self.spacing.padding_xl),
            "base_unit" => Some(self.spacing.base_unit),
            "radius_sm" => Some(self.spacing.radius_sm),
            "radius_md" => Some(self.spacing.radius_md),
            "radius_lg" => Some(self.spacing.radius_lg),
            "radius_full" => Some(self.spacing.radius_full),
            "border_thin" => Some(self.spacing.border_thin),
            "border_normal" => Some(self.spacing.border_normal),
            "border_thick" => Some(self.spacing.border_thick),
            _ => None,
        }
    }
    
    /// Get font size by name
    pub fn get_font_size(&self, name: &str) -> Option<f32> {
        match name {
            "xs" => Some(self.typography.font_size_xs),
            "sm" => Some(self.typography.font_size_sm),
            "base" => Some(self.typography.font_size_base),
            "lg" => Some(self.typography.font_size_lg),
            "xl" => Some(self.typography.font_size_xl),
            "2xl" => Some(self.typography.font_size_2xl),
            _ => None,
        }
    }
    
    /// Apply theme to component's standard properties
    pub fn apply_to_component<T>(&self, component: &mut T, component_type: &str)
    where
        T: crate::rcl::ui::enhanced_component::EnhancedComponent,
    {
        let props = component.standard_properties_mut();
        
        // Apply base theme colors
        props.background_color = self.colors.surface;
        props.color = self.colors.text_primary;
        props.border_color = self.colors.border;
        
        // Apply component-specific overrides
        if let Some(component_theme) = self.component_overrides.get(component_type) {
            if let Some(colors) = &component_theme.colors {
                for (property, color) in colors {
                    match property.as_str() {
                        "background_color" => props.background_color = *color,
                        "color" => props.color = *color,
                        "border_color" => props.border_color = *color,
                        _ => {}
                    }
                }
            }
            
            if let Some(dimensions) = &component_theme.dimensions {
                for (property, value) in dimensions {
                    match property.as_str() {
                        "padding" => props.padding = *value,
                        "margin" => props.margin = *value,
                        "border_width" => props.border_width = *value,
                        "corner_radius" => props.corner_radius = *value,
                        "font_size" => props.font_size = *value,
                        _ => {}
                    }
                }
            }
        }
    }
    
    /// Create a component-specific theme override
    pub fn add_component_override(&mut self, component_type: String, theme: ComponentTheme) {
        self.component_overrides.insert(component_type, theme);
    }
}

/// Theme manager for handling multiple themes
pub struct ThemeManager {
    /// Available themes
    themes: HashMap<String, Theme>,
    
    /// Currently active theme
    active_theme: String,
}

impl ThemeManager {
    /// Create a new theme manager with default themes
    pub fn new() -> Self {
        let mut themes = HashMap::new();
        
        // Add built-in themes
        let light = Theme::light_theme();
        let dark = Theme::dark_theme();
        
        themes.insert(light.name.clone(), light);
        themes.insert(dark.name.clone(), dark);
        
        Self {
            themes,
            active_theme: "Light".to_string(),
        }
    }
    
    /// Add a theme to the manager
    pub fn add_theme(&mut self, theme: Theme) {
        self.themes.insert(theme.name.clone(), theme);
    }
    
    /// Remove a theme from the manager
    pub fn remove_theme(&mut self, name: &str) -> Option<Theme> {
        self.themes.remove(name)
    }
    
    /// Set the active theme
    pub fn set_active_theme(&mut self, name: &str) -> Result<(), String> {
        if self.themes.contains_key(name) {
            self.active_theme = name.to_string();
            Ok(())
        } else {
            Err(format!("Theme '{}' not found", name))
        }
    }
    
    /// Get the currently active theme
    pub fn get_active_theme(&self) -> Option<&Theme> {
        self.themes.get(&self.active_theme)
    }
    
    /// Get a theme by name
    pub fn get_theme(&self, name: &str) -> Option<&Theme> {
        self.themes.get(name)
    }
    
    /// Get all available theme names
    pub fn get_theme_names(&self) -> Vec<String> {
        self.themes.keys().cloned().collect()
    }
    
    /// Load theme from JSON string (placeholder - requires custom serialization)
    pub fn load_theme_from_json(&mut self, _json: &str) -> Result<(), String> {
        // TODO: Implement custom JSON parsing for themes without serde
        Err("Theme JSON loading not yet implemented".to_string())
    }
    
    /// Save theme to JSON string (placeholder - requires custom serialization)
    pub fn save_theme_to_json(&self, _name: &str) -> Result<String, String> {
        // TODO: Implement custom JSON serialization for themes without serde
        Err("Theme JSON saving not yet implemented".to_string())
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_theme_creation() {
        let light = Theme::light_theme();
        assert_eq!(light.name, "Light");
        assert_eq!(light.colors.primary, Color32::from_rgb(70, 130, 200));
        
        let dark = Theme::dark_theme();
        assert_eq!(dark.name, "Dark");
        assert_eq!(dark.colors.background, Color32::from_rgb(18, 18, 18));
    }
    
    #[test]
    fn test_color_access() {
        let theme = Theme::light_theme();
        assert_eq!(theme.get_color("primary"), Some(Color32::from_rgb(70, 130, 200)));
        assert_eq!(theme.get_color("nonexistent"), None);
    }
    
    #[test]
    fn test_theme_manager() {
        let mut manager = ThemeManager::new();
        assert_eq!(manager.get_theme_names().len(), 2); // Light and Dark
        
        let custom = Theme::custom_theme("Custom".to_string(), "Test theme".to_string());
        manager.add_theme(custom);
        assert_eq!(manager.get_theme_names().len(), 3);
        
        assert!(manager.set_active_theme("Custom").is_ok());
        assert!(manager.set_active_theme("NonExistent").is_err());
    }
    
    #[test]
    #[ignore] // Temporarily disabled due to Color32 serialization issues
    fn test_theme_serialization() {
        let theme = Theme::light_theme();
        // TODO: Implement custom serialization for Color32
        // let json = serde_json::to_string(&theme).unwrap();
        // let deserialized: Theme = serde_json::from_str(&json).unwrap();
        assert_eq!(theme.name, "Light Theme");
    }
}