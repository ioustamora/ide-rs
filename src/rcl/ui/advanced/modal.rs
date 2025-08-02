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
        "Modal"
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
    
    fn get_property(&self, name: &str) -> Option<String> {
        match name {
            "title" => Some(self.title.clone()),
            "content" => Some(self.content.clone()),
            "open" => Some(self.open.to_string()),
            "editable" => Some(self.editable.to_string()),
            _ => None,
        }
    }
    
    fn set_property(&mut self, name: &str, value: &str) -> bool {
        match name {
            "title" => {
                self.title = value.to_string();
                true
            }
            "content" => {
                self.content = value.to_string();
                true
            }
            "open" => {
                if let Ok(open) = value.parse::<bool>() {
                    self.open = open;
                    true
                } else {
                    false
                }
            }
            "editable" => {
                if let Ok(editable) = value.parse::<bool>() {
                    self.editable = editable;
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
    
    fn get_property_names(&self) -> Vec<String> {
        vec![
            "title".to_string(),
            "content".to_string(),
            "open".to_string(),
            "editable".to_string(),
        ]
    }
}
