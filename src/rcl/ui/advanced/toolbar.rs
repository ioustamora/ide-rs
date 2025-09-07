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
                    let _ = ui.button(button);
                }
            });
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
    
    fn get_property(&self, name: &str) -> Option<String> {
        match name {
            "buttons" => Some(self.buttons.join(",")),
            "editable" => Some(self.editable.to_string()),
            _ => None,
        }
    }
    
    fn set_property(&mut self, name: &str, value: &str) -> bool {
        match name {
            "buttons" => {
                self.buttons = value.split(',').map(|s| s.trim().to_string()).collect();
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
        vec!["buttons".to_string(), "editable".to_string()]
    }
}
