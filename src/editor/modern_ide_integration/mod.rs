//! Modern IDE Integration - Modular Architecture
//!
//! This module provides advanced IDE features through a modular architecture:
//! - Design token system for consistent styling
//! - Component library management and reuse
//! - Framework export (React, Vue, Angular, etc.)
//! - Theme management and design system integration
//! - Code generation and template systems

pub mod design_tokens;
pub mod component_library;
pub mod framework_export;
pub mod theme_system;
pub mod code_generation;

// Re-export main types for convenience
pub use design_tokens::{DesignTokenSystem, ColorToken, TypographyToken, SpacingToken, ShadowToken};
pub use component_library::{ComponentLibrary, ComponentTemplate, UserComponent, LibraryMetadata};
pub use framework_export::{FrameworkExportManager, ExportTarget, ExportSettings, GeneratedFile};
pub use theme_system::{ThemeSystem, Theme, ColorPalette, ThemeType};
pub use code_generation::{CodeGenerator, CodeTemplate, GeneratedCode, GenerationContext};

use egui::*;
use serde::{Serialize, Deserialize};

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
    
    /// Get current design tokens
    pub fn get_design_tokens(&self) -> &DesignTokenSystem {
        &self.design_tokens
    }
    
    /// Get component library
    pub fn get_component_library(&self) -> &ComponentLibrary {
        &self.component_library
    }
    
    /// Export components to framework
    pub fn export_to_framework(
        &self,
        target: &str,
        components: &[framework_export::ComponentData],
    ) -> Result<Vec<GeneratedFile>, framework_export::ExportError> {
        self.framework_export.export_to_framework(target, components)
    }
    
    /// Generate code from template
    pub fn generate_code(
        &self,
        template_id: &str,
        context: &code_generation::GenerationContext,
    ) -> Result<GeneratedCode, code_generation::GenerationError> {
        self.code_generator.generate_code(template_id, context)
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
            egui::Grid::new("color_tokens")
                .num_columns(3)
                .spacing([10.0, 5.0])
                .show(ui, |ui| {
                    ui.label("Name");
                    ui.label("Value");
                    ui.label("Actions");
                    ui.end_row();
                    
                    for (name, token) in &self.design_tokens.colors {
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
                        
                        ui.horizontal(|ui| {
                            if ui.small_button("Edit").clicked() {
                                // Handle color token editing
                            }
                            if ui.small_button("Delete").clicked() {
                                // Handle color token deletion
                            }
                        });
                        ui.end_row();
                    }
                });
            
            if ui.button("+ Add Color Token").clicked() {
                // Handle adding new color token
            }
        });
        
        ui.collapsing("Typography Tokens", |ui| {
            egui::Grid::new("typography_tokens")
                .num_columns(4)
                .spacing([10.0, 5.0])
                .show(ui, |ui| {
                    ui.label("Name");
                    ui.label("Font Family");
                    ui.label("Font Size");
                    ui.label("Actions");
                    ui.end_row();
                    
                    for (name, token) in &self.design_tokens.typography {
                        ui.label(name);
                        ui.label(&token.font_family);
                        ui.label(format!("{}px", token.font_size));
                        
                        ui.horizontal(|ui| {
                            if ui.small_button("Edit").clicked() {
                                // Handle typography token editing
                            }
                            if ui.small_button("Delete").clicked() {
                                // Handle typography token deletion
                            }
                        });
                        ui.end_row();
                    }
                });
            
            if ui.button("+ Add Typography Token").clicked() {
                // Handle adding new typography token
            }
        });
        
        ui.collapsing("Spacing Tokens", |ui| {
            egui::Grid::new("spacing_tokens")
                .num_columns(3)
                .spacing([10.0, 5.0])
                .show(ui, |ui| {
                    ui.label("Name");
                    ui.label("Value");
                    ui.label("Actions");
                    ui.end_row();
                    
                    for (name, token) in &self.design_tokens.spacing {
                        ui.label(name);
                        ui.label(format!("{}px", token.value));
                        
                        ui.horizontal(|ui| {
                            if ui.small_button("Edit").clicked() {
                                // Handle spacing token editing
                            }
                            if ui.small_button("Delete").clicked() {
                                // Handle spacing token deletion
                            }
                        });
                        ui.end_row();
                    }
                });
            
            if ui.button("+ Add Spacing Token").clicked() {
                // Handle adding new spacing token
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
                    // In a real implementation, save to file or clipboard
                    println!("Exported tokens: {}", json);
                }
            }
        });
    }
    
    /// Export current state as JSON
    pub fn export_json(&self) -> serde_json::Result<String> {
        #[derive(Serialize)]
        struct ExportData {
            design_tokens: &DesignTokenSystem,
            component_library: &ComponentLibrary,
            theme_system: &ThemeSystem,
        }
        
        let export_data = ExportData {
            design_tokens: &self.design_tokens,
            component_library: &self.component_library,
            theme_system: &self.theme_system,
        };
        
        serde_json::to_string_pretty(&export_data)
    }
    
    /// Import state from JSON
    pub fn import_json(&mut self, json: &str) -> serde_json::Result<()> {
        #[derive(Deserialize)]
        struct ImportData {
            design_tokens: Option<DesignTokenSystem>,
            component_library: Option<ComponentLibrary>,
            theme_system: Option<ThemeSystem>,
        }
        
        let import_data: ImportData = serde_json::from_str(json)?;
        
        if let Some(design_tokens) = import_data.design_tokens {
            self.design_tokens = design_tokens;
        }
        
        if let Some(component_library) = import_data.component_library {
            self.component_library = component_library;
        }
        
        if let Some(theme_system) = import_data.theme_system {
            self.theme_system = theme_system;
        }
        
        Ok(())
    }
}