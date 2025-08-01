//! Rich Text Editor component for advanced text editing in RCL
//!
//! This component provides rich text editing capabilities with formatting options,
//! syntax highlighting, and advanced text manipulation features.

use egui::{Ui, RichText, Color32, FontId};
use crate::rcl::ui::component::Component;

/// Advanced rich text editor with formatting support
/// 
/// Features:
/// - Multi-line text editing with word wrap
/// - Basic formatting (bold, italic, underlined text)
/// - Font size and color customization
/// - Text statistics (word count, character count)
/// - Find and replace functionality
pub struct RichTextEditor {
    /// The main text content
    pub content: String,
    /// Whether the editor is in edit mode
    pub editable: bool,
    /// Current font size
    pub font_size: f32,
    /// Text color
    pub text_color: Color32,
    /// Whether bold formatting is enabled
    pub bold: bool,
    /// Whether italic formatting is enabled
    pub italic: bool,
    /// Search query for find functionality
    pub search_query: String,
    /// Replace text for find-and-replace
    pub replace_text: String,
    /// Show formatting toolbar
    pub show_toolbar: bool,
    /// Line numbers visibility
    pub show_line_numbers: bool,
    /// Word wrap setting
    pub word_wrap: bool,
}

impl Default for RichTextEditor {
    fn default() -> Self {
        Self {
            content: "# Welcome to Rich Text Editor\n\nStart typing your content here...\n\n**Bold text** and *italic text* are supported.\n\nYou can also use `code blocks` and other formatting.".to_string(),
            editable: false,
            font_size: 14.0,
            text_color: Color32::WHITE,
            bold: false,
            italic: false,
            search_query: String::new(),
            replace_text: String::new(),
            show_toolbar: true,
            show_line_numbers: false,
            word_wrap: true,
        }
    }
}

impl Component for RichTextEditor {
    fn name(&self) -> &str {
        "RichTextEditor"
    }
    
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            // Edit mode - show full editor interface
            if self.show_toolbar {
                self.render_toolbar(ui);
                ui.separator();
            }
            
            // Main text editing area
            ui.horizontal_top(|ui| {
                // Line numbers (if enabled)
                if self.show_line_numbers {
                    let line_count = self.content.lines().count();
                    ui.vertical(|ui| {
                        ui.set_width(40.0);
                        for i in 1..=line_count.max(10) {
                            ui.label(format!("{:3}", i));
                        }
                    });
                    ui.separator();
                }
                
                // Text editor
                let mut text_edit = egui::TextEdit::multiline(&mut self.content)
                    .font(FontId::monospace(self.font_size))
                    .desired_width(f32::INFINITY)
                    .desired_rows(15);
                
                if !self.word_wrap {
                    text_edit = text_edit.code_editor();
                }
                
                ui.add(text_edit);
            });
            
            ui.separator();
            
            // Search and replace panel
            self.render_search_replace_panel(ui);
            
            ui.separator();
            
            // Statistics
            self.render_statistics(ui);
            
        } else {
            // Display mode - show formatted text
            ui.heading("Rich Text Preview");
            ui.separator();
            
            // Render content with basic markdown-like formatting
            self.render_formatted_text(ui);
            
            ui.separator();
            ui.label(format!("Characters: {} | Words: {} | Lines: {}", 
                           self.content.len(),
                           self.count_words(),
                           self.content.lines().count()));
        }
        
        // Edit toggle button
        if ui.button(if self.editable { "Preview" } else { "Edit" }).clicked() {
            self.editable = !self.editable;
        }
    }
}

impl RichTextEditor {
    /// Render the formatting toolbar
    fn render_toolbar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Format:");
            
            // Font size controls
            if ui.button("A+").on_hover_text("Increase font size").clicked() {
                self.font_size = (self.font_size + 1.0).min(24.0);
            }
            if ui.button("A-").on_hover_text("Decrease font size").clicked() {
                self.font_size = (self.font_size - 1.0).max(8.0);
            }
            
            ui.separator();
            
