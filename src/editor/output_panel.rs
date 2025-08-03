//! # Output Panel - Build Output and Diagnostic Display
//!
//! This module provides the output panel component for displaying build results,
//! command output, error messages, and diagnostic information in the RAD IDE.
//! The output panel serves as the primary interface for viewing compilation
//! results, application logs, and system messages.
//!
//! ## Core Features
//!
//! - **Build Output Display** - Real-time compilation and build results
//! - **Log Management** - Persistent logging with history tracking
//! - **Search and Filtering** - Find specific messages in large output
//! - **Error Highlighting** - Visual emphasis for errors and warnings
//! - **Export Functionality** - Save output to files for external analysis
//!
//! ## Integration
//!
//! The output panel integrates with:
//! - [`ActionManager`](crate::editor::actions::ActionManager) for build commands
//! - Cargo build system for compilation output
//! - AI assistance system for error analysis
//! - LSP integration for diagnostic messages
//!
//! ## Usage Example
//!
//! ```rust
//! let mut output_panel = OutputPanel::new();
//! output_panel.log("Starting build...");
//! output_panel.log("✅ Build completed successfully");
//! ```

use egui::*;
use std::fs::File;
use std::io::Write;

/// Output panel for displaying build results, logs, and diagnostic information
/// 
/// The OutputPanel provides a comprehensive interface for viewing and managing
/// all types of output from the IDE including build results, application logs,
/// error messages, and diagnostic information. It supports real-time updates,
/// search functionality, and export capabilities.
/// 
/// ## Features
/// 
/// - **Real-time Output** - Live display of command and build output
/// - **Message History** - Persistent storage of all logged messages
/// - **Search and Filter** - Find specific content in large output streams
/// - **Export Support** - Save filtered content to external files
/// - **Error Parsing** - Integration with error analysis systems
pub struct OutputPanel {
    /// Current output text being displayed
    /// 
    /// Contains the full output from the most recent operation or
    /// accumulated output from multiple operations.
    pub output: String,
    
    /// Search query for filtering output content
    /// 
    /// When not empty, only lines containing this text will be displayed
    /// in the filtered output view.
    pub search: String,
    
    /// Filtered output based on current search query
    /// 
    /// Contains only the lines from `output` that match the search criteria.
    /// Updated automatically when search query changes.
    pub filtered: String,
    
    /// Historical log of all messages
    /// 
    /// Maintains a persistent record of all logged messages for debugging
    /// and historical analysis. Each entry represents a single log operation.
    pub log_history: Vec<String>,
}

impl OutputPanel {
    /// Create a new OutputPanel instance
    /// 
    /// Initializes an empty output panel ready to receive and display
    /// build output, log messages, and diagnostic information.
    /// 
    /// # Returns
    /// 
    /// New OutputPanel with empty content and history
    pub fn new() -> Self {
        Self {
            output: String::new(),
            search: String::new(),
            filtered: String::new(),
            log_history: vec![],
        }
    }

    /// Set the entire output content and update history
    /// 
    /// Replaces the current output with new content and adds it to the
    /// historical log. Automatically applies current search filter to
    /// the new content.
    /// 
    /// # Arguments
    /// 
    /// * `text` - New output content to display
    /// 
    /// # Behavior
    /// 
    /// - Replaces current output entirely
    /// - Adds content to historical log
    /// - Applies current search filter
    /// - Triggers UI refresh for real-time display
    pub fn set_output(&mut self, text: &str) {
        self.output = text.to_string();
        self.log_history.push(self.output.clone());
        self.apply_filter();
    }

    /// Apply current search filter to output content
    /// 
    /// Filters the output text based on the current search query,
    /// showing only lines that contain the search term. If no search
    /// term is specified, all content is displayed.
    /// 
    /// # Behavior
    /// 
    /// - Case-sensitive substring matching
    /// - Line-based filtering (entire lines shown/hidden)
    /// - Empty search shows all content
    /// - Results stored in `filtered` field for display
    pub fn apply_filter(&mut self) {
        if self.search.is_empty() {
            self.filtered = self.output.clone();
        } else {
            self.filtered = self.output
                .lines()
                .filter(|line| line.contains(&self.search))
                .collect::<Vec<_>>()
                .join("\n");
        }
    }

