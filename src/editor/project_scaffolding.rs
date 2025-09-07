//! Advanced Project Templates and Scaffolding System
//!
//! This module provides a comprehensive project scaffolding system with:
//! - Pre-built project templates for various use cases
//! - Interactive project wizard
//! - Custom template creation and management
//! - Smart dependency management
//! - Code generation and boilerplate automation

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use egui::{Ui, Vec2, Color32, RichText};

/// Main project scaffolding engine
#[derive(Debug, Clone)]
pub struct ProjectScaffoldingEngine {
    /// Available project templates
    pub templates: HashMap<String, ProjectTemplate>,
    /// Template categories for organization
    pub categories: Vec<TemplateCategory>,
    /// Custom user templates
    pub user_templates: HashMap<String, ProjectTemplate>,
    /// Template wizard state
    pub wizard: ProjectWizard,
    /// Scaffolding settings
    pub settings: ScaffoldingSettings,
    /// Recent template usage
    pub recent_templates: Vec<String>,
}

/// Project template definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTemplate {
    /// Template identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Template description
    pub description: String,
    /// Template version
    pub version: String,
    /// Template author
    pub author: String,
    /// Template category
    pub category: TemplateCategory,
    /// Template tags for filtering
    pub tags: Vec<String>,
    /// File tree structure
    pub file_tree: FileTreeNode,
    /// Template variables for customization
    pub variables: Vec<TemplateVariable>,
    /// Dependencies to include
    pub dependencies: Vec<Dependency>,
    /// Development dependencies
    pub dev_dependencies: Vec<Dependency>,
    /// Build scripts and configurations
    pub build_config: BuildConfiguration,
    /// Post-creation scripts
    pub post_scripts: Vec<PostScript>,
    /// Template assets (images, resources, etc.)
    pub assets: Vec<TemplateAsset>,
    /// Minimum Rust version required
    pub min_rust_version: Option<String>,
    /// Template popularity/usage count
    pub usage_count: usize,
    /// Whether template is featured
    pub featured: bool,
}

/// Template categories for organization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TemplateCategory {
    Application,
    Library,
    WebFramework,
    GameDevelopment,
    SystemProgramming,
    DataScience,
    Blockchain,
    CLI,
    API,
    Desktop,
    Mobile,
    IoT,
    Custom(String),
}

/// File tree node representing project structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTreeNode {
    /// Node name (file or directory)
    pub name: String,
    /// Node type
    pub node_type: FileNodeType,
    /// File content (for files) or None (for directories)
    pub content: Option<String>,
    /// Child nodes (for directories)
    pub children: Vec<FileTreeNode>,
    /// File permissions (Unix-style)
    pub permissions: Option<u32>,
    /// Whether this is a template file (contains variables)
    pub is_template: bool,
}

/// Type of file tree node
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FileNodeType {
    File,
    Directory,
    SymbolicLink(String),
}

/// Template variable for customization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    /// Variable name (used in templates as {{name}})
    pub name: String,
    /// Display label
    pub label: String,
    /// Variable description
    pub description: String,
    /// Variable type
    pub var_type: VariableType,
    /// Default value
    pub default_value: String,
    /// Whether variable is required
    pub required: bool,
    /// Validation pattern (regex)
    pub validation: Option<String>,
    /// Predefined choices (for choice type)
    pub choices: Vec<String>,
}

/// Types of template variables
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VariableType {
    Text,
    Number,
    Boolean,
    Choice,
    Path,
    Email,
    Url,
    Version,
    Identifier,
}

/// Dependency specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    /// Crate name
    pub name: String,
    /// Version requirement
    pub version: String,
    /// Features to enable
    pub features: Vec<String>,
    /// Whether dependency is optional
    pub optional: bool,
    /// Default features enabled
    pub default_features: bool,
    /// Git repository (if not from crates.io)
    pub git: Option<String>,
    /// Git branch/tag/commit
    pub git_ref: Option<String>,
    /// Local path (for path dependencies)
    pub path: Option<String>,
}

/// Build configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfiguration {
    /// Custom build scripts
    pub build_scripts: Vec<String>,
    /// Environment variables
    pub env_vars: HashMap<String, String>,
    /// Compiler flags
    pub rustflags: Vec<String>,
    /// Target platforms
    pub targets: Vec<String>,
    /// Build profiles
    pub profiles: HashMap<String, BuildProfile>,
}

