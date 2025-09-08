//! # Terminal Themes and Appearance
//!
//! This module provides theming capabilities for terminal appearance,
//! including color schemes, fonts, and visual styling options.

use egui::Color32;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Complete terminal theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalTheme {
    /// Theme name
    pub name: String,
    /// Theme description
    pub description: String,
    /// Color scheme
    pub colors: TerminalColorScheme,
    /// Font settings
    pub font: FontSettings,
    /// Cursor settings
    pub cursor: CursorSettings,
    /// Background settings
    pub background: BackgroundSettings,
    /// Scrollbar settings
    pub scrollbar: ScrollbarSettings,
    /// Selection settings
    pub selection: SelectionSettings,
}

/// Terminal color scheme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalColorScheme {
    /// Background color
    pub background: Color32,
    /// Foreground (text) color
    pub foreground: Color32,
    /// Cursor color
    pub cursor: Color32,
    /// Selection background
    pub selection_background: Color32,
    /// Selection foreground
    pub selection_foreground: Option<Color32>,
    
    /// ANSI colors (16 colors total)
    pub ansi_colors: AnsiColors,
    
    /// Special colors
    pub special: SpecialColors,
    
    /// UI element colors
    pub ui: UiColors,
}

/// Standard ANSI color palette
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnsiColors {
    /// Black (normal and bright)
    pub black: Color32,
    pub bright_black: Color32,
    
    /// Red (normal and bright)
    pub red: Color32,
    pub bright_red: Color32,
    
    /// Green (normal and bright)
    pub green: Color32,
    pub bright_green: Color32,
    
    /// Yellow (normal and bright)
    pub yellow: Color32,
    pub bright_yellow: Color32,
    
    /// Blue (normal and bright)
    pub blue: Color32,
    pub bright_blue: Color32,
    
    /// Magenta (normal and bright)
    pub magenta: Color32,
    pub bright_magenta: Color32,
    
    /// Cyan (normal and bright)
    pub cyan: Color32,
    pub bright_cyan: Color32,
    
    /// White (normal and bright)
    pub white: Color32,
    pub bright_white: Color32,
}

/// Special terminal colors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialColors {
    /// Error text color
    pub error: Color32,
    /// Warning text color
    pub warning: Color32,
    /// Success text color
    pub success: Color32,
    /// Info text color
    pub info: Color32,
    /// System message color
    pub system: Color32,
    /// User input color
    pub user_input: Color32,
    /// Command prompt color
    pub prompt: Color32,
    /// Line number color
    pub line_numbers: Color32,
}

/// UI element colors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiColors {
    /// Border color
    pub border: Color32,
    /// Tab background
    pub tab_background: Color32,
    /// Active tab background
    pub active_tab_background: Color32,
    /// Tab text
    pub tab_text: Color32,
    /// Active tab text
    pub active_tab_text: Color32,
    /// Status bar background
    pub status_bar_background: Color32,
    /// Status bar text
    pub status_bar_text: Color32,
}

/// Font configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontSettings {
    /// Font family name
    pub family: String,
    /// Font size in points
    pub size: f32,
    /// Font weight
    pub weight: FontWeight,
    /// Font style
    pub style: FontStyle,
    /// Line height multiplier
    pub line_height: f32,
    /// Character spacing adjustment
    pub character_spacing: f32,
    /// Enable font ligatures
    pub ligatures: bool,
}

/// Font weight options
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

/// Font style options
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

/// Cursor appearance settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorSettings {
    /// Cursor shape
    pub shape: CursorShape,
    /// Cursor color (None = use theme default)
    pub color: Option<Color32>,
    /// Cursor blink rate (0 = no blinking)
    pub blink_rate_ms: u32,
    /// Cursor width (for block cursors)
    pub width: f32,
    /// Whether cursor is visible when not focused
    pub visible_when_unfocused: bool,
}

/// Cursor shape options
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CursorShape {
    /// Block cursor (■)
    Block,
    /// Underline cursor (_)
    Underline,
    /// Vertical bar cursor (|)
    Bar,
    /// Hollow block cursor (□)
    HollowBlock,
}

/// Background settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundSettings {
    /// Background opacity (0.0 = transparent, 1.0 = opaque)
    pub opacity: f32,
    /// Background image path (optional)
    pub image_path: Option<String>,
    /// Background image opacity
    pub image_opacity: f32,
    /// Background blur radius
    pub blur_radius: f32,
}

