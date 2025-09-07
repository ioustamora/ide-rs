//! Workspace Management System
//!
//! Provides multi-project workspace management similar to VS Code workspaces,
//! allowing users to work with multiple projects simultaneously with shared
//! settings and cross-project features.

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::editor::project_manager::project::IdeProject;

/// Multi-project workspace
#[derive(Serialize, Deserialize, Clone)]
pub struct Workspace {
    /// Workspace metadata
    pub metadata: WorkspaceMetadata,
    /// Projects in this workspace
    pub projects: Vec<ProjectReference>,
    /// Workspace-wide settings
    pub settings: WorkspaceSettings,
    /// Recent files across all projects
    pub recent_files: Vec<PathBuf>,
    /// Workspace-specific tasks and build configurations
    pub tasks: Vec<WorkspaceTask>,
}

/// Workspace metadata
#[derive(Serialize, Deserialize, Clone)]
pub struct WorkspaceMetadata {
    /// Workspace name
    pub name: String,
    /// Workspace description
    pub description: String,
    /// Workspace root directory
    pub root_path: PathBuf,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last modified timestamp
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// Workspace version for compatibility
    pub version: String,
}

/// Reference to a project within a workspace
#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectReference {
    /// Project name
    pub name: String,
    /// Relative path from workspace root
    pub path: PathBuf,
    /// Whether this project is currently active
    pub active: bool,
    /// Project-specific settings overrides
    pub settings_overrides: HashMap<String, String>,
    /// Last opened timestamp
    pub last_opened: Option<chrono::DateTime<chrono::Utc>>,
}

/// Workspace-wide settings
#[derive(Serialize, Deserialize, Clone)]
pub struct WorkspaceSettings {
    /// Default project template
    pub default_template: Option<String>,
    /// Shared build configurations
    pub build_configurations: HashMap<String, BuildConfiguration>,
    /// Code formatting settings
    pub formatting: FormattingSettings,
    /// Version control settings
    pub version_control: VersionControlSettings,
    /// Extension settings
    pub extensions: HashMap<String, serde_json::Value>,
}

/// Build configuration for workspace tasks
#[derive(Serialize, Deserialize, Clone)]
pub struct BuildConfiguration {
    /// Configuration name
    pub name: String,
    /// Build command
    pub command: String,
    /// Build arguments
    pub args: Vec<String>,
    /// Working directory (relative to workspace)
    pub working_directory: Option<PathBuf>,
    /// Environment variables
    pub env: HashMap<String, String>,
}

/// Code formatting settings
#[derive(Serialize, Deserialize, Clone)]
pub struct FormattingSettings {
    /// Tab size
    pub tab_size: usize,
    /// Use spaces instead of tabs
    pub use_spaces: bool,
    /// Auto-format on save
    pub format_on_save: bool,
    /// Language-specific formatting rules
    pub language_rules: HashMap<String, serde_json::Value>,
}

/// Version control settings
#[derive(Serialize, Deserialize, Clone)]
pub struct VersionControlSettings {
    /// Auto-fetch remote changes
    pub auto_fetch: bool,
    /// Show git status in file explorer
    pub show_git_status: bool,
    /// Auto-stage changes
    pub auto_stage: bool,
    /// Ignored file patterns
    pub ignore_patterns: Vec<String>,
}

/// Workspace task definition
#[derive(Serialize, Deserialize, Clone)]
pub struct WorkspaceTask {
    /// Task name
    pub name: String,
    /// Task description
    pub description: String,
    /// Command to execute
    pub command: String,
    /// Command arguments
    pub args: Vec<String>,
    /// Working directory
    pub working_directory: Option<PathBuf>,
    /// Environment variables
    pub env: HashMap<String, String>,
    /// Task group (build, test, etc.)
    pub group: TaskGroup,
    /// Whether this task runs in background
    pub background: bool,
}

/// Task groupings
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum TaskGroup {
    Build,
    Test,
    Debug,
    Deploy,
    Custom(String),
}

/// Workspace manager for multi-project environments
pub struct WorkspaceManager {
    /// Currently active workspace
    pub current_workspace: Option<Workspace>,
    /// Recently opened workspaces
    pub recent_workspaces: Vec<PathBuf>,
    /// Maximum recent workspaces to track
    pub max_recent_workspaces: usize,
    /// Loaded projects cache
    pub loaded_projects: HashMap<PathBuf, IdeProject>,
    /// Active project (currently focused)
    pub active_project: Option<PathBuf>,
}

impl WorkspaceManager {
    /// Create a new workspace manager
    pub fn new() -> Self {
        Self {
            current_workspace: None,
            recent_workspaces: Vec::new(),
            max_recent_workspaces: 10,
            loaded_projects: HashMap::new(),
            active_project: None,
        }
    }

