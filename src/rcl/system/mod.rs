//! # System Integration Components
//!
//! This module provides cross-platform system integration components that allow
//! RCL applications to interact with the underlying operating system and hardware.
//! These components abstract platform-specific operations behind consistent APIs.
//!
//! ## Component Categories
//!
//! ### Data Management
//! - [`clipboard`] - Cross-platform clipboard operations and data exchange
//! - [`file_system`] - File and directory operations with path management
//!
//! ### System Services
//! - [`process_manager`] - Process lifecycle management and monitoring
//! - [`power_manager`] - Power state management and energy optimization
//! - [`system_info`] - Hardware and operating system information
//!
//! ## Design Philosophy
//!
//! System components follow these principles:
//!
//! - **Cross-platform**: Consistent API across Windows, macOS, and Linux
//! - **Safe Abstractions**: Memory-safe wrappers around system APIs
//! - **Error Handling**: Comprehensive error reporting with context
//! - **Performance**: Efficient system resource utilization
//! - **Security**: Secure access patterns and permission handling
//!
//! ## Platform Support
//!
//! All system components are designed to work across major desktop platforms:
//! - **Windows**: Win32 API integration with proper error handling
//! - **macOS**: Cocoa and system framework integration
//! - **Linux**: Standard Unix APIs with distribution compatibility
//!
//! ## Security Considerations
//!
//! System components handle sensitive operations and follow security best practices:
//! - Minimal privilege requirements
//! - Input validation and sanitization
//! - Secure temporary file handling
//! - Proper resource cleanup and disposal
//!
//! # Examples
//!
//! ```ignore
//! use crate::rcl::system::{clipboard::Clipboard, file_system::FileSystem};
//!
//! // Cross-platform clipboard operations
//! let mut clipboard = Clipboard::new();
//! clipboard.set_text("Hello, World!".to_string()).expect("Failed to set clipboard");
//! let text = clipboard.get_text().expect("Failed to get clipboard text");
//!
//! // File system operations
//! let fs = FileSystem::new();
//! let files = fs.list_directory("/home/user").expect("Failed to list directory");
//! ```

/// Cross-platform clipboard operations and data exchange
/// 
/// Provides secure access to system clipboard functionality with support
/// for text, images, and custom data formats across different platforms.
pub mod clipboard;

/// File and directory operations with cross-platform path management
/// 
/// Comprehensive file system interface with support for reading, writing,
/// directory operations, and metadata access with proper error handling.
pub mod file_system;

/// Power state management and energy optimization
/// 
/// System power management including sleep/wake control, battery monitoring,
/// and energy-efficient operation patterns for desktop applications.
pub mod power_manager;

/// Process lifecycle management and system monitoring
/// 
/// Tools for process creation, monitoring, and control with cross-platform
/// compatibility and comprehensive process information access.
pub mod process_manager;

/// Hardware and operating system information services
/// 
/// Provides detailed system information including hardware specifications,
/// OS version details, and runtime environment characteristics.
pub mod system_info;

// Re-export main types for convenience
