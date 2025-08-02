//! Spacing Guide System
//!
//! This module provides intelligent spacing suggestions and consistent
//! component distribution helpers.

use egui::*;
use std::collections::HashMap;

/// Spacing guide for consistent component spacing
#[derive(Clone, Debug)]
pub struct SpacingGuide {
    /// Start position of the spacing
    pub start_pos: Pos2,
    /// End position of the spacing
    pub end_pos: Pos2,
    /// Suggested spacing distance
    pub suggested_distance: f32,
    /// Current actual distance
    pub actual_distance: f32,
    /// Components involved in this spacing
    pub components: [usize; 2],
    /// Spacing type
    pub spacing_type: SpacingType,
    /// Whether this guide is active
    pub active: bool,
}

/// Type of spacing guide
#[derive(Clone, Debug, PartialEq)]
pub enum SpacingType {
    /// Horizontal spacing between components
    Horizontal,
    /// Vertical spacing between components
    Vertical,
    /// Diagonal spacing
    Diagonal,
    /// Margin spacing (component to container edge)
    Margin,
}

/// Spacing guide manager
pub struct SpacingGuideManager {
    /// Active spacing guides
    pub guides: Vec<SpacingGuide>,
    /// Spacing preferences
    pub preferences: SpacingPreferences,
    /// Whether spacing guides are enabled
    pub enabled: bool,
    /// Visual style for guides
    pub style: SpacingGuideStyle,
}

/// Spacing behavior preferences
#[derive(Clone, Debug)]
pub struct SpacingPreferences {
    /// Default spacing unit
    pub default_spacing: f32,
    /// Spacing tolerance for suggestions
    pub tolerance: f32,
    /// Preferred spacing values
    pub preferred_spacings: Vec<f32>,
    /// Enable automatic spacing suggestions
    pub auto_suggest: bool,
    /// Minimum spacing to consider
    pub min_spacing: f32,
    /// Maximum spacing to consider
    pub max_spacing: f32,
}

/// Visual style for spacing guides
#[derive(Clone, Debug)]
pub struct SpacingGuideStyle {
    /// Line color for spacing indicators
    pub line_color: Color32,
    /// Line width
    pub line_width: f32,
    /// Text color for spacing labels
    pub text_color: Color32,
    /// Font size for labels
    pub font_size: f32,
    /// Arrow style for direction indicators
    pub arrow_style: ArrowStyle,
}

/// Arrow style for spacing indicators
#[derive(Clone, Debug, PartialEq)]
pub enum ArrowStyle {
    Simple,
    Double,
    Filled,
    None,
}

/// Result of spacing analysis
#[derive(Debug, Clone)]
pub struct SpacingAnalysis {
    /// Detected spacing patterns
    pub patterns: Vec<SpacingPattern>,
    /// Spacing inconsistencies
    pub inconsistencies: Vec<SpacingInconsistency>,
    /// Suggested improvements
    pub suggestions: Vec<SpacingSuggestion>,
}

/// Detected spacing pattern
#[derive(Debug, Clone)]
pub struct SpacingPattern {
    /// Components involved in the pattern
    pub components: Vec<usize>,
    /// Average spacing in the pattern
    pub average_spacing: f32,
    /// Pattern direction
    pub direction: SpacingType,
    /// Pattern confidence (0.0 to 1.0)
    pub confidence: f32,
}

/// Spacing inconsistency detection
#[derive(Debug, Clone)]
pub struct SpacingInconsistency {
    /// Components with inconsistent spacing
    pub components: [usize; 2],
    /// Expected spacing based on pattern
    pub expected_spacing: f32,
    /// Actual spacing
    pub actual_spacing: f32,
    /// Severity of inconsistency (0.0 to 1.0)
    pub severity: f32,
}

/// Spacing improvement suggestion
#[derive(Debug, Clone)]
pub struct SpacingSuggestion {
    /// Component to move
    pub component_id: usize,
    /// Suggested new position
    pub suggested_position: Pos2,
    /// Improvement type
    pub improvement_type: ImprovementType,
    /// Confidence in suggestion (0.0 to 1.0)
    pub confidence: f32,
}

