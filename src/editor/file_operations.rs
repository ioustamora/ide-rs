//! File Operations module for the RAD IDE
//!
//! This module provides comprehensive file and project management capabilities
//! including saving, loading, creating, and managing project files.

use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};
use crate::rcl::ui::component::Component;

/// Project file format version for compatibility
const PROJECT_FORMAT_VERSION: u32 = 1;

/// Complete project data structure for serialization
#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectData {
    pub version: u32,
    pub metadata: ProjectMetadata,
    pub components: Vec<SerializableComponent>,
    pub settings: ProjectSettings,
    pub files: Vec<ProjectFile>,
    pub dependencies: Vec<String>,
}

/// Project metadata
#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectMetadata {
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,
    pub created_at: String,
    pub last_modified: String,
    pub project_type: String,
}

/// Serializable component representation
#[derive(Serialize, Deserialize, Clone)]
pub struct SerializableComponent {
    pub component_type: String,
    pub name: String,
    pub properties: std::collections::HashMap<String, String>,
    pub position: Option<(f32, f32)>,
    pub size: Option<(f32, f32)>,
    pub id: String,
}

/// Project-specific settings
#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectSettings {
    pub editor_theme: String,
    pub font_size: f32,
    pub auto_save: bool,
    pub show_line_numbers: bool,
    pub tab_size: usize,
    pub target_platform: String,
}

/// Individual project file
#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectFile {
    pub path: String,
    pub content: String,
    pub file_type: String,
    pub last_modified: String,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            editor_theme: "dark".to_string(),
            font_size: 14.0,
            auto_save: true,
            show_line_numbers: true,
            tab_size: 4,
            target_platform: "desktop".to_string(),
        }
    }
}

/// File operations manager
pub struct FileOperations {
    current_project_path: Option<PathBuf>,
    auto_save_enabled: bool,
    backup_enabled: bool,
}

impl Default for FileOperations {
    fn default() -> Self {
        Self {
            current_project_path: None,
            auto_save_enabled: true,
            backup_enabled: true,
        }
    }
}

impl FileOperations {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new project with the given name and template
    pub fn create_project(&mut self, name: &str, location: &Path, template: &str) -> Result<ProjectData> {
        let project_path = location.join(name);
        
        // Create project directory
        fs::create_dir_all(&project_path)
            .context("Failed to create project directory")?;

        // Create project metadata
        let metadata = ProjectMetadata {
            name: name.to_string(),
            description: format!("A new {} project", template),
            author: whoami::username(),
            version: "0.1.0".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            last_modified: chrono::Utc::now().to_rfc3339(),
            project_type: template.to_string(),
        };

        // Create initial project structure
        let project_data = ProjectData {
            version: PROJECT_FORMAT_VERSION,
            metadata,
            components: Vec::new(),
            settings: ProjectSettings::default(),
            files: self.create_initial_files(template)?,
            dependencies: self.get_template_dependencies(template),
        };

        // Save project file
        self.save_project(&project_data, &project_path)?;
        self.current_project_path = Some(project_path);

        Ok(project_data)
    }

    /// Load an existing project from the given path
    pub fn load_project(&mut self, project_path: &Path) -> Result<ProjectData> {
        let project_file = project_path.join("project.json");
        
        if !project_file.exists() {
            // Try to find .radide file as alternative
            let alt_file = project_path.join(".radide");
            if alt_file.exists() {
                return self.load_project_from_file(&alt_file);
            }
            return Err(anyhow::anyhow!("Project file not found"));
        }

        self.load_project_from_file(&project_file)
    }

    /// Load project from a specific file
    fn load_project_from_file(&mut self, file_path: &Path) -> Result<ProjectData> {
        let content = fs::read_to_string(file_path)
            .context("Failed to read project file")?;

        let project_data: ProjectData = serde_json::from_str(&content)
            .context("Failed to parse project file")?;

        // Validate project format version
        if project_data.version > PROJECT_FORMAT_VERSION {
            return Err(anyhow::anyhow!(
                "Project format version {} is newer than supported version {}",
                project_data.version,
                PROJECT_FORMAT_VERSION
            ));
        }

        self.current_project_path = Some(file_path.parent().unwrap().to_path_buf());
        Ok(project_data)
    }

