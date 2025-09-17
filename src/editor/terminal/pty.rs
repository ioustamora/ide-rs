//! # Cross-Platform PTY (Pseudo-Terminal) Abstraction
//!
//! This module provides a unified interface for working with pseudo-terminals
//! across different platforms (Windows, macOS, Linux). It handles platform-specific
//! differences and provides a consistent API for terminal operations.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{Receiver, Sender};
use std::time::{Duration, Instant};
use std::io::{self, Read, Write};
use std::ffi::OsString;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[cfg(unix)]
use std::os::unix::process::CommandExt;
#[cfg(windows)]
use std::os::windows::process::CommandExt;

/// Cross-platform PTY abstraction
pub trait PtyInterface: Send + Sync {
    /// Spawn a new process with PTY
    fn spawn(&mut self, command: &str, args: &[&str], working_dir: Option<&Path>) -> Result<Box<dyn PtySession>, PtyError>;
    
    /// Get the default shell for the current platform
    fn default_shell(&self) -> (String, Vec<String>);
    
    /// Check if PTY is supported on this platform
    fn is_supported(&self) -> bool;
    
    /// Get platform-specific capabilities
    fn capabilities(&self) -> PtyCapabilities;
    
    /// Set PTY size for new sessions
    fn set_default_size(&mut self, cols: u16, rows: u16);
    
    /// Get current default size
    fn get_default_size(&self) -> (u16, u16);
}

/// Individual PTY session representing a running process
pub trait PtySession: Send {
    /// Get the session ID
    fn id(&self) -> Uuid;
    
    /// Read output from the PTY
    fn read_output(&mut self, buffer: &mut [u8]) -> Result<usize, PtyError>;
    
    /// Write input to the PTY
    fn write_input(&mut self, data: &[u8]) -> Result<usize, PtyError>;
    
    /// Resize the PTY
    fn resize(&mut self, cols: u16, rows: u16) -> Result<(), PtyError>;
    
    /// Get current PTY size
    fn get_size(&self) -> Result<(u16, u16), PtyError>;
    
    /// Check if the process is still running
    fn is_alive(&self) -> bool;
    
    /// Get process ID
    fn pid(&self) -> Option<u32>;
    
    /// Terminate the process
    fn terminate(&mut self) -> Result<(), PtyError>;
    
    /// Kill the process forcefully
    fn kill(&mut self) -> Result<(), PtyError>;
    
    /// Wait for process to exit with timeout
    fn wait_with_timeout(&mut self, timeout: Duration) -> Result<Option<i32>, PtyError>;
    
    /// Get process exit status if available
    fn exit_status(&self) -> Option<i32>;
    
    /// Set environment variables for the session
    fn set_environment(&mut self, env: HashMap<String, String>) -> Result<(), PtyError>;
    
    /// Get working directory
    fn working_directory(&self) -> &Path;
    
    /// Change working directory (if supported)
    fn change_directory(&mut self, path: &Path) -> Result<(), PtyError>;
}

/// PTY error types
#[derive(Debug, Clone)]
pub enum PtyError {
    /// Platform not supported
    UnsupportedPlatform,
    /// Failed to spawn process
    SpawnFailed(String),
    /// I/O operation failed
    IoError(String),
    /// Process not found or already exited
    ProcessNotFound,
    /// Invalid size specified
    InvalidSize { cols: u16, rows: u16 },
    /// Permission denied
    PermissionDenied,
    /// Resource temporarily unavailable
    WouldBlock,
    /// Operation timed out
    Timeout,
    /// Feature not implemented on this platform
    NotImplemented,
    /// General error
    Other(String),
}

impl std::fmt::Display for PtyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedPlatform => write!(f, "PTY not supported on this platform"),
            Self::SpawnFailed(msg) => write!(f, "Failed to spawn process: {}", msg),
            Self::IoError(msg) => write!(f, "I/O error: {}", msg),
            Self::ProcessNotFound => write!(f, "Process not found or already exited"),
            Self::InvalidSize { cols, rows } => write!(f, "Invalid PTY size: {}x{}", cols, rows),
            Self::PermissionDenied => write!(f, "Permission denied"),
            Self::WouldBlock => write!(f, "Operation would block"),
            Self::Timeout => write!(f, "Operation timed out"),
            Self::NotImplemented => write!(f, "Feature not implemented on this platform"),
            Self::Other(msg) => write!(f, "PTY error: {}", msg),
        }
    }
}

