//! File System Browser and Navigation
//!
//! Provides file system navigation, filtering, and display functionality
//! for the project manager.

use std::path::{Path, PathBuf};
use std::fs;
use egui::*;
use crate::editor::output_panel::OutputPanel;
use super::project::IdeProject;

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
#[derive(Debug, Clone, PartialEq)]
pub enum FileViewMode {
    List,
    Grid,
    Tree,
}

/// File sorting options
#[derive(Debug, Clone, PartialEq)]
pub enum FileSortOrder {
    Name,
    Modified,
    Size,
    Type,
}

/// File entry information
pub struct FileEntry {
    /// File path
    pub path: PathBuf,
    /// File name
    pub name: String,
    /// Whether it's a directory
    pub is_dir: bool,
    /// File size (for files)
    pub size: Option<u64>,
    /// Last modified time
    pub modified: Option<std::time::SystemTime>,
}

impl FileBrowser {
    /// Create a new file browser
    pub fn new(default_path: &Path) -> Self {
        Self {
            current_dir: default_path.to_path_buf(),
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
        }
    }

    /// Set current directory
    pub fn set_current_dir(&mut self, path: &Path) {
        if path.exists() && path.is_dir() {
            self.current_dir = path.to_path_buf();
        }
    }

    /// Navigate to parent directory
    pub fn navigate_up(&mut self) {
        if let Some(parent) = self.current_dir.parent() {
            self.history.push(self.current_dir.clone());
            self.current_dir = parent.to_path_buf();
        }
    }

    /// Navigate to subdirectory
    pub fn navigate_to(&mut self, path: &Path) {
        if path.exists() && path.is_dir() {
            self.history.push(self.current_dir.clone());
            self.current_dir = path.to_path_buf();
        }
    }

    /// Navigate back in history
    pub fn navigate_back(&mut self) {
        if let Some(previous) = self.history.pop() {
            self.current_dir = previous;
        }
    }

    /// Get directory entries
    pub fn get_entries(&self) -> Result<Vec<FileEntry>, std::io::Error> {
        let mut entries = Vec::new();

        for entry in fs::read_dir(&self.current_dir)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            // Apply filters
            if !self.filters.show_hidden && file_name.starts_with('.') {
                continue;
            }

            if path.is_dir() && self.filters.exclude_dirs.iter().any(|exclude| file_name == *exclude) {
                continue;
            }

            let metadata = entry.metadata().ok();
            let size = metadata.as_ref()
                .filter(|m| m.is_file())
                .map(|m| m.len());
            let modified = metadata.as_ref()
                .and_then(|m| m.modified().ok());

            entries.push(FileEntry {
                path: path.clone(),
                name: file_name,
                is_dir: path.is_dir(),
                size,
                modified,
            });
        }

        // Sort entries
        self.sort_entries(&mut entries);

