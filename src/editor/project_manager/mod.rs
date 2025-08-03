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
    /// UI state for new project dialog
    pub show_new_project_dialog: bool,
    pub new_project_name: String,
    pub new_project_location: String,
    pub selected_template_name: String,
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
        // Use a safe default path that always exists
        let default_project_path = if let Some(home_dir) = dirs::home_dir() {
            home_dir.join("Documents").join("IDE_Projects")
        } else {
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("projects")
        };

        // Ensure the directory exists
        if !default_project_path.exists() {
            let _ = std::fs::create_dir_all(&default_project_path);
        }

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
            show_new_project_dialog: false,
            new_project_name: String::new(),
            new_project_location: String::new(),
            selected_template_name: "GUI Application".to_string(),
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

    /// Render project management UI
    pub fn render_project_ui(&mut self, ui: &mut egui::Ui, output_panel: &mut OutputPanel) {
        ui.vertical(|ui| {
            // Project actions
            ui.horizontal(|ui| {
                if ui.button("📂 New Project").clicked() {
                    self.show_new_project_dialog = true;
                }
                if ui.button("🔄 Open Project").clicked() {
                    // TODO: Show open project dialog
                    output_panel.log("📂 Open project dialog would appear here");
                }
                if let Some(_) = &self.current_project {
                    if ui.button("💾 Save Project").clicked() {
                        self.save_current_project_ui(output_panel);
                    }
                }
            });
            
            ui.separator();
            
            // Show current project info
            if let Some(ref project) = self.current_project {
                ui.group(|ui| {
                    ui.heading("Current Project");
                    ui.label(format!("Name: {}", project.metadata.name));
                    ui.label(format!("Type: {:?}", project.metadata.project_type));
                    ui.label(format!("Components: {}", project.designer_data.components.len()));
                });
                ui.separator();
            }
            
            // File browser
            self.render_file_browser(ui, output_panel);
            
            // New project dialog
            if self.show_new_project_dialog {
                self.render_new_project_dialog(ui, output_panel);
            }
        });
    }
    
    /// Render new project creation dialog
    fn render_new_project_dialog(&mut self, ui: &mut egui::Ui, output_panel: &mut OutputPanel) {
        egui::Window::new("Create New Project")
            .collapsible(false)
            .resizable(false)
            .show(ui.ctx(), |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Project Name:");
                        ui.text_edit_singleline(&mut self.new_project_name);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Location:");
                        ui.text_edit_singleline(&mut self.new_project_location);
                        if ui.button("Browse").clicked() {
                            if let Some(folder) = rfd::FileDialog::new()
                                .set_title("Select Project Directory")
                                .pick_folder() {
                                self.new_project_location = folder.display().to_string();
                            }
                        }
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Template:");
                        egui::ComboBox::from_label("")
                            .selected_text(&self.selected_template_name)
                            .show_ui(ui, |ui| {
                                for template in &self.templates {
                                    if ui.selectable_label(self.selected_template_name == template.name, &template.name).clicked() {
                                        self.selected_template_name = template.name.clone();
                                    }
                                }
                            });
                    });
                    
                    ui.separator();
                    
                    ui.horizontal(|ui| {
                        if ui.button("Create").clicked() {
                            self.create_new_project(output_panel);
                        }
                        if ui.button("Cancel").clicked() {
                            self.show_new_project_dialog = false;
                            self.reset_new_project_dialog();
                        }
                    });
                });
            });
    }
    
    /// Create a new project with the specified settings
    fn create_new_project(&mut self, output_panel: &mut OutputPanel) {
        if self.new_project_name.trim().is_empty() {
            output_panel.log("❌ Project name cannot be empty");
            return;
        }
        
        let template = self.templates.iter()
            .find(|t| t.name == self.selected_template_name)
            .cloned()
            .unwrap_or_else(|| self.templates[0].clone());
        
        let location = if self.new_project_location.trim().is_empty() {
            self.settings.default_project_path.clone()
        } else {
            std::path::PathBuf::from(&self.new_project_location)
        };
        
        match self.operations.create_project(&self.new_project_name, &template, &location, output_panel) {
            Ok(project) => {
                self.current_project = Some(project);
                self.show_new_project_dialog = false;
                self.reset_new_project_dialog();
                output_panel.log(&format!("✅ Project '{}' created successfully!", self.new_project_name));
            }
            Err(e) => {
                output_panel.log(&format!("❌ Failed to create project: {}", e));
            }
        }
    }
    
    /// Reset new project dialog fields
    fn reset_new_project_dialog(&mut self) {
        self.new_project_name.clear();
        self.new_project_location.clear();
        self.selected_template_name = if !self.templates.is_empty() {
            self.templates[0].name.clone()
        } else {
            "Default".to_string()
        };
    }

    /// Save the current project with UI feedback
    pub fn save_current_project_ui(&self, output_panel: &mut OutputPanel) {
        if let Some(ref project) = self.current_project {
            match self.save_project(project, output_panel) {
                Ok(_) => output_panel.log("💾 Project saved successfully!"),
                Err(e) => output_panel.log(&format!("❌ Failed to save project: {}", e)),
            }
        } else {
            output_panel.log("❌ No project to save");
        }
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