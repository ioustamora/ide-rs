// Moved from rcl/dropdown.rs
//! Dropdown component for RCL
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct Dropdown {
    pub label: String,
    pub options: Vec<String>,
    pub selected: usize,
    pub editable: bool,
}

impl Component for Dropdown {
    fn name(&self) -> &str {
        "Dropdown"
    }
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            ui.text_edit_singleline(&mut self.label);
            // Optionally allow editing options
        } else {
            egui::ComboBox::from_label(&self.label)
                .selected_text(&self.options[self.selected])
                .show_ui(ui, |ui| {
                    for (i, option) in self.options.iter().enumerate() {
                        ui.selectable_value(&mut self.selected, i, option);
                    }
                });
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
}
