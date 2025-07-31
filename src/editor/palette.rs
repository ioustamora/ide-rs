//! Component palette for selecting components to add to the canvas
use egui::*;

#[allow(dead_code)]
pub struct ComponentPalette {}

#[allow(dead_code)]
impl ComponentPalette {
    pub fn new() -> Self {
        Self {}
    }

    pub fn ui(&self, ui: &mut Ui) {
        ui.label("Component Palette");
        let _ = ui.button("Label");
        let _ = ui.button("Button");
        let _ = ui.button("Textbox");
        let _ = ui.button("Checkbox");
        let _ = ui.button("Slider");
        let _ = ui.button("Dropdown");
        let _ = ui.button("RadioButton");
        // Add more as needed
    }
}
