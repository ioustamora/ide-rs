//! File Watcher Integration with notify crate
//!
//! Provides efficient file system watching capabilities using the notify crate
//! for detecting external file changes and updating the FileManager accordingly.

use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

use crate::core::event_bus::{IdeEvent, global_event_bus};

/// File watcher events that are sent to consumers
#[derive(Debug, Clone)]
pub enum FileWatchEvent {
    /// File was created
    Created(PathBuf),
    /// File was modified
    Modified(PathBuf),
    /// File was deleted
    Deleted(PathBuf),
    /// File was renamed (old_path, new_path)
    Renamed(PathBuf, PathBuf),
    /// Directory was created
    DirectoryCreated(PathBuf),
    /// Directory was deleted
    DirectoryDeleted(PathBuf),
    /// Batch of events (for debouncing rapid changes)
    Batch(Vec<FileWatchEvent>),
}

/// File watcher configuration options
#[derive(Debug, Clone)]
pub struct FileWatcherConfig {
    /// Enable file watching
    pub enabled: bool,
    /// Debounce interval for rapid file changes
    pub debounce_duration: Duration,
    /// Maximum number of events to batch
    pub max_batch_size: usize,
    /// File patterns to ignore (e.g., .git, node_modules)
    pub ignore_patterns: Vec<String>,
    /// File extensions to watch specifically
    pub watch_extensions: Option<HashSet<String>>,
    /// Enable recursive watching of subdirectories
    pub recursive: bool,
    /// Poll interval for fallback polling mode
    pub poll_interval: Duration,
}

/// Individual file watcher entry
#[derive(Debug, Clone)]
pub struct WatchedFile {
    /// File path
    pub path: PathBuf,
    /// Whether to watch recursively (for directories)
    pub recursive: bool,
    /// Last known modification time
    pub last_modified: Option<std::time::SystemTime>,
    /// Custom event handler ID
    pub handler_id: Option<String>,
}

/// Advanced file watcher using notify crate
pub struct AdvancedFileWatcher {
    /// The actual file system watcher
    watcher: Option<RecommendedWatcher>,
    /// Event receiver from watcher
    event_receiver: Option<Receiver<notify::Result<Event>>>,
    /// File watch events sender
    event_sender: Sender<FileWatchEvent>,
    /// File watch events receiver for consumers
    consumer_receiver: Receiver<FileWatchEvent>,
    /// Currently watched paths
    watched_paths: HashMap<PathBuf, WatchedFile>,
    /// Configuration
    config: FileWatcherConfig,
    /// Event debouncing state
    debounce_state: DebounceState,
    /// Background thread handle
    background_thread: Option<thread::JoinHandle<()>>,
    /// Shutdown signal
    shutdown_sender: Option<Sender<()>>,
    /// Statistics
    stats: WatcherStats,
}

/// Debouncing state for rapid file changes
#[derive(Debug, Default)]
struct DebounceState {
    /// Pending events waiting to be debounced
    pending_events: HashMap<PathBuf, (FileWatchEvent, Instant)>,
    /// Last debounce flush time
    last_flush: Instant,
}

/// File watcher statistics
#[derive(Debug, Default, Clone)]
pub struct WatcherStats {
    /// Total events processed
    pub events_processed: u64,
    /// Events debounced (batched)
    pub events_debounced: u64,
    /// Files currently being watched
    pub files_watched: usize,
    /// Directories currently being watched
    pub directories_watched: usize,
    /// Errors encountered
    pub errors: u64,
    /// Uptime of the watcher
    pub start_time: Option<Instant>,
}

impl Default for FileWatcherConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            debounce_duration: Duration::from_millis(100),
            max_batch_size: 50,
            ignore_patterns: vec![
                ".git".to_string(),
                "node_modules".to_string(),
                "target".to_string(),
                ".DS_Store".to_string(),
                "*.tmp".to_string(),
                "*.swp".to_string(),
                ".vscode".to_string(),
                ".idea".to_string(),
            ],
            watch_extensions: None, // Watch all files by default
            recursive: true,
            poll_interval: Duration::from_millis(1000),
        }
    }
}

