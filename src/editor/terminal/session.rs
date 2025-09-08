//! # Terminal Session Management
//!
//! This module provides session management capabilities for terminals,
//! including session persistence, bookmarks, and session restoration.

use super::core::{TerminalId, ShellType, TerminalState};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Session manager for terminal sessions
#[derive(Debug, Clone)]
pub struct SessionManager {
    /// Active sessions
    pub sessions: HashMap<SessionId, TerminalSession>,
    /// Session templates
    pub templates: HashMap<String, SessionTemplate>,
    /// Auto-save settings
    pub auto_save: AutoSaveSettings,
    /// Session history
    pub session_history: Vec<SessionHistoryEntry>,
    /// Settings
    pub settings: SessionSettings,
}

/// Unique identifier for sessions
pub type SessionId = Uuid;

/// Terminal session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalSession {
    /// Session ID
    pub id: SessionId,
    /// Session name
    pub name: String,
    /// Terminal ID this session belongs to
    pub terminal_id: Option<TerminalId>,
    /// Session creation time
    pub created_at: SystemTime,
    /// Last activity time
    pub last_activity: SystemTime,
    /// Session duration
    pub duration: Duration,
    /// Working directory
    pub working_directory: PathBuf,
    /// Shell type used
    pub shell_type: ShellType,
    /// Environment variables at session start
    pub initial_environment: HashMap<String, String>,
    /// Command history
    pub command_history: Vec<SessionCommand>,
    /// Session state
    pub state: SessionState,
    /// Session tags for organization
    pub tags: Vec<String>,
    /// Session description
    pub description: Option<String>,
    /// Session bookmarks
    pub bookmarks: Vec<SessionBookmark>,
    /// Session statistics
    pub statistics: SessionStatistics,
}

/// Session command with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionCommand {
    /// Command text
    pub command: String,
    /// Command execution time
    pub executed_at: SystemTime,
    /// Command duration
    pub duration: Option<Duration>,
    /// Exit code
    pub exit_code: Option<i32>,
    /// Working directory when executed
    pub working_directory: PathBuf,
    /// Whether command was successful
    pub success: bool,
    /// Output length (for statistics)
    pub output_length: usize,
}

/// Session state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionState {
    /// Session is active
    Active,
    /// Session is paused
    Paused,
    /// Session has ended normally
    Ended,
    /// Session was terminated
    Terminated,
    /// Session crashed
    Crashed,
}

/// Session bookmark for important moments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionBookmark {
    /// Bookmark ID
    pub id: Uuid,
    /// Bookmark name
    pub name: String,
    /// Bookmark description
    pub description: Option<String>,
    /// Timestamp when bookmark was created
    pub timestamp: SystemTime,
    /// Command index at bookmark time
    pub command_index: usize,
    /// Working directory at bookmark time
    pub working_directory: PathBuf,
    /// Bookmark tags
    pub tags: Vec<String>,
    /// Whether bookmark is important
    pub important: bool,
}

/// Session statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionStatistics {
    /// Total commands executed
    pub commands_executed: u64,
    /// Successful commands
    pub commands_successful: u64,
    /// Failed commands
    pub commands_failed: u64,
    /// Total output characters
    pub total_output_chars: u64,
    /// Directories visited
    pub directories_visited: Vec<PathBuf>,
    /// Most used commands
    pub command_frequency: HashMap<String, u64>,
    /// Average command duration
    pub avg_command_duration: Duration,
    /// Peak activity periods
    pub activity_periods: Vec<ActivityPeriod>,
}

/// Period of high activity in a session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityPeriod {
    /// Period start time
    pub start: SystemTime,
    /// Period end time
    pub end: SystemTime,
    /// Commands executed in this period
    pub commands_count: u32,
    /// Activity intensity (commands per minute)
    pub intensity: f32,
}

