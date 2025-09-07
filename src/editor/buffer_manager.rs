//! BufferManager with file dirty tracking and notify-based watcher
//!
//! Provides open buffers map, file dirty tracking, notify-based watcher, persisted config,
//! and async WorkspaceTask queue as specified in the improvement plan Phase P0.

use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event, EventKind};
use tokio::sync::mpsc;
use serde::{Serialize, Deserialize};
use crate::editor::text_buffer::{TextBuffer, TextBufferError};

/// Central buffer manager for all open files
pub struct BufferManager {
    /// Open buffers by file path
    pub buffers: HashMap<PathBuf, Buffer>,
    /// Active buffer (currently focused)
    pub active_buffer: Option<PathBuf>,
    /// File system watcher
    pub file_watcher: Option<FileWatcher>,
    /// Workspace task queue
    pub task_queue: WorkspaceTaskQueue,
    /// Configuration
    pub config: BufferManagerConfig,
    /// Buffer statistics
    pub stats: BufferManagerStats,
    /// Change listeners
    pub change_listeners: Vec<Box<dyn BufferChangeListener>>,
}

/// Individual buffer with metadata and state
pub struct Buffer {
    /// File path (None for unsaved buffers)
    pub file_path: Option<PathBuf>,
    /// Buffer ID for identification
    pub buffer_id: BufferId,
    /// Text content
    pub text_buffer: TextBuffer,
    /// Buffer metadata
    pub metadata: BufferMetadata,
    /// Dirty state tracking
    pub dirty_state: DirtyState,
    /// Auto-save configuration
    pub auto_save: AutoSaveConfig,
    /// Buffer-specific settings
    pub settings: BufferSettings,
    /// Last access time
    pub last_accessed: Instant,
    /// Buffer statistics
    pub stats: BufferStats,
}

/// Unique buffer identifier
pub type BufferId = uuid::Uuid;

/// Buffer metadata and file information
#[derive(Clone, Debug)]
pub struct BufferMetadata {
    /// Display name
    pub display_name: String,
    /// File size in bytes
    pub file_size: u64,
    /// File last modified time
    pub last_modified: Option<SystemTime>,
    /// File permissions
    pub readonly: bool,
    /// File encoding
    pub encoding: String,
    /// Line ending style
    pub line_ending: LineEndingStyle,
    /// Language/syntax type
    pub language: Option<String>,
    /// File type category
    pub file_type: FileType,
}

/// Line ending styles
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LineEndingStyle {
    Unix,    // \n
    Windows, // \r\n
    Mac,     // \r
    Mixed,   // Mixed line endings
}

/// File type categorization
#[derive(Clone, Debug, PartialEq)]
pub enum FileType {
    Text,
    Code(String), // Language name
    Binary,
    Image,
    Archive,
    Unknown,
}

/// Buffer dirty state tracking
#[derive(Clone, Debug)]
pub struct DirtyState {
    /// Buffer has unsaved changes
    pub is_dirty: bool,
    /// Dirty since timestamp
    pub dirty_since: Option<Instant>,
    /// Change counter
    pub change_count: u64,
    /// Last save time
    pub last_saved: Option<Instant>,
    /// Checksum of last saved content
    pub saved_checksum: Option<u64>,
}

/// Auto-save configuration per buffer
#[derive(Clone, Debug)]
pub struct AutoSaveConfig {
    /// Auto-save enabled
    pub enabled: bool,
    /// Auto-save interval
    pub interval: Duration,
    /// Last auto-save time
    pub last_auto_save: Option<Instant>,
    /// Auto-save on focus loss
    pub save_on_focus_loss: bool,
}

/// Buffer-specific settings
#[derive(Clone, Debug)]
pub struct BufferSettings {
    /// Tab size
    pub tab_size: usize,
    /// Use spaces instead of tabs
    pub insert_spaces: bool,
    /// Trim trailing whitespace on save
    pub trim_trailing_whitespace: bool,
    /// Insert final newline on save
    pub insert_final_newline: bool,
    /// Word wrap
    pub word_wrap: bool,
    /// Show line numbers
    pub show_line_numbers: bool,
}

/// Buffer statistics
#[derive(Clone, Debug, Default)]
pub struct BufferStats {
    /// Number of times opened
    pub open_count: u64,
    /// Total edit time
    pub edit_time: Duration,
    /// Number of edits
    pub edit_count: u64,
    /// Number of saves
    pub save_count: u64,
}

/// File system watcher integration
pub struct FileWatcher {
    /// Notify watcher instance
    pub watcher: RecommendedWatcher,
    /// Event receiver
    pub event_receiver: mpsc::UnboundedReceiver<notify::Result<Event>>,
    /// Watched paths
    pub watched_paths: HashMap<PathBuf, WatchConfig>,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn FileEventHandler>>,
}

