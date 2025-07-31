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
            ui.image(texture, [128.0, 128.0]);
        } else {
            ui.label("No image loaded");
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
}