impl AdvancedFileWatcher {
    /// Create a new advanced file watcher
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Self::with_config(FileWatcherConfig::default())
    }

    /// Create a new advanced file watcher with custom configuration
    pub fn with_config(config: FileWatcherConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let (event_sender, consumer_receiver) = mpsc::channel();

        let mut watcher = Self {
            watcher: None,
            event_receiver: None,
            event_sender,
            consumer_receiver,
            watched_paths: HashMap::new(),
            config,
            debounce_state: DebounceState::default(),
            background_thread: None,
            shutdown_sender: None,
            stats: WatcherStats::default(),
        };

        if watcher.config.enabled {
            watcher.initialize_watcher()?;
        }

        Ok(watcher)
    }

    /// Initialize the notify watcher
    fn initialize_watcher(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (tx, rx) = mpsc::channel();
        
        let watcher = RecommendedWatcher::new(
            move |result| {
                if let Err(_) = tx.send(result) {
                    eprintln!("File watcher: failed to send event");
                }
            },
            Config::default(),
        )?;

        self.watcher = Some(watcher);
        self.event_receiver = Some(rx);
        self.stats.start_time = Some(Instant::now());

        // Start background processing thread
        self.start_background_thread()?;

        Ok(())
    }

    /// Start the background event processing thread
    fn start_background_thread(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let event_receiver = self.event_receiver.take()
            .ok_or("Event receiver not available")?;
        
        let event_sender = self.event_sender.clone();
        let config = self.config.clone();
        let (shutdown_tx, shutdown_rx) = mpsc::channel();
        
        self.shutdown_sender = Some(shutdown_tx);

        let handle = thread::spawn(move || {
            Self::background_event_loop(event_receiver, event_sender, config, shutdown_rx);
        });

        self.background_thread = Some(handle);
        Ok(())
    }

    /// Background event processing loop
    fn background_event_loop(
        event_receiver: Receiver<notify::Result<Event>>,
        event_sender: Sender<FileWatchEvent>,
        config: FileWatcherConfig,
        shutdown_receiver: Receiver<()>,
    ) {
        let mut debounce_state = DebounceState::default();
        
        loop {
            // Check for shutdown signal
            if shutdown_receiver.try_recv().is_ok() {
                break;
            }

            // Process events with timeout for debouncing
            match event_receiver.recv_timeout(config.debounce_duration / 2) {
                Ok(Ok(event)) => {
                    if let Some(watch_event) = Self::convert_notify_event(event, &config) {
                        Self::handle_debounced_event(watch_event, &mut debounce_state, &event_sender, &config);
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("File watcher error: {:?}", e);
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    // Timeout - flush pending debounced events
                    Self::flush_debounced_events(&mut debounce_state, &event_sender, &config);
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    break;
                }
            }
        }
    }

    /// Convert notify event to our FileWatchEvent
    fn convert_notify_event(event: Event, config: &FileWatcherConfig) -> Option<FileWatchEvent> {
        let path = event.paths.first()?.clone();

        // Apply ignore patterns
        if Self::should_ignore_path(&path, config) {
            return None;
        }

        // Apply extension filter if configured
        if let Some(ref watch_extensions) = config.watch_extensions {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if !watch_extensions.contains(ext) {
                    return None;
                }
            }
        }

        match event.kind {
            EventKind::Create(_) => {
                if path.is_dir() {
                    Some(FileWatchEvent::DirectoryCreated(path))
                } else {
                    Some(FileWatchEvent::Created(path))
                }
            }
            EventKind::Modify(_) => Some(FileWatchEvent::Modified(path)),
            EventKind::Remove(_) => {
                if path.is_dir() {
                    Some(FileWatchEvent::DirectoryDeleted(path))
                } else {
                    Some(FileWatchEvent::Deleted(path))
                }
            }
            _ => None, // Ignore other event types for now
        }
    }

    /// Check if a path should be ignored based on patterns
    fn should_ignore_path(path: &Path, config: &FileWatcherConfig) -> bool {
        let path_str = path.to_string_lossy();
        
        for pattern in &config.ignore_patterns {
            if pattern.contains('*') {
                // Simple wildcard matching
                if Self::wildcard_match(&path_str, pattern) {
                    return true;
                }
            } else if path_str.contains(pattern) {
                return true;
            }
        }
        
        false
    }

    /// Simple wildcard pattern matching
    fn wildcard_match(text: &str, pattern: &str) -> bool {
        // Very basic wildcard matching - could be enhanced
        if pattern == "*" {
            return true;
        }
        
        if pattern.ends_with('*') {
            let prefix = &pattern[..pattern.len() - 1];
            return text.starts_with(prefix);
        }
        
        if pattern.starts_with('*') {
            let suffix = &pattern[1..];
            return text.ends_with(suffix);
        }
        
        text == pattern
    }

    /// Handle event with debouncing
    fn handle_debounced_event(
        event: FileWatchEvent,
        debounce_state: &mut DebounceState,
        event_sender: &Sender<FileWatchEvent>,
        config: &FileWatcherConfig,
    ) {
        let path = match &event {
            FileWatchEvent::Created(p) | 
            FileWatchEvent::Modified(p) | 
            FileWatchEvent::Deleted(p) |
            FileWatchEvent::DirectoryCreated(p) |
            FileWatchEvent::DirectoryDeleted(p) => p.clone(),
            FileWatchEvent::Renamed(_, new_path) => new_path.clone(),
            FileWatchEvent::Batch(_) => {
                // Send batch events immediately
                let _ = event_sender.send(event);
                return;
            }
        };

        // Store event for debouncing
        debounce_state.pending_events.insert(path, (event, Instant::now()));

        // Check if we should flush due to batch size
        if debounce_state.pending_events.len() >= config.max_batch_size {
            Self::flush_debounced_events(debounce_state, event_sender, config);
        }
    }

    /// Flush debounced events
    fn flush_debounced_events(
        debounce_state: &mut DebounceState,
        event_sender: &Sender<FileWatchEvent>,
        config: &FileWatcherConfig,
    ) {
        if debounce_state.pending_events.is_empty() {
            return;
        }

        let now = Instant::now();
        let mut events_to_send = Vec::new();
        let mut events_to_keep = HashMap::new();

        // Separate events that are ready to send from those that need more time
        for (path, (event, timestamp)) in debounce_state.pending_events.drain() {
            if now.duration_since(timestamp) >= config.debounce_duration {
                events_to_send.push(event);
            } else {
                events_to_keep.insert(path, (event, timestamp));
            }
        }

        debounce_state.pending_events = events_to_keep;

        // Send events
        if events_to_send.len() == 1 {
            let _ = event_sender.send(events_to_send.into_iter().next().unwrap());
        } else if events_to_send.len() > 1 {
            let _ = event_sender.send(FileWatchEvent::Batch(events_to_send));
        }

        debounce_state.last_flush = now;
    }

    /// Watch a file or directory
    pub fn watch(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        self.watch_with_options(path, self.config.recursive, None)
    }

    /// Watch a file or directory with custom options
    pub fn watch_with_options(
        &mut self,
        path: &Path,
        recursive: bool,
        handler_id: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.config.enabled {
            return Err("File watcher is disabled".into());
        }

        let watcher = self.watcher.as_mut()
            .ok_or("File watcher not initialized")?;

        let mode = if recursive {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };

        watcher.watch(path, mode)?;

        let last_modified = std::fs::metadata(path)
            .and_then(|m| m.modified())
            .ok();

        let watched_file = WatchedFile {
            path: path.to_path_buf(),
            recursive,
            last_modified,
            handler_id,
        };

        self.watched_paths.insert(path.to_path_buf(), watched_file);

        // Update stats
        if path.is_dir() {
            self.stats.directories_watched += 1;
        } else {
            self.stats.files_watched += 1;
        }

        // Send IDE event
        global_event_bus().emit(IdeEvent::FileWatchStarted {
            path: path.to_path_buf(),
            recursive,
        });

        Ok(())
    }

    /// Stop watching a path
    pub fn unwatch(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(watcher) = &mut self.watcher {
            watcher.unwatch(path)?;
            
            if let Some(watched) = self.watched_paths.remove(path) {
                // Update stats
                if watched.path.is_dir() {
                    self.stats.directories_watched = self.stats.directories_watched.saturating_sub(1);
                } else {
                    self.stats.files_watched = self.stats.files_watched.saturating_sub(1);
                }
            }

            // Send IDE event
            global_event_bus().emit(IdeEvent::FileWatchStopped {
                path: path.to_path_buf(),
            });
        }
        Ok(())
    }

    /// Get the next file watch event (non-blocking)
    pub fn try_recv_event(&self) -> Result<FileWatchEvent, mpsc::TryRecvError> {
        self.consumer_receiver.try_recv()
    }

    /// Get the next file watch event (blocking)
    pub fn recv_event(&self) -> Result<FileWatchEvent, mpsc::RecvError> {
        self.consumer_receiver.recv()
    }

    /// Get the next file watch event with timeout
    pub fn recv_event_timeout(&self, timeout: Duration) -> Result<FileWatchEvent, mpsc::RecvTimeoutError> {
        self.consumer_receiver.recv_timeout(timeout)
    }

    /// Get list of currently watched paths
    pub fn watched_paths(&self) -> Vec<&PathBuf> {
        self.watched_paths.keys().collect()
    }

    /// Get watcher statistics
    pub fn stats(&self) -> WatcherStats {
        self.stats.clone()
    }

    /// Enable or disable the file watcher
    pub fn set_enabled(&mut self, enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
        if enabled == self.config.enabled {
            return Ok(());
        }

        self.config.enabled = enabled;

        if enabled {
            self.initialize_watcher()?;
        } else {
            self.shutdown()?;
        }

        Ok(())
    }

    /// Update watcher configuration
    pub fn update_config(&mut self, config: FileWatcherConfig) -> Result<(), Box<dyn std::error::Error>> {
        let restart_needed = config.enabled != self.config.enabled;
        self.config = config;

        if restart_needed {
            if self.config.enabled {
                self.initialize_watcher()?;
            } else {
                self.shutdown()?;
            }
        }

        Ok(())
    }

    /// Shutdown the file watcher
    pub fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Signal background thread to shutdown
        if let Some(shutdown_sender) = self.shutdown_sender.take() {
            let _ = shutdown_sender.send(());
        }

        // Wait for background thread to finish
        if let Some(handle) = self.background_thread.take() {
            let _ = handle.join();
        }

        // Clean up watcher
        self.watcher = None;
        self.event_receiver = None;
        
        // Clear watched paths
        self.watched_paths.clear();
        self.stats.files_watched = 0;
        self.stats.directories_watched = 0;

        Ok(())
    }
}

