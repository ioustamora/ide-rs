//! Floating panel component for RCL advanced UI
use egui::*;
use crate::rcl::ui::component::Component;

pub struct FloatingPanel {
    pub title: String,
    pub content: String,
    pub open: bool,
    pub editable: bool,
    pub position: Option<Pos2>,
    pub size: Option<Vec2>,
    pub resizable: bool,
    pub collapsible: bool,
}

#[allow(dead_code)]
impl FloatingPanel {
    pub fn new(title: &str, content: &str) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
            open: true,
            editable: false,
            position: None,
            size: Some(Vec2::new(300.0, 200.0)),
            resizable: true,
            collapsible: true,
        }
    }
    
    pub fn with_position(mut self, pos: Pos2) -> Self {
        self.position = Some(pos);
        self
    }
    
    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = Some(size);
        self
    }
    
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }
    
    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }
}

impl Component for FloatingPanel {
    fn name(&self) -> &str {
        "FloatingPanel"
    }
    
    fn render(&mut self, ui: &mut Ui) {
        if self.open {
            let mut window = egui::Window::new(&self.title)
                .open(&mut self.open)
                .resizable(self.resizable)
                .collapsible(self.collapsible);
            
            // Set initial position if specified
            if let Some(pos) = self.position {
                window = window.current_pos(pos);
            }
            
            // Set initial size if specified
            if let Some(size) = self.size {
                window = window.default_size(size);
            }
            
            // Constrain to screen bounds
            window = window.constrain(true);
            
            let response = window.show(ui.ctx(), |ui| {
                // Panel header with controls
                ui.horizontal(|ui| {
                    ui.heading(&self.title);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.small_button("Edit").clicked() {
                            self.editable = !self.editable;
                        }
                    });
                });
                
                ui.separator();
                
                // Content area
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if self.editable {
                            ui.label("Title:");
                            ui.text_edit_singleline(&mut self.title);
                            ui.label("Content:");
                            ui.text_edit_multiline(&mut self.content);
                        } else {
                            ui.label(&self.content);
                        }
                    });
                
                // Footer with panel info
                ui.separator();
                ui.horizontal(|ui| {
                    ui.small(format!("Resizable: {} | Collapsible: {}", 
                                   self.resizable, self.collapsible));
                });
            });
            
            // Update position and size from window response
            if let Some(response) = response {
                let rect = response.response.rect;
                self.position = Some(rect.min);
                self.size = Some(rect.size());
            }
        }
    }
}
