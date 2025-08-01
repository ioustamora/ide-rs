//! Build System for RAD IDE
//!
//! This module provides comprehensive build and execution capabilities
//! including compilation, testing, and debugging support.

use std::process::{Command, Stdio};
use std::path::{Path, PathBuf};
use std::io::{BufRead, BufReader};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use anyhow::{Result, Context};
use crate::editor::output_panel::OutputPanel;

/// Build configuration options
#[derive(Clone, Debug)]
pub struct BuildConfig {
    pub profile: BuildProfile,
    pub features: Vec<String>,
    pub target: Option<String>,
    pub verbose: bool,
    pub offline: bool,
    pub frozen: bool,
}

/// Build profiles
#[derive(Clone, Debug, PartialEq)]
pub enum BuildProfile {
    Debug,
    Release,
    Test,
    Bench,
}

/// Build result information
#[derive(Debug)]
pub struct BuildResult {
    pub success: bool,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub warnings: Vec<CompilerMessage>,
    pub errors: Vec<CompilerMessage>,
    pub build_time: std::time::Duration,
}

/// Compiler diagnostic message
#[derive(Debug, Clone)]
pub struct CompilerMessage {
    pub level: MessageLevel,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub code: Option<String>,
}

/// Message severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum MessageLevel {
    Error,
    Warning,
    Info,
    Note,
    Help,
}

/// Build system manager
pub struct BuildSystem {
    project_path: Option<PathBuf>,
    config: BuildConfig,
    build_sender: Option<Sender<BuildCommand>>,
    output_receiver: Option<Receiver<BuildOutput>>,
}

/// Build commands
#[derive(Debug, Clone)]
pub enum BuildCommand {
    Build,
    Run,
    Test,
    Clean,
    Doc,
    Check,
    Clippy,
    Format,
    Stop,
}

/// Build output messages
#[derive(Debug)]
pub enum BuildOutput {
    Started(BuildCommand),
    Progress(String),
    Finished(BuildResult),
    Error(String),
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            profile: BuildProfile::Debug,
            features: Vec::new(),
            target: None,
            verbose: false,
            offline: false,
            frozen: false,
        }
    }
}

impl BuildSystem {
    pub fn new() -> Self {
        Self {
            project_path: None,
            config: BuildConfig::default(),
            build_sender: None,
            output_receiver: None,
        }
    }

    /// Set the project path for build operations
    pub fn set_project_path(&mut self, path: PathBuf) {
        self.project_path = Some(path);
    }

    /// Update build configuration
    pub fn set_config(&mut self, config: BuildConfig) {
        self.config = config;
    }

    /// Initialize the build system with async execution
    pub fn initialize(&mut self) -> Result<()> {
        let (build_tx, build_rx) = mpsc::channel();
        let (output_tx, output_rx) = mpsc::channel();

        let project_path = self.project_path.clone();
        let config = self.config.clone();

        // Spawn build worker thread
        thread::spawn(move || {
            let mut worker = BuildWorker::new(project_path, config, output_tx);
            worker.run(build_rx);
        });

        self.build_sender = Some(build_tx);
        self.output_receiver = Some(output_rx);

        Ok(())
    }

    /// Execute a build command
    pub fn execute_command(&self, command: BuildCommand) -> Result<()> {
        if let Some(sender) = &self.build_sender {
            sender.send(command)
                .context("Failed to send build command")?;
        }
        Ok(())
    }

    /// Check for build output messages
    pub fn poll_output(&self) -> Vec<BuildOutput> {
        let mut outputs = Vec::new();
        if let Some(receiver) = &self.output_receiver {
            while let Ok(output) = receiver.try_recv() {
                outputs.push(output);
            }
        }
        outputs
    }

    /// Build the project
    pub fn build(&self) -> Result<()> {
        self.execute_command(BuildCommand::Build)
    }

    /// Run the project
    pub fn run(&self) -> Result<()> {
        self.execute_command(BuildCommand::Run)
    }

    /// Run tests
    pub fn test(&self) -> Result<()> {
        self.execute_command(BuildCommand::Test)
    }

    /// Check code without building
    pub fn check(&self) -> Result<()> {
        self.execute_command(BuildCommand::Check)
    }

    /// Run clippy linter
    pub fn clippy(&self) -> Result<()> {
        self.execute_command(BuildCommand::Clippy)
    }

    /// Format code with rustfmt
    pub fn format(&self) -> Result<()> {
        self.execute_command(BuildCommand::Format)
    }

    /// Clean build artifacts
    pub fn clean(&self) -> Result<()> {
        self.execute_command(BuildCommand::Clean)
    }

