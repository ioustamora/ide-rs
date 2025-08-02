//! Project Management System
//!
//! Comprehensive project lifecycle management for the RAD IDE, handling everything from
//! project creation to deployment. This system provides a unified interface for managing
//! Rust projects with integrated visual design data and build configuration.
//!
//! ## Architecture
//! 
//! The project manager is organized into focused modules:
//! - **project**: Core project structures and metadata
//! - **templates**: Project templates and scaffolding
//! - **file_browser**: File system navigation and management
//! - **serialization**: Project serialization and persistence
//! - **operations**: High-level project operations (create, load, save)

pub mod project;
pub mod templates;
pub mod file_browser;
pub mod serialization;
pub mod operations;

use std::path::{Path, PathBuf};
use crate::editor::output_panel::OutputPanel;
use crate::rcl::ui::component::Component;

// Re-export main types
pub use project::{IdeProject, ProjectMetadata, DesignerData, ComponentData, 
                 ProjectFileStructure, BuildConfiguration, ProjectType};
pub use templates::{ProjectTemplate, TemplateFile, TemplateConfig};
pub use file_browser::{FileBrowser, FileFilters, FileViewSettings};
pub use serialization::ProjectSerializer;
pub use operations::ProjectOperations;

/// Main project manager handling all project operations
/// 
/// This is a much lighter coordinator compared to the original monolithic structure.
pub struct ProjectManager {
    /// Currently active project
    pub current_project: Option<IdeProject>,
    /// Recent projects list
    pub recent_projects: Vec<PathBuf>,
    /// Project templates available
    pub templates: Vec<ProjectTemplate>,
    /// File system browser state
    pub file_browser: FileBrowser,
    /// Project settings
    pub settings: ProjectSettings,
    /// Project operations handler
    pub operations: ProjectOperations,
    /// Project serializer
    pub serializer: ProjectSerializer,
}

/// Project manager settings
pub struct ProjectSettings {
    /// Auto-save interval in seconds
    pub auto_save_interval: u64,
    /// Maximum recent projects to remember
    pub max_recent_projects: usize,
    /// Default project location
    pub default_project_path: PathBuf,
    /// Backup settings
    pub backup_settings: BackupSettings,
}

/// Backup configuration
pub struct BackupSettings {
    /// Enable automatic backups
    pub enabled: bool,
    /// Backup interval in minutes
    pub interval_minutes: u32,
    /// Number of backups to keep
    pub max_backups: u32,
    /// Backup directory
    pub backup_dir: PathBuf,
}

impl ProjectManager {
    /// Create a new project manager
    pub fn new() -> Self {
        let default_project_path = std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("projects");

        let backup_dir = default_project_path.join(".ide-backups");

        Self {
            current_project: None,
            recent_projects: Vec::new(),
            templates: templates::create_default_templates(),
            file_browser: FileBrowser::new(&default_project_path),
            settings: ProjectSettings {
                auto_save_interval: 300, // 5 minutes
                max_recent_projects: 10,
                default_project_path,
                backup_settings: BackupSettings {
                    enabled: true,
                    interval_minutes: 30,
                    max_backups: 10,
                    backup_dir,
                },
            },
            operations: ProjectOperations::new(),
            serializer: ProjectSerializer::new(),
        }
    }

    /// Create a new project from template
    pub fn create_project(&mut self, name: &str, template: &ProjectTemplate, location: &Path, output_panel: &mut OutputPanel) -> Result<(), Box<dyn std::error::Error>> {
        let project = self.operations.create_project(name, template, location, output_panel)?;
        
        // Save project file
        self.serializer.save_project(&project, output_panel)?;
        
        // Set as current project
        self.current_project = Some(project);
        
        // Add to recent projects
        self.add_to_recent_projects(&location.join(name));
        
        Ok(())
    }

    /// Load an existing project
    pub fn load_project(&mut self, project_path: &Path, output_panel: &mut OutputPanel) -> Result<(), Box<dyn std::error::Error>> {
        let project = self.serializer.load_project(project_path, output_panel)?;
        
        // Update current directory
        self.file_browser.set_current_dir(&project.metadata.root_path);
        
        // Set as current project
        self.current_project = Some(project);
        
        // Add to recent projects
        self.add_to_recent_projects(project_path);
        
        output_panel.log(&format!("✅ Project '{}' loaded successfully!", self.current_project.as_ref().unwrap().metadata.name));
        
        Ok(())
    }

    /// Save the current project
    pub fn save_project(&self, project: &IdeProject, output_panel: &mut OutputPanel) -> Result<(), Box<dyn std::error::Error>> {
        self.serializer.save_project(project, output_panel)
    }

    /// Add project to recent projects list
    fn add_to_recent_projects(&mut self, project_path: &Path) {
        let path_buf = project_path.to_path_buf();
        
        // Remove if already exists
        self.recent_projects.retain(|p| p != &path_buf);
        
        // Add to front
        self.recent_projects.insert(0, path_buf);
        
        // Limit size
        if self.recent_projects.len() > self.settings.max_recent_projects {
            self.recent_projects.truncate(self.settings.max_recent_projects);
        }
    }

    /// Get list of recent projects
    pub fn get_recent_projects(&self) -> &[PathBuf] {
        &self.recent_projects
    }

    /// Get available project templates
    pub fn get_templates(&self) -> &[ProjectTemplate] {
        &self.templates
    }

    /// Update current project with new component data
    pub fn update_project_components(&mut self, components: &[Box<dyn Component>]) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref mut project) = self.current_project {
            self.operations.update_project_components(project, components)?;
        }
        
        Ok(())
    }

    /// Get current project reference
    pub fn get_current_project(&self) -> Option<&IdeProject> {
        self.current_project.as_ref()
    }

    /// Check if a project is currently loaded
    pub fn has_current_project(&self) -> bool {
        self.current_project.is_some()
    }

    /// Create a new file browser panel
    pub fn render_file_browser(&mut self, ui: &mut egui::Ui, output_panel: &mut OutputPanel) {
        self.file_browser.render(ui, output_panel, &self.current_project);
    }

    /// Save the current project (placeholder implementation)
    pub fn save_current_project(&self) {
        // Placeholder implementation - would save the current project
        // In a real implementation, this would call save_project with the current project
    }

    /// Show save project as dialog (placeholder implementation)
    pub fn save_project_as_dialog(&self) {
        // Placeholder implementation - would show a file dialog to save project as
        // In a real implementation, this would open a file picker dialog
    }

    /// Close the current project (placeholder implementation)
    pub fn close_current_project(&mut self) {
        // Placeholder implementation - would close the current project
        self.current_project = None;
    }

    /// Create a new project (placeholder implementation)
    pub fn create_new_project(&mut self) {
        // Placeholder implementation - would create a new project
        // In a real implementation, this would open a project creation dialog
    }

    /// Open project dialog (placeholder implementation)
    pub fn open_project_dialog(&mut self) {
        // Placeholder implementation - would open a file dialog to select project
        // In a real implementation, this would open a file picker dialog
    }
}

impl Default for ProjectManager {
    fn default() -> Self {
        Self::new()
    }
}