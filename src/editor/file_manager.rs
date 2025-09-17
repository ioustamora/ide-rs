//! File Manager with Multi-Tab Support
//! 
//! Provides VS Code-inspired file management with tabs, file type recognition,
//! and automatic mode switching between code editor and visual designer.

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use crate::editor::code_editor::CodeEditor;
use crate::editor::visual_designer::VisualDesigner;
use crate::editor::file_watcher::{AdvancedFileWatcher, FileWatchEvent, FileWatcherConfig};
use crate::core::event_bus::{IdeEvent, global_event_bus};

/// File type classification for editor mode selection
#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    /// Code files (.rs, .js, .ts, .py, etc.)
    Code(String), // language
    /// UI/Form files (.ui, .form, .designer, etc.)  
    UIDesign,
    /// Unknown file type
    Unknown,
}

/// Open file tab containing file data and editor state
pub struct FileTab {
    /// File path
    pub path: PathBuf,
    /// File display name (filename)
    pub name: String,
    /// File type classification
    pub file_type: FileType,
    /// File content
    pub content: String,
    /// Whether file has unsaved changes
    pub is_dirty: bool,
    /// Last modified timestamp
    pub last_modified: Option<std::time::SystemTime>,
    /// Code editor state (for code files)
    pub code_editor: Option<CodeEditor>,
    /// Visual designer state (for UI files)  
    pub visual_designer: Option<VisualDesigner>,
}

/// Multi-file tab management system
pub struct FileManager {
    /// Currently open file tabs
    pub open_tabs: HashMap<PathBuf, FileTab>,
    /// Currently active tab path
    pub active_tab: Option<PathBuf>,
    /// Tab order for UI display
    pub tab_order: Vec<PathBuf>,
    /// Recently opened files
    pub recent_files: Vec<PathBuf>,
    /// Maximum number of recent files to track
    pub max_recent_files: usize,
    /// Auto-save functionality
    pub auto_save_enabled: bool,
    /// Auto-save interval in seconds
    pub auto_save_interval: u64,
    /// Last auto-save time
    pub last_auto_save: std::time::Instant,
    /// File type associations
    pub file_associations: HashMap<String, FileType>,
    /// File search index for fast searching
    pub search_index: FileSearchIndex,
    /// Advanced file watcher for detecting external changes
    pub file_watcher: Option<AdvancedFileWatcher>,
}

impl FileTab {
    /// Create a new file tab
    pub fn new(path: PathBuf, content: String) -> Self {
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string();
        
        let file_type = FileManager::classify_file_type(&path);
        
        let mut tab = Self {
            path: path.clone(),
            name,
            file_type: file_type.clone(),
            content: content.clone(),
            is_dirty: false,
            last_modified: None,
            code_editor: None,
            visual_designer: None,
        };
        
        // Initialize appropriate editor based on file type
        match file_type {
            FileType::Code(language) => {
                tab.code_editor = Some(CodeEditor::with_content(&language, content));
            }
            FileType::UIDesign => {
                tab.visual_designer = Some(VisualDesigner::new());
                // TODO: Load UI design from file content
            }
            FileType::Unknown => {
                // Default to text editor
                tab.code_editor = Some(CodeEditor::with_content("text", content));
            }
        }
        
        tab
    }
    
    /// Get current content from active editor
    pub fn get_current_content(&self) -> String {
        match &self.file_type {
            FileType::Code(_) => {
                self.code_editor.as_ref()
                    .map(|editor| editor.code.clone())
                    .unwrap_or_else(|| self.content.clone())
            }
            FileType::UIDesign => {
                // TODO: Serialize visual designer state to string
                self.content.clone()
            }
            FileType::Unknown => {
                self.code_editor.as_ref()
                    .map(|editor| editor.code.clone())
                    .unwrap_or_else(|| self.content.clone())
            }
        }
    }
    
    /// Mark tab as dirty (has unsaved changes)
    pub fn mark_dirty(&mut self) {
        self.is_dirty = true;
    }
    
    /// Mark tab as clean (saved)
    pub fn mark_clean(&mut self) {
        self.is_dirty = false;
    }
}

