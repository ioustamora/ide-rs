// Moved from rcl/tabs.rs
//! Tabs component for RCL advanced UI
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct Tabs {
    pub labels: Vec<String>,
    pub selected: usize,
    pub editable: bool,
}

impl Component for Tabs {
    fn name(&self) -> &str {
        "Tabs"
    }
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            for label in &mut self.labels {
                ui.text_edit_singleline(label);
            }
        } else {
            egui::ComboBox::from_label("Tabs")
                .selected_text(&self.labels[self.selected])
                .show_ui(ui, |ui| {
                    for (i, label) in self.labels.iter().enumerate() {
                        ui.selectable_value(&mut self.selected, i, label);
                    }
                });
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
}
