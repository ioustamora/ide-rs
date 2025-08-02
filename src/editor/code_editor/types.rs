//! Shared types for code editor

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct TextSelection {
    pub start: (usize, usize),
    pub end: (usize, usize),
}

#[derive(Default)]
pub struct FindReplaceState {
    pub find_text: String,
    pub replace_text: String,
    pub case_sensitive: bool,
    pub show_panel: bool,
    pub search_results: Vec<TextSelection>,
    pub current_result: usize,
    pub use_regex: bool,
}

pub struct EditorSettings {
    pub font_size: f32,
    pub tab_size: usize,
    pub show_line_numbers: bool,
    pub show_inline_diagnostics: bool,
    pub auto_complete: bool,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            font_size: 14.0,
            tab_size: 4,
            show_line_numbers: true,
            show_inline_diagnostics: true,
            auto_complete: true,
        }
    }
}

pub struct EditHistory {
    pub operations: Vec<EditOperation>,
    pub current_index: usize,
    pub max_size: usize,
}

impl Default for EditHistory {
    fn default() -> Self {
        Self {
            operations: Vec::new(),
            current_index: 0,
            max_size: 100,
        }
    }
}

#[derive(Clone)]
pub struct EditOperation {
    pub op_type: EditOperationType,
    pub position: (usize, usize),
    pub text: String,
    pub length: usize,
}

#[derive(Clone)]
pub enum EditOperationType {
    Insert,
    Delete,
    Replace,
}

/// Main code editor struct
#[derive(Default)]
pub struct CodeEditor {
    pub code: String,
    pub language: String,
    pub cursor_pos: (usize, usize),
    pub selection: Option<TextSelection>,
    pub settings: EditorSettings,
    pub history: EditHistory,
    pub find_replace: FindReplaceState,
    pub folded_regions: HashMap<usize, bool>,
}

impl CodeEditor {
    pub fn new() -> Self {
        Self {
            code: String::new(),
            language: "rust".to_string(),
            cursor_pos: (0, 0),
            selection: None,
            settings: EditorSettings::default(),
            history: EditHistory::default(),
            find_replace: FindReplaceState::default(),
            folded_regions: HashMap::new(),
        }
    }

    pub fn with_content(language: &str, content: String) -> Self {
        let mut editor = Self::new();
        editor.language = language.to_string();
        editor.code = content;
        editor
    }

    pub fn analyze_foldable_regions(&mut self) {
        // Simple implementation - mark lines with braces as foldable
        let lines: Vec<&str> = self.code.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            if line.contains('{') || line.contains("fn ") || line.contains("impl ") {
                self.folded_regions.insert(i, false);
            }
        }
    }

    pub fn render_enhanced(
        &mut self,
        ui: &mut eframe::egui::Ui,
        _lsp_client: &mut crate::editor::lsp_integration::LspClient,
        _output_panel: &mut crate::editor::output_panel::OutputPanel,
    ) {
        // Basic code editor rendering
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Language: {}", self.language));
                ui.separator();
                ui.label(format!("Lines: {}", self.code.lines().count()));
            });
            ui.separator();
            
            eframe::egui::ScrollArea::vertical()
                .max_height(400.0)
                .show(ui, |ui| {
                    ui.text_edit_multiline(&mut self.code);
                });
        });
    }
}

pub struct CodeFoldingState {
    pub folded_regions: HashMap<usize, usize>,
    pub foldable_regions: Vec<FoldableRegion>,
}

#[derive(Clone)]
pub struct FoldableRegion {
    pub start_line: usize,
    pub end_line: usize,
    pub is_folded: bool,
}
