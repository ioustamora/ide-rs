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

    /// Render the code editor with advanced features
    pub fn render(&mut self, ui: &mut eframe::egui::Ui) {
        ui.vertical(|ui| {
            // Toolbar
            self.render_toolbar(ui);
            ui.separator();
            
            // Main editor area with line numbers and syntax highlighting
            ui.horizontal(|ui| {
                // Line numbers column
                if self.settings.show_line_numbers {
                    self.render_line_numbers(ui);
                }
                
                // Code editor with syntax highlighting
                eframe::egui::ScrollArea::both()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if self.language == "rust" {
                            self.render_with_syntax_highlighting(ui);
                        } else {
                            // Fallback to simple editor
                            ui.text_edit_multiline(&mut self.code);
                        }
                    });
            });
            
            // Status bar
            self.render_status_bar(ui);
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
            ui.checkbox(&mut self.settings.auto_complete, "Auto Complete");
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
                ui.style_mut().visuals.extreme_bg_color = eframe::egui::Color32::from_gray(245);
                
                for line_num in 1..=line_count {
                    ui.label(format!("{:3}", line_num));
                }
            },
        );
        ui.separator();
    }
    
    /// Render code with basic syntax highlighting for Rust
    fn render_with_syntax_highlighting(&mut self, ui: &mut eframe::egui::Ui) {
        let mut job = eframe::egui::text::LayoutJob::default();
        
        for line in self.code.lines() {
            self.highlight_rust_line(line, &mut job);
            job.append("\n", 0.0, eframe::egui::TextFormat::default());
        }
        
        // Create a text edit that preserves formatting
        ui.add(eframe::egui::TextEdit::multiline(&mut self.code)
            .font(eframe::egui::TextStyle::Monospace)
            .desired_width(f32::INFINITY)
            .desired_rows(20));
    }
    
    /// Simple Rust syntax highlighting
    fn highlight_rust_line(&self, line: &str, job: &mut eframe::egui::text::LayoutJob) {
        let keywords = ["fn", "let", "mut", "if", "else", "for", "while", "loop", "match", "struct", "enum", "impl", "pub", "use", "mod"];
        let types = ["String", "i32", "i64", "f32", "f64", "bool", "char", "Vec", "Option", "Result"];
        
        let words: Vec<&str> = line.split_whitespace().collect();
        let mut pos = 0;
        
        for word in words {
            // Find the actual position in the line
            while pos < line.len() && !line[pos..].starts_with(word) {
                job.append(&line[pos..pos+1], 0.0, eframe::egui::TextFormat::default());
                pos += 1;
            }
            
            let format = if keywords.contains(&word) {
                eframe::egui::TextFormat {
                    color: eframe::egui::Color32::from_rgb(0, 0, 255), // Blue for keywords
                    ..Default::default()
                }
            } else if types.contains(&word) {
                eframe::egui::TextFormat {
                    color: eframe::egui::Color32::from_rgb(43, 145, 175), // Teal for types
                    ..Default::default()
                }
            } else if word.starts_with("//") {
                eframe::egui::TextFormat {
                    color: eframe::egui::Color32::from_rgb(0, 128, 0), // Green for comments
                    ..Default::default()
                }
            } else if word.starts_with('"') && word.ends_with('"') {
                eframe::egui::TextFormat {
                    color: eframe::egui::Color32::from_rgb(163, 21, 21), // Red for strings
                    ..Default::default()
                }
            } else {
                eframe::egui::TextFormat::default()
            };
            
            job.append(word, 0.0, format);
            pos += word.len();
        }
        
        // Add remaining characters
        if pos < line.len() {
            job.append(&line[pos..], 0.0, eframe::egui::TextFormat::default());
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
