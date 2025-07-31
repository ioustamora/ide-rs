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
}

impl IdeMenu {
    /// Creates a new IDE menu with all subsystems initialized
    pub fn new() -> Self {
        Self {
            toolbar: IdeToolbar::new(),
            output_panel: OutputPanel::new(),
            ai_panel: AiPanel::new(),
            registry: ComponentRegistry::new(),
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
            // Build and Run Commands
            ui.label("Build & Run:");
            if ui.button("Build Debug").clicked() {
                actions::build_debug(&mut self.output_panel);
            }
            if ui.button("Build Release").clicked() {
                actions::build_release(&mut self.output_panel);
            }
            if ui.button("Run Debug").clicked() {
                actions::run_debug(&mut self.output_panel);
            }
            if ui.button("Run Release").clicked() {
                actions::run_release(&mut self.output_panel);
            }
            
            ui.separator();
            
            // AI Assistant Commands
            ui.label("AI Assistant:");
            if ui.button("AI Chat").clicked() {
                self.ai_panel.ui(ui);
            }
            if ui.button("Fix with AI").clicked() {
                // TODO: Trigger AI fix logic for current errors
            }
            
            ui.separator();
            
            // Component Management Commands
            ui.label("Components:");
            if ui.button("Package Components").clicked() {
                actions::package_component("MyComponent", "src/rcl/ui/basic/button.rs", "packages/MyComponent.pkg", &mut self.registry);
            }
            if ui.button("Install Component").clicked() {
                actions::install_component("packages/MyComponent.pkg", "installed_components", &mut self.registry);
            }
            if ui.button("Uninstall Component").clicked() {
                actions::uninstall_component("MyComponent.pkg", "installed_components", &mut self.registry);
            }
            
            ui.separator();
            
            // Project and Tool Commands
            ui.label("Project & Tools:");
            if ui.button("Export Project").clicked() {
                // TODO: Trigger export project logic
            }
            if ui.button("Format Code").clicked() {
                // TODO: Trigger code formatting logic (rustfmt integration)
            }
            if ui.button("Settings").clicked() {
                // TODO: Open IDE settings panel
            }
            if ui.button("Show Output").clicked() {
                self.output_panel.ui(ui);
            }
        });
    }
}
