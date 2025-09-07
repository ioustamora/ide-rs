//! # RCL (Rapid Component Library)
//!
//! RCL is a comprehensive component library providing UI components, system utilities,
//! and network functionality for the IDE. It is designed to offer a unified API for
//! building modern desktop applications with advanced functionality.
//!
//! ## Architecture
//!
//! RCL is organized into three main modules:
//! - [`ui`] - User interface components (basic and advanced)
//! - [`system`] - System integration utilities (clipboard, file system, etc.)
//! - [`network`] - Network communication components (HTTP, WebSocket, etc.)
//!
//! ## Design Philosophy
//!
//! - **Component-based**: All functionality is encapsulated in reusable components
//! - **Type-safe**: Leverages Rust's type system for compile-time correctness
//! - **Async-first**: Network and I/O operations use async/await patterns
//! - **Cross-platform**: Designed to work across different operating systems
//!
//! ## Usage
//!
//! Components can be used independently or composed together to build complex applications.
//! The library provides both low-level building blocks and high-level composite components.

/// User interface components module
/// 
/// Contains both basic UI elements (buttons, labels, etc.) and advanced components
/// (charts, editors, etc.) for building modern application interfaces.
pub mod ui;

/// System integration utilities module
/// 
/// Provides cross-platform access to system resources like clipboard, file system,
/// process management, and power management.
pub mod system;

/// Network communication components module
/// 
/// Implements various network protocols and utilities including HTTP clients,
/// WebSocket connections, and network monitoring capabilities.
pub mod network;

/// Component registry and metadata system
/// 
/// Provides unified metadata registry & property typing with schemas and
/// auto-generated property inspector functionality.
pub mod component_registry;

// Re-export UI components from correct paths
// Components are re-exported at the crate level for convenient access
