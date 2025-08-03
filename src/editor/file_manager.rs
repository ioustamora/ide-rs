//! File Manager with Multi-Tab Support
//! 
//! Provides VS Code-inspired file management with tabs, file type recognition,
//! and automatic mode switching between code editor and visual designer.

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use crate::editor::code_editor::CodeEditor;
use crate::editor::visual_designer::VisualDesigner;

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
    /// File type associations
    pub file_associations: HashMap<String, FileType>,
    /// Recent files history
    pub recent_files: Vec<PathBuf>,
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
            file_associations: HashMap::new(),
            recent_files: Vec::new(),
        };
        
        manager.initialize_file_associations();
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
        
        // Remove from open tabs
        self.open_tabs.remove(path);
        self.tab_order.retain(|p| p != path);
        
        // Update active tab
        if self.active_tab.as_ref() == Some(path) {
            self.active_tab = self.tab_order.last().cloned();
        }
        
        Ok(())
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
                let _ = self.close_tab(&path);
            }
            
            // New file button
            if ui.button("+ New").clicked() {
                // TODO: Implement new file creation
            }
        });
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