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
    
    fn get_property(&self, name: &str) -> Option<String> {
        match name {
            "text" => Some(self.text.clone()),
            "editable" => Some(self.editable.to_string()),
            _ => None,
        }
    }
    
    fn set_property(&mut self, name: &str, value: &str) -> bool {
        match name {
            "text" => {
                self.text = value.to_string();
                true
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
        vec!["text".to_string(), "editable".to_string()]
    }
}
