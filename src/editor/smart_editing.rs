//! Smart Editing Features for Visual Designer
//!
//! This module provides intelligent editing assistance including:
//! - Auto-alignment guides and snapping
//! - Component magnetism for precise positioning
//! - Smart spacing and distribution
//! - Visual guides for better alignment

use egui::*;
use crate::editor::visual_designer::VisualDesigner;
use std::collections::HashMap;

/// Smart editing system for enhanced visual design experience
pub struct SmartEditingSystem {
    /// Whether smart guides are enabled
    pub guides_enabled: bool,
    /// Whether component magnetism is enabled
    pub magnetism_enabled: bool,
    /// Magnetism strength (snap distance in pixels)
    pub magnetism_strength: f32,
    /// Smart guides for alignment
    pub alignment_guides: Vec<AlignmentGuide>,
    /// Spacing suggestions
    pub spacing_guides: Vec<SpacingGuide>,
    /// Component magnetism zones
    pub magnet_zones: Vec<MagnetZone>,
    /// Auto-alignment threshold in pixels
    pub alignment_threshold: f32,
    /// History of recent guide activations for learning
    pub guide_history: Vec<GuideActivation>,
}

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

/// Spacing guide for consistent component spacing
#[derive(Clone, Debug)]
pub struct SpacingGuide {
    /// Start position of the spacing
    pub start: Pos2,
    /// End position of the spacing
    pub end: Pos2,
    /// Recommended spacing distance
    pub spacing: f32,
    /// Components involved in this spacing
    pub components: (usize, usize),
    /// Confidence level of the suggestion
    pub confidence: f32,
}

/// Magnetism zone for component snapping
#[derive(Clone, Debug)]
pub struct MagnetZone {
    /// Center position of the magnetic zone
    pub center: Pos2,
    /// Zone radius
    pub radius: f32,
    /// Magnetic strength
    pub strength: f32,
    /// Target snap position
    pub snap_position: Pos2,
    /// Zone type
    pub zone_type: MagnetType,
}

/// Direction of alignment guides
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GuideDirection {
    Horizontal,
    Vertical,
}

/// Types of alignment for different visual styling
#[derive(Clone, Copy, Debug)]
pub enum AlignmentType {
    /// Alignment to component edges
    ComponentEdge,
    /// Alignment to component centers
    ComponentCenter,
    /// Alignment to canvas boundaries
    CanvasEdge,
    /// Custom grid alignment
    GridLine,
}

/// Types of magnetic zones
#[derive(Clone, Copy, Debug)]
pub enum MagnetType {
    /// Component edge magnetism
    ComponentEdge,
    /// Component center magnetism
    ComponentCenter,
    /// Grid point magnetism
    GridPoint,
    /// Canvas edge magnetism
    CanvasEdge,
}

/// Guide activation for learning user preferences
#[derive(Clone, Debug)]
pub struct GuideActivation {
    /// Type of guide that was used
    pub guide_type: AlignmentType,
    /// Position where it was used
    pub position: Pos2,
    /// Timestamp
    pub timestamp: std::time::Instant,
    /// How long the user kept the alignment
    pub duration: Option<std::time::Duration>,
}

/// Result of smart editing analysis
#[derive(Clone, Debug)]
pub struct SmartEditingResult {
    /// Suggested snap position
    pub snap_position: Option<Pos2>,
    /// Active alignment guides
    pub active_guides: Vec<AlignmentGuide>,
    /// Spacing suggestions
    pub spacing_suggestions: Vec<SpacingGuide>,
    /// Magnetism feedback
    pub magnetism_strength: f32,
}

impl Default for SmartEditingSystem {
    fn default() -> Self {
        Self {
            guides_enabled: true,
            magnetism_enabled: true,
            magnetism_strength: 8.0,
            alignment_guides: Vec::new(),
            spacing_guides: Vec::new(),
            magnet_zones: Vec::new(),
            alignment_threshold: 5.0,
            guide_history: Vec::new(),
        }
    }
}

