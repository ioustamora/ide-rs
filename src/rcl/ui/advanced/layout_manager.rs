//! Layout Manager component for advanced UI layouts in RCL
//!
//! This component provides flexible layout management with support for
//! horizontal, vertical, and grid-based layouts with spacing and alignment options.

use egui::{Ui, Vec2};
use crate::rcl::ui::component::Component;

/// Advanced layout manager with support for different layout types
/// 
/// The Layout Manager supports:
/// - Horizontal layouts with configurable spacing
/// - Vertical layouts with proper alignment
/// - Grid layouts with custom column counts
/// - Responsive sizing and padding
pub struct LayoutManager {
    /// Type of layout to use
    pub layout_type: LayoutType,
    /// Spacing between elements
    pub spacing: f32,
    /// Padding around the entire layout
    pub padding: Vec2,
    /// Grid columns (only used for Grid layout)
    pub grid_columns: usize,
    /// Whether the component is in edit mode
    pub editable: bool,
    /// Child components managed by this layout
    pub children: Vec<String>, // Simplified - in real implementation would be Box<dyn Component>
}

/// Types of layouts supported by the Layout Manager
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutType {
    Horizontal,
    Vertical,
    Grid,
}

impl Default for LayoutManager {
    fn default() -> Self {
        Self {
            layout_type: LayoutType::Vertical,
            spacing: 4.0,
            padding: Vec2::new(8.0, 8.0),
            grid_columns: 2,
            editable: false,
            children: vec!["Child 1".to_string(), "Child 2".to_string()],
        }
    }
}

impl Component for LayoutManager {
    fn name(&self) -> &str {
        "LayoutManager"
    }
    
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            // Edit mode - show configuration options
            ui.heading("Layout Manager Configuration");
            ui.separator();
            
            // Layout type selection
            ui.horizontal(|ui| {
                ui.label("Layout Type:");
                ui.selectable_value(&mut self.layout_type, LayoutType::Horizontal, "Horizontal");
                ui.selectable_value(&mut self.layout_type, LayoutType::Vertical, "Vertical");
                ui.selectable_value(&mut self.layout_type, LayoutType::Grid, "Grid");
            });
            
            // Spacing configuration
            ui.horizontal(|ui| {
                ui.label("Spacing:");
                ui.add(egui::Slider::new(&mut self.spacing, 0.0..=20.0).suffix("px"));
            });
            
            // Padding configuration
            ui.horizontal(|ui| {
                ui.label("Padding X:");
                ui.add(egui::Slider::new(&mut self.padding.x, 0.0..=50.0).suffix("px"));
            });
            ui.horizontal(|ui| {
                ui.label("Padding Y:");
                ui.add(egui::Slider::new(&mut self.padding.y, 0.0..=50.0).suffix("px"));
            });
            
            // Grid columns (only for grid layout)
            if self.layout_type == LayoutType::Grid {
                ui.horizontal(|ui| {
                    ui.label("Grid Columns:");
                    ui.add(egui::Slider::new(&mut self.grid_columns, 1..=6));
                });
            }
            
            ui.separator();
            
            // Child management
            ui.heading("Child Components");
            for (i, child) in self.children.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("Child {}:", i + 1));
                    ui.text_edit_singleline(child);
                    if ui.button("Remove").clicked() {
                        // Mark for removal (simplified implementation)
                    }
                });
            }
            
            if ui.button("Add Child").clicked() {
                self.children.push(format!("Child {}", self.children.len() + 1));
            }
            
        } else {
            // Display mode - show the actual layout with children
            let _frame = egui::Frame::none()
                .inner_margin(egui::Margin::symmetric(self.padding.x, self.padding.y))
                .show(ui, |ui| {
                    match self.layout_type {
                        LayoutType::Horizontal => {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = self.spacing;
                                for child in &self.children {
                                    ui.label(child);
                                    ui.separator();
                                }
                            });
                        }
                        LayoutType::Vertical => {
                            ui.spacing_mut().item_spacing.y = self.spacing;
                            for child in &self.children {
                                ui.label(child);
                                ui.separator();
                            }
                        }
                        LayoutType::Grid => {
                            egui::Grid::new("layout_manager_grid")
                                .num_columns(self.grid_columns)
                                .spacing([self.spacing, self.spacing])
                                .show(ui, |ui| {
                                    for (i, child) in self.children.iter().enumerate() {
                                        ui.label(child);
                                        if (i + 1) % self.grid_columns == 0 {
                                            ui.end_row();
                                        }
                                    }
                                });
                        }
                    }
                });
            
            // Show layout info
            ui.label(format!("Layout: {:?} | Children: {}", self.layout_type, self.children.len()));
        }
        
        // Edit toggle button
        if ui.button(if self.editable { "Done" } else { "Configure" }).clicked() {
            self.editable = !self.editable;
        }
    }
}