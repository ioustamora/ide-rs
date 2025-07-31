// Moved from rcl/toolbar.rs
//! Toolbar component for RCL advanced UI
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct Toolbar {
    pub buttons: Vec<String>,
    pub editable: bool,
}

impl Component for Toolbar {
    fn name(&self) -> &str {
        "Toolbar"
    }
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            for button in &mut self.buttons {
                ui.text_edit_singleline(button);
            }
        } else {
            ui.horizontal(|ui| {
                for button in &self.buttons {
                    ui.button(button);
                }
            });
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
}