/// Scrollbar appearance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollbarSettings {
    /// Scrollbar width
    pub width: f32,
    /// Scrollbar background color
    pub background: Color32,
    /// Scrollbar thumb color
    pub thumb: Color32,
    /// Scrollbar thumb hover color
    pub thumb_hover: Color32,
    /// Auto-hide scrollbar when not needed
    pub auto_hide: bool,
}

/// Selection appearance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionSettings {
    /// Selection background color
    pub background: Color32,
    /// Selection foreground color (None = use normal text color)
    pub foreground: Option<Color32>,
    /// Selection border width
    pub border_width: f32,
    /// Selection border color
    pub border_color: Color32,
}

/// Theme manager for handling multiple themes
#[derive(Debug, Clone)]
pub struct ThemeManager {
    /// Available themes
    pub themes: HashMap<String, TerminalTheme>,
    /// Current active theme
    pub active_theme: String,
    /// User custom themes directory
    pub custom_themes_dir: Option<std::path::PathBuf>,
}

impl ThemeManager {
    /// Create a new theme manager with default themes
    pub fn new() -> Self {
        let mut themes = HashMap::new();
        
        // Add built-in themes
        themes.insert("dark".to_string(), Self::create_dark_theme());
        themes.insert("light".to_string(), Self::create_light_theme());
        themes.insert("solarized_dark".to_string(), Self::create_solarized_dark_theme());
        themes.insert("solarized_light".to_string(), Self::create_solarized_light_theme());
        themes.insert("monokai".to_string(), Self::create_monokai_theme());
        themes.insert("dracula".to_string(), Self::create_dracula_theme());
        
        Self {
            themes,
            active_theme: "dark".to_string(),
            custom_themes_dir: None,
        }
    }
    
    /// Get the current active theme
    pub fn get_active_theme(&self) -> &TerminalTheme {
        self.themes.get(&self.active_theme)
            .unwrap_or_else(|| self.themes.get("dark").unwrap())
    }
    
    /// Set the active theme
    pub fn set_active_theme(&mut self, theme_name: &str) -> bool {
        if self.themes.contains_key(theme_name) {
            self.active_theme = theme_name.to_string();
            true
        } else {
            false
        }
    }
    
    /// Add a custom theme
    pub fn add_theme(&mut self, theme: TerminalTheme) {
        self.themes.insert(theme.name.clone(), theme);
    }
    
    /// Remove a theme
    pub fn remove_theme(&mut self, theme_name: &str) -> bool {
        if theme_name != "dark" && theme_name != "light" { // Protect built-in themes
            self.themes.remove(theme_name).is_some()
        } else {
            false
        }
    }
    
    /// Get list of available themes
    pub fn get_theme_names(&self) -> Vec<String> {
        self.themes.keys().cloned().collect()
    }
    
    /// Create the default dark theme
    fn create_dark_theme() -> TerminalTheme {
        TerminalTheme {
            name: "Dark".to_string(),
            description: "Default dark theme".to_string(),
            colors: TerminalColorScheme {
                background: Color32::from_rgb(30, 30, 30),
                foreground: Color32::from_rgb(204, 204, 204),
                cursor: Color32::from_rgb(255, 255, 255),
                selection_background: Color32::from_rgb(58, 58, 58),
                selection_foreground: None,
                ansi_colors: AnsiColors::default_dark(),
                special: SpecialColors::default_dark(),
                ui: UiColors::default_dark(),
            },
            font: FontSettings::default(),
            cursor: CursorSettings::default(),
            background: BackgroundSettings::default(),
            scrollbar: ScrollbarSettings::default_dark(),
            selection: SelectionSettings::default_dark(),
        }
    }
    
    /// Create the default light theme
    fn create_light_theme() -> TerminalTheme {
        TerminalTheme {
            name: "Light".to_string(),
            description: "Default light theme".to_string(),
            colors: TerminalColorScheme {
                background: Color32::from_rgb(255, 255, 255),
                foreground: Color32::from_rgb(51, 51, 51),
                cursor: Color32::from_rgb(0, 0, 0),
                selection_background: Color32::from_rgb(197, 197, 197),
                selection_foreground: None,
                ansi_colors: AnsiColors::default_light(),
                special: SpecialColors::default_light(),
                ui: UiColors::default_light(),
            },
            font: FontSettings::default(),
            cursor: CursorSettings::default(),
            background: BackgroundSettings::default(),
            scrollbar: ScrollbarSettings::default_light(),
            selection: SelectionSettings::default_light(),
        }
    }
    
