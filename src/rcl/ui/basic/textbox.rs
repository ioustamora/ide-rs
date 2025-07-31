// Moved from rcl/textbox.rs
//! TextBox component for RCL
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct TextBox {
    pub value: String,
    pub editable: bool,
}

impl Component for TextBox {
    fn name(&self) -> &str {
        "TextBox"
    }
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            ui.text_edit_multiline(&mut self.value);
        } else {
            ui.label(&self.value);
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
}