/// Type of spacing improvement
#[derive(Debug, Clone, PartialEq)]
pub enum ImprovementType {
    /// Make spacing consistent with neighbors
    ConsistentSpacing,
    /// Align to grid
    GridAlignment,
    /// Improve visual balance
    VisualBalance,
    /// Follow design system spacing
    DesignSystemSpacing,
}

impl Default for SpacingGuideManager {
    fn default() -> Self {
        Self {
            guides: Vec::new(),
            preferences: SpacingPreferences::default(),
            enabled: true,
            style: SpacingGuideStyle::default(),
        }
    }
}

impl Default for SpacingPreferences {
    fn default() -> Self {
        Self {
            default_spacing: 16.0,
            tolerance: 4.0,
            preferred_spacings: vec![8.0, 16.0, 24.0, 32.0, 48.0],
            auto_suggest: true,
            min_spacing: 4.0,
            max_spacing: 200.0,
        }
    }
}

impl Default for SpacingGuideStyle {
    fn default() -> Self {
        Self {
            line_color: Color32::from_rgb(255, 165, 0), // Orange
            line_width: 1.5,
            text_color: Color32::from_rgb(200, 120, 0),
            font_size: 11.0,
            arrow_style: ArrowStyle::Simple,
        }
    }
}

impl SpacingGuideManager {
    /// Create a new spacing guide manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Update spacing guides based on component positions
    pub fn update_guides(&mut self, positions: &HashMap<usize, Pos2>, sizes: &HashMap<usize, Vec2>) {
        if !self.enabled {
            return;
        }
        
        self.guides.clear();
        
        let components: Vec<(usize, Rect)> = positions
            .iter()
            .map(|(&id, &pos)| {
                let size = sizes.get(&id).copied().unwrap_or(Vec2::ZERO);
                (id, Rect::from_min_size(pos, size))
            })
            .collect();
        
        // Generate horizontal spacing guides
        self.generate_horizontal_guides(&components);
        
        // Generate vertical spacing guides
        self.generate_vertical_guides(&components);
        
        // Filter guides based on preferences
        self.filter_guides();
    }
    
    /// Generate horizontal spacing guides
    fn generate_horizontal_guides(&mut self, components: &[(usize, Rect)]) {
        let mut sorted_components = components.to_vec();
        sorted_components.sort_by(|a, b| a.1.min.x.partial_cmp(&b.1.min.x).unwrap());
        
        for i in 0..sorted_components.len() {
            for j in (i + 1)..sorted_components.len() {
                let (id1, rect1) = &sorted_components[i];
                let (id2, rect2) = &sorted_components[j];
                
                // Check if components are horizontally aligned (vertically overlapping)
                let vertical_overlap = rect1.min.y.max(rect2.min.y) < rect1.max.y.min(rect2.max.y);
                
                if vertical_overlap {
                    let start_pos = Pos2::new(rect1.max.x, rect1.center().y);
                    let end_pos = Pos2::new(rect2.min.x, rect2.center().y);
                    let actual_distance = end_pos.x - start_pos.x;
                    
                    if actual_distance > self.preferences.min_spacing && actual_distance < self.preferences.max_spacing {
                        let suggested_distance = self.suggest_spacing(actual_distance);
                        
                        self.guides.push(SpacingGuide {
                            start_pos,
                            end_pos,
                            suggested_distance,
                            actual_distance,
                            components: [*id1, *id2],
                            spacing_type: SpacingType::Horizontal,
                            active: false,
                        });
                    }
                }
            }
        }
    }
    
