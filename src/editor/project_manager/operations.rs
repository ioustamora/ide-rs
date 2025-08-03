//! High-Level Project Operations
//!
//! Handles complex project operations like creation, validation,
//! component management, and file operations.

use std::path::Path;
use std::fs;
use std::collections::HashMap;
use crate::editor::output_panel::OutputPanel;
use crate::rcl::ui::component::Component;
use super::project::{IdeProject, ProjectMetadata, ProjectType, DesignerData, 
                   ProjectFileStructure, ComponentData, GridSettings, 
                   LayoutConfiguration, LayoutType, AlignmentSettings, 
                   HorizontalAlign, VerticalAlign, create_default_build_profiles, 
                   create_default_dependencies};
use super::templates::{ProjectTemplate, TemplateProcessor};

/// High-level project operations handler
pub struct ProjectOperations;

impl ProjectOperations {
    /// Create a new project operations handler
    pub fn new() -> Self {
        Self
    }

    /// Create a new project with cargo new integration
    pub fn create_gui_project_with_cargo(
        &self,
        name: &str,
        location: &Path,
        output_panel: &mut OutputPanel
    ) -> Result<IdeProject, Box<dyn std::error::Error>> {
        output_panel.log(&format!("🚀 Creating new GUI project with cargo: {}", name));
        
        let project_path = location.join(name);
        
        // Step 1: Execute cargo new command
        self.execute_cargo_new(name, location, output_panel)?;
        
        // Step 2: Enhance project with GUI-specific files
        self.enhance_project_for_gui(name, &project_path, output_panel)?;
        
        // Step 3: Create and return IDE project structure
        self.create_ide_project_structure(name, &project_path, output_panel)
    }

    /// Create a new project from template
    pub fn create_project(
        &self, 
        name: &str, 
        template: &ProjectTemplate, 
        location: &Path, 
        output_panel: &mut OutputPanel
    ) -> Result<IdeProject, Box<dyn std::error::Error>> {
        output_panel.log(&format!("🏗️ Creating new project: {}", name));
        
        let project_path = location.join(name);
        
        // Create project directory
        fs::create_dir_all(&project_path)?;
        output_panel.log(&format!("📁 Created project directory: {}", project_path.display()));
        
        // Create project metadata
        let metadata = ProjectMetadata {
            name: name.to_string(),
            description: format!("A new {} project", template.name),
            version: "0.1.0".to_string(),
            author: std::env::var("USER").unwrap_or_else(|_| "Developer".to_string()),
            root_path: project_path.clone(),
            created_at: chrono::Utc::now(),
            modified_at: chrono::Utc::now(),
            project_type: self.determine_project_type(&template.category),
            targets: vec!["desktop".to_string()],
        };

        // Create project structure
        let project = IdeProject {
            metadata,
            designer_data: DesignerData {
                components: template.default_components.clone(),
                grid_settings: GridSettings {
                    size: 20.0,
                    visible: true,
                    snap_enabled: true,
                    color: (128, 128, 128, 64),
                },
                layout_config: LayoutConfiguration {
                    layout_type: LayoutType::Free,
                    spacing: 10.0,
                    padding: 20.0,
                    alignment: AlignmentSettings {
                        horizontal: HorizontalAlign::Left,
                        vertical: VerticalAlign::Top,
                    },
                },
                styles: HashMap::new(),
            },
            file_structure: ProjectFileStructure {
                source_files: Vec::new(),
                resources: Vec::new(),
                config_files: Vec::new(),
                generated_files: Vec::new(),
            },
            build_config: super::project::BuildConfiguration {
                target_name: name.to_string(),
                profiles: create_default_build_profiles(),
                dependencies: create_default_dependencies(),
                features: Vec::new(),
                build_scripts: Vec::new(),
            },
            custom_settings: HashMap::new(),
        };

        // Create template files
        for template_file in &template.files {
            let file_path = project_path.join(&template_file.path);
            
            // Create parent directories if needed
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            // Process template content
            let content = TemplateProcessor::process_template_content(
                &template_file.content, 
                name, 
                &template.config
            );
            fs::write(&file_path, content)?;
            
            // Set executable permissions if needed
            if template_file.executable {
                self.set_executable_permissions(&file_path)?;
            }
            
            output_panel.log(&format!("📄 Created file: {}", template_file.path.display()));
        }

        // Create standard directories
        self.create_standard_directories(&project_path, output_panel)?;
        
        output_panel.log(&format!("✅ Project '{}' created successfully!", name));
        
        Ok(project)
    }

