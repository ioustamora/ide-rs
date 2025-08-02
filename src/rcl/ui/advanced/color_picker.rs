// Moved from rcl/color_picker.rs
//! ColorPicker component for RCL advanced UI
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct ColorPicker {
    pub color: egui::Color32,
    pub editable: bool,
}

impl Component for ColorPicker {
    fn name(&self) -> &str {
        "ColorPicker"
    }
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            ui.color_edit_button_srgba(&mut self.color);
        } else {
            ui.label(format!("Color: #{:02X}{:02X}{:02X}", self.color.r(), self.color.g(), self.color.b()));
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
    
    fn get_property(&self, name: &str) -> Option<String> {
        match name {
            "color" => Some(format!("#{:02X}{:02X}{:02X}", self.color.r(), self.color.g(), self.color.b())),
            "color_rgba" => Some(format!("#{:02X}{:02X}{:02X}{:02X}", self.color.r(), self.color.g(), self.color.b(), self.color.a())),
            "red" => Some(self.color.r().to_string()),
            "green" => Some(self.color.g().to_string()),
            "blue" => Some(self.color.b().to_string()),
            "alpha" => Some(self.color.a().to_string()),
            "editable" => Some(self.editable.to_string()),
            _ => None,
        }
    }
    
    fn set_property(&mut self, name: &str, value: &str) -> bool {
        match name {
            "color" => {
                // Parse hex color format #RRGGBB
                if value.len() == 7 && value.starts_with('#') {
                    if let (Ok(r), Ok(g), Ok(b)) = (
                        u8::from_str_radix(&value[1..3], 16),
                        u8::from_str_radix(&value[3..5], 16),
                        u8::from_str_radix(&value[5..7], 16),
                    ) {
                        self.color = egui::Color32::from_rgba_unmultiplied(r, g, b, self.color.a());
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            "color_rgba" => {
                // Parse hex color format #RRGGBBAA
                if value.len() == 9 && value.starts_with('#') {
                    if let (Ok(r), Ok(g), Ok(b), Ok(a)) = (
                        u8::from_str_radix(&value[1..3], 16),
                        u8::from_str_radix(&value[3..5], 16),
                        u8::from_str_radix(&value[5..7], 16),
                        u8::from_str_radix(&value[7..9], 16),
                    ) {
                        self.color = egui::Color32::from_rgba_unmultiplied(r, g, b, a);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            "red" => {
                if let Ok(r) = value.parse::<u8>() {
                    self.color = egui::Color32::from_rgba_unmultiplied(r, self.color.g(), self.color.b(), self.color.a());
                    true
                } else {
                    false
                }
            }
            "green" => {
                if let Ok(g) = value.parse::<u8>() {
                    self.color = egui::Color32::from_rgba_unmultiplied(self.color.r(), g, self.color.b(), self.color.a());
                    true
                } else {
                    false
                }
            }
            "blue" => {
                if let Ok(b) = value.parse::<u8>() {
                    self.color = egui::Color32::from_rgba_unmultiplied(self.color.r(), self.color.g(), b, self.color.a());
                    true
                } else {
                    false
                }
            }
            "alpha" => {
                if let Ok(a) = value.parse::<u8>() {
                    self.color = egui::Color32::from_rgba_unmultiplied(self.color.r(), self.color.g(), self.color.b(), a);
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
            "color".to_string(),
            "color_rgba".to_string(),
            "red".to_string(),
            "green".to_string(),
            "blue".to_string(),
            "alpha".to_string(),
            "editable".to_string(),
        ]
    }
}
