// Moved from rcl/notification.rs
//! Notification component for RCL advanced UI
use egui::Ui;
use crate::rcl::ui::component::Component;

pub enum NotificationKind {
    Info,
    Warning,
    Error,
}

pub struct Notification {
    pub message: String,
    pub kind: NotificationKind,
    pub editable: bool,
}

impl Component for Notification {
    fn name(&self) -> &str {
        "Notification"
    }
    fn render(&mut self, ui: &mut Ui) {
        let color = match self.kind {
            NotificationKind::Info => egui::Color32::LIGHT_BLUE,
            NotificationKind::Warning => egui::Color32::YELLOW,
            NotificationKind::Error => egui::Color32::RED,
        };
        if self.editable {
            ui.text_edit_singleline(&mut self.message);
            egui::ComboBox::from_label("Kind")
                .selected_text(match self.kind {
                    NotificationKind::Info => "Info",
                    NotificationKind::Warning => "Warning",
                    NotificationKind::Error => "Error",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.kind, NotificationKind::Info, "Info");
                    ui.selectable_value(&mut self.kind, NotificationKind::Warning, "Warning");
                    ui.selectable_value(&mut self.kind, NotificationKind::Error, "Error");
                });
        } else {
            ui.colored_label(color, &self.message);
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
}
