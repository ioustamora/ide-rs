//! Project Management System for RAD IDE
//!
//! This module provides comprehensive project management functionality including:
//! - Project creation, loading, and saving
//! - File system integration and browsing
//! - Cargo project integration
//! - Project templates and scaffolding
//! - Version control integration hooks

use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::editor::output_panel::OutputPanel;
use crate::rcl::ui::component::Component;

/// Main project manager handling all project operations
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
}

/// IDE project structure containing all project data
#[derive(Serialize, Deserialize, Clone)]
pub struct IdeProject {
    /// Project metadata
    pub metadata: ProjectMetadata,
    /// Visual designer components and layout
    pub designer_data: DesignerData,
    /// Project files and structure
    pub file_structure: ProjectFileStructure,
    /// Build configuration
    pub build_config: BuildConfiguration,
    /// Custom settings for this project
    pub custom_settings: HashMap<String, String>,
}

/// Project metadata and information
#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectMetadata {
    /// Project name
    pub name: String,
    /// Project description
    pub description: String,
    /// Project version
    pub version: String,
    /// Author information
    pub author: String,
    /// Project root directory
    pub root_path: PathBuf,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last modified timestamp
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// Project type
    pub project_type: ProjectType,
    /// Target platforms
    pub targets: Vec<String>,
}

/// Visual designer data for the project
#[derive(Serialize, Deserialize, Clone)]
pub struct DesignerData {
    /// Components in the designer
    pub components: Vec<ComponentData>,
    /// Grid settings
    pub grid_settings: GridSettings,
    /// Layout configuration
    pub layout_config: LayoutConfiguration,
    /// Custom styling
    pub styles: HashMap<String, String>,
}

/// Serializable component data
#[derive(Serialize, Deserialize, Clone)]
pub struct ComponentData {
    /// Component type name
    pub component_type: String,
    /// Component properties
    pub properties: HashMap<String, String>,
    /// Position in designer
    pub position: (f32, f32),
    /// Size of component
    pub size: (f32, f32),
    /// Z-order for layering
    pub z_order: i32,
    /// Whether component is locked
    pub locked: bool,
    /// Component ID for references
    pub id: String,
}

/// Grid settings for visual designer
#[derive(Serialize, Deserialize, Clone)]
pub struct GridSettings {
    /// Grid size in pixels
    pub size: f32,
    /// Whether grid is visible
    pub visible: bool,
    /// Whether snap to grid is enabled
    pub snap_enabled: bool,
    /// Grid color
    pub color: (u8, u8, u8, u8),
}

/// Layout configuration
#[derive(Serialize, Deserialize, Clone)]
pub struct LayoutConfiguration {
    /// Layout type (grid, vertical, horizontal, free)
    pub layout_type: LayoutType,
    /// Spacing between components
    pub spacing: f32,
    /// Padding around layout
    pub padding: f32,
    /// Alignment settings
    pub alignment: AlignmentSettings,
}

/// Types of layouts available
#[derive(Serialize, Deserialize, Clone)]
pub enum LayoutType {
    Grid,
    Vertical,
    Horizontal,
    Free,
    Custom(String),
}

/// Alignment settings for components
#[derive(Serialize, Deserialize, Clone)]
pub struct AlignmentSettings {
    /// Horizontal alignment
    pub horizontal: HorizontalAlign,
    /// Vertical alignment
    pub vertical: VerticalAlign,
}

/// Horizontal alignment options
#[derive(Serialize, Deserialize, Clone)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

/// Vertical alignment options
#[derive(Serialize, Deserialize, Clone)]
pub enum VerticalAlign {
    Top,
    Center,
    Bottom,
}

/// Project file structure and organization
#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectFileStructure {
    /// Source files
    pub source_files: Vec<ProjectFile>,
    /// Resource files (images, etc.)
    pub resources: Vec<ProjectFile>,
    /// Configuration files
    pub config_files: Vec<ProjectFile>,
    /// Generated files (should not be edited manually)
    pub generated_files: Vec<ProjectFile>,
}

/// Individual project file information
#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectFile {
    /// Relative path from project root
    pub path: PathBuf,
    /// File type
    pub file_type: FileType,
    /// Whether this file is auto-generated
    pub is_generated: bool,
    /// Last modified timestamp
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// File size in bytes
    pub size: u64,
}

/// Types of files in the project
#[derive(Serialize, Deserialize, Clone)]
pub enum FileType {
    RustSource,
    CargoToml,
    Configuration,
    Image,
    Text,
    Binary,
    Generated,
    Unknown,
}

/// Build configuration for the project
#[derive(Serialize, Deserialize, Clone)]
pub struct BuildConfiguration {
    /// Target name
    pub target_name: String,
    /// Build profiles
    pub profiles: HashMap<String, BuildProfile>,
    /// Dependencies
    pub dependencies: Vec<Dependency>,
    /// Features to enable
    pub features: Vec<String>,
    /// Custom build scripts
    pub build_scripts: Vec<String>,
}

