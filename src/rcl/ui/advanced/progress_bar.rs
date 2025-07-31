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
}