        Ok(entries)
    }

    /// Sort file entries based on current settings
    fn sort_entries(&self, entries: &mut Vec<FileEntry>) {
        entries.sort_by(|a, b| {
            // Directories first
            match (a.is_dir, b.is_dir) {
                (true, false) => return std::cmp::Ordering::Less,
                (false, true) => return std::cmp::Ordering::Greater,
                _ => {}
            }

            let ordering = match self.view_settings.sort_by {
                FileSortOrder::Name => a.name.cmp(&b.name),
                FileSortOrder::Modified => {
                    match (a.modified, b.modified) {
                        (Some(a_time), Some(b_time)) => a_time.cmp(&b_time),
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => a.name.cmp(&b.name),
                    }
                }
                FileSortOrder::Size => {
                    match (a.size, b.size) {
                        (Some(a_size), Some(b_size)) => a_size.cmp(&b_size),
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => a.name.cmp(&b.name),
                    }
                }
                FileSortOrder::Type => {
                    let a_ext = a.path.extension().unwrap_or_default();
                    let b_ext = b.path.extension().unwrap_or_default();
                    match a_ext.cmp(&b_ext) {
                        std::cmp::Ordering::Equal => a.name.cmp(&b.name),
                        other => other,
                    }
                }
            };

            if self.view_settings.sort_ascending {
                ordering
            } else {
                ordering.reverse()
            }
        });
    }

    /// Render the file browser UI
    pub fn render(&mut self, ui: &mut Ui, output_panel: &mut OutputPanel, current_project: &Option<IdeProject>) {
        ui.heading("📁 Project Explorer");
        ui.separator();

        // Toolbar
        self.render_toolbar(ui, current_project);
        ui.separator();

        // File listing
        self.render_file_listing(ui, output_panel);
    }

    /// Render file browser toolbar
    fn render_toolbar(&mut self, ui: &mut Ui, current_project: &Option<IdeProject>) {
        ui.horizontal(|ui| {
            // Navigation buttons
            if ui.button("🏠").on_hover_text("Go to project root").clicked() {
                if let Some(project) = current_project {
                    self.set_current_dir(&project.metadata.root_path);
                }
            }
            
            if ui.button("⬆").on_hover_text("Parent directory").clicked() {
                self.navigate_up();
            }
            
            if ui.button("⬅").on_hover_text("Back").clicked() && !self.history.is_empty() {
                self.navigate_back();
            }
            
            ui.separator();
            
            // View mode selector
            ComboBox::from_label("View")
                .selected_text(match self.view_settings.view_mode {
                    FileViewMode::List => "List",
                    FileViewMode::Grid => "Grid", 
                    FileViewMode::Tree => "Tree",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.view_settings.view_mode, FileViewMode::List, "List");
                    ui.selectable_value(&mut self.view_settings.view_mode, FileViewMode::Grid, "Grid");
                    ui.selectable_value(&mut self.view_settings.view_mode, FileViewMode::Tree, "Tree");
                });
            
            // Sort selector
            ComboBox::from_label("Sort")
                .selected_text(match self.view_settings.sort_by {
                    FileSortOrder::Name => "Name",
                    FileSortOrder::Modified => "Modified",
                    FileSortOrder::Size => "Size",
                    FileSortOrder::Type => "Type",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.view_settings.sort_by, FileSortOrder::Name, "Name");
                    ui.selectable_value(&mut self.view_settings.sort_by, FileSortOrder::Modified, "Modified");
                    ui.selectable_value(&mut self.view_settings.sort_by, FileSortOrder::Size, "Size");
                    ui.selectable_value(&mut self.view_settings.sort_by, FileSortOrder::Type, "Type");
                });
            
            // Sort order toggle
            let sort_icon = if self.view_settings.sort_ascending { "⬆" } else { "⬇" };
            if ui.button(sort_icon).on_hover_text("Toggle sort order").clicked() {
                self.view_settings.sort_ascending = !self.view_settings.sort_ascending;
            }
        });

        // Current path display
        ui.horizontal(|ui| {
            ui.label("📂");
            ui.label(self.current_dir.display().to_string());
        });

        // Filters
        ui.collapsing("Filters", |ui| {
            ui.checkbox(&mut self.filters.show_hidden, "Show hidden files");
            
            ui.horizontal(|ui| {
                ui.label("Extensions:");
                let extensions_text = self.filters.extensions.join(", ");
                ui.label(extensions_text);
            });
            
            ui.horizontal(|ui| {
                ui.label("Exclude dirs:");
                let exclude_text = self.filters.exclude_dirs.join(", ");
                ui.label(exclude_text);
            });
        });
    }

    /// Render file listing
    fn render_file_listing(&mut self, ui: &mut Ui, output_panel: &mut OutputPanel) {
        ScrollArea::vertical().show(ui, |ui| {
            match self.get_entries() {
                Ok(entries) => {
                    for entry in entries {
                        self.render_file_entry(ui, &entry, output_panel);
                    }
                }
                Err(err) => {
                    ui.colored_label(Color32::RED, format!("Error reading directory: {}", err));
                }
            }
        });
    }

    /// Render individual file entry
    fn render_file_entry(&mut self, ui: &mut Ui, entry: &FileEntry, output_panel: &mut OutputPanel) {
        let icon = if entry.is_dir { "📁" } else { self.get_file_icon(&entry.path) };
        let display_name = format!("{} {}", icon, entry.name);
        
        ui.horizontal(|ui| {
            let response = ui.selectable_label(
                self.selected_files.contains(&entry.path), 
                &display_name
            );
            
            if response.clicked() {
                if entry.is_dir {
                    self.navigate_to(&entry.path);
                } else {
                    // Toggle selection
                    if let Some(pos) = self.selected_files.iter().position(|p| p == &entry.path) {
                        self.selected_files.remove(pos);
                    } else {
                        self.selected_files.push(entry.path.clone());
                    }
                    output_panel.log(&format!("📂 Selected file: {}", entry.path.display()));
                }
            }
            
            if response.double_clicked() && !entry.is_dir {
                output_panel.log(&format!("📂 Opening file: {}", entry.path.display()));
                // TODO: Integrate with code editor
            }
            
            // Show file info
            if let Some(size) = entry.size {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(format_file_size(size));
                });
            }
        });
    }

    /// Get appropriate icon for file type
    fn get_file_icon(&self, path: &Path) -> &'static str {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("rs") => "🦀",
            Some("toml") => "⚙️",
            Some("md") => "📝",
            Some("txt") => "📄",
            Some("json") => "📋",
            Some("png") | Some("jpg") | Some("jpeg") | Some("gif") => "🖼️",
            Some("exe") | Some("bin") => "⚙️",
            _ => "📄",
        }
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selected_files.clear();
    }

    /// Get selected files
    pub fn get_selected_files(&self) -> &[PathBuf] {
        &self.selected_files
    }
}

/// Format file size for display
fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}