/// Build profile configuration
#[derive(Serialize, Deserialize, Clone)]
pub struct BuildProfile {
    /// Profile name (debug, release, etc.)
    pub name: String,
    /// Optimization level
    pub opt_level: String,
    /// Debug info
    pub debug: bool,
    /// Custom flags
    pub flags: Vec<String>,
}

/// Dependency specification
#[derive(Serialize, Deserialize, Clone)]
pub struct Dependency {
    /// Crate name
    pub name: String,
    /// Version requirement
    pub version: String,
    /// Features to enable
    pub features: Vec<String>,
    /// Whether it's optional
    pub optional: bool,
}

/// Project types supported
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ProjectType {
    GuiApplication,
    ConsoleApplication,
    Library,
    WebApplication,
    GameProject,
    Custom(String),
}

/// Project template for creating new projects
#[derive(Clone)]
pub struct ProjectTemplate {
    /// Template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Template category
    pub category: String,
    /// Files to create
    pub files: Vec<TemplateFile>,
    /// Default components to add
    pub default_components: Vec<ComponentData>,
    /// Template configuration
    pub config: TemplateConfig,
}

/// Template file specification
#[derive(Clone)]
pub struct TemplateFile {
    /// File path relative to project root
    pub path: PathBuf,
    /// File content template
    pub content: String,
    /// Whether this file is executable
    pub executable: bool,
}

/// Template configuration
#[derive(Clone)]
pub struct TemplateConfig {
    /// Required Rust version
    pub min_rust_version: String,
    /// Default dependencies
    pub default_dependencies: Vec<String>,
    /// Template variables
    pub variables: HashMap<String, String>,
}

/// File browser for navigating project files
pub struct FileBrowser {
    /// Current directory
    pub current_dir: PathBuf,
    /// Directory history for navigation
    pub history: Vec<PathBuf>,
    /// Currently selected files
    pub selected_files: Vec<PathBuf>,
    /// File filters
    pub filters: FileFilters,
    /// View settings
    pub view_settings: FileViewSettings,
}

/// File filtering options
pub struct FileFilters {
    /// Show hidden files
    pub show_hidden: bool,
    /// File extensions to show
    pub extensions: Vec<String>,
    /// Directories to exclude
    pub exclude_dirs: Vec<String>,
}

/// File view display settings
pub struct FileViewSettings {
    /// View mode (list, grid, tree)
    pub view_mode: FileViewMode,
    /// Sort order
    pub sort_by: FileSortOrder,
    /// Sort ascending or descending
    pub sort_ascending: bool,
}

/// File view modes
pub enum FileViewMode {
    List,
    Grid,
    Tree,
}

