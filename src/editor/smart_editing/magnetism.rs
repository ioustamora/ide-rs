//! Component Magnetism System
//!
//! This module provides magnetic snapping between components for precise
//! positioning and consistent spacing.

use egui::*;
use std::collections::HashMap;

/// Component magnetism zone for snapping
#[derive(Clone, Debug)]
pub struct MagnetZone {
    /// Zone center position
    pub position: Pos2,
    /// Zone radius for magnetic effect
    pub radius: f32,
    /// Zone strength (0.0 to 1.0)
    pub strength: f32,
    /// Components that create this magnetic zone
    pub source_components: Vec<usize>,
    /// Zone type for different behaviors
    pub zone_type: MagnetType,
    /// Whether this zone is currently active
    pub active: bool,
}

/// Type of magnetic zone
#[derive(Clone, Debug, PartialEq)]
pub enum MagnetType {
    /// Edge-to-edge magnetism
    EdgeToEdge,
    /// Center-to-center magnetism
    CenterToCenter,
    /// Corner magnetism
    Corner,
    /// Grid point magnetism
    GridPoint,
    /// Custom magnetism
    Custom,
}

/// Magnetism manager for component snapping
pub struct MagnetismManager {
    /// Active magnetic zones
    pub zones: Vec<MagnetZone>,
    /// Global magnetism strength
    pub global_strength: f32,
    /// Magnetism detection radius
    pub detection_radius: f32,
    /// Whether magnetism is enabled
    pub enabled: bool,
    /// Magnetism preferences
    pub preferences: MagnetismPreferences,
}

/// Magnetism behavior preferences
#[derive(Clone, Debug)]
pub struct MagnetismPreferences {
    /// Enable edge magnetism
    pub edge_magnetism: bool,
    /// Enable center magnetism
    pub center_magnetism: bool,
    /// Enable corner magnetism
    pub corner_magnetism: bool,
    /// Enable grid magnetism
    pub grid_magnetism: bool,
    /// Grid size for grid magnetism
    pub grid_size: f32,
    /// Minimum distance for magnetism to activate
    pub min_distance: f32,
    /// Maximum distance for magnetism to activate
    pub max_distance: f32,
}

/// Result of magnetism calculation
#[derive(Debug, Clone)]
pub struct MagnetismResult {
    /// Original position
    pub original_position: Pos2,
    /// Snapped position
    pub snapped_position: Pos2,
    /// Whether snapping occurred
    pub snapped: bool,
    /// Active magnetic zones that influenced the snap
    pub active_zones: Vec<usize>,
    /// Snap direction
    pub snap_direction: Option<Vec2>,
}

impl Default for MagnetismManager {
    fn default() -> Self {
        Self {
            zones: Vec::new(),
            global_strength: 1.0,
            detection_radius: 15.0,
            enabled: true,
            preferences: MagnetismPreferences::default(),
        }
    }
}

impl Default for MagnetismPreferences {
    fn default() -> Self {
        Self {
            edge_magnetism: true,
            center_magnetism: true,
            corner_magnetism: false,
            grid_magnetism: false,
            grid_size: 20.0,
            min_distance: 2.0,
            max_distance: 30.0,
        }
    }
}

impl MagnetismManager {
    /// Create a new magnetism manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Update magnetic zones based on component positions
    pub fn update_zones(&mut self, positions: &HashMap<usize, Pos2>, sizes: &HashMap<usize, Vec2>) {
        if !self.enabled {
            return;
        }
        
        self.zones.clear();
        
        // Generate magnetic zones for each component
        for (&component_id, &pos) in positions {
            let size = sizes.get(&component_id).copied().unwrap_or(Vec2::ZERO);
            self.generate_zones_for_component(component_id, pos, size);
        }
        
        // Generate grid zones if enabled
        if self.preferences.grid_magnetism {
            self.generate_grid_zones(positions, sizes);
        }
    }
    