/// Session template for creating pre-configured sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionTemplate {
    /// Template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Default working directory
    pub working_directory: Option<PathBuf>,
    /// Shell type to use
    pub shell_type: ShellType,
    /// Environment variables to set
    pub environment_variables: HashMap<String, String>,
    /// Initial commands to run
    pub startup_commands: Vec<String>,
    /// Template tags
    pub tags: Vec<String>,
    /// Auto-create bookmarks for certain events
    pub auto_bookmark_events: Vec<AutoBookmarkEvent>,
}

/// Events that can trigger automatic bookmarks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutoBookmarkEvent {
    /// Bookmark on command failure
    CommandFailure,
    /// Bookmark on long-running command completion
    LongCommandCompletion { min_duration_seconds: u64 },
    /// Bookmark on directory change
    DirectoryChange,
    /// Bookmark on specific command pattern
    CommandPattern { pattern: String },
    /// Bookmark on error output
    ErrorOutput,
}

/// Auto-save settings for sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoSaveSettings {
    /// Enable auto-save
    pub enabled: bool,
    /// Auto-save interval
    pub interval: Duration,
    /// Save location
    pub save_directory: PathBuf,
    /// Maximum sessions to keep
    pub max_saved_sessions: usize,
    /// Compress saved sessions
    pub compress: bool,
    /// Auto-save on session end
    pub save_on_session_end: bool,
}

/// Session history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionHistoryEntry {
    /// Session ID
    pub session_id: SessionId,
    /// Session name
    pub session_name: String,
    /// Session start time
    pub start_time: SystemTime,
    /// Session end time
    pub end_time: Option<SystemTime>,
    /// Final session state
    pub final_state: SessionState,
    /// Commands executed count
    pub commands_count: u64,
    /// Session duration
    pub duration: Duration,
    /// Working directory
    pub working_directory: PathBuf,
    /// Session file path (if saved)
    pub saved_file_path: Option<PathBuf>,
}

/// Session management settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSettings {
    /// Maximum session history entries
    pub max_history_entries: usize,
    /// Enable session statistics
    pub collect_statistics: bool,
    /// Auto-create sessions for new terminals
    pub auto_create_sessions: bool,
    /// Default session template
    pub default_template: Option<String>,
    /// Maximum command history per session
    pub max_command_history: usize,
    /// Enable session bookmarks
    pub enable_bookmarks: bool,
    /// Auto-bookmark important events
    pub auto_bookmark_events: Vec<AutoBookmarkEvent>,
}

/// Bookmark manager for organizing session bookmarks
#[derive(Debug, Clone)]
pub struct BookmarkManager {
    /// All bookmarks across sessions
    pub bookmarks: HashMap<Uuid, SessionBookmark>,
    /// Bookmark collections/folders
    pub collections: HashMap<String, BookmarkCollection>,
    /// Bookmark search index
    pub search_index: BookmarkSearchIndex,
    /// Settings
    pub settings: BookmarkSettings,
}

/// Collection of related bookmarks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkCollection {
    /// Collection name
    pub name: String,
    /// Collection description
    pub description: Option<String>,
    /// Bookmark IDs in this collection
    pub bookmark_ids: Vec<Uuid>,
    /// Collection tags
    pub tags: Vec<String>,
    /// Collection color for UI
    pub color: Option<String>,
    /// Creation time
    pub created_at: SystemTime,
}

/// Search index for bookmarks
#[derive(Debug, Clone)]
pub struct BookmarkSearchIndex {
    /// Index by tags
    pub by_tags: HashMap<String, Vec<Uuid>>,
    /// Index by text content
    pub by_content: HashMap<String, Vec<Uuid>>,
    /// Index by date ranges
    pub by_date: Vec<(SystemTime, SystemTime, Vec<Uuid>)>,
    /// Index by session
    pub by_session: HashMap<SessionId, Vec<Uuid>>,
}

