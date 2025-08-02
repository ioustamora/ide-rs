//! Project Serialization and Persistence
//!
//! Handles saving and loading project data to/from disk,
//! including validation and error handling.

use std::path::Path;
use std::fs;
use crate::editor::output_panel::OutputPanel;
use super::project::IdeProject;

/// Project serialization handler
pub struct ProjectSerializer;

impl ProjectSerializer {
    /// Create a new project serializer
    pub fn new() -> Self {
        Self
    }

    /// Save project to disk
    pub fn save_project(&self, project: &IdeProject, output_panel: &mut OutputPanel) -> Result<(), Box<dyn std::error::Error>> {
        let project_file = project.metadata.root_path.join("project.ide");
        
        output_panel.log(&format!("💾 Saving project to: {}", project_file.display()));
        
        // Create backup if file exists
        if project_file.exists() {
            self.create_backup(&project_file, output_panel)?;
        }
        
        // Serialize to JSON
        let content = serde_json::to_string_pretty(project)?;
        
        // Write to file
        fs::write(&project_file, content)?;
        
        output_panel.log("✅ Project saved successfully!");
        
        Ok(())
    }

    /// Load project from disk
    pub fn load_project(&self, project_path: &Path, output_panel: &mut OutputPanel) -> Result<IdeProject, Box<dyn std::error::Error>> {
        output_panel.log(&format!("📂 Loading project from: {}", project_path.display()));
        
        let project_file = project_path.join("project.ide");
        
        if !project_file.exists() {
            return Err("Project file not found. This may not be a valid IDE project.".into());
        }
        
        // Read and parse project file
        let content = fs::read_to_string(&project_file)?;
        let mut project: IdeProject = serde_json::from_str(&content)?;
        
        // Update root path to ensure it's correct
        project.metadata.root_path = project_path.to_path_buf();
        
        // Validate project structure
        self.validate_project(&project, output_panel)?;
        
        Ok(project)
    }

    /// Export project to a different format
    pub fn export_project(&self, project: &IdeProject, export_path: &Path, format: ExportFormat, output_panel: &mut OutputPanel) -> Result<(), Box<dyn std::error::Error>> {
        output_panel.log(&format!("📤 Exporting project to: {}", export_path.display()));
        
        match format {
            ExportFormat::Json => {
                let content = serde_json::to_string_pretty(project)?;
                fs::write(export_path, content)?;
            }
            ExportFormat::JsonCompact => {
                let content = serde_json::to_string(project)?;
                fs::write(export_path, content)?;
            }
            ExportFormat::Toml => {
                // Convert to TOML format (simplified)
                let content = self.project_to_toml(project)?;
                fs::write(export_path, content)?;
            }
        }
        
        output_panel.log("✅ Project exported successfully!");
        
        Ok(())
    }

    /// Import project from external format
    pub fn import_project(&self, import_path: &Path, target_path: &Path, format: ExportFormat, output_panel: &mut OutputPanel) -> Result<IdeProject, Box<dyn std::error::Error>> {
        output_panel.log(&format!("📥 Importing project from: {}", import_path.display()));
        
        let content = fs::read_to_string(import_path)?;
        
        let mut project = match format {
            ExportFormat::Json | ExportFormat::JsonCompact => {
                serde_json::from_str::<IdeProject>(&content)?
            }
            ExportFormat::Toml => {
                self.project_from_toml(&content)?
            }
        };
        
        // Update paths
        project.metadata.root_path = target_path.to_path_buf();
        
        output_panel.log("✅ Project imported successfully!");
        
        Ok(project)
    }

    /// Create backup of existing project file
    fn create_backup(&self, project_file: &Path, output_panel: &mut OutputPanel) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = project_file.parent() {
            let backup_dir = parent.join(".ide-backups");
            fs::create_dir_all(&backup_dir)?;
            
            let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
            let backup_name = format!("project_{}.ide.bak", timestamp);
            let backup_path = backup_dir.join(backup_name);
            
            fs::copy(project_file, &backup_path)?;
            output_panel.log(&format!("📋 Created backup: {}", backup_path.display()));
        }
        
