// Moved from rcl/progress_bar.rs
//! ProgressBar component for RCL advanced UI
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct ProgressBar {
    pub value: f32,
    pub max: f32,
    pub editable: bool,
}

impl Component for ProgressBar {
    fn name(&self) -> &str {
        "ProgressBar"
    }
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            ui.add(egui::Slider::new(&mut self.value, 0.0..=self.max).text("Progress"));
        } else {
            ui.add(egui::ProgressBar::new(self.value / self.max).text(format!("{:.0}%", self.value / self.max * 100.0)));
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
    
    fn get_property(&self, name: &str) -> Option<String> {
        match name {
            "value" => Some(self.value.to_string()),
            "max" => Some(self.max.to_string()),
            "editable" => Some(self.editable.to_string()),
            "percentage" => Some(format!("{:.1}", (self.value / self.max) * 100.0)),
            "normalized" => Some((self.value / self.max).to_string()),
            _ => None,
        }
    }
    
    fn set_property(&mut self, name: &str, value: &str) -> bool {
        match name {
            "value" => {
                if let Ok(val) = value.parse::<f32>() {
                    if val >= 0.0 && val <= self.max {
                        self.value = val;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            "max" => {
                if let Ok(max) = value.parse::<f32>() {
                    if max > 0.0 {
                        self.max = max;
                        // Ensure current value doesn't exceed new max
                        if self.value > self.max {
                            self.value = self.max;
                        }
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
            "percentage" => {
                if let Ok(percentage) = value.parse::<f32>() {
                    if percentage >= 0.0 && percentage <= 100.0 {
                        self.value = (percentage / 100.0) * self.max;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            "normalized" => {
                if let Ok(normalized) = value.parse::<f32>() {
                    if normalized >= 0.0 && normalized <= 1.0 {
                        self.value = normalized * self.max;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }
    
    fn get_property_names(&self) -> Vec<String> {
        vec![
            "value".to_string(),
            "max".to_string(),
            "editable".to_string(),
            "percentage".to_string(),
            "normalized".to_string(),
        ]
    }
}