impl FileManager {
    /// Create a new file manager
    pub fn new() -> Self {
        let mut manager = Self {
            open_tabs: HashMap::new(),
            active_tab: None,
            tab_order: Vec::new(),
            recent_files: Vec::new(),
            max_recent_files: 10,
            auto_save_enabled: true,
            auto_save_interval: 30, // 30 seconds
            last_auto_save: std::time::Instant::now(),
            file_associations: HashMap::new(),
            search_index: FileSearchIndex::new(),
            file_watcher: None,
        };
        
        manager.initialize_file_associations();
        
        // Initialize file watcher
        if let Err(e) = manager.initialize_file_watcher() {
            eprintln!("Failed to initialize file watcher: {}", e);
        }
        
        manager
    }
    
    /// Initialize default file type associations
    fn initialize_file_associations(&mut self) {
        let associations = [
            // Rust
            ("rs", FileType::Code("rust".to_string())),
            // JavaScript/TypeScript
            ("js", FileType::Code("javascript".to_string())),
            ("jsx", FileType::Code("javascript".to_string())),
            ("ts", FileType::Code("typescript".to_string())),
            ("tsx", FileType::Code("typescript".to_string())),
            // Python
            ("py", FileType::Code("python".to_string())),
            // UI/Design files
            ("ui", FileType::UIDesign),
            ("form", FileType::UIDesign),
            ("designer", FileType::UIDesign),
            ("design", FileType::UIDesign),
            // Web
            ("html", FileType::Code("html".to_string())),
            ("css", FileType::Code("css".to_string())),
            ("scss", FileType::Code("scss".to_string())),
            ("vue", FileType::Code("vue".to_string())),
            // Config
            ("json", FileType::Code("json".to_string())),
            ("toml", FileType::Code("toml".to_string())),
            ("yaml", FileType::Code("yaml".to_string())),
            ("yml", FileType::Code("yaml".to_string())),
            // Other
            ("md", FileType::Code("markdown".to_string())),
            ("txt", FileType::Code("text".to_string())),
        ];
        
        for (ext, file_type) in associations.iter() {
            self.file_associations.insert(ext.to_string(), file_type.clone());
        }
    }
    
    /// Initialize file watcher with default configuration
    fn initialize_file_watcher(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let config = FileWatcherConfig::default();
        self.initialize_file_watcher_with_config(config)
    }
    
    /// Initialize file watcher with custom configuration
    fn initialize_file_watcher_with_config(&mut self, config: FileWatcherConfig) -> Result<(), Box<dyn std::error::Error>> {
        if !config.enabled {
            return Ok(());
        }
        
        // Create the advanced file watcher (it manages its own background threads)
        let watcher = AdvancedFileWatcher::with_config(config)?;
        
        // Store the watcher - no need for additional threading as it's self-contained
        self.file_watcher = Some(watcher);
        
        Ok(())
    }
    
    /// Handle file watch events
    fn handle_file_watch_event(event: FileWatchEvent) {
        // Send events to the global event bus for other components to handle
        match event {
            FileWatchEvent::Created(path) => {
                global_event_bus().publish(IdeEvent::FileCreated { path });
            }
            FileWatchEvent::Modified(path) => {
                global_event_bus().publish(IdeEvent::FileModified { path, external: true });
            }
            FileWatchEvent::Deleted(path) => {
                global_event_bus().publish(IdeEvent::FileDeleted { path });
            }
            FileWatchEvent::Renamed(old_path, new_path) => {
                global_event_bus().publish(IdeEvent::FileRenamed { old_path, new_path });
            }
            FileWatchEvent::DirectoryCreated(path) => {
                global_event_bus().publish(IdeEvent::DirectoryCreated { path });
            }
            FileWatchEvent::DirectoryDeleted(path) => {
                global_event_bus().publish(IdeEvent::DirectoryDeleted { path });
            }
            FileWatchEvent::Batch(events) => {
                // Process each event in the batch
                for event in events {
                    Self::handle_file_watch_event(event);
                }
            }
        }
    }
    
