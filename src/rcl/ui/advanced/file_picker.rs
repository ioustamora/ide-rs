// Moved from rcl/file_picker.rs
//! FilePicker component for RCL advanced UI
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct FilePicker {
    pub path: String,
    pub editable: bool,
}

impl Component for FilePicker {
    fn name(&self) -> &str {
        "FilePicker"
    }
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            ui.label("File Path:");
            ui.text_edit_singleline(&mut self.path);
        } else {
            ui.label(format!("FilePicker: {}", self.path));
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
    
    fn get_property(&self, name: &str) -> Option<String> {
        match name {
            "path" => Some(self.path.clone()),
            "editable" => Some(self.editable.to_string()),
            "file_name" => {
                if let Some(file_name) = std::path::Path::new(&self.path).file_name() {
                    file_name.to_str().map(|s| s.to_string())
                } else {
                    None
                }
            }
            "file_extension" => {
                if let Some(extension) = std::path::Path::new(&self.path).extension() {
                    extension.to_str().map(|s| s.to_string())
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    
    fn set_property(&mut self, name: &str, value: &str) -> bool {
        match name {
            "path" => {
                self.path = value.to_string();
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
            "path".to_string(),
            "editable".to_string(),
            "file_name".to_string(),
            "file_extension".to_string(),
        ]
    }
}
