//! Rust RAD IDE - A modern, extensible IDE for Rust development
//! 
//! This is the main entry point for the Rust RAD IDE application.
//! The IDE features a visual drag-and-drop GUI builder, integrated AI assistance,
//! and a comprehensive Rust Component Library (RCL).

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod rcl;      // Rust Component Library - UI, system, and network components
mod ai_agent; // AI integration for code assistance and automation  
mod ide_app;  // Main IDE application logic and UI
mod editor;   // IDE editor features: panels, actions, project management
mod core;     // Core infrastructure: logging, events, services

/// Main entry point for the Rust RAD IDE application
/// 
/// Initializes logging, then launches the eframe/egui application with default settings
/// and starts the main IDE interface.
fn main() {
    // Initialize logging as early as possible
    core::logging::init_logging();
    
    #[cfg(feature = "logging")]
    tracing::info!("Starting Rust RAD IDE");

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust RAD IDE",
        options,
        Box::new(|cc| {
            #[cfg(feature = "logging")]
            tracing::debug!("Creating IDE app with creation context");
            
            Box::new(ide_app::IdeApp::new(cc))
        }),
    ).unwrap();
}
