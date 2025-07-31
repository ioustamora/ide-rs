#![allow(dead_code)]
//! Backend logic for IDE actions
use crate::editor::{output_panel::OutputPanel, ai_panel::AiPanel, component_registry::{ComponentRegistry, ComponentMetadata}};
use std::process::Command;
use std::process::Stdio;
use std::thread;
use std::sync::mpsc;
use anyhow::Result;
use reqwest::blocking::Client;

pub fn build_debug(output_panel: &mut OutputPanel) {
    run_cargo_command(output_panel, &["build"]);
}

pub fn build_release(output_panel: &mut OutputPanel) {
    run_cargo_command(output_panel, &["build", "--release"]);
}

pub fn run_debug(output_panel: &mut OutputPanel) {
    run_cargo_command(output_panel, &["run"]);
}

pub fn run_release(output_panel: &mut OutputPanel) {
    run_cargo_command(output_panel, &["run", "--release"]);
}

pub fn run_custom(output_panel: &mut OutputPanel, args: &[&str]) {
    run_cargo_command(output_panel, args);
}

pub fn run_cargo_command(output_panel: &mut OutputPanel, args: &[&str]) {
    let output = Command::new("cargo")
        .args(args)
        .output();
    match output {
        Ok(res) => output_panel.set_output(&String::from_utf8_lossy(&res.stdout)),
        Err(e) => output_panel.set_output(&format!("Error: {}", e)),
    }
}

pub fn build_multi_project(output_panel: &mut OutputPanel, project_paths: &[&str], args: &[&str]) {
    for path in project_paths {
        let output = Command::new("cargo")
            .args(args)
            .current_dir(path)
            .output();
        match output {
            Ok(res) => output_panel.set_output(&format!("{}:\n{}", path, String::from_utf8_lossy(&res.stdout))),
            Err(e) => output_panel.set_output(&format!("{}: Error: {}", path, e)),
        }
    }
}

pub fn ai_chat(ai_panel: &mut AiPanel, message: &str) {
    // Placeholder: integrate with Ollama AI agent
    ai_panel.chat_history.push(format!("AI: (response to '{}')", message));
}

/// Package a component for distribution
pub fn package_component(name: &str, source: &str, _output: &str, registry: &mut ComponentRegistry) {
    // TODO: Add real packaging logic (zip/tar/custom format)
    // For now, just register metadata
    let metadata = ComponentMetadata {
        name: name.to_string(),
        version: "0.1.0".to_string(),
        source: source.to_string(),
        description: format!("Packaged component {} from {}", name, source),
    };
    registry.install(metadata);
    // TODO: Write package file to output
}

/// Install a component from a package file
pub fn install_component(pkg_path: &str, install_dir: &str, registry: &mut ComponentRegistry) {
    // TODO: Add real install logic (extract, copy, validate)
    // For now, just register dummy metadata
    let metadata = ComponentMetadata {
        name: pkg_path.to_string(),
        version: "0.1.0".to_string(),
        source: install_dir.to_string(),
        description: format!("Installed component from {}", pkg_path),
    };
    registry.install(metadata);
}

/// Uninstall a component
pub fn uninstall_component(pkg_name: &str, _install_dir: &str, registry: &mut ComponentRegistry) {
    // TODO: Add real uninstall logic (remove files, update registry)
    registry.uninstall(pkg_name);
}

/// Export the current project to a specified directory
pub fn export_project(project_dir: &str, export_dir: &str, output_panel: &mut OutputPanel) {
    // TODO: Add real export logic (copy files, update metadata, etc.)
    output_panel.log(&format!("Exported project from {} to {}", project_dir, export_dir));
}

/// Format code in the current project
pub fn format_code(project_dir: &str, output_panel: &mut OutputPanel) {
    // TODO: Add real code formatting logic (e.g., rustfmt)
    output_panel.log(&format!("Formatted code in project: {}", project_dir));
}

/// Open the settings panel
pub fn open_settings_panel() {
    // TODO: Implement settings panel logic
}

/// Build the project using Cargo and log output
pub fn build_project(project_dir: &str, release: bool, output_panel: &mut OutputPanel) {
    let mut cmd = Command::new("cargo");
    cmd.arg("build");
    if release {
        cmd.arg("--release");
    }
    cmd.current_dir(project_dir);
    match cmd.output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            output_panel.log(&format!("Build output:\n{}", stdout));
            if !stderr.is_empty() {
                output_panel.log(&format!("Build errors:\n{}", stderr));
            }
        }
        Err(e) => {
            output_panel.log(&format!("Failed to run cargo build: {}", e));
        }
    }
}

