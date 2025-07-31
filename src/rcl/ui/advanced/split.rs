// Moved from rcl/split.rs
//! Split (paned) component for RCL
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct Split {
    pub orientation: String, // "Horizontal" or "Vertical"
    pub ratio: f32,
    pub editable: bool,
}

impl Component for Split {
    fn name(&self) -> &str {
        "Split"
    }
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            egui::ComboBox::from_label("Orientation")
                .selected_text(&self.orientation)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.orientation, "Horizontal".to_string(), "Horizontal");
                    ui.selectable_value(&mut self.orientation, "Vertical".to_string(), "Vertical");
                });
            ui.add(egui::Slider::new(&mut self.ratio, 0.0..=1.0).text("Ratio"));
        } else {
            ui.label(format!("Split: {} ({:.2})", self.orientation, self.ratio));
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
}
