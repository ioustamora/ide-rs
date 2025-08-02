//! Project Templates and Scaffolding
//!
//! Handles project template definitions, template processing, and
//! new project scaffolding functionality.

use std::path::PathBuf;
use std::collections::HashMap;
use super::project::ComponentData;
// Template system integration would go here

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

/// Template processing utilities
pub struct TemplateProcessor;

impl TemplateProcessor {
    /// Process template content with variable substitution
    pub fn process_template_content(content: &str, project_name: &str, config: &TemplateConfig) -> String {
        let mut processed = content.to_string();
        
        // Replace common variables
        processed = processed.replace("{{PROJECT_NAME}}", project_name);
        processed = processed.replace("{{PROJECT_NAME_SNAKE}}", &project_name.to_lowercase().replace('-', "_"));
        processed = processed.replace("{{PROJECT_NAME_PASCAL}}", &Self::to_pascal_case(project_name));
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
    fn to_pascal_case(s: &str) -> String {
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
}

/// Create default project templates
pub fn create_default_templates() -> Vec<ProjectTemplate> {
    vec![
        // GUI Application Template
        ProjectTemplate {
            name: "GUI Application".to_string(),
            description: "A GUI application using egui framework".to_string(),
            category: "Application".to_string(),
            files: vec![
                TemplateFile {
                    path: PathBuf::from("Cargo.toml"),
                    content: get_gui_cargo_template(),
                    executable: false,
                },
                TemplateFile {
                    path: PathBuf::from("src/main.rs"),
                    content: get_gui_main_template(),
                    executable: false,
                },
                TemplateFile {
                    path: PathBuf::from("README.md"),
                    content: get_readme_template(),
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
        // Console Application Template
        ProjectTemplate {
            name: "Console Application".to_string(),
            description: "A command-line application".to_string(),
            category: "Application".to_string(),
            files: vec![
                TemplateFile {
                    path: PathBuf::from("Cargo.toml"),
                    content: get_console_cargo_template(),
                    executable: false,
                },
                TemplateFile {
                    path: PathBuf::from("src/main.rs"),
                    content: get_console_main_template(),
                    executable: false,
                },
                TemplateFile {
                    path: PathBuf::from("README.md"),
                    content: get_readme_template(),
                    executable: false,
                },
            ],
            default_components: Vec::new(),
            config: TemplateConfig {
                min_rust_version: "1.75.0".to_string(),
                default_dependencies: vec!["clap".to_string()],
                variables: HashMap::new(),
            },
        },
        // Library Template
        ProjectTemplate {
            name: "Library".to_string(),
            description: "A Rust library crate".to_string(),
            category: "Library".to_string(),
            files: vec![
                TemplateFile {
                    path: PathBuf::from("Cargo.toml"),
                    content: get_library_cargo_template(),
                    executable: false,
                },
                TemplateFile {
                    path: PathBuf::from("src/lib.rs"),
                    content: get_library_lib_template(),
                    executable: false,
                },
                TemplateFile {
                    path: PathBuf::from("README.md"),
                    content: get_readme_template(),
                    executable: false,
                },
            ],
            default_components: Vec::new(),
            config: TemplateConfig {
                min_rust_version: "1.75.0".to_string(),
                default_dependencies: Vec::new(),
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

/// Get console application Cargo.toml template
fn get_console_cargo_template() -> String {
    r#"[package]
name = "{{PROJECT_NAME}}"
version = "0.1.0"
edition = "2021"
authors = ["{{AUTHOR}}"]

[dependencies]
clap = { version = "4.0", features = ["derive"] }
"#.to_string()
}

/// Get library Cargo.toml template
fn get_library_cargo_template() -> String {
    r#"[package]
name = "{{PROJECT_NAME}}"
version = "0.1.0"
edition = "2021"
authors = ["{{AUTHOR}}"]

[dependencies]
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

/// Get console application main.rs template
fn get_console_main_template() -> String {
    r#"//! {{PROJECT_NAME}} - Generated by RAD IDE
//! 
//! A console application created with RAD IDE.
//! Author: {{AUTHOR}}
//! Created: {{DATE}}

use clap::Parser;

/// {{PROJECT_NAME}} command-line interface
#[derive(Parser)]
#[command(name = "{{PROJECT_NAME}}")]
#[command(about = "A console application created with RAD IDE")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Input file
    #[arg(value_name = "FILE")]
    input: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    
    if cli.verbose {
        println!("Verbose mode enabled");
    }
    
    match cli.input {
        Some(input) => {
            println!("Processing input: {}", input);
            // Add your application logic here
        }
        None => {
            println!("Welcome to {{PROJECT_NAME}}!");
            println!("Use --help for usage information.");
        }
    }
}
"#.to_string()
}

/// Get library lib.rs template
fn get_library_lib_template() -> String {
    r#"//! {{PROJECT_NAME}} - Generated by RAD IDE
//! 
//! A Rust library created with RAD IDE.
//! Author: {{AUTHOR}}
//! Created: {{DATE}}

/// This is the main library module.
/// 
/// Add your library's public API here.
pub fn hello_world() -> String {
    "Hello from {{PROJECT_NAME}}!".to_string()
}

/// Example function that demonstrates basic functionality
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        let result = hello_world();
        assert!(result.contains("{{PROJECT_NAME}}"));
    }

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
"#.to_string()
}

/// Get README.md template
fn get_readme_template() -> String {
    r#"# {{PROJECT_NAME}}

A Rust project created with RAD IDE.

## Description

{{PROJECT_NAME}} is a Rust project built using RAD IDE.

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

# Run tests
cargo test
```

## Features

- Created with RAD IDE
- Modern Rust project structure
- Extensible architecture

## Author

{{AUTHOR}} - {{DATE}}

## License

This project is licensed under the MIT License.
"#.to_string()
}