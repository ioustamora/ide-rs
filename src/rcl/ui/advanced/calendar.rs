// Moved from rcl/calendar.rs
//! Calendar component for RCL advanced UI
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct Calendar {
    pub selected_date: String,
    pub editable: bool,
}

impl Component for Calendar {
    fn name(&self) -> &str {
        "Calendar"
    }
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            ui.label("Selected Date:");
            ui.text_edit_singleline(&mut self.selected_date);
        } else {
            ui.label(format!("Calendar: {}", self.selected_date));
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
    
    fn get_property(&self, name: &str) -> Option<String> {
        match name {
            "selected_date" => Some(self.selected_date.clone()),
            "editable" => Some(self.editable.to_string()),
            "is_date_set" => Some((!self.selected_date.is_empty()).to_string()),
            _ => None,
        }
    }
    
    fn set_property(&mut self, name: &str, value: &str) -> bool {
        match name {
            "selected_date" => {
                // TODO: Add date validation here
                self.selected_date = value.to_string();
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
        vec![
            "selected_date".to_string(),
            "editable".to_string(),
            "is_date_set".to_string(),
        ]
    }
}
