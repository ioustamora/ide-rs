// Moved from rcl/code_editor.rs
//! CodeEditor component for RCL advanced UI
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct CodeEditor {
    pub code: String,
    pub language: String,
    pub editable: bool,
}

impl Component for CodeEditor {
    fn name(&self) -> &str {
        "CodeEditor"
    }
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            ui.text_edit_multiline(&mut self.code);
            ui.text_edit_singleline(&mut self.language);
        } else {
            ui.label(format!("Code Editor ({})", self.language));
            ui.code(&self.code);
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
    
    fn get_property(&self, name: &str) -> Option<String> {
        match name {
            "code" => Some(self.code.clone()),
            "language" => Some(self.language.clone()),
            "editable" => Some(self.editable.to_string()),
            "line_count" => Some(self.code.lines().count().to_string()),
            "character_count" => Some(self.code.len().to_string()),
            _ => None,
        }
    }
    
    fn set_property(&mut self, name: &str, value: &str) -> bool {
        match name {
            "code" => {
                self.code = value.to_string();
                true
            }
            "language" => {
                self.language = value.to_string();
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
            "code".to_string(),
            "language".to_string(),
            "editable".to_string(),
            "line_count".to_string(),
            "character_count".to_string(),
        ]
    }
}
