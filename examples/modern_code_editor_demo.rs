//! Modern Code Editor Demo
//!
//! This example demonstrates the Phase 1 improvements to the code editor,
//! including VS Code-like features such as:
//! - Advanced syntax highlighting with syntect
//! - Autocomplete popup with keyboard navigation
//! - Real-time diagnostics display
//! - Modern editor settings and themes
//! - Enhanced status bar with file state
//! - Line numbers, minimap, and code folding

use eframe::egui;
use ide_rs::editor::code_editor::CodeEditor;
use ide_rs::editor::lsp_integration::LspClient;
use ide_rs::editor::output_panel::OutputPanel;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Modern Code Editor Demo - Phase 1 Improvements",
        options,
        Box::new(|_cc| Box::new(ModernEditorApp::default())),
    )
}

struct ModernEditorApp {
    code_editor: CodeEditor,
    lsp_client: LspClient,
    output_panel: OutputPanel,
    demo_mode: DemoMode,
}

#[derive(Default, PartialEq)]
enum DemoMode {
    #[default]
    SyntaxHighlighting,
    Autocomplete,
    Diagnostics,
    Themes,
}

impl Default for ModernEditorApp {
    fn default() -> Self {
        let mut code_editor = CodeEditor::with_content("rust", include_str!("demo_code.rs").to_string());
        
        // Add some demo diagnostics
        let demo_diagnostics = vec![
            ide_rs::editor::lsp_integration::Diagnostic {
                range: ide_rs::editor::lsp_integration::Range {
                    start: ide_rs::editor::lsp_integration::Position { line: 5, character: 8 },
                    end: ide_rs::editor::lsp_integration::Position { line: 5, character: 15 },
                },
                severity: Some(ide_rs::editor::lsp_integration::DiagnosticSeverity::Error),
                code: Some("E0308".to_string()),
                source: Some("rustc".to_string()),
                message: "mismatched types: expected `i32`, found `&str`".to_string(),
                related_information: None,
            },
            ide_rs::editor::lsp_integration::Diagnostic {
                range: ide_rs::editor::lsp_integration::Range {
                    start: ide_rs::editor::lsp_integration::Position { line: 10, character: 4 },
                    end: ide_rs::editor::lsp_integration::Position { line: 10, character: 12 },
                },
                severity: Some(ide_rs::editor::lsp_integration::DiagnosticSeverity::Warning),
                code: Some("unused_variable".to_string()),
                source: Some("rustc".to_string()),
                message: "unused variable: `unused_var`".to_string(),
                related_information: None,
            },
        ];
        
        code_editor.update_diagnostics(demo_diagnostics);
        
        Self {
            code_editor,
            lsp_client: LspClient::new(),
            output_panel: OutputPanel::new(),
            demo_mode: DemoMode::default(),
        }
    }
}

