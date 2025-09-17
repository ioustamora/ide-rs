//! # Performance Module
//!
//! This module provides performance optimization components for the code editor,
//! including virtual scrolling, caching, and background processing.

pub mod virtual_editor;
pub mod syntax_cache;
pub mod performance_monitor;
pub mod memory_optimizer;

// Re-export key types
pub use virtual_editor::{VirtualCodeEditor, VirtualViewport};
pub use syntax_cache::{SyntaxHighlightCache, BackgroundHighlighter};
pub use performance_monitor::{PerformanceMonitor, PerformanceMetrics};
pub use memory_optimizer::{MemoryOptimizer, MemoryStats};