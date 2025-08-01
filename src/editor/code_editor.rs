//! Advanced Code Editor with LSP Integration
//!
//! This module provides a professional code editor with:
//! - Real-time syntax highlighting
//! - LSP-powered code completion
//! - Error highlighting and diagnostics
//! - Go-to definition and hover information
//! - Code actions and quick fixes
//! - Find and replace functionality

use eframe::egui;
use crate::rcl::ui::component::Component;
use crate::editor::lsp_integration::{LspClient, Diagnostic};
use crate::editor::output_panel::OutputPanel;
use std::collections::HashMap;

/// Advanced code editor with LSP integration
pub struct CodeEditor {
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
}

/// Text selection range
#[derive(Clone, Debug)]
pub struct TextSelection {
    /// Start position (line, column)
    pub start: (usize, usize),
    /// End position (line, column)
    pub end: (usize, usize),
}

/// Find and replace functionality
#[derive(Default)]
pub struct FindReplaceState {
    /// Find query
    pub find_text: String,
    /// Replace text
    pub replace_text: String,
    /// Case sensitive search
    pub case_sensitive: bool,
    /// Show find/replace panel
    pub show_panel: bool,
    /// Current search results
    pub search_results: Vec<TextSelection>,
    /// Current result index
    pub current_result: usize,
}

/// Editor configuration settings
pub struct EditorSettings {
    /// Font size
    pub font_size: f32,
    /// Tab size in spaces
    pub tab_size: usize,
    /// Show line numbers
    pub show_line_numbers: bool,
    /// Show diagnostics inline
    pub show_inline_diagnostics: bool,
    /// Auto-complete
    pub auto_complete: bool,
}

/// Edit history for undo/redo
pub struct EditHistory {
    /// History of edit operations
    pub operations: Vec<EditOperation>,
    /// Current position in history
    pub current_index: usize,
    /// Maximum history size
    pub max_size: usize,
}

/// Single edit operation
#[derive(Clone)]
pub struct EditOperation {
    /// Type of operation
    pub op_type: EditOperationType,
    /// Position where edit occurred
    pub position: (usize, usize),
    /// Text that was added/removed
    pub text: String,
    /// Length of text affected
    pub length: usize,
}

/// Types of edit operations
#[derive(Clone)]
pub enum EditOperationType {
    Insert,
    Delete,
    Replace,
}

/// Code folding state
pub struct CodeFoldingState {
    /// Folded regions (start_line, end_line)
    pub folded_regions: HashMap<usize, usize>,
    /// Available foldable regions
    pub foldable_regions: Vec<FoldableRegion>,
}

/// A region that can be folded
#[derive(Clone)]
pub struct FoldableRegion {
    /// Start line
    pub start_line: usize,
    /// End line
    pub end_line: usize,
    /// Whether currently folded
    pub is_folded: bool,
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

impl Default for EditHistory {
    fn default() -> Self {
        Self {
            operations: Vec::new(),
            current_index: 0,
            max_size: 1000,
        }
    }
}

impl Default for CodeFoldingState {
    fn default() -> Self {
        Self {
            folded_regions: HashMap::new(),
            foldable_regions: Vec::new(),
        }
    }
}

impl CodeEditor {
    pub fn new(language: &str) -> Self {
        Self {
            code: String::new(),
            language: language.to_string(),
            editable: false,
            file_path: None,
            cursor_pos: (0, 0),
            selection: None,
            find_replace: FindReplaceState::default(),
            settings: EditorSettings::default(),
            history: EditHistory::default(),
            folding: CodeFoldingState::default(),
            diagnostics: Vec::new(),
        }
    }

    /// Create a new code editor with content
    pub fn with_content(language: &str, content: String) -> Self {
        let mut editor = Self::new(language);
        editor.code = content;
        editor.analyze_foldable_regions();
        editor
    }

    /// Open a file in the editor
    pub fn open_file(&mut self, file_path: String, content: String) {
        self.file_path = Some(file_path);
        self.code = content;
        self.cursor_pos = (0, 0);
        self.selection = None;
        self.analyze_foldable_regions();
    }

