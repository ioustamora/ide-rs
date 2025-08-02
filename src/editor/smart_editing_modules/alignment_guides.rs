//! Alignment Guide System
//!
//! This module provides intelligent alignment guides that help users
//! position components with precision and consistency.

use egui::*;
use std::collections::HashMap;

/// Alignment guide for visual feedback
#[derive(Clone, Debug)]
pub struct AlignmentGuide {
    /// Guide position (x or y coordinate)
    pub position: f32,
    /// Guide direction
    pub direction: GuideDirection,
    /// Components that align to this guide
    pub aligned_components: Vec<usize>,
    /// Guide strength (visual prominence)
    pub strength: f32,
    /// Guide type for different styling
    pub guide_type: AlignmentType,
    /// Whether this guide is currently active
    pub active: bool,
}

/// Guide direction enumeration
#[derive(Clone, Debug, PartialEq)]
pub enum GuideDirection {
    Horizontal,
    Vertical,
}

/// Alignment type for different guide styles
#[derive(Clone, Debug, PartialEq)]
pub enum AlignmentType {
    /// Component edge alignment
    Edge,
    /// Component center alignment
    Center,
    /// Grid-based alignment
    Grid,
    /// Custom alignment
    Custom,
}

/// Alignment guide manager
pub struct AlignmentGuideManager {
    /// Active alignment guides
    pub guides: Vec<AlignmentGuide>,
    /// Guide detection threshold
    pub detection_threshold: f32,
    /// Guide visual style settings
    pub style: GuideStyle,
    /// Whether guides are enabled
    pub enabled: bool,
}

/// Visual style configuration for guides
#[derive(Clone, Debug)]
pub struct GuideStyle {
    /// Guide line color
    pub line_color: Color32,
    /// Guide line width
    pub line_width: f32,
    /// Guide line style
    pub line_style: LineStyle,
    /// Text color for guide labels
    pub text_color: Color32,
    /// Font size for guide labels
    pub font_size: f32,
}

/// Line style for guides
#[derive(Clone, Debug, PartialEq)]
pub enum LineStyle {
    Solid,
    Dashed,
    Dotted,
}

impl Default for AlignmentGuideManager {
    fn default() -> Self {
        Self {
            guides: Vec::new(),
            detection_threshold: 5.0,
            style: GuideStyle::default(),
            enabled: true,
        }
    }
}

impl Default for GuideStyle {
    fn default() -> Self {
        Self {
            line_color: Color32::from_rgb(0, 150, 255),
            line_width: 1.0,
            line_style: LineStyle::Solid,
            text_color: Color32::from_rgb(0, 100, 200),
            font_size: 12.0,
        }
    }
}

impl AlignmentGuideManager {
    /// Create a new alignment guide manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Update guides based on component positions
    pub fn update_guides(&mut self, component_positions: &HashMap<usize, Pos2>, component_sizes: &HashMap<usize, Vec2>) {
        if !self.enabled {
            return;
        }
        
        self.guides.clear();
        
        // Generate horizontal guides
        self.generate_horizontal_guides(component_positions, component_sizes);
        
        // Generate vertical guides
        self.generate_vertical_guides(component_positions, component_sizes);
        
        // Remove duplicate or overlapping guides
        self.deduplicate_guides();
    }
    
    /// Generate horizontal alignment guides
    fn generate_horizontal_guides(&mut self, positions: &HashMap<usize, Pos2>, sizes: &HashMap<usize, Vec2>) {
        let mut y_positions: Vec<(f32, usize, AlignmentType)> = Vec::new();
        
        for (&component_id, &pos) in positions {
            let size = sizes.get(&component_id).copied().unwrap_or(Vec2::ZERO);
            
            // Top edge
            y_positions.push((pos.y, component_id, AlignmentType::Edge));
            // Center
            y_positions.push((pos.y + size.y / 2.0, component_id, AlignmentType::Center));
            // Bottom edge
            y_positions.push((pos.y + size.y, component_id, AlignmentType::Edge));
        }
        
        // Group similar Y positions
        y_positions.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        
        let mut i = 0;
        while i < y_positions.len() {
            let mut group = vec![y_positions[i]];
            let base_y = y_positions[i].0;
            
            // Find all positions within threshold
            let mut j = i + 1;
            while j < y_positions.len() && (y_positions[j].0 - base_y).abs() <= self.detection_threshold {
                group.push(y_positions[j]);
                j += 1;
            }
            
            // Create guide if we have multiple components
            if group.len() > 1 {
                let avg_y = group.iter().map(|(y, _, _)| *y).sum::<f32>() / group.len() as f32;
                let aligned_components: Vec<usize> = group.iter().map(|(_, id, _)| *id).collect();
                let guide_type = group[0].2.clone();
                
                self.guides.push(AlignmentGuide {
                    position: avg_y,
                    direction: GuideDirection::Horizontal,
                    aligned_components,
                    strength: group.len() as f32,
                    guide_type,
                    active: false,
                });
            }
            
            i = j;
        }
    }
    
