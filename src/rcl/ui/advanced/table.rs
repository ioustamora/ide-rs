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
    
    fn get_property(&self, name: &str) -> Option<String> {
        match name {
            "headers" => Some(self.headers.join(",")),
            "editable" => Some(self.editable.to_string()),
            "column_count" => Some(self.headers.len().to_string()),
            "row_count" => Some(self.rows.len().to_string()),
            "total_cells" => Some((self.headers.len() + self.rows.len() * self.headers.len()).to_string()),
            _ => None,
        }
    }
    
    fn set_property(&mut self, name: &str, value: &str) -> bool {
        match name {
            "headers" => {
                self.headers = value.split(',').map(|s| s.trim().to_string()).collect();
                // Adjust existing rows to match new header count
                let header_count = self.headers.len();
                for row in &mut self.rows {
                    row.resize(header_count, String::new());
                }
                true
            }
            "editable" => {
                if let Ok(editable) = value.parse::<bool>() {
                    self.editable = editable;
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
    
    fn get_property_names(&self) -> Vec<String> {
        vec![
            "headers".to_string(),
            "editable".to_string(),
            "column_count".to_string(),
            "row_count".to_string(),
            "total_cells".to_string(),
        ]
    }
}