    /// Generate magnetic zones for a single component
    fn generate_zones_for_component(&mut self, component_id: usize, pos: Pos2, size: Vec2) {
        let rect = Rect::from_min_size(pos, size);
        
        // Edge magnetism zones
        if self.preferences.edge_magnetism {
            // Left edge
            self.zones.push(MagnetZone {
                position: Pos2::new(rect.min.x, rect.center().y),
                radius: self.detection_radius,
                strength: self.global_strength,
                source_components: vec![component_id],
                zone_type: MagnetType::EdgeToEdge,
                active: false,
            });
            
            // Right edge
            self.zones.push(MagnetZone {
                position: Pos2::new(rect.max.x, rect.center().y),
                radius: self.detection_radius,
                strength: self.global_strength,
                source_components: vec![component_id],
                zone_type: MagnetType::EdgeToEdge,
                active: false,
            });
            
            // Top edge
            self.zones.push(MagnetZone {
                position: Pos2::new(rect.center().x, rect.min.y),
                radius: self.detection_radius,
                strength: self.global_strength,
                source_components: vec![component_id],
                zone_type: MagnetType::EdgeToEdge,
                active: false,
            });
            
            // Bottom edge
            self.zones.push(MagnetZone {
                position: Pos2::new(rect.center().x, rect.max.y),
                radius: self.detection_radius,
                strength: self.global_strength,
                source_components: vec![component_id],
                zone_type: MagnetType::EdgeToEdge,
                active: false,
            });
        }
        
        // Center magnetism zone
        if self.preferences.center_magnetism {
            self.zones.push(MagnetZone {
                position: rect.center(),
                radius: self.detection_radius,
                strength: self.global_strength * 0.8, // Slightly weaker than edge
                source_components: vec![component_id],
                zone_type: MagnetType::CenterToCenter,
                active: false,
            });
        }
        
        // Corner magnetism zones
        if self.preferences.corner_magnetism {
            let corners = [
                rect.min,                                    // Top-left
                Pos2::new(rect.max.x, rect.min.y),         // Top-right
                Pos2::new(rect.min.x, rect.max.y),         // Bottom-left
                rect.max,                                    // Bottom-right
            ];
            
            for &corner in &corners {
                self.zones.push(MagnetZone {
                    position: corner,
                    radius: self.detection_radius * 0.7, // Smaller radius for corners
                    strength: self.global_strength * 0.6, // Weaker than edges
                    source_components: vec![component_id],
                    zone_type: MagnetType::Corner,
                    active: false,
                });
            }
        }
    }
    
    /// Generate grid-based magnetic zones
    fn generate_grid_zones(&mut self, positions: &HashMap<usize, Pos2>, sizes: &HashMap<usize, Vec2>) {
        if positions.is_empty() {
            return;
        }
        
        // Calculate bounding box of all components
        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_y = f32::NEG_INFINITY;
        
        for (&component_id, &pos) in positions {
            let size = sizes.get(&component_id).copied().unwrap_or(Vec2::ZERO);
            min_x = min_x.min(pos.x);
            max_x = max_x.max(pos.x + size.x);
            min_y = min_y.min(pos.y);
            max_y = max_y.max(pos.y + size.y);
        }
        
        // Generate grid points
        let grid_size = self.preferences.grid_size;
        let start_x = (min_x / grid_size).floor() * grid_size;
        let start_y = (min_y / grid_size).floor() * grid_size;
        
        let mut x = start_x;
        while x <= max_x + grid_size {
            let mut y = start_y;
            while y <= max_y + grid_size {
                self.zones.push(MagnetZone {
                    position: Pos2::new(x, y),
                    radius: self.detection_radius * 0.5,
                    strength: self.global_strength * 0.3, // Much weaker than component zones
                    source_components: vec![], // No source component for grid
                    zone_type: MagnetType::GridPoint,
                    active: false,
                });
                y += grid_size;
            }
            x += grid_size;
        }
    }
    
    /// Apply magnetism to a position
    pub fn apply_magnetism(&self, pos: Pos2, moving_component_id: Option<usize>) -> MagnetismResult {
        if !self.enabled {
            return MagnetismResult {
                original_position: pos,
                snapped_position: pos,
                snapped: false,
                active_zones: vec![],
                snap_direction: None,
            };
        }
        
        let mut best_snap = pos;
        let mut best_distance = f32::INFINITY;
        let mut active_zones = vec![];
        let mut snap_direction = None;
        
        for (zone_index, zone) in self.zones.iter().enumerate() {
            // Skip zones created by the component being moved
            if let Some(moving_id) = moving_component_id {
                if zone.source_components.contains(&moving_id) {
                    continue;
                }
            }
            
            let distance = (zone.position - pos).length();
            
            // Check if within magnetism range
            if distance <= zone.radius && distance >= self.preferences.min_distance {
                let magnetic_strength = zone.strength * self.calculate_falloff(distance, zone.radius);
                
                // Calculate attraction force
                if distance < best_distance && magnetic_strength > 0.1 {
                    let direction = (zone.position - pos).normalized();
                    let snap_strength = magnetic_strength * (1.0 - distance / zone.radius);
                    
                    best_snap = pos + direction * (distance * snap_strength);
                    best_distance = distance;
                    active_zones = vec![zone_index];
                    snap_direction = Some(direction);
                }
            }
        }
        
        let snapped = best_distance < f32::INFINITY && (best_snap - pos).length() > 1.0;
        
        MagnetismResult {
            original_position: pos,
            snapped_position: if snapped { best_snap } else { pos },
            snapped,
            active_zones,
            snap_direction,
        }
    }
    