/// Watch configuration for paths
#[derive(Clone, Debug)]
pub struct WatchConfig {
    /// Recursive watching
    pub recursive: bool,
    /// File patterns to watch
    pub patterns: Vec<String>,
    /// File patterns to ignore
    pub ignore_patterns: Vec<String>,
    /// Debounce delay
    pub debounce_delay: Duration,
}

/// File system event handler
pub trait FileEventHandler: Send + Sync {
    /// Handle file system event
    fn handle_event(&self, event: &FileSystemEvent);
    /// Get handler name
    fn name(&self) -> &str;
}

/// Processed file system event
#[derive(Clone, Debug)]
pub struct FileSystemEvent {
    /// Event type
    pub event_type: FileSystemEventType,
    /// Affected file path
    pub path: PathBuf,
    /// Event timestamp
    pub timestamp: Instant,
    /// Additional event data
    pub metadata: HashMap<String, String>,
}

/// File system event types
#[derive(Clone, Debug, PartialEq)]
pub enum FileSystemEventType {
    Created,
    Modified,
    Deleted,
    Renamed(PathBuf), // Old path
    MovedIn,
    MovedOut,
}

/// Workspace task queue for background operations
pub struct WorkspaceTaskQueue {
    /// Task queue
    pub tasks: VecDeque<WorkspaceTask>,
    /// Running tasks
    pub running_tasks: HashMap<TaskId, RunningTask>,
    /// Task executor
    pub executor: TaskExecutor,
    /// Task history
    pub task_history: VecDeque<TaskHistoryEntry>,
    /// Queue statistics
    pub stats: TaskQueueStats,
}

/// Unique task identifier
pub type TaskId = uuid::Uuid;

/// Background workspace task
#[derive(Clone, Debug)]
pub struct WorkspaceTask {
    /// Task ID
    pub task_id: TaskId,
    /// Task type
    pub task_type: WorkspaceTaskType,
    /// Task priority
    pub priority: TaskPriority,
    /// Task parameters
    pub parameters: HashMap<String, String>,
    /// Progress callback
    pub progress_callback: Option<String>,
    /// Task dependencies
    pub dependencies: Vec<TaskId>,
    /// Created timestamp
    pub created_at: Instant,
    /// Estimated duration
    pub estimated_duration: Option<Duration>,
}

/// Types of workspace tasks
#[derive(Clone, Debug, PartialEq)]
pub enum WorkspaceTaskType {
    /// Index files for search
    IndexFiles,
    /// Parse syntax for highlighting
    ParseSyntax,
    /// Load file content
    LoadFile,
    /// Save file content
    SaveFile,
    /// Format file
    FormatFile,
    /// Run linter
    RunLinter,
    /// Build project
    BuildProject,
    /// Run tests
    RunTests,
    /// Custom task
    Custom(String),
}

/// Task priority levels
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Running task information
pub struct RunningTask {
    /// Task information
    pub task: WorkspaceTask,
    /// Start time
    pub started_at: Instant,
    /// Progress (0.0 to 1.0)
    pub progress: f32,
    /// Status message
    pub status_message: String,
    /// Cancellation token
    pub cancel_token: tokio_util::sync::CancellationToken,
}

/// Task executor for background operations
pub struct TaskExecutor {
    /// Worker threads
    pub worker_count: usize,
    /// Task handlers by type
    pub task_handlers: HashMap<WorkspaceTaskType, Box<dyn TaskHandler>>,
    /// Execution statistics
    pub execution_stats: HashMap<WorkspaceTaskType, TaskExecutionStats>,
}

/// Task handler trait
pub trait TaskHandler: Send + Sync {
    /// Execute task synchronously
    fn execute_sync(&self, task: &WorkspaceTask) -> Result<TaskResult, TaskError>;
    /// Check if can handle task type
    fn can_handle(&self, task_type: &WorkspaceTaskType) -> bool;
    /// Get estimated duration
    fn estimate_duration(&self, task: &WorkspaceTask) -> Option<Duration>;
}

/// Task execution result
#[derive(Clone, Debug)]
pub struct TaskResult {
    /// Result data
    pub data: HashMap<String, String>,
    /// Output messages
    pub output: Vec<String>,
    /// Execution time
    pub execution_time: Duration,
}