impl eframe::App for ModernEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🦀 Modern Code Editor - Phase 1 Improvements");
            ui.separator();
            
            // Demo mode selector
            ui.horizontal(|ui| {
                ui.label("Demo Mode:");
                ui.selectable_value(&mut self.demo_mode, DemoMode::SyntaxHighlighting, "🎨 Syntax Highlighting");
                ui.selectable_value(&mut self.demo_mode, DemoMode::Autocomplete, "🔧 Autocomplete");
                ui.selectable_value(&mut self.demo_mode, DemoMode::Diagnostics, "🔍 Diagnostics");
                ui.selectable_value(&mut self.demo_mode, DemoMode::Themes, "🎭 Themes");
            });
            ui.separator();
            
            // Show features for current demo mode
            match self.demo_mode {
                DemoMode::SyntaxHighlighting => {
                    ui.label("✨ Enhanced syntax highlighting with syntect library");
                    ui.label("• Multi-language support with proper token-level highlighting");
                    ui.label("• Theme-aware colors with dark/light/monokai themes");
                    ui.label("• Real-time highlighting as you type");
                }
                DemoMode::Autocomplete => {
                    ui.label("🔧 VS Code-like autocomplete popup");
                    ui.label("• Press Ctrl+Space to trigger autocomplete");
                    ui.label("• Use ↑↓ arrow keys to navigate suggestions");
                    ui.label("• Press Enter to accept, Esc to cancel");
                    ui.label("• Shows function signatures and documentation");
                }
                DemoMode::Diagnostics => {
                    ui.label("🔍 Real-time diagnostics from LSP");
                    ui.label("• Inline error and warning indicators");
                    ui.label("• Hover to see detailed error messages");
                    ui.label("• Click 🎯 to jump to error location");
                    ui.label("• Status bar shows error/warning counts");
                }
                DemoMode::Themes => {
                    ui.label("🎭 Professional editor themes");
                    ui.label("• Dark, Light, and Monokai themes");
                    ui.label("• Consistent color schemes across all features");
                    ui.label("• Current line highlighting");
                    ui.label("• Customizable editor appearance");
                }
            }
            ui.separator();
            
            // Render the enhanced code editor
            self.code_editor.render_enhanced(ui, &mut self.lsp_client, &mut self.output_panel);
            
            // Show demo shortcuts
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("🔧 Trigger Autocomplete").clicked() {
                    // Show demo autocomplete
                    let demo_completions = vec![
                        ide_rs::editor::lsp_integration::CompletionItem {
                            label: "println!".to_string(),
                            kind: Some(ide_rs::editor::lsp_integration::CompletionItemKind::Function),
                            detail: Some("macro".to_string()),
                            documentation: Some("Prints to the standard output, with a newline.".to_string()),
                            sort_text: None,
                            filter_text: None,
                            insert_text: Some("println!(\"{}\", )".to_string()),
                            insert_text_format: None,
                        },
                        ide_rs::editor::lsp_integration::CompletionItem {
                            label: "String::new".to_string(),
                            kind: Some(ide_rs::editor::lsp_integration::CompletionItemKind::Function),
                            detail: Some("fn() -> String".to_string()),
                            documentation: Some("Creates a new empty String.".to_string()),
                            sort_text: None,
                            filter_text: None,
                            insert_text: Some("String::new()".to_string()),
                            insert_text_format: None,
                        },
                        ide_rs::editor::lsp_integration::CompletionItem {
                            label: "Vec::new".to_string(),
                            kind: Some(ide_rs::editor::lsp_integration::CompletionItemKind::Function),
                            detail: Some("fn() -> Vec<T>".to_string()),
                            documentation: Some("Constructs a new, empty Vec<T>.".to_string()),
                            sort_text: None,
                            filter_text: None,
                            insert_text: Some("Vec::new()".to_string()),
                            insert_text_format: None,
                        },
                    ];
                    self.code_editor.show_autocomplete(demo_completions);
                }
                
                if ui.button("💡 Toggle Diagnostics").clicked() {
                    self.code_editor.diagnostics.show_error_popup = !self.code_editor.diagnostics.show_error_popup;
                }
                
                if ui.button("🎨 Cycle Theme").clicked() {
                    match self.code_editor.settings.current_theme.name.as_str() {
                        "Dark" => self.code_editor.settings.current_theme = ide_rs::editor::code_editor::EditorTheme::light_theme(),
                        "Light" => self.code_editor.settings.current_theme = ide_rs::editor::code_editor::EditorTheme::monokai_theme(),
                        _ => self.code_editor.settings.current_theme = ide_rs::editor::code_editor::EditorTheme::dark_theme(),
                    }
                }
            });
        });
        
        // Handle keyboard shortcuts
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Space) && i.modifiers.ctrl {
                // Trigger autocomplete with Ctrl+Space
                let demo_completions = vec![
                    ide_rs::editor::lsp_integration::CompletionItem {
                        label: "match".to_string(),
                        kind: Some(ide_rs::editor::lsp_integration::CompletionItemKind::Keyword),
                        detail: Some("keyword".to_string()),
                        documentation: Some("Pattern matching with match expressions".to_string()),
                        sort_text: None,
                        filter_text: None,
                        insert_text: Some("match value {\n    pattern => result,\n}".to_string()),
                        insert_text_format: None,
                    },
                ];
                self.code_editor.show_autocomplete(demo_completions);
            }
            
            if self.code_editor.autocomplete.visible {
                if i.key_pressed(egui::Key::ArrowUp) {
                    self.code_editor.autocomplete_previous();
                }
                if i.key_pressed(egui::Key::ArrowDown) {
                    self.code_editor.autocomplete_next();
                }
                if i.key_pressed(egui::Key::Enter) {
                    self.code_editor.autocomplete_accept();
                }
                if i.key_pressed(egui::Key::Escape) {
                    self.code_editor.hide_autocomplete();
                }
            }
        });
    }
}