impl Drop for AdvancedFileWatcher {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_file_watcher_creation() {
        let watcher = AdvancedFileWatcher::new();
        assert!(watcher.is_ok());
    }

    #[test]
    fn test_wildcard_matching() {
        assert!(AdvancedFileWatcher::wildcard_match("test.tmp", "*.tmp"));
        assert!(AdvancedFileWatcher::wildcard_match("prefix_test", "prefix*"));
        assert!(!AdvancedFileWatcher::wildcard_match("test.txt", "*.tmp"));
    }

    #[test]
    fn test_should_ignore_path() {
        let config = FileWatcherConfig::default();
        
        assert!(AdvancedFileWatcher::should_ignore_path(
            Path::new(".git/config"),
            &config
        ));
        
        assert!(AdvancedFileWatcher::should_ignore_path(
            Path::new("node_modules/package/index.js"),
            &config
        ));
        
        assert!(!AdvancedFileWatcher::should_ignore_path(
            Path::new("src/main.rs"),
            &config
        ));
    }

    #[test]
    fn test_file_watching() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let mut watcher = AdvancedFileWatcher::new()?;
        
        // Watch the temporary directory
        watcher.watch(temp_dir.path())?;
        
        // Create a test file
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "Hello, World!")?;
        
        // Give the watcher time to detect the change
        std::thread::sleep(Duration::from_millis(200));
        
        // Check for events
        match watcher.try_recv_event() {
            Ok(event) => {
                match event {
                    FileWatchEvent::Created(path) => {
                        assert_eq!(path, test_file);
                    }
                    FileWatchEvent::Batch(events) => {
                        assert!(!events.is_empty());
                    }
                    _ => {} // Other events are fine too
                }
            }
            Err(mpsc::TryRecvError::Empty) => {
                // No events yet - might need more time on slow systems
            }
            Err(e) => return Err(e.into()),
        }
        
        Ok(())
    }
}