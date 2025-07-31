// Moved from rcl/slider.rs
//! Slider component for RCL
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct Slider {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub editable: bool,
}

impl Component for Slider {
    fn name(&self) -> &str {
        "Slider"
    }
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            ui.add(egui::Slider::new(&mut self.value, self.min..=self.max).text("Value"));
        } else {
            ui.label(format!("Value: {:.2}", self.value));
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
}