/// File sorting options
pub enum FileSortOrder {
    Name,
    Modified,
    Size,
    Type,
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
            templates: Self::create_default_templates(),
            file_browser: FileBrowser {
                current_dir: default_project_path.clone(),
                history: Vec::new(),
                selected_files: Vec::new(),
                filters: FileFilters {
                    show_hidden: false,
                    extensions: vec!["rs".to_string(), "toml".to_string(), "md".to_string()],
                    exclude_dirs: vec!["target".to_string(), ".git".to_string()],
                },
                view_settings: FileViewSettings {
                    view_mode: FileViewMode::Tree,
                    sort_by: FileSortOrder::Name,
                    sort_ascending: true,
                },
            },
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
        }
    }

    /// Create a new project from template
    pub fn create_project(&mut self, name: &str, template: &ProjectTemplate, location: &Path, output_panel: &mut OutputPanel) -> Result<(), Box<dyn std::error::Error>> {
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
            project_type: ProjectType::GuiApplication,
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
            build_config: BuildConfiguration {
                target_name: name.to_string(),
                profiles: Self::create_default_build_profiles(),
                dependencies: Self::create_default_dependencies(),
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
            let content = self.process_template_content(&template_file.content, name, &template.config);
            fs::write(&file_path, content)?;
            
            if template_file.executable {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mut perms = fs::metadata(&file_path)?.permissions();
                    perms.set_mode(0o755);
                    fs::set_permissions(&file_path, perms)?;
                }
            }
            
            output_panel.log(&format!("📄 Created file: {}", template_file.path.display()));
        }

        // Save project file
        self.save_project(&project, output_panel)?;
        
        // Set as current project
        self.current_project = Some(project);
        
        // Add to recent projects
        self.add_to_recent_projects(&project_path);
        
        output_panel.log(&format!("✅ Project '{}' created successfully!", name));
        
        Ok(())
    }

    /// Load an existing project
    pub fn load_project(&mut self, project_path: &Path, output_panel: &mut OutputPanel) -> Result<(), Box<dyn std::error::Error>> {
        output_panel.log(&format!("📂 Loading project from: {}", project_path.display()));
        
        let project_file = project_path.join("project.ide");
        
        if !project_file.exists() {
            return Err("Project file not found. This may not be a valid IDE project.".into());
        }
        
        let content = fs::read_to_string(&project_file)?;
        let project: IdeProject = serde_json::from_str(&content)?;
        
        // Validate project structure
        self.validate_project(&project, output_panel)?;
        
        // Update current directory
        self.file_browser.current_dir = project.metadata.root_path.clone();
        
        // Set as current project
        self.current_project = Some(project);
        
        // Add to recent projects
        self.add_to_recent_projects(project_path);
        
        output_panel.log(&format!("✅ Project '{}' loaded successfully!", self.current_project.as_ref().unwrap().metadata.name));
        
        Ok(())
    }

    /// Save the current project
    pub fn save_project(&self, project: &IdeProject, output_panel: &mut OutputPanel) -> Result<(), Box<dyn std::error::Error>> {
        let project_file = project.metadata.root_path.join("project.ide");
        
        output_panel.log(&format!("💾 Saving project to: {}", project_file.display()));
        
        let content = serde_json::to_string_pretty(project)?;
        fs::write(&project_file, content)?;
        
        output_panel.log("✅ Project saved successfully!");
        
        Ok(())
    }

    /// Process template content with variable substitution
    fn process_template_content(&self, content: &str, project_name: &str, config: &TemplateConfig) -> String {
        let mut processed = content.to_string();
        
        // Replace common variables
        processed = processed.replace("{{PROJECT_NAME}}", project_name);
        processed = processed.replace("{{PROJECT_NAME_SNAKE}}", &project_name.to_lowercase().replace('-', "_"));
        processed = processed.replace("{{PROJECT_NAME_PASCAL}}", &self.to_pascal_case(project_name));
        processed = processed.replace("{{AUTHOR}}", &std::env::var("USER").unwrap_or_else(|_| "Developer".to_string()));
        processed = processed.replace("{{DATE}}", &chrono::Utc::now().format("%Y-%m-%d").to_string());
        processed = processed.replace("{{YEAR}}", &chrono::Utc::now().format("%Y").to_string());
        
        // Replace custom template variables
        for (key, value) in &config.variables {
            processed = processed.replace(&format!("{{{{{}}}}}", key), value);
        }
        
        processed
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
        
        Ok(())
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

    /// Create default build profiles
    fn create_default_build_profiles() -> HashMap<String, BuildProfile> {
        let mut profiles = HashMap::new();
        
        profiles.insert("debug".to_string(), BuildProfile {
            name: "debug".to_string(),
            opt_level: "0".to_string(),
            debug: true,
            flags: Vec::new(),
        });
        
        profiles.insert("release".to_string(), BuildProfile {
            name: "release".to_string(),
            opt_level: "3".to_string(),
            debug: false,
            flags: vec!["--release".to_string()],
        });
        
        profiles
    }

    /// Create default dependencies
    fn create_default_dependencies() -> Vec<Dependency> {
        vec![
            Dependency {
                name: "eframe".to_string(),
                version: "0.27".to_string(),
                features: Vec::new(),
                optional: false,
            },
            Dependency {
                name: "egui".to_string(),
                version: "0.27".to_string(),
                features: Vec::new(),
                optional: false,
            },
        ]
    }

    /// Create default project templates
    fn create_default_templates() -> Vec<ProjectTemplate> {
        vec![
            // GUI Application Template
            ProjectTemplate {
                name: "GUI Application".to_string(),
                description: "A GUI application using egui framework".to_string(),
                category: "Application".to_string(),
                files: vec![
                    TemplateFile {
                        path: PathBuf::from("Cargo.toml"),
                        content: Self::get_gui_cargo_template(),
                        executable: false,
                    },
                    TemplateFile {
                        path: PathBuf::from("src/main.rs"),
                        content: Self::get_gui_main_template(),
                        executable: false,
                    },
                    TemplateFile {
                        path: PathBuf::from("README.md"),
                        content: Self::get_readme_template(),
                        executable: false,
                    },
                ],
                default_components: vec![
                    ComponentData {
                        component_type: "Button".to_string(),
                        properties: {
                            let mut props = HashMap::new();
                            props.insert("label".to_string(), "Click Me".to_string());
                            props
                        },
                        position: (50.0, 50.0),
                        size: (100.0, 30.0),
                        z_order: 0,
                        locked: false,
                        id: "button_1".to_string(),
                    },
                ],
                config: TemplateConfig {
                    min_rust_version: "1.75.0".to_string(),
                    default_dependencies: vec!["eframe".to_string(), "egui".to_string()],
                    variables: HashMap::new(),
                },
            },
        ]
    }

    /// Get GUI application Cargo.toml template
    fn get_gui_cargo_template() -> String {
        r#"[package]
name = "{{PROJECT_NAME}}"
version = "0.1.0"
edition = "2021"
authors = ["{{AUTHOR}}"]

[dependencies]
eframe = "0.27"
egui = "0.27"
"#.to_string()
    }

    /// Get GUI application main.rs template
    fn get_gui_main_template() -> String {
        r#"//! {{PROJECT_NAME}} - Generated by RAD IDE
//! 
//! A GUI application created with the RAD IDE visual designer.
//! Author: {{AUTHOR}}
//! Created: {{DATE}}

use eframe::egui;

/// Main application structure
#[derive(Default)]
pub struct {{PROJECT_NAME_PASCAL}}App {
    // Add your application state here
}

impl eframe::App for {{PROJECT_NAME_PASCAL}}App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to {{PROJECT_NAME}}!");
            ui.separator();
            
            ui.label("This is a GUI application created with RAD IDE.");
            ui.label("You can modify this code or use the visual designer to add components.");
            
            if ui.button("Click me!").clicked() {
                println!("Button was clicked!");
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "{{PROJECT_NAME}}",
        options,
        Box::new(|_cc| Box::new({{PROJECT_NAME_PASCAL}}App::default())),
    )
}
"#.to_string()
    }

    /// Get README.md template
    fn get_readme_template() -> String {
        r#"# {{PROJECT_NAME}}

A GUI application created with RAD IDE.

## Description

{{PROJECT_NAME}} is a Rust GUI application built using the egui framework and RAD IDE visual designer.

## Building and Running

### Prerequisites
- Rust 1.75.0 or later
- Cargo

### Build and Run
```bash
# Debug build
cargo run

# Release build
cargo run --release

# Build only
cargo build --release
```

## Features

- Cross-platform GUI using egui
- Created with RAD IDE visual designer
- Extensible architecture

## Author

{{AUTHOR}} - {{DATE}}

## License

This project is licensed under the MIT License.
"#.to_string()
    }

    /// Get available project templates
    pub fn get_templates(&self) -> &[ProjectTemplate] {
        &self.templates
    }

    /// Update current project with new component data
    pub fn update_project_components(&mut self, components: &[Box<dyn Component>]) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref mut project) = self.current_project {
            project.designer_data.components.clear();
            
            for (i, component) in components.iter().enumerate() {
                let component_data = ComponentData {
                    component_type: component.name().to_string(),
                    properties: HashMap::new(), // TODO: Extract actual properties
                    position: (i as f32 * 50.0, i as f32 * 50.0), // Default positioning
                    size: (100.0, 30.0), // Default size
                    z_order: i as i32,
                    locked: false,
                    id: format!("{}_{}", component.name().to_lowercase(), i),
                };
                
                project.designer_data.components.push(component_data);
            }
            
            project.metadata.modified_at = chrono::Utc::now();
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
        ui.heading("📁 Project Explorer");
        ui.separator();

        // Current directory display
        ui.horizontal(|ui| {
            if ui.button("🏠").on_hover_text("Go to project root").clicked() {
                if let Some(project) = &self.current_project {
                    self.file_browser.current_dir = project.metadata.root_path.clone();
                }
            }
            
            if ui.button("⬆").on_hover_text("Parent directory").clicked() {
                if let Some(parent) = self.file_browser.current_dir.parent() {
                    self.file_browser.current_dir = parent.to_path_buf();
                }
            }
            
            ui.label(format!("📂 {}", self.file_browser.current_dir.display()));
        });

        ui.separator();

        // File listing
        egui::ScrollArea::vertical().show(ui, |ui| {
            if let Ok(entries) = fs::read_dir(&self.file_browser.current_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                    
                    // Skip hidden files if filter is enabled
                    if !self.file_browser.filters.show_hidden && file_name.starts_with('.') {
                        continue;
                    }
                    
                    // Skip excluded directories
                    if path.is_dir() && self.file_browser.filters.exclude_dirs.iter().any(|exclude| file_name == *exclude) {
                        continue;
                    }
                    
                    let icon = if path.is_dir() { "📁" } else { "📄" };
                    let display_name = format!("{} {}", icon, file_name);
                    
                    if ui.selectable_label(false, &display_name).clicked() {
                        if path.is_dir() {
                            self.file_browser.current_dir = path;
                        } else {
                            // Open file in editor
                            output_panel.log(&format!("📂 Opening file: {}", path.display()));
                            // TODO: Integrate with code editor
                        }
                    }
                }
            }
        });
    }
}

impl Default for ProjectManager {
    fn default() -> Self {
        Self::new()
    }
}