    /// Generate vertical spacing guides
    fn generate_vertical_guides(&mut self, components: &[(usize, Rect)]) {
        let mut sorted_components = components.to_vec();
        sorted_components.sort_by(|a, b| a.1.min.y.partial_cmp(&b.1.min.y).unwrap());
        
        for i in 0..sorted_components.len() {
            for j in (i + 1)..sorted_components.len() {
                let (id1, rect1) = &sorted_components[i];
                let (id2, rect2) = &sorted_components[j];
                
                // Check if components are vertically aligned (horizontally overlapping)
                let horizontal_overlap = rect1.min.x.max(rect2.min.x) < rect1.max.x.min(rect2.max.x);
                
                if horizontal_overlap {
                    let start_pos = Pos2::new(rect1.center().x, rect1.max.y);
                    let end_pos = Pos2::new(rect2.center().x, rect2.min.y);
                    let actual_distance = end_pos.y - start_pos.y;
                    
                    if actual_distance > self.preferences.min_spacing && actual_distance < self.preferences.max_spacing {
                        let suggested_distance = self.suggest_spacing(actual_distance);
                        
                        self.guides.push(SpacingGuide {
                            start_pos,
                            end_pos,
                            suggested_distance,
                            actual_distance,
                            components: [*id1, *id2],
                            spacing_type: SpacingType::Vertical,
                            active: false,
                        });
                    }
                }
            }
        }
    }
    
    /// Suggest optimal spacing based on current distance
    fn suggest_spacing(&self, actual_distance: f32) -> f32 {
        // Find the closest preferred spacing
        self.preferences
            .preferred_spacings
            .iter()
            .min_by(|&&a, &&b| {
                let diff_a = (a - actual_distance).abs();
                let diff_b = (b - actual_distance).abs();
                diff_a.partial_cmp(&diff_b).unwrap()
            })
            .copied()
            .unwrap_or(self.preferences.default_spacing)
    }
    
    /// Filter guides based on preferences and relevance
    fn filter_guides(&mut self) {
        self.guides.retain(|guide| {
            let difference = (guide.suggested_distance - guide.actual_distance).abs();
            difference > self.preferences.tolerance
        });
    }
    
    /// Analyze spacing patterns in the current layout
    pub fn analyze_spacing(&self, positions: &HashMap<usize, Pos2>, sizes: &HashMap<usize, Vec2>) -> SpacingAnalysis {
        let components: Vec<(usize, Rect)> = positions
            .iter()
            .map(|(&id, &pos)| {
                let size = sizes.get(&id).copied().unwrap_or(Vec2::ZERO);
                (id, Rect::from_min_size(pos, size))
            })
            .collect();
        
        let patterns = self.detect_patterns(&components);
        let inconsistencies = self.detect_inconsistencies(&components, &patterns);
        let suggestions = self.generate_suggestions(&components, &patterns, &inconsistencies);
        
        SpacingAnalysis {
            patterns,
            inconsistencies,
            suggestions,
        }
    }
    
    /// Detect spacing patterns in component layout
    fn detect_patterns(&self, components: &[(usize, Rect)]) -> Vec<SpacingPattern> {
        let mut patterns = Vec::new();
        
        // Detect horizontal patterns
        patterns.extend(self.detect_horizontal_patterns(components));
        
        // Detect vertical patterns
        patterns.extend(self.detect_vertical_patterns(components));
        
        patterns
    }
    
    /// Detect horizontal spacing patterns
    fn detect_horizontal_patterns(&self, components: &[(usize, Rect)]) -> Vec<SpacingPattern> {
        let mut patterns = Vec::new();
        
        // Group components by horizontal alignment
        let mut y_groups: HashMap<i32, Vec<(usize, Rect)>> = HashMap::new();
        
        for &(id, rect) in components {
            let y_key = (rect.center().y / 10.0) as i32; // Group within 10 pixels
            y_groups.entry(y_key).or_default().push((id, rect));
        }
        
        for (_, mut group) in y_groups {
            if group.len() < 3 {
                continue; // Need at least 3 components for a pattern
            }
            
            group.sort_by(|a, b| a.1.min.x.partial_cmp(&b.1.min.x).unwrap());
            
            let mut spacings = Vec::new();
            for i in 0..group.len() - 1 {
                let spacing = group[i + 1].1.min.x - group[i].1.max.x;
                spacings.push(spacing);
            }
            
            // Check if spacings are consistent
            let avg_spacing = spacings.iter().sum::<f32>() / spacings.len() as f32;
            let variance = spacings.iter()
                .map(|&s| (s - avg_spacing).powi(2))
                .sum::<f32>() / spacings.len() as f32;
            
            let consistency = 1.0 - (variance.sqrt() / avg_spacing).min(1.0);
            
            if consistency > 0.7 { // 70% consistency threshold
                patterns.push(SpacingPattern {
                    components: group.iter().map(|(id, _)| *id).collect(),
                    average_spacing: avg_spacing,
                    direction: SpacingType::Horizontal,
                    confidence: consistency,
                });
            }
        }
        
        patterns
    }
    
