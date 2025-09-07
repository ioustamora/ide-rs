//! Git Integration System
//!
//! Provides comprehensive Git version control integration with visual diff,
//! branch management, commit history, and file status tracking.

use egui::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Git integration manager
pub struct GitIntegration {
    /// Current repository path
    pub repository_path: Option<PathBuf>,
    /// Repository status cache
    pub status_cache: GitStatusCache,
    /// Branch information
    pub branch_info: BranchInfo,
    /// Commit history cache
    pub commit_history: Vec<GitCommit>,
    /// File diff cache
    pub diff_cache: HashMap<PathBuf, FileDiff>,
    /// Git configuration
    pub config: GitConfig,
    /// UI state
    pub ui_state: GitUIState,
}

/// Git repository status cache
pub struct GitStatusCache {
    /// Modified files
    pub modified: Vec<PathBuf>,
    /// Added files (staged)
    pub added: Vec<PathBuf>,
    /// Deleted files
    pub deleted: Vec<PathBuf>,
    /// Untracked files
    pub untracked: Vec<PathBuf>,
    /// Staged files
    pub staged: Vec<PathBuf>,
    /// Last update time
    pub last_updated: std::time::Instant,
    /// Update interval in seconds
    pub update_interval: u64,
}

impl Default for GitStatusCache {
    fn default() -> Self {
        Self {
            modified: Vec::new(),
            added: Vec::new(),
            deleted: Vec::new(),
            untracked: Vec::new(),
            staged: Vec::new(),
            last_updated: std::time::Instant::now(),
            update_interval: 30,
        }
    }
}

/// Branch information
#[derive(Default)]
pub struct BranchInfo {
    /// Current branch name
    pub current_branch: String,
    /// All local branches
    pub local_branches: Vec<String>,
    /// Remote branches
    pub remote_branches: Vec<String>,
    /// Upstream branch (if any)
    pub upstream: Option<String>,
    /// Branch tracking info
    pub tracking_info: Option<TrackingInfo>,
}

/// Branch tracking information
#[derive(Debug, Clone)]
pub struct TrackingInfo {
    /// Commits ahead of upstream
    pub ahead: usize,
    /// Commits behind upstream
    pub behind: usize,
    /// Remote name
    pub remote: String,
}

/// Git commit information
#[derive(Debug, Clone)]
pub struct GitCommit {
    /// Commit hash (full)
    pub hash: String,
    /// Short hash (7 characters)
    pub short_hash: String,
    /// Commit message
    pub message: String,
    /// Author name
    pub author: String,
    /// Author email
    pub author_email: String,
    /// Commit timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Changed files count
    pub files_changed: usize,
    /// Lines added
    pub lines_added: usize,
    /// Lines deleted
    pub lines_deleted: usize,
}

/// File diff information
#[derive(Debug, Clone)]
pub struct FileDiff {
    /// File path
    pub file_path: PathBuf,
    /// Diff hunks
    pub hunks: Vec<DiffHunk>,
    /// File status
    pub status: FileStatus,
    /// Old file path (for renames)
    pub old_path: Option<PathBuf>,
}

/// Individual diff hunk
#[derive(Debug, Clone)]
pub struct DiffHunk {
    /// Old line range (start, count)
    pub old_range: (usize, usize),
    /// New line range (start, count)  
    pub new_range: (usize, usize),
    /// Diff lines
    pub lines: Vec<DiffLine>,
    /// Context around the change
    pub context: String,
}

/// Single line in a diff
#[derive(Debug, Clone)]
pub struct DiffLine {
    /// Line type (added, removed, context)
    pub line_type: DiffLineType,
    /// Line content
    pub content: String,
    /// Old line number (if applicable)
    pub old_line_number: Option<usize>,
    /// New line number (if applicable)
    pub new_line_number: Option<usize>,
}

/// Type of diff line
#[derive(Debug, Clone, PartialEq)]
pub enum DiffLineType {
    /// Line added
    Added,
    /// Line removed
    Removed,
    /// Context line (unchanged)
    Context,
    /// No newline at end of file
    NoNewline,
}

/// File status in Git
#[derive(Debug, Clone, PartialEq)]
pub enum FileStatus {
    /// Unmodified
    Unmodified,
    /// Modified but not staged
    Modified,
    /// Staged for commit
    Staged,
    /// New file (untracked)
    Untracked,
    /// Deleted
    Deleted,
    /// Renamed
    Renamed,
    /// Copied
    Copied,
    /// Both modified (merge conflict)
    Conflicted,
}