impl SmartEditingSystem {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Analyze the current editing context and provide smart suggestions
    pub fn analyze_editing_context(&mut self, 
                                  visual_designer: &VisualDesigner,
                                  dragging_component: Option<usize>,
                                  current_position: Pos2) -> SmartEditingResult {
        self.clear_temporary_guides();
        
        if !self.guides_enabled && !self.magnetism_enabled {
            return SmartEditingResult::default();
        }
        
        let mut result = SmartEditingResult::default();
        
        // Generate alignment guides
        if self.guides_enabled {
            self.generate_alignment_guides(visual_designer, dragging_component);
            result.active_guides = self.get_active_guides(current_position);
        }
        
        // Calculate magnetism
        if self.magnetism_enabled {
            if let Some(snap_pos) = self.calculate_magnetism(visual_designer, current_position, dragging_component) {
                result.snap_position = Some(snap_pos);
                result.magnetism_strength = self.calculate_magnetism_strength(current_position, snap_pos);
            }
        }
        
        // Generate spacing suggestions
        if let Some(component_idx) = dragging_component {
            result.spacing_suggestions = self.generate_spacing_suggestions(visual_designer, component_idx, current_position);
        }
        
        result
    }
    
    /// Generate alignment guides based on existing components
    fn generate_alignment_guides(&mut self, visual_designer: &VisualDesigner, dragging_component: Option<usize>) {
        self.alignment_guides.clear();
        
        // Generate guides from existing components
        for (component_idx, position) in &visual_designer.layout.positions {
            // Skip the component being dragged
            if Some(*component_idx) == dragging_component {
                continue;
            }
            
            let size = visual_designer.layout.sizes.get(component_idx)
                .copied().unwrap_or(Vec2::new(100.0, 30.0));
            
            // Left edge guide
            self.alignment_guides.push(AlignmentGuide {
                position: position.x,
                direction: GuideDirection::Vertical,
                aligned_components: vec![*component_idx],
                strength: 1.0,
                guide_type: AlignmentType::ComponentEdge,
                active: false,
            });
            
            // Right edge guide
            self.alignment_guides.push(AlignmentGuide {
                position: position.x + size.x,
                direction: GuideDirection::Vertical,
                aligned_components: vec![*component_idx],
                strength: 1.0,
                guide_type: AlignmentType::ComponentEdge,
                active: false,
            });
            
            // Center vertical guide
            self.alignment_guides.push(AlignmentGuide {
                position: position.x + size.x / 2.0,
                direction: GuideDirection::Vertical,
                aligned_components: vec![*component_idx],
                strength: 0.8,
                guide_type: AlignmentType::ComponentCenter,
                active: false,
            });
            
            // Top edge guide
            self.alignment_guides.push(AlignmentGuide {
                position: position.y,
                direction: GuideDirection::Horizontal,
                aligned_components: vec![*component_idx],
                strength: 1.0,
                guide_type: AlignmentType::ComponentEdge,
                active: false,
            });
            
            // Bottom edge guide
            self.alignment_guides.push(AlignmentGuide {
                position: position.y + size.y,
                direction: GuideDirection::Horizontal,
                aligned_components: vec![*component_idx],
                strength: 1.0,
                guide_type: AlignmentType::ComponentEdge,
                active: false,
            });
            
            // Center horizontal guide
            self.alignment_guides.push(AlignmentGuide {
                position: position.y + size.y / 2.0,
                direction: GuideDirection::Horizontal,
                aligned_components: vec![*component_idx],
                strength: 0.8,
                guide_type: AlignmentType::ComponentCenter,
                active: false,
            });
        }
        
        // Merge nearby guides
        self.merge_nearby_guides();
    }
    