/// Build profile configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildProfile {
    /// Optimization level
    pub opt_level: String,
    /// Debug information
    pub debug: bool,
    /// Link-time optimization
    pub lto: bool,
    /// Code generation units
    pub codegen_units: Option<u32>,
    /// Panic strategy
    pub panic: Option<String>,
}

/// Post-creation script
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostScript {
    /// Script name
    pub name: String,
    /// Script description
    pub description: String,
    /// Shell command to execute
    pub command: String,
    /// Working directory (relative to project root)
    pub working_dir: Option<String>,
    /// Whether script is required or optional
    pub required: bool,
    /// Platform compatibility
    pub platforms: Vec<Platform>,
}

/// Platform compatibility
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
    Unix,
    All,
}

/// Template asset (binary files, images, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateAsset {
    /// Asset source path (relative to template)
    pub source: String,
    /// Asset destination path (relative to project root)
    pub destination: String,
    /// Asset type
    pub asset_type: AssetType,
    /// Whether to process as template (replace variables)
    pub process_template: bool,
}

/// Types of template assets
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AssetType {
    Binary,
    Text,
    Image,
    Config,
    Documentation,
}

/// Project creation wizard
#[derive(Debug, Clone)]
pub struct ProjectWizard {
    /// Current wizard step
    pub current_step: WizardStep,
    /// Selected template
    pub selected_template: Option<String>,
    /// Project configuration
    pub project_config: ProjectConfiguration,
    /// Variable values
    pub variable_values: HashMap<String, String>,
    /// Wizard UI state
    pub ui_state: WizardUIState,
}

/// Wizard steps
#[derive(Debug, Clone, PartialEq)]
pub enum WizardStep {
    TemplateSelection,
    ProjectConfiguration,
    VariableConfiguration,
    DependencySelection,
    Review,
    Creation,
    Complete,
}

/// Project configuration
#[derive(Debug, Clone)]
pub struct ProjectConfiguration {
    /// Project name
    pub name: String,
    /// Project path
    pub path: PathBuf,
    /// Project description
    pub description: String,
    /// Project author
    pub author: String,
    /// Project license
    pub license: String,
    /// Git repository initialization
    pub init_git: bool,
    /// Create initial commit
    pub initial_commit: bool,
    /// Project visibility
    pub visibility: ProjectVisibility,
}

/// Project visibility
#[derive(Debug, Clone, PartialEq)]
pub enum ProjectVisibility {
    Public,
    Private,
}

/// Wizard UI state
#[derive(Debug, Clone)]
pub struct WizardUIState {
    /// Search query for templates
    pub search_query: String,
    /// Selected category filter
    pub category_filter: Option<TemplateCategory>,
    /// Show only featured templates
    pub show_featured_only: bool,
    /// Template preview visible
    pub show_preview: bool,
    /// Advanced options expanded
    pub advanced_options: bool,
}

/// Scaffolding settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaffoldingSettings {
    /// Default project location
    pub default_project_path: PathBuf,
    /// Default author name
    pub default_author: String,
    /// Default license
    pub default_license: String,
    /// Auto-initialize git repositories
    pub auto_init_git: bool,
    /// Auto-create initial commit
    pub auto_initial_commit: bool,
    /// Template update check frequency (days)
    pub update_check_frequency: u32,
    /// Enable template analytics
    pub analytics_enabled: bool,
}

impl Default for ProjectScaffoldingEngine {
    fn default() -> Self {
        let mut engine = Self {
            templates: HashMap::new(),
            categories: vec![
                TemplateCategory::Application,
                TemplateCategory::Library,
                TemplateCategory::WebFramework,
                TemplateCategory::CLI,
                TemplateCategory::Desktop,
                TemplateCategory::GameDevelopment,
                TemplateCategory::SystemProgramming,
                TemplateCategory::API,
            ],
            user_templates: HashMap::new(),
            wizard: ProjectWizard::new(),
            settings: ScaffoldingSettings::default(),
            recent_templates: Vec::new(),
        };
        
        engine.initialize_default_templates();
        engine
    }
}

impl ProjectScaffoldingEngine {
    /// Create a new scaffolding engine
    pub fn new() -> Self {
        Self::default()
    }

