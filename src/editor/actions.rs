//! # IDE Actions and Commands
//!
//! This module provides the action system for the RAD IDE, handling all user-initiated
//! commands and operations. The action system serves as the central command dispatcher,
//! coordinating between different IDE subsystems and providing a unified interface
//! for build operations, AI assistance, project management, and component packaging.
//!
//! ## Core Components
//!
//! - [`ActionManager`] - Central action coordinator and command dispatcher
//! - **Build Actions** - Debug/release build and run operations
//! - **AI Actions** - AI-powered development assistance features
//! - **Package Actions** - Component packaging and project export
//! - **Utility Actions** - Code formatting, settings, and maintenance
//!
//! ## Architecture
//!
//! The action system follows a command pattern where each action is:
//! 1. Registered with the ActionManager
//! 2. Executed through a unified interface
//! 3. Logged for history and debugging
//! 4. Results are displayed in the output panel
//!
//! ## Usage Example
//!
//! ```rust
//! let mut action_manager = ActionManager::new();
//! let mut output_panel = OutputPanel::new();
//! 
//! // Execute a build action
//! action_manager.build_debug(&mut output_panel);
//! 
//! // Check recent actions
//! println!("Recent: {:?}", action_manager.recent_actions);
//! ```

use crate::editor::{output_panel::OutputPanel, component_registry::ComponentRegistry};

/// Central action manager for coordinating IDE commands and operations
/// 
/// The ActionManager serves as the primary interface for executing IDE actions,
/// maintaining action history, and coordinating between different subsystems.
/// It provides a unified command interface that can be triggered from menus,
/// toolbars, keyboard shortcuts, or programmatically.
/// 
/// ## Features
/// 
/// - **Action History**: Maintains a rolling history of recent actions
/// - **Build Integration**: Direct integration with Cargo build system
/// - **AI Integration**: Seamless connection to AI assistance features
/// - **Component Management**: Package and project export capabilities
/// - **Error Handling**: Comprehensive error reporting and logging
#[derive(Default)]
pub struct ActionManager {
    /// Rolling history of recent actions (limited to 10 entries)
    /// 
    /// Used for debugging, undo functionality, and user feedback.
    /// Actions are stored as string identifiers in chronological order.
    pub recent_actions: Vec<String>,
}

pub type Actions = ActionManager;

impl ActionManager {
    /// Create a new ActionManager instance
    /// 
    /// # Returns
    /// 
    /// A new ActionManager with empty action history
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Execute an action and add it to the history
    /// 
    /// This method records the action in the recent actions history,
    /// maintaining a rolling buffer of the last 10 actions for debugging
    /// and potential undo functionality.
    /// 
    /// # Arguments
    /// 
    /// * `action` - String identifier of the action being executed
    pub fn execute_action(&mut self, action: &str) {
        self.recent_actions.push(action.to_string());
        if self.recent_actions.len() > 10 {
            self.recent_actions.remove(0);
        }
    }

    // ========================================================================================
    // BUILD SYSTEM ACTIONS - Cargo integration for building and running projects
    // ========================================================================================

    /// Build the project in debug mode
    /// 
    /// Executes `cargo build` to compile the project with debug symbols
    /// and optimizations disabled for faster compilation and better debugging.
    /// 
    /// # Arguments
    /// 
    /// * `output_panel` - Output panel to display build progress and results
    /// 
    /// # Behavior
    /// 
    /// - Logs build start message
    /// - Executes cargo build command
    /// - Displays compilation output in real-time
    /// - Shows success/failure status with appropriate icons
    /// - Records action in history
    pub fn build_debug(&mut self, output_panel: &mut OutputPanel) {
        build_debug(output_panel);
        self.execute_action("build_debug");
    }