    /// Save project to the specified location
    pub fn save_project(&self, project_data: &ProjectData, project_path: &Path) -> Result<()> {
        // Create backup if enabled
        if self.backup_enabled {
            self.create_backup(project_path)?;
        }

        // Update last modified timestamp
        let mut updated_data = project_data.clone();
        updated_data.metadata.last_modified = chrono::Utc::now().to_rfc3339();

        // Serialize project data
        let json_content = serde_json::to_string_pretty(&updated_data)
            .context("Failed to serialize project data")?;

        // Save to project.json
        let project_file = project_path.join("project.json");
        fs::write(&project_file, json_content)
            .context("Failed to write project file")?;

        // Save individual source files
        for file in &updated_data.files {
            let file_path = project_path.join(&file.path);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)
                    .context("Failed to create file directory")?;
            }
            fs::write(&file_path, &file.content)
                .context("Failed to write source file")?;
        }

        // Create Cargo.toml if it's a Rust project
        if updated_data.metadata.project_type == "rust_gui" || updated_data.metadata.project_type == "rust_lib" {
            self.create_cargo_toml(&updated_data, project_path)?;
        }

        Ok(())
    }

    /// Save a single file
    pub fn save_file(&self, file_path: &Path, content: &str) -> Result<()> {
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create file directory")?;
        }

        fs::write(file_path, content)
            .context("Failed to write file")?;

        Ok(())
    }

    /// Load content from a file
    pub fn load_file(&self, file_path: &Path) -> Result<String> {
        fs::read_to_string(file_path)
            .context("Failed to read file")
    }

    /// Export project as a standalone Rust project
    pub fn export_rust_project(&self, project_data: &ProjectData, export_path: &Path) -> Result<()> {
        // Create export directory structure
        fs::create_dir_all(export_path)?;
        fs::create_dir_all(export_path.join("src"))?;

        // Create Cargo.toml
        self.create_cargo_toml(project_data, export_path)?;

        // Create main.rs or lib.rs
        let main_content = self.generate_main_rust_code(project_data)?;
        let main_file = if project_data.metadata.project_type == "rust_lib" {
            "lib.rs"
        } else {
            "main.rs"
        };
        fs::write(export_path.join("src").join(main_file), main_content)?;

        // Copy additional source files
        for file in &project_data.files {
            if file.file_type == "rust" {
                let file_path = export_path.join("src").join(&file.path);
                if let Some(parent) = file_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(file_path, &file.content)?;
            }
        }

        // Create README.md
        let readme_content = format!(
            "# {}\n\n{}\n\nGenerated by RAD IDE.\n\n## Usage\n\n```bash\ncargo run\n```\n",
            project_data.metadata.name,
            project_data.metadata.description
        );
        fs::write(export_path.join("README.md"), readme_content)?;

        Ok(())
    }

    /// Create initial files based on template
    fn create_initial_files(&self, template: &str) -> Result<Vec<ProjectFile>> {
        let mut files = Vec::new();

        match template {
            "rust_gui" => {
                files.push(ProjectFile {
                    path: "main.rs".to_string(),
                    content: include_str!("templates/main_gui.rs").to_string(),
                    file_type: "rust".to_string(),
                    last_modified: chrono::Utc::now().to_rfc3339(),
                });
            }
            "rust_lib" => {
                files.push(ProjectFile {
                    path: "lib.rs".to_string(),
                    content: include_str!("templates/lib.rs").to_string(),
                    file_type: "rust".to_string(),
                    last_modified: chrono::Utc::now().to_rfc3339(),
                });
            }
            "empty" => {
                files.push(ProjectFile {
                    path: "main.rs".to_string(),
                    content: "fn main() {\n    println!(\"Hello, world!\");\n}\n".to_string(),
                    file_type: "rust".to_string(),
                    last_modified: chrono::Utc::now().to_rfc3339(),
                });
            }
            _ => {
                return Err(anyhow::anyhow!("Unknown template: {}", template));
            }
        }

        Ok(files)
    }

    /// Get dependencies for a template
    fn get_template_dependencies(&self, template: &str) -> Vec<String> {
        match template {
            "rust_gui" => vec![
                "eframe = \"0.27\"".to_string(),
                "egui = \"0.27\"".to_string(),
                "serde = { version = \"1.0\", features = [\"derive\"] }".to_string(),
                "anyhow = \"1.0\"".to_string(),
            ],
            "rust_lib" => vec![
                "serde = { version = \"1.0\", features = [\"derive\"] }".to_string(),
                "anyhow = \"1.0\"".to_string(),
            ],
            _ => Vec::new(),
        }
    }

    /// Create Cargo.toml file
    fn create_cargo_toml(&self, project_data: &ProjectData, project_path: &Path) -> Result<()> {
        let mut cargo_content = format!(
            "[package]\n\
             name = \"{}\"\n\
             version = \"{}\"\n\
             edition = \"2021\"\n\
             authors = [\"{}\"]\n\
             description = \"{}\"\n\n\
             [dependencies]\n",
            project_data.metadata.name.replace(" ", "-").to_lowercase(),
            project_data.metadata.version,
            project_data.metadata.author,
            project_data.metadata.description
        );

        for dep in &project_data.dependencies {
            cargo_content.push_str(&format!("{}\n", dep));
        }

        // Add dev dependencies for testing
        cargo_content.push_str("\n[dev-dependencies]\n");
        cargo_content.push_str("tokio = { version = \"1.0\", features = [\"macros\", \"rt\"] }\n");

        fs::write(project_path.join("Cargo.toml"), cargo_content)
            .context("Failed to create Cargo.toml")?;

        Ok(())
    }

    /// Generate main Rust code from project data
    fn generate_main_rust_code(&self, project_data: &ProjectData) -> Result<String> {
        let mut code = String::new();

        // Add header comment
        code.push_str(&format!(
            "//! {}\n//!\n//! {}\n//! Generated by RAD IDE\n\n",
            project_data.metadata.name,
            project_data.metadata.description
        ));

        // Add imports based on project type
        if project_data.metadata.project_type == "rust_gui" {
            code.push_str("use eframe::egui;\n\n");
            
            // Generate struct for the app
            code.push_str("#[derive(Default)]\n");
            code.push_str("struct MyApp {\n");
            
            // Add fields for each component
            for (i, component) in project_data.components.iter().enumerate() {
                match component.component_type.as_str() {
                    "Button" => code.push_str(&format!("    button_{}_clicked: bool,\n", i)),
                    "TextBox" => code.push_str(&format!("    textbox_{}_value: String,\n", i)),
                    "Checkbox" => code.push_str(&format!("    checkbox_{}_checked: bool,\n", i)),
                    "Slider" => code.push_str(&format!("    slider_{}_value: f32,\n", i)),
                    _ => {}
                }
            }
            
            code.push_str("}\n\n");

            // Generate eframe::App implementation
            code.push_str("impl eframe::App for MyApp {\n");
            code.push_str("    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {\n");
            code.push_str("        egui::CentralPanel::default().show(ctx, |ui| {\n");
            code.push_str(&format!("            ui.heading(\"{}\");\n", project_data.metadata.name));
            code.push_str("            ui.separator();\n\n");

            // Generate UI for each component
            for (i, component) in project_data.components.iter().enumerate() {
                match component.component_type.as_str() {
                    "Button" => {
                        code.push_str(&format!(
                            "            if ui.button(\"{}\").clicked() {{\n",
                            component.properties.get("label").unwrap_or(&format!("Button {}", i))
                        ));
                        code.push_str(&format!("                self.button_{}_clicked = !self.button_{}_clicked;\n", i, i));
                        code.push_str("            }\n");
                        code.push_str(&format!("            if self.button_{}_clicked {{\n", i));
                        code.push_str("                ui.label(\"Button was clicked!\");\n");
                        code.push_str("            }\n\n");
                    }
                    "TextBox" => {
                        code.push_str(&format!("            ui.text_edit_singleline(&mut self.textbox_{}_value);\n\n", i));
                    }
                    "Checkbox" => {
                        let default_label = format!("Checkbox {}", i);
                        let label = component.properties.get("label").unwrap_or(&default_label);
                        code.push_str(&format!("            ui.checkbox(&mut self.checkbox_{}_checked, \"{}\");\n\n", i, label));
                    }
                    "Slider" => {
                        code.push_str(&format!("            ui.add(egui::Slider::new(&mut self.slider_{}_value, 0.0..=100.0).text(\"Slider {}\"));\n\n", i, i));
                    }
                    _ => {}
                }
            }

            code.push_str("        });\n");
            code.push_str("    }\n");
            code.push_str("}\n\n");

            // Generate main function
            code.push_str("fn main() -> Result<(), eframe::Error> {\n");
            code.push_str("    let options = eframe::NativeOptions::default();\n");
            code.push_str(&format!("    eframe::run_native(\n        \"{}\",\n        options,\n        Box::new(|_cc| Box::new(MyApp::default())),\n    )\n", project_data.metadata.name));
            code.push_str("}\n");
        } else {
            // Library project
            code.push_str("pub fn hello() -> String {\n");
            code.push_str(&format!("    \"Hello from {}!\".to_string()\n", project_data.metadata.name));
            code.push_str("}\n\n");
            
            code.push_str("#[cfg(test)]\n");
            code.push_str("mod tests {\n");
            code.push_str("    use super::*;\n\n");
            code.push_str("    #[test]\n");
            code.push_str("    fn test_hello() {\n");
            code.push_str("        assert!(!hello().is_empty());\n");
            code.push_str("    }\n");
            code.push_str("}\n");
        }

        Ok(code)
    }

    /// Create backup of existing project
    fn create_backup(&self, project_path: &Path) -> Result<()> {
        let project_file = project_path.join("project.json");
        if project_file.exists() {
            let backup_name = format!(
                "project.backup.{}.json",
                chrono::Utc::now().format("%Y%m%d_%H%M%S")
            );
            let backup_path = project_path.join("backups").join(backup_name);
            
            if let Some(parent) = backup_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            fs::copy(&project_file, &backup_path)
                .context("Failed to create backup")?;
        }
        Ok(())
    }

    /// Get recent projects from a config file
    pub fn get_recent_projects(&self) -> Result<Vec<PathBuf>> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("rad-ide");
        
        let recent_file = config_dir.join("recent_projects.json");
        
        if !recent_file.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(recent_file)?;
        let paths: Vec<String> = serde_json::from_str(&content)?;
        
        Ok(paths.into_iter()
            .map(PathBuf::from)
            .filter(|p| p.exists())
            .collect())
    }

    /// Add project to recent projects list
    pub fn add_to_recent(&self, project_path: &Path) -> Result<()> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("rad-ide");
        
        fs::create_dir_all(&config_dir)?;
        
        let recent_file = config_dir.join("recent_projects.json");
        let mut recent_projects = self.get_recent_projects().unwrap_or_default();
        
        // Remove if already exists
        recent_projects.retain(|p| p != project_path);
        
        // Add to front
        recent_projects.insert(0, project_path.to_path_buf());
        
        // Keep only last 10
        recent_projects.truncate(10);
        
        let paths: Vec<String> = recent_projects.into_iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect();
        
        let content = serde_json::to_string_pretty(&paths)?;
        fs::write(recent_file, content)?;
        
        Ok(())
    }

    /// Get current project path
    pub fn get_current_project_path(&self) -> Option<&Path> {
        self.current_project_path.as_deref()
    }

    /// Convert components to serializable format
    pub fn components_to_serializable(&self, components: &[Box<dyn Component>]) -> Vec<SerializableComponent> {
        components.iter().enumerate().map(|(i, component)| {
            SerializableComponent {
                component_type: component.name().to_string(),
                name: format!("{}_{}", component.name(), i),
                properties: std::collections::HashMap::new(), // TODO: Extract actual properties
                position: None, // TODO: Get from visual designer
                size: None,     // TODO: Get from visual designer
                id: format!("component_{}", i),
            }
        }).collect()
    }
}