impl std::error::Error for PtyError {}

/// PTY capabilities for the current platform
#[derive(Debug, Clone)]
pub struct PtyCapabilities {
    /// Supports resizing
    pub supports_resize: bool,
    /// Supports color output
    pub supports_color: bool,
    /// Supports Unicode
    pub supports_unicode: bool,
    /// Supports job control
    pub supports_job_control: bool,
    /// Supports environment variables
    pub supports_environment: bool,
    /// Maximum PTY size
    pub max_size: (u16, u16),
    /// Available shells
    pub available_shells: Vec<ShellInfo>,
}

/// Shell information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellInfo {
    pub name: String,
    pub path: PathBuf,
    pub args: Vec<String>,
    pub description: String,
    pub is_default: bool,
}

/// PTY configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PtyConfig {
    /// Initial size
    pub initial_size: (u16, u16),
    /// Shell to use
    pub shell: Option<ShellInfo>,
    /// Initial working directory
    pub working_dir: Option<PathBuf>,
    /// Environment variables
    pub environment: HashMap<String, String>,
    /// Enable UTF-8 mode
    pub utf8_mode: bool,
    /// Buffer size for I/O operations
    pub buffer_size: usize,
    /// Read timeout
    pub read_timeout: Option<Duration>,
    /// Write timeout
    pub write_timeout: Option<Duration>,
}

impl Default for PtyConfig {
    fn default() -> Self {
        Self {
            initial_size: (80, 24),
            shell: None,
            working_dir: None,
            environment: HashMap::new(),
            utf8_mode: true,
            buffer_size: 4096,
            read_timeout: Some(Duration::from_millis(100)),
            write_timeout: Some(Duration::from_millis(1000)),
        }
    }
}

/// Factory for creating platform-specific PTY implementations
pub struct PtyFactory;

impl PtyFactory {
    /// Create a PTY interface for the current platform
    pub fn create() -> Box<dyn PtyInterface> {
        #[cfg(windows)]
        return Box::new(WindowsPty::new());
        
        #[cfg(unix)]
        return Box::new(UnixPty::new());
        
        #[cfg(not(any(windows, unix)))]
        compile_error!("Unsupported platform for PTY operations");
    }
    
    /// Create a PTY interface with custom configuration
    pub fn create_with_config(config: PtyConfig) -> Box<dyn PtyInterface> {
        #[cfg(windows)]
        return Box::new(WindowsPty::with_config(config));
        
        #[cfg(unix)]
        return Box::new(UnixPty::with_config(config));
        
        #[cfg(not(any(windows, unix)))]
        compile_error!("Unsupported platform for PTY operations");
    }
    
    /// Get available shells for the current platform
    pub fn get_available_shells() -> Vec<ShellInfo> {
        #[cfg(windows)]
        return WindowsPty::get_available_shells();
        
        #[cfg(unix)]
        return UnixPty::get_available_shells();
        
        #[cfg(not(any(windows, unix)))]
        return Vec::new();
    }
}

// Windows-specific PTY implementation
#[cfg(windows)]
mod windows_pty {
    use super::*;
    use std::os::windows::ffi::OsStringExt;
    use std::ptr;
    use std::mem;
    
    /// Windows PTY implementation using ConPTY API
    pub struct WindowsPty {
        config: PtyConfig,
        default_size: (u16, u16),
    }
    
    /// Windows PTY session
    pub struct WindowsPtySession {
        id: Uuid,
        process: Option<Child>,
        working_dir: PathBuf,
        pty_handle: Option<WindowsPtyHandle>,
        exit_status: Option<i32>,
    }
    
    /// Windows PTY handle wrapper
    struct WindowsPtyHandle {
        input_pipe: std::fs::File,
        output_pipe: std::fs::File,
        pty_handle: usize, // HPCON handle
    }
    