    /// Watch a file or directory for changes
    pub fn watch_path(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref mut watcher) = self.file_watcher {
            watcher.watch(path)?;
            
            #[cfg(feature = "logging")]
            tracing::info!("Started watching path: {}", path.display());
        }
        Ok(())
    }
    
    /// Stop watching a path
    pub fn unwatch_path(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref mut watcher) = self.file_watcher {
            watcher.unwatch(path)?;
            
            #[cfg(feature = "logging")]
            tracing::info!("Stopped watching path: {}", path.display());
        }
        Ok(())
    }
    
    /// Get file watcher statistics
    pub fn watcher_stats(&self) -> Option<crate::editor::file_watcher::WatcherStats> {
        self.file_watcher.as_ref().map(|w| w.stats())
    }
    
    /// Process pending file watch events
    pub fn process_file_watch_events(&mut self) {
        if let Some(watcher) = self.file_watcher.take() {
            // Process all pending events non-blocking
            let mut events = Vec::new();
            while let Ok(event) = watcher.try_recv_event() {
                events.push(event);
            }
            
            // Put watcher back
            self.file_watcher = Some(watcher);
            
            // Process events
            for event in events {
                self.handle_file_watch_event_locally(event.clone());
                Self::handle_file_watch_event(event);
            }
        }
    }
    
    /// Handle file watch events locally (update FileManager state)
    fn handle_file_watch_event_locally(&mut self, event: FileWatchEvent) {
        match event {
            FileWatchEvent::Modified(ref path) => {
                // Mark file as externally modified if it's open
                if let Some(_tab) = self.open_tabs.get_mut(path) {
                    // We might want to check if the file content has actually changed
                    // and potentially offer to reload the file
                    #[cfg(feature = "logging")]
                    tracing::info!("File {} was modified externally", path.display());
                }
            }
            FileWatchEvent::Deleted(ref path) => {
                // Handle file deletion - might want to close the tab or mark as deleted
                if self.open_tabs.contains_key(path) {
                    #[cfg(feature = "logging")]
                    tracing::warn!("Open file {} was deleted externally", path.display());
                }
            }
            FileWatchEvent::Renamed(ref old_path, ref new_path) => {
                // Handle file rename - update the tab with new path
                if let Some(mut tab) = self.open_tabs.remove(old_path) {
                    tab.path = new_path.clone();
                    tab.name = new_path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Renamed File")
                        .to_string();
                    
                    self.open_tabs.insert(new_path.clone(), tab);
                    
                    // Update tab order
                    if let Some(pos) = self.tab_order.iter().position(|p| p == old_path) {
                        self.tab_order[pos] = new_path.clone();
                    }
                    
                    // Update active tab if needed
                    if self.active_tab.as_ref() == Some(old_path) {
                        self.active_tab = Some(new_path.clone());
                    }
                }
            }
            _ => {
                // Handle other events as needed
            }
        }
    }
    
    /// Shutdown file watcher
    fn shutdown_file_watcher(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Shutdown the watcher (it handles its own cleanup)
        if let Some(mut watcher) = self.file_watcher.take() {
            watcher.shutdown()?;
        }
        
        Ok(())
    }
    
    /// Classify file type based on extension
    pub fn classify_file_type(path: &Path) -> FileType {
        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
            match extension.to_lowercase().as_str() {
                "rs" => FileType::Code("rust".to_string()),
                "js" | "jsx" => FileType::Code("javascript".to_string()),
                "ts" | "tsx" => FileType::Code("typescript".to_string()),
                "py" => FileType::Code("python".to_string()),
                "ui" | "form" | "designer" | "design" => FileType::UIDesign,
                "html" => FileType::Code("html".to_string()),
                "css" | "scss" => FileType::Code("css".to_string()),
                "vue" => FileType::Code("vue".to_string()),
                "json" => FileType::Code("json".to_string()),
                "toml" => FileType::Code("toml".to_string()),
                "yaml" | "yml" => FileType::Code("yaml".to_string()),
                "md" => FileType::Code("markdown".to_string()),
                _ => FileType::Unknown,
            }
        } else {
            FileType::Unknown
        }
    }
    
    /// Open a file in a new tab
    pub fn open_file(&mut self, path: PathBuf, content: String) -> Result<(), FileManagerError> {
        // Check if file is already open
        if self.open_tabs.contains_key(&path) {
            self.active_tab = Some(path);
            return Ok(());
        }
        
        // Create new tab
        let tab = FileTab::new(path.clone(), content);
        
        // Add to open tabs
        self.open_tabs.insert(path.clone(), tab);
        self.tab_order.push(path.clone());
        self.active_tab = Some(path.clone());
        
        // Add to recent files
        self.add_to_recent(&path);
        
        // Start watching the file for external changes
        if let Err(e) = self.watch_path(&path) {
            #[cfg(feature = "logging")]
            tracing::warn!("Failed to watch file {}: {}", path.display(), e);
        }
        
        // Emit file opened event
        global_event_bus().publish(IdeEvent::FileOpened { 
            path: path.clone(), 
            buffer_id: None // We could add buffer IDs later if needed
        });
        
        Ok(())
    }
    
    /// Close a file tab
    pub fn close_tab(&mut self, path: &PathBuf) -> Result<(), FileManagerError> {
        // Check if file has unsaved changes
        if let Some(tab) = self.open_tabs.get(path) {
            if tab.is_dirty {
                return Err(FileManagerError::UnsavedChanges(path.clone()));
            }
        }
        
        // Stop watching the file
        if let Err(e) = self.unwatch_path(path) {
            #[cfg(feature = "logging")]
            tracing::warn!("Failed to stop watching file {}: {}", path.display(), e);
        }
        
        // Remove from open tabs
        self.open_tabs.remove(path);
        self.tab_order.retain(|p| p != path);
        
        // Update active tab
        if self.active_tab.as_ref() == Some(path) {
            self.active_tab = self.tab_order.last().cloned();
        }
        
        // Emit file closed event
        global_event_bus().publish(IdeEvent::FileClosed { 
            path: path.clone(), 
            buffer_id: None 
        });
        
        Ok(())
    }
    
    /// Force close a tab without checking for unsaved changes
    pub fn force_close_tab(&mut self, path: &PathBuf) {
        // Remove from open tabs
        self.open_tabs.remove(path);
        self.tab_order.retain(|p| p != path);
        
        // Update active tab
        if self.active_tab.as_ref() == Some(path) {
            self.active_tab = self.tab_order.last().cloned();
        }
    }
    
    /// Switch to a different tab
    pub fn switch_to_tab(&mut self, path: &PathBuf) -> Result<(), FileManagerError> {
        if self.open_tabs.contains_key(path) {
            self.active_tab = Some(path.clone());
            Ok(())
        } else {
            Err(FileManagerError::TabNotFound(path.clone()))
        }
    }
    
    /// Get the currently active tab
    pub fn get_active_tab(&self) -> Option<&FileTab> {
        self.active_tab.as_ref()
            .and_then(|path| self.open_tabs.get(path))
    }
    
    /// Get mutable reference to active tab
    pub fn get_active_tab_mut(&mut self) -> Option<&mut FileTab> {
        if let Some(path) = self.active_tab.clone() {
            self.open_tabs.get_mut(&path)
        } else {
            None
        }
    }
    
    /// Get all open tabs in order
    pub fn get_tabs_in_order(&self) -> Vec<&FileTab> {
        self.tab_order.iter()
            .filter_map(|path| self.open_tabs.get(path))
            .collect()
    }
    
    /// Save the currently active tab
    pub fn save_active_tab(&mut self) -> Result<String, FileManagerError> {
        if let Some(tab) = self.get_active_tab_mut() {
            let content = tab.get_current_content();
            tab.mark_clean();
            Ok(content)
        } else {
            Err(FileManagerError::NoActiveTab)
        }
    }
    
    /// Add file to recent files list
    fn add_to_recent(&mut self, path: &PathBuf) {
        // Remove if already in list
        self.recent_files.retain(|p| p != path);
        
        // Add to front
        self.recent_files.insert(0, path.clone());
        
        // Keep only last 10 files
        if self.recent_files.len() > 10 {
            self.recent_files.truncate(10);
        }
    }
    
    /// Check if any tabs have unsaved changes
    pub fn has_unsaved_changes(&self) -> bool {
        self.open_tabs.values().any(|tab| tab.is_dirty)
    }
    
    /// Get list of unsaved files
    pub fn get_unsaved_files(&self) -> Vec<&PathBuf> {
        self.open_tabs.iter()
            .filter(|(_, tab)| tab.is_dirty)
            .map(|(path, _)| path)
            .collect()
    }
    
    /// Render tab bar UI
    pub fn render_tab_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Clone tab_order to avoid borrowing issues
            let tab_order = self.tab_order.clone();
            let mut tabs_to_close = Vec::new();
            
            for path in tab_order {
                if let Some(tab) = self.open_tabs.get(&path) {
                    let is_active = self.active_tab.as_ref() == Some(&path);
                    
                    // File icon based on type
                    let icon = match tab.file_type {
                        FileType::Code(_) => "📄",
                        FileType::UIDesign => "🎨",
                        FileType::Unknown => "❓",
                    };
                    
                    // Tab name with dirty indicator
                    let display_name = if tab.is_dirty {
                        format!("● {}", tab.name)
                    } else {
                        tab.name.clone()
                    };
                    
                    // Tab button with close button
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            // Tab button
                            let tab_button = ui.selectable_label(is_active, format!("{} {}", icon, display_name));
                            if tab_button.clicked() {
                                self.active_tab = Some(path.clone());
                            }
                            
                            // Close button
                            if ui.small_button("×").clicked() {
                                tabs_to_close.push(path.clone());
                            }
                        });
                    });
                }
            }
            
            // Close tabs after rendering to avoid borrowing issues
            for path in tabs_to_close {
                match self.close_tab(&path) {
                    Ok(()) => {
                        // Tab closed successfully
                    }
                    Err(FileManagerError::UnsavedChanges(_)) => {
                        // Force close for now - in a real implementation, would show a dialog
                        // asking "Do you want to save changes before closing?"
                        self.force_close_tab(&path);
                    }
                    Err(_) => {
                        // Other errors - could log or show notification
                    }
                }
            }
            
            // New file button
            if ui.button("+ New").clicked() {
                let _ = self.create_new_file();
            }
            
            // Save all button
            if ui.button("💾 Save All").clicked() {
                self.save_all_tabs();
            }
        });
    }
    
    /// Create a new untitled file
    pub fn create_new_file(&mut self) -> Result<PathBuf, FileManagerError> {
        let mut file_counter = 1;
        let mut new_path;
        
        // Find available untitled name
        loop {
            new_path = PathBuf::from(format!("Untitled{}.rs", file_counter));
            if !self.open_tabs.contains_key(&new_path) {
                break;
            }
            file_counter += 1;
        }
        
        let default_content = "fn main() {\n    println!(\"Hello, world!\");\n}\n".to_string();
        self.open_file(new_path.clone(), default_content)?;
        Ok(new_path)
    }
    
    /// Save all open tabs
    pub fn save_all_tabs(&mut self) -> Vec<Result<(), FileManagerError>> {
        let mut results = Vec::new();
        let paths: Vec<_> = self.open_tabs.keys().cloned().collect();
        
        for path in paths {
            results.push(self.save_tab(&path));
        }
        
        results
    }
    
    /// Save a specific tab
    pub fn save_tab(&mut self, path: &PathBuf) -> Result<(), FileManagerError> {
        if let Some(tab) = self.open_tabs.get_mut(path) {
            // Only save if file is dirty and has a real path (not untitled)
            if tab.is_dirty && path.parent().is_some() {
                std::fs::write(path, &tab.content)?;
                tab.mark_clean();
                tab.last_modified = std::fs::metadata(path)?.modified().ok();
            }
            Ok(())
        } else {
            Err(FileManagerError::TabNotFound(path.clone()))
        }
    }
    
    /// Add file to recent files list
    pub fn add_to_recent_files(&mut self, path: PathBuf) {
        // Remove if already exists
        self.recent_files.retain(|p| p != &path);
        
        // Add to front
        self.recent_files.insert(0, path);
        
        // Limit to max recent files
        if self.recent_files.len() > self.max_recent_files {
            self.recent_files.truncate(self.max_recent_files);
        }
    }
    
    /// Get recent files list
    pub fn get_recent_files(&self) -> &[PathBuf] {
        &self.recent_files
    }
    
    /// Auto-save functionality
    pub fn auto_save_check(&mut self) -> Vec<Result<(), FileManagerError>> {
        let mut results = Vec::new();
        
        if !self.auto_save_enabled {
            return results;
        }
        
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.last_auto_save).as_secs();
        
        if elapsed >= self.auto_save_interval {
            // Auto-save all dirty tabs
            let dirty_paths: Vec<_> = self.open_tabs.iter()
                .filter(|(_, tab)| tab.is_dirty)
                .map(|(path, _)| path.clone())
                .collect();
            
            for path in dirty_paths {
                results.push(self.save_tab(&path));
            }
            
            self.last_auto_save = now;
        }
        
        results
    }
    
    
    /// Enable/disable auto-save
    pub fn set_auto_save(&mut self, enabled: bool, interval_seconds: u64) {
        self.auto_save_enabled = enabled;
        self.auto_save_interval = interval_seconds;
    }
}

