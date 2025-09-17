//! # Terminal Themes and Appearance
//!
//! This module provides theming capabilities for terminal appearance,
//! including color schemes, fonts, and visual styling options.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::editor::terminal::core::SerializableColor;

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
    pub background: SerializableColor,
    /// Foreground (text) color
    pub foreground: SerializableColor,
    /// Cursor color
    pub cursor: SerializableColor,
    /// Selection background
    pub selection_background: SerializableColor,
    /// Selection foreground
    pub selection_foreground: Option<SerializableColor>,
    
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
    pub black: SerializableColor,
    pub bright_black: SerializableColor,

    /// Red (normal and bright)
    pub red: SerializableColor,
    pub bright_red: SerializableColor,

    /// Green (normal and bright)
    pub green: SerializableColor,
    pub bright_green: SerializableColor,

    /// Yellow (normal and bright)
    pub yellow: SerializableColor,
    pub bright_yellow: SerializableColor,

    /// Blue (normal and bright)
    pub blue: SerializableColor,
    pub bright_blue: SerializableColor,

    /// Magenta (normal and bright)
    pub magenta: SerializableColor,
    pub bright_magenta: SerializableColor,

    /// Cyan (normal and bright)
    pub cyan: SerializableColor,
    pub bright_cyan: SerializableColor,

    /// White (normal and bright)
    pub white: SerializableColor,
    pub bright_white: SerializableColor,
}

/// Special terminal colors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialColors {
    /// Error text color
    pub error: SerializableColor,
    /// Warning text color
    pub warning: SerializableColor,
    /// Success text color
    pub success: SerializableColor,
    /// Info text color
    pub info: SerializableColor,
    /// System message color
    pub system: SerializableColor,
    /// User input color
    pub user_input: SerializableColor,
    /// Command prompt color
    pub prompt: SerializableColor,
    /// Line number color
    pub line_numbers: SerializableColor,
}

/// UI element colors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiColors {
    /// Border color
    pub border: SerializableColor,
    /// Tab background
    pub tab_background: SerializableColor,
    /// Active tab background
    pub active_tab_background: SerializableColor,
    /// Tab text
    pub tab_text: SerializableColor,
    /// Active tab text
    pub active_tab_text: SerializableColor,
    /// Status bar background
    pub status_bar_background: SerializableColor,
    /// Status bar text
    pub status_bar_text: SerializableColor,
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
    pub color: Option<SerializableColor>,
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
    pub background: SerializableColor,
    /// Scrollbar thumb color
    pub thumb: SerializableColor,
    /// Scrollbar thumb hover color
    pub thumb_hover: SerializableColor,
    /// Auto-hide scrollbar when not needed
    pub auto_hide: bool,
}

