//! Library entry point for ide-rs
pub mod rcl;
pub mod editor;
pub mod ai_agent;
pub mod ai_development_assistant;
pub mod ide_app;
pub mod shared;
pub mod core;

// Re-export main modules for easy access
pub use ide_app::IdeApp;
pub use editor::project_manager;
