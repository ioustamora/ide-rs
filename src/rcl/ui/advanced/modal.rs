// Moved from rcl/modal.rs
//! Modal dialog component for RCL
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct Modal {
    pub title: String,
    pub content: String,
    pub open: bool,
    pub editable: bool,
}

impl Component for Modal {
    fn name(&self) -> &str {
        &self.title
    }
    fn render(&mut self, ui: &mut Ui) {
        if self.open {
            egui::Window::new(&self.title)
                .open(&mut self.open)
                .show(ui.ctx(), |ui| {
                    if self.editable {
                        ui.text_edit_singleline(&mut self.title);
                        ui.text_edit_multiline(&mut self.content);
                    } else {
                        ui.label(&self.content);
                    }
                    if ui.button("Edit").clicked() {
                        self.editable = !self.editable;
                    }
                });
        }
    }
}