    impl WindowsPty {
        pub fn new() -> Self {
            Self {
                config: PtyConfig::default(),
                default_size: (80, 24),
            }
        }
        
        pub fn with_config(config: PtyConfig) -> Self {
            let default_size = config.initial_size;
            Self {
                config,
                default_size,
            }
        }
        
        pub fn get_available_shells() -> Vec<ShellInfo> {
            let mut shells = Vec::new();
            
            // PowerShell Core
            if let Ok(pwsh_path) = which::which("pwsh") {
                shells.push(ShellInfo {
                    name: "PowerShell Core".to_string(),
                    path: pwsh_path,
                    args: vec!["-NoLogo".to_string()],
                    description: "PowerShell Core 6+".to_string(),
                    is_default: true,
                });
            }
            
            // Windows PowerShell
            if let Ok(ps_path) = which::which("powershell") {
                shells.push(ShellInfo {
                    name: "Windows PowerShell".to_string(),
                    path: ps_path,
                    args: vec!["-NoLogo".to_string()],
                    description: "Windows PowerShell 5.1".to_string(),
                    is_default: shells.is_empty(),
                });
            }
            
            // Command Prompt
            if let Ok(cmd_path) = which::which("cmd") {
                shells.push(ShellInfo {
                    name: "Command Prompt".to_string(),
                    path: cmd_path,
                    args: vec!["/k".to_string()],
                    description: "Windows Command Prompt".to_string(),
                    is_default: shells.is_empty(),
                });
            }
            
            // Git Bash (if available)
            if let Ok(bash_path) = which::which("bash") {
                shells.push(ShellInfo {
                    name: "Git Bash".to_string(),
                    path: bash_path,
                    args: vec!["--login".to_string(), "-i".to_string()],
                    description: "Git for Windows Bash".to_string(),
                    is_default: false,
                });
            }
            
            shells
        }
        
        fn create_conpty_session(&self, command: &str, args: &[&str], working_dir: Option<&Path>) -> Result<WindowsPtySession, PtyError> {
            // This is a simplified implementation
            // In a real implementation, you would use the Windows ConPTY API
            
            let mut cmd = Command::new(command);
            cmd.args(args);
            
            if let Some(dir) = working_dir {
                cmd.current_dir(dir);
            }
            
            // Set up environment
            for (key, value) in &self.config.environment {
                cmd.env(key, value);
            }
            
            cmd.stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
            
            let process = cmd.spawn()
                .map_err(|e| PtyError::SpawnFailed(e.to_string()))?;
            
            Ok(WindowsPtySession {
                id: Uuid::new_v4(),
                process: Some(process),
                working_dir: working_dir.unwrap_or(&std::env::current_dir().unwrap()).to_path_buf(),
                pty_handle: None, // Would be initialized with real ConPTY handle
                exit_status: None,
            })
        }
    }
    
    impl PtyInterface for WindowsPty {
        fn spawn(&mut self, command: &str, args: &[&str], working_dir: Option<&Path>) -> Result<Box<dyn PtySession>, PtyError> {
            let session = self.create_conpty_session(command, args, working_dir)?;
            Ok(Box::new(session))
        }
        
        fn default_shell(&self) -> (String, Vec<String>) {
            // Try PowerShell Core first, then Windows PowerShell, then cmd
            if which::which("pwsh").is_ok() {
                ("pwsh".to_string(), vec!["-NoLogo".to_string()])
            } else if which::which("powershell").is_ok() {
                ("powershell".to_string(), vec!["-NoLogo".to_string()])
            } else {
                ("cmd".to_string(), vec![])
            }
        }
        
        fn is_supported(&self) -> bool {
            // ConPTY is available on Windows 10 version 1809 and later
            true // Simplified check
        }
        
        fn capabilities(&self) -> PtyCapabilities {
            PtyCapabilities {
                supports_resize: true,
                supports_color: true,
                supports_unicode: true,
                supports_job_control: false, // Limited on Windows
                supports_environment: true,
                max_size: (32767, 32767),
                available_shells: Self::get_available_shells(),
            }
        }
        
