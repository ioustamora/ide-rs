//! Core Project Structures and Metadata
//!
//! Defines the fundamental project structures, metadata, and data types
//! used throughout the project management system.

use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

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
#[derive(Serialize, Deserialize, Clone, Debug)]
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

impl IdeProject {
    /// Create a new project with basic settings
    pub fn new(name: String, root_path: PathBuf, project_type: ProjectType) -> Self {
        let now = chrono::Utc::now();
        
        Self {
            metadata: ProjectMetadata {
                name: name.clone(),
                description: format!("A new {} project", match project_type {
                    ProjectType::GuiApplication => "GUI application",
                    ProjectType::ConsoleApplication => "console application",
                    ProjectType::Library => "library",
                    ProjectType::WebApplication => "web application",
                    ProjectType::GameProject => "game",
                    ProjectType::Custom(ref custom) => custom,
                }),
                version: "0.1.0".to_string(),
                author: std::env::var("USER").unwrap_or_else(|_| "Developer".to_string()),
                root_path,
                created_at: now,
                modified_at: now,
                project_type,
                targets: vec!["desktop".to_string()],
            },
            designer_data: DesignerData {
                components: Vec::new(),
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
                target_name: name,
                profiles: create_default_build_profiles(),
                dependencies: create_default_dependencies(),
                features: Vec::new(),
                build_scripts: Vec::new(),
            },
            custom_settings: HashMap::new(),
        }
    }
    
    /// Update the last modified timestamp
    pub fn mark_modified(&mut self) {
        self.metadata.modified_at = chrono::Utc::now();
    }
}

/// Create default build profiles
pub fn create_default_build_profiles() -> HashMap<String, BuildProfile> {
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
pub fn create_default_dependencies() -> Vec<Dependency> {
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