    /// Generate documentation
    pub fn doc(&self) -> Result<()> {
        self.execute_command(BuildCommand::Doc)
    }

    /// Stop current build
    pub fn stop(&self) -> Result<()> {
        self.execute_command(BuildCommand::Stop)
    }

    /// Get current project path
    pub fn get_project_path(&self) -> Option<&Path> {
        self.project_path.as_deref()
    }
}

/// Build worker that runs in a separate thread
struct BuildWorker {
    project_path: Option<PathBuf>,
    config: BuildConfig,
    output_sender: Sender<BuildOutput>,
    current_process: Option<std::process::Child>,
}

impl BuildWorker {
    fn new(project_path: Option<PathBuf>, config: BuildConfig, output_sender: Sender<BuildOutput>) -> Self {
        Self {
            project_path,
            config,
            output_sender,
            current_process: None,
        }
    }

    fn run(&mut self, command_receiver: Receiver<BuildCommand>) {
        while let Ok(command) = command_receiver.recv() {
            match command {
                BuildCommand::Stop => {
                    self.stop_current_process();
                }
                _ => {
                    self.execute_build_command(command);
                }
            }
        }
    }

    fn execute_build_command(&mut self, command: BuildCommand) {
        let _ = self.output_sender.send(BuildOutput::Started(command.clone()));

        let result = match command {
            BuildCommand::Build => self.run_cargo_build(),
            BuildCommand::Run => self.run_cargo_run(),
            BuildCommand::Test => self.run_cargo_test(),
            BuildCommand::Check => self.run_cargo_check(),
            BuildCommand::Clippy => self.run_cargo_clippy(),
            BuildCommand::Format => self.run_cargo_fmt(),
            BuildCommand::Clean => self.run_cargo_clean(),
            BuildCommand::Doc => self.run_cargo_doc(),
            BuildCommand::Stop => return, // Handled separately
        };

        match result {
            Ok(build_result) => {
                let _ = self.output_sender.send(BuildOutput::Finished(build_result));
            }
            Err(error) => {
                let _ = self.output_sender.send(BuildOutput::Error(error.to_string()));
            }
        }
    }

    fn stop_current_process(&mut self) {
        if let Some(mut process) = self.current_process.take() {
            let _ = process.kill();
            let _ = process.wait();
        }
    }

    fn run_cargo_build(&mut self) -> Result<BuildResult> {
        let mut args = vec!["build"];
        
        match self.config.profile {
            BuildProfile::Release => args.push("--release"),
            BuildProfile::Debug => {}, // Default
            BuildProfile::Test => args.push("--tests"),
            BuildProfile::Bench => args.push("--benches"),
        }

        if self.config.verbose {
            args.push("--verbose");
        }

        let features_string;
        if !self.config.features.is_empty() {
            args.push("--features");
            features_string = self.config.features.join(",");
            args.push(&features_string);
        }

        let target_string;
        if let Some(target) = &self.config.target {
            args.push("--target");
            target_string = target.clone();
            args.push(&target_string);
        }

        self.run_cargo_command(&args)
    }

    fn run_cargo_run(&mut self) -> Result<BuildResult> {
        let mut args = vec!["run"];
        
        if matches!(self.config.profile, BuildProfile::Release) {
            args.push("--release");
        }

        if self.config.verbose {
            args.push("--verbose");
        }

        self.run_cargo_command(&args)
    }

    fn run_cargo_test(&mut self) -> Result<BuildResult> {
        let args = if self.config.verbose {
            vec!["test", "--verbose"]
        } else {
            vec!["test"]
        };

        self.run_cargo_command(&args)
    }

    fn run_cargo_check(&mut self) -> Result<BuildResult> {
        let args = if self.config.verbose {
            vec!["check", "--verbose"]
        } else {
            vec!["check"]
        };

        self.run_cargo_command(&args)
    }

    fn run_cargo_clippy(&mut self) -> Result<BuildResult> {
        let args = vec!["clippy", "--", "-D", "warnings"];
        self.run_cargo_command(&args)
    }

    fn run_cargo_fmt(&mut self) -> Result<BuildResult> {
        let args = vec!["fmt", "--all"];
        self.run_cargo_command(&args)
    }

    fn run_cargo_clean(&mut self) -> Result<BuildResult> {
        let args = vec!["clean"];
        self.run_cargo_command(&args)
    }

    fn run_cargo_doc(&mut self) -> Result<BuildResult> {
        let args = vec!["doc", "--open"];
        self.run_cargo_command(&args)
    }