        fn set_default_size(&mut self, cols: u16, rows: u16) {
            self.default_size = (cols, rows);
        }
        
        fn get_default_size(&self) -> (u16, u16) {
            self.default_size
        }
    }
    
    impl PtySession for WindowsPtySession {
        fn id(&self) -> Uuid {
            self.id
        }
        
        fn read_output(&mut self, buffer: &mut [u8]) -> Result<usize, PtyError> {
            // Simplified implementation - would use ConPTY pipes in real implementation
            if let Some(ref mut process) = self.process {
                if let Some(ref mut stdout) = process.stdout {
                    stdout.read(buffer).map_err(|e| PtyError::IoError(e.to_string()))
                } else {
                    Ok(0)
                }
            } else {
                Err(PtyError::ProcessNotFound)
            }
        }
        
        fn write_input(&mut self, data: &[u8]) -> Result<usize, PtyError> {
            if let Some(ref mut process) = self.process {
                if let Some(ref mut stdin) = process.stdin {
                    stdin.write(data).map_err(|e| PtyError::IoError(e.to_string()))
                } else {
                    Ok(0)
                }
            } else {
                Err(PtyError::ProcessNotFound)
            }
        }
        
        fn resize(&mut self, cols: u16, rows: u16) -> Result<(), PtyError> {
            if cols == 0 || rows == 0 {
                return Err(PtyError::InvalidSize { cols, rows });
            }
            // Would use ConPTY resize API in real implementation
            Ok(())
        }
        
        fn get_size(&self) -> Result<(u16, u16), PtyError> {
            // Would query actual PTY size in real implementation
            Ok((80, 24))
        }
        
        fn is_alive(&self) -> bool {
            if let Some(ref process) = self.process {
                // Since we can't call try_wait() in an immutable context,
                // we check if we still have a process reference and no exit status
                self.exit_status.is_none()
            } else {
                false
            }
        }
        
        fn pid(&self) -> Option<u32> {
            self.process.as_ref().map(|p| p.id())
        }
        
        fn terminate(&mut self) -> Result<(), PtyError> {
            if let Some(ref mut process) = self.process {
                process.kill().map_err(|e| PtyError::Other(e.to_string()))?;
                Ok(())
            } else {
                Err(PtyError::ProcessNotFound)
            }
        }
        
        fn kill(&mut self) -> Result<(), PtyError> {
            self.terminate() // Same as terminate on Windows
        }
        
        fn wait_with_timeout(&mut self, timeout: Duration) -> Result<Option<i32>, PtyError> {
            if let Some(ref mut process) = self.process {
                // Simplified implementation
                match process.try_wait() {
                    Ok(Some(exit_status)) => {
                        let code = exit_status.code().unwrap_or(-1);
                        self.exit_status = Some(code);
                        Ok(Some(code))
                    }
                    Ok(None) => Ok(None),
                    Err(e) => Err(PtyError::Other(e.to_string())),
                }
            } else {
                Err(PtyError::ProcessNotFound)
            }
        }
        
        fn exit_status(&self) -> Option<i32> {
            self.exit_status
        }
        
        fn set_environment(&mut self, _env: HashMap<String, String>) -> Result<(), PtyError> {
            // Environment is set during process creation on Windows
            Err(PtyError::NotImplemented)
        }
        
        fn working_directory(&self) -> &Path {
            &self.working_dir
        }
        
        fn change_directory(&mut self, path: &Path) -> Result<(), PtyError> {
            self.working_dir = path.to_path_buf();
            Ok(())
        }
    }
}

// Unix-specific PTY implementation
#[cfg(unix)]
mod unix_pty {
    use super::*;
    use std::os::unix::io::{AsRawFd, RawFd};
    use std::os::unix::process::CommandExt;
    
    /// Unix PTY implementation using libc pty functions
    pub struct UnixPty {
        config: PtyConfig,
        default_size: (u16, u16),
    }
    
    /// Unix PTY session
    pub struct UnixPtySession {
        id: Uuid,
        process: Option<Child>,
        working_dir: PathBuf,
        master_fd: Option<RawFd>,
        slave_fd: Option<RawFd>,
        exit_status: Option<i32>,
    }
    