    /// Update project with new component data
    pub fn update_project_components(
        &self, 
        project: &mut IdeProject, 
        components: &[Box<dyn Component>]
    ) -> Result<(), Box<dyn std::error::Error>> {
        project.designer_data.components.clear();
        
        for (i, component) in components.iter().enumerate() {
            let component_data = ComponentData {
                component_type: component.name().to_string(),
                properties: self.extract_component_properties(component),
                position: (i as f32 * 50.0, i as f32 * 50.0), // Default positioning
                size: (100.0, 30.0), // Default size
                z_order: i as i32,
                locked: false,
                id: format!("{}_{}", component.name().to_lowercase(), i),
            };
            
            project.designer_data.components.push(component_data);
        }
        
        project.metadata.modified_at = chrono::Utc::now();
        
        Ok(())
    }

    /// Validate project integrity
    pub fn validate_project(
        &self, 
        project: &IdeProject, 
        output_panel: &mut OutputPanel
    ) -> Result<ValidationResult, Box<dyn std::error::Error>> {
        let mut result = ValidationResult::new();
        
        // Validate basic project structure
        self.validate_project_structure(project, &mut result, output_panel);
        
        // Validate files
        self.validate_project_files(project, &mut result, output_panel);
        
        // Validate components
        self.validate_project_components(project, &mut result, output_panel);
        
        // Validate build configuration
        self.validate_build_configuration(project, &mut result, output_panel);
        
        Ok(result)
    }

    /// Synchronize project files with file system
    pub fn sync_project_files(
        &self, 
        project: &mut IdeProject, 
        output_panel: &mut OutputPanel
    ) -> Result<(), Box<dyn std::error::Error>> {
        output_panel.log("🔄 Synchronizing project files...");
        
        let root_path = project.metadata.root_path.clone();
        
        // Clear existing file lists
        project.file_structure.source_files.clear();
        project.file_structure.resources.clear();
        project.file_structure.config_files.clear();
        project.file_structure.generated_files.clear();
        
        // Scan project directory
        self.scan_directory(&root_path, project, output_panel)?;
        
        project.metadata.modified_at = chrono::Utc::now();
        
        output_panel.log("✅ Project files synchronized!");
        
        Ok(())
    }

    /// Clean project (remove build artifacts, etc.)
    pub fn clean_project(
        &self, 
        project: &IdeProject, 
        output_panel: &mut OutputPanel
    ) -> Result<(), Box<dyn std::error::Error>> {
        output_panel.log("🧹 Cleaning project...");
        
        let target_dir = project.metadata.root_path.join("target");
        if target_dir.exists() {
            fs::remove_dir_all(&target_dir)?;
            output_panel.log("🗑️ Removed target directory");
        }
        
        // Remove other build artifacts
        let artifacts = vec![
            "Cargo.lock",
            ".cargo",
            "*.pdb",
            "*.tmp",
        ];
        
        for artifact in artifacts {
            let artifact_path = project.metadata.root_path.join(artifact);
            if artifact_path.exists() {
                if artifact_path.is_file() {
                    fs::remove_file(&artifact_path)?;
                } else {
                    fs::remove_dir_all(&artifact_path)?;
                }
                output_panel.log(&format!("🗑️ Removed: {}", artifact));
            }
        }
        
        output_panel.log("✅ Project cleaned successfully!");
        
        Ok(())
    }

    /// Execute cargo new command
    fn execute_cargo_new(
        &self,
        name: &str,
        location: &Path,
        output_panel: &mut OutputPanel
    ) -> Result<(), Box<dyn std::error::Error>> {
        use std::process::Command;
        
        output_panel.log(&format!("📦 Executing: cargo new {}", name));
        
        let output = Command::new("cargo")
            .arg("new")
            .arg(name)
            .current_dir(location)
            .output()?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if !stdout.is_empty() {
                output_panel.log(&stdout);
            }
            output_panel.log("✅ Cargo new completed successfully");
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            output_panel.log(&format!("❌ Cargo new failed: {}", stderr));
            return Err(format!("Cargo new failed: {}", stderr).into());
        }
        