/// Task execution error
#[derive(Debug, thiserror::Error)]
pub enum TaskError {
    #[error("Task cancelled")]
    Cancelled,
    #[error("Task timeout")]
    Timeout,
    #[error("Task failed: {0}")]
    Failed(String),
    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Task history entry
#[derive(Clone, Debug)]
pub struct TaskHistoryEntry {
    /// Task that was executed
    pub task: WorkspaceTask,
    /// Execution result
    pub result: Result<TaskResult, TaskError>,
    /// Started timestamp
    pub started_at: Instant,
    /// Completed timestamp
    pub completed_at: Instant,
}

/// Task execution statistics
#[derive(Clone, Debug, Default)]
pub struct TaskExecutionStats {
    /// Total executions
    pub total_executions: u64,
    /// Successful executions
    pub successful_executions: u64,
    /// Failed executions
    pub failed_executions: u64,
    /// Average execution time
    pub average_execution_time: Duration,
    /// Total execution time
    pub total_execution_time: Duration,
}

/// Task queue statistics
#[derive(Clone, Debug, Default)]
pub struct TaskQueueStats {
    /// Total tasks queued
    pub total_queued: u64,
    /// Total tasks completed
    pub total_completed: u64,
    /// Currently running tasks
    pub currently_running: u64,
    /// Average queue time
    pub average_queue_time: Duration,
}

/// Buffer manager configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BufferManagerConfig {
    /// Maximum number of open buffers
    pub max_open_buffers: usize,
    /// Auto-save interval
    pub auto_save_interval: Duration,
    /// Auto-save enabled by default
    pub auto_save_enabled: bool,
    /// File watcher enabled
    pub file_watcher_enabled: bool,
    /// Watch patterns
    pub watch_patterns: Vec<String>,
    /// Ignore patterns
    pub ignore_patterns: Vec<String>,
    /// Task queue size limit
    pub task_queue_limit: usize,
    /// Worker thread count
    pub worker_thread_count: usize,
    /// Buffer cleanup interval
    pub cleanup_interval: Duration,
    /// Inactive buffer timeout
    pub inactive_buffer_timeout: Duration,
}

/// Buffer manager statistics
#[derive(Clone, Debug, Default)]
pub struct BufferManagerStats {
    /// Total buffers created
    pub total_buffers_created: u64,
    /// Total buffers closed
    pub total_buffers_closed: u64,
    /// Current open buffers
    pub current_open_buffers: u64,
    /// Total bytes managed
    pub total_bytes_managed: u64,
    /// File system events processed
    pub fs_events_processed: u64,
    /// Tasks executed
    pub tasks_executed: u64,
}

/// Buffer change listener trait
pub trait BufferChangeListener: Send + Sync {
    /// Buffer was opened
    fn on_buffer_opened(&self, buffer_id: BufferId, path: &Option<PathBuf>);
    /// Buffer was closed
    fn on_buffer_closed(&self, buffer_id: BufferId, path: &Option<PathBuf>);
    /// Buffer content changed
    fn on_buffer_changed(&self, buffer_id: BufferId, path: &Option<PathBuf>);
    /// Buffer was saved
    fn on_buffer_saved(&self, buffer_id: BufferId, path: &PathBuf);
    /// Buffer dirty state changed
    fn on_dirty_state_changed(&self, buffer_id: BufferId, is_dirty: bool);
}

impl BufferManager {
    /// Create new buffer manager
    pub fn new(config: BufferManagerConfig) -> Result<Self, BufferManagerError> {
        let mut manager = Self {
            buffers: HashMap::new(),
            active_buffer: None,
            file_watcher: None,
            task_queue: WorkspaceTaskQueue::new(config.worker_thread_count),
            config,
            stats: BufferManagerStats::default(),
            change_listeners: Vec::new(),
        };
        
        // Initialize file watcher if enabled
        if manager.config.file_watcher_enabled {
            manager.file_watcher = Some(FileWatcher::new()?);
        }
        
        Ok(manager)
    }

    /// Open file in buffer
    pub async fn open_file<P: AsRef<Path>>(&mut self, path: P) -> Result<BufferId, BufferManagerError> {
        let path = path.as_ref().to_path_buf();
        
        // Check if file is already open
        if let Some(buffer) = self.buffers.get(&path) {
            self.active_buffer = Some(path.clone());
            return Ok(buffer.buffer_id);
        }
        
        // Check buffer limit
        if self.buffers.len() >= self.config.max_open_buffers {
            self.close_least_recently_used_buffer()?;
        }
        
        // Load file content
        let text_buffer = TextBuffer::from_file(path.clone())
            .map_err(BufferManagerError::Io)?;
        
        // Create buffer
        let buffer_id = BufferId::new_v4();
        let metadata = Self::create_metadata(&path, &text_buffer).await?;
        
        let buffer = Buffer {
            file_path: Some(path.clone()),
            buffer_id,
            text_buffer,
            metadata,
            dirty_state: DirtyState::clean(),
            auto_save: AutoSaveConfig::new(self.config.auto_save_enabled, self.config.auto_save_interval),
            settings: BufferSettings::default(),
            last_accessed: Instant::now(),
            stats: BufferStats::default(),
        };
        
        // Add to buffers
        self.buffers.insert(path.clone(), buffer);
        self.active_buffer = Some(path.clone());
        
        // Start watching file
        if let Some(ref mut watcher) = self.file_watcher {
            watcher.watch_path(&path, WatchConfig::default())?;
        }
        
        // Update statistics
        self.stats.total_buffers_created += 1;
        self.stats.current_open_buffers += 1;
        
        // Notify listeners
        for listener in &self.change_listeners {
            listener.on_buffer_opened(buffer_id, &Some(path.clone()));
        }
        
        Ok(buffer_id)
    }