/// Git configuration
#[derive(Debug, Clone)]
pub struct GitConfig {
    /// User name
    pub user_name: String,
    /// User email
    pub user_email: String,
    /// Default editor
    pub editor: String,
    /// Auto-fetch interval in minutes
    pub auto_fetch_interval: u32,
    /// Show file status in explorer
    pub show_file_status: bool,
    /// Automatically stage modified files
    pub auto_stage_modified: bool,
}

/// Git UI state
#[derive(Default)]
pub struct GitUIState {
    /// Show commit dialog
    pub show_commit_dialog: bool,
    /// Commit message being typed
    pub commit_message: String,
    /// Show branch selector
    pub show_branch_selector: bool,
    /// Show history dialog
    pub show_history_dialog: bool,
    /// Selected files for staging
    pub selected_files: Vec<PathBuf>,
    /// Show diff for file
    pub show_diff_for: Option<PathBuf>,
    /// Expanded folders in file tree
    pub expanded_folders: std::collections::HashSet<PathBuf>,
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            user_name: "User".to_string(),
            user_email: "user@example.com".to_string(),
            editor: "code".to_string(),
            auto_fetch_interval: 5,
            show_file_status: true,
            auto_stage_modified: false,
        }
    }
}

impl GitIntegration {
    /// Create new Git integration
    pub fn new() -> Self {
        Self {
            repository_path: None,
            status_cache: GitStatusCache::default(),
            branch_info: BranchInfo::default(),
            commit_history: Vec::new(),
            diff_cache: HashMap::new(),
            config: GitConfig::default(),
            ui_state: GitUIState::default(),
        }
    }

    /// Initialize Git integration for a repository
    pub fn init_repository(&mut self, path: PathBuf) -> Result<(), GitError> {
        // Check if path is a Git repository
        if !self.is_git_repository(&path)? {
            return Err(GitError::NotARepository(path));
        }

        self.repository_path = Some(path);
        self.load_git_config()?;
        self.refresh_status()?;
        self.refresh_branches()?;
        
        Ok(())
    }

    /// Check if path is a Git repository
    pub fn is_git_repository(&self, path: &Path) -> Result<bool, GitError> {
        let git_dir = path.join(".git");
        Ok(git_dir.exists())
    }

    /// Refresh Git status
    pub fn refresh_status(&mut self) -> Result<(), GitError> {
        let repo_path = self.repository_path.as_ref()
            .ok_or(GitError::NoRepository)?;

        // Run git status --porcelain
        let output = Command::new("git")
            .arg("status")
            .arg("--porcelain")
            .current_dir(repo_path)
            .output()
            .map_err(GitError::CommandFailed)?;

        if !output.status.success() {
            return Err(GitError::CommandFailed(
                std::io::Error::new(std::io::ErrorKind::Other, "Git status failed")
            ));
        }

        let status_output = String::from_utf8_lossy(&output.stdout);
        self.parse_git_status(&status_output);
        self.status_cache.last_updated = std::time::Instant::now();

        Ok(())
    }

    /// Parse git status output
    fn parse_git_status(&mut self, output: &str) {
        self.status_cache.modified.clear();
        self.status_cache.added.clear();
        self.status_cache.deleted.clear();
        self.status_cache.untracked.clear();
        self.status_cache.staged.clear();

        for line in output.lines() {
            if line.len() < 3 {
                continue;
            }

            let status_chars = &line[0..2];
            let file_path = PathBuf::from(&line[3..]);

            match status_chars {
                " M" => self.status_cache.modified.push(file_path),
                "M " | "MM" => self.status_cache.staged.push(file_path),
                "A " => self.status_cache.added.push(file_path),
                " D" => self.status_cache.deleted.push(file_path),
                "??" => self.status_cache.untracked.push(file_path),
                _ => {} // Handle other statuses as needed
            }
        }
    }

    /// Refresh branch information
    pub fn refresh_branches(&mut self) -> Result<(), GitError> {
        let repo_path = self.repository_path.as_ref()
            .ok_or(GitError::NoRepository)?;

        // Get current branch
        let current_output = Command::new("git")
            .arg("branch")
            .arg("--show-current")
            .current_dir(repo_path)
            .output()
            .map_err(GitError::CommandFailed)?;

        self.branch_info.current_branch = String::from_utf8_lossy(&current_output.stdout)
            .trim().to_string();

        // Get all local branches
        let branches_output = Command::new("git")
            .arg("branch")
            .current_dir(repo_path)
            .output()
            .map_err(GitError::CommandFailed)?;

        let branches_text = String::from_utf8_lossy(&branches_output.stdout);
        self.branch_info.local_branches = branches_text
            .lines()
            .map(|line| line.trim_start_matches("* ").trim().to_string())
            .filter(|branch| !branch.is_empty())
            .collect();

        // Get remote branches
        let remote_output = Command::new("git")
            .arg("branch")
            .arg("-r")
            .current_dir(repo_path)
            .output()
            .map_err(GitError::CommandFailed)?;

        let remote_text = String::from_utf8_lossy(&remote_output.stdout);
        self.branch_info.remote_branches = remote_text
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|branch| !branch.is_empty() && !branch.contains("->"))
            .collect();

