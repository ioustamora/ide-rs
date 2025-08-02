//! Modern IDE Integration - Legacy Compatibility Layer
//!
//! This module provides backward compatibility for the old monolithic structure.
//! New code should use the modular structure in the `modern_ide_integration` submodule.

// Re-export the new modular structure for backward compatibility
pub use crate::editor::modern_ide_integration_modules::*;

// The modular structure is already declared in editor/mod.rs

use egui::*;

/// Main Modern IDE Integration system that orchestrates all subsystems
#[derive(Default)]
pub struct ModernIdeIntegration {
    /// Design token system
    pub design_tokens: DesignTokenSystem,
    /// Component library manager
    pub component_library: ComponentLibrary,
    /// Framework export manager
    pub framework_export: FrameworkExportManager,
    /// Theme system
    pub theme_system: ThemeSystem,
    /// Code generation engine
    pub code_generator: CodeGenerator,
    /// Current active tab
    active_tab: ModernIdeTab,
}

/// Tabs for the modern IDE integration UI
#[derive(Default, Clone, Debug, PartialEq)]
enum ModernIdeTab {
    #[default]
    DesignTokens,
    ComponentLibrary,
    ThemeSystem,
    FrameworkExport,
    CodeGeneration,
}

impl ModernIdeIntegration {
    /// Create a new modern IDE integration system
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Apply current theme to egui context
    pub fn apply_theme(&self, ctx: &Context) {
        self.theme_system.apply_to_egui(ctx);
    }
    
    /// Render the integration panel UI
    pub fn render_integration_panel(&mut self, ui: &mut Ui) {
        self.render_ui(ui);
    }
    
    /// Render the modern IDE integration UI
    pub fn render_ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            // Tab bar
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.active_tab, ModernIdeTab::DesignTokens, "🎨 Design Tokens");
                ui.selectable_value(&mut self.active_tab, ModernIdeTab::ComponentLibrary, "📚 Library");
                ui.selectable_value(&mut self.active_tab, ModernIdeTab::ThemeSystem, "🌓 Themes");
                ui.selectable_value(&mut self.active_tab, ModernIdeTab::FrameworkExport, "📤 Export");
                ui.selectable_value(&mut self.active_tab, ModernIdeTab::CodeGeneration, "⚡ Generate");
            });
            
            ui.separator();
            
            // Tab content
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    match self.active_tab {
                        ModernIdeTab::DesignTokens => {
                            self.render_design_tokens_tab(ui);
                        }
                        ModernIdeTab::ComponentLibrary => {
                            self.component_library.render_ui(ui);
                        }
                        ModernIdeTab::ThemeSystem => {
                            self.theme_system.render_ui(ui);
                        }
                        ModernIdeTab::FrameworkExport => {
                            self.framework_export.render_ui(ui);
                        }
                        ModernIdeTab::CodeGeneration => {
                            self.code_generator.render_ui(ui);
                        }
                    }
                });
        });
    }
    
    /// Render design tokens tab
    fn render_design_tokens_tab(&mut self, ui: &mut Ui) {
        ui.heading("Design Tokens");
        
        ui.collapsing("Color Tokens", |ui| {
            for (name, token) in &self.design_tokens.colors {
                ui.horizontal(|ui| {
                    ui.label(name);
                    
                    // Color preview
                    let color = Color32::from_rgba_premultiplied(
                        token.value[0],
                        token.value[1],
                        token.value[2],
                        token.value[3],
                    );
                    let rect = ui.allocate_space(Vec2::new(40.0, 20.0)).1;
                    ui.painter().rect_filled(rect, 2.0, color);
                    
                    if ui.small_button("Edit").clicked() {
                        // Handle color token editing
                    }
                });
            }
            
            if ui.button("+ Add Color Token").clicked() {
                // Handle adding new color token
            }
        });
        
        ui.collapsing("Typography Tokens", |ui| {
            for (name, token) in &self.design_tokens.typography {
                ui.horizontal(|ui| {
                    ui.label(name);
                    ui.label(&token.font_family);
                    ui.label(format!("{}px", token.font_size));
                    
                    if ui.small_button("Edit").clicked() {
                        // Handle typography token editing
                    }
                });
            }
            
            if ui.button("+ Add Typography Token").clicked() {
                // Handle adding new typography token
            }
        });
        
        ui.separator();
        
        ui.horizontal(|ui| {
            if ui.button("Import Tokens").clicked() {
                // Handle token import
            }
            
            if ui.button("Export Tokens").clicked() {
                // Handle token export
                if let Ok(json) = self.design_tokens.export_json() {
                    println!("Exported tokens: {}", json);
                }
            }
        });
    }
}