/// Selection appearance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionSettings {
    /// Selection background color
    pub background: SerializableColor,
    /// Selection foreground color (None = use normal text color)
    pub foreground: Option<SerializableColor>,
    /// Selection border width
    pub border_width: f32,
    /// Selection border color
    pub border_color: SerializableColor,
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
                background: SerializableColor { r: 30, g: 30, b: 30, a: 255 },
                foreground: SerializableColor { r: 204, g: 204, b: 204, a: 255 },
                cursor: SerializableColor { r: 255, g: 255, b: 255, a: 255 },
                selection_background: SerializableColor { r: 58, g: 58, b: 58, a: 255 },
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
                background: SerializableColor { r: 255, g: 255, b: 255, a: 255 },
                foreground: SerializableColor { r: 51, g: 51, b: 51, a: 255 },
                cursor: SerializableColor { r: 0, g: 0, b: 0, a: 255 },
                selection_background: SerializableColor { r: 197, g: 197, b: 197, a: 255 },
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
                background: SerializableColor { r: 0, g: 43, b: 54, a: 255 },     // base03
                foreground: SerializableColor { r: 131, g: 148, b: 150, a: 255 }, // base0
                cursor: SerializableColor { r: 147, g: 161, b: 161, a: 255 },     // base1
                selection_background: SerializableColor { r: 7, g: 54, b: 66, a: 255 }, // base02
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
                background: SerializableColor { r: 253, g: 246, b: 227, a: 255 }, // base3
                foreground: SerializableColor { r: 101, g: 123, b: 131, a: 255 }, // base00
                cursor: SerializableColor { r: 88, g: 110, b: 117, a: 255 },      // base01
                selection_background: SerializableColor { r: 238, g: 232, b: 213, a: 255 }, // base2
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
                background: SerializableColor { r: 39, g: 40, b: 34, a: 255 },
                foreground: SerializableColor { r: 248, g: 248, b: 242, a: 255 },
                cursor: SerializableColor { r: 248, g: 248, b: 240, a: 255 },
                selection_background: SerializableColor { r: 73, g: 72, b: 62, a: 255 },
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
                background: SerializableColor { r: 40, g: 42, b: 54, a: 255 },
                foreground: SerializableColor { r: 248, g: 248, b: 242, a: 255 },
                cursor: SerializableColor { r: 248, g: 248, b: 242, a: 255 },
                selection_background: SerializableColor { r: 68, g: 71, b: 90, a: 255 },
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
            black: SerializableColor { r: 0, g: 0, b: 0, a: 255 },
            bright_black: SerializableColor { r: 128, g: 128, b: 128, a: 255 },
            red: SerializableColor { r: 205, g: 49, b: 49, a: 255 },
            bright_red: SerializableColor { r: 241, g: 76, b: 76, a: 255 },
            green: SerializableColor { r: 13, g: 188, b: 121, a: 255 },
            bright_green: SerializableColor { r: 35, g: 209, b: 139, a: 255 },
            yellow: SerializableColor { r: 229, g: 229, b: 16, a: 255 },
            bright_yellow: SerializableColor { r: 245, g: 245, b: 67, a: 255 },
            blue: SerializableColor { r: 36, g: 114, b: 200, a: 255 },
            bright_blue: SerializableColor { r: 59, g: 142, b: 234, a: 255 },
            magenta: SerializableColor { r: 188, g: 63, b: 188, a: 255 },
            bright_magenta: SerializableColor { r: 214, g: 112, b: 214, a: 255 },
            cyan: SerializableColor { r: 17, g: 168, b: 205, a: 255 },
            bright_cyan: SerializableColor { r: 41, g: 184, b: 219, a: 255 },
            white: SerializableColor { r: 229, g: 229, b: 229, a: 255 },
            bright_white: SerializableColor { r: 255, g: 255, b: 255, a: 255 },
        }
    }
    
    fn default_light() -> Self {
        Self {
            black: SerializableColor { r: 0, g: 0, b: 0, a: 255 },
            bright_black: SerializableColor { r: 128, g: 128, b: 128, a: 255 },
            red: SerializableColor { r: 170, g: 0, b: 0, a: 255 },
            bright_red: SerializableColor { r: 255, g: 0, b: 0, a: 255 },
            green: SerializableColor { r: 0, g: 170, b: 0, a: 255 },
            bright_green: SerializableColor { r: 0, g: 255, b: 0, a: 255 },
            yellow: SerializableColor { r: 170, g: 170, b: 0, a: 255 },
            bright_yellow: SerializableColor { r: 255, g: 255, b: 0, a: 255 },
            blue: SerializableColor { r: 0, g: 0, b: 170, a: 255 },
            bright_blue: SerializableColor { r: 0, g: 0, b: 255, a: 255 },
            magenta: SerializableColor { r: 170, g: 0, b: 170, a: 255 },
            bright_magenta: SerializableColor { r: 255, g: 0, b: 255, a: 255 },
            cyan: SerializableColor { r: 0, g: 170, b: 170, a: 255 },
            bright_cyan: SerializableColor { r: 0, g: 255, b: 255, a: 255 },
            white: SerializableColor { r: 170, g: 170, b: 170, a: 255 },
            bright_white: SerializableColor { r: 255, g: 255, b: 255, a: 255 },
        }
    }
    
    fn solarized() -> Self {
        Self {
            black: SerializableColor { r: 7, g: 54, b: 66, a: 255 },      // base02
            bright_black: SerializableColor { r: 0, g: 43, b: 54, a: 255 },     // base03
            red: SerializableColor { r: 220, g: 50, b: 47, a: 255 },      // red
            bright_red: SerializableColor { r: 203, g: 75, b: 22, a: 255 },     // orange
            green: SerializableColor { r: 133, g: 153, b: 0, a: 255 },    // green
            bright_green: SerializableColor { r: 88, g: 110, b: 117, a: 255 },  // base01
            yellow: SerializableColor { r: 181, g: 137, b: 0, a: 255 },   // yellow
            bright_yellow: SerializableColor { r: 101, g: 123, b: 131, a: 255 }, // base00
            blue: SerializableColor { r: 38, g: 139, b: 210, a: 255 },    // blue
            bright_blue: SerializableColor { r: 131, g: 148, b: 150, a: 255 },  // base0
            magenta: SerializableColor { r: 211, g: 54, b: 130, a: 255 }, // magenta
            bright_magenta: SerializableColor { r: 108, g: 113, b: 196, a: 255 }, // violet
            cyan: SerializableColor { r: 42, g: 161, b: 152, a: 255 },    // cyan
            bright_cyan: SerializableColor { r: 147, g: 161, b: 161, a: 255 },  // base1
            white: SerializableColor { r: 238, g: 232, b: 213, a: 255 },  // base2
            bright_white: SerializableColor { r: 253, g: 246, b: 227, a: 255 }, // base3
        }
    }
    
    fn monokai() -> Self {
        Self {
            black: SerializableColor { r: 39, g: 40, b: 34, a: 255 },
            bright_black: SerializableColor { r: 117, g: 113, b: 94, a: 255 },
            red: SerializableColor { r: 249, g: 38, b: 114, a: 255 },
            bright_red: SerializableColor { r: 255, g: 89, b: 149, a: 255 },
            green: SerializableColor { r: 166, g: 226, b: 46, a: 255 },
            bright_green: SerializableColor { r: 182, g: 227, b: 84, a: 255 },
            yellow: SerializableColor { r: 244, g: 191, b: 117, a: 255 },
            bright_yellow: SerializableColor { r: 255, g: 214, b: 154, a: 255 },
            blue: SerializableColor { r: 102, g: 217, b: 239, a: 255 },
            bright_blue: SerializableColor { r: 129, g: 222, b: 240, a: 255 },
            magenta: SerializableColor { r: 174, g: 129, b: 255, a: 255 },
            bright_magenta: SerializableColor { r: 192, g: 152, b: 255, a: 255 },
            cyan: SerializableColor { r: 161, g: 239, b: 228, a: 255 },
            bright_cyan: SerializableColor { r: 177, g: 242, b: 232, a: 255 },
            white: SerializableColor { r: 248, g: 248, b: 242, a: 255 },
            bright_white: SerializableColor { r: 255, g: 255, b: 255, a: 255 },
        }
    }
    
    fn dracula() -> Self {
        Self {
            black: SerializableColor { r: 40, g: 42, b: 54, a: 255 },
            bright_black: SerializableColor { r: 98, g: 114, b: 164, a: 255 },
            red: SerializableColor { r: 255, g: 85, b: 85, a: 255 },
            bright_red: SerializableColor { r: 255, g: 110, b: 103, a: 255 },
            green: SerializableColor { r: 80, g: 250, b: 123, a: 255 },
            bright_green: SerializableColor { r: 90, g: 247, b: 142, a: 255 },
            yellow: SerializableColor { r: 241, g: 250, b: 140, a: 255 },
            bright_yellow: SerializableColor { r: 244, g: 249, b: 157, a: 255 },
            blue: SerializableColor { r: 139, g: 233, b: 253, a: 255 },
            bright_blue: SerializableColor { r: 154, g: 237, b: 254, a: 255 },
            magenta: SerializableColor { r: 255, g: 121, b: 198, a: 255 },
            bright_magenta: SerializableColor { r: 255, g: 146, b: 208, a: 255 },
            cyan: SerializableColor { r: 139, g: 233, b: 253, a: 255 },
            bright_cyan: SerializableColor { r: 154, g: 237, b: 254, a: 255 },
            white: SerializableColor { r: 248, g: 248, b: 242, a: 255 },
            bright_white: SerializableColor { r: 255, g: 255, b: 255, a: 255 },
        }
    }
}

