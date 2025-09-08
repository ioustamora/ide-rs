//! # Unified Terminal System
//!
//! This module provides a unified terminal infrastructure that consolidates the basic
//! and advanced terminal functionality to eliminate duplication while maintaining
//! feature-rich capabilities.
//!
//! The architecture follows the improvement plan by creating:
//! - Core terminal abstractions shared between basic and advanced features
//! - Feature-gated advanced functionality
//! - Unified API for terminal creation and management

pub mod core;
pub mod features;
pub mod themes;
pub mod session;

// Re-export core types
pub use core::{
    Terminal, TerminalManager, TerminalSettings, TerminalState,
    TerminalLine, ShellType, LineType, TerminalUIState,
};

// Re-export feature modules
pub use features::{
    AutoCompletion, EnvironmentManager, CommandIntelligence,
    PackageIntegration, GitIntegration, ShellIntegration,
};

pub use themes::{TerminalTheme, TerminalColorScheme};
pub use session::{SessionManager, TerminalSession, BookmarkManager};