    /// Generate vertical alignment guides
    fn generate_vertical_guides(&mut self, positions: &HashMap<usize, Pos2>, sizes: &HashMap<usize, Vec2>) {
        let mut x_positions: Vec<(f32, usize, AlignmentType)> = Vec::new();
        
        for (&component_id, &pos) in positions {
            let size = sizes.get(&component_id).copied().unwrap_or(Vec2::ZERO);
            
            // Left edge
            x_positions.push((pos.x, component_id, AlignmentType::Edge));
            // Center
            x_positions.push((pos.x + size.x / 2.0, component_id, AlignmentType::Center));
            // Right edge
            x_positions.push((pos.x + size.x, component_id, AlignmentType::Edge));
        }
        
        // Group similar X positions
        x_positions.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        
        let mut i = 0;
        while i < x_positions.len() {
            let mut group = vec![x_positions[i]];
            let base_x = x_positions[i].0;
            
            // Find all positions within threshold
            let mut j = i + 1;
            while j < x_positions.len() && (x_positions[j].0 - base_x).abs() <= self.detection_threshold {
                group.push(x_positions[j]);
                j += 1;
            }
            
            // Create guide if we have multiple components
            if group.len() > 1 {
                let avg_x = group.iter().map(|(x, _, _)| *x).sum::<f32>() / group.len() as f32;
                let aligned_components: Vec<usize> = group.iter().map(|(_, id, _)| *id).collect();
                let guide_type = group[0].2.clone();
                
                self.guides.push(AlignmentGuide {
                    position: avg_x,
                    direction: GuideDirection::Vertical,
                    aligned_components,
                    strength: group.len() as f32,
                    guide_type,
                    active: false,
                });
            }
            
            i = j;
        }
    }
    
    /// Remove duplicate or overlapping guides
    fn deduplicate_guides(&mut self) {
        // Sort guides by position within each direction
        self.guides.sort_by(|a, b| {
            match a.direction.cmp(&b.direction) {
                std::cmp::Ordering::Equal => a.position.partial_cmp(&b.position).unwrap(),
                other => other,
            }
        });
        
        // Remove duplicates
        self.guides.dedup_by(|a, b| {
            a.direction == b.direction && 
            (a.position - b.position).abs() < 1.0 &&
            a.guide_type == b.guide_type
        });
    }
    
    /// Find the closest guide to a position
    pub fn find_closest_guide(&self, pos: Pos2, direction: GuideDirection) -> Option<&AlignmentGuide> {
        self.guides
            .iter()
            .filter(|guide| guide.direction == direction)
            .min_by(|a, b| {
                let dist_a = match direction {
                    GuideDirection::Horizontal => (a.position - pos.y).abs(),
                    GuideDirection::Vertical => (a.position - pos.x).abs(),
                };
                let dist_b = match direction {
                    GuideDirection::Horizontal => (b.position - pos.y).abs(),
                    GuideDirection::Vertical => (b.position - pos.x).abs(),
                };
                dist_a.partial_cmp(&dist_b).unwrap()
            })
    }
    
    /// Snap position to nearest guide
    pub fn snap_to_guide(&self, mut pos: Pos2) -> Pos2 {
        if !self.enabled {
            return pos;
        }
        
        // Snap to horizontal guide
        if let Some(guide) = self.find_closest_guide(pos, GuideDirection::Horizontal) {
            if (guide.position - pos.y).abs() <= self.detection_threshold {
                pos.y = guide.position;
            }
        }
        
        // Snap to vertical guide
        if let Some(guide) = self.find_closest_guide(pos, GuideDirection::Vertical) {
            if (guide.position - pos.x).abs() <= self.detection_threshold {
                pos.x = guide.position;
            }
        }
        
        pos
    }
    