    /// Create new unsaved buffer
    pub fn create_buffer(&mut self, content: Option<String>) -> BufferId {
        let buffer_id = BufferId::new_v4();
        let text_buffer = content
            .map(TextBuffer::from_string)
            .unwrap_or_else(TextBuffer::new);
        
        let metadata = BufferMetadata::for_unsaved_buffer();
        
        let buffer = Buffer {
            file_path: None,
            buffer_id,
            text_buffer,
            metadata,
            dirty_state: DirtyState::clean(),
            auto_save: AutoSaveConfig::new(false, Duration::from_secs(300)), // No auto-save for unsaved
            settings: BufferSettings::default(),
            last_accessed: Instant::now(),
            stats: BufferStats::default(),
        };
        
        // Generate temporary path for unsaved buffer
        let temp_path = PathBuf::from(format!("unsaved-{}", buffer_id));
        self.buffers.insert(temp_path.clone(), buffer);
        self.active_buffer = Some(temp_path);
        
        // Update statistics
        self.stats.total_buffers_created += 1;
        self.stats.current_open_buffers += 1;
        
        // Notify listeners
        for listener in &self.change_listeners {
            listener.on_buffer_opened(buffer_id, &None);
        }
        
        buffer_id
    }

    /// Save buffer to file
    pub async fn save_buffer(&mut self, buffer_id: BufferId, path: Option<PathBuf>) -> Result<(), BufferManagerError> {
        let buffer_path = self.find_buffer_path(buffer_id)
            .ok_or(BufferManagerError::BufferNotFound(buffer_id))?;
        
        let buffer = self.buffers.get_mut(&buffer_path)
            .ok_or(BufferManagerError::BufferNotFound(buffer_id))?;
        
        let save_path = path.or_else(|| buffer.file_path.clone())
            .ok_or(BufferManagerError::NoSavePath)?;
        
        // Save content to file
        let content = buffer.text_buffer.to_string();
        tokio::fs::write(&save_path, content).await
            .map_err(BufferManagerError::Io)?;
        
        // Update buffer state
        buffer.file_path = Some(save_path.clone());
        buffer.dirty_state.mark_saved();
        buffer.stats.save_count += 1;
        
        // Update metadata
        buffer.metadata = Self::create_metadata(&save_path, &buffer.text_buffer).await?;
        
        // If path changed, update buffer mapping
        if buffer_path != save_path {
            let buffer = self.buffers.remove(&buffer_path).unwrap();
            self.buffers.insert(save_path.clone(), buffer);
            
            if self.active_buffer == Some(buffer_path) {
                self.active_buffer = Some(save_path.clone());
            }
        }
        
        // Start watching new file
        if let Some(ref mut watcher) = self.file_watcher {
            watcher.watch_path(&save_path, WatchConfig::default())?;
        }
        
        // Notify listeners
        for listener in &self.change_listeners {
            listener.on_buffer_saved(buffer_id, &save_path);
            listener.on_dirty_state_changed(buffer_id, false);
        }
        
        Ok(())
    }

    /// Close buffer
    pub fn close_buffer(&mut self, buffer_id: BufferId) -> Result<(), BufferManagerError> {
        let buffer_path = self.find_buffer_path(buffer_id)
            .ok_or(BufferManagerError::BufferNotFound(buffer_id))?;
        
        let buffer = self.buffers.remove(&buffer_path)
            .ok_or(BufferManagerError::BufferNotFound(buffer_id))?;
        
        // Stop watching file
        if let (Some(ref mut watcher), Some(ref path)) = (&mut self.file_watcher, &buffer.file_path) {
            watcher.unwatch_path(path);
        }
        
        // Update active buffer
        if self.active_buffer == Some(buffer_path) {
            self.active_buffer = self.buffers.keys().next().cloned();
        }
        
        // Update statistics
        self.stats.total_buffers_closed += 1;
        self.stats.current_open_buffers -= 1;
        
        // Notify listeners
        for listener in &self.change_listeners {
            listener.on_buffer_closed(buffer_id, &buffer.file_path);
        }
        
        Ok(())
    }

    /// Get buffer by ID
    pub fn get_buffer(&self, buffer_id: BufferId) -> Option<&Buffer> {
        let path = self.find_buffer_path(buffer_id)?;
        self.buffers.get(&path)
    }

    /// Get buffer mutably by ID
    pub fn get_buffer_mut(&mut self, buffer_id: BufferId) -> Option<&mut Buffer> {
        let path = self.find_buffer_path(buffer_id)?;
        self.buffers.get_mut(&path)
    }

    /// Get active buffer
    pub fn get_active_buffer(&self) -> Option<&Buffer> {
        let path = self.active_buffer.as_ref()?;
        self.buffers.get(path)
    }

