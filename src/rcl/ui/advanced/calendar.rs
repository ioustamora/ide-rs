// Moved from rcl/calendar.rs
//! Calendar component for RCL advanced UI
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct Calendar {
    pub selected_date: String,
    pub editable: bool,
}

impl Component for Calendar {
    fn name(&self) -> &str {
        "Calendar"
    }
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            ui.label("Selected Date:");
            ui.text_edit_singleline(&mut self.selected_date);
        } else {
            ui.label(format!("Calendar: {}", self.selected_date));
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
}