    /// Initialize default project templates
    fn initialize_default_templates(&mut self) {
        // Basic Rust Application Template
        let basic_app = ProjectTemplate {
            id: "rust-app-basic".to_string(),
            name: "Basic Rust Application".to_string(),
            description: "A simple Rust application with basic structure and configuration".to_string(),
            version: "1.0.0".to_string(),
            author: "RAD IDE".to_string(),
            category: TemplateCategory::Application,
            tags: vec!["rust".to_string(), "application".to_string(), "basic".to_string()],
            file_tree: self.create_basic_app_structure(),
            variables: vec![
                TemplateVariable {
                    name: "project_name".to_string(),
                    label: "Project Name".to_string(),
                    description: "The name of your project".to_string(),
                    var_type: VariableType::Identifier,
                    default_value: "my_app".to_string(),
                    required: true,
                    validation: Some(r"^[a-zA-Z][a-zA-Z0-9_]*$".to_string()),
                    choices: Vec::new(),
                },
                TemplateVariable {
                    name: "author_name".to_string(),
                    label: "Author Name".to_string(),
                    description: "Your name as the project author".to_string(),
                    var_type: VariableType::Text,
                    default_value: "Your Name".to_string(),
                    required: true,
                    validation: None,
                    choices: Vec::new(),
                },
            ],
            dependencies: vec![],
            dev_dependencies: vec![],
            build_config: BuildConfiguration::default(),
            post_scripts: vec![],
            assets: vec![],
            min_rust_version: Some("1.70.0".to_string()),
            usage_count: 0,
            featured: true,
        };

        // CLI Application Template
        let cli_app = ProjectTemplate {
            id: "rust-cli".to_string(),
            name: "CLI Application".to_string(),
            description: "A command-line application with argument parsing and error handling".to_string(),
            version: "1.0.0".to_string(),
            author: "RAD IDE".to_string(),
            category: TemplateCategory::CLI,
            tags: vec!["rust".to_string(), "cli".to_string(), "clap".to_string()],
            file_tree: self.create_cli_app_structure(),
            variables: vec![
                TemplateVariable {
                    name: "project_name".to_string(),
                    label: "Project Name".to_string(),
                    description: "The name of your CLI tool".to_string(),
                    var_type: VariableType::Identifier,
                    default_value: "my_cli".to_string(),
                    required: true,
                    validation: Some(r"^[a-zA-Z][a-zA-Z0-9_]*$".to_string()),
                    choices: Vec::new(),
                },
                TemplateVariable {
                    name: "cli_framework".to_string(),
                    label: "CLI Framework".to_string(),
                    description: "Choose the command-line parsing framework".to_string(),
                    var_type: VariableType::Choice,
                    default_value: "clap".to_string(),
                    required: true,
                    validation: None,
                    choices: vec!["clap".to_string(), "structopt".to_string(), "argh".to_string()],
                },
            ],
            dependencies: vec![
                Dependency {
                    name: "clap".to_string(),
                    version: "4.0".to_string(),
                    features: vec!["derive".to_string()],
                    optional: false,
                    default_features: true,
                    git: None,
                    git_ref: None,
                    path: None,
                },
                Dependency {
                    name: "anyhow".to_string(),
                    version: "1.0".to_string(),
                    features: vec![],
                    optional: false,
                    default_features: true,
                    git: None,
                    git_ref: None,
                    path: None,
                },
            ],
            dev_dependencies: vec![],
            build_config: BuildConfiguration::default(),
            post_scripts: vec![],
            assets: vec![],
            min_rust_version: Some("1.70.0".to_string()),
            usage_count: 0,
            featured: true,
        };

        // Web API Template
        let web_api = ProjectTemplate {
            id: "rust-web-api".to_string(),
            name: "Web API Server".to_string(),
            description: "A REST API server using Axum with database integration".to_string(),
            version: "1.0.0".to_string(),
            author: "RAD IDE".to_string(),
            category: TemplateCategory::API,
            tags: vec!["rust".to_string(), "web".to_string(), "api".to_string(), "axum".to_string()],
            file_tree: self.create_web_api_structure(),
            variables: vec![
                TemplateVariable {
                    name: "project_name".to_string(),
                    label: "Project Name".to_string(),
                    description: "The name of your API server".to_string(),
                    var_type: VariableType::Identifier,
                    default_value: "my_api".to_string(),
                    required: true,
                    validation: Some(r"^[a-zA-Z][a-zA-Z0-9_]*$".to_string()),
                    choices: Vec::new(),
                },
                TemplateVariable {
                    name: "database".to_string(),
                    label: "Database".to_string(),
                    description: "Choose the database to integrate".to_string(),
                    var_type: VariableType::Choice,
                    default_value: "postgresql".to_string(),
                    required: true,
                    validation: None,
                    choices: vec!["postgresql".to_string(), "mysql".to_string(), "sqlite".to_string()],
                },
            ],
            dependencies: vec![
                Dependency {
                    name: "axum".to_string(),
                    version: "0.7".to_string(),
                    features: vec![],
                    optional: false,
                    default_features: true,
                    git: None,
                    git_ref: None,
                    path: None,
                },
                Dependency {
                    name: "tokio".to_string(),
                    version: "1.0".to_string(),
                    features: vec!["full".to_string()],
                    optional: false,
                    default_features: true,
                    git: None,
                    git_ref: None,
                    path: None,
                },
                Dependency {
                    name: "serde".to_string(),
                    version: "1.0".to_string(),
                    features: vec!["derive".to_string()],
                    optional: false,
                    default_features: true,
                    git: None,
                    git_ref: None,
                    path: None,
                },
            ],
            dev_dependencies: vec![],
            build_config: BuildConfiguration::default(),
            post_scripts: vec![
                PostScript {
                    name: "Database Setup".to_string(),
                    description: "Initialize database migrations".to_string(),
                    command: "cargo install sqlx-cli".to_string(),
                    working_dir: None,
                    required: false,
                    platforms: vec![Platform::All],
                },
            ],
            assets: vec![],
            min_rust_version: Some("1.75.0".to_string()),
            usage_count: 0,
            featured: true,
        };

        // Add templates to engine
        self.templates.insert(basic_app.id.clone(), basic_app);
        self.templates.insert(cli_app.id.clone(), cli_app);
        self.templates.insert(web_api.id.clone(), web_api);
    }

