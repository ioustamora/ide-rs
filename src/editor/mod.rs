//! # Visual Editor Module
//!
//! This module provides a comprehensive visual editor for building desktop applications
//! with drag-and-drop functionality, live preview, and intelligent code generation.
//! The editor combines visual design tools with powerful IDE features to create
//! a modern development experience.
//!
//! ## Core Components
//!
//! ### Visual Design System
//! - [`canvas`] - Main design surface with drag-and-drop functionality
//! - [`palette`] - Component selection and organization interface
//! - [`visual_designer`] - Complete visual design environment
//! - [`state`] - Centralized state management for editor operations
//!
//! ### User Interface Elements
//! - [`toolbar`] - Main editor toolbar with tools and actions
//! - [`inspector`] - Property editing and component configuration
//! - [`menu`] - Application menu system and command interface
//! - [`output_panel`] - Build output, logs, and diagnostic information
//!
//! ### Project Management
//! - [`project_manager`] - Project creation, loading, and management
//! - [`file_operations`] - File system operations and workspace management
//! - [`packaging`] - Application building and distribution
//! - [`template_system`] - Project templates and scaffolding
//!
//! ### Code Intelligence
//! - [`code_editor`] - Advanced code editing with syntax highlighting
//! - [`rust_analyzer`] - Rust language server integration
//! - [`lsp_integration`] - Language Server Protocol support
//! - [`smart_editing`] - Intelligent code completion and refactoring
//!
//! ### AI Integration
//! - [`ai_panel`] - AI assistance interface and controls
//! - [`smart_ai_assistant`] - Context-aware AI development helper
//! - [`live_feedback`] - Real-time AI suggestions and improvements
//!
//! ### Component System
//! - [`component_registry`] - Component discovery and management
//! - [`custom_component`] - Custom component creation and editing
//! - [`hierarchy_manager`] - Component hierarchy and relationships
//!
//! ### Advanced Features
//! - [`build_system`] - Automated building and compilation
//! - [`multi_device_preview`] - Cross-platform preview and testing
//! - [`advanced_alignment`] - Sophisticated layout and alignment tools
//! - [`modern_ide_integration`] - Integration with external IDEs
//!
//! ### Core Operations
//! - [`actions`] - Backend actions and command execution
//!
//! ## Architecture
//!
//! The visual editor follows a modular architecture where each component
//! handles specific functionality while maintaining clear interfaces for
//! integration and communication.
//!
//! ### Event Flow
//! 1. User interactions are captured by UI components
//! 2. State changes are coordinated through the central state manager
//! 3. Actions trigger backend operations and code generation
//! 4. Visual feedback is provided through live preview and updates
//!
//! ### Code Generation
//! The editor translates visual designs into clean, maintainable Rust code
//! that follows best practices and integrates with the RCL component system.
//!
//! ## Usage
//!
//! The editor is designed to be used as an integrated development environment
//! for creating desktop applications with visual tools while maintaining
//! full access to code-level customization.

/// Backend action handlers and command execution
/// 
/// Provides the interface between the visual editor and backend operations
/// like building, project management, and code generation.
pub mod actions;

/// Visual design canvas with drag-and-drop functionality
/// 
/// The main design surface where users create and manipulate UI layouts
/// through direct manipulation and visual feedback.
pub mod canvas;

/// Component palette for selecting and organizing UI elements
/// 
/// Provides browsable access to available components that can be added
/// to designs through drag-and-drop or selection interfaces.
pub mod palette;

/// Property inspector for component configuration
/// 
/// Allows detailed editing of component properties, styling, and behavior
/// through a context-sensitive interface.
pub mod inspector;

/// Object Inspector with component hierarchy tree (Delphi/C++ Builder inspired)
/// 
/// Provides a hierarchical view of all components in the form, allowing users to
/// navigate parent-child relationships, select components, and manage hierarchy.
pub mod object_inspector;

/// Enhanced property inspector with modern UX features
/// 
/// Advanced property editing system with multi-selection, design system integration,
/// AI suggestions, animation system, and contextual panels.
// TODO: Fix type mismatches before enabling
// pub mod enhanced_property_inspector;

/// Centralized state management for editor operations
/// 
/// Manages selection, drag-and-drop state, and coordination between
/// different parts of the editor interface.
pub mod state;

/// Advanced code editor with syntax highlighting and intelligence
/// 
/// Provides rich text editing capabilities with language-specific features
/// for viewing and editing generated code.
pub mod code_editor;

/// Advanced code editor with enhanced LSP integration
/// 
/// Professional code editor with VS Code-style features including real-time
/// diagnostics, go-to-definition, find references, code actions, and signature help.
pub mod advanced_code_editor;

