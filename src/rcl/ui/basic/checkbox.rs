// Moved from rcl/checkbox.rs
//! Checkbox component for RCL
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct Checkbox {
    pub label: String,
    pub checked: bool,
    pub editable: bool,
}

impl Component for Checkbox {
    fn name(&self) -> &str {
        "Checkbox"
    }
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            ui.text_edit_singleline(&mut self.label);
        } else {
            ui.checkbox(&mut self.checked, &self.label);
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
}
