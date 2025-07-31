// Moved from rcl/color_picker.rs
//! ColorPicker component for RCL advanced UI
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct ColorPicker {
    pub color: egui::Color32,
    pub editable: bool,
}

impl Component for ColorPicker {
    fn name(&self) -> &str {
        "ColorPicker"
    }
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            ui.color_edit_button_srgba(&mut self.color);
        } else {
            ui.label(format!("Color: #{:02X}{:02X}{:02X}", self.color.r(), self.color.g(), self.color.b()));
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
}
