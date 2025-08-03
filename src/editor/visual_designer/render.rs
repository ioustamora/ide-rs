use egui;

/// Grid display settings
#[derive(Clone)]
pub struct GridSettings {
    pub size: f32,
    pub color: egui::Color32,
    pub major_lines: u32,
    pub visible: bool,
    pub snap_enabled: bool,
}

impl Default for GridSettings {
    fn default() -> Self {
        Self {
            size: 20.0,
            color: egui::Color32::from_rgba_unmultiplied(128, 128, 128, 64),
            major_lines: 5,
            visible: true,
            snap_enabled: true,
        }
    }
}

/// Rendering logic for Visual Designer
impl GuideSystem {
    pub fn draw_guides(&self, ui: &mut egui::Ui, canvas_rect: egui::Rect) {
        let painter = ui.painter();
        for &y in &self.horizontal_guides {
            painter.line_segment(
                [egui::pos2(canvas_rect.min.x, y), egui::pos2(canvas_rect.max.x, y)],
                egui::Stroke::new(1.0, self.guide_color)
            );
        }
        for &x in &self.vertical_guides {
            painter.line_segment(
                [egui::pos2(x, canvas_rect.min.y), egui::pos2(x, canvas_rect.max.y)],
                egui::Stroke::new(1.0, self.guide_color)
            );
        }
    }

    pub fn draw_rulers(&self, ui: &mut egui::Ui, canvas_rect: egui::Rect) {
        let painter = ui.painter();
        let ruler_size = 20.0;
        let h_ruler_rect = egui::Rect::from_min_size(
            egui::pos2(canvas_rect.min.x, canvas_rect.min.y - ruler_size),
            egui::vec2(canvas_rect.width(), ruler_size)
        );
        painter.rect_filled(h_ruler_rect, 0.0, self.ruler_color);
        let v_ruler_rect = egui::Rect::from_min_size(
            egui::pos2(canvas_rect.min.x - ruler_size, canvas_rect.min.y),
            egui::vec2(ruler_size, canvas_rect.height())
        );
        painter.rect_filled(v_ruler_rect, 0.0, self.ruler_color);
        Self::draw_ruler_marks(ui, h_ruler_rect, v_ruler_rect);
    }

    fn draw_ruler_marks(ui: &mut egui::Ui, h_ruler: egui::Rect, v_ruler: egui::Rect) {
        let painter = ui.painter();
        let mark_interval = 50.0;
        let mut x = h_ruler.min.x;
        let mut pixel_count = 0;
        while x <= h_ruler.max.x {
            let mark_height = if pixel_count % 100 == 0 { 8.0 } else { 4.0 };
            painter.line_segment(
                [egui::pos2(x, h_ruler.max.y - mark_height), egui::pos2(x, h_ruler.max.y)],
                egui::Stroke::new(1.0, egui::Color32::WHITE)
            );
            if pixel_count % 100 == 0 {
                painter.text(
                    egui::pos2(x + 2.0, h_ruler.max.y - 12.0),
                    egui::Align2::LEFT_BOTTOM,
                    pixel_count.to_string(),
                    egui::FontId::monospace(8.0),
                    egui::Color32::WHITE
                );
            }
            x += mark_interval;
            pixel_count += mark_interval as i32;
        }
        let mut y = v_ruler.min.y;
        pixel_count = 0;
        while y <= v_ruler.max.y {
            let mark_width = if pixel_count % 100 == 0 { 8.0 } else { 4.0 };
            painter.line_segment(
                [egui::pos2(v_ruler.max.x - mark_width, y), egui::pos2(v_ruler.max.x, y)],
                egui::Stroke::new(1.0, egui::Color32::WHITE)
            );
            if pixel_count % 100 == 0 {
                painter.text(
                    egui::pos2(v_ruler.max.x - 2.0, y - 2.0),
                    egui::Align2::RIGHT_BOTTOM,
                    pixel_count.to_string(),
                    egui::FontId::monospace(8.0),
                    egui::Color32::WHITE
                );
            }
            y += mark_interval;
            pixel_count += mark_interval as i32;
        }
    }
}