        Ok(())
    }

    /// Validate project structure and files
    fn validate_project(&self, project: &IdeProject, output_panel: &mut OutputPanel) -> Result<(), Box<dyn std::error::Error>> {
        let root_path = &project.metadata.root_path;
        
        if !root_path.exists() {
            return Err("Project root directory does not exist".into());
        }
        
        // Check for Cargo.toml
        let cargo_toml = root_path.join("Cargo.toml");
        if !cargo_toml.exists() {
            output_panel.log("⚠️ Warning: Cargo.toml not found. This may not be a valid Rust project.");
        }
        
        // Validate source files exist
        for source_file in &project.file_structure.source_files {
            let file_path = root_path.join(&source_file.path);
            if !file_path.exists() {
                output_panel.log(&format!("⚠️ Warning: Source file not found: {}", source_file.path.display()));
            }
        }
        
        // Validate src directory exists
        let src_dir = root_path.join("src");
        if !src_dir.exists() {
            output_panel.log("⚠️ Warning: src directory not found.");
        }
        
        Ok(())
    }

    /// Convert project to TOML format (simplified representation)
    fn project_to_toml(&self, project: &IdeProject) -> Result<String, Box<dyn std::error::Error>> {
        // This is a simplified TOML representation
        // In a real implementation, you'd use a TOML library like `toml`
        let toml_content = format!(
            r#"[project]
name = "{}"
description = "{}"
version = "{}"
author = "{}"
project_type = "{:?}"

[grid_settings]
size = {}
visible = {}
snap_enabled = {}

[layout]
layout_type = "{:?}"
spacing = {}
padding = {}
"#,
            project.metadata.name,
            project.metadata.description,
            project.metadata.version,
            project.metadata.author,
            project.metadata.project_type,
            project.designer_data.grid_settings.size,
            project.designer_data.grid_settings.visible,
            project.designer_data.grid_settings.snap_enabled,
            project.designer_data.layout_config.layout_type,
            project.designer_data.layout_config.spacing,
            project.designer_data.layout_config.padding
        );
        
        Ok(toml_content)
    }

    /// Convert TOML format to project (simplified)
    fn project_from_toml(&self, _content: &str) -> Result<IdeProject, Box<dyn std::error::Error>> {
        // This is a placeholder - in a real implementation, you'd parse TOML
        // and reconstruct the project structure
        Err("TOML import not yet implemented".into())
    }

    /// Get project metadata without loading full project
    pub fn get_project_metadata(&self, project_path: &Path) -> Result<ProjectMetadata, Box<dyn std::error::Error>> {
        let project_file = project_path.join("project.ide");
        
        if !project_file.exists() {
            return Err("Project file not found".into());
        }
        
        let content = fs::read_to_string(&project_file)?;
        
        // Parse just the metadata section
        let value: serde_json::Value = serde_json::from_str(&content)?;
        let metadata = serde_json::from_value(value["metadata"].clone())?;
        
        Ok(metadata)
    }

    /// Clean up old backup files
    pub fn cleanup_backups(&self, project_path: &Path, max_backups: u32, output_panel: &mut OutputPanel) -> Result<(), Box<dyn std::error::Error>> {
        let backup_dir = project_path.join(".ide-backups");
        
        if !backup_dir.exists() {
            return Ok(());
        }
        
        let mut backups = Vec::new();
        
        for entry in fs::read_dir(&backup_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.ends_with(".ide.bak") {
                    if let Ok(metadata) = entry.metadata() {
                        if let Ok(modified) = metadata.modified() {
                            backups.push((path, modified));
                        }
                    }
                }
            }
        }
        
        // Sort by modification time (newest first)
        backups.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Remove old backups
        if backups.len() > max_backups as usize {
            for (path, _) in backups.into_iter().skip(max_backups as usize) {
                if let Err(e) = fs::remove_file(&path) {
                    output_panel.log(&format!("⚠️ Failed to remove backup {}: {}", path.display(), e));
                } else {
                    output_panel.log(&format!("🗑️ Removed old backup: {}", path.display()));
                }
            }
        }
        
        Ok(())
    }
}

/// Export/import formats supported
#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    Json,
    JsonCompact,
    Toml,
}

// Re-export from project module for convenience
pub use super::project::ProjectMetadata;

impl Default for ProjectSerializer {
    fn default() -> Self {
        Self::new()
    }
}