impl SpecialColors {
    fn default_dark() -> Self {
        Self {
            error: SerializableColor { r: 241, g: 76, b: 76, a: 255 },
            warning: SerializableColor { r: 245, g: 245, b: 67, a: 255 },
            success: SerializableColor { r: 35, g: 209, b: 139, a: 255 },
            info: SerializableColor { r: 59, g: 142, b: 234, a: 255 },
            system: SerializableColor { r: 128, g: 128, b: 128, a: 255 },
            user_input: SerializableColor { r: 255, g: 255, b: 255, a: 255 },
            prompt: SerializableColor { r: 35, g: 209, b: 139, a: 255 },
            line_numbers: SerializableColor { r: 128, g: 128, b: 128, a: 255 },
        }
    }
    
    fn default_light() -> Self {
        Self {
            error: SerializableColor { r: 255, g: 0, b: 0, a: 255 },
            warning: SerializableColor { r: 255, g: 140, b: 0, a: 255 },
            success: SerializableColor { r: 0, g: 200, b: 0, a: 255 },
            info: SerializableColor { r: 0, g: 0, b: 255, a: 255 },
            system: SerializableColor { r: 128, g: 128, b: 128, a: 255 },
            user_input: SerializableColor { r: 0, g: 0, b: 0, a: 255 },
            prompt: SerializableColor { r: 0, g: 150, b: 0, a: 255 },
            line_numbers: SerializableColor { r: 100, g: 100, b: 100, a: 255 },
        }
    }
    