    /// Create a new workspace
    pub fn create_workspace(&mut self, name: String, root_path: PathBuf) -> Result<(), String> {
        if root_path.exists() && !root_path.is_dir() {
            return Err("Workspace path must be a directory".to_string());
        }

        // Create workspace directory if it doesn't exist
        if !root_path.exists() {
            std::fs::create_dir_all(&root_path)
                .map_err(|e| format!("Failed to create workspace directory: {}", e))?;
        }

        let workspace = Workspace {
            metadata: WorkspaceMetadata {
                name: name.clone(),
                description: String::new(),
                root_path: root_path.clone(),
                created_at: chrono::Utc::now(),
                modified_at: chrono::Utc::now(),
                version: "1.0.0".to_string(),
            },
            projects: Vec::new(),
            settings: WorkspaceSettings {
                default_template: None,
                build_configurations: HashMap::new(),
                formatting: FormattingSettings {
                    tab_size: 4,
                    use_spaces: true,
                    format_on_save: true,
                    language_rules: HashMap::new(),
                },
                version_control: VersionControlSettings {
                    auto_fetch: false,
                    show_git_status: true,
                    auto_stage: false,
                    ignore_patterns: vec![
                        ".git/".to_string(),
                        "target/".to_string(),
                        "node_modules/".to_string(),
                    ],
                },
                extensions: HashMap::new(),
            },
            recent_files: Vec::new(),
            tasks: Vec::new(),
        };

        // Save workspace file
        self.save_workspace(&workspace)?;
        self.current_workspace = Some(workspace);
        
        // Add to recent workspaces
        self.add_to_recent_workspaces(root_path);

        Ok(())
    }

    /// Open an existing workspace
    pub fn open_workspace(&mut self, workspace_path: PathBuf) -> Result<(), String> {
        let workspace_file = workspace_path.join(".ide-workspace.json");
        
        if !workspace_file.exists() {
            return Err("Workspace file not found".to_string());
        }

        let content = std::fs::read_to_string(&workspace_file)
            .map_err(|e| format!("Failed to read workspace file: {}", e))?;
            
        let workspace: Workspace = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse workspace file: {}", e))?;

        self.current_workspace = Some(workspace);
        self.add_to_recent_workspaces(workspace_path);

        Ok(())
    }

    /// Save the current workspace
    pub fn save_workspace(&self, workspace: &Workspace) -> Result<(), String> {
        let workspace_file = workspace.metadata.root_path.join(".ide-workspace.json");
        
        let content = serde_json::to_string_pretty(workspace)
            .map_err(|e| format!("Failed to serialize workspace: {}", e))?;
            
        std::fs::write(&workspace_file, content)
            .map_err(|e| format!("Failed to write workspace file: {}", e))?;

        Ok(())
    }

    /// Add a project to the current workspace
    pub fn add_project_to_workspace(&mut self, project_path: PathBuf, project_name: String) -> Result<(), String> {
        {
            let workspace = self.current_workspace.as_mut()
                .ok_or("No workspace is currently open")?;

            // Make path relative to workspace root
            let relative_path = project_path.strip_prefix(&workspace.metadata.root_path)
                .map_err(|_| "Project must be within workspace directory")?
                .to_path_buf();

            let project_ref = ProjectReference {
                name: project_name,
                path: relative_path,
                active: workspace.projects.is_empty(), // First project becomes active
                settings_overrides: HashMap::new(),
                last_opened: Some(chrono::Utc::now()),
            };

            workspace.projects.push(project_ref);
            workspace.metadata.modified_at = chrono::Utc::now();
        }

        if let Some(workspace) = &self.current_workspace {
            self.save_workspace(workspace)?;
        }
        Ok(())
    }

    /// Remove a project from the current workspace
    pub fn remove_project_from_workspace(&mut self, project_path: &Path) -> Result<(), String> {
        {
            let workspace = self.current_workspace.as_mut()
                .ok_or("No workspace is currently open")?;

            let relative_path = project_path.strip_prefix(&workspace.metadata.root_path)
                .map_err(|_| "Invalid project path")?;

            workspace.projects.retain(|p| p.path != relative_path);
            workspace.metadata.modified_at = chrono::Utc::now();
        }

        if let Some(workspace) = &self.current_workspace {
            self.save_workspace(workspace)?;
        }
        Ok(())
    }

    /// Set the active project in the workspace
    pub fn set_active_project(&mut self, project_path: PathBuf) -> Result<(), String> {
        {
            let workspace = self.current_workspace.as_mut()
                .ok_or("No workspace is currently open")?;

            let relative_path = project_path.strip_prefix(&workspace.metadata.root_path)
                .map_err(|_| "Invalid project path")?;

            // Deactivate all projects
            for project in &mut workspace.projects {
                project.active = false;
            }

            // Activate the specified project
            for project in &mut workspace.projects {
                if project.path == relative_path {
                    project.active = true;
                    project.last_opened = Some(chrono::Utc::now());
                    break;
                }
            }

            self.active_project = Some(project_path);
            workspace.metadata.modified_at = chrono::Utc::now();
        }

        if let Some(workspace) = &self.current_workspace {
            self.save_workspace(workspace)?;
        }

        Ok(())
    }