    /// Calculate magnetic field falloff
    fn calculate_falloff(&self, distance: f32, radius: f32) -> f32 {
        if distance >= radius {
            return 0.0;
        }
        
        // Quadratic falloff for natural feeling
        let normalized_distance = distance / radius;
        (1.0 - normalized_distance * normalized_distance).max(0.0)
    }
    
    /// Render magnetic zones for debugging
    pub fn render_zones(&self, ui: &mut Ui, show_inactive: bool) {
        if !self.enabled {
            return;
        }
        
        let painter = ui.painter();
        
        for zone in &self.zones {
            if !zone.active && !show_inactive {
                continue;
            }
            
            let alpha = if zone.active { 100 } else { 30 };
            
            let color = match zone.zone_type {
                MagnetType::EdgeToEdge => Color32::from_rgba_premultiplied(255, 100, 100, alpha),
                MagnetType::CenterToCenter => Color32::from_rgba_premultiplied(100, 255, 100, alpha),
                MagnetType::Corner => Color32::from_rgba_premultiplied(100, 100, 255, alpha),
                MagnetType::GridPoint => Color32::from_rgba_premultiplied(200, 200, 200, alpha),
                MagnetType::Custom => Color32::from_rgba_premultiplied(255, 255, 100, alpha),
            };
            
            // Draw zone circle
            painter.circle_stroke(zone.position, zone.radius, Stroke::new(1.0, color));
            
            // Draw zone center
            painter.circle_filled(zone.position, 2.0, color);
        }
    }
    
    /// Activate zones near a position
    pub fn activate_zones_near(&mut self, pos: Pos2) {
        for zone in &mut self.zones {
            let distance = (zone.position - pos).length();
            zone.active = distance <= zone.radius * 1.5; // Slightly larger activation area
        }
    }
    
    /// Deactivate all zones
    pub fn deactivate_all_zones(&mut self) {
        for zone in &mut self.zones {
            zone.active = false;
        }
    }
    
    /// Get zone statistics
    pub fn get_statistics(&self) -> MagnetismStatistics {
        let edge_zones = self.zones.iter().filter(|z| z.zone_type == MagnetType::EdgeToEdge).count();
        let center_zones = self.zones.iter().filter(|z| z.zone_type == MagnetType::CenterToCenter).count();
        let corner_zones = self.zones.iter().filter(|z| z.zone_type == MagnetType::Corner).count();
        let grid_zones = self.zones.iter().filter(|z| z.zone_type == MagnetType::GridPoint).count();
        let active_zones = self.zones.iter().filter(|z| z.active).count();
        
        MagnetismStatistics {
            total_zones: self.zones.len(),
            edge_zones,
            center_zones,
            corner_zones,
            grid_zones,
            active_zones,
        }
    }
    
    /// Update magnetism preferences
    pub fn update_preferences(&mut self, preferences: MagnetismPreferences) {
        self.preferences = preferences;
    }
    
    /// Set global magnetism strength
    pub fn set_global_strength(&mut self, strength: f32) {
        self.global_strength = strength.clamp(0.0, 2.0);
    }
    
    /// Set detection radius
    pub fn set_detection_radius(&mut self, radius: f32) {
        self.detection_radius = radius.clamp(5.0, 50.0);
    }
}

/// Magnetism statistics for debugging and analytics
#[derive(Debug, Clone)]
pub struct MagnetismStatistics {
    pub total_zones: usize,
    pub edge_zones: usize,
    pub center_zones: usize,
    pub corner_zones: usize,
    pub grid_zones: usize,
    pub active_zones: usize,
}