    /// Create Solarized Dark theme
    fn create_solarized_dark_theme() -> TerminalTheme {
        TerminalTheme {
            name: "Solarized Dark".to_string(),
            description: "Solarized dark color scheme".to_string(),
            colors: TerminalColorScheme {
                background: Color32::from_rgb(0, 43, 54),     // base03
                foreground: Color32::from_rgb(131, 148, 150), // base0
                cursor: Color32::from_rgb(147, 161, 161),     // base1
                selection_background: Color32::from_rgb(7, 54, 66), // base02
                selection_foreground: None,
                ansi_colors: AnsiColors::solarized(),
                special: SpecialColors::solarized_dark(),
                ui: UiColors::solarized_dark(),
            },
            font: FontSettings::default(),
            cursor: CursorSettings::default(),
            background: BackgroundSettings::default(),
            scrollbar: ScrollbarSettings::solarized_dark(),
            selection: SelectionSettings::solarized_dark(),
        }
    }
    
    /// Create Solarized Light theme
    fn create_solarized_light_theme() -> TerminalTheme {
        TerminalTheme {
            name: "Solarized Light".to_string(),
            description: "Solarized light color scheme".to_string(),
            colors: TerminalColorScheme {
                background: Color32::from_rgb(253, 246, 227), // base3
                foreground: Color32::from_rgb(101, 123, 131), // base00
                cursor: Color32::from_rgb(88, 110, 117),      // base01
                selection_background: Color32::from_rgb(238, 232, 213), // base2
                selection_foreground: None,
                ansi_colors: AnsiColors::solarized(),
                special: SpecialColors::solarized_light(),
                ui: UiColors::solarized_light(),
            },
            font: FontSettings::default(),
            cursor: CursorSettings::default(),
            background: BackgroundSettings::default(),
            scrollbar: ScrollbarSettings::solarized_light(),
            selection: SelectionSettings::solarized_light(),
        }
    }
    
    /// Create Monokai theme
    fn create_monokai_theme() -> TerminalTheme {
        TerminalTheme {
            name: "Monokai".to_string(),
            description: "Monokai color scheme".to_string(),
            colors: TerminalColorScheme {
                background: Color32::from_rgb(39, 40, 34),
                foreground: Color32::from_rgb(248, 248, 242),
                cursor: Color32::from_rgb(248, 248, 240),
                selection_background: Color32::from_rgb(73, 72, 62),
                selection_foreground: None,
                ansi_colors: AnsiColors::monokai(),
                special: SpecialColors::monokai(),
                ui: UiColors::monokai(),
            },
            font: FontSettings::default(),
            cursor: CursorSettings::default(),
            background: BackgroundSettings::default(),
            scrollbar: ScrollbarSettings::monokai(),
            selection: SelectionSettings::monokai(),
        }
    }
    
    /// Create Dracula theme
    fn create_dracula_theme() -> TerminalTheme {
        TerminalTheme {
            name: "Dracula".to_string(),
            description: "Dracula color scheme".to_string(),
            colors: TerminalColorScheme {
                background: Color32::from_rgb(40, 42, 54),
                foreground: Color32::from_rgb(248, 248, 242),
                cursor: Color32::from_rgb(248, 248, 242),
                selection_background: Color32::from_rgb(68, 71, 90),
                selection_foreground: None,
                ansi_colors: AnsiColors::dracula(),
                special: SpecialColors::dracula(),
                ui: UiColors::dracula(),
            },
            font: FontSettings::default(),
            cursor: CursorSettings::default(),
            background: BackgroundSettings::default(),
            scrollbar: ScrollbarSettings::dracula(),
            selection: SelectionSettings::dracula(),
        }
    }
}

// Default implementations for various color schemes
impl AnsiColors {
    fn default_dark() -> Self {
        Self {
            black: Color32::from_rgb(0, 0, 0),
            bright_black: Color32::from_rgb(128, 128, 128),
            red: Color32::from_rgb(205, 49, 49),
            bright_red: Color32::from_rgb(241, 76, 76),
            green: Color32::from_rgb(13, 188, 121),
            bright_green: Color32::from_rgb(35, 209, 139),
            yellow: Color32::from_rgb(229, 229, 16),
            bright_yellow: Color32::from_rgb(245, 245, 67),
            blue: Color32::from_rgb(36, 114, 200),
            bright_blue: Color32::from_rgb(59, 142, 234),
            magenta: Color32::from_rgb(188, 63, 188),
            bright_magenta: Color32::from_rgb(214, 112, 214),
            cyan: Color32::from_rgb(17, 168, 205),
            bright_cyan: Color32::from_rgb(41, 184, 219),
            white: Color32::from_rgb(229, 229, 229),
            bright_white: Color32::from_rgb(255, 255, 255),
        }
    }
    
