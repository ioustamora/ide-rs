//! Rust RAD IDE - A modern, extensible IDE for Rust development
//! 
//! This is the main entry point for the Rust RAD IDE application.
//! The IDE features a visual drag-and-drop GUI builder, integrated AI assistance,
//! and a comprehensive Rust Component Library (RCL).

mod rcl;      // Rust Component Library - UI, system, and network components
mod ai_agent; // AI integration for code assistance and automation  
mod ide_app;  // Main IDE application logic and UI
mod editor;   // IDE editor features: panels, actions, project management

/// Main entry point for the Rust RAD IDE application
/// 
/// Initializes the eframe/egui application with default settings
/// and launches the main IDE interface.
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust RAD IDE",
        options,
        Box::new(|_cc| Box::new(ide_app::IdeApp::default())),
    ).unwrap();
}
