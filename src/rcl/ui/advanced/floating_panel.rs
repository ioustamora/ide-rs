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
    
    fn get_property(&self, name: &str) -> Option<String> {
        match name {
            "title" => Some(self.title.clone()),
            "content" => Some(self.content.clone()),
            "open" => Some(self.open.to_string()),
            "editable" => Some(self.editable.to_string()),
            "resizable" => Some(self.resizable.to_string()),
            "collapsible" => Some(self.collapsible.to_string()),
            "position_x" => self.position.map(|pos| pos.x.to_string()),
            "position_y" => self.position.map(|pos| pos.y.to_string()),
            "width" => self.size.map(|size| size.x.to_string()),
            "height" => self.size.map(|size| size.y.to_string()),
            _ => None,
        }
    }
    
    fn set_property(&mut self, name: &str, value: &str) -> bool {
        match name {
            "title" => {
                self.title = value.to_string();
                true
            }
            "content" => {
                self.content = value.to_string();
                true
            }
            "open" => {
                if let Ok(open) = value.parse::<bool>() {
                    self.open = open;
                    true
                } else {
                    false
                }
            }
            "editable" => {
                if let Ok(editable) = value.parse::<bool>() {
                    self.editable = editable;
                    true
                } else {
                    false
                }
            }
            "resizable" => {
                if let Ok(resizable) = value.parse::<bool>() {
                    self.resizable = resizable;
                    true
                } else {
                    false
                }
            }
            "collapsible" => {
                if let Ok(collapsible) = value.parse::<bool>() {
                    self.collapsible = collapsible;
                    true
                } else {
                    false
                }
            }
            "position_x" => {
                if let Ok(x) = value.parse::<f32>() {
                    if let Some(pos) = &mut self.position {
                        pos.x = x;
                    } else {
                        self.position = Some(Pos2::new(x, 0.0));
                    }
                    true
                } else {
                    false
                }
            }
            "position_y" => {
                if let Ok(y) = value.parse::<f32>() {
                    if let Some(pos) = &mut self.position {
                        pos.y = y;
                    } else {
                        self.position = Some(Pos2::new(0.0, y));
                    }
                    true
                } else {
                    false
                }
            }
            "width" => {
                if let Ok(w) = value.parse::<f32>() {
                    if let Some(size) = &mut self.size {
                        size.x = w;
                    } else {
                        self.size = Some(Vec2::new(w, 200.0));
                    }
                    true
                } else {
                    false
                }
            }
            "height" => {
                if let Ok(h) = value.parse::<f32>() {
                    if let Some(size) = &mut self.size {
                        size.y = h;
                    } else {
                        self.size = Some(Vec2::new(300.0, h));
                    }
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
            "title".to_string(),
            "content".to_string(),
            "open".to_string(),
            "editable".to_string(),
            "resizable".to_string(),
            "collapsible".to_string(),
            "position_x".to_string(),
            "position_y".to_string(),
            "width".to_string(),
            "height".to_string(),
        ]
    }
}
