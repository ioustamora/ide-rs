// Moved from rcl/table.rs
//! Table component for RCL advanced UI
use egui::Ui;
use crate::rcl::ui::component::Component;

pub struct Table {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub editable: bool,
}

impl Component for Table {
    fn name(&self) -> &str {
        "Table"
    }
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            for header in &mut self.headers {
                ui.text_edit_singleline(header);
            }
            for row in &mut self.rows {
                for cell in row {
                    ui.text_edit_singleline(cell);
                }
            }
        } else {
            egui::Grid::new("table_grid").show(ui, |ui| {
                for header in &self.headers {
                    ui.label(header);
                }
                ui.end_row();
                for row in &self.rows {
                    for cell in row {
                        ui.label(cell);
                    }
                    ui.end_row();
                }
            });
        }
        if ui.button("Edit").clicked() {
            self.editable = !self.editable;
        }
    }
}
