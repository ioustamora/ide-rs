//! Geometry Utilities
//!
//! Common geometric calculations, transformations, and spatial data structures
//! used throughout the IDE for layout, positioning, and collision detection.

use egui::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// 2D bounding box with utility methods
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bounds {
    pub min: Pos2,
    pub max: Pos2,
}

/// 2D transformation matrix
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transform2D {
    pub translation: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

/// Spatial index for efficient collision detection and spatial queries
pub struct SpatialIndex<T> {
    /// Grid cells for spatial partitioning
    cells: HashMap<(i32, i32), Vec<SpatialItem<T>>>,
    /// Cell size for partitioning
    cell_size: f32,
}

/// Item in the spatial index
#[derive(Clone, Debug)]
pub struct SpatialItem<T> {
    pub bounds: Bounds,
    pub data: T,
}

impl Bounds {
    /// Create bounds from min and max points
    pub fn new(min: Pos2, max: Pos2) -> Self {
        Self { min, max }
    }
    
    /// Create bounds from center and size
    pub fn from_center_size(center: Pos2, size: Vec2) -> Self {
        let half_size = size / 2.0;
        Self {
            min: center - half_size,
            max: center + half_size,
        }
    }
    
    /// Create bounds from rect
    pub fn from_rect(rect: Rect) -> Self {
        Self {
            min: rect.min,
            max: rect.max,
        }
    }
    
    /// Convert to egui Rect
    pub fn to_rect(&self) -> Rect {
        Rect::from_min_max(self.min, self.max)
    }
    
    /// Get center point
    pub fn center(&self) -> Pos2 {
        (self.min.to_vec2() + self.max.to_vec2()) / 2.0
    }
    
    /// Get size
    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }
    
    /// Get width
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }
    
    /// Get height
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }
    
    /// Check if point is inside bounds
    pub fn contains_point(&self, point: Pos2) -> bool {
        point.x >= self.min.x && point.x <= self.max.x &&
        point.y >= self.min.y && point.y <= self.max.y
    }
    
    /// Check if bounds intersect
    pub fn intersects(&self, other: &Bounds) -> bool {
        self.max.x >= other.min.x && self.min.x <= other.max.x &&
        self.max.y >= other.min.y && self.min.y <= other.max.y
    }
    
    /// Get intersection with another bounds
    pub fn intersection(&self, other: &Bounds) -> Option<Bounds> {
        if !self.intersects(other) {
            return None;
        }
        
        let min = Pos2::new(
            self.min.x.max(other.min.x),
            self.min.y.max(other.min.y),
        );
        let max = Pos2::new(
            self.max.x.min(other.max.x),
            self.max.y.min(other.max.y),
        );
        
        Some(Bounds::new(min, max))
    }
    
    /// Expand bounds by margin
    pub fn expanded(&self, margin: f32) -> Bounds {
        Bounds::new(
            self.min - Vec2::splat(margin),
            self.max + Vec2::splat(margin),
        )
    }
    
    /// Shrink bounds by margin
    pub fn shrunk(&self, margin: f32) -> Bounds {
        self.expanded(-margin)
    }
    
    /// Translate bounds by offset
    pub fn translated(&self, offset: Vec2) -> Bounds {
        Bounds::new(
            self.min + offset,
            self.max + offset,
        )
    }
    
    /// Scale bounds from center
    pub fn scaled(&self, scale: f32) -> Bounds {
        let center = self.center();
        let size = self.size() * scale;
        Bounds::from_center_size(center, size)
    }
    
    /// Get distance to point
    pub fn distance_to_point(&self, point: Pos2) -> f32 {
        if self.contains_point(point) {
            return 0.0;
        }
        
        let dx = if point.x < self.min.x {
            self.min.x - point.x
        } else if point.x > self.max.x {
            point.x - self.max.x
        } else {
            0.0
        };
        
        let dy = if point.y < self.min.y {
            self.min.y - point.y
        } else if point.y > self.max.y {
            point.y - self.max.y
        } else {
            0.0
        };
        
        (dx * dx + dy * dy).sqrt()
    }
    
    /// Get closest point on bounds to given point
    pub fn closest_point(&self, point: Pos2) -> Pos2 {
        Pos2::new(
            point.x.clamp(self.min.x, self.max.x),
            point.y.clamp(self.min.y, self.max.y),
        )
    }
}