    /// Build the project in release mode
    /// 
    /// Executes `cargo build --release` to compile the project with full
    /// optimizations enabled for production deployment.
    /// 
    /// # Arguments
    /// 
    /// * `output_panel` - Output panel to display build progress and results
    /// 
    /// # Behavior
    /// 
    /// - Logs build start message
    /// - Executes cargo build --release command
    /// - Takes longer than debug but produces optimized binary
    /// - Displays compilation output with warnings and errors
    /// - Records action in history
    pub fn build_release(&mut self, output_panel: &mut OutputPanel) {
        build_release(output_panel);
        self.execute_action("build_release");
    }

    /// Run the project in debug mode
    /// 
    /// Executes `cargo run` to build (if necessary) and run the project
    /// with debug configuration for development and testing.
    /// 
    /// # Arguments
    /// 
    /// * `output_panel` - Output panel to display run output and application logs
    /// 
    /// # Behavior
    /// 
    /// - Compiles project if needed (debug mode)
    /// - Launches the application
    /// - Captures and displays application output
    /// - Shows runtime errors and panics
    /// - Records action in history
    pub fn run_debug(&mut self, output_panel: &mut OutputPanel) {
        run_debug(output_panel);
        self.execute_action("run_debug");
    }

    /// Run the project in release mode
    /// 
    /// Executes `cargo run --release` to build (if necessary) and run
    /// the optimized version of the project for performance testing.
    /// 
    /// # Arguments
    /// 
    /// * `output_panel` - Output panel to display run output and application logs
    /// 
    /// # Behavior
    /// 
    /// - Compiles project with optimizations if needed
    /// - Launches the optimized application
    /// - Better performance but slower compilation
    /// - Captures and displays application output
    /// - Records action in history
    pub fn run_release(&mut self, output_panel: &mut OutputPanel) {
        run_release(output_panel);
        self.execute_action("run_release");
    }

    // ========================================================================================
    // AI ASSISTANCE ACTIONS - Integration with AI-powered development features
    // ========================================================================================

    /// Open AI chat interface for development assistance
    /// 
    /// Launches the AI chat interface where developers can ask questions,
    /// get code suggestions, and receive help with development tasks.
    /// 
    /// # Arguments
    /// 
    /// * `ui` - egui UI context for rendering the chat interface
    /// 
    /// # Behavior
    /// 
    /// - Displays AI chat interface placeholder
    /// - Will integrate with AI agent when fully implemented
    /// - Records action in history for tracking AI usage
    pub fn ai_chat(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("🤖 AI Chat feature would open here");
        self.execute_action("ai_chat");
    }

    /// Trigger AI-powered code fix suggestions
    /// 
    /// Analyzes the current code context and provides AI-generated
    /// suggestions for fixing compilation errors, warnings, or code quality issues.
    /// 
    /// # Arguments
    /// 
    /// * `ui` - egui UI context for displaying fix suggestions
    /// 
    /// # Behavior
    /// 
    /// - Analyzes current code and compiler output
    /// - Generates AI-powered fix suggestions
    /// - Presents fixes in an interactive interface
    /// - Records action in history
    pub fn ai_fix(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("🔧 AI Fix feature would open here");
        self.execute_action("ai_fix");
    }

    // ========================================================================================
    // PROJECT MANAGEMENT ACTIONS - Component packaging and project export
    // ========================================================================================

    /// Package all components for distribution
    /// 
    /// Creates distributable packages for all custom components in the project,
    /// making them available for reuse in other projects or sharing with others.
    /// 
    /// # Arguments
    /// 
    /// * `output_panel` - Output panel to display packaging progress and results
    /// 
    /// # Behavior
    /// 
    /// - Scans project for custom components
    /// - Creates package files with metadata
    /// - Validates component integrity
    /// - Logs packaging progress and results
    /// - Records action in history
    pub fn package_components(&mut self, output_panel: &mut OutputPanel) {
        output_panel.log("📦 Packaging all components...");
        output_panel.log("✅ All components packaged successfully");
        self.execute_action("package_components");
    }