    /// Merge guides that are very close to each other
    /// 
    /// This algorithm optimizes the guide system by consolidating nearby guides into stronger
    /// single guides. This prevents visual clutter and creates more prominent alignment targets
    /// when multiple components create similar guide lines. The strength calculation uses square
    /// root to prevent over-amplification while still boosting important guides.
    fn merge_nearby_guides(&mut self) {
        // Distance threshold for considering guides "nearby" - this prevents merger of
        // guides that are visually distinct but still creates clean consolidation
        let merge_threshold = 2.0;
        let mut merged_guides = Vec::new();
        // Track which guides have been processed to avoid duplicate merging
        let mut processed = vec![false; self.alignment_guides.len()];
        
        // Process each guide as a potential merge target
        for i in 0..self.alignment_guides.len() {
            // Skip guides that have already been merged into other guides
            if processed[i] {
                continue;
            }
            
            // Start with the current guide as the base for merging
            let mut guide = self.alignment_guides[i].clone();
            let mut merge_count = 1;  // Count how many guides we merge (starts with self)
            
            // Look for other guides that are close enough to merge
            for j in (i + 1)..self.alignment_guides.len() {
                // Skip already processed guides
                if processed[j] {
                    continue;
                }
                
                let other_guide = &self.alignment_guides[j];
                // Only merge guides of the same direction (horizontal with horizontal, etc.)
                // and within the merge threshold distance
                if guide.direction == other_guide.direction && 
                   (guide.position - other_guide.position).abs() < merge_threshold {
                    
                    // Merge the guides by averaging their positions (creates balanced result)
                    guide.position = (guide.position + other_guide.position) / 2.0;
                    // Combine the list of aligned components from both guides
                    guide.aligned_components.extend(&other_guide.aligned_components);
                    // Take the stronger of the two guide strengths (preserves importance)
                    guide.strength = guide.strength.max(other_guide.strength);
                    // Track that we've merged another guide
                    merge_count += 1;
                    // Mark the other guide as processed so it won't be considered again
                    processed[j] = true;
                }
            }
            
            // Boost strength based on how many guides were merged
            // Square root prevents over-amplification while still rewarding popular alignment points
            guide.strength *= (merge_count as f32).sqrt();
            // Add the merged guide to our final collection
            merged_guides.push(guide);
            // Mark this guide as processed
            processed[i] = true;
        }
        
        // Replace the original guides with the merged collection
        self.alignment_guides = merged_guides;
    }
    
    /// Get guides that are active for the current position
    fn get_active_guides(&mut self, current_position: Pos2) -> Vec<AlignmentGuide> {
        let mut active_guides = Vec::new();
        
        for guide in &mut self.alignment_guides {
            let distance = match guide.direction {
                GuideDirection::Horizontal => (current_position.y - guide.position).abs(),
                GuideDirection::Vertical => (current_position.x - guide.position).abs(),
            };
            
            if distance <= self.alignment_threshold {
                guide.active = true;
                active_guides.push(guide.clone());
            } else {
                guide.active = false;
            }
        }
        
        // Sort by strength (strongest guides first)
        active_guides.sort_by(|a, b| b.strength.partial_cmp(&a.strength).unwrap_or(std::cmp::Ordering::Equal));
        
        active_guides
    }
    
    /// Calculate magnetism effect and return snap position
    /// 
    /// This is the core magnetism algorithm that creates the "magnetic" feeling when dragging
    /// components near alignment points. It generates invisible magnetic zones around important
    /// points (component edges, centers, grid intersections) and calculates the strongest
    /// attraction to provide smooth, predictable snapping behavior.
    fn calculate_magnetism(&mut self, 
                          visual_designer: &VisualDesigner,
                          current_position: Pos2,
                          dragging_component: Option<usize>) -> Option<Pos2> {
        // Clear previous magnetic zones to recalculate fresh zones for current drag operation
        self.magnet_zones.clear();
        
        // Generate magnetic zones from existing components (object-to-object magnetism)
        for (component_idx, position) in &visual_designer.layout.positions {
            // Skip the component being dragged - it shouldn't be magnetic to itself
            if Some(*component_idx) == dragging_component {
                continue;
            }
            
            // Get component size with fallback to reasonable default
            let size = visual_designer.layout.sizes.get(component_idx)
                .copied().unwrap_or(Vec2::new(100.0, 30.0));
            
            // Create edge magnetism zones - these provide strong alignment points
            // Left edge magnetism (at vertical center of component for better UX)
            self.add_magnet_zone(Pos2::new(position.x, position.y + size.y / 2.0), MagnetType::ComponentEdge);
            // Right edge magnetism (at vertical center)
            self.add_magnet_zone(Pos2::new(position.x + size.x, position.y + size.y / 2.0), MagnetType::ComponentEdge);
            // Top edge magnetism (at horizontal center)
            self.add_magnet_zone(Pos2::new(position.x + size.x / 2.0, position.y), MagnetType::ComponentEdge);
            // Bottom edge magnetism (at horizontal center)
            self.add_magnet_zone(Pos2::new(position.x + size.x / 2.0, position.y + size.y), MagnetType::ComponentEdge);
            
            // Center magnetism zone - useful for center-to-center alignment
            // This creates satisfying center-to-center snapping behavior
            self.add_magnet_zone(Pos2::new(position.x + size.x / 2.0, position.y + size.y / 2.0), MagnetType::ComponentCenter);
        }
        
        // Generate grid magnetism zones if grid snapping is enabled
        {
            let grid_size = visual_designer.grid.size;
            let canvas_size = Vec2::new(800.0, 600.0); // TODO: Get actual canvas size from context
            
            // Create magnetic zones at each grid intersection
            // This provides the classic "snap to grid" behavior that users expect
            for x in (0..=(canvas_size.x as i32)).step_by(grid_size as usize) {
                for y in (0..=(canvas_size.y as i32)).step_by(grid_size as usize) {
                    self.add_magnet_zone(Pos2::new(x as f32, y as f32), MagnetType::GridPoint);
                }
            }
        }
        
        // Find the strongest magnetic attraction using distance-based strength calculation
        let mut best_snap: Option<Pos2> = None;
        let mut best_strength = 0.0;
        
        // Evaluate each magnetic zone to find the most attractive one
        for zone in &self.magnet_zones {
            // Calculate distance from current position to magnetic zone center
            let distance = current_position.distance(zone.center);
            // Only consider zones within their magnetic radius
            if distance <= zone.radius {
                // Calculate magnetic strength - stronger when closer, using linear falloff
                // Strength formula: base_strength * (1 - distance_ratio)
                // This creates smooth attraction that peaks at the center and fades to zero at radius
                let strength = zone.strength * (1.0 - distance / zone.radius);
                // Keep track of the strongest magnetic attraction found so far
                if strength > best_strength {
                    best_strength = strength;
                    best_snap = Some(zone.snap_position);
                }
            }
        }
        
        // Return the snap position of the strongest magnetic zone, or None if no magnetism
        best_snap
    }
    