impl Default for FileManager {
    fn default() -> Self {
        Self::new()
    }
}

/// File manager error types
#[derive(Debug, thiserror::Error)]
pub enum FileManagerError {
    #[error("Tab not found: {0:?}")]
    TabNotFound(PathBuf),
    #[error("File has unsaved changes: {0:?}")]
    UnsavedChanges(PathBuf),
    #[error("No active tab")]
    NoActiveTab,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// File search index for fast full-text and filename searching
pub struct FileSearchIndex {
    /// Filename index
    pub filename_index: HashMap<String, Vec<PathBuf>>,
    /// Content index (word -> files containing word)
    pub content_index: HashMap<String, Vec<PathBuf>>,
    /// File modification times for cache invalidation
    pub modification_times: HashMap<PathBuf, std::time::SystemTime>,
    /// Index update status
    pub is_indexing: bool,
}

impl Default for FileSearchIndex {
    fn default() -> Self {
        Self {
            filename_index: HashMap::new(),
            content_index: HashMap::new(),
            modification_times: HashMap::new(),
            is_indexing: false,
        }
    }
}

impl FileSearchIndex {
    /// Create a new search index
    pub fn new() -> Self {
        Self::default()
    }

    /// Index a file (both filename and content)
    pub fn index_file(&mut self, file_path: PathBuf) -> Result<(), std::io::Error> {
        // Index filename
        if let Some(filename) = file_path.file_name() {
            if let Some(filename_str) = filename.to_str() {
                let words = Self::tokenize_filename(filename_str);
                for word in words {
                    self.filename_index
                        .entry(word.to_lowercase())
                        .or_default()
                        .push(file_path.clone());
                }
            }
        }

        // Index content (for text files)
        if Self::is_text_file(&file_path) {
            if let Ok(content) = std::fs::read_to_string(&file_path) {
                let words = Self::tokenize_content(&content);
                for word in words {
                    if word.len() > 2 { // Skip very short words
                        self.content_index
                            .entry(word.to_lowercase())
                            .or_default()
                            .push(file_path.clone());
                    }
                }
            }
        }

        // Update modification time
        if let Ok(metadata) = std::fs::metadata(&file_path) {
            if let Ok(modified) = metadata.modified() {
                self.modification_times.insert(file_path, modified);
            }
        }

        Ok(())
    }

