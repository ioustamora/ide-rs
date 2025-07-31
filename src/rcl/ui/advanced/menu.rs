// Moved from rcl/menu.rs
//! Menu component for RCL advanced UI
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct Menu {
    pub items: Vec<String>,
    pub editable: bool,
}

impl Component for Menu {
    fn name(&self) -> &str {
        "Menu"
    }
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            for item in &mut self.items {
                ui.text_edit_singleline(item);
            }
        } else {
            egui::menu::bar(ui, |ui| {
                for item in &self.items {
                    ui.menu_button(item, |_| {});
                }
            });
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
}