impl GridSettings {
    pub fn draw_grid(&self, ui: &mut egui::Ui, canvas_rect: egui::Rect) {
        let painter = ui.painter();
        let grid_size = self.size.max(1.0);
        let max_lines = 1000;
        let h_lines = ((canvas_rect.width() / grid_size) as i32).min(max_lines);
        let v_lines = ((canvas_rect.height() / grid_size) as i32).min(max_lines);
        for i in 0..=h_lines {
            let x = canvas_rect.min.x + (i as f32 * grid_size);
            if x > canvas_rect.max.x { break; }
            let is_major = i % (self.major_lines as i32) == 0;
            let color = if is_major {
                self.color.gamma_multiply(1.8)
            } else {
                self.color
            };
            painter.line_segment(
                [egui::pos2(x, canvas_rect.min.y), egui::pos2(x, canvas_rect.max.y)],
                egui::Stroke::new(if is_major { 1.2 } else { 0.6 }, color)
            );
        }
        for i in 0..=v_lines {
            let y = canvas_rect.min.y + (i as f32 * grid_size);
            if y > canvas_rect.max.y { break; }
            let is_major = i % (self.major_lines as i32) == 0;
            let color = if is_major {
                self.color.gamma_multiply(1.8)
            } else {
                self.color
            };
            painter.line_segment(
                [egui::pos2(canvas_rect.min.x, y), egui::pos2(canvas_rect.max.x, y)],
                egui::Stroke::new(if is_major { 1.2 } else { 0.6 }, color)
            );
        }
        if canvas_rect.contains(egui::pos2(canvas_rect.min.x, canvas_rect.min.y)) {
            painter.circle_filled(
                egui::pos2(canvas_rect.min.x, canvas_rect.min.y),
                3.0,
                egui::Color32::from_rgb(255, 100, 100)
            );
        }
    }
}

impl super::ComponentSelection {
    pub fn draw_selection_indicators(&self, ui: &mut egui::Ui, layout: &super::LayoutManager) {
        let painter = ui.painter();
        if let Some(selection_rect) = self.selection_rect {
            painter.rect_stroke(
                selection_rect,
                0.0,
                egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(0, 120, 255, 128))
            );
            painter.rect_filled(
                selection_rect,
                0.0,
                egui::Color32::from_rgba_unmultiplied(0, 120, 255, 32)
            );
        }
        if let Some(hover_idx) = self.hover_component {
            if !self.selected.contains(&hover_idx) {
                if let Some(pos) = layout.positions.get(&hover_idx) {
                    let size = layout.sizes.get(&hover_idx).cloned().unwrap_or(egui::vec2(100.0, 30.0));
                    let rect = egui::Rect::from_min_size(*pos, size);
                    painter.rect_stroke(
                        rect.expand(1.0),
                        1.0,
                        egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(0, 120, 255, 128))
                    );
                }
            }
        }
        for &component_idx in &self.selected {
            if let Some(pos) = layout.positions.get(&component_idx) {
                let size = layout.sizes.get(&component_idx).cloned().unwrap_or(egui::vec2(100.0, 30.0));
                let rect = egui::Rect::from_min_size(*pos, size);
                let is_primary = Some(component_idx) == self.primary;
                let selection_color = if is_primary {
                    egui::Color32::from_rgb(0, 120, 255)
                } else {
                    egui::Color32::from_rgb(120, 120, 255)
                };
                painter.rect_stroke(
                    rect.expand(2.0),
                    2.0,
                    egui::Stroke::new(2.0, selection_color)
                );
                if is_primary {
                    Self::draw_resize_handles(&painter, rect);
                }
                // Drag shadow (optional, can be moved to drag logic)
            }
        }
    }

    fn draw_resize_handles(painter: &egui::Painter, rect: egui::Rect) {
        let handle_size = 6.0;
        let handle_color = egui::Color32::from_rgb(0, 120, 255);
        let handles = [
            rect.min,
            egui::pos2(rect.max.x, rect.min.y),
            rect.max,
            egui::pos2(rect.min.x, rect.max.y),
            egui::pos2(rect.center().x, rect.min.y),
            egui::pos2(rect.max.x, rect.center().y),
            egui::pos2(rect.center().x, rect.max.y),
            egui::pos2(rect.min.x, rect.center().y),
        ];
        for handle_pos in handles {
            let handle_rect = egui::Rect::from_center_size(handle_pos, egui::vec2(handle_size, handle_size));
            painter.rect_filled(handle_rect, 1.0, handle_color);
            painter.rect_stroke(handle_rect, 1.0, egui::Stroke::new(1.0, egui::Color32::WHITE));
        }
    }
}
/// Visual guides and rulers system with smart alignment features
pub struct GuideSystem {
    /// Horizontal guides
    pub horizontal_guides: Vec<f32>,
    /// Vertical guides
    pub vertical_guides: Vec<f32>,
    /// Whether rulers are visible
    pub rulers_visible: bool,
    /// Ruler color
    pub ruler_color: egui::Color32,
    /// Guide color
    pub guide_color: egui::Color32,
    /// Smart alignment guides (temporary during drag operations)
    pub smart_guides: SmartGuides,
    /// Distance measurement display
    pub show_distances: bool,
}

