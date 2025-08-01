//! Library entry point for ide-rs
pub mod rcl;
pub mod editor;
pub mod ai_agent;
pub mod ide_app;

// Re-export main modules for easy access
pub use ide_app::IdeApp;
pub use editor::project_manager;