    fn run_cargo_command(&mut self, args: &[&str]) -> Result<BuildResult> {
        let project_path = self.project_path.as_ref()
            .ok_or_else(|| anyhow::anyhow!("No project path set"))?;

        let start_time = std::time::Instant::now();
        
        let mut command = Command::new("cargo");
        command
            .args(args)
            .current_dir(project_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut process = command.spawn()
            .context("Failed to start cargo command")?;

        // Read output in real-time
        let stdout = process.stdout.take().unwrap();
        let stderr = process.stderr.take().unwrap();

        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);

        let mut stdout_lines = Vec::new();
        let mut stderr_lines = Vec::new();

        // Read stdout
        for line in stdout_reader.lines() {
            let line = line?;
            stdout_lines.push(line.clone());
            let _ = self.output_sender.send(BuildOutput::Progress(line));
        }

        // Read stderr
        for line in stderr_reader.lines() {
            let line = line?;
            stderr_lines.push(line.clone());
            let _ = self.output_sender.send(BuildOutput::Progress(line));
        }

        let output = process.wait()?;
        let build_time = start_time.elapsed();

        let stdout_text = stdout_lines.join("\n");
        let stderr_text = stderr_lines.join("\n");

        // Parse compiler messages
        let (warnings, errors) = self.parse_compiler_messages(&stderr_text);

        Ok(BuildResult {
            success: output.success(),
            exit_code: output.code().unwrap_or(-1),
            stdout: stdout_text,
            stderr: stderr_text,
            warnings,
            errors,
            build_time,
        })
    }

    fn parse_compiler_messages(&self, stderr: &str) -> (Vec<CompilerMessage>, Vec<CompilerMessage>) {
        let mut warnings = Vec::new();
        let mut errors = Vec::new();

        for line in stderr.lines() {
            if let Some(message) = self.parse_compiler_line(line) {
                match message.level {
                    MessageLevel::Error => errors.push(message),
                    MessageLevel::Warning => warnings.push(message),
                    _ => {} // Ignore other levels for now
                }
            }
        }

        (warnings, errors)
    }

    fn parse_compiler_line(&self, line: &str) -> Option<CompilerMessage> {
        // Simple parsing - in a real implementation, this would be more sophisticated
        if line.contains("error:") {
            Some(CompilerMessage {
                level: MessageLevel::Error,
                message: line.to_string(),
                file: self.extract_file_from_line(line),
                line: self.extract_line_number_from_line(line),
                column: None,
                code: None,
            })
        } else if line.contains("warning:") {
            Some(CompilerMessage {
                level: MessageLevel::Warning,
                message: line.to_string(),
                file: self.extract_file_from_line(line),
                line: self.extract_line_number_from_line(line),
                column: None,
                code: None,
            })
        } else {
            None
        }
    }

    fn extract_file_from_line(&self, line: &str) -> Option<String> {
        // Simple extraction - look for src/filename.rs pattern
        if let Some(start) = line.find("src/") {
            if let Some(end) = line[start..].find(':') {
                return Some(line[start..start + end].to_string());
            }
        }
        None
    }

    fn extract_line_number_from_line(&self, line: &str) -> Option<usize> {
        // Look for :number: pattern
        for part in line.split(':') {
            if let Ok(num) = part.trim().parse::<usize>() {
                return Some(num);
            }
        }
        None
    }
}

/// Extension trait for OutputPanel to handle build output
pub trait BuildOutputHandler {
    fn handle_build_output(&mut self, output: BuildOutput);
    fn display_build_result(&mut self, result: &BuildResult);
}

impl BuildOutputHandler for OutputPanel {
    fn handle_build_output(&mut self, output: BuildOutput) {
        match output {
            BuildOutput::Started(command) => {
                self.log(&format!("🚀 Starting {:?}...", command));
            }
            BuildOutput::Progress(line) => {
                self.log(&line);
            }
            BuildOutput::Finished(result) => {
                self.display_build_result(&result);
            }
            BuildOutput::Error(error) => {
                self.log(&format!("❌ Build error: {}", error));
            }
        }
    }

    fn display_build_result(&mut self, result: &BuildResult) {
        if result.success {
            self.log("✅ Build completed successfully!");
        } else {
            self.log(&format!("❌ Build failed with exit code {}", result.exit_code));
        }

        self.log(&format!("⏱️ Build time: {:.2}s", result.build_time.as_secs_f64()));

        if !result.warnings.is_empty() {
            self.log(&format!("⚠️ {} warnings", result.warnings.len()));
        }

        if !result.errors.is_empty() {
            self.log(&format!("❌ {} errors", result.errors.len()));
            for error in &result.errors {
                self.log(&format!("  {}", error.message));
            }
        }
    }
}