/// Smart alignment guides for enhanced visual design
pub struct SmartGuides {
    /// Active alignment guides during drag operations
    pub active_guides: Vec<AlignmentGuide>,
    /// Distance measurements between components
    pub distance_guides: Vec<DistanceGuide>,
    /// Snap threshold in pixels
    pub snap_threshold: f32,
}

/// Individual alignment guide
#[derive(Clone)]
pub struct AlignmentGuide {
    /// Position of the guide line
    pub position: f32,
    /// Whether this is a horizontal or vertical guide
    pub is_horizontal: bool,
    /// Type of alignment (center, edge, etc.)
    pub guide_type: AlignmentType,
    /// Color for this specific guide
    pub color: egui::Color32,
}

/// Distance measurement guide
#[derive(Clone)]
pub struct DistanceGuide {
    /// Start point of measurement
    pub start: egui::Pos2,
    /// End point of measurement
    pub end: egui::Pos2,
    /// Distance value in pixels
    pub distance: f32,
    /// Display label for the distance
    pub label: String,
}

/// Types of alignment guides
#[derive(Clone, PartialEq)]
pub enum AlignmentType {
    /// Align to left edge
    LeftEdge,
    /// Align to right edge
    RightEdge,
    /// Align to horizontal center
    HorizontalCenter,
    /// Align to top edge
    TopEdge,
    /// Align to bottom edge
    BottomEdge,
    /// Align to vertical center
    VerticalCenter,
    /// Align to baseline (for text)
    Baseline,
}

impl Default for GuideSystem {
    fn default() -> Self {
        Self {
            horizontal_guides: Vec::new(),
            vertical_guides: Vec::new(),
            rulers_visible: true,
            ruler_color: egui::Color32::from_rgb(100, 100, 100),
            guide_color: egui::Color32::from_rgb(0, 150, 255),
            smart_guides: SmartGuides::default(),
            show_distances: true,
        }
    }
}

impl Default for SmartGuides {
    fn default() -> Self {
        Self {
            active_guides: Vec::new(),
            distance_guides: Vec::new(),
            snap_threshold: 5.0,
        }
    }
}

impl GuideSystem {
    /// Generate smart alignment guides for a dragging component
    pub fn generate_smart_guides(&mut self, 
        dragging_rect: egui::Rect, 
        other_rects: &[(usize, egui::Rect)],
        form_rect: egui::Rect
    ) {
        self.smart_guides.active_guides.clear();
        self.smart_guides.distance_guides.clear();
        
        // Generate alignment guides from other components
        for (_idx, other_rect) in other_rects {
            self.generate_alignment_guides_from_rect(dragging_rect, *other_rect);
        }
        
        // Generate alignment guides from form bounds
        self.generate_form_alignment_guides(dragging_rect, form_rect);
        
        // Generate distance measurements
        self.generate_distance_guides(dragging_rect, other_rects);
    }
    
