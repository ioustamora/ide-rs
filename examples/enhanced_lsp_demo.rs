//! Enhanced LSP Integration Demo
//!
//! This example demonstrates the advanced LSP integration with VS Code-style features:
//! - Enhanced go-to-definition and find references
//! - Code actions and quick fixes
//! - Signature help and parameter hints
//! - Real-time diagnostics with error squiggles
//! - Advanced autocomplete with documentation
//! - Symbol navigation and workspace symbols

use eframe::egui;
use ide_rs::editor::advanced_code_editor::AdvancedCodeEditor;
use ide_rs::editor::lsp_integration::LspClient;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1400.0, 900.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Enhanced LSP Integration Demo - Professional IDE Features",
        options,
        Box::new(|_cc| Box::new(EnhancedLspApp::default())),
    )
}

struct EnhancedLspApp {
    advanced_editor: AdvancedCodeEditor,
    lsp_client: LspClient,
    demo_mode: DemoMode,
    status_message: String,
}

#[derive(Default, PartialEq)]
enum DemoMode {
    #[default]
    GoToDefinition,
    FindReferences,
    CodeActions,
    SignatureHelp,
    Diagnostics,
    Autocomplete,
    SymbolNavigation,
}

impl Default for EnhancedLspApp {
    fn default() -> Self {
        let file_uri = "file:///demo.rs".to_string();
        let language = "rust".to_string();
        let demo_content = include_str!("../examples/demo_rust_code.rs").to_string();
        
        let mut advanced_editor = AdvancedCodeEditor::new(
            file_uri,
            language,
            demo_content
        );
        
        // Try to start LSP (may fail if rust-analyzer not available)
        if let Err(e) = advanced_editor.start_lsp() {
            eprintln!("LSP not available: {:?}", e);
        }
        
        Self {
            advanced_editor,
            lsp_client: LspClient::new(),
            demo_mode: DemoMode::default(),
            status_message: "Welcome to Enhanced LSP Demo! Try the features below.".to_string(),
        }
    }
}

impl eframe::App for EnhancedLspApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top panel with demo controls
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🦀 Enhanced LSP Integration Demo");
                ui.separator();
                
                // Demo mode selector
                ui.label("Demo Mode:");
                ui.selectable_value(&mut self.demo_mode, DemoMode::GoToDefinition, "🎯 Go-to-Definition");
                ui.selectable_value(&mut self.demo_mode, DemoMode::FindReferences, "🔍 Find References");
                ui.selectable_value(&mut self.demo_mode, DemoMode::CodeActions, "⚡ Code Actions");
                ui.selectable_value(&mut self.demo_mode, DemoMode::SignatureHelp, "📝 Signature Help");
                ui.selectable_value(&mut self.demo_mode, DemoMode::Diagnostics, "🔴 Diagnostics");
                ui.selectable_value(&mut self.demo_mode, DemoMode::Autocomplete, "💡 Autocomplete");
                ui.selectable_value(&mut self.demo_mode, DemoMode::SymbolNavigation, "🧭 Symbol Navigation");
            });
        });
        
        // Left panel with feature explanation
        egui::SidePanel::left("feature_panel").resizable(true).show(ctx, |ui| {
            ui.set_min_width(300.0);
            self.render_feature_explanation(ui);
        });
        
        // Bottom panel with status and instructions
        egui::TopBottomPanel::bottom("status_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Status:");
                ui.label(&self.status_message);
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // LSP connection status
                    let lsp_status = if self.advanced_editor.enhanced_lsp.is_connected() {
                        "🟢 Enhanced LSP Connected"
                    } else if self.lsp_client.is_connected() {
                        "🟡 Basic LSP Connected"
                    } else {
                        "🔴 LSP Disconnected"
                    };
                    ui.label(lsp_status);
                });
            });
        });
        
        // Central panel with the advanced code editor
        egui::CentralPanel::default().show(ctx, |ui| {
            // Render the advanced code editor
            self.advanced_editor.render(ui, &mut self.lsp_client);
            
            // Handle demo-specific interactions
            self.handle_demo_interactions(ui, ctx);
        });
        
        // Update status message based on current mode
        self.update_status_message();
    }
}