    fn solarized_dark() -> Self {
        Self {
            error: SerializableColor { r: 220, g: 50, b: 47, a: 255 },
            warning: SerializableColor { r: 181, g: 137, b: 0, a: 255 },
            success: SerializableColor { r: 133, g: 153, b: 0, a: 255 },
            info: SerializableColor { r: 38, g: 139, b: 210, a: 255 },
            system: SerializableColor { r: 88, g: 110, b: 117, a: 255 },
            user_input: SerializableColor { r: 147, g: 161, b: 161, a: 255 },
            prompt: SerializableColor { r: 133, g: 153, b: 0, a: 255 },
            line_numbers: SerializableColor { r: 88, g: 110, b: 117, a: 255 },
        }
    }
    
    fn solarized_light() -> Self {
        Self {
            error: SerializableColor { r: 220, g: 50, b: 47, a: 255 },
            warning: SerializableColor { r: 181, g: 137, b: 0, a: 255 },
            success: SerializableColor { r: 133, g: 153, b: 0, a: 255 },
            info: SerializableColor { r: 38, g: 139, b: 210, a: 255 },
            system: SerializableColor { r: 147, g: 161, b: 161, a: 255 },
            user_input: SerializableColor { r: 88, g: 110, b: 117, a: 255 },
            prompt: SerializableColor { r: 133, g: 153, b: 0, a: 255 },
            line_numbers: SerializableColor { r: 147, g: 161, b: 161, a: 255 },
        }
    }
    
    fn monokai() -> Self {
        Self {
            error: SerializableColor { r: 249, g: 38, b: 114, a: 255 },
            warning: SerializableColor { r: 244, g: 191, b: 117, a: 255 },
            success: SerializableColor { r: 166, g: 226, b: 46, a: 255 },
            info: SerializableColor { r: 102, g: 217, b: 239, a: 255 },
            system: SerializableColor { r: 117, g: 113, b: 94, a: 255 },
            user_input: SerializableColor { r: 248, g: 248, b: 242, a: 255 },
            prompt: SerializableColor { r: 166, g: 226, b: 46, a: 255 },
            line_numbers: SerializableColor { r: 117, g: 113, b: 94, a: 255 },
        }
    }
    
    fn dracula() -> Self {
        Self {
            error: SerializableColor { r: 255, g: 85, b: 85, a: 255 },
            warning: SerializableColor { r: 241, g: 250, b: 140, a: 255 },
            success: SerializableColor { r: 80, g: 250, b: 123, a: 255 },
            info: SerializableColor { r: 139, g: 233, b: 253, a: 255 },
            system: SerializableColor { r: 98, g: 114, b: 164, a: 255 },
            user_input: SerializableColor { r: 248, g: 248, b: 242, a: 255 },
            prompt: SerializableColor { r: 80, g: 250, b: 123, a: 255 },
            line_numbers: SerializableColor { r: 98, g: 114, b: 164, a: 255 },
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
            background: SerializableColor { r: 50, g: 50, b: 50, a: 255 },
            thumb: SerializableColor { r: 100, g: 100, b: 100, a: 255 },
            thumb_hover: SerializableColor { r: 130, g: 130, b: 130, a: 255 },
            auto_hide: true,
        }
    }
    