    /// Generate alignment guides from a single component
    fn generate_alignment_guides_from_rect(&mut self, dragging_rect: egui::Rect, target_rect: egui::Rect) {
        let snap_threshold = self.smart_guides.snap_threshold;
        
        // Vertical alignment guides
        let left_distance = (dragging_rect.left() - target_rect.left()).abs();
        let right_distance = (dragging_rect.right() - target_rect.right()).abs();
        let h_center_distance = (dragging_rect.center().x - target_rect.center().x).abs();
        
        if left_distance <= snap_threshold {
            self.smart_guides.active_guides.push(AlignmentGuide {
                position: target_rect.left(),
                is_horizontal: false,
                guide_type: AlignmentType::LeftEdge,
                color: egui::Color32::from_rgb(255, 100, 100),
            });
        }
        
        if right_distance <= snap_threshold {
            self.smart_guides.active_guides.push(AlignmentGuide {
                position: target_rect.right(),
                is_horizontal: false,
                guide_type: AlignmentType::RightEdge,
                color: egui::Color32::from_rgb(255, 100, 100),
            });
        }
        
        if h_center_distance <= snap_threshold {
            self.smart_guides.active_guides.push(AlignmentGuide {
                position: target_rect.center().x,
                is_horizontal: false,
                guide_type: AlignmentType::HorizontalCenter,
                color: egui::Color32::from_rgb(100, 255, 100),
            });
        }
        
        // Horizontal alignment guides
        let top_distance = (dragging_rect.top() - target_rect.top()).abs();
        let bottom_distance = (dragging_rect.bottom() - target_rect.bottom()).abs();
        let v_center_distance = (dragging_rect.center().y - target_rect.center().y).abs();
        
        if top_distance <= snap_threshold {
            self.smart_guides.active_guides.push(AlignmentGuide {
                position: target_rect.top(),
                is_horizontal: true,
                guide_type: AlignmentType::TopEdge,
                color: egui::Color32::from_rgb(255, 100, 100),
            });
        }
        
        if bottom_distance <= snap_threshold {
            self.smart_guides.active_guides.push(AlignmentGuide {
                position: target_rect.bottom(),
                is_horizontal: true,
                guide_type: AlignmentType::BottomEdge,
                color: egui::Color32::from_rgb(255, 100, 100),
            });
        }
        
        if v_center_distance <= snap_threshold {
            self.smart_guides.active_guides.push(AlignmentGuide {
                position: target_rect.center().y,
                is_horizontal: true,
                guide_type: AlignmentType::VerticalCenter,
                color: egui::Color32::from_rgb(100, 255, 100),
            });
        }
    }
    
    /// Generate alignment guides from form bounds
    fn generate_form_alignment_guides(&mut self, dragging_rect: egui::Rect, form_rect: egui::Rect) {
        let snap_threshold = self.smart_guides.snap_threshold;
        
        // Form center alignment
        let h_center_distance = (dragging_rect.center().x - form_rect.center().x).abs();
        let v_center_distance = (dragging_rect.center().y - form_rect.center().y).abs();
        
        if h_center_distance <= snap_threshold {
            self.smart_guides.active_guides.push(AlignmentGuide {
                position: form_rect.center().x,
                is_horizontal: false,
                guide_type: AlignmentType::HorizontalCenter,
                color: egui::Color32::from_rgb(100, 100, 255),
            });
        }
        
        if v_center_distance <= snap_threshold {
            self.smart_guides.active_guides.push(AlignmentGuide {
                position: form_rect.center().y,
                is_horizontal: true,
                guide_type: AlignmentType::VerticalCenter,
                color: egui::Color32::from_rgb(100, 100, 255),
            });
        }
    }
    
    /// Generate distance measurement guides
    fn generate_distance_guides(&mut self, dragging_rect: egui::Rect, other_rects: &[(usize, egui::Rect)]) {
        for (_idx, other_rect) in other_rects {
            // Horizontal distance
            if dragging_rect.top() <= other_rect.bottom() && dragging_rect.bottom() >= other_rect.top() {
                let distance = if dragging_rect.right() < other_rect.left() {
                    other_rect.left() - dragging_rect.right()
                } else if other_rect.right() < dragging_rect.left() {
                    dragging_rect.left() - other_rect.right()
                } else {
                    0.0 // Overlapping
                };
                
                if distance > 0.0 && distance < 100.0 {
                    let start = if dragging_rect.right() < other_rect.left() {
                        egui::pos2(dragging_rect.right(), dragging_rect.center().y)
                    } else {
                        egui::pos2(other_rect.right(), other_rect.center().y)
                    };
                    let end = if dragging_rect.right() < other_rect.left() {
                        egui::pos2(other_rect.left(), other_rect.center().y)
                    } else {
                        egui::pos2(dragging_rect.left(), dragging_rect.center().y)
                    };
                    
                    self.smart_guides.distance_guides.push(DistanceGuide {
                        start,
                        end,
                        distance,
                        label: format!("{:.0}px", distance),
                    });
                }
            }
            
            // Vertical distance
            if dragging_rect.left() <= other_rect.right() && dragging_rect.right() >= other_rect.left() {
                let distance = if dragging_rect.bottom() < other_rect.top() {
                    other_rect.top() - dragging_rect.bottom()
                } else if other_rect.bottom() < dragging_rect.top() {
                    dragging_rect.top() - other_rect.bottom()
                } else {
                    0.0 // Overlapping
                };
                
                if distance > 0.0 && distance < 100.0 {
                    let start = if dragging_rect.bottom() < other_rect.top() {
                        egui::pos2(dragging_rect.center().x, dragging_rect.bottom())
                    } else {
                        egui::pos2(other_rect.center().x, other_rect.bottom())
                    };
                    let end = if dragging_rect.bottom() < other_rect.top() {
                        egui::pos2(other_rect.center().x, other_rect.top())
                    } else {
                        egui::pos2(dragging_rect.center().x, dragging_rect.top())
                    };
                    
                    self.smart_guides.distance_guides.push(DistanceGuide {
                        start,
                        end,
                        distance,
                        label: format!("{:.0}px", distance),
                    });
                }
            }
        }
    }
    