    /// Get active buffer mutably
    pub fn get_active_buffer_mut(&mut self) -> Option<&mut Buffer> {
        let path = self.active_buffer.as_ref()?.clone();
        self.buffers.get_mut(&path)
    }

    /// Set active buffer
    pub fn set_active_buffer(&mut self, buffer_id: BufferId) -> Result<(), BufferManagerError> {
        let path = self.find_buffer_path(buffer_id)
            .ok_or(BufferManagerError::BufferNotFound(buffer_id))?;
        
        self.active_buffer = Some(path.clone());
        
        // Update last accessed time
        if let Some(buffer) = self.buffers.get_mut(&path) {
            buffer.last_accessed = Instant::now();
        }
        
        Ok(())
    }

    /// Get all open buffers
    pub fn get_open_buffers(&self) -> Vec<(BufferId, &PathBuf)> {
        self.buffers
            .iter()
            .map(|(path, buffer)| (buffer.buffer_id, path))
            .collect()
    }

    /// Check if any buffers are dirty
    pub fn has_dirty_buffers(&self) -> bool {
        self.buffers.values().any(|buffer| buffer.dirty_state.is_dirty)
    }

    /// Get dirty buffers
    pub fn get_dirty_buffers(&self) -> Vec<BufferId> {
        self.buffers
            .values()
            .filter(|buffer| buffer.dirty_state.is_dirty)
            .map(|buffer| buffer.buffer_id)
            .collect()
    }

    /// Add buffer change listener
    pub fn add_change_listener(&mut self, listener: Box<dyn BufferChangeListener>) {
        self.change_listeners.push(listener);
    }

    /// Queue workspace task
    pub fn queue_task(&mut self, task: WorkspaceTask) -> Result<(), BufferManagerError> {
        self.task_queue.queue_task(task)?;
        self.stats.tasks_executed += 1;
        Ok(())
    }

    /// Process file system events
    pub async fn process_file_events(&mut self) -> Result<(), BufferManagerError> {
        let mut fs_events = Vec::new();
        
        // First collect all events to avoid borrowing issues
        if let Some(ref mut watcher) = self.file_watcher {
            while let Ok(event) = watcher.event_receiver.try_recv() {
                match event {
                    Ok(notify_event) => {
                        let fs_event = Self::convert_notify_event(notify_event);
                        fs_events.push(fs_event);
                    }
                    Err(e) => {
                        eprintln!("File watcher error: {:?}", e);
                    }
                }
            }
        }
        
        // Now process collected events
        for fs_event in fs_events {
            self.handle_file_system_event(&fs_event).await?;
            self.stats.fs_events_processed += 1;
        }
        
        Ok(())
    }

    /// Handle file system event
    async fn handle_file_system_event(&mut self, event: &FileSystemEvent) -> Result<(), BufferManagerError> {
        match &event.event_type {
            FileSystemEventType::Modified => {
                // Check if file is open and update if needed
                if let Some(buffer) = self.buffers.get_mut(&event.path) {
                    // Check if file was modified externally
                    let metadata = tokio::fs::metadata(&event.path).await
                        .map_err(BufferManagerError::Io)?;
                    
                    if let (Some(file_modified), Some(buffer_modified)) = 
                        (metadata.modified().ok(), buffer.metadata.last_modified) {
                        if file_modified > buffer_modified {
                            // File was modified externally - could trigger reload dialog
                            // For now, just update metadata
                            buffer.metadata.last_modified = Some(file_modified);
                        }
                    }
                }
            }
            FileSystemEventType::Deleted => {
                // Mark buffer as deleted if open
                if let Some(buffer) = self.buffers.get_mut(&event.path) {
                    buffer.metadata.file_type = FileType::Unknown;
                    // Could show warning that file was deleted externally
                }
            }
            FileSystemEventType::Renamed(old_path) => {
                // Update buffer path if renamed
                if let Some(buffer) = self.buffers.remove(old_path) {
                    self.buffers.insert(event.path.clone(), buffer);
                    
                    if self.active_buffer.as_ref() == Some(old_path) {
                        self.active_buffer = Some(event.path.clone());
                    }
                }
            }
            _ => {
                // Handle other events as needed
            }
        }
        Ok(())
    }

    /// Auto-save dirty buffers
    pub async fn auto_save(&mut self) -> Result<Vec<BufferId>, BufferManagerError> {
        let mut saved_buffers = Vec::new();
        let now = Instant::now();
        
        for buffer in self.buffers.values_mut() {
            if buffer.should_auto_save(now) {
                if let Some(ref path) = buffer.file_path {
                    let content = buffer.text_buffer.to_string();
                    tokio::fs::write(path, content).await
                        .map_err(BufferManagerError::Io)?;
                    
                    buffer.dirty_state.mark_saved();
                    buffer.auto_save.last_auto_save = Some(now);
                    saved_buffers.push(buffer.buffer_id);
                    
                    // Notify listeners
                    for listener in &self.change_listeners {
                        listener.on_buffer_saved(buffer.buffer_id, &path);
                        listener.on_dirty_state_changed(buffer.buffer_id, false);
                    }
                }
            }
        }
        
        Ok(saved_buffers)
    }