    fn default_light() -> Self {
        Self {
            width: 14.0,
            background: SerializableColor { r: 240, g: 240, b: 240, a: 255 },
            thumb: SerializableColor { r: 150, g: 150, b: 150, a: 255 },
            thumb_hover: SerializableColor { r: 120, g: 120, b: 120, a: 255 },
            auto_hide: true,
        }
    }
    
    fn solarized_dark() -> Self {
        Self {
            width: 14.0,
            background: SerializableColor { r: 7, g: 54, b: 66, a: 255 },
            thumb: SerializableColor { r: 88, g: 110, b: 117, a: 255 },
            thumb_hover: SerializableColor { r: 101, g: 123, b: 131, a: 255 },
            auto_hide: true,
        }
    }
    
    fn solarized_light() -> Self {
        Self {
            width: 14.0,
            background: SerializableColor { r: 238, g: 232, b: 213, a: 255 },
            thumb: SerializableColor { r: 147, g: 161, b: 161, a: 255 },
            thumb_hover: SerializableColor { r: 131, g: 148, b: 150, a: 255 },
            auto_hide: true,
        }
    }
    
    fn monokai() -> Self {
        Self {
            width: 14.0,
            background: SerializableColor { r: 73, g: 72, b: 62, a: 255 },
            thumb: SerializableColor { r: 117, g: 113, b: 94, a: 255 },
            thumb_hover: SerializableColor { r: 136, g: 131, b: 107, a: 255 },
            auto_hide: true,
        }
    }
    
    fn dracula() -> Self {
        Self {
            width: 14.0,
            background: SerializableColor { r: 68, g: 71, b: 90, a: 255 },
            thumb: SerializableColor { r: 98, g: 114, b: 164, a: 255 },
            thumb_hover: SerializableColor { r: 139, g: 233, b: 253, a: 255 },
            auto_hide: true,
        }
    }
}

impl UiColors {
    fn default_dark() -> Self {
        Self {
            border: SerializableColor { r: 70, g: 70, b: 70, a: 255 },
            tab_background: SerializableColor { r: 45, g: 45, b: 45, a: 255 },
            active_tab_background: SerializableColor { r: 30, g: 30, b: 30, a: 255 },
            tab_text: SerializableColor { r: 180, g: 180, b: 180, a: 255 },
            active_tab_text: SerializableColor { r: 255, g: 255, b: 255, a: 255 },
            status_bar_background: SerializableColor { r: 25, g: 25, b: 25, a: 255 },
            status_bar_text: SerializableColor { r: 200, g: 200, b: 200, a: 255 },
        }
    }
    
    fn default_light() -> Self {
        Self {
            border: SerializableColor { r: 190, g: 190, b: 190, a: 255 },
            tab_background: SerializableColor { r: 230, g: 230, b: 230, a: 255 },
            active_tab_background: SerializableColor { r: 255, g: 255, b: 255, a: 255 },
            tab_text: SerializableColor { r: 80, g: 80, b: 80, a: 255 },
            active_tab_text: SerializableColor { r: 0, g: 0, b: 0, a: 255 },
            status_bar_background: SerializableColor { r: 240, g: 240, b: 240, a: 255 },
            status_bar_text: SerializableColor { r: 60, g: 60, b: 60, a: 255 },
        }
    }
    
    fn solarized_dark() -> Self {
        Self {
            border: SerializableColor { r: 88, g: 110, b: 117, a: 255 },
            tab_background: SerializableColor { r: 7, g: 54, b: 66, a: 255 },
            active_tab_background: SerializableColor { r: 0, g: 43, b: 54, a: 255 },
            tab_text: SerializableColor { r: 131, g: 148, b: 150, a: 255 },
            active_tab_text: SerializableColor { r: 147, g: 161, b: 161, a: 255 },
            status_bar_background: SerializableColor { r: 7, g: 54, b: 66, a: 255 },
            status_bar_text: SerializableColor { r: 131, g: 148, b: 150, a: 255 },
        }
    }
    