impl Default for Transform2D {
    fn default() -> Self {
        Self {
            translation: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::splat(1.0),
        }
    }
}

impl Transform2D {
    /// Create identity transform
    pub fn identity() -> Self {
        Self::default()
    }
    
    /// Create translation transform
    pub fn translation(offset: Vec2) -> Self {
        Self {
            translation: offset,
            ..Default::default()
        }
    }
    
    /// Create rotation transform
    pub fn rotation(angle: f32) -> Self {
        Self {
            rotation: angle,
            ..Default::default()
        }
    }
    
    /// Create scale transform
    pub fn scale(scale: Vec2) -> Self {
        Self {
            scale,
            ..Default::default()
        }
    }
    
    /// Apply transform to point
    pub fn transform_point(&self, point: Pos2) -> Pos2 {
        // Scale
        let scaled = Pos2::new(point.x * self.scale.x, point.y * self.scale.y);
        
        // Rotate
        let cos_r = self.rotation.cos();
        let sin_r = self.rotation.sin();
        let rotated = Pos2::new(
            scaled.x * cos_r - scaled.y * sin_r,
            scaled.x * sin_r + scaled.y * cos_r,
        );
        
        // Translate
        rotated + self.translation
    }
    
    /// Apply transform to bounds
    pub fn transform_bounds(&self, bounds: &Bounds) -> Bounds {
        let corners = [
            bounds.min,
            Pos2::new(bounds.max.x, bounds.min.y),
            bounds.max,
            Pos2::new(bounds.min.x, bounds.max.y),
        ];
        
        let transformed_corners: Vec<Pos2> = corners
            .iter()
            .map(|&corner| self.transform_point(corner))
            .collect();
        
        let min_x = transformed_corners.iter().map(|p| p.x).fold(f32::INFINITY, f32::min);
        let min_y = transformed_corners.iter().map(|p| p.y).fold(f32::INFINITY, f32::min);
        let max_x = transformed_corners.iter().map(|p| p.x).fold(f32::NEG_INFINITY, f32::max);
        let max_y = transformed_corners.iter().map(|p| p.y).fold(f32::NEG_INFINITY, f32::max);
        
        Bounds::new(Pos2::new(min_x, min_y), Pos2::new(max_x, max_y))
    }
    
    /// Combine with another transform
    pub fn combine(&self, other: &Transform2D) -> Transform2D {
        Transform2D {
            translation: self.translation + other.translation,
            rotation: self.rotation + other.rotation,
            scale: Vec2::new(self.scale.x * other.scale.x, self.scale.y * other.scale.y),
        }
    }
    
    /// Get inverse transform
    pub fn inverse(&self) -> Transform2D {
        Transform2D {
            translation: -self.translation,
            rotation: -self.rotation,
            scale: Vec2::new(1.0 / self.scale.x, 1.0 / self.scale.y),
        }
    }
}

impl<T> SpatialIndex<T> {
    /// Create new spatial index
    pub fn new(cell_size: f32) -> Self {
        Self {
            cells: HashMap::new(),
            cell_size,
        }
    }
    
    /// Insert item into spatial index
    pub fn insert(&mut self, bounds: Bounds, data: T) {
        let item = SpatialItem { bounds: bounds.clone(), data };
        
        // Calculate which cells this item spans
        let min_cell = self.world_to_cell(bounds.min);
        let max_cell = self.world_to_cell(bounds.max);
        
        for x in min_cell.0..=max_cell.0 {
            for y in min_cell.1..=max_cell.1 {
                self.cells.entry((x, y)).or_default().push(item.clone());
            }
        }
    }
    