    /// Cleanup inactive buffers
    pub fn cleanup_inactive_buffers(&mut self) -> Result<Vec<BufferId>, BufferManagerError> {
        let mut closed_buffers = Vec::new();
        let now = Instant::now();
        let timeout = self.config.inactive_buffer_timeout;
        
        let paths_to_remove: Vec<PathBuf> = self.buffers
            .iter()
            .filter(|(_, buffer)| {
                !buffer.dirty_state.is_dirty && 
                now.duration_since(buffer.last_accessed) > timeout
            })
            .map(|(path, _)| path.clone())
            .collect();
        
        for path in paths_to_remove {
            if let Some(buffer) = self.buffers.remove(&path) {
                closed_buffers.push(buffer.buffer_id);
                self.stats.total_buffers_closed += 1;
                self.stats.current_open_buffers -= 1;
                
                // Notify listeners
                for listener in &self.change_listeners {
                    listener.on_buffer_closed(buffer.buffer_id, &buffer.file_path);
                }
            }
        }
        
        Ok(closed_buffers)
    }

    // Private helper methods
    fn find_buffer_path(&self, buffer_id: BufferId) -> Option<PathBuf> {
        self.buffers
            .iter()
            .find(|(_, buffer)| buffer.buffer_id == buffer_id)
            .map(|(path, _)| path.clone())
    }

    fn close_least_recently_used_buffer(&mut self) -> Result<(), BufferManagerError> {
        let lru_path = self.buffers
            .iter()
            .min_by_key(|(_, buffer)| buffer.last_accessed)
            .map(|(path, _)| path.clone());
        
        if let Some(path) = lru_path {
            if let Some(buffer) = self.buffers.remove(&path) {
                self.close_buffer(buffer.buffer_id)?;
            }
        }
        
        Ok(())
    }

    async fn create_metadata(path: &Path, text_buffer: &TextBuffer) -> Result<BufferMetadata, BufferManagerError> {
        let file_metadata = tokio::fs::metadata(path).await
            .map_err(BufferManagerError::Io)?;
        
        let display_name = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Unknown")
            .to_string();
        
        let language = Self::detect_language(path);
        let file_type = Self::determine_file_type(path, &file_metadata);
        
        Ok(BufferMetadata {
            display_name,
            file_size: file_metadata.len(),
            last_modified: file_metadata.modified().ok(),
            readonly: file_metadata.permissions().readonly(),
            encoding: "UTF-8".to_string(), // Would detect encoding
            line_ending: Self::detect_line_ending(&text_buffer.to_string()),
            language,
            file_type,
        })
    }

    fn detect_language(path: &Path) -> Option<String> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| match ext.to_lowercase().as_str() {
                "rs" => "rust",
                "js" | "jsx" => "javascript",
                "ts" | "tsx" => "typescript",
                "py" => "python",
                "java" => "java",
                "cpp" | "cc" | "cxx" => "cpp",
                "c" => "c",
                "h" | "hpp" => "c",
                "cs" => "csharp",
                "go" => "go",
                "php" => "php",
                "rb" => "ruby",
                "swift" => "swift",
                "kt" => "kotlin",
                "scala" => "scala",
                "clj" => "clojure",
                "hs" => "haskell",
                "ml" => "ocaml",
                "fs" => "fsharp",
                "elm" => "elm",
                "dart" => "dart",
                "lua" => "lua",
                "r" => "r",
                "sql" => "sql",
                "sh" | "bash" => "bash",
                "ps1" => "powershell",
                "bat" | "cmd" => "batch",
                "html" | "htm" => "html",
                "css" => "css",
                "scss" | "sass" => "scss",
                "less" => "less",
                "xml" => "xml",
                "json" => "json",
                "yaml" | "yml" => "yaml",
                "toml" => "toml",
                "md" | "markdown" => "markdown",
                "tex" => "latex",
                _ => return None,
            })
            .map(String::from)
    }

    fn determine_file_type(path: &Path, metadata: &std::fs::Metadata) -> FileType {
        if metadata.is_dir() {
            return FileType::Unknown;
        }
        
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            match ext.to_lowercase().as_str() {
                "png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" | "ico" => FileType::Image,
                "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" => FileType::Archive,
                "exe" | "dll" | "so" | "dylib" | "bin" => FileType::Binary,
                ext => {
                    if Self::detect_language(path).is_some() {
                        FileType::Code(ext.to_string())
                    } else {
                        FileType::Text
                    }
                }
            }
        } else {
            FileType::Text
        }
    }

    fn detect_line_ending(content: &str) -> LineEndingStyle {
        let has_crlf = content.contains("\r\n");
        let has_lf = content.contains('\n');
        let has_cr = content.contains('\r');
        
        match (has_crlf, has_lf, has_cr) {
            (true, true, true) => LineEndingStyle::Mixed,
            (true, _, _) => LineEndingStyle::Windows,
            (false, true, false) => LineEndingStyle::Unix,
            (false, false, true) => LineEndingStyle::Mac,
            _ => LineEndingStyle::Unix, // Default
        }
    }

    fn convert_notify_event(event: Event) -> FileSystemEvent {
        let event_type = match event.kind {
            EventKind::Create(_) => FileSystemEventType::Created,
            EventKind::Modify(_) => FileSystemEventType::Modified,
            EventKind::Remove(_) => FileSystemEventType::Deleted,
            _ => FileSystemEventType::Modified, // Default
        };
        
        let path = event.paths.into_iter().next().unwrap_or_default();
        
        FileSystemEvent {
            event_type,
            path,
            timestamp: Instant::now(),
            metadata: HashMap::new(),
        }
    }
}

