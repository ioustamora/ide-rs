//! Shared types for code editor

use std::collections::HashMap;
use crate::editor::lsp_integration::{CompletionItem, Diagnostic};

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

/// Autocomplete popup state
#[derive(Default)]
pub struct AutocompleteState {
    pub visible: bool,
    pub items: Vec<CompletionItem>,
    pub selected_index: usize,
    pub trigger_position: (usize, usize),
    pub filter_text: String,
    pub popup_rect: Option<eframe::egui::Rect>,
}

/// Inline diagnostics display
#[derive(Default)]
pub struct InlineDiagnostics {
    pub diagnostics: Vec<Diagnostic>,
    pub show_error_popup: bool,
    pub hover_diagnostic: Option<Diagnostic>,
    pub error_positions: HashMap<usize, Vec<Diagnostic>>, // line -> diagnostics
}

/// Code folding regions
#[derive(Default)]
pub struct CodeFolding {
    pub regions: Vec<FoldableRegion>,
    pub folded_lines: std::collections::HashSet<usize>,
}

/// Enhanced editor settings with modern IDE features
pub struct EditorSettings {
    pub font_size: f32,
    pub tab_size: usize,
    pub show_line_numbers: bool,
    pub show_inline_diagnostics: bool,
    pub auto_complete: bool,
    pub show_minimap: bool,
    pub current_theme: EditorTheme,
    pub minimap_width: f32,
    pub word_wrap: bool,
    pub auto_save: bool,
    pub show_whitespace: bool,
    pub highlight_current_line: bool,
    pub auto_indent: bool,
    pub bracket_matching: bool,
    pub code_folding: bool,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            font_size: 14.0,
            tab_size: 4,
            show_line_numbers: true,
            show_inline_diagnostics: true,
            auto_complete: true,
            show_minimap: true,
            current_theme: EditorTheme::default(),
            minimap_width: 120.0,
            word_wrap: false,
            auto_save: false,
            show_whitespace: false,
            highlight_current_line: true,
            auto_indent: true,
            bracket_matching: true,
            code_folding: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct EditorTheme {
    pub name: String,
    pub background: egui::Color32,
    pub text: egui::Color32,
    pub comment: egui::Color32,
    pub keyword: egui::Color32,
    pub string: egui::Color32,
    pub type_name: egui::Color32,
    pub number: egui::Color32,
    pub line_number: egui::Color32,
    pub line_number_bg: egui::Color32,
    pub selection: egui::Color32,
    pub cursor: egui::Color32,
    pub current_line: egui::Color32,
}

impl Default for EditorTheme {
    fn default() -> Self {
        Self::dark_theme()
    }
}

impl EditorTheme {
    pub fn dark_theme() -> Self {
        Self {
            name: "Dark".to_string(),
            background: egui::Color32::from_rgb(30, 30, 30),
            text: egui::Color32::from_rgb(220, 220, 220),
            comment: egui::Color32::from_rgb(106, 153, 85),
            keyword: egui::Color32::from_rgb(86, 156, 214),
            string: egui::Color32::from_rgb(206, 145, 120),
            type_name: egui::Color32::from_rgb(78, 201, 176),
            number: egui::Color32::from_rgb(181, 206, 168),
            line_number: egui::Color32::from_rgb(133, 133, 133),
            line_number_bg: egui::Color32::from_rgb(37, 37, 38),
            selection: egui::Color32::from_rgba_premultiplied(0, 122, 204, 60),
            cursor: egui::Color32::WHITE,
            current_line: egui::Color32::from_rgba_premultiplied(255, 255, 255, 8),
        }
    }
    
    pub fn light_theme() -> Self {
        Self {
            name: "Light".to_string(),
            background: egui::Color32::WHITE,
            text: egui::Color32::BLACK,
            comment: egui::Color32::from_rgb(0, 128, 0),
            keyword: egui::Color32::from_rgb(0, 0, 255),
            string: egui::Color32::from_rgb(163, 21, 21),
            type_name: egui::Color32::from_rgb(43, 145, 175),
            number: egui::Color32::from_rgb(9, 134, 88),
            line_number: egui::Color32::from_rgb(128, 128, 128),
            line_number_bg: egui::Color32::from_rgb(245, 245, 245),
            selection: egui::Color32::from_rgba_premultiplied(173, 214, 255, 120),
            cursor: egui::Color32::BLACK,
            current_line: egui::Color32::from_rgba_premultiplied(0, 0, 0, 8),
        }
    }
    
