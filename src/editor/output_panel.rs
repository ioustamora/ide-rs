//! Output panel for displaying Cargo and command output
use egui::*;
use std::fs::File;
use std::io::Write;

pub struct OutputPanel {
    pub output: String,
    pub search: String,
    pub filtered: String,
    pub log_history: Vec<String>,
}

impl OutputPanel {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            search: String::new(),
            filtered: String::new(),
            log_history: vec![],
        }
    }

    pub fn set_output(&mut self, text: &str) {
        self.output = text.to_string();
        self.log_history.push(self.output.clone());
        self.apply_filter();
    }

    pub fn apply_filter(&mut self) {
        if self.search.is_empty() {
            self.filtered = self.output.clone();
        } else {
            self.filtered = self.output
                .lines()
                .filter(|line| line.contains(&self.search))
                .collect::<Vec<_>>()
                .join("\n");
        }
    }

    pub fn export(&self, path: &str) {
        if let Ok(mut file) = File::create(path) {
            let _ = file.write_all(self.filtered.as_bytes());
        }
    }

    #[allow(dead_code)]
    /// Append a log entry to the output panel and history
    pub fn log(&mut self, text: &str) {
        self.output.push_str(text);
        self.output.push('\n');
        self.log_history.push(text.to_string());
        self.apply_filter();
    }

    #[allow(dead_code)]
    /// Display parsed errors in a dedicated panel section
    pub fn show_errors(&self, ui: &mut Ui, error_output: &str) {
        let parsed = crate::editor::actions::parse_errors(error_output);
        ui.label("Errors:");
        ui.text_edit_multiline(&mut parsed.clone());
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label("Cargo Output:");
        ui.text_edit_singleline(&mut self.search);
        if ui.button("Search").clicked() {
            self.apply_filter();
        }
        ui.text_edit_multiline(&mut self.filtered);
        if ui.button("Clear Output").clicked() {
            self.output.clear();
            self.filtered.clear();
        }
        if ui.button("Export Output").clicked() {
            self.export("output.log");
        }
        if ui.button("Show Log History").clicked() {
            ui.label("Log History:");
            for log in &self.log_history {
                ui.text_edit_multiline(&mut log.clone());
            }
        }
    }
}