    fn default_light() -> Self {
        Self {
            black: Color32::from_rgb(0, 0, 0),
            bright_black: Color32::from_rgb(128, 128, 128),
            red: Color32::from_rgb(170, 0, 0),
            bright_red: Color32::from_rgb(255, 0, 0),
            green: Color32::from_rgb(0, 170, 0),
            bright_green: Color32::from_rgb(0, 255, 0),
            yellow: Color32::from_rgb(170, 170, 0),
            bright_yellow: Color32::from_rgb(255, 255, 0),
            blue: Color32::from_rgb(0, 0, 170),
            bright_blue: Color32::from_rgb(0, 0, 255),
            magenta: Color32::from_rgb(170, 0, 170),
            bright_magenta: Color32::from_rgb(255, 0, 255),
            cyan: Color32::from_rgb(0, 170, 170),
            bright_cyan: Color32::from_rgb(0, 255, 255),
            white: Color32::from_rgb(170, 170, 170),
            bright_white: Color32::from_rgb(255, 255, 255),
        }
    }
    
    fn solarized() -> Self {
        Self {
            black: Color32::from_rgb(7, 54, 66),      // base02
            bright_black: Color32::from_rgb(0, 43, 54),     // base03
            red: Color32::from_rgb(220, 50, 47),      // red
            bright_red: Color32::from_rgb(203, 75, 22),     // orange
            green: Color32::from_rgb(133, 153, 0),    // green
            bright_green: Color32::from_rgb(88, 110, 117),  // base01
            yellow: Color32::from_rgb(181, 137, 0),   // yellow
            bright_yellow: Color32::from_rgb(101, 123, 131), // base00
            blue: Color32::from_rgb(38, 139, 210),    // blue
            bright_blue: Color32::from_rgb(131, 148, 150),  // base0
            magenta: Color32::from_rgb(211, 54, 130), // magenta
            bright_magenta: Color32::from_rgb(108, 113, 196), // violet
            cyan: Color32::from_rgb(42, 161, 152),    // cyan
            bright_cyan: Color32::from_rgb(147, 161, 161),  // base1
            white: Color32::from_rgb(238, 232, 213),  // base2
            bright_white: Color32::from_rgb(253, 246, 227), // base3
        }
    }
    
    fn monokai() -> Self {
        Self {
            black: Color32::from_rgb(39, 40, 34),
            bright_black: Color32::from_rgb(117, 113, 94),
            red: Color32::from_rgb(249, 38, 114),
            bright_red: Color32::from_rgb(255, 89, 149),
            green: Color32::from_rgb(166, 226, 46),
            bright_green: Color32::from_rgb(182, 227, 84),
            yellow: Color32::from_rgb(244, 191, 117),
            bright_yellow: Color32::from_rgb(255, 214, 154),
            blue: Color32::from_rgb(102, 217, 239),
            bright_blue: Color32::from_rgb(129, 222, 240),
            magenta: Color32::from_rgb(174, 129, 255),
            bright_magenta: Color32::from_rgb(192, 152, 255),
            cyan: Color32::from_rgb(161, 239, 228),
            bright_cyan: Color32::from_rgb(177, 242, 232),
            white: Color32::from_rgb(248, 248, 242),
            bright_white: Color32::from_rgb(255, 255, 255),
        }
    }
    
    fn dracula() -> Self {
        Self {
            black: Color32::from_rgb(40, 42, 54),
            bright_black: Color32::from_rgb(98, 114, 164),
            red: Color32::from_rgb(255, 85, 85),
            bright_red: Color32::from_rgb(255, 110, 103),
            green: Color32::from_rgb(80, 250, 123),
            bright_green: Color32::from_rgb(90, 247, 142),
            yellow: Color32::from_rgb(241, 250, 140),
            bright_yellow: Color32::from_rgb(244, 249, 157),
            blue: Color32::from_rgb(139, 233, 253),
            bright_blue: Color32::from_rgb(154, 237, 254),
            magenta: Color32::from_rgb(255, 121, 198),
            bright_magenta: Color32::from_rgb(255, 146, 208),
            cyan: Color32::from_rgb(139, 233, 253),
            bright_cyan: Color32::from_rgb(154, 237, 254),
            white: Color32::from_rgb(248, 248, 242),
            bright_white: Color32::from_rgb(255, 255, 255),
        }
    }
}