    pub fn monokai_theme() -> Self {
        Self {
            name: "Monokai".to_string(),
            background: egui::Color32::from_rgb(39, 40, 34),
            text: egui::Color32::from_rgb(248, 248, 242),
            comment: egui::Color32::from_rgb(117, 113, 94),
            keyword: egui::Color32::from_rgb(249, 38, 114),
            string: egui::Color32::from_rgb(230, 219, 116),
            type_name: egui::Color32::from_rgb(102, 217, 239),
            number: egui::Color32::from_rgb(174, 129, 255),
            line_number: egui::Color32::from_rgb(90, 90, 90),
            line_number_bg: egui::Color32::from_rgb(35, 36, 31),
            selection: egui::Color32::from_rgba_premultiplied(73, 72, 62, 180),
            cursor: egui::Color32::WHITE,
            current_line: egui::Color32::from_rgba_premultiplied(255, 255, 255, 6),
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

/// Main code editor struct with modern IDE features
#[derive(Default)]
pub struct CodeEditor {
    /// The code content
    pub code: String,
    /// Programming language
    pub language: String,
    /// Current cursor position (line, column)
    pub cursor_pos: (usize, usize),
    /// Current text selection
    pub selection: Option<TextSelection>,
    /// Editor settings and preferences
    pub settings: EditorSettings,
    /// Edit history for undo/redo
    pub history: EditHistory,
    /// Find and replace state
    pub find_replace: FindReplaceState,
    /// Folded regions
    pub folded_regions: HashMap<usize, bool>,
    /// Autocomplete popup state
    pub autocomplete: AutocompleteState,
    /// Inline diagnostics
    pub diagnostics: InlineDiagnostics,
    /// Code folding state
    pub code_folding: CodeFolding,
    /// Last known file modification time
    pub last_modified: Option<std::time::SystemTime>,
    /// Dirty flag for unsaved changes
    pub is_dirty: bool,
    /// Scroll position
    pub scroll_offset: (f32, f32),
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
            autocomplete: AutocompleteState::default(),
            diagnostics: InlineDiagnostics::default(),
            code_folding: CodeFolding::default(),
            last_modified: None,
            is_dirty: false,
            scroll_offset: (0.0, 0.0),
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
        lsp_client: &mut crate::editor::lsp_integration::LspClient,
        _output_panel: &mut crate::editor::output_panel::OutputPanel,
    ) {
        ui.vertical(|ui| {
            // Enhanced toolbar with LSP status
            ui.horizontal(|ui| {
                ui.label(format!("📄 {} ({})", 
                    if self.is_dirty { "● Modified" } else { "✓ Saved" },
                    self.language
                ));
                ui.separator();
                
                // LSP connection status
                let lsp_status = if lsp_client.is_connected() {
                    ui.colored_label(eframe::egui::Color32::GREEN, "🟢 LSP Connected");
                } else {
                    ui.colored_label(eframe::egui::Color32::RED, "🔴 LSP Disconnected");
                };
                
                ui.separator();
                
                // Code actions
                if ui.button("🔧 Complete").on_hover_text("Trigger Autocomplete (Ctrl+Space)").clicked() {
                    self.trigger_autocomplete(lsp_client);
                }
                
                if ui.button("💡 Hover").on_hover_text("Show Hover Info").clicked() {
                    self.request_hover_info(lsp_client);
                }
                
                if ui.button("🎯 Format").on_hover_text("Format Code").clicked() {
                    // TODO: Implement code formatting
                    self.mark_dirty();
                }
            });
            ui.separator();
            
            // Main editor area with enhanced features
            self.render(ui);
            
            // Show diagnostics panel if there are any
            if !self.diagnostics.diagnostics.is_empty() {
                ui.separator();
                ui.collapsing("🔍 Diagnostics", |ui| {
                    self.render_diagnostics_panel(ui);
                });
            }
        });
    }

    /// Trigger autocomplete request from LSP
    fn trigger_autocomplete(&mut self, lsp_client: &mut crate::editor::lsp_integration::LspClient) {
        let (line, character) = self.cursor_pos;
        let uri = format!("file://current_file.{}", self.language);
        
        // For now, show a simple placeholder autocomplete
        // In a real implementation, we would handle the LSP callback properly
        if lsp_client.is_connected() {
            let placeholder_completions = vec![
                crate::editor::lsp_integration::CompletionItem {
                    label: "println!".to_string(),
                    kind: Some(crate::editor::lsp_integration::CompletionItemKind::Function),
                    detail: Some("Print to stdout".to_string()),
                    documentation: Some("Prints to the standard output".to_string()),
                    sort_text: None,
                    filter_text: None,
                    insert_text: Some("println!(\"{}\", );".to_string()),
                    insert_text_format: None,
                },
                crate::editor::lsp_integration::CompletionItem {
                    label: "String".to_string(),
                    kind: Some(crate::editor::lsp_integration::CompletionItemKind::Class),
                    detail: Some("String type".to_string()),
                    documentation: Some("UTF-8 encoded string".to_string()),
                    sort_text: None,
                    filter_text: None,
                    insert_text: Some("String::new()".to_string()),
                    insert_text_format: None,
                },
            ];
            self.show_autocomplete(placeholder_completions);
        }
    }

    /// Request hover information from LSP
    fn request_hover_info(&mut self, lsp_client: &mut crate::editor::lsp_integration::LspClient) {
        let (_line, _character) = self.cursor_pos;
        
        // For now, show a simple placeholder hover
        if lsp_client.is_connected() {
            self.diagnostics.show_error_popup = true;
        }
    }

    /// Render diagnostics panel
    fn render_diagnostics_panel(&mut self, ui: &mut eframe::egui::Ui) {
        eframe::egui::ScrollArea::vertical()
            .max_height(150.0)
            .show(ui, |ui| {
                for diagnostic in &self.diagnostics.diagnostics {
                    ui.horizontal(|ui| {
                        let (icon, color) = match diagnostic.severity {
                            Some(crate::editor::lsp_integration::DiagnosticSeverity::Error) => ("❌", eframe::egui::Color32::RED),
                            Some(crate::editor::lsp_integration::DiagnosticSeverity::Warning) => ("⚠️", eframe::egui::Color32::YELLOW),
                            Some(crate::editor::lsp_integration::DiagnosticSeverity::Information) => ("ℹ️", eframe::egui::Color32::BLUE),
                            Some(crate::editor::lsp_integration::DiagnosticSeverity::Hint) => ("💡", eframe::egui::Color32::GRAY),
                            None => ("•", eframe::egui::Color32::WHITE),
                        };
                        
                        ui.colored_label(color, icon);
                        ui.label(&diagnostic.message);
                        ui.label(format!("Line {}", diagnostic.range.start.line + 1));
                        
                        if ui.small_button("🎯").on_hover_text("Go to error").clicked() {
                            self.cursor_pos = (diagnostic.range.start.line as usize, diagnostic.range.start.character as usize);
                        }
                    });
                }
            });
    }

    /// Undo the last operation
    pub fn undo(&mut self) {
        if self.history.current_index > 0 {
            self.history.current_index -= 1;
            // Apply reverse operation - placeholder implementation
        }
    }

    /// Redo the last undone operation
    pub fn redo(&mut self) {
        if self.history.current_index < self.history.operations.len() {
            // Apply operation - placeholder implementation
            self.history.current_index += 1;
        }
    }

    /// Cut selected text to clipboard
    pub fn cut(&mut self) {
        if let Some(_selection) = &self.selection {
            // Copy to clipboard first
            self.copy();
            // Then delete selected text - placeholder implementation
            // In a real implementation, this would remove the selected text
        }
    }

    /// Copy selected text to clipboard
    pub fn copy(&mut self) {
        if let Some(_selection) = &self.selection {
            // Placeholder implementation - would copy selected text to clipboard
            // In a real implementation, this would use a clipboard crate
        }
    }

    /// Paste text from clipboard
    pub fn paste(&mut self) {
        // Placeholder implementation - would paste from clipboard
        // In a real implementation, this would get text from clipboard and insert it
    }

    /// Select all text in the editor
    pub fn select_all(&mut self) {
        let lines: Vec<&str> = self.code.lines().collect();
        if !lines.is_empty() {
            let last_line = lines.len() - 1;
            let last_col = lines[last_line].len();
            self.selection = Some(TextSelection {
                start: (0, 0),
                end: (last_line, last_col),
            });
        }
    }

    /// Show autocomplete popup at current cursor position
    pub fn show_autocomplete(&mut self, completions: Vec<CompletionItem>) {
        self.autocomplete.visible = true;
        self.autocomplete.items = completions;
        self.autocomplete.selected_index = 0;
        self.autocomplete.trigger_position = self.cursor_pos;
        self.autocomplete.filter_text.clear();
    }

    /// Hide autocomplete popup
    pub fn hide_autocomplete(&mut self) {
        self.autocomplete.visible = false;
        self.autocomplete.items.clear();
        self.autocomplete.selected_index = 0;
    }

    /// Move autocomplete selection up
    pub fn autocomplete_previous(&mut self) {
        if self.autocomplete.visible && !self.autocomplete.items.is_empty() {
            if self.autocomplete.selected_index > 0 {
                self.autocomplete.selected_index -= 1;
            } else {
                self.autocomplete.selected_index = self.autocomplete.items.len() - 1;
            }
        }
    }

    /// Move autocomplete selection down
    pub fn autocomplete_next(&mut self) {
        if self.autocomplete.visible && !self.autocomplete.items.is_empty() {
            if self.autocomplete.selected_index < self.autocomplete.items.len() - 1 {
                self.autocomplete.selected_index += 1;
            } else {
                self.autocomplete.selected_index = 0;
            }
        }
    }

    /// Accept current autocomplete selection
    pub fn autocomplete_accept(&mut self) {
        if self.autocomplete.visible && !self.autocomplete.items.is_empty() {
            let selected_index = self.autocomplete.selected_index;
            let item = self.autocomplete.items[selected_index].clone();
            
            // Insert the completion text
            if let Some(insert_text) = &item.insert_text {
                self.insert_text_at_cursor(insert_text);
                self.mark_dirty();
            } else {
                self.insert_text_at_cursor(&item.label);
                self.mark_dirty();
            }
            
            self.hide_autocomplete();
        }
    }

    /// Insert text at current cursor position
    pub fn insert_text_at_cursor(&mut self, text: &str) {
        let lines: Vec<&str> = self.code.lines().collect();
        let (line, col) = self.cursor_pos;
        
        if line < lines.len() {
            let current_line = lines[line];
            let (before, after) = current_line.split_at(col.min(current_line.len()));
            let new_line = format!("{}{}{}", before, text, after);
            
            let mut new_lines = lines.clone();
            new_lines[line] = &new_line;
            self.code = new_lines.join("\n");
            
            // Update cursor position
            self.cursor_pos.1 += text.len();
        }
    }

    /// Update diagnostics from LSP
    pub fn update_diagnostics(&mut self, diagnostics: Vec<Diagnostic>) {
        self.diagnostics.diagnostics = diagnostics.clone();
        self.diagnostics.error_positions.clear();
        
        // Group diagnostics by line
        for diagnostic in diagnostics {
            let line = diagnostic.range.start.line as usize;
            self.diagnostics.error_positions
                .entry(line)
                .or_insert_with(Vec::new)
                .push(diagnostic);
        }
    }

    /// Mark editor as dirty (has unsaved changes)
    pub fn mark_dirty(&mut self) {
        self.is_dirty = true;
    }

    /// Mark editor as clean (saved)
    pub fn mark_clean(&mut self) {
        self.is_dirty = false;
        self.last_modified = Some(std::time::SystemTime::now());
    }

    /// Toggle code folding for a region
    pub fn toggle_fold(&mut self, line: usize) {
        if let Some(region) = self.code_folding.regions.iter_mut().find(|r| r.start_line == line) {
            region.is_folded = !region.is_folded;
            if region.is_folded {
                self.code_folding.folded_lines.insert(line);
            } else {
                self.code_folding.folded_lines.remove(&line);
            }
        }
    }

    /// Get word at cursor position for autocomplete
    pub fn get_word_at_cursor(&self) -> String {
        let lines: Vec<&str> = self.code.lines().collect();
        let (line, col) = self.cursor_pos;
        
        if line < lines.len() {
            let current_line = lines[line];
            let bytes = current_line.as_bytes();
            
            // Find word boundaries
            let mut start = col;
            let mut end = col;
            
            // Move backward to find start of word
            while start > 0 && Self::is_word_char(bytes[start - 1]) {
                start -= 1;
            }
            
            // Move forward to find end of word
            while end < bytes.len() && Self::is_word_char(bytes[end]) {
                end += 1;
            }
            
            if start < end {
                return current_line[start..end].to_string();
            }
        }
        
        String::new()
    }

    /// Check if character is part of a word (alphanumeric or underscore)
    fn is_word_char(c: u8) -> bool {
        (c >= b'a' && c <= b'z') ||
        (c >= b'A' && c <= b'Z') ||
        (c >= b'0' && c <= b'9') ||
        c == b'_'
    }

    /// Render the code editor with advanced features
    pub fn render(&mut self, ui: &mut eframe::egui::Ui) {
        ui.vertical(|ui| {
            // Toolbar
            self.render_toolbar(ui);
            ui.separator();
            
            // Main editor area with line numbers, code, and minimap
            ui.horizontal(|ui| {
                // Apply theme background and styling
                let frame = eframe::egui::Frame::none()
                    .fill(self.settings.current_theme.background)
                    .stroke(eframe::egui::Stroke::new(1.0, self.settings.current_theme.line_number))
                    .inner_margin(eframe::egui::Margin::same(4.0));
                
                frame.show(ui, |ui| {
                    // Override UI style with theme colors
                    ui.style_mut().visuals.extreme_bg_color = self.settings.current_theme.background;
                    ui.style_mut().visuals.override_text_color = Some(self.settings.current_theme.text);
                    ui.horizontal(|ui| {
                        // Line numbers column
                        if self.settings.show_line_numbers {
                            self.render_line_numbers(ui);
                        }
                        
                        // Code editor with syntax highlighting
                        let available_width = if self.settings.show_minimap {
                            ui.available_width() - self.settings.minimap_width - 8.0
                        } else {
                            ui.available_width()
                        };
                        
                        ui.allocate_ui_with_layout(
                            eframe::egui::Vec2::new(available_width, ui.available_height()),
                            eframe::egui::Layout::top_down(eframe::egui::Align::LEFT),
                            |ui| {
                                eframe::egui::ScrollArea::both()
                                    .auto_shrink([false, false])
                                    .show(ui, |ui| {
                                        // Use enhanced syntax highlighting for all languages
                                        self.render_enhanced_syntax_highlighted(ui);
                                    });
                            }
                        );
                        
                        // Minimap
                        if self.settings.show_minimap {
                            ui.separator();
                            self.render_minimap(ui);
                        }
                    });
                });
            });
            
            // Status bar
            self.render_status_bar(ui);
            
            // Render autocomplete popup overlay
            if self.autocomplete.visible {
                self.render_autocomplete_popup(ui);
            }
            
            // Render diagnostics popup overlay
            if self.settings.show_inline_diagnostics && self.diagnostics.show_error_popup {
                self.render_diagnostics_popup(ui);
            }
        });
    }
    
    /// Render editor toolbar
    fn render_toolbar(&mut self, ui: &mut eframe::egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(format!("Language: {}", self.language));
            ui.separator();
            
            if ui.button("🔍").on_hover_text("Find/Replace").clicked() {
                self.find_replace.show_panel = !self.find_replace.show_panel;
            }
            
            if ui.button("⟲").on_hover_text("Undo").clicked() {
                self.undo();
            }
            
            if ui.button("⟳").on_hover_text("Redo").clicked() {
                self.redo();
            }
            
            ui.separator();
            
            // Settings toggles
            ui.checkbox(&mut self.settings.show_line_numbers, "Line Numbers");
            ui.checkbox(&mut self.settings.show_minimap, "Minimap");
            ui.checkbox(&mut self.settings.auto_complete, "Auto Complete");
            ui.checkbox(&mut self.settings.word_wrap, "Word Wrap");
            
            ui.separator();
            
            // Modern editor features
            ui.checkbox(&mut self.settings.show_whitespace, "Whitespace");
            ui.checkbox(&mut self.settings.highlight_current_line, "Highlight Line");
            ui.checkbox(&mut self.settings.auto_indent, "Auto Indent");
            ui.checkbox(&mut self.settings.bracket_matching, "Bracket Match");
            ui.checkbox(&mut self.settings.code_folding, "Code Folding");
            
            ui.separator();
            
            // Theme selector
            egui::ComboBox::from_label("Theme")
                .selected_text(&self.settings.current_theme.name)
                .show_ui(ui, |ui| {
                    if ui.selectable_label(self.settings.current_theme.name == "Dark", "Dark").clicked() {
                        self.settings.current_theme = EditorTheme::dark_theme();
                    }
                    if ui.selectable_label(self.settings.current_theme.name == "Light", "Light").clicked() {
                        self.settings.current_theme = EditorTheme::light_theme();
                    }
                    if ui.selectable_label(self.settings.current_theme.name == "Monokai", "Monokai").clicked() {
                        self.settings.current_theme = EditorTheme::monokai_theme();
                    }
                });
        });
        
        // Find/Replace panel
        if self.find_replace.show_panel {
            ui.horizontal(|ui| {
                ui.label("Find:");
                ui.text_edit_singleline(&mut self.find_replace.find_text);
                ui.label("Replace:");
                ui.text_edit_singleline(&mut self.find_replace.replace_text);
                
                if ui.button("Find Next").clicked() {
                    // TODO: Implement find functionality
                }
                if ui.button("Replace").clicked() {
                    // TODO: Implement replace functionality
                }
                ui.checkbox(&mut self.find_replace.case_sensitive, "Case Sensitive");
            });
        }
    }
    