/// Advanced syntax highlighting using syntect
/// 
/// Provides high-quality syntax highlighting for multiple programming
/// languages with theme support and token-level highlighting.
pub mod syntax_highlighter;

/// Rust Analyzer integration for language services
/// 
/// Integrates with the Rust language server to provide intelligent
/// code completion, error checking, and refactoring support.
pub mod rust_analyzer;

/// Project creation, management, and workspace operations
/// 
/// Handles project lifecycle from creation through development with
/// support for templates, dependencies, and configuration.
pub mod project_manager;

/// Main editor toolbar with tools and actions
/// 
/// Provides quick access to common operations, view modes, and
/// development tools through a customizable toolbar interface.
pub mod toolbar;

/// Build output, diagnostics, and logging interface
/// 
/// Displays compilation results, error messages, and development
/// feedback in an organized, searchable format.
pub mod output_panel;

/// Component discovery and registration system
/// 
/// Manages available components, their metadata, and integration
/// with the visual design system.
pub mod component_registry;

/// Application building and distribution tools
/// 
/// Handles compilation, bundling, and packaging of applications
/// for deployment across different platforms.
pub mod packaging;

/// AI assistance interface and controls
/// 
/// Provides access to AI-powered development assistance including
/// code generation, optimization suggestions, and problem solving.
pub mod ai_panel;

/// Custom component creation and editing tools
/// 
/// Allows developers to create reusable custom components with
/// visual tools and code generation support.
pub mod custom_component;

/// Application menu system and command interface
/// 
/// Implements the main application menu with organized access to
/// all editor features and development operations.
pub mod menu;

/// Complete visual design environment and orchestration
/// 
/// Coordinates all visual design components into a cohesive
/// development environment with integrated tooling.
pub mod visual_designer;

/// Context-aware AI development assistant
/// 
/// Provides intelligent assistance based on current project context,
/// development patterns, and user behavior.
pub mod smart_ai_assistant;

/// Language Server Protocol integration layer
/// 
/// Generic LSP support for multiple programming languages with
/// standardized interfaces for code intelligence features.
pub mod lsp_integration;

/// Enhanced LSP client with advanced IDE features
/// 
/// Professional LSP client with VS Code-style capabilities including
/// go-to-definition, find references, code actions, signature help, and more.
pub mod enhanced_lsp_client;

/// File system operations and workspace management
/// 
/// Handles file operations, workspace organization, and integration
/// with version control and build systems.
pub mod file_operations;

/// Automated building and compilation system
/// 
/// Manages build processes, dependency resolution, and integration
/// with external build tools and systems.
pub mod build_system;

/// Real-time feedback and suggestion system
/// 
/// Provides immediate feedback on design decisions and code quality
/// with actionable suggestions for improvement.
pub mod live_feedback;

/// Intelligent code completion and refactoring tools
/// 
/// Advanced editing assistance with context-aware suggestions,
/// automated refactoring, and code quality improvements.
pub mod smart_editing;

/// Component hierarchy and relationship management
/// 
/// Manages the tree structure of UI components with tools for
/// navigation, reorganization, and relationship visualization.
pub mod hierarchy_manager;

/// Integration with external IDEs and development tools
/// 
/// Provides bridges to popular development environments for
/// enhanced workflow integration and tool interoperability.
pub mod modern_ide_integration;

/// Cross-platform preview and testing interface
/// 
/// Enables testing and preview of applications across different
/// devices, screen sizes, and platform configurations.
pub mod multi_device_preview;

/// Advanced layout and alignment tools
/// 
/// Sophisticated tools for precise component positioning,
/// alignment guides, and responsive layout design.
pub mod advanced_alignment;

/// Project templates and scaffolding system
/// 
/// Provides project templates, code generation patterns, and
/// scaffolding tools for rapid application development.
pub mod template_system_simple;

/// Smart editing modular components
/// 
/// Modular architecture for intelligent editing features including
/// alignment guides, magnetism, spacing guides, and learning systems.
pub mod smart_editing_modules;

/// Modern IDE integration modular components
/// 
/// Modular architecture for IDE integration including design tokens,
/// component library, framework export, theme system, and code generation.
pub mod modern_ide_integration_modules;

/// Multi-file management with tab system
/// 
/// VS Code-inspired file management with tabs, file type recognition,
/// and automatic mode switching between code editor and visual designer.
pub mod file_manager;

/// Real-time synchronization between visual designer and code
/// 
/// Provides bidirectional synchronization ensuring changes are reflected
/// immediately in both visual designer and generated code.
pub mod realtime_sync;
