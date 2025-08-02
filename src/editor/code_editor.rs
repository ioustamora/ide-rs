
//! Advanced Code Editor with LSP Integration
//!
//! This module provides a professional code editor with:
//! - Real-time syntax highlighting
//! - LSP-powered code completion
//! - Error highlighting and diagnostics
//! - Go-to definition and hover information
//! - Code actions and quick fixes
//! - Find and replace functionality




pub use crate::editor::code_editor::types::*;
pub use crate::editor::code_editor::state::*;
pub use crate::editor::code_editor::find_replace::*;
pub use crate::editor::code_editor::render::*;
pub use crate::editor::code_editor::lsp::*;
pub use crate::editor::code_editor::ai::*;
use crate::editor::syntax_highlighter::SyntaxHighlighter;
use crate::rcl::ui::component::Component;
use crate::editor::output_panel::OutputPanel;
use crate::editor::lsp_integration::{LspClient, Diagnostic};
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

/// Advanced code editor with LSP integration
pub struct CodeEditor {
    /// Show go-to-line dialog
    pub show_goto_line: bool,
    /// Go-to-line input buffer
    pub goto_line_input: String,
    /// Current file content
    pub code: String,
    /// Programming language
    pub language: String,
    /// Whether editor is in edit mode
    pub editable: bool,
    /// Current file path
    pub file_path: Option<String>,
    /// Cursor position (line, column)
    pub cursor_pos: (usize, usize),
    /// Current selection
    pub selection: Option<TextSelection>,
    /// Additional selections for multicursor
    pub extra_selections: Vec<TextSelection>,
    /// Find/replace state
    pub find_replace: FindReplaceState,
    /// Editor settings
    pub settings: EditorSettings,
    /// Undo/redo history
    pub history: EditHistory,
    /// Code folding state
    pub folding: CodeFoldingState,
    /// Diagnostics for current file
    pub diagnostics: Vec<Diagnostic>,
    /// Syntax highlighter
    pub syntax_highlighter: SyntaxHighlighter,
    /// Current theme name
    pub theme_name: String,
    /// Path to user config for theme persistence
    pub config_path: Option<PathBuf>,
}

// ...types and state moved to code_editor/types.rs...

impl CodeEditor {
    /// Render a minimap of the code to the right of the editor
    fn render_minimap(&mut self, ui: &mut egui::Ui) {
        render::render_minimap(self, ui);
    }
    pub fn new(language: &str) -> Self {
        state::new_editor(language)
    }

    /// Create a new code editor with content
    pub fn with_content(language: &str, content: String) -> Self {
        state::with_content(language, content)
    }

    /// Open a file in the editor
    pub fn open_file(&mut self, file_path: String, content: String) {
        state::open_file(self, file_path, content);
    }

    /// Save the current file
    pub fn save_file(&mut self, output_panel: &mut OutputPanel) -> Result<(), std::io::Error> {
        state::save_file(self, output_panel)
    }

    /// Update diagnostics from LSP
    pub fn update_diagnostics(&mut self, lsp_client: &LspClient) {
        lsp::update_diagnostics(self, lsp_client);
    }

    /// Render the code editor with enhanced features

    pub fn render_enhanced(&mut self, ui: &mut egui::Ui, lsp_client: &mut LspClient, output_panel: &mut OutputPanel) {
        render::render_enhanced(self, ui, lsp_client, output_panel);
    }

    /// Render the editor toolbar
    fn render_toolbar(&mut self, ui: &mut egui::Ui, output_panel: &mut OutputPanel) {
        render::render_toolbar(self, ui, output_panel);
    }

    // ...find/replace panel logic moved to code_editor/find_replace.rs...

    /// Render the main editor content with advanced features
    fn render_editor_content(&mut self, ui: &mut egui::Ui) {
        render::render_editor_content(self, ui);
    }

    /// Render status bar
    fn render_status_bar(&self, ui: &mut egui::Ui) {
        render::render_status_bar(self, ui);
    }

    // get_syntax_color is now obsolete (replaced by token-based highlighting)

    /// Get diagnostic at specific line
    fn get_diagnostic_at_line(&self, line_num: usize) -> Option<&Diagnostic> {
        lsp::get_diagnostic_at_line(self, line_num)
    }

    /// Get foldable region at specific line
    fn get_foldable_region_at_line(&self, line_num: usize) -> Option<&FoldableRegion> {
        state::get_foldable_region_at_line(self, line_num)
    }

    /// Check if line is folded
    fn is_line_folded(&self, line_num: usize) -> bool {
        state::is_line_folded(self, line_num)
    }

    /// Toggle fold at line
    fn toggle_fold(&mut self, line_num: usize) {
        state::toggle_fold(self, line_num);
    }

    /// Analyze code for foldable regions - public method
    pub fn analyze_foldable_regions(&mut self) {
        state::analyze_foldable_regions(self);
    }

    /// Find next occurrence
    fn find_next(&mut self) {
        find_replace::find_next(self);
    }

    /// Find previous occurrence
    fn find_previous(&mut self) {
        find_replace::find_previous(self);
    }

    /// Replace current selection
    fn replace_current(&mut self) {
        find_replace::replace_current(self);
    }

    /// Replace all occurrences
    fn replace_all(&mut self) {
        find_replace::replace_all(self);
    }

    /// Update search results
    fn update_search_results(&mut self) {
        find_replace::update_search_results(self);
    }

    /// Convert byte position to line/column
    fn byte_pos_to_line_col(&self, byte_pos: usize) -> (usize, usize) {
        find_replace::byte_pos_to_line_col(self, byte_pos)
    }

    /// Format code
    fn format_code(&mut self, output_panel: &mut OutputPanel) {
        render::format_code(self, output_panel);
    }

    /// Undo last operation
    pub fn undo(&mut self) -> bool {
        state::undo(self)
    }

    pub fn redo(&mut self) -> bool {
        state::redo(self)
    }

    /// Go to definition (delegated to LSP)
    fn go_to_definition(&mut self) {
        lsp::go_to_definition(self);
    }

    /// Add multicursor below (AI/advanced editing)

    fn add_multicursor_below(&mut self) {
        ai::add_multicursor_below(self);
    }
}