    /// Search files by filename
    pub fn search_filenames(&self, query: &str) -> Vec<PathBuf> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();

        // Exact matches first
        if let Some(files) = self.filename_index.get(&query_lower) {
            results.extend_from_slice(files);
        }

        // Partial matches
        for (word, files) in &self.filename_index {
            if word.contains(&query_lower) && word != &query_lower {
                results.extend_from_slice(files);
            }
        }

        // Remove duplicates and return
        results.sort();
        results.dedup();
        results
    }

    /// Search files by content
    pub fn search_content(&self, query: &str) -> Vec<PathBuf> {
        let query_words: Vec<String> = query.to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        if query_words.is_empty() {
            return Vec::new();
        }

        // Find files containing all query words
        let mut result_files: Option<Vec<PathBuf>> = None;

        for word in query_words {
            if let Some(files) = self.content_index.get(&word) {
                match result_files {
                    None => result_files = Some(files.clone()),
                    Some(ref mut existing) => {
                        existing.retain(|f| files.contains(f));
                    }
                }
            } else {
                // If any word is not found, no results
                return Vec::new();
            }
        }

        result_files.unwrap_or_default()
    }

    /// Combined search (filename + content)
    pub fn search_combined(&self, query: &str) -> Vec<(PathBuf, SearchResultType)> {
        let mut results = Vec::new();

        // Search filenames
        let filename_results = self.search_filenames(query);
        for file in filename_results {
            results.push((file, SearchResultType::Filename));
        }

        // Search content
        let content_results = self.search_content(query);
        for file in content_results {
            // Avoid duplicates from filename search
            if !results.iter().any(|(f, _)| f == &file) {
                results.push((file, SearchResultType::Content));
            }
        }

        results
    }