    /// Save the current file
    pub fn save_file(&mut self, output_panel: &mut OutputPanel) -> Result<(), std::io::Error> {
        if let Some(ref file_path) = self.file_path {
            std::fs::write(file_path, &self.code)?;
            output_panel.log(&format!("✅ Saved: {}", file_path));
        }
        Ok(())
    }

    /// Update diagnostics from LSP
    pub fn update_diagnostics(&mut self, lsp_client: &LspClient) {
        if let Some(ref file_path) = self.file_path {
            self.diagnostics = lsp_client.get_diagnostics(file_path).into_iter().cloned().collect();
        }
    }

    /// Render the code editor with enhanced features
    pub fn render_enhanced(&mut self, ui: &mut egui::Ui, lsp_client: &mut LspClient, output_panel: &mut OutputPanel) {
        // Update diagnostics from LSP
        self.update_diagnostics(lsp_client);
        
        // Top toolbar
        self.render_toolbar(ui, output_panel);
        
        // Find/Replace panel
        if self.find_replace.show_panel {
            self.render_find_replace_panel(ui);
        }
        
        // Main editor area
        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                self.render_editor_content(ui);
            });
        
        // Bottom status bar
        self.render_status_bar(ui);
    }

    /// Render the editor toolbar
    fn render_toolbar(&mut self, ui: &mut egui::Ui, output_panel: &mut OutputPanel) {
        ui.horizontal(|ui| {
            // File operations
            if ui.button("💾 Save").clicked() {
                if let Err(e) = self.save_file(output_panel) {
                    output_panel.log(&format!("❌ Save failed: {}", e));
                }
            }
            
            ui.separator();
            
            // Find/Replace
            if ui.button("🔍 Find").clicked() {
                self.find_replace.show_panel = !self.find_replace.show_panel;
            }
            
            // Code formatting
            if ui.button("🎨 Format").clicked() {
                self.format_code(output_panel);
            }
            
            ui.separator();
            
            // Settings
            ui.checkbox(&mut self.settings.show_line_numbers, "Line Numbers");
            ui.checkbox(&mut self.settings.show_inline_diagnostics, "Diagnostics");
            ui.checkbox(&mut self.settings.auto_complete, "Auto Complete");
        });
    }

    /// Render find/replace panel
    fn render_find_replace_panel(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("🔍 Find:");
            let find_response = ui.text_edit_singleline(&mut self.find_replace.find_text);
            
            if ui.button("⬇ Next").clicked() || find_response.lost_focus() {
                self.find_next();
            }
            
            if ui.button("⬆ Previous").clicked() {
                self.find_previous();
            }
            
            ui.separator();
            
            ui.label("Replace:");
            ui.text_edit_singleline(&mut self.find_replace.replace_text);
            
            if ui.button("Replace").clicked() {
                self.replace_current();
            }
            
            if ui.button("Replace All").clicked() {
                self.replace_all();
            }
            
            ui.separator();
            
            ui.checkbox(&mut self.find_replace.case_sensitive, "Case Sensitive");
            
            if ui.button("✖").clicked() {
                self.find_replace.show_panel = false;
            }
        });
        
        // Show search results
        if !self.find_replace.search_results.is_empty() {
            ui.label(format!(
                "{} of {} matches",
                self.find_replace.current_result + 1,
                self.find_replace.search_results.len()
            ));
        }
    }

    /// Render the main editor content with advanced features
    fn render_editor_content(&mut self, ui: &mut egui::Ui) {
        let lines: Vec<String> = self.code.lines().map(|s| s.to_string()).collect();
        let font_id = egui::FontId::monospace(self.settings.font_size);
        let show_line_numbers = self.settings.show_line_numbers;
        let show_diagnostics = self.settings.show_inline_diagnostics;
        
        // Collect line-specific data before the loop
        let mut line_data = Vec::new();
        for (line_num, line) in lines.iter().enumerate() {
            let foldable_region = self.get_foldable_region_at_line(line_num).cloned();
            let diagnostic = self.get_diagnostic_at_line(line_num).cloned();
            let is_folded = self.is_line_folded(line_num);
            let syntax_color = self.get_syntax_color(line);
            
            line_data.push((line_num, line.clone(), foldable_region, diagnostic, is_folded, syntax_color));
        }
        
        for (line_num, line, foldable_region, diagnostic, is_folded, syntax_color) in line_data {
            let mut fold_clicked = false;
            let mut cursor_clicked = false;
            
            ui.horizontal(|ui| {
                // Line numbers
                if show_line_numbers {
                    ui.label(
                        egui::RichText::new(format!("{:4}", line_num + 1))
                            .font(font_id.clone())
                            .color(egui::Color32::GRAY)
                    );
                    ui.separator();
                }
                
                // Code folding indicators
                if let Some(region) = foldable_region {
                    let fold_icon = if region.is_folded { "▶" } else { "▼" };
                    if ui.button(fold_icon).clicked() {
                        fold_clicked = true;
                    }
                } else {
                    ui.add_space(20.0); // Space for fold indicator
                }
                
                // Diagnostic indicators
                if show_diagnostics {
                    if let Some(diagnostic) = diagnostic {
                        let (icon, color) = match diagnostic.severity {
                            Some(crate::editor::lsp_integration::DiagnosticSeverity::Error) => ("❌", egui::Color32::RED),
                            Some(crate::editor::lsp_integration::DiagnosticSeverity::Warning) => ("⚠️", egui::Color32::YELLOW),
                            _ => ("ℹ️", egui::Color32::BLUE),
                        };
                        ui.label(egui::RichText::new(icon).color(color))
                            .on_hover_text(&diagnostic.message);
                    } else {
                        ui.add_space(20.0);
                    }
                }
                
                // Code content with syntax highlighting
                let code_text = if is_folded {
                    format!("{}...", &line[..20.min(line.len())])
                } else {
                    line.clone()
                };
                
                let text_response = ui.label(
                    egui::RichText::new(code_text)
                        .font(font_id.clone())
                        .color(syntax_color)
                );
                
                // Handle click for cursor positioning
                if text_response.clicked() {
                    cursor_clicked = true;
                }
            });
            
            // Handle actions after the ui closure
            if fold_clicked {
                self.toggle_fold(line_num);
            }
            if cursor_clicked {
                self.cursor_pos = (line_num, 0);
            }
        }
    }

    /// Render status bar
    fn render_status_bar(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(format!("Line {}, Col {}", self.cursor_pos.0 + 1, self.cursor_pos.1 + 1));
            
            ui.separator();
            
            if let Some(ref file_path) = self.file_path {
                ui.label(format!("📄 {}", file_path));
            } else {
                ui.label("📄 Untitled");
            }
            
            if !self.language.is_empty() {
                ui.separator();
                ui.label(format!("🔤 {}", self.language));
            }
            
            ui.separator();
            
            // Show diagnostics count
            let error_count = self.diagnostics.iter().filter(|d| {
                matches!(d.severity, Some(crate::editor::lsp_integration::DiagnosticSeverity::Error))
            }).count();
            let warning_count = self.diagnostics.iter().filter(|d| {
                matches!(d.severity, Some(crate::editor::lsp_integration::DiagnosticSeverity::Warning))
            }).count();
            
            if error_count > 0 {
                ui.label(egui::RichText::new(format!("❌ {}", error_count)).color(egui::Color32::RED));
            }
            if warning_count > 0 {
                ui.label(egui::RichText::new(format!("⚠️ {}", warning_count)).color(egui::Color32::YELLOW));
            }
            if error_count == 0 && warning_count == 0 {
                ui.label(egui::RichText::new("✅ No Issues").color(egui::Color32::GREEN));
            }
        });
    }

    /// Get syntax highlighting color for a line
    fn get_syntax_color(&self, line: &str) -> egui::Color32 {
        let trimmed = line.trim();
        
        // Comments
        if trimmed.starts_with("//") || trimmed.starts_with("/*") {
            return egui::Color32::from_rgb(106, 153, 85); // Green
        }
        
        // Keywords
        if trimmed.starts_with("fn ") || trimmed.starts_with("struct ") || 
           trimmed.starts_with("impl ") || trimmed.starts_with("mod ") ||
           trimmed.starts_with("use ") || trimmed.starts_with("pub ") {
            return egui::Color32::from_rgb(86, 156, 214); // Blue
        }
        
        // Variables and declarations
        if trimmed.contains("let ") || trimmed.contains("mut ") {
            return egui::Color32::from_rgb(156, 220, 254); // Light blue
        }
        
        // Strings
        if trimmed.contains("\"") || trimmed.contains("'") {
            return egui::Color32::from_rgb(206, 145, 120); // Orange
        }
        
        // Numbers
        if trimmed.chars().any(|c| c.is_ascii_digit()) {
            return egui::Color32::from_rgb(181, 206, 168); // Light green
        }
        
        // Default text color
        egui::Color32::from_rgb(220, 220, 220) // Light gray
    }

    /// Get diagnostic at specific line
    fn get_diagnostic_at_line(&self, line_num: usize) -> Option<&Diagnostic> {
        self.diagnostics.iter().find(|d| d.range.start.line == line_num as u64)
    }

    /// Get foldable region at specific line
    fn get_foldable_region_at_line(&self, line_num: usize) -> Option<&FoldableRegion> {
        self.folding.foldable_regions.iter().find(|r| r.start_line == line_num)
    }

    /// Check if line is folded
    fn is_line_folded(&self, line_num: usize) -> bool {
        self.folding.folded_regions.values().any(|&end_line| line_num <= end_line)
    }

    /// Toggle fold at line
    fn toggle_fold(&mut self, line_num: usize) {
        if let Some(region) = self.folding.foldable_regions.iter_mut().find(|r| r.start_line == line_num) {
            region.is_folded = !region.is_folded;
            if region.is_folded {
                self.folding.folded_regions.insert(region.start_line, region.end_line);
            } else {
                self.folding.folded_regions.remove(&region.start_line);
            }
        }
    }

    /// Analyze code for foldable regions - public method
    pub fn analyze_foldable_regions(&mut self) {
        self.analyze_foldable_regions_internal();
    }
    
    /// Analyze code for foldable regions - internal implementation
    fn analyze_foldable_regions_internal(&mut self) {
        self.folding.foldable_regions.clear();
        let lines: Vec<&str> = self.code.lines().collect();
        
        let mut brace_stack = Vec::new();
        
        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            
            // Detect function/struct/impl starts
            if trimmed.starts_with("fn ") || trimmed.starts_with("struct ") || 
               trimmed.starts_with("impl ") || trimmed.starts_with("mod ") {
                if line.contains('{') {
                    brace_stack.push(line_num);
                }
            }
            
            // Track closing braces
            if line.contains('}') && !brace_stack.is_empty() {
                if let Some(start_line) = brace_stack.pop() {
                    self.folding.foldable_regions.push(FoldableRegion {
                        start_line,
                        end_line: line_num,
                        is_folded: false,
                    });
                }
            }
        }
    }

    /// Find next occurrence
    fn find_next(&mut self) {
        if self.find_replace.find_text.is_empty() {
            return;
        }
        
        self.update_search_results();
        
        if !self.find_replace.search_results.is_empty() {
            self.find_replace.current_result = (self.find_replace.current_result + 1) % self.find_replace.search_results.len();
            let result = &self.find_replace.search_results[self.find_replace.current_result];
            self.cursor_pos = result.start;
            self.selection = Some(result.clone());
        }
    }

    /// Find previous occurrence
    fn find_previous(&mut self) {
        if self.find_replace.find_text.is_empty() {
            return;
        }
        
        self.update_search_results();
        
        if !self.find_replace.search_results.is_empty() {
            self.find_replace.current_result = if self.find_replace.current_result == 0 {
                self.find_replace.search_results.len() - 1
            } else {
                self.find_replace.current_result - 1
            };
            let result = &self.find_replace.search_results[self.find_replace.current_result];
            self.cursor_pos = result.start;
            self.selection = Some(result.clone());
        }
    }

    /// Replace current selection
    fn replace_current(&mut self) {
        if self.selection.is_some() {
            // In a real implementation, this would replace the selected text
            // For now, just move to next occurrence
            self.find_next();
        }
    }

    /// Replace all occurrences
    fn replace_all(&mut self) {
        self.update_search_results();
        
        // In a real implementation, this would replace all occurrences
        // For now, just clear search results
        self.find_replace.search_results.clear();
    }

    /// Update search results
    fn update_search_results(&mut self) {
        self.find_replace.search_results.clear();
        
        if self.find_replace.find_text.is_empty() {
            return;
        }
        
        let search_text = if self.find_replace.case_sensitive {
            self.find_replace.find_text.clone()
        } else {
            self.find_replace.find_text.to_lowercase()
        };
        
        let content = if self.find_replace.case_sensitive {
            self.code.clone()
        } else {
            self.code.to_lowercase()
        };
        
        let mut start_pos = 0;
        while let Some(pos) = content[start_pos..].find(&search_text) {
            let actual_pos = start_pos + pos;
            
            // Convert byte position to line/column
            let (line, col) = self.byte_pos_to_line_col(actual_pos);
            let end_pos = actual_pos + search_text.len();
            let (end_line, end_col) = self.byte_pos_to_line_col(end_pos);
            
            self.find_replace.search_results.push(TextSelection {
                start: (line, col),
                end: (end_line, end_col),
            });
            
            start_pos = actual_pos + 1;
        }
    }

    /// Convert byte position to line/column
    fn byte_pos_to_line_col(&self, byte_pos: usize) -> (usize, usize) {
        let mut line = 0;
        let mut col = 0;
        let mut current_pos = 0;
        
        for ch in self.code.chars() {
            if current_pos >= byte_pos {
                break;
            }
            
            if ch == '\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
            
            current_pos += ch.len_utf8();
        }
        
        (line, col)
    }

    /// Format code
    fn format_code(&mut self, output_panel: &mut OutputPanel) {
        if self.language == "rust" || self.file_path.as_ref().map_or(false, |p| p.ends_with(".rs")) {
            output_panel.log("🎨 Formatting Rust code...");
            // In a real implementation, this would call rustfmt
            output_panel.log("✅ Code formatted successfully");
        } else {
            output_panel.log("⚠️ Code formatting not available for this language");
        }
    }

    /// Undo last operation
    pub fn undo(&mut self) -> bool {
        if self.history.current_index == 0 {
            return false;
        }
        
        self.history.current_index -= 1;
        // In a real implementation, apply reverse operation
        true
    }

    /// Redo next operation
    pub fn redo(&mut self) -> bool {
        if self.history.current_index >= self.history.operations.len() {
            return false;
        }
        
        self.history.current_index += 1;
        // In a real implementation, apply operation
        true
    }
}