    /// Detect vertical spacing patterns
    fn detect_vertical_patterns(&self, components: &[(usize, Rect)]) -> Vec<SpacingPattern> {
        let mut patterns = Vec::new();
        
        // Group components by vertical alignment
        let mut x_groups: HashMap<i32, Vec<(usize, Rect)>> = HashMap::new();
        
        for &(id, rect) in components {
            let x_key = (rect.center().x / 10.0) as i32; // Group within 10 pixels
            x_groups.entry(x_key).or_default().push((id, rect));
        }
        
        for (_, mut group) in x_groups {
            if group.len() < 3 {
                continue; // Need at least 3 components for a pattern
            }
            
            group.sort_by(|a, b| a.1.min.y.partial_cmp(&b.1.min.y).unwrap());
            
            let mut spacings = Vec::new();
            for i in 0..group.len() - 1 {
                let spacing = group[i + 1].1.min.y - group[i].1.max.y;
                spacings.push(spacing);
            }
            
            // Check if spacings are consistent
            let avg_spacing = spacings.iter().sum::<f32>() / spacings.len() as f32;
            let variance = spacings.iter()
                .map(|&s| (s - avg_spacing).powi(2))
                .sum::<f32>() / spacings.len() as f32;
            
            let consistency = 1.0 - (variance.sqrt() / avg_spacing).min(1.0);
            
            if consistency > 0.7 { // 70% consistency threshold
                patterns.push(SpacingPattern {
                    components: group.iter().map(|(id, _)| *id).collect(),
                    average_spacing: avg_spacing,
                    direction: SpacingType::Vertical,
                    confidence: consistency,
                });
            }
        }
        
        patterns
    }
    
    /// Detect spacing inconsistencies
    fn detect_inconsistencies(&self, _components: &[(usize, Rect)], patterns: &[SpacingPattern]) -> Vec<SpacingInconsistency> {
        let mut inconsistencies = Vec::new();
        
        for pattern in patterns {
            // Check each spacing in the pattern for inconsistencies
            // This is a simplified implementation
            for guide in &self.guides {
                if pattern.components.contains(&guide.components[0]) && 
                   pattern.components.contains(&guide.components[1]) {
                    
                    let expected = pattern.average_spacing;
                    let actual = guide.actual_distance;
                    let difference = (expected - actual).abs();
                    
                    if difference > self.preferences.tolerance {
                        let severity = (difference / expected).min(1.0);
                        
                        inconsistencies.push(SpacingInconsistency {
                            components: guide.components,
                            expected_spacing: expected,
                            actual_spacing: actual,
                            severity,
                        });
                    }
                }
            }
        }
        
        inconsistencies
    }
    
    /// Generate spacing improvement suggestions
    fn generate_suggestions(&self, _components: &[(usize, Rect)], _patterns: &[SpacingPattern], inconsistencies: &[SpacingInconsistency]) -> Vec<SpacingSuggestion> {
        let mut suggestions = Vec::new();
        
        for inconsistency in inconsistencies {
            // Simple suggestion: move the second component to achieve expected spacing
            // In a real implementation, this would be more sophisticated
            if inconsistency.severity > 0.3 {
                suggestions.push(SpacingSuggestion {
                    component_id: inconsistency.components[1],
                    suggested_position: Pos2::ZERO, // Would calculate actual position
                    improvement_type: ImprovementType::ConsistentSpacing,
                    confidence: 1.0 - inconsistency.severity,
                });
            }
        }
        
        suggestions
    }
    
