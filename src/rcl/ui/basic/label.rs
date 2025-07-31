// Moved from rcl/label.rs
//! Label component for RCL
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct Label {
    pub text: String,
    pub editable: bool,
}

impl Component for Label {
    fn name(&self) -> &str {
        "Label"
    }
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            ui.text_edit_singleline(&mut self.text);
        } else {
            ui.label(&self.text);
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
}