/// Run the project using Cargo and log output
pub fn run_project(project_dir: &str, release: bool, output_panel: &mut OutputPanel) {
    let mut cmd = Command::new("cargo");
    cmd.arg("run");
    if release {
        cmd.arg("--release");
    }
    cmd.current_dir(project_dir);
    match cmd.output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            output_panel.log(&format!("Run output:\n{}", stdout));
            if !stderr.is_empty() {
                output_panel.log(&format!("Run errors:\n{}", stderr));
            }
        }
        Err(e) => {
            output_panel.log(&format!("Failed to run cargo run: {}", e));
        }
    }
}

/// Run tests using Cargo and log output
pub fn run_tests_cargo(project_dir: &str, output_panel: &mut OutputPanel) {
    let mut cmd = Command::new("cargo");
    cmd.arg("test");
    cmd.current_dir(project_dir);
    match cmd.output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            output_panel.log(&format!("Test output:\n{}", stdout));
            if !stderr.is_empty() {
                output_panel.log(&format!("Test errors:\n{}", stderr));
            }
        }
        Err(e) => {
            output_panel.log(&format!("Failed to run cargo test: {}", e));
        }
    }
}

/// Generate documentation using Cargo and log output
pub fn generate_docs_cargo(project_dir: &str, output_panel: &mut OutputPanel) {
    let mut cmd = Command::new("cargo");
    cmd.arg("doc");
    cmd.current_dir(project_dir);
    match cmd.output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            output_panel.log(&format!("Doc output:\n{}", stdout));
            if !stderr.is_empty() {
                output_panel.log(&format!("Doc errors:\n{}", stderr));
            }
        }
        Err(e) => {
            output_panel.log(&format!("Failed to run cargo doc: {}", e));
        }
    }
}

/// Run a command asynchronously and log output/errors
pub fn run_command_async(cmd: Vec<&str>, project_dir: &str, output_panel: &mut OutputPanel) -> Result<()> {
    let (tx, rx) = mpsc::channel();
    let cmd_vec = cmd.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    let dir = project_dir.to_string();
    thread::spawn(move || {
        let mut command = std::process::Command::new(&cmd_vec[0]);
        for arg in &cmd_vec[1..] {
            command.arg(arg);
        }
        command.current_dir(&dir);
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        match command.output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                tx.send((stdout, stderr)).ok();
            }
            Err(e) => {
                tx.send((String::new(), format!("Failed to run command: {}", e))).ok();
            }
        }
    });
    // Wait for result and log
    if let Ok((stdout, stderr)) = rx.recv() {
        output_panel.log(&format!("Async output:\n{}", stdout));
        if !stderr.is_empty() {
            output_panel.log(&format!("Async errors:\n{}", parse_errors(&stderr)));
        }
    }
    Ok(())
}

/// Parse Rust/Cargo error output for improved display
pub fn parse_errors(error_output: &str) -> String {
    error_output
        .lines()
        .filter(|line| line.contains("error") || line.contains("failed") || line.contains("panic"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// AI-powered automation stub (to be integrated with Ollama or other agent)
pub fn ai_automate(task: &str, context: &str, output_panel: &mut OutputPanel) {
    // TODO: Integrate with real AI agent
    output_panel.log(&format!("AI automation requested: {}\nContext: {}", task, context));
    // Simulate AI response
    output_panel.log("AI: Task analyzed and automation steps generated.");
}

/// AI-powered error fixing stub
pub fn ai_fix_errors(error_output: &str, context: &str, output_panel: &mut OutputPanel) {
    // TODO: Integrate with real AI agent for error fixing
    let parsed = parse_errors(error_output);
    output_panel.log(&format!("AI error fix requested.\nErrors:\n{}\nContext: {}", parsed, context));
    // Simulate AI response
    output_panel.log("AI: Errors analyzed and fix suggestions generated.");
}

/// Connect to a real AI agent (Ollama HTTP API example)
pub fn ai_query_ollama(prompt: &str, context: &str, output_panel: &mut OutputPanel) {
    let client = Client::new();
    let url = "http://localhost:11434/api/generate";
    let body = serde_json::json!({
        "model": "llama2",
        "prompt": format!("{}\nContext: {}", prompt, context),
        "stream": false
    });
    match client.post(url).json(&body).send() {
        Ok(resp) => {
            match resp.json::<serde_json::Value>() {
                Ok(json) => {
                    if let Some(answer) = json.get("response") {
                        output_panel.log(&format!("AI agent response:\n{}", answer));
                    } else {
                        output_panel.log("AI agent: No response field in result.");
                    }
                }
                Err(e) => {
                    output_panel.log(&format!("AI agent: Failed to parse response: {}", e));
                }
            }
        }
        Err(e) => {
            output_panel.log(&format!("AI agent: Request failed: {}", e));
        }
    }
}