    /// Render spacing guides
    pub fn render_guides(&self, ui: &mut Ui) {
        if !self.enabled {
            return;
        }
        
        let painter = ui.painter();
        
        for guide in &self.guides {
            if !guide.active {
                continue;
            }
            
            let color = self.style.line_color;
            let stroke = Stroke::new(self.style.line_width, color);
            
            // Draw spacing line
            painter.line_segment([guide.start_pos, guide.end_pos], stroke);
            
            // Draw arrows based on style
            self.draw_arrows(painter, guide.start_pos, guide.end_pos, stroke);
            
            // Draw spacing label
            let center = (guide.start_pos + guide.end_pos.to_vec2()) / 2.0;
            let label = format!("{:.0}px", guide.actual_distance);
            
            painter.text(
                center,
                Align2::CENTER_CENTER,
                &label,
                FontId::proportional(self.style.font_size),
                self.style.text_color,
            );
            
            // Show suggestion if different from actual
            if (guide.suggested_distance - guide.actual_distance).abs() > self.preferences.tolerance {
                let suggestion_label = format!("→ {:.0}px", guide.suggested_distance);
                let suggestion_pos = center + Vec2::new(0.0, 15.0);
                
                painter.text(
                    suggestion_pos,
                    Align2::CENTER_CENTER,
                    &suggestion_label,
                    FontId::proportional(self.style.font_size * 0.9),
                    Color32::from_rgb(100, 200, 100),
                );
            }
        }
    }
    
    /// Draw arrows for spacing indicators
    fn draw_arrows(&self, painter: &Painter, start: Pos2, end: Pos2, stroke: Stroke) {
        if self.style.arrow_style == ArrowStyle::None {
            return;
        }
        
        let direction = (end - start).normalized();
        let perpendicular = Vec2::new(-direction.y, direction.x);
        let arrow_size = 5.0;
        
        // Start arrow
        let start_arrow_tip = start + direction * arrow_size;
        let start_arrow_1 = start_arrow_tip + perpendicular * arrow_size * 0.5;
        let start_arrow_2 = start_arrow_tip - perpendicular * arrow_size * 0.5;
        
        painter.line_segment([start, start_arrow_1], stroke);
        painter.line_segment([start, start_arrow_2], stroke);
        
        // End arrow
        let end_arrow_tip = end - direction * arrow_size;
        let end_arrow_1 = end_arrow_tip + perpendicular * arrow_size * 0.5;
        let end_arrow_2 = end_arrow_tip - perpendicular * arrow_size * 0.5;
        
        painter.line_segment([end, end_arrow_1], stroke);
        painter.line_segment([end, end_arrow_2], stroke);
    }
    
    /// Activate guides near a position
    pub fn activate_guides_near(&mut self, pos: Pos2) {
        for guide in &mut self.guides {
            let line_center = (guide.start_pos + guide.end_pos.to_vec2()) / 2.0;
            let distance = (line_center - pos).length();
            guide.active = distance <= 50.0; // Activation radius
        }
    }
    
    /// Deactivate all guides
    pub fn deactivate_all_guides(&mut self) {
        for guide in &mut self.guides {
            guide.active = false;
        }
    }
    
    /// Get spacing statistics
    pub fn get_statistics(&self) -> SpacingStatistics {
        let horizontal_guides = self.guides.iter().filter(|g| g.spacing_type == SpacingType::Horizontal).count();
        let vertical_guides = self.guides.iter().filter(|g| g.spacing_type == SpacingType::Vertical).count();
        let active_guides = self.guides.iter().filter(|g| g.active).count();
        
        let total_variance = self.guides.iter()
            .map(|g| (g.suggested_distance - g.actual_distance).abs())
            .sum::<f32>() / self.guides.len().max(1) as f32;
        
        SpacingStatistics {
            total_guides: self.guides.len(),
            horizontal_guides,
            vertical_guides,
            active_guides,
            average_variance: total_variance,
        }
    }
}

/// Spacing statistics for debugging and analytics
#[derive(Debug, Clone)]
pub struct SpacingStatistics {
    pub total_guides: usize,
    pub horizontal_guides: usize,
    pub vertical_guides: usize,
    pub active_guides: usize,
    pub average_variance: f32,
}