    fn solarized_light() -> Self {
        Self {
            border: SerializableColor { r: 147, g: 161, b: 161, a: 255 },
            tab_background: SerializableColor { r: 238, g: 232, b: 213, a: 255 },
            active_tab_background: SerializableColor { r: 253, g: 246, b: 227, a: 255 },
            tab_text: SerializableColor { r: 101, g: 123, b: 131, a: 255 },
            active_tab_text: SerializableColor { r: 88, g: 110, b: 117, a: 255 },
            status_bar_background: SerializableColor { r: 238, g: 232, b: 213, a: 255 },
            status_bar_text: SerializableColor { r: 101, g: 123, b: 131, a: 255 },
        }
    }
    
    fn monokai() -> Self {
        Self {
            border: SerializableColor { r: 117, g: 113, b: 94, a: 255 },
            tab_background: SerializableColor { r: 73, g: 72, b: 62, a: 255 },
            active_tab_background: SerializableColor { r: 39, g: 40, b: 34, a: 255 },
            tab_text: SerializableColor { r: 204, g: 204, b: 204, a: 255 },
            active_tab_text: SerializableColor { r: 248, g: 248, b: 242, a: 255 },
            status_bar_background: SerializableColor { r: 73, g: 72, b: 62, a: 255 },
            status_bar_text: SerializableColor { r: 248, g: 248, b: 242, a: 255 },
        }
    }
    
    fn dracula() -> Self {
        Self {
            border: SerializableColor { r: 98, g: 114, b: 164, a: 255 },
            tab_background: SerializableColor { r: 68, g: 71, b: 90, a: 255 },
            active_tab_background: SerializableColor { r: 40, g: 42, b: 54, a: 255 },
            tab_text: SerializableColor { r: 248, g: 248, b: 242, a: 255 },
            active_tab_text: SerializableColor { r: 255, g: 255, b: 255, a: 255 },
            status_bar_background: SerializableColor { r: 68, g: 71, b: 90, a: 255 },
            status_bar_text: SerializableColor { r: 248, g: 248, b: 242, a: 255 },
        }
    }
}

impl SelectionSettings {
    fn default_dark() -> Self {
        Self {
            background: SerializableColor { r: 58, g: 58, b: 58, a: 255 },
            foreground: None,
            border_width: 0.0,
            border_color: SerializableColor { r: 100, g: 100, b: 100, a: 255 },
        }
    }
    
    fn default_light() -> Self {
        Self {
            background: SerializableColor { r: 197, g: 197, b: 197, a: 255 },
            foreground: None,
            border_width: 0.0,
            border_color: SerializableColor { r: 150, g: 150, b: 150, a: 255 },
        }
    }
    
    fn solarized_dark() -> Self {
        Self {
            background: SerializableColor { r: 7, g: 54, b: 66, a: 255 },
            foreground: None,
            border_width: 0.0,
            border_color: SerializableColor { r: 88, g: 110, b: 117, a: 255 },
        }
    }
    
    fn solarized_light() -> Self {
        Self {
            background: SerializableColor { r: 238, g: 232, b: 213, a: 255 },
            foreground: None,
            border_width: 0.0,
            border_color: SerializableColor { r: 147, g: 161, b: 161, a: 255 },
        }
    }
    
    fn monokai() -> Self {
        Self {
            background: SerializableColor { r: 73, g: 72, b: 62, a: 255 },
            foreground: None,
            border_width: 0.0,
            border_color: SerializableColor { r: 117, g: 113, b: 94, a: 255 },
        }
    }
    
    fn dracula() -> Self {
        Self {
            background: SerializableColor { r: 68, g: 71, b: 90, a: 255 },
            foreground: None,
            border_width: 0.0,
            border_color: SerializableColor { r: 98, g: 114, b: 164, a: 255 },
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
                background: SerializableColor { r: 255, g: 0, b: 0, a: 255 },
                foreground: SerializableColor { r: 255, g: 255, b: 255, a: 255 },
                cursor: SerializableColor { r: 0, g: 0, b: 255, a: 255 },
                selection_background: SerializableColor { r: 128, g: 128, b: 128, a: 255 },
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
        assert_ne!(dark_theme.colors.background.a, 0);
        assert_ne!(dark_theme.colors.foreground.a, 0);
        
        // Test ANSI colors are different
        assert_ne!(dark_theme.colors.ansi_colors.red, dark_theme.colors.ansi_colors.blue);
    }
}