// Implementation for associated types and traits
impl Buffer {
    fn should_auto_save(&self, now: Instant) -> bool {
        self.auto_save.enabled &&
        self.dirty_state.is_dirty &&
        self.auto_save.last_auto_save
            .map_or(true, |last| now.duration_since(last) >= self.auto_save.interval)
    }
}

impl DirtyState {
    fn clean() -> Self {
        Self {
            is_dirty: false,
            dirty_since: None,
            change_count: 0,
            last_saved: None,
            saved_checksum: None,
        }
    }
    
    fn mark_dirty(&mut self) {
        if !self.is_dirty {
            self.dirty_since = Some(Instant::now());
        }
        self.is_dirty = true;
        self.change_count += 1;
    }
    
    fn mark_saved(&mut self) {
        self.is_dirty = false;
        self.dirty_since = None;
        self.last_saved = Some(Instant::now());
    }
}

impl AutoSaveConfig {
    fn new(enabled: bool, interval: Duration) -> Self {
        Self {
            enabled,
            interval,
            last_auto_save: None,
            save_on_focus_loss: true,
        }
    }
}

impl Default for BufferSettings {
    fn default() -> Self {
        Self {
            tab_size: 4,
            insert_spaces: true,
            trim_trailing_whitespace: true,
            insert_final_newline: true,
            word_wrap: false,
            show_line_numbers: true,
        }
    }
}

impl BufferMetadata {
    fn for_unsaved_buffer() -> Self {
        Self {
            display_name: "Untitled".to_string(),
            file_size: 0,
            last_modified: None,
            readonly: false,
            encoding: "UTF-8".to_string(),
            line_ending: LineEndingStyle::Unix,
            language: None,
            file_type: FileType::Text,
        }
    }
}

impl FileWatcher {
    fn new() -> Result<Self, BufferManagerError> {
        let (tx, rx) = mpsc::unbounded_channel();
        
        let watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            notify::Config::default(),
        ).map_err(BufferManagerError::FileWatcher)?;
        
        Ok(Self {
            watcher,
            event_receiver: rx,
            watched_paths: HashMap::new(),
            event_handlers: Vec::new(),
        })
    }
    
    fn watch_path(&mut self, path: &Path, config: WatchConfig) -> Result<(), BufferManagerError> {
        let mode = if config.recursive {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };
        
        self.watcher.watch(path, mode)
            .map_err(BufferManagerError::FileWatcher)?;
        
        self.watched_paths.insert(path.to_path_buf(), config);
        Ok(())
    }
    
    fn unwatch_path(&mut self, path: &Path) {
        let _ = self.watcher.unwatch(path);
        self.watched_paths.remove(path);
    }
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            recursive: false,
            patterns: vec!["*".to_string()],
            ignore_patterns: vec![
                ".git/**".to_string(),
                "node_modules/**".to_string(),
                "target/**".to_string(),
                ".vscode/**".to_string(),
            ],
            debounce_delay: Duration::from_millis(250),
        }
    }
}

impl WorkspaceTaskQueue {
    fn new(worker_count: usize) -> Self {
        Self {
            tasks: VecDeque::new(),
            running_tasks: HashMap::new(),
            executor: TaskExecutor::new(worker_count),
            task_history: VecDeque::new(),
            stats: TaskQueueStats::default(),
        }
    }
    
    fn queue_task(&mut self, task: WorkspaceTask) -> Result<(), BufferManagerError> {
        if self.tasks.len() >= 1000 { // Configurable limit
            return Err(BufferManagerError::TaskQueueFull);
        }
        
        self.tasks.push_back(task);
        self.stats.total_queued += 1;
        Ok(())
    }
}