    /// Render line numbers
    fn render_line_numbers(&self, ui: &mut eframe::egui::Ui) {
        let line_count = self.code.lines().count();
        let line_height = ui.text_style_height(&eframe::egui::TextStyle::Monospace);
        
        ui.allocate_ui_with_layout(
            eframe::egui::Vec2::new(40.0, line_height * line_count as f32),
            eframe::egui::Layout::top_down(eframe::egui::Align::RIGHT),
            |ui| {
                ui.style_mut().visuals.extreme_bg_color = self.settings.current_theme.line_number_bg;
                
                for line_num in 1..=line_count {
                    ui.colored_label(self.settings.current_theme.line_number, format!("{:3}", line_num));
                }
            },
        );
        ui.separator();
    }
    
    /// Render minimap
    fn render_minimap(&self, ui: &mut eframe::egui::Ui) {
        let minimap_width = self.settings.minimap_width;
        let available_height = ui.available_height();
        
        ui.allocate_ui_with_layout(
            eframe::egui::Vec2::new(minimap_width, available_height),
            eframe::egui::Layout::top_down(eframe::egui::Align::LEFT),
            |ui| {
                ui.style_mut().visuals.extreme_bg_color = self.settings.current_theme.background;
                
                ui.heading("📍 Minimap");
                ui.separator();
                
                let lines: Vec<&str> = self.code.lines().collect();
                let total_lines = lines.len();
                
                if total_lines > 0 {
                    let visible_lines = (available_height / 3.0) as usize; // Simplified minimap with smaller line height
                    let lines_to_show = visible_lines.min(total_lines);
                    
                    eframe::egui::ScrollArea::vertical()
                        .max_height(available_height - 40.0)
                        .show(ui, |ui| {
                            for (_i, line) in lines.iter().enumerate().take(lines_to_show) {
                                let preview = if line.len() > 20 {
                                    format!("{}...", &line[..17])
                                } else {
                                    line.to_string()
                                };
                                
                                if !preview.trim().is_empty() {
                                    ui.small(preview);
                                } else {
                                    ui.small(" ");
                                }
                            }
                            
                            if total_lines > lines_to_show {
                                ui.small(format!("... {} more lines", total_lines - lines_to_show));
                            }
                        });
                } else {
                    ui.small("Empty file");
                }
            },
        );
    }
    
