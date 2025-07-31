//! Editor canvas for drag-and-drop placement of components
use egui::*;
use crate::rcl::ui::component::Component;

#[allow(dead_code)]
pub struct EditorCanvas {
    pub components: Vec<Box<dyn Component>>,
}

#[allow(dead_code)]
impl EditorCanvas {
    pub fn new() -> Self {
        Self { components: vec![] }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label("Editor Canvas (drag and drop components here)");
        // Placeholder: render components
        for comp in &mut self.components {
            comp.render(ui);
        }
    }
}