    /// Check if a file needs re-indexing
    pub fn needs_reindex(&self, file_path: &Path) -> bool {
        if let Ok(metadata) = std::fs::metadata(file_path) {
            if let Ok(modified) = metadata.modified() {
                return self.modification_times.get(file_path) != Some(&modified);
            }
        }
        true // Re-index if we can't get modification time
    }

    /// Remove a file from the index
    pub fn remove_file(&mut self, file_path: &Path) {
        // Remove from filename index
        for files in self.filename_index.values_mut() {
            files.retain(|f| f != file_path);
        }

        // Remove from content index
        for files in self.content_index.values_mut() {
            files.retain(|f| f != file_path);
        }

        // Remove modification time
        self.modification_times.remove(file_path);
    }

    /// Clear the entire index
    pub fn clear(&mut self) {
        self.filename_index.clear();
        self.content_index.clear();
        self.modification_times.clear();
    }

    /// Tokenize filename into searchable words
    fn tokenize_filename(filename: &str) -> Vec<String> {
        let mut words = Vec::new();
        
        // Split on common separators
        for part in filename.split(&['.', '_', '-', ' '][..]) {
            if !part.is_empty() {
                words.push(part.to_string());
            }
        }

        // Split camelCase/PascalCase
        let mut current_word = String::new();
        for c in filename.chars() {
            if c.is_uppercase() && !current_word.is_empty() {
                words.push(current_word);
                current_word = String::new();
            }
            current_word.push(c);
        }
        if !current_word.is_empty() {
            words.push(current_word);
        }

        words
    }