        Ok(())
    }

    /// Stage a file
    pub fn stage_file(&self, file_path: &Path) -> Result<(), GitError> {
        let repo_path = self.repository_path.as_ref()
            .ok_or(GitError::NoRepository)?;

        let output = Command::new("git")
            .arg("add")
            .arg(file_path)
            .current_dir(repo_path)
            .output()
            .map_err(GitError::CommandFailed)?;

        if !output.status.success() {
            return Err(GitError::StagingFailed(file_path.to_path_buf()));
        }

        Ok(())
    }

    /// Unstage a file
    pub fn unstage_file(&self, file_path: &Path) -> Result<(), GitError> {
        let repo_path = self.repository_path.as_ref()
            .ok_or(GitError::NoRepository)?;

        let output = Command::new("git")
            .arg("reset")
            .arg("HEAD")
            .arg(file_path)
            .current_dir(repo_path)
            .output()
            .map_err(GitError::CommandFailed)?;

        if !output.status.success() {
            return Err(GitError::UnstagingFailed(file_path.to_path_buf()));
        }

        Ok(())
    }

    /// Commit staged changes
    pub fn commit(&self, message: &str) -> Result<String, GitError> {
        let repo_path = self.repository_path.as_ref()
            .ok_or(GitError::NoRepository)?;

        let output = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(message)
            .current_dir(repo_path)
            .output()
            .map_err(GitError::CommandFailed)?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(GitError::CommitFailed(error_msg.to_string()));
        }

        let commit_output = String::from_utf8_lossy(&output.stdout);
        Ok(commit_output.to_string())
    }

    /// Create new branch
    pub fn create_branch(&self, branch_name: &str) -> Result<(), GitError> {
        let repo_path = self.repository_path.as_ref()
            .ok_or(GitError::NoRepository)?;

        let output = Command::new("git")
            .arg("checkout")
            .arg("-b")
            .arg(branch_name)
            .current_dir(repo_path)
            .output()
            .map_err(GitError::CommandFailed)?;

        if !output.status.success() {
            return Err(GitError::BranchCreationFailed(branch_name.to_string()));
        }

        Ok(())
    }

    /// Switch to branch
    pub fn switch_branch(&self, branch_name: &str) -> Result<(), GitError> {
        let repo_path = self.repository_path.as_ref()
            .ok_or(GitError::NoRepository)?;

        let output = Command::new("git")
            .arg("checkout")
            .arg(branch_name)
            .current_dir(repo_path)
            .output()
            .map_err(GitError::CommandFailed)?;

        if !output.status.success() {
            return Err(GitError::BranchSwitchFailed(branch_name.to_string()));
        }

        Ok(())
    }

    /// Get file diff
    pub fn get_file_diff(&mut self, file_path: &Path) -> Result<FileDiff, GitError> {
        let repo_path = self.repository_path.as_ref()
            .ok_or(GitError::NoRepository)?;

        // Check cache first
        if let Some(cached_diff) = self.diff_cache.get(file_path) {
            return Ok(cached_diff.clone());
        }

        let output = Command::new("git")
            .arg("diff")
            .arg("--no-index")
            .arg("/dev/null") // Compare against empty file for new files
            .arg(file_path)
            .current_dir(repo_path)
            .output()
            .map_err(GitError::CommandFailed)?;

        let diff_text = String::from_utf8_lossy(&output.stdout);
        let file_diff = self.parse_diff(&diff_text, file_path)?;
        
        // Cache the result
        self.diff_cache.insert(file_path.to_path_buf(), file_diff.clone());
        
        Ok(file_diff)
    }

    /// Parse diff output into structured format
    fn parse_diff(&self, diff_text: &str, file_path: &Path) -> Result<FileDiff, GitError> {
        let mut hunks = Vec::new();
        let mut current_hunk: Option<DiffHunk> = None;

        for line in diff_text.lines() {
            if line.starts_with("@@") {
                // Save previous hunk if exists
                if let Some(hunk) = current_hunk.take() {
                    hunks.push(hunk);
                }

                // Parse hunk header
                if let Some(ranges) = self.parse_hunk_header(line) {
                    current_hunk = Some(DiffHunk {
                        old_range: ranges.0,
                        new_range: ranges.1,
                        lines: Vec::new(),
                        context: line.to_string(),
                    });
                }
            } else if let Some(ref mut hunk) = current_hunk {
                let diff_line = match line.chars().next() {
                    Some('+') => DiffLine {
                        line_type: DiffLineType::Added,
                        content: line[1..].to_string(),
                        old_line_number: None,
                        new_line_number: Some(hunk.lines.len() + hunk.new_range.0),
                    },
                    Some('-') => DiffLine {
                        line_type: DiffLineType::Removed,
                        content: line[1..].to_string(),
                        old_line_number: Some(hunk.lines.len() + hunk.old_range.0),
                        new_line_number: None,
                    },
                    Some(' ') => DiffLine {
                        line_type: DiffLineType::Context,
                        content: line[1..].to_string(),
                        old_line_number: Some(hunk.lines.len() + hunk.old_range.0),
                        new_line_number: Some(hunk.lines.len() + hunk.new_range.0),
                    },
                    _ => DiffLine {
                        line_type: DiffLineType::Context,
                        content: line.to_string(),
                        old_line_number: None,
                        new_line_number: None,
                    },
                };
                hunk.lines.push(diff_line);
            }
        }

        // Add final hunk
        if let Some(hunk) = current_hunk {
            hunks.push(hunk);
        }

        Ok(FileDiff {
            file_path: file_path.to_path_buf(),
            hunks,
            status: FileStatus::Modified,
            old_path: None,
        })
    }

    /// Parse hunk header like "@@ -1,4 +1,6 @@"
    fn parse_hunk_header(&self, header: &str) -> Option<((usize, usize), (usize, usize))> {
        let parts: Vec<&str> = header.split_whitespace().collect();
        if parts.len() >= 3 {
            let old_part = parts[1].trim_start_matches('-');
            let new_part = parts[2].trim_start_matches('+');
            
            let old_range = self.parse_range(old_part)?;
            let new_range = self.parse_range(new_part)?;
            
            Some((old_range, new_range))
        } else {
            None
        }
    }

    /// Parse range like "1,4" into (start, count)
    fn parse_range(&self, range_str: &str) -> Option<(usize, usize)> {
        if let Some(comma_pos) = range_str.find(',') {
            let start = range_str[..comma_pos].parse().ok()?;
            let count = range_str[comma_pos + 1..].parse().ok()?;
            Some((start, count))
        } else {
            let start = range_str.parse().ok()?;
            Some((start, 1))
        }
    }

    /// Load Git configuration
    fn load_git_config(&mut self) -> Result<(), GitError> {
        let repo_path = self.repository_path.as_ref()
            .ok_or(GitError::NoRepository)?;

        // Get user name
        if let Ok(output) = Command::new("git")
            .arg("config")
            .arg("user.name")
            .current_dir(repo_path)
            .output() 
        {
            if output.status.success() {
                self.config.user_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
            }
        }

        // Get user email
        if let Ok(output) = Command::new("git")
            .arg("config")
            .arg("user.email")
            .current_dir(repo_path)
            .output()
        {
            if output.status.success() {
                self.config.user_email = String::from_utf8_lossy(&output.stdout).trim().to_string();
            }
        }

        Ok(())
    }

    /// Render Git UI panel
    pub fn render_git_panel(&mut self, ui: &mut Ui) {
        ui.heading("Git");

        if self.repository_path.is_none() {
            ui.label("No Git repository detected");
            return;
        }

        // Repository info
        ui.horizontal(|ui| {
            ui.label("Branch:");
            ui.label(&self.branch_info.current_branch);
            if ui.small_button("🔄").clicked() {
                let _ = self.refresh_status();
                let _ = self.refresh_branches();
            }
        });

        ui.separator();

        // File status
        ui.collapsing("Changes", |ui| {
            if !self.status_cache.modified.is_empty() {
                ui.label("Modified:");
                let modified_files: Vec<_> = self.status_cache.modified.clone();
                for file in &modified_files {
                    ui.horizontal(|ui| {
                        ui.label("M");
                        if ui.small_button(file.file_name().unwrap().to_string_lossy()).clicked() {
                            self.ui_state.show_diff_for = Some(file.clone());
                        }
                        if ui.small_button("+").clicked() {
                            let _ = self.stage_file(file);
                            let _ = self.refresh_status();
                        }
                    });
                }
            }

            if !self.status_cache.staged.is_empty() {
                ui.label("Staged:");
                let staged_files: Vec<_> = self.status_cache.staged.clone();
                for file in &staged_files {
                    ui.horizontal(|ui| {
                        ui.label("A");
                        ui.label(file.file_name().unwrap().to_string_lossy());
                        if ui.small_button("-").clicked() {
                            let _ = self.unstage_file(file);
                            let _ = self.refresh_status();
                        }
                    });
                }
            }

            if !self.status_cache.untracked.is_empty() {
                ui.label("Untracked:");
                let untracked_files: Vec<_> = self.status_cache.untracked.clone();
                for file in &untracked_files {
                    ui.horizontal(|ui| {
                        ui.label("?");
                        ui.label(file.file_name().unwrap().to_string_lossy());
                        if ui.small_button("+").clicked() {
                            let _ = self.stage_file(file);
                            let _ = self.refresh_status();
                        }
                    });
                }
            }
        });

        // Commit section
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("📝 Commit").clicked() {
                self.ui_state.show_commit_dialog = true;
            }
            if ui.button("🌿 Branches").clicked() {
                self.ui_state.show_branch_selector = true;
            }
            if ui.button("📜 History").clicked() {
                self.ui_state.show_history_dialog = true;
            }
        });

        // Commit dialog
        if self.ui_state.show_commit_dialog {
            let mut commit_message = self.ui_state.commit_message.clone();
            let mut show_dialog = true;
            let mut should_commit = false;
            let mut should_cancel = false;
            
            egui::Window::new("Commit Changes")
                .open(&mut show_dialog)
                .show(ui.ctx(), |ui| {
                    ui.label("Commit message:");
                    ui.text_edit_multiline(&mut commit_message);
                    
                    ui.horizontal(|ui| {
                        if ui.button("Commit").clicked() && !commit_message.trim().is_empty() {
                            should_commit = true;
                        }
                        if ui.button("Cancel").clicked() {
                            should_cancel = true;
                        }
                    });
                });
                
            // Handle actions outside the closure
            if should_commit {
                match self.commit(&commit_message) {
                    Ok(_) => {
                        commit_message.clear();
                        show_dialog = false;
                        let _ = self.refresh_status();
                    }
                    Err(_) => {
                        // Show error message
                    }
                }
            }
            
            if should_cancel || !show_dialog {
                self.ui_state.show_commit_dialog = false;
            } else {
                self.ui_state.commit_message = commit_message;
            }
        }

        // Branch selector
        if self.ui_state.show_branch_selector {
            let local_branches = self.branch_info.local_branches.clone();
            let current_branch = self.branch_info.current_branch.clone();
            let mut show_selector = self.ui_state.show_branch_selector;
            
            egui::Window::new("Branches")
                .open(&mut show_selector)
                .show(ui.ctx(), |ui| {
                    ui.label("Local branches:");
                    for branch in &local_branches {
                        ui.horizontal(|ui| {
                            let is_current = branch == &current_branch;
                            let text = if is_current {
                                format!("* {}", branch)
                            } else {
                                branch.clone()
                            };
                            
                            if ui.selectable_label(is_current, text).clicked() && !is_current {
                                let _ = self.switch_branch(branch);
                                let _ = self.refresh_branches();
                                let _ = self.refresh_status();
                            }
                        });
                    }
                });
                
            self.ui_state.show_branch_selector = show_selector;
        }
    }

    /// Get file status for display in file explorer
    pub fn get_file_status(&self, file_path: &Path) -> FileStatus {
        if self.status_cache.modified.contains(&file_path.to_path_buf()) {
            FileStatus::Modified
        } else if self.status_cache.staged.contains(&file_path.to_path_buf()) {
            FileStatus::Staged
        } else if self.status_cache.untracked.contains(&file_path.to_path_buf()) {
            FileStatus::Untracked
        } else if self.status_cache.deleted.contains(&file_path.to_path_buf()) {
            FileStatus::Deleted
        } else {
            FileStatus::Unmodified
        }
    }
}

/// Git integration errors
#[derive(Debug, thiserror::Error)]
pub enum GitError {
    #[error("Not a Git repository: {0:?}")]
    NotARepository(PathBuf),
    #[error("No repository initialized")]
    NoRepository,
    #[error("Git command failed: {0}")]
    CommandFailed(std::io::Error),
    #[error("Failed to stage file: {0:?}")]
    StagingFailed(PathBuf),
    #[error("Failed to unstage file: {0:?}")]
    UnstagingFailed(PathBuf),
    #[error("Commit failed: {0}")]
    CommitFailed(String),
    #[error("Failed to create branch: {0}")]
    BranchCreationFailed(String),
    #[error("Failed to switch branch: {0}")]
    BranchSwitchFailed(String),
}

impl Default for GitIntegration {
    fn default() -> Self {
        Self::new()
    }
}