/// Bookmark management settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkSettings {
    /// Maximum bookmarks per session
    pub max_bookmarks_per_session: usize,
    /// Enable bookmark search indexing
    pub enable_search_indexing: bool,
    /// Auto-delete old bookmarks
    pub auto_delete_old_bookmarks: bool,
    /// Days to keep bookmarks
    pub bookmark_retention_days: u32,
    /// Enable bookmark export/import
    pub enable_export_import: bool,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            templates: HashMap::new(),
            auto_save: AutoSaveSettings::default(),
            session_history: Vec::new(),
            settings: SessionSettings::default(),
        }
    }
    
    /// Create a new session
    pub fn create_session(&mut self, name: String, terminal_id: TerminalId) -> SessionId {
        let session_id = Uuid::new_v4();
        let session = TerminalSession {
            id: session_id,
            name,
            terminal_id: Some(terminal_id),
            created_at: SystemTime::now(),
            last_activity: SystemTime::now(),
            duration: Duration::from_secs(0),
            working_directory: std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")),
            shell_type: ShellType::Bash, // Default, should be set from terminal
            initial_environment: std::env::vars().collect(),
            command_history: Vec::new(),
            state: SessionState::Active,
            tags: Vec::new(),
            description: None,
            bookmarks: Vec::new(),
            statistics: SessionStatistics::default(),
        };
        
        self.sessions.insert(session_id, session);
        session_id
    }
    
    /// Create session from template
    pub fn create_session_from_template(&mut self, template_name: &str, terminal_id: TerminalId) -> Result<SessionId, String> {
        let template = self.templates.get(template_name)
            .ok_or_else(|| format!("Template '{}' not found", template_name))?;
        
        let session_id = self.create_session(template.name.clone(), terminal_id);
        
        if let Some(session) = self.sessions.get_mut(&session_id) {
            session.working_directory = template.working_directory.clone()
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")));
            session.shell_type = template.shell_type;
            session.tags = template.tags.clone();
            session.description = Some(template.description.clone());
            
            // Apply environment variables
            for (key, value) in &template.environment_variables {
                session.initial_environment.insert(key.clone(), value.clone());
            }
        }
        
        Ok(session_id)
    }
    
    /// End a session
    pub fn end_session(&mut self, session_id: SessionId, final_state: SessionState) -> Result<(), String> {
        if let Some(session) = self.sessions.get_mut(&session_id) {
            session.state = final_state;
            session.duration = session.created_at.elapsed().unwrap_or(Duration::from_secs(0));
            
            // Add to history
            let history_entry = SessionHistoryEntry {
                session_id,
                session_name: session.name.clone(),
                start_time: session.created_at,
                end_time: Some(SystemTime::now()),
                final_state,
                commands_count: session.command_history.len() as u64,
                duration: session.duration,
                working_directory: session.working_directory.clone(),
                saved_file_path: None, // TODO: Implement session saving
            };
            
            self.session_history.push(history_entry);
            
            // Trim history if needed
            if self.session_history.len() > self.settings.max_history_entries {
                self.session_history.remove(0);
            }
            
            // Auto-save if enabled
            if self.auto_save.enabled && self.auto_save.save_on_session_end {
                let _ = self.save_session(session_id);
            }
            
            Ok(())
        } else {
            Err("Session not found".to_string())
        }
    }
    
    /// Add command to session
    pub fn add_command_to_session(
        &mut self,
        session_id: SessionId,
        command: String,
        exit_code: Option<i32>,
        duration: Option<Duration>,
        output_length: usize,
    ) -> Result<(), String> {
        if let Some(session) = self.sessions.get_mut(&session_id) {
            let session_command = SessionCommand {
                command: command.clone(),
                executed_at: SystemTime::now(),
                duration,
                exit_code,
                working_directory: session.working_directory.clone(),
                success: exit_code.map(|code| code == 0).unwrap_or(false),
                output_length,
            };
            
            session.command_history.push(session_command);
            session.last_activity = SystemTime::now();
            
            // Update statistics
            session.statistics.commands_executed += 1;
            if exit_code.map(|code| code == 0).unwrap_or(false) {
                session.statistics.commands_successful += 1;
            } else {
                session.statistics.commands_failed += 1;
            }
            
            session.statistics.total_output_chars += output_length as u64;
            
            // Update command frequency
            *session.statistics.command_frequency.entry(command.clone()).or_insert(0) += 1;
            
            // Update average duration
            if let Some(dur) = duration {
                let total_duration = session.statistics.avg_command_duration.as_nanos() as u64 
                    * (session.statistics.commands_executed - 1)
                    + dur.as_nanos() as u64;
                session.statistics.avg_command_duration = Duration::from_nanos(
                    total_duration / session.statistics.commands_executed
                );
            }
            
            // Trim command history if needed
            if session.command_history.len() > self.settings.max_command_history {
                session.command_history.remove(0);
            }
            
            // Check for auto-bookmark events
            self.check_auto_bookmark_events(session_id, &command, exit_code, duration);
            
            Ok(())
        } else {
            Err("Session not found".to_string())
        }
    }
    
    /// Add bookmark to session
    pub fn add_bookmark(
        &mut self,
        session_id: SessionId,
        name: String,
        description: Option<String>,
        important: bool,
    ) -> Result<Uuid, String> {
        if let Some(session) = self.sessions.get_mut(&session_id) {
            let bookmark = SessionBookmark {
                id: Uuid::new_v4(),
                name,
                description,
                timestamp: SystemTime::now(),
                command_index: session.command_history.len(),
                working_directory: session.working_directory.clone(),
                tags: Vec::new(),
                important,
            };
            
            let bookmark_id = bookmark.id;
            session.bookmarks.push(bookmark);
            
            Ok(bookmark_id)
        } else {
            Err("Session not found".to_string())
        }
    }
    
    /// Get session by ID
    pub fn get_session(&self, session_id: SessionId) -> Option<&TerminalSession> {
        self.sessions.get(&session_id)
    }
    
    /// Get session statistics
    pub fn get_session_statistics(&self, session_id: SessionId) -> Option<&SessionStatistics> {
        self.sessions.get(&session_id).map(|session| &session.statistics)
    }
    
    /// Save session to disk
    pub fn save_session(&self, session_id: SessionId) -> Result<PathBuf, String> {
        if let Some(session) = self.sessions.get(&session_id) {
            let filename = format!("session_{}_{}.json", 
                session_id.as_simple(),
                session.created_at.duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or(Duration::from_secs(0))
                    .as_secs()
            );
            
            let file_path = self.auto_save.save_directory.join(filename);
            
            let serialized = serde_json::to_string_pretty(session)
                .map_err(|e| format!("Failed to serialize session: {}", e))?;
            
            std::fs::write(&file_path, serialized)
                .map_err(|e| format!("Failed to write session file: {}", e))?;
            
            Ok(file_path)
        } else {
            Err("Session not found".to_string())
        }
    }
    
    /// Load session from disk
    pub fn load_session(&mut self, file_path: &PathBuf) -> Result<SessionId, String> {
        let content = std::fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read session file: {}", e))?;
        
        let session: TerminalSession = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to deserialize session: {}", e))?;
        
        let session_id = session.id;
        self.sessions.insert(session_id, session);
        
        Ok(session_id)
    }
    
    /// Check for auto-bookmark events
    fn check_auto_bookmark_events(
        &mut self,
        session_id: SessionId,
        command: &str,
        exit_code: Option<i32>,
        duration: Option<Duration>,
    ) {
        for event in &self.settings.auto_bookmark_events.clone() {
            let should_bookmark = match event {
                AutoBookmarkEvent::CommandFailure => {
                    exit_code.map(|code| code != 0).unwrap_or(false)
                }
                AutoBookmarkEvent::LongCommandCompletion { min_duration_seconds } => {
                    duration.map(|d| d.as_secs() >= *min_duration_seconds).unwrap_or(false)
                }
                AutoBookmarkEvent::DirectoryChange => {
                    command.starts_with("cd ") || command == "cd"
                }
                AutoBookmarkEvent::CommandPattern { pattern } => {
                    command.contains(pattern)
                }
                AutoBookmarkEvent::ErrorOutput => {
                    // This would need to be checked against actual output
                    false
                }
            };
            
            if should_bookmark {
                let bookmark_name = format!("Auto: {}", self.get_bookmark_name_for_event(event, command));
                let _ = self.add_bookmark(session_id, bookmark_name, None, false);
            }
        }
    }
    
    /// Get bookmark name for auto-bookmark event
    fn get_bookmark_name_for_event(&self, event: &AutoBookmarkEvent, command: &str) -> String {
        match event {
            AutoBookmarkEvent::CommandFailure => format!("Command failed: {}", command),
            AutoBookmarkEvent::LongCommandCompletion { .. } => format!("Long command: {}", command),
            AutoBookmarkEvent::DirectoryChange => format!("Directory change: {}", command),
            AutoBookmarkEvent::CommandPattern { pattern } => format!("Pattern '{}' in: {}", pattern, command),
            AutoBookmarkEvent::ErrorOutput => format!("Error output: {}", command),
        }
    }
}

