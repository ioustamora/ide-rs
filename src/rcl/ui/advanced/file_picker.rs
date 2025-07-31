// Moved from rcl/file_picker.rs
//! FilePicker component for RCL advanced UI
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct FilePicker {
    pub path: String,
    pub editable: bool,
}

impl Component for FilePicker {
    fn name(&self) -> &str {
        "FilePicker"
    }
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            ui.label("File Path:");
            ui.text_edit_singleline(&mut self.path);
        } else {
            ui.label(format!("FilePicker: {}", self.path));
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
}