impl SpecialColors {
    fn default_dark() -> Self {
        Self {
            error: Color32::from_rgb(241, 76, 76),
            warning: Color32::from_rgb(245, 245, 67),
            success: Color32::from_rgb(35, 209, 139),
            info: Color32::from_rgb(59, 142, 234),
            system: Color32::from_rgb(128, 128, 128),
            user_input: Color32::from_rgb(255, 255, 255),
            prompt: Color32::from_rgb(35, 209, 139),
            line_numbers: Color32::from_rgb(128, 128, 128),
        }
    }
    
    fn default_light() -> Self {
        Self {
            error: Color32::from_rgb(255, 0, 0),
            warning: Color32::from_rgb(255, 140, 0),
            success: Color32::from_rgb(0, 200, 0),
            info: Color32::from_rgb(0, 0, 255),
            system: Color32::from_rgb(128, 128, 128),
            user_input: Color32::from_rgb(0, 0, 0),
            prompt: Color32::from_rgb(0, 150, 0),
            line_numbers: Color32::from_rgb(100, 100, 100),
        }
    }
    
    fn solarized_dark() -> Self {
        Self {
            error: Color32::from_rgb(220, 50, 47),
            warning: Color32::from_rgb(181, 137, 0),
            success: Color32::from_rgb(133, 153, 0),
            info: Color32::from_rgb(38, 139, 210),
            system: Color32::from_rgb(88, 110, 117),
            user_input: Color32::from_rgb(147, 161, 161),
            prompt: Color32::from_rgb(133, 153, 0),
            line_numbers: Color32::from_rgb(88, 110, 117),
        }
    }
    
    fn solarized_light() -> Self {
        Self {
            error: Color32::from_rgb(220, 50, 47),
            warning: Color32::from_rgb(181, 137, 0),
            success: Color32::from_rgb(133, 153, 0),
            info: Color32::from_rgb(38, 139, 210),
            system: Color32::from_rgb(147, 161, 161),
            user_input: Color32::from_rgb(88, 110, 117),
            prompt: Color32::from_rgb(133, 153, 0),
            line_numbers: Color32::from_rgb(147, 161, 161),
        }
    }
    
    fn monokai() -> Self {
        Self {
            error: Color32::from_rgb(249, 38, 114),
            warning: Color32::from_rgb(244, 191, 117),
            success: Color32::from_rgb(166, 226, 46),
            info: Color32::from_rgb(102, 217, 239),
            system: Color32::from_rgb(117, 113, 94),
            user_input: Color32::from_rgb(248, 248, 242),
            prompt: Color32::from_rgb(166, 226, 46),
            line_numbers: Color32::from_rgb(117, 113, 94),
        }
    }
    
    fn dracula() -> Self {
        Self {
            error: Color32::from_rgb(255, 85, 85),
            warning: Color32::from_rgb(241, 250, 140),
            success: Color32::from_rgb(80, 250, 123),
            info: Color32::from_rgb(139, 233, 253),
            system: Color32::from_rgb(98, 114, 164),
            user_input: Color32::from_rgb(248, 248, 242),
            prompt: Color32::from_rgb(80, 250, 123),
            line_numbers: Color32::from_rgb(98, 114, 164),
        }
    }
}

impl Default for FontSettings {
    fn default() -> Self {
        Self {
            family: "JetBrains Mono".to_string(),
            size: 12.0,
            weight: FontWeight::Normal,
            style: FontStyle::Normal,
            line_height: 1.2,
            character_spacing: 0.0,
            ligatures: true,
        }
    }
}

impl Default for CursorSettings {
    fn default() -> Self {
        Self {
            shape: CursorShape::Block,
            color: None,
            blink_rate_ms: 530,
            width: 2.0,
            visible_when_unfocused: true,
        }
    }
}

impl Default for BackgroundSettings {
    fn default() -> Self {
        Self {
            opacity: 1.0,
            image_path: None,
            image_opacity: 0.5,
            blur_radius: 0.0,
        }
    }
}