impl TaskExecutor {
    fn new(worker_count: usize) -> Self {
        Self {
            worker_count,
            task_handlers: HashMap::new(),
            execution_stats: HashMap::new(),
        }
    }
}

impl Default for BufferManagerConfig {
    fn default() -> Self {
        Self {
            max_open_buffers: 50,
            auto_save_interval: Duration::from_secs(300), // 5 minutes
            auto_save_enabled: true,
            file_watcher_enabled: true,
            watch_patterns: vec!["**/*".to_string()],
            ignore_patterns: vec![
                ".git/**".to_string(),
                "node_modules/**".to_string(),
                "target/**".to_string(),
            ],
            task_queue_limit: 1000,
            worker_thread_count: 4,
            cleanup_interval: Duration::from_secs(300),
            inactive_buffer_timeout: Duration::from_secs(3600), // 1 hour
        }
    }
}

/// Buffer manager errors
#[derive(Debug, thiserror::Error)]
pub enum BufferManagerError {
    #[error("Buffer not found: {0}")]
    BufferNotFound(BufferId),
    #[error("No save path specified for unsaved buffer")]
    NoSavePath,
    #[error("Task queue is full")]
    TaskQueueFull,
    #[error("Text buffer error: {0}")]
    TextBuffer(#[from] TextBufferError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("File watcher error: {0}")]
    FileWatcher(#[from] notify::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use tokio;

    #[tokio::test]
    async fn test_buffer_manager_creation() {
        let config = BufferManagerConfig::default();
        let manager = BufferManager::new(config).unwrap();
        
        assert_eq!(manager.buffers.len(), 0);
        assert!(manager.active_buffer.is_none());
    }

    #[tokio::test]
    async fn test_create_buffer() {
        let config = BufferManagerConfig::default();
        let mut manager = BufferManager::new(config).unwrap();
        
        let buffer_id = manager.create_buffer(Some("Hello World".to_string()));
        
        assert_eq!(manager.buffers.len(), 1);
        let buffer = manager.get_buffer(buffer_id).unwrap();
        assert_eq!(buffer.text_buffer.to_string(), "Hello World");
    }

    #[tokio::test]
    async fn test_open_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        tokio::fs::write(&file_path, "Test content").await.unwrap();
        
        let config = BufferManagerConfig::default();
        let mut manager = BufferManager::new(config).unwrap();
        
        let buffer_id = manager.open_file(&file_path).await.unwrap();
        
        let buffer = manager.get_buffer(buffer_id).unwrap();
        assert_eq!(buffer.text_buffer.to_string(), "Test content");
        assert_eq!(buffer.file_path, Some(file_path));
    }

    #[tokio::test]
    async fn test_save_buffer() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        
        let config = BufferManagerConfig::default();
        let mut manager = BufferManager::new(config).unwrap();
        
        let buffer_id = manager.create_buffer(Some("Test content".to_string()));
        manager.save_buffer(buffer_id, Some(file_path.clone())).await.unwrap();
        
        let content = tokio::fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(content, "Test content");
        
        let buffer = manager.get_buffer(buffer_id).unwrap();
        assert_eq!(buffer.file_path, Some(file_path));
        assert!(!buffer.dirty_state.is_dirty);
    }

    #[tokio::test]
    async fn test_dirty_state_tracking() {
        let config = BufferManagerConfig::default();
        let mut manager = BufferManager::new(config).unwrap();
        
        let buffer_id = manager.create_buffer(Some("Initial".to_string()));
        let buffer = manager.get_buffer_mut(buffer_id).unwrap();
        
        // Mark as dirty
        buffer.dirty_state.mark_dirty();
        assert!(buffer.dirty_state.is_dirty);
        assert!(buffer.dirty_state.dirty_since.is_some());
        
        // Mark as saved
        buffer.dirty_state.mark_saved();
        assert!(!buffer.dirty_state.is_dirty);
        assert!(buffer.dirty_state.last_saved.is_some());
    }

    #[test]
    fn test_line_ending_detection() {
        assert_eq!(
            BufferManager::detect_line_ending("line1\nline2\n"),
            LineEndingStyle::Unix
        );
        
        assert_eq!(
            BufferManager::detect_line_ending("line1\r\nline2\r\n"),
            LineEndingStyle::Windows
        );
        
        assert_eq!(
            BufferManager::detect_line_ending("line1\rline2\r"),
            LineEndingStyle::Mac
        );
        
        assert_eq!(
            BufferManager::detect_line_ending("line1\nline2\r\nline3\r"),
            LineEndingStyle::Mixed
        );
    }

    #[test]
    fn test_language_detection() {
        assert_eq!(
            BufferManager::detect_language(Path::new("test.rs")),
            Some("rust".to_string())
        );
        
        assert_eq!(
            BufferManager::detect_language(Path::new("test.js")),
            Some("javascript".to_string())
        );
        
        assert_eq!(
            BufferManager::detect_language(Path::new("test.unknown")),
            None
        );
    }
}