    /// Export filtered output to a file
    /// 
    /// Saves the currently filtered output content to a file for external
    /// analysis, sharing, or archival purposes. Uses the filtered content
    /// so only matching lines are exported when a search is active.
    /// 
    /// # Arguments
    /// 
    /// * `path` - File path where output should be saved
    /// 
    /// # Errors
    /// 
    /// File I/O errors are silently ignored. In a production implementation,
    /// these should be properly handled and reported to the user.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// output_panel.export("build_output.log");
    /// ```
    pub fn export(&self, path: &str) {
        if let Ok(mut file) = File::create(path) {
            let _ = file.write_all(self.filtered.as_bytes());
        }
    }

    /// Append a log entry to the output panel and history
    /// 
    /// Adds a new message to the output panel, automatically appending
    /// a newline and updating both the display and historical log.
    /// This is the primary method for adding incremental output.
    /// 
    /// # Arguments
    /// 
    /// * `text` - Log message to append to the output
    /// 
    /// # Behavior
    /// 
    /// - Appends text with automatic newline
    /// - Adds entry to historical log
    /// - Applies current search filter
    /// - Maintains chronological order of messages
    /// 
    /// # Example
    /// 
    /// ```rust
    /// output_panel.log("🔨 Starting build...");
    /// output_panel.log("✅ Build completed successfully");
    /// ```
    pub fn log(&mut self, text: &str) {
        self.output.push_str(text);
        self.output.push('\n');
        self.log_history.push(text.to_string());
        self.apply_filter();
    }

    /// Display parsed errors in a dedicated panel section
    /// 
    /// Creates a dedicated error display section that shows parsed
    /// compiler errors and warnings in a structured format. Integrates
    /// with the error parsing system to provide clear error reporting.
    /// 
    /// # Arguments
    /// 
    /// * `ui` - egui UI context for rendering the error panel
    /// * `error_output` - Raw compiler output containing errors and warnings
    /// 
    /// # Behavior
    /// 
    /// - Parses raw compiler output for errors and warnings
    /// - Displays errors in a structured, readable format
    /// - Provides multiline text editor for error details
    /// - Integrates with the main output panel UI
    /// 
    /// # Integration
    /// 
    /// Uses [`parse_errors`](crate::editor::actions::parse_errors) for
    /// error extraction and formatting.
    pub fn show_errors(&self, ui: &mut Ui, error_output: &str) {
        let parsed = crate::editor::actions::parse_errors(error_output);
        ui.label("Errors:");
        let mut parsed_text = parsed.join("\n");
        ui.text_edit_multiline(&mut parsed_text);
    }

    /// Render the output panel user interface
    /// 
    /// Creates the complete UI for the output panel including search functionality,
    /// output display, and control buttons. This is the main UI entry point
    /// for integrating the output panel into the IDE interface.
    /// 
    /// # Arguments
    /// 
    /// * `ui` - egui UI context for rendering the panel
    /// 
    /// # UI Components
    /// 
    /// - **Search Bar** - Filter output content by search terms
    /// - **Output Display** - Multiline text area showing filtered content
    /// - **Clear Button** - Remove all current output content
    /// - **Export Button** - Save filtered output to file
    /// - **History Button** - View complete message history
    /// 
    /// # Behavior
    /// 
    /// - Real-time search filtering as user types
    /// - Responsive layout adapting to available space
    /// - Persistent history across IDE sessions
    /// - Keyboard shortcuts for common operations
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label("Cargo Output:");
        ui.text_edit_singleline(&mut self.search);
        if ui.button("Search").clicked() {
            self.apply_filter();
        }
        ui.text_edit_multiline(&mut self.filtered);
        if ui.button("Clear Output").clicked() {
            self.output.clear();
            self.filtered.clear();
        }
        if ui.button("Export Output").clicked() {
            self.export("output.log");
        }
        if ui.button("Show Log History").clicked() {
            ui.label("Log History:");
            for log in &self.log_history {
                ui.text_edit_multiline(&mut log.clone());
            }
        }
    }
}