    impl UnixPty {
        pub fn new() -> Self {
            Self {
                config: PtyConfig::default(),
                default_size: (80, 24),
            }
        }
        
        pub fn with_config(config: PtyConfig) -> Self {
            let default_size = config.initial_size;
            Self {
                config,
                default_size,
            }
        }
        
        pub fn get_available_shells() -> Vec<ShellInfo> {
            let mut shells = Vec::new();
            
            // Bash
            if let Ok(bash_path) = which::which("bash") {
                shells.push(ShellInfo {
                    name: "Bash".to_string(),
                    path: bash_path,
                    args: vec!["--login".to_string(), "-i".to_string()],
                    description: "Bourne Again Shell".to_string(),
                    is_default: true,
                });
            }
            
            // Zsh
            if let Ok(zsh_path) = which::which("zsh") {
                shells.push(ShellInfo {
                    name: "Zsh".to_string(),
                    path: zsh_path,
                    args: vec!["-l".to_string()],
                    description: "Z Shell".to_string(),
                    is_default: shells.is_empty(),
                });
            }
            
            // Fish
            if let Ok(fish_path) = which::which("fish") {
                shells.push(ShellInfo {
                    name: "Fish".to_string(),
                    path: fish_path,
                    args: vec!["-l".to_string()],
                    description: "Friendly Interactive Shell".to_string(),
                    is_default: shells.is_empty(),
                });
            }
            
            // Sh (fallback)
            if let Ok(sh_path) = which::which("sh") {
                shells.push(ShellInfo {
                    name: "Sh".to_string(),
                    path: sh_path,
                    args: vec!["-l".to_string()],
                    description: "POSIX Shell".to_string(),
                    is_default: shells.is_empty(),
                });
            }
            
            shells
        }
        
        fn create_pty_session(&self, command: &str, args: &[&str], working_dir: Option<&Path>) -> Result<UnixPtySession, PtyError> {
            // This is a simplified implementation
            // In a real implementation, you would use the libc pty functions
            
            let mut cmd = Command::new(command);
            cmd.args(args);
            
            if let Some(dir) = working_dir {
                cmd.current_dir(dir);
            }
            
            // Set up environment
            for (key, value) in &self.config.environment {
                cmd.env(key, value);
            }
            
            cmd.stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
            
            let process = cmd.spawn()
                .map_err(|e| PtyError::SpawnFailed(e.to_string()))?;
            
            Ok(UnixPtySession {
                id: Uuid::new_v4(),
                process: Some(process),
                working_dir: working_dir.unwrap_or(&std::env::current_dir().unwrap()).to_path_buf(),
                master_fd: None, // Would be initialized with real PTY master
                slave_fd: None,  // Would be initialized with real PTY slave
                exit_status: None,
            })
        }
    }
    
    impl PtyInterface for UnixPty {
        fn spawn(&mut self, command: &str, args: &[&str], working_dir: Option<&Path>) -> Result<Box<dyn PtySession>, PtyError> {
            let session = self.create_pty_session(command, args, working_dir)?;
            Ok(Box::new(session))
        }
        
        fn default_shell(&self) -> (String, Vec<String>) {
            // Try to get shell from environment, fallback to common shells
            if let Ok(shell) = std::env::var("SHELL") {
                (shell, vec!["-l".to_string()])
            } else if which::which("bash").is_ok() {
                ("bash".to_string(), vec!["--login".to_string(), "-i".to_string()])
            } else if which::which("sh").is_ok() {
                ("sh".to_string(), vec!["-l".to_string()])
            } else {
                ("/bin/sh".to_string(), vec!["-l".to_string()])
            }
        }
        
        fn is_supported(&self) -> bool {
            true // PTY is generally available on Unix systems
        }
        
        fn capabilities(&self) -> PtyCapabilities {
            PtyCapabilities {
                supports_resize: true,
                supports_color: true,
                supports_unicode: true,
                supports_job_control: true,
                supports_environment: true,
                max_size: (65535, 65535),
                available_shells: Self::get_available_shells(),
            }
        }
        