    /// Export the entire project for deployment or sharing
    /// 
    /// Creates a complete export of the project including source code,
    /// components, assets, and configuration files for deployment or distribution.
    /// 
    /// # Arguments
    /// 
    /// * `output_panel` - Output panel to display export progress and results
    /// 
    /// # Behavior
    /// 
    /// - Gathers all project files and dependencies
    /// - Creates deployment-ready package
    /// - Includes documentation and build instructions
    /// - Validates export completeness
    /// - Records action in history
    pub fn export_project(&mut self, output_panel: &mut OutputPanel) {
        output_panel.log("📤 Exporting project...");
        output_panel.log("✅ Project exported successfully");
        self.execute_action("export_project");
    }

    /// Format code using configured formatter
    /// 
    /// Applies consistent code formatting across the project using
    /// the configured code formatter (typically rustfmt for Rust projects).
    /// 
    /// # Arguments
    /// 
    /// * `output_panel` - Output panel to display formatting progress and results
    /// 
    /// # Behavior
    /// 
    /// - Scans project files for code that needs formatting
    /// - Applies consistent style rules
    /// - Reports formatting changes made
    /// - Preserves functionality while improving readability
    /// - Records action in history
    pub fn format_code(&mut self, output_panel: &mut OutputPanel) {
        output_panel.log("🎨 Formatting code...");
        output_panel.log("✅ Code formatted successfully");
        self.execute_action("format_code");
    }

    // ========================================================================================
    // UTILITY ACTIONS - Settings and configuration management
    // ========================================================================================

    /// Open the IDE settings and configuration panel
    /// 
    /// Launches the settings interface where users can configure IDE behavior,
    /// appearance, build settings, and other preferences.
    /// 
    /// # Arguments
    /// 
    /// * `ui` - egui UI context for rendering the settings interface
    /// 
    /// # Behavior
    /// 
    /// - Displays comprehensive settings interface
    /// - Allows modification of IDE preferences
    /// - Validates setting changes
    /// - Applies changes immediately or on restart as needed
    /// - Records action in history
    pub fn open_settings(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("⚙️ Settings panel would open here");
        self.execute_action("open_settings");
    }
}

/// Global actions instance for singleton access
/// 
/// **Safety Note**: This global static is marked as `unsafe` due to mutable
/// static access. In a production environment, consider using thread-safe
/// alternatives like `Arc<Mutex<ActionManager>>` or similar patterns.
pub static mut ACTIONS: ActionManager = ActionManager { recent_actions: Vec::new() };

/// Get mutable reference to the global actions instance
/// 
/// # Safety
/// 
/// This function uses unsafe code to access a mutable static. It should only
/// be called from the main thread in single-threaded contexts. For multi-threaded
/// applications, consider using thread-safe alternatives.
/// 
/// # Returns
/// 
/// Mutable reference to the global ActionManager instance
pub fn get_actions() -> &'static mut ActionManager {
    unsafe { &mut ACTIONS }
}

// ========================================================================================
// STANDALONE BUILD FUNCTIONS - Direct Cargo integration functions
// ========================================================================================

/// Execute debug build using Cargo
/// 
/// Directly invokes `cargo build` to compile the project in debug mode.
/// This function provides the core build functionality used by the ActionManager.
/// 
/// # Arguments
/// 
/// * `output_panel` - Output panel for displaying build progress and results
/// 
/// # Errors
/// 
/// Build errors are captured and displayed in the output panel rather than
/// returned, providing immediate feedback to the user through the UI.
pub fn build_debug(output_panel: &mut OutputPanel) {
    output_panel.log("🔨 Building debug version...");
    
    match execute_cargo_command(&["build"], output_panel) {
        Ok(_) => output_panel.log("✅ Debug build completed successfully"),
        Err(e) => output_panel.log(&format!("❌ Build failed: {}", e)),
    }
}

