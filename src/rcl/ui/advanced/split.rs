// Moved from rcl/split.rs
//! Split (paned) component for RCL
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct Split {
    pub orientation: String, // "Horizontal" or "Vertical"
    pub ratio: f32,
    pub editable: bool,
}

impl Component for Split {
    fn name(&self) -> &str {
        "Split"
    }
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            egui::ComboBox::from_label("Orientation")
                .selected_text(&self.orientation)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.orientation, "Horizontal".to_string(), "Horizontal");
                    ui.selectable_value(&mut self.orientation, "Vertical".to_string(), "Vertical");
                });
            ui.add(egui::Slider::new(&mut self.ratio, 0.0..=1.0).text("Ratio"));
        } else {
            ui.label(format!("Split: {} ({:.2})", self.orientation, self.ratio));
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
    
    fn get_property(&self, name: &str) -> Option<String> {
        match name {
            "orientation" => Some(self.orientation.clone()),
            "ratio" => Some(self.ratio.to_string()),
            "editable" => Some(self.editable.to_string()),
            _ => None,
        }
    }
    
    fn set_property(&mut self, name: &str, value: &str) -> bool {
        match name {
            "orientation" => {
                let orientation = value.to_string();
                if orientation == "Horizontal" || orientation == "Vertical" {
                    self.orientation = orientation;
                    true
                } else {
                    false
                }
            }
            "ratio" => {
                if let Ok(ratio) = value.parse::<f32>() {
                    if ratio >= 0.0 && ratio <= 1.0 {
                        self.ratio = ratio;
                        true
                    } else {
                        false
                    }
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
        vec!["orientation".to_string(), "ratio".to_string(), "editable".to_string()]
    }
}
