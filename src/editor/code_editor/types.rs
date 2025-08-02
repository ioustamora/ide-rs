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

pub struct EditHistory {
    pub operations: Vec<EditOperation>,
    pub current_index: usize,
    pub max_size: usize,
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