    /// Get all projects in the current workspace
    pub fn get_workspace_projects(&self) -> Vec<ProjectReference> {
        self.current_workspace
            .as_ref()
            .map(|w| w.projects.clone())
            .unwrap_or_default()
    }

    /// Add a workspace task
    pub fn add_workspace_task(&mut self, task: WorkspaceTask) -> Result<(), String> {
        {
            let workspace = self.current_workspace.as_mut()
                .ok_or("No workspace is currently open")?;

            workspace.tasks.push(task);
            workspace.metadata.modified_at = chrono::Utc::now();
        }

        if let Some(workspace) = &self.current_workspace {
            self.save_workspace(workspace)?;
        }

        Ok(())
    }

    /// Get workspace tasks by group
    pub fn get_tasks_by_group(&self, group: &TaskGroup) -> Vec<&WorkspaceTask> {
        self.current_workspace
            .as_ref()
            .map(|w| w.tasks.iter().filter(|t| &t.group == group).collect())
            .unwrap_or_default()
    }

    /// Execute a workspace task
    pub fn execute_task(&self, task_name: &str) -> Result<(), String> {
        let workspace = self.current_workspace.as_ref()
            .ok_or("No workspace is currently open")?;

        let task = workspace.tasks.iter()
            .find(|t| t.name == task_name)
            .ok_or("Task not found")?;

        let working_dir = task.working_directory
            .as_ref()
            .map(|p| workspace.metadata.root_path.join(p))
            .unwrap_or_else(|| workspace.metadata.root_path.clone());

        let mut command = std::process::Command::new(&task.command);
        command.args(&task.args)
               .current_dir(working_dir)
               .envs(&task.env);

        if task.background {
            command.spawn()
                .map_err(|e| format!("Failed to start task: {}", e))?;
        } else {
            let status = command.status()
                .map_err(|e| format!("Failed to execute task: {}", e))?;
                
            if !status.success() {
                return Err(format!("Task '{}' failed with exit code: {:?}", task_name, status.code()));
            }
        }

        Ok(())
    }

    /// Add file to recent files across workspace
    pub fn add_recent_file(&mut self, file_path: PathBuf) {
        if let Some(workspace) = &mut self.current_workspace {
            // Remove if already exists
            workspace.recent_files.retain(|p| p != &file_path);
            
            // Add to front
            workspace.recent_files.insert(0, file_path);
            
            // Limit to reasonable number
            workspace.recent_files.truncate(20);
            
            workspace.metadata.modified_at = chrono::Utc::now();
        }
    }

    /// Get recent files from workspace
    pub fn get_recent_files(&self) -> Vec<PathBuf> {
        self.current_workspace
            .as_ref()
            .map(|w| w.recent_files.clone())
            .unwrap_or_default()
    }

    /// Search for files across the workspace
    pub fn search_workspace_files(&self, query: &str) -> Vec<PathBuf> {
        let workspace = match &self.current_workspace {
            Some(w) => w,
            None => return Vec::new(),
        };

        let mut results = Vec::new();
        let query_lower = query.to_lowercase();

        // Search in all project directories
        for project_ref in &workspace.projects {
            let project_path = workspace.metadata.root_path.join(&project_ref.path);
            if let Ok(entries) = std::fs::read_dir(project_path) {
                for entry in entries.flatten() {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        if file_name.to_lowercase().contains(&query_lower) {
                            results.push(entry.path());
                        }
                    }
                }
            }
        }

        results
    }

    /// Add workspace path to recent workspaces
    fn add_to_recent_workspaces(&mut self, workspace_path: PathBuf) {
        // Remove if already exists
        self.recent_workspaces.retain(|p| p != &workspace_path);
        
        // Add to front
        self.recent_workspaces.insert(0, workspace_path);
        
        // Limit to max recent workspaces
        self.recent_workspaces.truncate(self.max_recent_workspaces);
    }

    /// Get recently opened workspaces
    pub fn get_recent_workspaces(&self) -> &[PathBuf] {
        &self.recent_workspaces
    }

    /// Close the current workspace
    pub fn close_workspace(&mut self) -> Result<(), String> {
        if let Some(workspace) = &self.current_workspace {
            self.save_workspace(workspace)?;
        }
        
        self.current_workspace = None;
        self.loaded_projects.clear();
        self.active_project = None;
        
        Ok(())
    }
}

impl Default for WorkspaceManager {
    fn default() -> Self {
        Self::new()
    }
}