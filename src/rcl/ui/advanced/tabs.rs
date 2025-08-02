// Moved from rcl/tabs.rs
//! Tabs component for RCL advanced UI
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct Tabs {
    pub labels: Vec<String>,
    pub selected: usize,
    pub editable: bool,
}

impl Component for Tabs {
    fn name(&self) -> &str {
        "Tabs"
    }
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            for label in &mut self.labels {
                ui.text_edit_singleline(label);
            }
        } else {
            egui::ComboBox::from_label("Tabs")
                .selected_text(&self.labels[self.selected])
                .show_ui(ui, |ui| {
                    for (i, label) in self.labels.iter().enumerate() {
                        ui.selectable_value(&mut self.selected, i, label);
                    }
                });
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
    
    fn get_property(&self, name: &str) -> Option<String> {
        match name {
            "labels" => Some(self.labels.join(",")),
            "selected" => Some(self.selected.to_string()),
            "selected_label" => {
                if self.selected < self.labels.len() {
                    Some(self.labels[self.selected].clone())
                } else {
                    None
                }
            }
            "editable" => Some(self.editable.to_string()),
            "tab_count" => Some(self.labels.len().to_string()),
            _ => None,
        }
    }
    
    fn set_property(&mut self, name: &str, value: &str) -> bool {
        match name {
            "labels" => {
                self.labels = value.split(',').map(|s| s.trim().to_string()).collect();
                // Ensure selected index is still valid
                if self.selected >= self.labels.len() && !self.labels.is_empty() {
                    self.selected = self.labels.len() - 1;
                }
                true
            }
            "selected" => {
                if let Ok(index) = value.parse::<usize>() {
                    if index < self.labels.len() {
                        self.selected = index;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            "selected_label" => {
                if let Some(index) = self.labels.iter().position(|label| label == value) {
                    self.selected = index;
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
            "labels".to_string(),
            "selected".to_string(),
            "selected_label".to_string(),
            "editable".to_string(),
            "tab_count".to_string(),
        ]
    }
}