impl BookmarkManager {
    /// Create a new bookmark manager
    pub fn new() -> Self {
        Self {
            bookmarks: HashMap::new(),
            collections: HashMap::new(),
            search_index: BookmarkSearchIndex::new(),
            settings: BookmarkSettings::default(),
        }
    }
    
    /// Add bookmark to manager
    pub fn add_bookmark(&mut self, bookmark: SessionBookmark) {
        let bookmark_id = bookmark.id;
        
        // Update search index
        for tag in &bookmark.tags {
            self.search_index.by_tags.entry(tag.clone()).or_default().push(bookmark_id);
        }
        
        // Index by content
        let content_words: Vec<&str> = bookmark.name.split_whitespace().collect();
        for word in content_words {
            self.search_index.by_content.entry(word.to_lowercase()).or_default().push(bookmark_id);
        }
        
        self.bookmarks.insert(bookmark_id, bookmark);
    }
    
    /// Search bookmarks
    pub fn search_bookmarks(&self, query: &str) -> Vec<&SessionBookmark> {
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();
        
        // Search by name/description
        for bookmark in self.bookmarks.values() {
            if bookmark.name.to_lowercase().contains(&query_lower) {
                results.push(bookmark);
                continue;
            }
            
            if let Some(desc) = &bookmark.description {
                if desc.to_lowercase().contains(&query_lower) {
                    results.push(bookmark);
                }
            }
        }
        
        results
    }
    