    /// Create basic application file structure
    fn create_basic_app_structure(&self) -> FileTreeNode {
        FileTreeNode {
            name: "{{project_name}}".to_string(),
            node_type: FileNodeType::Directory,
            content: None,
            children: vec![
                FileTreeNode {
                    name: "Cargo.toml".to_string(),
                    node_type: FileNodeType::File,
                    content: Some(self.get_basic_cargo_toml()),
                    children: vec![],
                    permissions: None,
                    is_template: true,
                },
                FileTreeNode {
                    name: "src".to_string(),
                    node_type: FileNodeType::Directory,
                    content: None,
                    children: vec![
                        FileTreeNode {
                            name: "main.rs".to_string(),
                            node_type: FileNodeType::File,
                            content: Some(self.get_basic_main_rs()),
                            children: vec![],
                            permissions: None,
                            is_template: true,
                        },
                    ],
                    permissions: None,
                    is_template: false,
                },
                FileTreeNode {
                    name: "README.md".to_string(),
                    node_type: FileNodeType::File,
                    content: Some(self.get_basic_readme()),
                    children: vec![],
                    permissions: None,
                    is_template: true,
                },
                FileTreeNode {
                    name: ".gitignore".to_string(),
                    node_type: FileNodeType::File,
                    content: Some(self.get_rust_gitignore()),
                    children: vec![],
                    permissions: None,
                    is_template: false,
                },
            ],
            permissions: None,
            is_template: true,
        }
    }

    /// Create CLI application file structure
    fn create_cli_app_structure(&self) -> FileTreeNode {
        FileTreeNode {
            name: "{{project_name}}".to_string(),
            node_type: FileNodeType::Directory,
            content: None,
            children: vec![
                FileTreeNode {
                    name: "Cargo.toml".to_string(),
                    node_type: FileNodeType::File,
                    content: Some(self.get_cli_cargo_toml()),
                    children: vec![],
                    permissions: None,
                    is_template: true,
                },
                FileTreeNode {
                    name: "src".to_string(),
                    node_type: FileNodeType::Directory,
                    content: None,
                    children: vec![
                        FileTreeNode {
                            name: "main.rs".to_string(),
                            node_type: FileNodeType::File,
                            content: Some(self.get_cli_main_rs()),
                            children: vec![],
                            permissions: None,
                            is_template: true,
                        },
                        FileTreeNode {
                            name: "cli.rs".to_string(),
                            node_type: FileNodeType::File,
                            content: Some(self.get_cli_module()),
                            children: vec![],
                            permissions: None,
                            is_template: true,
                        },
                    ],
                    permissions: None,
                    is_template: false,
                },
            ],
            permissions: None,
            is_template: true,
        }
    }