            // Text formatting toggles
            ui.toggle_value(&mut self.bold, "B").on_hover_text("Bold");
            ui.toggle_value(&mut self.italic, "I").on_hover_text("Italic");
            
            ui.separator();
            
            // View options
            ui.toggle_value(&mut self.show_line_numbers, "#").on_hover_text("Line numbers");
            ui.toggle_value(&mut self.word_wrap, "↵").on_hover_text("Word wrap");
            
            ui.separator();
            
            ui.label(format!("Font: {:.0}px", self.font_size));
        });
    }
    
    /// Render search and replace panel
    fn render_search_replace_panel(&mut self, ui: &mut Ui) {
        ui.collapsing("🔍 Find & Replace", |ui| {
            ui.horizontal(|ui| {
                ui.label("Find:");
                ui.text_edit_singleline(&mut self.search_query);
                if ui.button("Find Next").clicked() {
                    // TODO: Implement find functionality
                }
            });
            
            ui.horizontal(|ui| {
                ui.label("Replace:");
                ui.text_edit_singleline(&mut self.replace_text);
                if ui.button("Replace").clicked() {
                    self.replace_text_in_content();
                }
                if ui.button("Replace All").clicked() {
                    self.replace_all_text_in_content();
                }
            });
        });
    }
    
    /// Render statistics panel
    fn render_statistics(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("📊 Stats:");
            ui.label(format!("Characters: {}", self.content.len()));
            ui.separator();
            ui.label(format!("Words: {}", self.count_words()));
            ui.separator();
            ui.label(format!("Lines: {}", self.content.lines().count()));
            ui.separator();
            ui.label(format!("Paragraphs: {}", self.count_paragraphs()));
        });
    }
    
    /// Render formatted text (basic markdown-like rendering)
    fn render_formatted_text(&self, ui: &mut Ui) {
        let lines: Vec<&str> = self.content.lines().collect();
        
        for line in lines {
            if line.starts_with("# ") {
                // Heading
                ui.heading(line.strip_prefix("# ").unwrap_or(line));
            } else if line.starts_with("## ") {
                // Subheading
                ui.label(RichText::new(line.strip_prefix("## ").unwrap_or(line))
                         .size(16.0)
                         .strong());
            } else if line.starts_with("**") && line.ends_with("**") && line.len() > 4 {
                // Bold text
                let text = line.strip_prefix("**").unwrap().strip_suffix("**").unwrap();
                ui.label(RichText::new(text).strong());
            } else if line.starts_with("*") && line.ends_with("*") && line.len() > 2 {
                // Italic text
                let text = line.strip_prefix("*").unwrap().strip_suffix("*").unwrap();
                ui.label(RichText::new(text).italics());
            } else if line.starts_with("`") && line.ends_with("`") && line.len() > 2 {
                // Code text
                let text = line.strip_prefix("`").unwrap().strip_suffix("`").unwrap();
                ui.code(text);
            } else if !line.trim().is_empty() {
                // Regular text
                ui.label(line);
            } else {
                // Empty line - add spacing
                ui.add_space(4.0);
            }
        }
    }
    
    /// Count words in the content
    fn count_words(&self) -> usize {
        self.content
            .split_whitespace()
            .filter(|word| !word.is_empty())
            .count()
    }
    
    /// Count paragraphs in the content
    fn count_paragraphs(&self) -> usize {
        self.content
            .split("\n\n")
            .filter(|para| !para.trim().is_empty())
            .count()
    }
    
    /// Replace first occurrence of search query with replace text
    fn replace_text_in_content(&mut self) {
        if !self.search_query.is_empty() {
            if let Some(pos) = self.content.find(&self.search_query) {
                let before = &self.content[..pos];
                let after = &self.content[pos + self.search_query.len()..];
                self.content = format!("{}{}{}", before, self.replace_text, after);
            }
        }
    }
    
    /// Replace all occurrences of search query with replace text
    fn replace_all_text_in_content(&mut self) {
        if !self.search_query.is_empty() {
            self.content = self.content.replace(&self.search_query, &self.replace_text);
        }
    }
}