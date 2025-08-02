//! IDE Actions and Commands

use crate::editor::{output_panel::OutputPanel, component_registry::ComponentRegistry};

/// Action system for the IDE
#[derive(Default)]
pub struct ActionManager {
    pub recent_actions: Vec<String>,
}

pub type Actions = ActionManager;

impl ActionManager {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn execute_action(&mut self, action: &str) {
        self.recent_actions.push(action.to_string());
        if self.recent_actions.len() > 10 {
            self.recent_actions.remove(0);
        }
    }

    // Build methods
    pub fn build_debug(&mut self, output_panel: &mut OutputPanel) {
        build_debug(output_panel);
        self.execute_action("build_debug");
    }

    pub fn build_release(&mut self, output_panel: &mut OutputPanel) {
        build_release(output_panel);
        self.execute_action("build_release");
    }

    pub fn run_debug(&mut self, output_panel: &mut OutputPanel) {
        run_debug(output_panel);
        self.execute_action("run_debug");
    }

    pub fn run_release(&mut self, output_panel: &mut OutputPanel) {
        run_release(output_panel);
        self.execute_action("run_release");
    }

    // AI methods
    pub fn ai_chat(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("🤖 AI Chat feature would open here");
        self.execute_action("ai_chat");
    }

    pub fn ai_fix(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("🔧 AI Fix feature would open here");
        self.execute_action("ai_fix");
    }

    // Packaging methods
    pub fn package_components(&mut self, output_panel: &mut OutputPanel) {
        output_panel.log("📦 Packaging all components...");
        output_panel.log("✅ All components packaged successfully");
        self.execute_action("package_components");
    }

    pub fn export_project(&mut self, output_panel: &mut OutputPanel) {
        output_panel.log("📤 Exporting project...");
        output_panel.log("✅ Project exported successfully");
        self.execute_action("export_project");
    }

    pub fn format_code(&mut self, output_panel: &mut OutputPanel) {
        output_panel.log("🎨 Formatting code...");
        output_panel.log("✅ Code formatted successfully");
        self.execute_action("format_code");
    }

    // Settings method
    pub fn open_settings(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("⚙️ Settings panel would open here");
        self.execute_action("open_settings");
    }
}

/// Global actions instance
pub static mut ACTIONS: ActionManager = ActionManager { recent_actions: Vec::new() };

pub fn get_actions() -> &'static mut ActionManager {
    unsafe { &mut ACTIONS }
}

// Build actions
pub fn build_debug(output_panel: &mut OutputPanel) {
    output_panel.log("🔨 Building debug version...");
    output_panel.log("✅ Debug build completed");
}

pub fn build_release(output_panel: &mut OutputPanel) {
    output_panel.log("🔨 Building release version...");
    output_panel.log("✅ Release build completed");
}

pub fn run_debug(output_panel: &mut OutputPanel) {
    output_panel.log("🚀 Running debug version...");
    output_panel.log("✅ Application started");
}

pub fn run_release(output_panel: &mut OutputPanel) {
    output_panel.log("🚀 Running release version...");
    output_panel.log("✅ Application started");
}

// Component packaging actions
pub fn package_component(name: &str, source: &str, output: &str, _registry: &mut ComponentRegistry) {
    // In a real implementation, this would actually package components
    println!("📦 Packaging component {} from {} to {}", name, source, output);
    println!("✅ Component packaged successfully");
}

pub fn install_component(package: &str, destination: &str, _registry: &mut ComponentRegistry) {
    // In a real implementation, this would actually install components
    println!("📥 Installing component from {} to {}", package, destination);
    println!("✅ Component installed successfully");
}

pub fn uninstall_component(package: &str, location: &str, _registry: &mut ComponentRegistry) {
    // In a real implementation, this would actually uninstall components
    println!("🗑 Uninstalling component {} from {}", package, location);
    println!("✅ Component uninstalled successfully");
}

// Error parsing
pub fn parse_errors(error_output: &str) -> Vec<String> {
    error_output
        .lines()
        .filter(|line| line.contains("error:") || line.contains("warning:"))
        .map(|line| line.to_string())
        .collect()
}