    /// Create web API file structure
    fn create_web_api_structure(&self) -> FileTreeNode {
        FileTreeNode {
            name: "{{project_name}}".to_string(),
            node_type: FileNodeType::Directory,
            content: None,
            children: vec![
                FileTreeNode {
                    name: "Cargo.toml".to_string(),
                    node_type: FileNodeType::File,
                    content: Some(self.get_web_api_cargo_toml()),
                    children: vec![],
                    permissions: None,
                    is_template: true,
                },
                FileTreeNode {
                    name: "src".to_string(),
                    node_type: FileNodeType::Directory,
                    content: None,
                    children: vec![
                        FileTreeNode {
                            name: "main.rs".to_string(),
                            node_type: FileNodeType::File,
                            content: Some(self.get_web_api_main_rs()),
                            children: vec![],
                            permissions: None,
                            is_template: true,
                        },
                        FileTreeNode {
                            name: "routes".to_string(),
                            node_type: FileNodeType::Directory,
                            content: None,
                            children: vec![
                                FileTreeNode {
                                    name: "mod.rs".to_string(),
                                    node_type: FileNodeType::File,
                                    content: Some("pub mod health;\npub mod api;".to_string()),
                                    children: vec![],
                                    permissions: None,
                                    is_template: false,
                                },
                            ],
                            permissions: None,
                            is_template: false,
                        },
                    ],
                    permissions: None,
                    is_template: false,
                },
            ],
            permissions: None,
            is_template: true,
        }
    }

    /// Get basic Cargo.toml template
    fn get_basic_cargo_toml(&self) -> String {
        r#"[package]
name = "{{project_name}}"
version = "0.1.0"  
edition = "2021"
authors = ["{{author_name}}"]
description = "A Rust application"
license = "MIT OR Apache-2.0"
readme = "README.md"

[dependencies]
"#.to_string()
    }