// Helper implementations for different theme scrollbars, UI colors, etc.
impl ScrollbarSettings {
    fn default_dark() -> Self {
        Self {
            width: 14.0,
            background: Color32::from_rgb(50, 50, 50),
            thumb: Color32::from_rgb(100, 100, 100),
            thumb_hover: Color32::from_rgb(130, 130, 130),
            auto_hide: true,
        }
    }
    
    fn default_light() -> Self {
        Self {
            width: 14.0,
            background: Color32::from_rgb(240, 240, 240),
            thumb: Color32::from_rgb(150, 150, 150),
            thumb_hover: Color32::from_rgb(120, 120, 120),
            auto_hide: true,
        }
    }
    
    fn solarized_dark() -> Self {
        Self {
            width: 14.0,
            background: Color32::from_rgb(7, 54, 66),
            thumb: Color32::from_rgb(88, 110, 117),
            thumb_hover: Color32::from_rgb(101, 123, 131),
            auto_hide: true,
        }
    }
    
    fn solarized_light() -> Self {
        Self {
            width: 14.0,
            background: Color32::from_rgb(238, 232, 213),
            thumb: Color32::from_rgb(147, 161, 161),
            thumb_hover: Color32::from_rgb(131, 148, 150),
            auto_hide: true,
        }
    }
    
    fn monokai() -> Self {
        Self {
            width: 14.0,
            background: Color32::from_rgb(73, 72, 62),
            thumb: Color32::from_rgb(117, 113, 94),
            thumb_hover: Color32::from_rgb(136, 131, 107),
            auto_hide: true,
        }
    }
    
    fn dracula() -> Self {
        Self {
            width: 14.0,
            background: Color32::from_rgb(68, 71, 90),
            thumb: Color32::from_rgb(98, 114, 164),
            thumb_hover: Color32::from_rgb(139, 233, 253),
            auto_hide: true,
        }
    }
}

impl UiColors {
    fn default_dark() -> Self {
        Self {
            border: Color32::from_rgb(70, 70, 70),
            tab_background: Color32::from_rgb(45, 45, 45),
            active_tab_background: Color32::from_rgb(30, 30, 30),
            tab_text: Color32::from_rgb(180, 180, 180),
            active_tab_text: Color32::from_rgb(255, 255, 255),
            status_bar_background: Color32::from_rgb(25, 25, 25),
            status_bar_text: Color32::from_rgb(200, 200, 200),
        }
    }
    
    fn default_light() -> Self {
        Self {
            border: Color32::from_rgb(190, 190, 190),
            tab_background: Color32::from_rgb(230, 230, 230),
            active_tab_background: Color32::from_rgb(255, 255, 255),
            tab_text: Color32::from_rgb(80, 80, 80),
            active_tab_text: Color32::from_rgb(0, 0, 0),
            status_bar_background: Color32::from_rgb(240, 240, 240),
            status_bar_text: Color32::from_rgb(60, 60, 60),
        }
    }
    
    fn solarized_dark() -> Self {
        Self {
            border: Color32::from_rgb(88, 110, 117),
            tab_background: Color32::from_rgb(7, 54, 66),
            active_tab_background: Color32::from_rgb(0, 43, 54),
            tab_text: Color32::from_rgb(131, 148, 150),
            active_tab_text: Color32::from_rgb(147, 161, 161),
            status_bar_background: Color32::from_rgb(7, 54, 66),
            status_bar_text: Color32::from_rgb(131, 148, 150),
        }
    }
    
    fn solarized_light() -> Self {
        Self {
            border: Color32::from_rgb(147, 161, 161),
            tab_background: Color32::from_rgb(238, 232, 213),
            active_tab_background: Color32::from_rgb(253, 246, 227),
            tab_text: Color32::from_rgb(101, 123, 131),
            active_tab_text: Color32::from_rgb(88, 110, 117),
            status_bar_background: Color32::from_rgb(238, 232, 213),
            status_bar_text: Color32::from_rgb(101, 123, 131),
        }
    }
    
    fn monokai() -> Self {
        Self {
            border: Color32::from_rgb(117, 113, 94),
            tab_background: Color32::from_rgb(73, 72, 62),
            active_tab_background: Color32::from_rgb(39, 40, 34),
            tab_text: Color32::from_rgb(204, 204, 204),
            active_tab_text: Color32::from_rgb(248, 248, 242),
            status_bar_background: Color32::from_rgb(73, 72, 62),
            status_bar_text: Color32::from_rgb(248, 248, 242),
        }
    }
    