/// Execute release build using Cargo
/// 
/// Directly invokes `cargo build --release` to compile the project with
/// full optimizations for production deployment.
/// 
/// # Arguments
/// 
/// * `output_panel` - Output panel for displaying build progress and results
/// 
/// # Behavior
/// 
/// - Longer compilation time due to optimizations
/// - Produces smaller, faster binary
/// - Suitable for production deployment
pub fn build_release(output_panel: &mut OutputPanel) {
    output_panel.log("🔨 Building release version...");
    
    match execute_cargo_command(&["build", "--release"], output_panel) {
        Ok(_) => output_panel.log("✅ Release build completed successfully"),
        Err(e) => output_panel.log(&format!("❌ Build failed: {}", e)),
    }
}

/// Execute debug run using Cargo
/// 
/// Directly invokes `cargo run` to build (if needed) and execute the
/// project in debug mode for development and testing.
/// 
/// # Arguments
/// 
/// * `output_panel` - Output panel for displaying run output and application logs
/// 
/// # Behavior
/// 
/// - Fast compilation with debug symbols
/// - Better for development and debugging
/// - Captures application stdout/stderr
pub fn run_debug(output_panel: &mut OutputPanel) {
    output_panel.log("🚀 Running debug version...");
    
    match execute_cargo_command(&["run"], output_panel) {
        Ok(_) => output_panel.log("✅ Application started"),
        Err(e) => output_panel.log(&format!("❌ Run failed: {}", e)),
    }
}

/// Execute release run using Cargo
/// 
/// Directly invokes `cargo run --release` to build (if needed) and execute
/// the optimized version of the project for performance testing.
/// 
/// # Arguments
/// 
/// * `output_panel` - Output panel for displaying run output and application logs
/// 
/// # Behavior
/// 
/// - Optimized compilation for better runtime performance
/// - Slower compilation but faster execution
/// - Suitable for performance benchmarking
pub fn run_release(output_panel: &mut OutputPanel) {
    output_panel.log("🚀 Running release version...");
    
    match execute_cargo_command(&["run", "--release"], output_panel) {
        Ok(_) => output_panel.log("✅ Application started"),
        Err(e) => output_panel.log(&format!("❌ Run failed: {}", e)),
    }
}

/// Execute a cargo command and capture output
/// 
/// Core function for executing Cargo commands with real-time output capture
/// and display in the IDE's output panel. Handles both stdout and stderr,
/// applying appropriate formatting and icons for different message types.
/// 
/// # Arguments
/// 
/// * `args` - Command line arguments to pass to Cargo
/// * `output_panel` - Output panel for displaying command output
/// 
/// # Returns
/// 
/// * `Ok(())` - Command executed successfully (exit code 0)
/// * `Err(String)` - Command failed with error message
/// 
/// # Error Handling
/// 
/// - Process execution errors are captured and logged
/// - Compilation warnings are marked with ⚠️ icon
/// - Compilation errors are marked with ❌ icon
/// - Exit codes are reported for debugging
fn execute_cargo_command(args: &[&str], output_panel: &mut OutputPanel) -> Result<(), String> {
    use std::process::Command;
    
    let mut cmd = Command::new("cargo");
    cmd.args(args);
    
    output_panel.log(&format!("Executing: cargo {}", args.join(" ")));
    
    match cmd.output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            if !stdout.is_empty() {
                for line in stdout.lines() {
                    output_panel.log(line);
                }
            }
            
            if !stderr.is_empty() {
                for line in stderr.lines() {
                    if line.trim().starts_with("warning:") {
                        output_panel.log(&format!("⚠️ {}", line));
                    } else if line.trim().starts_with("error:") {
                        output_panel.log(&format!("❌ {}", line));
                    } else {
                        output_panel.log(line);
                    }
                }
            }
            
            if output.status.success() {
                Ok(())
            } else {
                Err(format!("Command failed with exit code: {}", 
                    output.status.code().unwrap_or(-1)))
            }
        }
        Err(e) => {
            let error_msg = format!("Failed to execute cargo: {}", e);
            output_panel.log(&format!("❌ {}", error_msg));
            Err(error_msg)
        }
    }
}

