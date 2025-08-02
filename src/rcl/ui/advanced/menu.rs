// Moved from rcl/menu.rs
//! Menu component for RCL advanced UI
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct Menu {
    pub items: Vec<String>,
    pub editable: bool,
}

impl Component for Menu {
    fn name(&self) -> &str {
        "Menu"
    }
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            for item in &mut self.items {
                ui.text_edit_singleline(item);
            }
        } else {
            egui::menu::bar(ui, |ui| {
                for item in &self.items {
                    ui.menu_button(item, |_| {});
                }
            });
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
    
    fn get_property(&self, name: &str) -> Option<String> {
        match name {
            "items" => Some(self.items.join(",")),
            "editable" => Some(self.editable.to_string()),
            _ => None,
        }
    }
    
    fn set_property(&mut self, name: &str, value: &str) -> bool {
        match name {
            "items" => {
                self.items = value.split(',').map(|s| s.trim().to_string()).collect();
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
        vec!["items".to_string(), "editable".to_string()]
    }
}
