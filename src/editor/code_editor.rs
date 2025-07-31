//! Code editor component for IDE, with hooks for Rust Analyzer integration
use egui::*;
use crate::rcl::ui::component::Component;

pub struct CodeEditor {
    pub code: String,
    pub language: String,
    pub editable: bool,
}

#[allow(dead_code)]
impl CodeEditor {
    pub fn new(language: &str) -> Self {
        Self {
            code: String::new(),
            language: language.to_string(),
            editable: false,
        }
    }
}

impl Component for CodeEditor {
    fn name(&self) -> &str {
        "CodeEditor"
    }
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            ui.text_edit_multiline(&mut self.code);
            ui.text_edit_singleline(&mut self.language);
        } else {
            ui.label(format!("Code Editor ({})", self.language));
            ui.code(&self.code);
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
        // TODO: Integrate with Rust Analyzer and other tools
    }
}