    /// Create bookmark collection
    pub fn create_collection(&mut self, name: String, description: Option<String>) -> String {
        let collection = BookmarkCollection {
            name: name.clone(),
            description,
            bookmark_ids: Vec::new(),
            tags: Vec::new(),
            color: None,
            created_at: SystemTime::now(),
        };
        
        self.collections.insert(name.clone(), collection);
        name
    }
    
    /// Add bookmark to collection
    pub fn add_to_collection(&mut self, bookmark_id: Uuid, collection_name: &str) -> Result<(), String> {
        if let Some(collection) = self.collections.get_mut(collection_name) {
            if !collection.bookmark_ids.contains(&bookmark_id) {
                collection.bookmark_ids.push(bookmark_id);
            }
            Ok(())
        } else {
            Err("Collection not found".to_string())
        }
    }
}

impl BookmarkSearchIndex {
    fn new() -> Self {
        Self {
            by_tags: HashMap::new(),
            by_content: HashMap::new(),
            by_date: Vec::new(),
            by_session: HashMap::new(),
        }
    }
}

// Default implementations

impl Default for AutoSaveSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(300), // 5 minutes
            save_directory: std::env::temp_dir().join("ide_rs_sessions"),
            max_saved_sessions: 100,
            compress: true,
            save_on_session_end: true,
        }
    }
}