    /// Add a magnet zone
    fn add_magnet_zone(&mut self, position: Pos2, zone_type: MagnetType) {
        let (radius, strength) = match zone_type {
            MagnetType::ComponentEdge => (self.magnetism_strength, 1.0),
            MagnetType::ComponentCenter => (self.magnetism_strength * 0.8, 0.8),
            MagnetType::GridPoint => (self.magnetism_strength * 0.6, 0.6),
            MagnetType::CanvasEdge => (self.magnetism_strength * 1.2, 1.2),
        };
        
        self.magnet_zones.push(MagnetZone {
            center: position,
            radius,
            strength,
            snap_position: position,
            zone_type,
        });
    }
    
    /// Calculate magnetism strength for visual feedback
    fn calculate_magnetism_strength(&self, current_pos: Pos2, snap_pos: Pos2) -> f32 {
        let distance = current_pos.distance(snap_pos);
        if distance <= self.magnetism_strength {
            1.0 - (distance / self.magnetism_strength)
        } else {
            0.0
        }
    }
    
    /// Generate spacing suggestions for consistent component spacing
    /// 
    /// This algorithm analyzes the current drag position against existing components to suggest
    /// standard spacing values. It identifies when the user is positioning a component at a
    /// distance that's close to common design spacing values (8px, 16px, etc.) and provides
    /// visual feedback to encourage consistent spacing patterns.
    fn generate_spacing_suggestions(&self, 
                                  visual_designer: &VisualDesigner,
                                  dragging_component: usize,
                                  current_position: Pos2) -> Vec<SpacingGuide> {
        let mut suggestions = Vec::new();
        
        // Standard spacing values commonly used in UI design
        // These follow typical design system spacing scales (8px base unit)
        let common_spacings = [8.0, 16.0, 24.0, 32.0, 48.0, 64.0];
        
        // Analyze spacing against each existing component
        for (component_idx, position) in &visual_designer.layout.positions {
            // Skip the component being dragged - can't have spacing with itself
            if *component_idx == dragging_component {
                continue;
            }
            
            // Get component size with reasonable fallback
            let size = visual_designer.layout.sizes.get(component_idx)
                .copied().unwrap_or(Vec2::new(100.0, 30.0));
            
            // Calculate actual distances from dragged component to this component
            // Horizontal distance: from right edge of existing component to current position
            let horizontal_distance = (current_position.x - (position.x + size.x)).abs();
            // Vertical distance: from bottom edge of existing component to current position
            let vertical_distance = (current_position.y - (position.y + size.y)).abs();
            
            // Check each common spacing value to see if current positioning is close
            for &spacing in &common_spacings {
                // Tolerance for considering a distance "close enough" to suggest
                // 4px tolerance allows for slight imprecision while still being helpful
                let tolerance = 4.0;
                
                // Horizontal spacing suggestion - when components are side-by-side
                if (horizontal_distance - spacing).abs() < tolerance {
                    suggestions.push(SpacingGuide {
                        // Start point: right edge of existing component at its vertical center
                        start: Pos2::new(position.x + size.x, position.y + size.y / 2.0),
                        // End point: current drag position at same vertical level
                        end: Pos2::new(current_position.x, position.y + size.y / 2.0),
                        spacing,  // The suggested spacing value
                        components: (*component_idx, dragging_component),  // Components involved
                        // Confidence decreases as distance from ideal spacing increases
                        confidence: 1.0 - (horizontal_distance - spacing).abs() / tolerance,
                    });
                }
                
                // Vertical spacing suggestion - when components are stacked
                if (vertical_distance - spacing).abs() < tolerance {
                    suggestions.push(SpacingGuide {
                        // Start point: bottom edge of existing component at its horizontal center
                        start: Pos2::new(position.x + size.x / 2.0, position.y + size.y),
                        // End point: current drag position at same horizontal level
                        end: Pos2::new(position.x + size.x / 2.0, current_position.y),
                        spacing,  // The suggested spacing value
                        components: (*component_idx, dragging_component),  // Components involved
                        // Confidence calculation based on how close to ideal spacing
                        confidence: 1.0 - (vertical_distance - spacing).abs() / tolerance,
                    });
                }
            }
        }
        
        // Sort suggestions by confidence - most confident suggestions first
        // This ensures the most relevant spacing guides are shown prominently
        suggestions.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));
        
        // Return only the top 3 suggestions to avoid visual clutter
        // Too many suggestions can be overwhelming and counterproductive
        suggestions.into_iter().take(3).collect()
    }
    
    /// Render smart editing guides and feedback
    pub fn render_guides(&self, ui: &mut Ui, canvas_rect: Rect) {
        let painter = ui.painter();
        
        // Render alignment guides
        for guide in &self.alignment_guides {
            if guide.active {
                self.render_alignment_guide(&painter, guide, canvas_rect);
            }
        }
        
        // Render spacing guides
        for spacing in &self.spacing_guides {
            self.render_spacing_guide(&painter, spacing);
        }
        
        // Render magnetism feedback
        self.render_magnetism_feedback(&painter);
    }
    
    /// Render an alignment guide
    fn render_alignment_guide(&self, painter: &egui::Painter, guide: &AlignmentGuide, canvas_rect: Rect) {
        let color = match guide.guide_type {
            AlignmentType::ComponentEdge => Color32::from_rgba_unmultiplied(255, 100, 100, (150.0 * guide.strength) as u8),
            AlignmentType::ComponentCenter => Color32::from_rgba_unmultiplied(100, 255, 100, (120.0 * guide.strength) as u8),
            AlignmentType::CanvasEdge => Color32::from_rgba_unmultiplied(100, 100, 255, (180.0 * guide.strength) as u8),
            AlignmentType::GridLine => Color32::from_rgba_unmultiplied(200, 200, 200, (100.0 * guide.strength) as u8),
        };
        
        let stroke = Stroke::new(1.0 + guide.strength, color);
        
        match guide.direction {
            GuideDirection::Horizontal => {
                painter.line_segment(
                    [Pos2::new(canvas_rect.min.x, guide.position), Pos2::new(canvas_rect.max.x, guide.position)],
                    stroke,
                );
            }
            GuideDirection::Vertical => {
                painter.line_segment(
                    [Pos2::new(guide.position, canvas_rect.min.y), Pos2::new(guide.position, canvas_rect.max.y)],
                    stroke,
                );
            }
        }
    }
    
    /// Render a spacing guide
    fn render_spacing_guide(&self, painter: &egui::Painter, spacing: &SpacingGuide) {
        let color = Color32::from_rgba_unmultiplied(255, 165, 0, (200.0 * spacing.confidence) as u8);
        let stroke = Stroke::new(1.5, color);
        
        // Draw the spacing line
        painter.line_segment([spacing.start, spacing.end], stroke);
        
        // Draw arrows at the ends
        let direction = (spacing.end - spacing.start).normalized();
        let arrow_size = 5.0;
        let perpendicular = Vec2::new(-direction.y, direction.x) * arrow_size;
        
        // Start arrow
        painter.line_segment(
            [spacing.start, spacing.start + direction * arrow_size + perpendicular],
            stroke,
        );
        painter.line_segment(
            [spacing.start, spacing.start + direction * arrow_size - perpendicular],
            stroke,
        );
        
        // End arrow
        painter.line_segment(
            [spacing.end, spacing.end - direction * arrow_size + perpendicular],
            stroke,
        );
        painter.line_segment(
            [spacing.end, spacing.end - direction * arrow_size - perpendicular],
            stroke,
        );
        
        // Spacing label
        let mid_point = (spacing.start + spacing.end.to_vec2()) / 2.0;
        painter.text(
            mid_point + perpendicular * 0.5,
            Align2::CENTER_CENTER,
            format!("{:.0}px", spacing.spacing),
            FontId::monospace(10.0),
            color,
        );
    }
    
    /// Render magnetism feedback
    fn render_magnetism_feedback(&self, _painter: &egui::Painter) {
        // Magnetism feedback is usually subtle - could show magnet zones during debugging
        // For now, the snapping behavior itself is the primary feedback
    }
    
    /// Clear temporary guides
    fn clear_temporary_guides(&mut self) {
        self.alignment_guides.retain(|guide| !matches!(guide.guide_type, AlignmentType::ComponentEdge | AlignmentType::ComponentCenter));
        self.spacing_guides.clear();
        self.magnet_zones.clear();
    }
    
    /// Apply smart editing result to a position
    pub fn apply_smart_editing(&self, result: &SmartEditingResult, original_position: Pos2) -> Pos2 {
        // Apply magnetism if available
        if let Some(snap_pos) = result.snap_position {
            if result.magnetism_strength > 0.5 {
                return snap_pos;
            }
        }
        
        // Apply alignment guide snapping
        let mut adjusted_position = original_position;
        
        for guide in &result.active_guides {
            if guide.strength > 0.7 {
                match guide.direction {
                    GuideDirection::Horizontal => {
                        adjusted_position.y = guide.position;
                    }
                    GuideDirection::Vertical => {
                        adjusted_position.x = guide.position;
                    }
                }
            }
        }
        
        adjusted_position
    }
    
    /// Configure smart editing settings
    pub fn configure(&mut self, 
                    guides_enabled: bool, 
                    magnetism_enabled: bool,
                    magnetism_strength: f32,
                    alignment_threshold: f32) {
        self.guides_enabled = guides_enabled;
        self.magnetism_enabled = magnetism_enabled;
        self.magnetism_strength = magnetism_strength;
        self.alignment_threshold = alignment_threshold;
    }
    
    /// Learn from user interactions to improve suggestions
    pub fn learn_from_interaction(&mut self, guide_activation: GuideActivation) {
        self.guide_history.push(guide_activation);
        
        // Keep only recent history
        let max_history = 100;
        if self.guide_history.len() > max_history {
            self.guide_history.drain(0..self.guide_history.len() - max_history);
        }
        
        // Analyze patterns to adjust guide preferences
        self.analyze_user_patterns();
    }
    
    /// Add enhanced distribution guides for multiple components
    pub fn generate_distribution_guides(&mut self, visual_designer: &VisualDesigner, selected_components: &[usize]) {
        if selected_components.len() < 3 {
            return;
        }
        
        // Get positions and sizes of selected components
        let mut component_data: Vec<(usize, Pos2, Vec2)> = selected_components
            .iter()
            .filter_map(|&idx| {
                let pos = visual_designer.layout.positions.get(&idx)?;
                let size = visual_designer.layout.sizes.get(&idx).copied().unwrap_or(Vec2::new(100.0, 30.0));
                Some((idx, *pos, size))
            })
            .collect();
        
        if component_data.len() < 3 {
            return;
        }
        
        // Sort by horizontal position for horizontal distribution
        component_data.sort_by(|a, b| a.1.x.partial_cmp(&b.1.x).unwrap_or(std::cmp::Ordering::Equal));
        
        // Calculate even distribution spacing
        let first_pos = component_data[0].1.x;
        let last_pos = component_data[component_data.len() - 1].1.x;
        let total_width: f32 = component_data.iter().map(|(_, _, size)| size.x).sum();
        let available_space = last_pos - first_pos - total_width;
        let ideal_spacing = available_space / (component_data.len() - 1) as f32;
        
        // Generate spacing guides for even distribution
        for i in 1..component_data.len() {
            let prev_component = &component_data[i - 1];
            let current_component = &component_data[i];
            
            let spacing_start = Pos2::new(prev_component.1.x + prev_component.2.x, prev_component.1.y + prev_component.2.y / 2.0);
            let spacing_end = Pos2::new(current_component.1.x, prev_component.1.y + prev_component.2.y / 2.0);
            
            self.spacing_guides.push(SpacingGuide {
                start: spacing_start,
                end: spacing_end,
                spacing: ideal_spacing,
                components: (prev_component.0, current_component.0),
                confidence: 0.9,
            });
        }
    }
    
    /// Enhanced magnetism with object-to-object attraction
    pub fn calculate_enhanced_magnetism(&mut self, 
                                      visual_designer: &VisualDesigner,
                                      current_position: Pos2,
                                      dragging_component: Option<usize>,
                                      dragging_size: Vec2) -> Option<Pos2> {
        self.magnet_zones.clear();
        
        // Generate enhanced magnet zones from components
        for (component_idx, position) in &visual_designer.layout.positions {
            if Some(*component_idx) == dragging_component {
                continue;
            }
            
            let size = visual_designer.layout.sizes.get(component_idx)
                .copied().unwrap_or(Vec2::new(100.0, 30.0));
            
            // Edge-to-edge magnetism (strong attraction)
            self.add_edge_magnetism(*position, size, dragging_size, 1.2);
            
            // Center-to-center magnetism
            let center = *position + size / 2.0;
            self.add_magnet_zone(center, MagnetType::ComponentCenter);
            
            // Alignment magnetism (same x or y coordinate)
            self.add_alignment_magnetism(*position, size);
        }
        
        // Canvas edge magnetism
        self.add_canvas_edge_magnetism(dragging_size);
        
        // Find the strongest magnetic attraction with enhanced scoring
        let mut best_snap: Option<Pos2> = None;
        let mut best_strength = 0.0;
        
        for zone in &self.magnet_zones {
            let distance = current_position.distance(zone.center);
            if distance <= zone.radius {
                let strength = zone.strength * (1.0 - distance / zone.radius);
                
                // Boost strength for edge-to-edge alignment
                let boosted_strength = match zone.zone_type {
                    MagnetType::ComponentEdge => strength * 1.5,
                    _ => strength,
                };
                
                if boosted_strength > best_strength {
                    best_strength = boosted_strength;
                    best_snap = Some(zone.snap_position);
                }
            }
        }
        
        best_snap
    }
    
    /// Add edge-to-edge magnetism zones
    fn add_edge_magnetism(&mut self, target_pos: Pos2, target_size: Vec2, dragging_size: Vec2, strength: f32) {
        let margin = 8.0; // Standard margin between components
        
        // Right edge to left edge (component on the right)
        self.magnet_zones.push(MagnetZone {
            center: Pos2::new(target_pos.x + target_size.x + margin + dragging_size.x / 2.0, target_pos.y + target_size.y / 2.0),
            radius: self.magnetism_strength,
            strength,
            snap_position: Pos2::new(target_pos.x + target_size.x + margin, target_pos.y),
            zone_type: MagnetType::ComponentEdge,
        });
        
        // Left edge to right edge (component on the left)
        self.magnet_zones.push(MagnetZone {
            center: Pos2::new(target_pos.x - margin - dragging_size.x / 2.0, target_pos.y + target_size.y / 2.0),
            radius: self.magnetism_strength,
            strength,
            snap_position: Pos2::new(target_pos.x - margin - dragging_size.x, target_pos.y),
            zone_type: MagnetType::ComponentEdge,
        });
        
        // Bottom edge to top edge (component below)
        self.magnet_zones.push(MagnetZone {
            center: Pos2::new(target_pos.x + target_size.x / 2.0, target_pos.y + target_size.y + margin + dragging_size.y / 2.0),
            radius: self.magnetism_strength,
            strength,
            snap_position: Pos2::new(target_pos.x, target_pos.y + target_size.y + margin),
            zone_type: MagnetType::ComponentEdge,
        });
        
        // Top edge to bottom edge (component above)
        self.magnet_zones.push(MagnetZone {
            center: Pos2::new(target_pos.x + target_size.x / 2.0, target_pos.y - margin - dragging_size.y / 2.0),
            radius: self.magnetism_strength,
            strength,
            snap_position: Pos2::new(target_pos.x, target_pos.y - margin - dragging_size.y),
            zone_type: MagnetType::ComponentEdge,
        });
    }
    
    /// Add alignment-based magnetism
    fn add_alignment_magnetism(&mut self, target_pos: Pos2, target_size: Vec2) {
        // Vertical alignment zones (same X coordinate)
        self.add_magnet_zone(Pos2::new(target_pos.x, 0.0), MagnetType::ComponentEdge); // Left edge
        self.add_magnet_zone(Pos2::new(target_pos.x + target_size.x, 0.0), MagnetType::ComponentEdge); // Right edge
        self.add_magnet_zone(Pos2::new(target_pos.x + target_size.x / 2.0, 0.0), MagnetType::ComponentCenter); // Center
        
        // Horizontal alignment zones (same Y coordinate)
        self.add_magnet_zone(Pos2::new(0.0, target_pos.y), MagnetType::ComponentEdge); // Top edge
        self.add_magnet_zone(Pos2::new(0.0, target_pos.y + target_size.y), MagnetType::ComponentEdge); // Bottom edge
        self.add_magnet_zone(Pos2::new(0.0, target_pos.y + target_size.y / 2.0), MagnetType::ComponentCenter); // Center
    }
    
    /// Add canvas edge magnetism
    fn add_canvas_edge_magnetism(&mut self, dragging_size: Vec2) {
        let canvas_size = Vec2::new(800.0, 600.0); // TODO: Get actual canvas size
        let margin = 16.0; // Margin from canvas edges
        
        // Left edge
        self.magnet_zones.push(MagnetZone {
            center: Pos2::new(margin + dragging_size.x / 2.0, canvas_size.y / 2.0),
            radius: self.magnetism_strength * 1.5,
            strength: 1.0,
            snap_position: Pos2::new(margin, 0.0),
            zone_type: MagnetType::CanvasEdge,
        });
        
        // Right edge
        self.magnet_zones.push(MagnetZone {
            center: Pos2::new(canvas_size.x - margin - dragging_size.x / 2.0, canvas_size.y / 2.0),
            radius: self.magnetism_strength * 1.5,
            strength: 1.0,
            snap_position: Pos2::new(canvas_size.x - margin - dragging_size.x, 0.0),
            zone_type: MagnetType::CanvasEdge,
        });
        
        // Top edge
        self.magnet_zones.push(MagnetZone {
            center: Pos2::new(canvas_size.x / 2.0, margin + dragging_size.y / 2.0),
            radius: self.magnetism_strength * 1.5,
            strength: 1.0,
            snap_position: Pos2::new(0.0, margin),
            zone_type: MagnetType::CanvasEdge,
        });
        
        // Bottom edge
        self.magnet_zones.push(MagnetZone {
            center: Pos2::new(canvas_size.x / 2.0, canvas_size.y - margin - dragging_size.y / 2.0),
            radius: self.magnetism_strength * 1.5,
            strength: 1.0,
            snap_position: Pos2::new(0.0, canvas_size.y - margin - dragging_size.y),
            zone_type: MagnetType::CanvasEdge,
        });
    }
    
    /// Analyze user patterns to improve guide suggestions
    fn analyze_user_patterns(&mut self) {
        if self.guide_history.len() < 10 {
            return;
        }
        
        // Count usage of different guide types
        let mut type_counts = HashMap::new();
        for activation in &self.guide_history {
            *type_counts.entry(format!("{:?}", activation.guide_type)).or_insert(0) += 1;
        }
        
        // Adjust alignment threshold based on usage patterns
        let recent_activations = &self.guide_history[self.guide_history.len().saturating_sub(20)..];
        let avg_precision = recent_activations.len() as f32 / 20.0;
        
        if avg_precision > 0.8 {
            // User is precise, can reduce threshold
            self.alignment_threshold = (self.alignment_threshold * 0.95).max(2.0);
        } else if avg_precision < 0.3 {
            // User needs more help, increase threshold
            self.alignment_threshold = (self.alignment_threshold * 1.05).min(15.0);
        }
    }
}

impl SmartEditingResult {
    fn default() -> Self {
        Self {
            snap_position: None,
            active_guides: Vec::new(),
            spacing_suggestions: Vec::new(),
            magnetism_strength: 0.0,
        }
    }
}