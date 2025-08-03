// Token-based syntax highlighting and theme support for CodeEditor
// This module provides a wrapper around syntect for use in the code editor.

use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Theme, Style as SyntectStyle};
use syntect::easy::HighlightLines;
use egui::Color32;

pub struct SyntaxHighlighter {
    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,
    pub theme: Theme,
}

impl SyntaxHighlighter {
    pub fn new(theme_name: &str) -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();
        let theme = theme_set.themes.get(theme_name).cloned().unwrap_or_else(|| theme_set.themes["InspiredGitHub"].clone());
        Self { syntax_set, theme_set, theme }
    }

    pub fn highlight_line(&self, line: &str, language: &str) -> Vec<(String, Color32)> {
        let syntax = self.syntax_set.find_syntax_by_token(language).unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());
        let mut h = HighlightLines::new(syntax, &self.theme);
        let mut result = Vec::new();
        for (style, text) in h.highlight_line(line, &self.syntax_set).unwrap_or_default() {
            result.push((text.to_string(), Self::syntect_to_egui_color(style)));
        }
        result
    }

    fn syntect_to_egui_color(style: SyntectStyle) -> Color32 {
        Color32::from_rgba_unmultiplied(style.foreground.r, style.foreground.g, style.foreground.b, style.foreground.a)
    }

    pub fn available_themes(&self) -> Vec<String> {
        self.theme_set.themes.keys().cloned().collect()
    }

    pub fn set_theme(&mut self, theme_name: &str) {
        if let Some(theme) = self.theme_set.themes.get(theme_name) {
            self.theme = theme.clone();
        }
    }
}