        Ok(())
    }
    
    /// Enhance project with GUI-specific files and dependencies
    fn enhance_project_for_gui(
        &self,
        name: &str,
        project_path: &Path,
        output_panel: &mut OutputPanel
    ) -> Result<(), Box<dyn std::error::Error>> {
        output_panel.log("🎨 Enhancing project for GUI development...");
        
        // Replace Cargo.toml with GUI-enhanced version
        self.create_gui_cargo_toml(name, project_path, output_panel)?;
        
        // Replace main.rs with GUI template
        self.create_gui_main_rs(name, project_path, output_panel)?;
        
        // Create additional GUI-specific files
        self.create_gui_ui_file(name, project_path, output_panel)?;
        
        output_panel.log("✅ GUI enhancement completed");
        Ok(())
    }
    
    /// Create GUI-enhanced Cargo.toml
    fn create_gui_cargo_toml(
        &self,
        name: &str,
        project_path: &Path,
        output_panel: &mut OutputPanel
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cargo_content = format!(
r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"
authors = ["{}"]
description = "A GUI application created with RAD IDE"

[dependencies]
eframe = "0.27"
egui = "0.27"
serde = {{ version = "1.0", features = ["derive"] }}
"#, 
            name, 
            std::env::var("USER").unwrap_or_else(|_| "Developer".to_string())
        );
        
        let cargo_path = project_path.join("Cargo.toml");
        fs::write(&cargo_path, cargo_content)?;
        output_panel.log("📄 Created enhanced Cargo.toml with GUI dependencies");
        Ok(())
    }
    
    /// Create GUI main.rs file
    fn create_gui_main_rs(
        &self,
        name: &str,
        project_path: &Path,
        output_panel: &mut OutputPanel
    ) -> Result<(), Box<dyn std::error::Error>> {
        let main_content = self.get_gui_main_template(name);
        let main_path = project_path.join("src").join("main.rs");
        fs::write(&main_path, main_content)?;
        output_panel.log("📄 Created GUI main.rs with egui framework");
        Ok(())
    }
    
    /// Create dedicated UI file for visual designer
    fn create_gui_ui_file(
        &self,
        name: &str,
        project_path: &Path,
        output_panel: &mut OutputPanel
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ui_content = self.get_ui_module_template(name);
        let ui_path = project_path.join("src").join("ui.rs");
        fs::write(&ui_path, ui_content)?;
        output_panel.log("📄 Created ui.rs for visual designer integration");
        Ok(())
    }
    
    /// Create IDE project structure from cargo-created project
    fn create_ide_project_structure(
        &self,
        name: &str,
        project_path: &Path,
        output_panel: &mut OutputPanel
    ) -> Result<IdeProject, Box<dyn std::error::Error>> {
        output_panel.log("🏗️ Creating IDE project structure...");
        
        // Create project metadata
        let metadata = super::project::ProjectMetadata {
            name: name.to_string(),
            description: format!("A GUI application created with RAD IDE"),
            version: "0.1.0".to_string(),
            author: std::env::var("USER").unwrap_or_else(|_| "Developer".to_string()),
            root_path: project_path.to_path_buf(),
            created_at: chrono::Utc::now(),
            modified_at: chrono::Utc::now(),
            project_type: super::project::ProjectType::GuiApplication,
            targets: vec!["desktop".to_string()],
        };

        // Create project structure with GUI defaults
        let project = super::project::IdeProject {
            metadata,
            designer_data: super::project::DesignerData {
                components: self.get_default_gui_components(),
                grid_settings: super::project::GridSettings {
                    size: 20.0,
                    visible: true,
                    snap_enabled: true,
                    color: (128, 128, 128, 64),
                },
                layout_config: super::project::LayoutConfiguration {
                    layout_type: super::project::LayoutType::Free,
                    spacing: 10.0,
                    padding: 20.0,
                    alignment: super::project::AlignmentSettings {
                        horizontal: super::project::HorizontalAlign::Left,
                        vertical: super::project::VerticalAlign::Top,
                    },
                },
                styles: HashMap::new(),
            },
            file_structure: super::project::ProjectFileStructure {
                source_files: Vec::new(),
                resources: Vec::new(),
                config_files: Vec::new(),
                generated_files: Vec::new(),
            },
            build_config: super::project::BuildConfiguration {
                target_name: name.to_string(),
                profiles: super::project::create_default_build_profiles(),
                dependencies: self.get_gui_dependencies(),
                features: vec!["default".to_string()],
                build_scripts: Vec::new(),
            },
            custom_settings: HashMap::new(),
        };
        
        output_panel.log("✅ IDE project structure created successfully");
        Ok(project)
    }
    
    /// Get GUI main.rs template
    fn get_gui_main_template(&self, project_name: &str) -> String {
        let pascal_name = self.to_pascal_case(project_name);
        format!(
r#"//! {} - Generated by RAD IDE
//! 
//! A GUI application created with the RAD IDE visual designer.
//! This file contains the main application structure and entry point.

mod ui;

use eframe::egui;
use ui::AppUi;

/// Main application structure
#[derive(Default)]
pub struct {}App {{
    /// UI state and components
    ui: AppUi,
}}

impl eframe::App for {}App {{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {{
        // Render the main application UI
        self.ui.render(ctx);
    }}
}}

fn main() -> Result<(), eframe::Error> {{
    let options = eframe::NativeOptions {{
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("{}"),
        ..Default::default()
    }};
    
    eframe::run_native(
        "{}",
        options,
        Box::new(|_cc| Box::new({}App::default())),
    )
}}
"#, 
            project_name, pascal_name, pascal_name, project_name, project_name, pascal_name
        )
    }
    
    /// Get UI module template for visual designer
    fn get_ui_module_template(&self, project_name: &str) -> String {
        format!(
r#"//! UI Module for {} - Generated by RAD IDE
//! 
//! This module contains the user interface components and layout
//! that can be edited with the RAD IDE visual designer.

use eframe::egui;

/// Application UI state and components
#[derive(Default)]
pub struct AppUi {{
    // Add your UI state here
    button_clicked: bool,
    text_input: String,
}}

impl AppUi {{
    /// Render the main UI
    pub fn render(&mut self, ctx: &egui::Context) {{
        egui::CentralPanel::default().show(ctx, |ui| {{
            ui.heading("Welcome to {}!");
            ui.separator();
            
            ui.label("This is a GUI application created with RAD IDE.");
            ui.label("You can modify this code or use the visual designer to add components.");
            
            ui.horizontal(|ui| {{
                ui.label("Enter text:");
                ui.text_edit_singleline(&mut self.text_input);
            }});
            
            if ui.button("Click me!").clicked() {{
                self.button_clicked = !self.button_clicked;
                println!("Button was clicked! Text: {{}}", self.text_input);
            }}
            
            if self.button_clicked {{
                ui.label("Button was clicked!");
            }}
            
            ui.separator();
            ui.label("💡 Tip: Open this project in RAD IDE to use the visual designer!");
        }});
    }}
}}
"#, 
            project_name, project_name
        )
    }
    
    /// Get default GUI components for new projects
    fn get_default_gui_components(&self) -> Vec<super::project::ComponentData> {
        vec![
            super::project::ComponentData {
                component_type: "Button".to_string(),
                properties: {
                    let mut props = HashMap::new();
                    props.insert("label".to_string(), "Welcome Button".to_string());
                    props
                },
                position: (50.0, 50.0),
                size: (120.0, 35.0),
                z_order: 0,
                locked: false,
                id: "welcome_button".to_string(),
            },
            super::project::ComponentData {
                component_type: "Label".to_string(),
                properties: {
                    let mut props = HashMap::new();
                    props.insert("text".to_string(), "Created with RAD IDE".to_string());
                    props
                },
                position: (50.0, 100.0),
                size: (200.0, 25.0),
                z_order: 1,
                locked: false,
                id: "info_label".to_string(),
            },
        ]
    }
    
    /// Get GUI-specific dependencies
    fn get_gui_dependencies(&self) -> Vec<super::project::Dependency> {
        vec![
            super::project::Dependency {
                name: "eframe".to_string(),
                version: "0.27".to_string(),
                features: vec!["default".to_string()],
                optional: false,
            },
            super::project::Dependency {
                name: "egui".to_string(),
                version: "0.27".to_string(),
                features: vec!["default".to_string()],
                optional: false,
            },
            super::project::Dependency {
                name: "serde".to_string(),
                version: "1.0".to_string(),
                features: vec!["derive".to_string()],
                optional: false,
            },
        ]
    }
    
    /// Convert string to PascalCase
    fn to_pascal_case(&self, s: &str) -> String {
        s.split(|c: char| !c.is_alphanumeric())
            .filter(|word| !word.is_empty())
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                }
            })
            .collect()
    }

    /// Determine project type from template category
    fn determine_project_type(&self, category: &str) -> ProjectType {
        match category.to_lowercase().as_str() {
            "application" => ProjectType::GuiApplication,
            "console" => ProjectType::ConsoleApplication,
            "library" => ProjectType::Library,
            "web" => ProjectType::WebApplication,
            "game" => ProjectType::GameProject,
            _ => ProjectType::Custom(category.to_string()),
        }
    }

    /// Extract properties from a component
    fn extract_component_properties(&self, _component: &Box<dyn Component>) -> HashMap<String, String> {
        // Placeholder implementation
        // In a real implementation, this would extract actual component properties
        HashMap::new()
    }

    /// Set executable permissions on Unix systems
    fn set_executable_permissions(&self, _file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(_file_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(_file_path, perms)?;
        }
        Ok(())
    }

    /// Create standard project directories
    fn create_standard_directories(
        &self, 
        project_path: &Path, 
        output_panel: &mut OutputPanel
    ) -> Result<(), Box<dyn std::error::Error>> {
        let directories = vec![
            "src",
            "assets",
            "docs",
            "tests",
        ];
        
        for dir in directories {
            let dir_path = project_path.join(dir);
            if !dir_path.exists() {
                fs::create_dir_all(&dir_path)?;
                output_panel.log(&format!("📁 Created directory: {}", dir));
            }
        }
        
        Ok(())
    }

    /// Validate project structure
    fn validate_project_structure(
        &self, 
        project: &IdeProject, 
        result: &mut ValidationResult, 
        output_panel: &mut OutputPanel
    ) {
        let root_path = &project.metadata.root_path;
        
        if !root_path.exists() {
            result.add_error("Project root directory does not exist".to_string());
            return;
        }
        
        // Check for essential files
        let essential_files = vec!["Cargo.toml"];
        for file in essential_files {
            let file_path = root_path.join(file);
            if !file_path.exists() {
                result.add_warning(format!("{} not found", file));
                output_panel.log(&format!("⚠️ Warning: {} not found", file));
            }
        }
        
        // Check for src directory
        let src_dir = root_path.join("src");
        if !src_dir.exists() {
            result.add_warning("src directory not found".to_string());
        }
    }

    /// Validate project files
    fn validate_project_files(
        &self, 
        project: &IdeProject, 
        result: &mut ValidationResult, 
        output_panel: &mut OutputPanel
    ) {
        let root_path = &project.metadata.root_path;
        
        // Validate source files exist
        for source_file in &project.file_structure.source_files {
            let file_path = root_path.join(&source_file.path);
            if !file_path.exists() {
                result.add_warning(format!("Source file not found: {}", source_file.path.display()));
                output_panel.log(&format!("⚠️ Warning: Source file not found: {}", source_file.path.display()));
            }
        }
    }

    /// Validate project components
    fn validate_project_components(
        &self, 
        project: &IdeProject, 
        result: &mut ValidationResult, 
        _output_panel: &mut OutputPanel
    ) {
        // Check for duplicate component IDs
        let mut ids = std::collections::HashSet::new();
        for component in &project.designer_data.components {
            if !ids.insert(&component.id) {
                result.add_error(format!("Duplicate component ID: {}", component.id));
            }
        }
    }

    /// Validate build configuration
    fn validate_build_configuration(
        &self, 
        project: &IdeProject, 
        result: &mut ValidationResult, 
        _output_panel: &mut OutputPanel
    ) {
        // Check if we have at least one build profile
        if project.build_config.profiles.is_empty() {
            result.add_warning("No build profiles defined".to_string());
        }
        
        // Validate dependencies
        for dep in &project.build_config.dependencies {
            if dep.name.is_empty() {
                result.add_error("Dependency with empty name found".to_string());
            }
            if dep.version.is_empty() {
                result.add_warning(format!("Dependency '{}' has no version specified", dep.name));
            }
        }
    }

    /// Scan directory for project files
    fn scan_directory(
        &self, 
        dir: &Path, 
        project: &mut IdeProject, 
        output_panel: &mut OutputPanel
    ) -> Result<(), Box<dyn std::error::Error>> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                // Skip target and .git directories
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name == "target" || name == ".git" {
                        continue;
                    }
                }
                
                // Recursively scan subdirectories
                self.scan_directory(&path, project, output_panel)?;
            } else if path.is_file() {
                // Add file to appropriate category
                self.categorize_and_add_file(&path, project, output_panel)?;
            }
        }
        
        Ok(())
    }

    /// Categorize and add file to project structure
    fn categorize_and_add_file(
        &self, 
        file_path: &Path, 
        project: &mut IdeProject, 
        _output_panel: &mut OutputPanel
    ) -> Result<(), Box<dyn std::error::Error>> {
        let relative_path = file_path.strip_prefix(&project.metadata.root_path)?;
        let metadata = fs::metadata(file_path)?;
        
        let file_type = self.determine_file_type(file_path);
        let is_generated = self.is_generated_file(file_path);
        
        let project_file = super::project::ProjectFile {
            path: relative_path.to_path_buf(),
            file_type: file_type.clone(),
            is_generated,
            modified_at: metadata.modified()
                .unwrap_or(std::time::UNIX_EPOCH)
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| chrono::DateTime::from_timestamp(d.as_secs() as i64, 0))
                .unwrap_or(None)
                .unwrap_or_else(chrono::Utc::now),
            size: metadata.len(),
        };
        
        // Add to appropriate category
        match file_type {
            super::project::FileType::RustSource => {
                project.file_structure.source_files.push(project_file);
            }
            super::project::FileType::CargoToml | super::project::FileType::Configuration => {
                project.file_structure.config_files.push(project_file);
            }
            super::project::FileType::Image => {
                project.file_structure.resources.push(project_file);
            }
            super::project::FileType::Generated => {
                project.file_structure.generated_files.push(project_file);
            }
            _ => {
                // Add to source files by default
                project.file_structure.source_files.push(project_file);
            }
        }
        
        Ok(())
    }

    /// Determine file type from extension
    fn determine_file_type(&self, file_path: &Path) -> super::project::FileType {
        match file_path.extension().and_then(|ext| ext.to_str()) {
            Some("rs") => super::project::FileType::RustSource,
            Some("toml") if file_path.file_name().and_then(|n| n.to_str()) == Some("Cargo.toml") => {
                super::project::FileType::CargoToml
            }
            Some("toml") | Some("yaml") | Some("yml") | Some("json") => {
                super::project::FileType::Configuration
            }
            Some("png") | Some("jpg") | Some("jpeg") | Some("gif") | Some("svg") => {
                super::project::FileType::Image
            }
            Some("txt") | Some("md") => super::project::FileType::Text,
            Some("exe") | Some("dll") | Some("so") | Some("dylib") => {
                super::project::FileType::Binary
            }
            _ => super::project::FileType::Unknown,
        }
    }

    /// Check if file is generated
    fn is_generated_file(&self, file_path: &Path) -> bool {
        if let Some(name) = file_path.file_name().and_then(|n| n.to_str()) {
            // Check for common generated file patterns
            name.contains(".generated.") || 
            name.ends_with(".gen.rs") ||
            name == "Cargo.lock"
        } else {
            false
        }
    }
}

/// Project validation result
pub struct ValidationResult {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub info: Vec<String>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            info: Vec::new(),
        }
    }
    
    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }
    
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
    
    pub fn add_info(&mut self, info: String) {
        self.info.push(info);
    }
    
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
}

impl Default for ProjectOperations {
    fn default() -> Self {
        Self::new()
    }
}