    /// Query items that intersect with bounds
    pub fn query(&self, bounds: &Bounds) -> Vec<&T> {
        let mut results = Vec::new();
        let mut seen = std::collections::HashSet::new();
        
        let min_cell = self.world_to_cell(bounds.min);
        let max_cell = self.world_to_cell(bounds.max);
        
        for x in min_cell.0..=max_cell.0 {
            for y in min_cell.1..=max_cell.1 {
                if let Some(items) = self.cells.get(&(x, y)) {
                    for item in items {
                        let item_ptr = &item.data as *const T;
                        if seen.insert(item_ptr) && item.bounds.intersects(bounds) {
                            results.push(&item.data);
                        }
                    }
                }
            }
        }
        
        results
    }
    
    /// Query items that contain point
    pub fn query_point(&self, point: Pos2) -> Vec<&T> {
        let cell = self.world_to_cell(point);
        
        if let Some(items) = self.cells.get(&cell) {
            items
                .iter()
                .filter(|item| item.bounds.contains_point(point))
                .map(|item| &item.data)
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Clear all items
    pub fn clear(&mut self) {
        self.cells.clear();
    }
    
    /// Get total number of items (may include duplicates across cells)
    pub fn total_items(&self) -> usize {
        self.cells.values().map(|items| items.len()).sum()
    }
    
    /// Convert world coordinates to cell coordinates
    fn world_to_cell(&self, pos: Pos2) -> (i32, i32) {
        (
            (pos.x / self.cell_size).floor() as i32,
            (pos.y / self.cell_size).floor() as i32,
        )
    }
}

/// Geometric utility functions
pub mod utils {
    use super::*;
    
    /// Calculate distance between two points
    pub fn distance(a: Pos2, b: Pos2) -> f32 {
        (a - b).length()
    }
    
    /// Calculate squared distance (faster when you only need to compare distances)
    pub fn distance_squared(a: Pos2, b: Pos2) -> f32 {
        (a - b).length_sq()
    }
    
    /// Check if point is inside triangle
    pub fn point_in_triangle(point: Pos2, a: Pos2, b: Pos2, c: Pos2) -> bool {
        let v0 = c - a;
        let v1 = b - a;
        let v2 = point - a;
        
        let dot00 = v0.dot(v0);
        let dot01 = v0.dot(v1);
        let dot02 = v0.dot(v2);
        let dot11 = v1.dot(v1);
        let dot12 = v1.dot(v2);
        
        let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
        let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
        let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;
        
        u >= 0.0 && v >= 0.0 && u + v <= 1.0
    }
    
    /// Get closest point on line segment to given point
    pub fn closest_point_on_line(point: Pos2, line_start: Pos2, line_end: Pos2) -> Pos2 {
        let line_vec = line_end - line_start;
        let point_vec = point - line_start;
        
        let line_len_sq = line_vec.length_sq();
        if line_len_sq == 0.0 {
            return line_start;
        }
        
        let t = (point_vec.dot(line_vec) / line_len_sq).clamp(0.0, 1.0);
        line_start + line_vec * t
    }
    
    /// Calculate area of triangle
    pub fn triangle_area(a: Pos2, b: Pos2, c: Pos2) -> f32 {
        0.5 * ((b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y)).abs()
    }
    
    /// Calculate centroid of polygon
    pub fn polygon_centroid(points: &[Pos2]) -> Pos2 {
        if points.is_empty() {
            return Pos2::ZERO;
        }
        
        let sum = points.iter().fold(Vec2::ZERO, |acc, &p| acc + p.to_vec2());
        (sum / points.len() as f32).to_pos2()
    }
    
    /// Check if polygon is convex
    pub fn is_convex_polygon(points: &[Pos2]) -> bool {
        if points.len() < 3 {
            return false;
        }
        
        let mut sign = 0.0;
        
        for i in 0..points.len() {
            let p1 = points[i];
            let p2 = points[(i + 1) % points.len()];
            let p3 = points[(i + 2) % points.len()];
            
            let cross = (p2.x - p1.x) * (p3.y - p2.y) - (p2.y - p1.y) * (p3.x - p2.x);
            
            if cross != 0.0 {
                if sign == 0.0 {
                    sign = cross;
                } else if (cross > 0.0) != (sign > 0.0) {
                    return false;
                }
            }
        }
        
        true
    }
}