    /// Render code with enhanced syntect-based syntax highlighting
    fn render_enhanced_syntax_highlighted(&mut self, ui: &mut eframe::egui::Ui) {
        use crate::editor::syntax_highlighter::SyntaxHighlighter;
        
        // Create syntax highlighter based on current theme
        let theme_name = match self.settings.current_theme.name.as_str() {
            "Dark" => "base16-ocean.dark",
            "Light" => "InspiredGitHub", 
            "Monokai" => "Monokai",
            _ => "base16-ocean.dark",
        };
        
        let highlighter = SyntaxHighlighter::new(theme_name);
        
        eframe::egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let available_rect = ui.available_rect_before_wrap();
                let font_id = eframe::egui::FontId::monospace(self.settings.font_size);
                let line_height = ui.fonts(|fonts| fonts.row_height(&font_id));
                
                let lines: Vec<&str> = self.code.lines().collect();
                let visible_lines = (available_rect.height() / line_height).ceil() as usize + 2;
                let start_line = (self.scroll_offset.1 / line_height) as usize;
                let end_line = (start_line + visible_lines).min(lines.len());
                
                for (line_index, line) in lines.iter().enumerate().skip(start_line).take(end_line - start_line) {
                    let highlighted = highlighter.highlight_line(line, &self.language);
                    
                    ui.horizontal(|ui| {
                        // Current line highlighting
                        if self.settings.highlight_current_line && line_index == self.cursor_pos.0 {
                            let line_rect = ui.available_rect_before_wrap();
                            ui.painter().rect_filled(
                                line_rect,
                                0.0,
                                self.settings.current_theme.current_line,
                            );
                        }
                        
                        // Render highlighted text
                        let mut pos = 0.0;
                        for (text, color) in highlighted {
                            let text_galley = ui.fonts(|fonts| {
                                fonts.layout_no_wrap(text, font_id.clone(), color)
                            });
                            
                            let text_rect = eframe::egui::Rect::from_min_size(
                                ui.next_widget_position() + eframe::egui::Vec2::new(pos, 0.0),
                                text_galley.size(),
                            );
                            
                            ui.painter().galley(text_rect.min, text_galley, eframe::egui::Color32::TRANSPARENT);
                            pos += text_galley.size().x;
                        }
                        
                        // Add diagnostic indicators
                        if let Some(diagnostics) = self.diagnostics.error_positions.get(&line_index) {
                            for diagnostic in diagnostics {
                                let icon = match diagnostic.severity {
                                    Some(crate::editor::lsp_integration::DiagnosticSeverity::Error) => "❌",
                                    Some(crate::editor::lsp_integration::DiagnosticSeverity::Warning) => "⚠️",
                                    _ => "💡",
                                };
                                
                                let icon_pos = ui.next_widget_position() + eframe::egui::Vec2::new(pos + 10.0, 0.0);
                                ui.painter().text(
                                    icon_pos,
                                    eframe::egui::Align2::LEFT_TOP,
                                    icon,
                                    eframe::egui::FontId::default(),
                                    eframe::egui::Color32::RED,
                                );
                            }
                        }
                    });
                }
            });
    }
    
    /// Render syntax highlighting overlay
    fn render_syntax_overlay(&self, ui: &mut eframe::egui::Ui, lines: &[&str], rect: eframe::egui::Rect) {
        let line_height = ui.text_style_height(&eframe::egui::TextStyle::Monospace);
        let theme = &self.settings.current_theme;
        
        for (line_idx, line) in lines.iter().enumerate() {
            let y_offset = line_idx as f32 * line_height;
            let line_rect = eframe::egui::Rect::from_min_size(
                rect.min + eframe::egui::Vec2::new(0.0, y_offset),
                eframe::egui::Vec2::new(rect.width(), line_height)
            );
            
            // Highlight keywords, strings, etc.
            self.highlight_line_tokens(ui, line, line_rect, theme);
        }
    }
    
    /// Highlight tokens in a line
    fn highlight_line_tokens(&self, ui: &mut eframe::egui::Ui, line: &str, rect: eframe::egui::Rect, theme: &EditorTheme) {
        let keywords = ["fn", "let", "mut", "if", "else", "for", "while", "loop", "match", "struct", "enum", "impl", "pub", "use", "mod", "return", "const", "static"];
        let types = ["String", "i32", "i64", "f32", "f64", "bool", "char", "Vec", "Option", "Result", "usize", "isize"];
        
        // Simple token-based highlighting
        let _char_pos = 0.0;
        let char_width = ui.fonts(|fonts| fonts.glyph_width(&eframe::egui::TextStyle::Monospace.resolve(ui.style()), ' '));
        
        let words: Vec<&str> = line.split_whitespace().collect();
        let mut line_pos = 0;
        
        for word in words {
            // Find word position in line
            if let Some(word_start) = line[line_pos..].find(word) {
                line_pos += word_start;
                
                let color = if keywords.contains(&word) {
                    theme.keyword
                } else if types.contains(&word) {
                    theme.type_name
                } else if word.starts_with("//") {
                    theme.comment
                } else if word.starts_with('"') && word.ends_with('"') {
                    theme.string
                } else if word.chars().all(|c| c.is_numeric() || c == '.') {
                    theme.number
                } else {
                    continue; // Use default text color
                };
                
                let word_rect = eframe::egui::Rect::from_min_size(
                    rect.min + eframe::egui::Vec2::new(line_pos as f32 * char_width, 0.0),
                    eframe::egui::Vec2::new(word.len() as f32 * char_width, rect.height())
                );
                
                // Draw colored text overlay
                ui.painter().text(
                    word_rect.min,
                    eframe::egui::Align2::LEFT_TOP,
                    word,
                    eframe::egui::FontId::monospace(14.0),
                    color
                );
                
                line_pos += word.len();
            }
        }
    }

    /// Simple Rust syntax highlighting (legacy method)
    fn highlight_rust_line(&self, line: &str, job: &mut eframe::egui::text::LayoutJob) {
        let keywords = ["fn", "let", "mut", "if", "else", "for", "while", "loop", "match", "struct", "enum", "impl", "pub", "use", "mod"];
        let types = ["String", "i32", "i64", "f32", "f64", "bool", "char", "Vec", "Option", "Result"];
        let theme = &self.settings.current_theme;
        
        let words: Vec<&str> = line.split_whitespace().collect();
        let mut pos = 0;
        
        for word in words {
            // Find the actual position in the line
            while pos < line.len() && !line[pos..].starts_with(word) {
                let default_format = eframe::egui::TextFormat {
                    color: theme.text,
                    ..Default::default()
                };
                job.append(&line[pos..pos+1], 0.0, default_format);
                pos += 1;
            }
            
            let format = if keywords.contains(&word) {
                eframe::egui::TextFormat {
                    color: theme.keyword,
                    ..Default::default()
                }
            } else if types.contains(&word) {
                eframe::egui::TextFormat {
                    color: theme.type_name,
                    ..Default::default()
                }
            } else if word.starts_with("//") {
                eframe::egui::TextFormat {
                    color: theme.comment,
                    ..Default::default()
                }
            } else if word.starts_with('"') && word.ends_with('"') {
                eframe::egui::TextFormat {
                    color: theme.string,
                    ..Default::default()
                }
            } else if word.chars().all(|c| c.is_numeric() || c == '.') {
                eframe::egui::TextFormat {
                    color: theme.number,
                    ..Default::default()
                }
            } else {
                eframe::egui::TextFormat {
                    color: theme.text,
                    ..Default::default()
                }
            };
            
            job.append(word, 0.0, format);
            pos += word.len();
        }
        
        // Add remaining characters
        if pos < line.len() {
            let default_format = eframe::egui::TextFormat {
                color: theme.text,
                ..Default::default()
            };
            job.append(&line[pos..], 0.0, default_format);
        }
    }
    
    /// Render status bar
    fn render_status_bar(&self, ui: &mut eframe::egui::Ui) {
        ui.separator();
        ui.horizontal(|ui| {
            ui.label(format!("Lines: {}", self.code.lines().count()));
            ui.separator();
            ui.label(format!("Characters: {}", self.code.len()));
            ui.separator();
            ui.label(format!("Cursor: {}:{}", self.cursor_pos.0, self.cursor_pos.1));
            ui.separator();
            
            // Show dirty indicator
            if self.is_dirty {
                ui.colored_label(eframe::egui::Color32::YELLOW, "●");
            } else {
                ui.colored_label(eframe::egui::Color32::GREEN, "●");
            }
            
            // Show diagnostics count
            if !self.diagnostics.diagnostics.is_empty() {
                let error_count = self.diagnostics.diagnostics.iter()
                    .filter(|d| matches!(d.severity, Some(crate::editor::lsp_integration::DiagnosticSeverity::Error)))
                    .count();
                let warning_count = self.diagnostics.diagnostics.iter()
                    .filter(|d| matches!(d.severity, Some(crate::editor::lsp_integration::DiagnosticSeverity::Warning)))
                    .count();
                    
                if error_count > 0 {
                    ui.colored_label(eframe::egui::Color32::RED, format!("❌ {}", error_count));
                }
                if warning_count > 0 {
                    ui.colored_label(eframe::egui::Color32::YELLOW, format!("⚠️ {}", warning_count));
                }
            }
        });
    }

    /// Render autocomplete popup overlay
    fn render_autocomplete_popup(&mut self, ui: &mut eframe::egui::Ui) {
        if self.autocomplete.items.is_empty() {
            return;
        }

        // Calculate popup position (simplified)
        let popup_pos = ui.next_widget_position() + eframe::egui::Vec2::new(200.0, 100.0);
        
        eframe::egui::Area::new("autocomplete_popup".into())
            .fixed_pos(popup_pos)
            .order(eframe::egui::Order::Foreground)
            .show(ui.ctx(), |ui| {
                eframe::egui::Frame::popup(ui.style())
                    .inner_margin(4.0)
                    .show(ui, |ui| {
                        ui.set_max_width(300.0);
                        ui.set_max_height(200.0);
                        
                        ui.heading("🔧 Autocomplete");
                        ui.separator();
                        
                        eframe::egui::ScrollArea::vertical()
                            .max_height(150.0)
                            .show(ui, |ui| {
                                for (i, item) in self.autocomplete.items.iter().enumerate() {
                                    let is_selected = i == self.autocomplete.selected_index;
                                    
                                    let color = if is_selected {
                                        ui.visuals().selection.bg_fill
                                    } else {
                                        eframe::egui::Color32::TRANSPARENT
                                    };
                                    
                                    let frame = eframe::egui::Frame::none()
                                        .fill(color)
                                        .inner_margin(2.0);
                                        
                                    let response = frame.show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            // Kind icon
                                            let icon = match &item.kind {
                                                Some(crate::editor::lsp_integration::CompletionItemKind::Function) => "🔧",
                                                Some(crate::editor::lsp_integration::CompletionItemKind::Variable) => "📦",
                                                Some(crate::editor::lsp_integration::CompletionItemKind::Class) => "🏗️",
                                                Some(crate::editor::lsp_integration::CompletionItemKind::Method) => "⚙️",
                                                Some(crate::editor::lsp_integration::CompletionItemKind::Keyword) => "🔑",
                                                _ => "📄",
                                            };
                                            ui.label(icon);
                                            
                                            ui.vertical(|ui| {
                                                ui.label(&item.label);
                                                if let Some(detail) = &item.detail {
                                                    ui.small(detail);
                                                }
                                            });
                                        });
                                    });
                                    
                                    if response.response.clicked() {
                                        self.autocomplete.selected_index = i;
                                        // Accept completion on click
                                        let item = self.autocomplete.items[i].clone();
                                        if let Some(insert_text) = &item.insert_text {
                                            self.insert_text_at_cursor(insert_text);
                                        } else {
                                            self.insert_text_at_cursor(&item.label);
                                        }
                                        self.hide_autocomplete();
                                        self.mark_dirty();
                                    }
                                }
                            });
                        
                        ui.separator();
                        ui.small("Use ↑↓ to navigate, Enter to accept, Esc to cancel");
                    });
            });
    }

    /// Render diagnostics popup overlay
    fn render_diagnostics_popup(&mut self, ui: &mut eframe::egui::Ui) {
        if let Some(diagnostic) = &self.diagnostics.hover_diagnostic.clone() {
            let popup_pos = ui.next_widget_position() + eframe::egui::Vec2::new(250.0, 50.0);
            
            eframe::egui::Area::new("diagnostics_popup".into())
                .fixed_pos(popup_pos)
                .order(eframe::egui::Order::Foreground)
                .show(ui.ctx(), |ui| {
                    eframe::egui::Frame::popup(ui.style())
                        .inner_margin(8.0)
                        .show(ui, |ui| {
                            ui.set_max_width(400.0);
                            
                            let (icon, color) = match diagnostic.severity {
                                Some(crate::editor::lsp_integration::DiagnosticSeverity::Error) => ("❌", eframe::egui::Color32::RED),
                                Some(crate::editor::lsp_integration::DiagnosticSeverity::Warning) => ("⚠️", eframe::egui::Color32::YELLOW),
                                Some(crate::editor::lsp_integration::DiagnosticSeverity::Information) => ("ℹ️", eframe::egui::Color32::BLUE),
                                Some(crate::editor::lsp_integration::DiagnosticSeverity::Hint) => ("💡", eframe::egui::Color32::GRAY),
                                None => ("•", eframe::egui::Color32::WHITE),
                            };
                            
                            ui.horizontal(|ui| {
                                ui.colored_label(color, icon);
                                ui.colored_label(color, "Diagnostic");
                            });
                            ui.separator();
                            
                            ui.label(&diagnostic.message);
                            
                            if let Some(source) = &diagnostic.source {
                                ui.small(format!("Source: {}", source));
                            }
                            
                            if let Some(code) = &diagnostic.code {
                                ui.small(format!("Code: {}", code));
                            }
                        });
                });
        }
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
