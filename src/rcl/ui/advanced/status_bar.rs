// Moved from rcl/status_bar.rs
//! StatusBar component for RCL advanced UI
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct StatusBar {
    pub text: String,
    pub editable: bool,
}

impl Component for StatusBar {
    fn name(&self) -> &str {
        "StatusBar"
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
