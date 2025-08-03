//! # RAD IDE Application - Modular Architecture
//! 
//! This module implements the main IDE application using a modular architecture
//! that separates concerns into focused, maintainable components.
//!
//! ## Architecture Overview
//! 
//! The IDE follows a modular architecture with several key subsystems:
//! - **App State**: Central application state management
//! - **UI Manager**: Panel layout and UI orchestration
//! - **Content Manager**: Main content area and workspace management
//! - **Drag & Drop**: Component manipulation and interaction system
//! - **Event Handlers**: Input processing and command coordination
//!
//! ## Benefits of Modular Design
//! 
//! - **Maintainability**: Each module has a single responsibility
//! - **Testability**: Individual modules can be tested in isolation
//! - **Extensibility**: New features can be added without affecting existing modules
//! - **Code Organization**: Related functionality is grouped logically

pub mod app_state;
pub mod ui_manager;
pub mod content_manager;
pub mod drag_drop;
pub mod event_handlers;
pub mod animated_ui;

use eframe::egui;
use app_state::IdeAppState;
use ui_manager::UiManager;
use content_manager::ContentManager;
use drag_drop::DragState;
use event_handlers::EventHandlers;

/// # Main IDE Application
/// 
/// The main application struct that orchestrates all IDE subsystems.
/// This is a lightweight coordinator that delegates to specialized modules.
pub struct IdeApp {
    /// Core application state
    app_state: IdeAppState,
    
    /// Drag and drop interaction state
    drag_state: DragState,
    
    /// Event handling state
    event_handlers: EventHandlers,
}

impl IdeApp {
    /// Create a new IDE application instance
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            app_state: IdeAppState::new(),
            drag_state: DragState::new(),
            event_handlers: EventHandlers::new(),
        }
    }
    
    /// Get the default Rust code template for new projects
    fn default_rust_code() -> String {
        r#"fn main() {
    println!("Hello, RAD IDE!");
}

// Sample component structure
struct MyComponent {
    value: i32,
}

impl MyComponent {
    fn new(value: i32) -> Self {
        Self { value }
    }
    
    fn render(&self) {
        println!("Component value: {}", self.value);
    }
}
"#.to_string()
    }
}

impl eframe::App for IdeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle global keyboard shortcuts and events
        self.event_handlers.handle_global_events(&mut self.app_state, ctx);
        ContentManager::handle_shortcuts(&mut self.app_state, ctx);
        
        // Render UI panels in order
        UiManager::render_top_panel(&mut self.app_state, ctx);
        UiManager::render_left_panel(&mut self.app_state, ctx);
        UiManager::render_right_panel(&mut self.app_state, ctx);
        UiManager::render_bottom_panel(&mut self.app_state, ctx);
        
        // Render main content area
        ContentManager::render_central_panel(&mut self.app_state, &mut self.drag_state, ctx);
        
        // Process any pending events
        self.event_handlers.process_pending_events(&mut self.app_state);
    }
}

impl Default for IdeApp {
    fn default() -> Self {
        Self {
            app_state: IdeAppState::default(),
            drag_state: DragState::default(),
            event_handlers: EventHandlers::default(),
        }
    }
}

// Re-export public types for convenience
pub use drag_drop::{DragType, ComponentType, ResizeHandle};