    /// Tokenize content into searchable words
    fn tokenize_content(content: &str) -> Vec<String> {
        content
            .split_whitespace()
            .map(|word| {
                // Remove punctuation
                word.chars()
                    .filter(|c| c.is_alphanumeric() || *c == '_')
                    .collect::<String>()
            })
            .filter(|word| !word.is_empty())
            .collect()
    }

    /// Check if a file is a text file (for content indexing)
    fn is_text_file(path: &Path) -> bool {
        if let Some(extension) = path.extension() {
            if let Some(ext_str) = extension.to_str() {
                matches!(ext_str.to_lowercase().as_str(),
                    "rs" | "js" | "ts" | "tsx" | "py" | "html" | "css" | "scss" | 
                    "vue" | "json" | "toml" | "yaml" | "yml" | "md" | "txt" |
                    "go" | "java" | "cpp" | "c" | "h" | "hpp" | "cs" | "php" |
                    "rb" | "swift" | "kt" | "scala" | "sh" | "bat" | "ps1"
                )
            } else {
                false
            }
        } else {
            // Files without extensions might be text (like Dockerfile, README)
            if let Some(filename) = path.file_name() {
                if let Some(name_str) = filename.to_str() {
                    matches!(name_str.to_uppercase().as_str(),
                        "README" | "LICENSE" | "CHANGELOG" | "DOCKERFILE" |
                        "MAKEFILE" | "GITIGNORE" | "GITATTRIBUTES"
                    )
                } else {
                    false
                }
            } else {
                false
            }
        }
    }
}

/// Search result type
#[derive(Debug, Clone, PartialEq)]
pub enum SearchResultType {
    /// Found in filename
    Filename,
    /// Found in file content
    Content,
}

/// Drop implementation for FileManager to ensure proper cleanup
impl Drop for FileManager {
    fn drop(&mut self) {
        // Shutdown file watcher when FileManager is dropped
        if let Err(e) = self.shutdown_file_watcher() {
            eprintln!("Error shutting down file watcher: {}", e);
        }
    }
}