    /// Get basic main.rs template
    fn get_basic_main_rs(&self) -> String {
        r#"fn main() {
    println!("Hello from {{project_name}}!");
    
    // Your application logic here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Add your tests here
        assert_eq!(2 + 2, 4);
    }
}
"#.to_string()
    }

    /// Get basic README template
    fn get_basic_readme(&self) -> String {
        r#"# {{project_name}}

A Rust application created with RAD IDE.

## Description

{{project_description}}

## Installation

```bash
cargo install --path .
```

## Usage

```bash
{{project_name}}
```

## Development

```bash
# Run the application
cargo run

# Run tests
cargo test

# Build for release
cargo build --release
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT OR Apache-2.0 license.
"#.to_string()
    }

    /// Get Rust .gitignore template
    fn get_rust_gitignore(&self) -> String {
        r#"# Generated by Cargo
/target/

# IDE files
.vscode/
.idea/
*.swp
*.swo

# OS files
.DS_Store
Thumbs.db

# Logs
*.log

# Runtime data
pids
*.pid
*.seed
*.pid.lock

# Coverage directory used by tools like istanbul
coverage/

# Optional npm cache directory
.npm

# Optional REPL history
.node_repl_history

# Output of 'npm pack'
*.tgz

# Yarn Integrity file
.yarn-integrity

# dotenv environment variables file
.env
"#.to_string()
    }

    /// Get CLI Cargo.toml template
    fn get_cli_cargo_toml(&self) -> String {
        r#"[package]
name = "{{project_name}}"
version = "0.1.0"
edition = "2021"
authors = ["{{author_name}}"]
description = "A command-line tool"
license = "MIT OR Apache-2.0"

[[bin]]
name = "{{project_name}}"
path = "src/main.rs"

[dependencies]
clap = { version = "4.0", features = ["derive"] }
anyhow = "1.0"
"#.to_string()
    }

    /// Get CLI main.rs template
    fn get_cli_main_rs(&self) -> String {
        r#"use clap::Parser;
use anyhow::{Result, Context};

mod cli;

use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello { name } => {
            println!("Hello, {}!", name.unwrap_or_else(|| "World".to_string()));
        }
        Commands::Version => {
            println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}
"#.to_string()
    }

    /// Get CLI module template
    fn get_cli_module(&self) -> String {
        r#"use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Say hello to someone
    Hello {
        /// Name of the person to greet
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Show version information
    Version,
}
"#.to_string()
    }

    /// Get Web API Cargo.toml template
    fn get_web_api_cargo_toml(&self) -> String {
        r#"[package]
name = "{{project_name}}"
version = "0.1.0"
edition = "2021"
authors = ["{{author_name}}"]
description = "A web API server"
license = "MIT OR Apache-2.0"

[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tower = "0.4"
tower-http = { version = "0.5", features = ["cors"] }
tracing = "0.1"
tracing-subscriber = "0.3"
uuid = { version = "1.0", features = ["v4"] }
"#.to_string()
    }

    /// Get Web API main.rs template
    fn get_web_api_main_rs(&self) -> String {
        r#"use axum::{
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

mod routes;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::init();

    // Build our application with a route
    let app = Router::new()
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive());

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("{{project_name}} server listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
"#.to_string()
    }

    /// Start the project creation wizard
    pub fn start_wizard(&mut self) {
        self.wizard = ProjectWizard::new();
        self.wizard.current_step = WizardStep::TemplateSelection;
    }

    /// Render the project wizard UI
    pub fn render_wizard(&mut self, ui: &mut Ui) -> Option<ProjectConfiguration> {
        match self.wizard.current_step {
            WizardStep::TemplateSelection => self.render_template_selection(ui),
            WizardStep::ProjectConfiguration => self.render_project_configuration(ui),
            WizardStep::VariableConfiguration => self.render_variable_configuration(ui),
            WizardStep::DependencySelection => self.render_dependency_selection(ui),
            WizardStep::Review => self.render_review_step(ui),
            WizardStep::Creation => self.render_creation_step(ui),
            WizardStep::Complete => self.render_complete_step(ui),
        }
    }

    /// Render template selection step
    fn render_template_selection(&mut self, ui: &mut Ui) -> Option<ProjectConfiguration> {
        ui.heading("Choose a Project Template");

        // Search and filter controls
        ui.horizontal(|ui| {
            ui.label("Search:");
            ui.text_edit_singleline(&mut self.wizard.ui_state.search_query);
            
            ui.separator();
            
            ui.checkbox(&mut self.wizard.ui_state.show_featured_only, "Featured only");
        });

        // Category filter
        ui.horizontal(|ui| {
            ui.label("Category:");
            for category in &self.categories.clone() {
                let selected = self.wizard.ui_state.category_filter.as_ref() == Some(category);
                if ui.selectable_label(selected, format!("{:?}", category)).clicked() {
                    self.wizard.ui_state.category_filter = if selected {
                        None
                    } else {
                        Some(category.clone())
                    };
                }
            }
        });

        ui.separator();

        // Template grid
        let filtered_templates = self.get_filtered_templates();
        
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("template_grid")
                .num_columns(2)
                .spacing([20.0, 10.0])
                .show(ui, |ui| {
                    for (i, template) in filtered_templates.iter().enumerate() {
                        if i % 2 == 0 && i > 0 {
                            ui.end_row();
                        }

                        ui.group(|ui| {
                            ui.set_min_size(Vec2::new(300.0, 120.0));
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.heading(&template.name);
                                    if template.featured {
                                        ui.label(RichText::new("⭐").color(Color32::YELLOW));
                                    }
                                });
                                
                                ui.label(&template.description);
                                ui.label(format!("Version: {}", template.version));
                                
                                ui.horizontal(|ui| {
                                    for tag in template.tags.iter().take(3) {
                                        ui.small_button(tag);
                                    }
                                });

                                if ui.button("Select Template").clicked() {
                                    self.wizard.selected_template = Some(template.id.clone());
                                    self.wizard.current_step = WizardStep::ProjectConfiguration;
                                }
                            });
                        });
                    }
                });
        });

        None
    }

    /// Render project configuration step
    fn render_project_configuration(&mut self, ui: &mut Ui) -> Option<ProjectConfiguration> {
        ui.heading("Project Configuration");

        ui.horizontal(|ui| {
            ui.label("Project Name:");
            ui.text_edit_singleline(&mut self.wizard.project_config.name);
        });

        ui.horizontal(|ui| {
            ui.label("Project Path:");
            ui.text_edit_singleline(&mut format!("{}", self.wizard.project_config.path.display()));
            if ui.button("Browse").clicked() {
                // TODO: Open file dialog
            }
        });

        ui.horizontal(|ui| {
            ui.label("Description:");
            ui.text_edit_multiline(&mut self.wizard.project_config.description);
        });

        ui.horizontal(|ui| {
            ui.label("Author:");
            ui.text_edit_singleline(&mut self.wizard.project_config.author);
        });

        ui.horizontal(|ui| {
            ui.label("License:");
            ui.text_edit_singleline(&mut self.wizard.project_config.license);
        });

        ui.checkbox(&mut self.wizard.project_config.init_git, "Initialize Git repository");
        ui.checkbox(&mut self.wizard.project_config.initial_commit, "Create initial commit");

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("← Back").clicked() {
                self.wizard.current_step = WizardStep::TemplateSelection;
            }
            
            if ui.button("Next →").clicked() {
                self.wizard.current_step = WizardStep::VariableConfiguration;
            }
        });

        None
    }

    /// Render variable configuration step
    fn render_variable_configuration(&mut self, ui: &mut Ui) -> Option<ProjectConfiguration> {
        ui.heading("Template Variables");

        if let Some(template_id) = &self.wizard.selected_template {
            if let Some(template) = self.templates.get(template_id) {
                for variable in &template.variables {
                    ui.horizontal(|ui| {
                        ui.label(&variable.label);
                        if variable.required {
                            ui.label(RichText::new("*").color(Color32::RED));
                        }
                    });

                    let current_value = self.wizard.variable_values
                        .entry(variable.name.clone())
                        .or_insert_with(|| variable.default_value.clone());

                    match variable.var_type {
                        VariableType::Text | VariableType::Identifier => {
                            ui.text_edit_singleline(current_value);
                        }
                        VariableType::Boolean => {
                            let mut bool_val = current_value.parse::<bool>().unwrap_or(false);
                            if ui.checkbox(&mut bool_val, "").changed() {
                                *current_value = bool_val.to_string();
                            }
                        }
                        VariableType::Choice => {
                            egui::ComboBox::from_id_source(&variable.name)
                                .selected_text(current_value.as_str())
                                .show_ui(ui, |ui| {
                                    for choice in &variable.choices {
                                        ui.selectable_value(current_value, choice.clone(), choice);
                                    }
                                });
                        }
                        _ => {
                            ui.text_edit_singleline(current_value);
                        }
                    }

                    if !variable.description.is_empty() {
                        ui.small(&variable.description);
                    }
                    ui.separator();
                }
            }
        }

        ui.horizontal(|ui| {
            if ui.button("← Back").clicked() {
                self.wizard.current_step = WizardStep::ProjectConfiguration;
            }
            
            if ui.button("Next →").clicked() {
                self.wizard.current_step = WizardStep::Review;
            }
        });

        None
    }

    /// Render dependency selection step
    fn render_dependency_selection(&mut self, ui: &mut Ui) -> Option<ProjectConfiguration> {
        ui.heading("Dependencies");
        
        // This would show optional dependencies that can be added/removed
        ui.label("Dependency management coming soon...");

        ui.horizontal(|ui| {
            if ui.button("← Back").clicked() {
                self.wizard.current_step = WizardStep::VariableConfiguration;
            }
            
            if ui.button("Next →").clicked() {
                self.wizard.current_step = WizardStep::Review;
            }
        });

        None
    }

    /// Render review step
    fn render_review_step(&mut self, ui: &mut Ui) -> Option<ProjectConfiguration> {
        ui.heading("Review Configuration");

        ui.group(|ui| {
            ui.label(format!("Project Name: {}", self.wizard.project_config.name));
            ui.label(format!("Path: {}", self.wizard.project_config.path.display()));
            ui.label(format!("Author: {}", self.wizard.project_config.author));
            ui.label(format!("License: {}", self.wizard.project_config.license));
            
            if let Some(template_id) = &self.wizard.selected_template {
                if let Some(template) = self.templates.get(template_id) {
                    ui.label(format!("Template: {}", template.name));
                }
            }
        });

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("← Back").clicked() {
                self.wizard.current_step = WizardStep::VariableConfiguration;
            }
            
            if ui.button("Create Project").clicked() {
                self.wizard.current_step = WizardStep::Creation;
            }
        });

        None
    }

    /// Render creation step
    fn render_creation_step(&mut self, ui: &mut Ui) -> Option<ProjectConfiguration> {
        ui.heading("Creating Project...");
        
        ui.spinner();
        ui.label("Setting up your project structure...");

        // In a real implementation, this would create the project asynchronously
        // For now, we'll simulate completion
        self.wizard.current_step = WizardStep::Complete;

        None
    }

    /// Render completion step
    fn render_complete_step(&mut self, ui: &mut Ui) -> Option<ProjectConfiguration> {
        ui.heading("Project Created Successfully! 🎉");

        ui.label("Your project has been created and is ready for development.");
        ui.label(format!("Location: {}", self.wizard.project_config.path.display()));

        if ui.button("Open Project").clicked() {
            // Return the project configuration to signal completion
            return Some(self.wizard.project_config.clone());
        }

        if ui.button("Create Another Project").clicked() {
            self.start_wizard();
        }

        None
    }

    /// Get filtered templates based on current UI state
    fn get_filtered_templates(&self) -> Vec<&ProjectTemplate> {
        self.templates.values()
            .filter(|template| {
                // Category filter
                if let Some(ref category) = self.wizard.ui_state.category_filter {
                    if template.category != *category {
                        return false;
                    }
                }

                // Featured filter
                if self.wizard.ui_state.show_featured_only && !template.featured {
                    return false;
                }

                // Search filter
                if !self.wizard.ui_state.search_query.is_empty() {
                    let query = self.wizard.ui_state.search_query.to_lowercase();
                    let name_match = template.name.to_lowercase().contains(&query);
                    let desc_match = template.description.to_lowercase().contains(&query);
                    let tag_match = template.tags.iter().any(|tag| tag.to_lowercase().contains(&query));
                    
                    if !name_match && !desc_match && !tag_match {
                        return false;
                    }
                }

                true
            })
            .collect()
    }

    /// Create a new project from template
    pub fn create_project(&mut self, template_id: &str, config: &ProjectConfiguration, variables: &HashMap<String, String>) -> Result<(), String> {
        let template = self.templates.get(template_id)
            .ok_or_else(|| "Template not found".to_string())?;

        // TODO: Implement actual project creation
        // This would:
        // 1. Create directory structure
        // 2. Process template files
        // 3. Replace variables
        // 4. Create Cargo.toml with dependencies
        // 5. Run post-creation scripts
        // 6. Initialize git if requested

        // Update usage count
        self.templates.get_mut(template_id).unwrap().usage_count += 1;
        
        // Add to recent templates
        self.recent_templates.insert(0, template_id.to_string());
        self.recent_templates.truncate(10);

        Ok(())
    }

    /// Get template by ID
    pub fn get_template(&self, id: &str) -> Option<&ProjectTemplate> {
        self.templates.get(id)
    }

    /// Add custom template
    pub fn add_custom_template(&mut self, template: ProjectTemplate) {
        let id = template.id.clone();
        self.user_templates.insert(id.clone(), template.clone());
        self.templates.insert(id, template);
    }

    /// Export template to file
    pub fn export_template(&self, template_id: &str) -> Result<String, String> {
        let template = self.templates.get(template_id)
            .ok_or_else(|| "Template not found".to_string())?;
        
        serde_json::to_string_pretty(template)
            .map_err(|e| format!("Serialization error: {}", e))
    }

    /// Import template from file
    pub fn import_template(&mut self, json: &str) -> Result<String, String> {
        let template: ProjectTemplate = serde_json::from_str(json)
            .map_err(|e| format!("Deserialization error: {}", e))?;
        
        let id = template.id.clone();
        self.add_custom_template(template);
        Ok(id)
    }
}

impl ProjectWizard {
    fn new() -> Self {
        Self {
            current_step: WizardStep::TemplateSelection,
            selected_template: None,
            project_config: ProjectConfiguration::default(),
            variable_values: HashMap::new(),
            ui_state: WizardUIState::default(),
        }
    }
}

impl Default for ProjectConfiguration {
    fn default() -> Self {
        Self {
            name: "new_project".to_string(),
            path: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            description: String::new(),
            author: "Your Name".to_string(),
            license: "MIT OR Apache-2.0".to_string(),
            init_git: true,
            initial_commit: true,
            visibility: ProjectVisibility::Private,
        }
    }
}

impl Default for WizardUIState {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            category_filter: None,
            show_featured_only: false,
            show_preview: false,
            advanced_options: false,
        }
    }
}

impl Default for ScaffoldingSettings {
    fn default() -> Self {
        Self {
            default_project_path: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            default_author: "Your Name".to_string(),
            default_license: "MIT OR Apache-2.0".to_string(),
            auto_init_git: true,
            auto_initial_commit: true,
            update_check_frequency: 7,
            analytics_enabled: true,
        }
    }
}

impl Default for BuildConfiguration {
    fn default() -> Self {
        Self {
            build_scripts: Vec::new(),
            env_vars: HashMap::new(),
            rustflags: Vec::new(),
            targets: vec!["x86_64-unknown-linux-gnu".to_string()],
            profiles: HashMap::new(),
        }
    }
}