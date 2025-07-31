//! Property inspector for editing selected component properties
use egui::*;

#[allow(dead_code)]
pub struct PropertyInspector {}

#[allow(dead_code)]
impl PropertyInspector {
    pub fn new() -> Self {
        Self {}
    }

    pub fn ui(&self, ui: &mut Ui) {
        ui.label("Property Inspector");
        // Placeholder: show/edit properties of selected component
    }
}
