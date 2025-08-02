//! Main menu system for IDE actions and commands
//!
//! This module provides the primary menu interface for the Rust RAD IDE,
//! organizing all major IDE functions into a cohesive menu system.
//! Includes integration with build tools, AI assistance, component management, and project operations.

use egui::*;
use crate::editor::{toolbar::IdeToolbar, output_panel::OutputPanel, ai_panel::AiPanel, component_registry::ComponentRegistry};
use crate::editor::actions;

/// Main IDE menu system containing all major IDE functionality
/// 
/// The IdeMenu coordinates between different IDE subsystems:
/// - Build and run operations via toolbar integration
/// - Output display through the output panel
/// - AI assistance via the AI panel
/// - Component management through the registry
/// 
/// This structure serves as the central hub for IDE commands and provides
/// a unified interface for all major IDE operations.
#[allow(dead_code)]
pub struct IdeMenu {
    /// Toolbar interface for quick access to common actions
    pub toolbar: IdeToolbar,
    /// Panel for displaying build output, errors, and logs
    pub output_panel: OutputPanel,
    /// AI assistant panel for code help and automation
    pub ai_panel: AiPanel,
    /// Registry for managing installed and available components
    pub registry: ComponentRegistry,
    /// Show settings panel flag
    pub show_settings: bool,
}

impl IdeMenu {
    /// Creates a new IDE menu with all subsystems initialized
    pub fn new() -> Self {
        Self {
            toolbar: IdeToolbar::new(),
            output_panel: OutputPanel::new(),
            ai_panel: AiPanel::new(),
            registry: ComponentRegistry::new(),
            show_settings: false,
        }
    }

    /// Renders the main IDE menu with all available commands
    /// 
    /// This method creates a dropdown menu containing all major IDE operations
    /// organized into logical groups:
    /// - Build operations (debug/release builds and runs)
    /// - AI assistance (chat, automated fixes)
    /// - Component management (packaging, installation)
    /// - Project operations (export, settings)
    /// - Code tools (formatting, output display)
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.menu_button("IDE", |ui| {
            // Build and Run Submenu
            ui.menu_button("🔨 Build & Run", |ui| {
                if ui.button("Build Debug   Ctrl+B").clicked() {
                    actions::build_debug(&mut self.output_panel);
                }
                if ui.button("Build Release   Ctrl+Shift+B").clicked() {
                    actions::build_release(&mut self.output_panel);
                }
                if ui.button("Run Debug   F5").clicked() {
                    actions::run_debug(&mut self.output_panel);
                }
                if ui.button("Run Release   Ctrl+F5").clicked() {
                    actions::run_release(&mut self.output_panel);
                }
            });

            // AI Assistant Submenu
            ui.menu_button("🤖 AI Assistant", |ui| {
                if ui.button("AI Chat   Ctrl+Alt+A").clicked() {
                    self.ai_panel.ui(ui);
                }
                if ui.button("Fix with AI   Alt+F").clicked() {
                    // TODO: Trigger AI fix logic for current errors
                }
            });

            // Component Management Submenu
            ui.menu_button("📦 Components", |ui| {
                if ui.button("Package Components   Ctrl+P").clicked() {
                    actions::package_component("MyComponent", "src/rcl/ui/basic/button.rs", "packages/MyComponent.pkg", &mut self.registry);
                }
                if ui.button("Install Component   Ctrl+I").clicked() {
                    actions::install_component("packages/MyComponent.pkg", "installed_components", &mut self.registry);
                }
                if ui.button("Uninstall Component   Ctrl+U").clicked() {
                    actions::uninstall_component("MyComponent.pkg", "installed_components", &mut self.registry);
                }
            });

            // Project and Tools Submenu
            ui.menu_button("📁 Project & Tools", |ui| {
                if ui.button("Export Project   Ctrl+E").clicked() {
                    // TODO: Trigger export project logic
                }
                if ui.button("Format Code   Ctrl+Shift+F").clicked() {
                    // TODO: Trigger code formatting logic (rustfmt integration)
                }
                if ui.button("Settings   Ctrl+, ").clicked() {
                    self.show_settings = true;
                }
                if ui.button("Show Output   Ctrl+O").clicked() {
                    self.output_panel.ui(ui);
                }

                // Recent Projects submenu (placeholder, up to 5)
                ui.menu_button("Recent Projects", |ui| {
                    // In a real implementation, this would be loaded from project manager
                    let recent_projects = [
                        "MyApp1",
                        "MyApp2",
                        "MyApp3",
                        "MyApp4",
                        "MyApp5",
                    ];
                    for name in recent_projects.iter() {
                        if ui.button(*name).clicked() {
                            // TODO: Open the selected recent project
                        }
                    }
                });
            });

            // Settings dialog (modal)
            if self.show_settings {
                egui::Window::new("Settings")
                    .open(&mut self.show_settings)
                    .collapsible(false)
                    .resizable(true)
                    .show(ui.ctx(), |ui| {
                        ui.heading("IDE Settings (Placeholder)");
                        ui.label("This is a placeholder for the settings dialog.");
                        ui.label("Add settings options here.");
                    });
            }
        });
    }
}