    fn dracula() -> Self {
        Self {
            border: Color32::from_rgb(98, 114, 164),
            tab_background: Color32::from_rgb(68, 71, 90),
            active_tab_background: Color32::from_rgb(40, 42, 54),
            tab_text: Color32::from_rgb(248, 248, 242),
            active_tab_text: Color32::from_rgb(255, 255, 255),
            status_bar_background: Color32::from_rgb(68, 71, 90),
            status_bar_text: Color32::from_rgb(248, 248, 242),
        }
    }
}

impl SelectionSettings {
    fn default_dark() -> Self {
        Self {
            background: Color32::from_rgb(58, 58, 58),
            foreground: None,
            border_width: 0.0,
            border_color: Color32::from_rgb(100, 100, 100),
        }
    }
    
    fn default_light() -> Self {
        Self {
            background: Color32::from_rgb(197, 197, 197),
            foreground: None,
            border_width: 0.0,
            border_color: Color32::from_rgb(150, 150, 150),
        }
    }
    
    fn solarized_dark() -> Self {
        Self {
            background: Color32::from_rgb(7, 54, 66),
            foreground: None,
            border_width: 0.0,
            border_color: Color32::from_rgb(88, 110, 117),
        }
    }
    
    fn solarized_light() -> Self {
        Self {
            background: Color32::from_rgb(238, 232, 213),
            foreground: None,
            border_width: 0.0,
            border_color: Color32::from_rgb(147, 161, 161),
        }
    }
    
    fn monokai() -> Self {
        Self {
            background: Color32::from_rgb(73, 72, 62),
            foreground: None,
            border_width: 0.0,
            border_color: Color32::from_rgb(117, 113, 94),
        }
    }
    
    fn dracula() -> Self {
        Self {
            background: Color32::from_rgb(68, 71, 90),
            foreground: None,
            border_width: 0.0,
            border_color: Color32::from_rgb(98, 114, 164),
        }
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
    fn test_theme_manager_creation() {
        let theme_manager = ThemeManager::new();
        assert!(!theme_manager.themes.is_empty());
        assert!(theme_manager.themes.contains_key("dark"));
        assert!(theme_manager.themes.contains_key("light"));
    }

    #[test]
    fn test_active_theme() {
        let mut theme_manager = ThemeManager::new();
        assert_eq!(theme_manager.active_theme, "dark");
        
        assert!(theme_manager.set_active_theme("light"));
        assert_eq!(theme_manager.active_theme, "light");
        
        assert!(!theme_manager.set_active_theme("nonexistent"));
        assert_eq!(theme_manager.active_theme, "light");
    }

    #[test]
    fn test_custom_theme() {
        let mut theme_manager = ThemeManager::new();
        let initial_count = theme_manager.themes.len();
        
        let custom_theme = TerminalTheme {
            name: "Custom".to_string(),
            description: "Test theme".to_string(),
            colors: TerminalColorScheme {
                background: Color32::RED,
                foreground: Color32::WHITE,
                cursor: Color32::BLUE,
                selection_background: Color32::GRAY,
                selection_foreground: None,
                ansi_colors: AnsiColors::default_dark(),
                special: SpecialColors::default_dark(),
                ui: UiColors::default_dark(),
            },
            font: FontSettings::default(),
            cursor: CursorSettings::default(),
            background: BackgroundSettings::default(),
            scrollbar: ScrollbarSettings::default_dark(),
            selection: SelectionSettings::default_dark(),
        };
        
        theme_manager.add_theme(custom_theme);
        assert_eq!(theme_manager.themes.len(), initial_count + 1);
        assert!(theme_manager.themes.contains_key("Custom"));
    }

    #[test]
    fn test_theme_colors() {
        let theme_manager = ThemeManager::new();
        let dark_theme = theme_manager.themes.get("dark").unwrap();
        
        // Test that colors are properly set
        assert_ne!(dark_theme.colors.background, Color32::TRANSPARENT);
        assert_ne!(dark_theme.colors.foreground, Color32::TRANSPARENT);
        
        // Test ANSI colors are different
        assert_ne!(dark_theme.colors.ansi_colors.red, dark_theme.colors.ansi_colors.blue);
    }
}