impl Default for SessionSettings {
    fn default() -> Self {
        Self {
            max_history_entries: 1000,
            collect_statistics: true,
            auto_create_sessions: true,
            default_template: None,
            max_command_history: 10000,
            enable_bookmarks: true,
            auto_bookmark_events: vec![
                AutoBookmarkEvent::CommandFailure,
                AutoBookmarkEvent::LongCommandCompletion { min_duration_seconds: 30 },
            ],
        }
    }
}

impl Default for BookmarkSettings {
    fn default() -> Self {
        Self {
            max_bookmarks_per_session: 50,
            enable_search_indexing: true,
            auto_delete_old_bookmarks: false,
            bookmark_retention_days: 365,
            enable_export_import: true,
        }
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for BookmarkManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let mut manager = SessionManager::new();
        let terminal_id = Uuid::new_v4();
        let session_id = manager.create_session("Test Session".to_string(), terminal_id);
        
        assert!(manager.sessions.contains_key(&session_id));
        let session = manager.sessions.get(&session_id).unwrap();
        assert_eq!(session.name, "Test Session");
        assert_eq!(session.terminal_id, Some(terminal_id));
        assert_eq!(session.state, SessionState::Active);
    }

    #[test]
    fn test_command_tracking() {
        let mut manager = SessionManager::new();
        let terminal_id = Uuid::new_v4();
        let session_id = manager.create_session("Test Session".to_string(), terminal_id);
        
        manager.add_command_to_session(
            session_id,
            "ls -la".to_string(),
            Some(0),
            Some(Duration::from_millis(100)),
            50,
        ).unwrap();
        
        let session = manager.sessions.get(&session_id).unwrap();
        assert_eq!(session.command_history.len(), 1);
        assert_eq!(session.statistics.commands_executed, 1);
        assert_eq!(session.statistics.commands_successful, 1);
    }

    #[test]
    fn test_bookmark_creation() {
        let mut manager = SessionManager::new();
        let terminal_id = Uuid::new_v4();
        let session_id = manager.create_session("Test Session".to_string(), terminal_id);
        
        let bookmark_id = manager.add_bookmark(
            session_id,
            "Important Point".to_string(),
            Some("This is an important bookmark".to_string()),
            true,
        ).unwrap();
        
        let session = manager.sessions.get(&session_id).unwrap();
        assert_eq!(session.bookmarks.len(), 1);
        assert_eq!(session.bookmarks[0].id, bookmark_id);
        assert!(session.bookmarks[0].important);
    }

    #[test]
    fn test_bookmark_manager() {
        let mut bookmark_manager = BookmarkManager::new();
        
        let bookmark = SessionBookmark {
            id: Uuid::new_v4(),
            name: "Test Bookmark".to_string(),
            description: Some("A test bookmark".to_string()),
            timestamp: SystemTime::now(),
            command_index: 0,
            working_directory: PathBuf::from("/tmp"),
            tags: vec!["test".to_string(), "important".to_string()],
            important: false,
        };
        
        bookmark_manager.add_bookmark(bookmark);
        
        let search_results = bookmark_manager.search_bookmarks("test");
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].name, "Test Bookmark");
    }

    #[test]
    fn test_session_template() {
        let template = SessionTemplate {
            name: "Development".to_string(),
            description: "Development environment".to_string(),
            working_directory: Some(PathBuf::from("/workspace")),
            shell_type: ShellType::Bash,
            environment_variables: {
                let mut env = HashMap::new();
                env.insert("NODE_ENV".to_string(), "development".to_string());
                env
            },
            startup_commands: vec!["echo 'Starting development session'".to_string()],
            tags: vec!["dev".to_string(), "project".to_string()],
            auto_bookmark_events: vec![AutoBookmarkEvent::CommandFailure],
        };
        
        assert_eq!(template.name, "Development");
        assert_eq!(template.shell_type, ShellType::Bash);
        assert!(template.environment_variables.contains_key("NODE_ENV"));
    }
}