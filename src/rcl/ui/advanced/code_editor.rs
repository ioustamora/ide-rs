// Moved from rcl/code_editor.rs
//! CodeEditor component for RCL advanced UI
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct CodeEditor {
    pub code: String,
    pub language: String,
    pub editable: bool,
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
    }
}