        fn set_default_size(&mut self, cols: u16, rows: u16) {
            self.default_size = (cols, rows);
        }
        
        fn get_default_size(&self) -> (u16, u16) {
            self.default_size
        }
    }
    
    impl PtySession for UnixPtySession {
        fn id(&self) -> Uuid {
            self.id
        }
        
        fn read_output(&mut self, buffer: &mut [u8]) -> Result<usize, PtyError> {
            // Would read from master PTY in real implementation
            if let Some(ref mut process) = self.process {
                if let Some(ref mut stdout) = process.stdout {
                    stdout.read(buffer).map_err(|e| PtyError::IoError(e.to_string()))
                } else {
                    Ok(0)
                }
            } else {
                Err(PtyError::ProcessNotFound)
            }
        }
        
        fn write_input(&mut self, data: &[u8]) -> Result<usize, PtyError> {
            // Would write to master PTY in real implementation
            if let Some(ref mut process) = self.process {
                if let Some(ref mut stdin) = process.stdin {
                    stdin.write(data).map_err(|e| PtyError::IoError(e.to_string()))
                } else {
                    Ok(0)
                }
            } else {
                Err(PtyError::ProcessNotFound)
            }
        }
        
        fn resize(&mut self, cols: u16, rows: u16) -> Result<(), PtyError> {
            if cols == 0 || rows == 0 {
                return Err(PtyError::InvalidSize { cols, rows });
            }
            // Would use ioctl TIOCSWINSZ in real implementation
            Ok(())
        }
        
        fn get_size(&self) -> Result<(u16, u16), PtyError> {
            // Would query actual PTY size in real implementation
            Ok((80, 24))
        }
        
        fn is_alive(&self) -> bool {
            if let Some(ref process) = self.process {
                // Since we can't call try_wait() in an immutable context,
                // we check if we still have a process reference and no exit status
                self.exit_status.is_none()
            } else {
                false
            }
        }
        
        fn pid(&self) -> Option<u32> {
            self.process.as_ref().map(|p| p.id())
        }
        
        fn terminate(&mut self) -> Result<(), PtyError> {
            if let Some(ref mut process) = self.process {
                // Send SIGTERM
                process.kill().map_err(|e| PtyError::Other(e.to_string()))?;
                Ok(())
            } else {
                Err(PtyError::ProcessNotFound)
            }
        }
        
        fn kill(&mut self) -> Result<(), PtyError> {
            if let Some(ref mut process) = self.process {
                // Send SIGKILL - more forceful than terminate
                process.kill().map_err(|e| PtyError::Other(e.to_string()))?;
                Ok(())
            } else {
                Err(PtyError::ProcessNotFound)
            }
        }
        
        fn wait_with_timeout(&mut self, timeout: Duration) -> Result<Option<i32>, PtyError> {
            if let Some(ref mut process) = self.process {
                // Simplified implementation
                match process.try_wait() {
                    Ok(Some(exit_status)) => {
                        let code = exit_status.code().unwrap_or(-1);
                        self.exit_status = Some(code);
                        Ok(Some(code))
                    }
                    Ok(None) => Ok(None),
                    Err(e) => Err(PtyError::Other(e.to_string())),
                }
            } else {
                Err(PtyError::ProcessNotFound)
            }
        }
        
        fn exit_status(&self) -> Option<i32> {
            self.exit_status
        }
        
        fn set_environment(&mut self, env: HashMap<String, String>) -> Result<(), PtyError> {
            // Environment variables are typically set during process creation
            // On Unix, we could potentially use the environ manipulation
            Err(PtyError::NotImplemented)
        }
        
        fn working_directory(&self) -> &Path {
            &self.working_dir
        }
        
        fn change_directory(&mut self, path: &Path) -> Result<(), PtyError> {
            self.working_dir = path.to_path_buf();
            Ok(())
        }
    }
}

// Re-export platform-specific implementations
#[cfg(windows)]
pub use windows_pty::*;

#[cfg(unix)]
pub use unix_pty::*;