    /// Render alignment guides
    pub fn render_guides(&self, ui: &mut Ui, canvas_rect: Rect) {
        if !self.enabled {
            return;
        }
        
        let painter = ui.painter();
        
        for guide in &self.guides {
            if !guide.active {
                continue;
            }
            
            let color = Color32::from_rgba_premultiplied(
                self.style.line_color.r(),
                self.style.line_color.g(),
                self.style.line_color.b(),
                (255.0 * (guide.strength / 5.0).min(1.0)) as u8,
            );
            
            let stroke = Stroke::new(self.style.line_width, color);
            
            match guide.direction {
                GuideDirection::Horizontal => {
                    let y = guide.position;
                    if y >= canvas_rect.min.y && y <= canvas_rect.max.y {
                        match self.style.line_style {
                            LineStyle::Solid => {
                                painter.line_segment(
                                    [Pos2::new(canvas_rect.min.x, y), Pos2::new(canvas_rect.max.x, y)],
                                    stroke,
                                );
                            }
                            LineStyle::Dashed => {
                                self.draw_dashed_line(painter, 
                                    Pos2::new(canvas_rect.min.x, y), 
                                    Pos2::new(canvas_rect.max.x, y), 
                                    stroke
                                );
                            }
                            LineStyle::Dotted => {
                                self.draw_dotted_line(painter, 
                                    Pos2::new(canvas_rect.min.x, y), 
                                    Pos2::new(canvas_rect.max.x, y), 
                                    stroke
                                );
                            }
                        }
                    }
                }
                GuideDirection::Vertical => {
                    let x = guide.position;
                    if x >= canvas_rect.min.x && x <= canvas_rect.max.x {
                        match self.style.line_style {
                            LineStyle::Solid => {
                                painter.line_segment(
                                    [Pos2::new(x, canvas_rect.min.y), Pos2::new(x, canvas_rect.max.y)],
                                    stroke,
                                );
                            }
                            LineStyle::Dashed => {
                                self.draw_dashed_line(painter, 
                                    Pos2::new(x, canvas_rect.min.y), 
                                    Pos2::new(x, canvas_rect.max.y), 
                                    stroke
                                );
                            }
                            LineStyle::Dotted => {
                                self.draw_dotted_line(painter, 
                                    Pos2::new(x, canvas_rect.min.y), 
                                    Pos2::new(x, canvas_rect.max.y), 
                                    stroke
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    
    /// Draw dashed line
    fn draw_dashed_line(&self, painter: &Painter, start: Pos2, end: Pos2, stroke: Stroke) {
        let direction = (end - start).normalized();
        let length = (end - start).length();
        let dash_length = 8.0;
        let gap_length = 4.0;
        let cycle_length = dash_length + gap_length;
        
        let mut current_pos = start;
        let mut distance = 0.0;
        
        while distance < length {
            let remaining = length - distance;
            let segment_length = dash_length.min(remaining);
            let segment_end = current_pos + direction * segment_length;
            
            painter.line_segment([current_pos, segment_end], stroke);
            
            distance += cycle_length;
            current_pos = start + direction * distance;
        }
    }
    
    /// Draw dotted line
    fn draw_dotted_line(&self, painter: &Painter, start: Pos2, end: Pos2, stroke: Stroke) {
        let direction = (end - start).normalized();
        let length = (end - start).length();
        let dot_spacing = 4.0;
        let dot_count = (length / dot_spacing) as usize;
        
        for i in 0..dot_count {
            let t = i as f32 / (dot_count - 1) as f32;
            let pos = start + (end - start) * t;
            painter.circle_filled(pos, 1.0, stroke.color);
        }
    }
    
    /// Activate guides near a position
    pub fn activate_guides_near(&mut self, pos: Pos2) {
        for guide in &mut self.guides {
            let distance = match guide.direction {
                GuideDirection::Horizontal => (guide.position - pos.y).abs(),
                GuideDirection::Vertical => (guide.position - pos.x).abs(),
            };
            
            guide.active = distance <= self.detection_threshold * 2.0;
        }
    }
    
    /// Deactivate all guides
    pub fn deactivate_all_guides(&mut self) {
        for guide in &mut self.guides {
            guide.active = false;
        }
    }
    
    /// Get guide statistics
    pub fn get_statistics(&self) -> GuideStatistics {
        let horizontal_count = self.guides.iter().filter(|g| g.direction == GuideDirection::Horizontal).count();
        let vertical_count = self.guides.iter().filter(|g| g.direction == GuideDirection::Vertical).count();
        let active_count = self.guides.iter().filter(|g| g.active).count();
        
        GuideStatistics {
            total_guides: self.guides.len(),
            horizontal_guides: horizontal_count,
            vertical_guides: vertical_count,
            active_guides: active_count,
        }
    }
}

/// Guide statistics for debugging and analytics
#[derive(Debug, Clone)]
pub struct GuideStatistics {
    pub total_guides: usize,
    pub horizontal_guides: usize,
    pub vertical_guides: usize,
    pub active_guides: usize,
}