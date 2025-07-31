//! Project manager for IDE: open, close, create projects using Cargo
use std::path::PathBuf;
use std::process::Command;

#[allow(dead_code)]
pub struct ProjectManager {
    pub current_project: Option<PathBuf>,
}

#[allow(dead_code)]
impl ProjectManager {
    pub fn new() -> Self {
        Self { current_project: None }
    }

    pub fn open_project(&mut self, path: PathBuf) {
        self.current_project = Some(path);
        // TODO: Load project files, update UI
    }

    pub fn close_project(&mut self) {
        self.current_project = None;
        // TODO: Cleanup UI/state
    }

    pub fn create_project(&self, path: &PathBuf, name: &str) -> Result<(), anyhow::Error> {
        let status = Command::new("cargo")
            .arg("new")
            .arg(name)
            .current_dir(path)
            .status()?;
        if status.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to create project"))
        }
    }
}