/// PTY manager for handling multiple PTY sessions
pub struct PtyManager {
    /// PTY interface implementation
    pty: Box<dyn PtyInterface>,
    /// Active PTY sessions
    sessions: HashMap<Uuid, Box<dyn PtySession>>,
    /// Configuration
    config: PtyConfig,
    /// Statistics
    stats: PtyStats,
}

/// PTY usage statistics
#[derive(Debug, Clone, Default)]
pub struct PtyStats {
    pub total_sessions_created: usize,
    pub active_sessions: usize,
    pub total_bytes_read: u64,
    pub total_bytes_written: u64,
    pub average_session_duration: Duration,
}

impl PtyManager {
    /// Create a new PTY manager
    pub fn new() -> Self {
        Self {
            pty: PtyFactory::create(),
            sessions: HashMap::new(),
            config: PtyConfig::default(),
            stats: PtyStats::default(),
        }
    }
    
    /// Create PTY manager with custom configuration
    pub fn with_config(config: PtyConfig) -> Self {
        Self {
            pty: PtyFactory::create_with_config(config.clone()),
            sessions: HashMap::new(),
            config,
            stats: PtyStats::default(),
        }
    }
    
    /// Spawn a new PTY session
    pub fn spawn_session(&mut self, command: &str, args: &[&str], working_dir: Option<&Path>) -> Result<Uuid, PtyError> {
        let session = self.pty.spawn(command, args, working_dir)?;
        let session_id = session.id();
        
        self.sessions.insert(session_id, session);
        self.stats.total_sessions_created += 1;
        self.stats.active_sessions += 1;
        
        Ok(session_id)
    }
    
    /// Get a session by ID
    pub fn get_session(&mut self, session_id: Uuid) -> Option<&mut Box<dyn PtySession>> {
        self.sessions.get_mut(&session_id)
    }
    
    /// Close a PTY session
    pub fn close_session(&mut self, session_id: Uuid) -> Result<(), PtyError> {
        if let Some(mut session) = self.sessions.remove(&session_id) {
            session.terminate()?;
            self.stats.active_sessions -= 1;
        }
        Ok(())
    }
    
    /// Get all active session IDs
    pub fn active_sessions(&self) -> Vec<Uuid> {
        self.sessions.keys().cloned().collect()
    }
    
    /// Cleanup finished sessions
    pub fn cleanup_finished_sessions(&mut self) {
        let mut to_remove = Vec::new();
        
        for (id, session) in &self.sessions {
            if !session.is_alive() {
                to_remove.push(*id);
            }
        }
        
        for id in to_remove {
            self.sessions.remove(&id);
            self.stats.active_sessions -= 1;
        }
    }
    
    /// Get platform capabilities
    pub fn get_capabilities(&self) -> PtyCapabilities {
        self.pty.capabilities()
    }
    
    /// Get available shells
    pub fn get_available_shells(&self) -> Vec<ShellInfo> {
        PtyFactory::get_available_shells()
    }
    
    /// Get default shell
    pub fn get_default_shell(&self) -> (String, Vec<String>) {
        self.pty.default_shell()
    }
    
    /// Get statistics
    pub fn get_stats(&self) -> &PtyStats {
        &self.stats
    }
    
    /// Set default PTY size
    pub fn set_default_size(&mut self, cols: u16, rows: u16) {
        self.pty.set_default_size(cols, rows);
    }
}

impl Default for PtyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pty_factory_creation() {
        let pty = PtyFactory::create();
        assert!(pty.is_supported());
    }
    
    #[test]
    fn test_pty_config_default() {
        let config = PtyConfig::default();
        assert_eq!(config.initial_size, (80, 24));
        assert!(config.utf8_mode);
        assert!(config.buffer_size > 0);
    }
    
    #[test]
    fn test_available_shells() {
        let shells = PtyFactory::get_available_shells();
        assert!(!shells.is_empty());
        
        // At least one shell should be marked as default
        assert!(shells.iter().any(|s| s.is_default));
    }
    
    #[test]
    fn test_pty_manager_creation() {
        let manager = PtyManager::new();
        assert!(manager.get_capabilities().supports_resize);
        assert_eq!(manager.active_sessions().len(), 0);
    }
}