impl EnhancedLspApp {
    /// Render feature explanation panel
    fn render_feature_explanation(&mut self, ui: &mut egui::Ui) {
        ui.heading("Enhanced LSP Features");
        ui.separator();
        
        match self.demo_mode {
            DemoMode::GoToDefinition => {
                ui.strong("🎯 Go-to-Definition");
                ui.add_space(10.0);
                ui.label("Features:");
                ui.label("• Double-click on any symbol to jump to its definition");
                ui.label("• Ctrl+Click for quick navigation");
                ui.label("• Works across files and crates");
                ui.label("• Supports Rust, macros, and trait implementations");
                ui.add_space(10.0);
                ui.label("Try it:");
                ui.label("1. Double-click on 'String' or 'Vec'");
                ui.label("2. Double-click on function names");
                ui.label("3. Try Ctrl+G for go-to-definition");
            }
            DemoMode::FindReferences => {
                ui.strong("🔍 Find References");
                ui.add_space(10.0);
                ui.label("Features:");
                ui.label("• Find all usages of a symbol");
                ui.label("• Shows references across the entire workspace");
                ui.label("• Includes definition and usage sites");
                ui.label("• Smart filtering and grouping");
                ui.add_space(10.0);
                ui.label("Try it:");
                ui.label("1. Right-click on any variable");
                ui.label("2. Select 'Find References'");
                ui.label("3. Use Ctrl+Shift+F for quick search");
            }
            DemoMode::CodeActions => {
                ui.strong("⚡ Code Actions");
                ui.add_space(10.0);
                ui.label("Features:");
                ui.label("• Quick fixes for compiler errors");
                ui.label("• Refactoring suggestions");
                ui.label("• Import suggestions");
                ui.label("• Code generation helpers");
                ui.add_space(10.0);
                ui.label("Try it:");
                ui.label("1. Right-click on red-squiggled code");
                ui.label("2. Look for lightbulb 💡 icons");
                ui.label("3. Use Ctrl+. for quick actions");
            }
            DemoMode::SignatureHelp => {
                ui.strong("📝 Signature Help");
                ui.add_space(10.0);
                ui.label("Features:");
                ui.label("• Function signature tooltips");
                ui.label("• Parameter documentation");
                ui.label("• Active parameter highlighting");
                ui.label("• Overload information");
                ui.add_space(10.0);
                ui.label("Try it:");
                ui.label("1. Type 'println!(' to see signature help");
                ui.label("2. Use arrow keys to navigate parameters");
                ui.label("3. Signature help appears automatically");
            }
            DemoMode::Diagnostics => {
                ui.strong("🔴 Real-time Diagnostics");
                ui.add_space(10.0);
                ui.label("Features:");
                ui.label("• Real-time error detection");
                ui.label("• Warning and hint indicators");
                ui.label("• Error squiggles with hover tooltips");
                ui.label("• Integrated with compiler messages");
                ui.add_space(10.0);
                ui.label("Try it:");
                ui.label("1. Introduce a syntax error");
                ui.label("2. Hover over red squiggles");
                ui.label("3. See live error updates as you type");
            }
            DemoMode::Autocomplete => {
                ui.strong("💡 Advanced Autocomplete");
                ui.add_space(10.0);
                ui.label("Features:");
                ui.label("• Context-aware suggestions");
                ui.label("• Documentation previews");
                ui.label("• Fuzzy matching and ranking");
                ui.label("• Snippet insertion");
                ui.add_space(10.0);
                ui.label("Try it:");
                ui.label("1. Type 'std::' to see module suggestions");
                ui.label("2. Press Ctrl+Space for manual trigger");
                ui.label("3. Use arrow keys and Enter to accept");
            }
            DemoMode::SymbolNavigation => {
                ui.strong("🧭 Symbol Navigation");
                ui.add_space(10.0);
                ui.label("Features:");
                ui.label("• Document outline with symbols");
                ui.label("• Workspace-wide symbol search");
                ui.label("• Symbol hierarchy and organization");
                ui.label("• Quick symbol jumping");
                ui.add_space(10.0);
                ui.label("Try it:");
                ui.label("1. Use Ctrl+Shift+O for symbol search");
                ui.label("2. Browse document symbols");
                ui.label("3. Search across workspace");
            }
        }
        
        ui.add_space(20.0);
        ui.separator();
        ui.strong("Keyboard Shortcuts:");
        ui.label("• Ctrl+Space - Trigger autocomplete");
        ui.label("• Ctrl+G - Go to definition");
        ui.label("• Ctrl+Shift+F - Find references");
        ui.label("• Ctrl+. - Code actions");
        ui.label("• F12 - Go to definition");
        ui.label("• Shift+F12 - Find references");
    }
    
    /// Handle demo-specific interactions
    fn handle_demo_interactions(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ctx.input(|i| {
            // Handle keyboard shortcuts specific to demo modes
            match self.demo_mode {
                DemoMode::GoToDefinition => {
                    if i.key_pressed(egui::Key::F12) || (i.modifiers.ctrl && i.key_pressed(egui::Key::G)) {
                        self.status_message = "Go-to-definition triggered! (Demo mode)".to_string();
                    }
                }
                DemoMode::FindReferences => {
                    if i.key_pressed(egui::Key::F12) && i.modifiers.shift {
                        self.status_message = "Find references triggered! (Demo mode)".to_string();
                    }
                }
                DemoMode::CodeActions => {
                    if i.modifiers.ctrl && i.key_pressed(egui::Key::Period) {
                        self.status_message = "Code actions menu opened! (Demo mode)".to_string();
                    }
                }
                DemoMode::Autocomplete => {
                    if i.modifiers.ctrl && i.key_pressed(egui::Key::Space) {
                        self.status_message = "Autocomplete triggered! (Demo mode)".to_string();
                    }
                }
                _ => {}
            }
        });
    }
    
    /// Update status message based on current mode
    fn update_status_message(&mut self) {
        // Update status periodically or based on editor state
        // This would typically be called when LSP operations complete
    }
}