    /// Draw smart alignment guides
    pub fn draw_smart_guides(&self, ui: &mut egui::Ui, canvas_rect: egui::Rect) {
        let painter = ui.painter();
        
        // Draw alignment guides
        for guide in &self.smart_guides.active_guides {
            if guide.is_horizontal {
                painter.line_segment(
                    [egui::pos2(canvas_rect.min.x, guide.position), egui::pos2(canvas_rect.max.x, guide.position)],
                    egui::Stroke::new(1.5, guide.color)
                );
            } else {
                painter.line_segment(
                    [egui::pos2(guide.position, canvas_rect.min.y), egui::pos2(guide.position, canvas_rect.max.y)],
                    egui::Stroke::new(1.5, guide.color)
                );
            }
        }
        
        // Draw distance guides
        if self.show_distances {
            for distance_guide in &self.smart_guides.distance_guides {
                // Draw measurement line
                painter.line_segment(
                    [distance_guide.start, distance_guide.end],
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 150, 0))
                );
                
                // Draw end caps
                let cap_size = 3.0;
                painter.circle_filled(distance_guide.start, cap_size, egui::Color32::from_rgb(255, 150, 0));
                painter.circle_filled(distance_guide.end, cap_size, egui::Color32::from_rgb(255, 150, 0));
                
                // Draw distance label
                let mid_point = egui::pos2(
                    (distance_guide.start.x + distance_guide.end.x) / 2.0,
                    (distance_guide.start.y + distance_guide.end.y) / 2.0
                );
                
                // Background for text
                let text_size = painter.layout_no_wrap(
                    distance_guide.label.clone(),
                    egui::FontId::proportional(10.0),
                    egui::Color32::WHITE
                ).size();
                
                let text_rect = egui::Rect::from_center_size(mid_point, text_size + egui::vec2(4.0, 2.0));
                painter.rect_filled(text_rect, 2.0, egui::Color32::from_rgba_unmultiplied(0, 0, 0, 180));
                
                painter.text(
                    mid_point,
                    egui::Align2::CENTER_CENTER,
                    &distance_guide.label,
                    egui::FontId::proportional(10.0),
                    egui::Color32::WHITE
                );
            }
        }
    }
    
    /// Get snap position for alignment
    pub fn get_snap_position(&self, original_pos: egui::Pos2, dragging_size: egui::Vec2) -> egui::Pos2 {
        let mut snapped_pos = original_pos;
        
        // Find the closest alignment guide
        for guide in &self.smart_guides.active_guides {
            if !guide.is_horizontal {
                // Vertical guide - adjust X position
                let target_x = match guide.guide_type {
                    AlignmentType::LeftEdge => guide.position,
                    AlignmentType::RightEdge => guide.position - dragging_size.x,
                    AlignmentType::HorizontalCenter => guide.position - dragging_size.x / 2.0,
                    _ => continue,
                };
                
                if (original_pos.x - target_x).abs() <= self.smart_guides.snap_threshold {
                    snapped_pos.x = target_x;
                }
            } else {
                // Horizontal guide - adjust Y position
                let target_y = match guide.guide_type {
                    AlignmentType::TopEdge => guide.position,
                    AlignmentType::BottomEdge => guide.position - dragging_size.y,
                    AlignmentType::VerticalCenter => guide.position - dragging_size.y / 2.0,
                    _ => continue,
                };
                
                if (original_pos.y - target_y).abs() <= self.smart_guides.snap_threshold {
                    snapped_pos.y = target_y;
                }
            }
        }
        
        snapped_pos
    }
    
    /// Clear all smart guides
    pub fn clear_smart_guides(&mut self) {
        self.smart_guides.active_guides.clear();
        self.smart_guides.distance_guides.clear();
    }
}
