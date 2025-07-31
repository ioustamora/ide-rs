//! RadioButton component for RCL
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct RadioButton {
    pub label: String,
    pub selected: bool,
    pub editable: bool,
}

impl Component for RadioButton {
    fn name(&self) -> &str {
        "RadioButton"
    }
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            ui.text_edit_singleline(&mut self.label);
        } else {
            ui.radio_value(&mut self.selected, true, &self.label);
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
}
