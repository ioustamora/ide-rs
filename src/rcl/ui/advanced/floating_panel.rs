//! Floating panel component for RCL advanced UI
use egui::*;
use crate::rcl::ui::component::Component;

pub struct FloatingPanel {
    pub title: String,
    pub content: String,
    pub open: bool,
    pub editable: bool,
}

#[allow(dead_code)]
impl FloatingPanel {
    pub fn new(title: &str, content: &str) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
            open: true,
            editable: false,
        }
    }
}

impl Component for FloatingPanel {
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