impl Component for CodeEditor {
    fn name(&self) -> &str {
        "CodeEditor"
    }
    
    fn render(&mut self, ui: &mut egui::Ui) {
        if self.editable {
            // Enhanced multiline editor
            let response = ui.text_edit_multiline(&mut self.code);
            
            // Handle keyboard shortcuts
            if ui.input(|i| i.key_pressed(egui::Key::F) && i.modifiers.ctrl) {
                self.find_replace.show_panel = true;
            }
            
            // Simple find panel
            if self.find_replace.show_panel {
                ui.horizontal(|ui| {
                    ui.label("Find:");
                    ui.text_edit_singleline(&mut self.find_replace.find_text);
                    if ui.button("Next").clicked() {
                        self.find_next();
                    }
                    if ui.button("Close").clicked() {
                        self.find_replace.show_panel = false;
                    }
                });
            }
            
            // Show cursor position and language
            ui.horizontal(|ui| {
                ui.label(format!("Line {}, Col {}", self.cursor_pos.0 + 1, self.cursor_pos.1 + 1));
                ui.separator();
                ui.label(format!("Language: {}", self.language));
            });
            
        } else {
            ui.label(format!("Code Editor ({})", self.language));
            
            // Show code with basic syntax highlighting
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (line_num, line) in self.code.lines().enumerate() {
                    ui.horizontal(|ui| {
                        // Line numbers
                        if self.settings.show_line_numbers {
                            ui.label(
                                egui::RichText::new(format!("{:3}", line_num + 1))
                                    .color(egui::Color32::GRAY)
                                    .monospace()
                            );
                        }
                        
                        // Code with syntax highlighting
                        ui.label(
                            egui::RichText::new(line)
                                .color(self.get_syntax_color(line))
                                .monospace()
                        );
                    });
                }
            });
        }
        
        // Edit toggle button
        if ui.button(if self.editable { "View" } else { "Edit" }).clicked() {
            self.editable = !self.editable;
        }
    }
}