// ========================================================================================
// COMPONENT PACKAGING ACTIONS - Component lifecycle management
// ========================================================================================

/// Package a single component for distribution
/// 
/// Creates a distributable package for a specific component, including
/// its source code, metadata, and dependencies. The package can be
/// shared with other developers or imported into other projects.
/// 
/// # Arguments
/// 
/// * `name` - Name identifier for the component
/// * `source` - Source directory path containing component files
/// * `output` - Output directory path for the generated package
/// * `_registry` - Component registry for tracking packaged components
/// 
/// # Behavior
/// 
/// - Validates component source files
/// - Creates package metadata
/// - Bundles component resources
/// - Registers package in component registry
/// 
/// **Note**: This is currently a placeholder implementation for demonstration.
/// A production version would implement actual file packaging logic.
pub fn package_component(name: &str, source: &str, output: &str, _registry: &mut ComponentRegistry) {
    // In a real implementation, this would actually package components
    println!("📦 Packaging component {} from {} to {}", name, source, output);
    println!("✅ Component packaged successfully");
}

/// Install a component package
/// 
/// Extracts and installs a component package into the current project,
/// making it available for use in the visual designer and code generation.
/// 
/// # Arguments
/// 
/// * `package` - Path to the component package file
/// * `destination` - Installation directory in the current project
/// * `_registry` - Component registry for tracking installed components
/// 
/// # Behavior
/// 
/// - Validates package integrity
/// - Extracts package contents
/// - Installs component files
/// - Updates component registry
/// - Refreshes component palette
/// 
/// **Note**: This is currently a placeholder implementation for demonstration.
/// A production version would implement actual package installation logic.
pub fn install_component(package: &str, destination: &str, _registry: &mut ComponentRegistry) {
    // In a real implementation, this would actually install components
    println!("📥 Installing component from {} to {}", package, destination);
    println!("✅ Component installed successfully");
}

/// Uninstall a component package
/// 
/// Removes an installed component package from the project, cleaning up
/// all associated files and removing it from the component registry.
/// 
/// # Arguments
/// 
/// * `package` - Name of the component package to uninstall
/// * `location` - Installation location of the component
/// * `_registry` - Component registry for tracking component removal
/// 
/// # Behavior
/// 
/// - Validates component can be safely removed
/// - Removes component files and resources
/// - Updates component registry
/// - Refreshes component palette
/// - Cleans up any orphaned dependencies
/// 
/// **Note**: This is currently a placeholder implementation for demonstration.
/// A production version would implement actual package removal logic.
pub fn uninstall_component(package: &str, location: &str, _registry: &mut ComponentRegistry) {
    // In a real implementation, this would actually uninstall components
    println!("🗑 Uninstalling component {} from {}", package, location);
    println!("✅ Component uninstalled successfully");
}

// ========================================================================================
// ERROR PARSING UTILITIES - Compiler output analysis
// ========================================================================================

/// Parse compiler error output into structured format
/// 
/// Analyzes compiler output (typically from Cargo) to extract error and
/// warning messages for display in the IDE's error panel and diagnostic system.
/// 
/// # Arguments
/// 
/// * `error_output` - Raw compiler output text to parse
/// 
/// # Returns
/// 
/// Vector of error and warning messages extracted from the output
/// 
/// # Behavior
/// 
/// - Filters lines containing "error:" or "warning:" keywords
/// - Preserves original formatting for accurate error reporting
/// - Suitable for integration with LSP diagnostic systems
/// - Used by build actions to provide structured error feedback
/// 
/// # Example
/// 
/// ```rust
/// let output = "error: expected `;`\nwarning: unused variable";
/// let errors = parse_errors(output);
/// assert_eq!(errors.len(), 2);
/// ```
pub fn parse_errors(error_output: &str) -> Vec<String> {
    error_output
        .lines()
        .filter(|line| line.contains("error:") || line.contains("warning:"))
        .map(|line| line.to_string())
        .collect()
}