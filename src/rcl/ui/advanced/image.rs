// Moved from rcl/image.rs
//! Image component for RCL advanced UI
use egui::{Ui, TextureHandle};
use crate::rcl::ui::component::Component;

pub struct Image {
    pub texture: Option<TextureHandle>,
    pub path: String,
    pub editable: bool,
}

impl Component for Image {
    fn name(&self) -> &str {
        "Image"
    }
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            ui.text_edit_singleline(&mut self.path);
            // TODO: Load texture from path
        } else if let Some(texture) = &self.texture {
            ui.add(egui::Image::new(texture).max_size(egui::Vec2::new(128.0, 128.0)));
        } else {
            ui.label("No image loaded");
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
    
    fn get_property(&self, name: &str) -> Option<String> {
        match name {
            "path" => Some(self.path.clone()),
            "editable" => Some(self.editable.to_string()),
            "has_texture" => Some(self.texture.is_some().to_string()),
            _ => None,
        }
    }
    
    fn set_property(&mut self, name: &str, value: &str) -> bool {
        match name {
            "path" => {
                self.path = value.to_string();
                // TODO: Load texture from new path
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
            "has_texture".to_string(),
        ]
    }
}
