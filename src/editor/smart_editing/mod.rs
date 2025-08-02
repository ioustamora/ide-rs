//! Smart Editing System - Modular Architecture
//!
//! This module provides intelligent editing assistance through a modular system:
//! - Alignment guides for precise component positioning
//! - Component magnetism for snapping and spatial relationships
//! - Spacing guides for consistent layout patterns
//! - Learning system for adaptive behavior based on user preferences

pub mod alignment_guides;
pub mod magnetism;
pub mod spacing_guides;
pub mod learning_system;

// Re-export main types for convenience
pub use alignment_guides::{AlignmentGuideManager, AlignmentGuide, GuideDirection, AlignmentType};
pub use magnetism::{MagnetismManager, MagnetZone, MagnetType, MagnetismResult};
pub use spacing_guides::{SpacingGuideManager, SpacingGuide, SpacingType, SpacingAnalysis};
pub use learning_system::{SmartEditingLearningSystem, GuideActivation, GuideType, UserAction};

use egui::*;
use std::collections::HashMap;

/// Main smart editing system that orchestrates all subsystems
pub struct SmartEditingSystem {
    /// Alignment guide manager
    pub alignment_guides: AlignmentGuideManager,
    /// Component magnetism manager
    pub magnetism: MagnetismManager,
    /// Spacing guide manager
    pub spacing_guides: SpacingGuideManager,
    /// Learning system for adaptive behavior
    pub learning_system: SmartEditingLearningSystem,
    /// Whether smart editing is globally enabled
    pub enabled: bool,
    /// Current active tab in UI
    active_tab: SmartEditingTab,
}

/// Smart editing result containing all feedback
#[derive(Debug, Clone)]
pub struct SmartEditingResult {
    /// Original position before smart editing
    pub original_position: Pos2,
    /// Final position after all smart editing effects
    pub final_position: Pos2,
    /// Whether any snapping occurred
    pub position_changed: bool,
    /// Active alignment guides
    pub active_alignment_guides: Vec<usize>,
    /// Active magnetism zones
    pub active_magnetism_zones: Vec<usize>,
    /// Active spacing guides
    pub active_spacing_guides: Vec<usize>,
    /// Smart editing confidence (0.0 to 1.0)
    pub confidence: f32,
}

/// Tabs for smart editing UI
#[derive(Clone, Debug, PartialEq)]
enum SmartEditingTab {
    Overview,
    AlignmentGuides,
    Magnetism,
    SpacingGuides,
    Learning,
    Settings,
}

impl Default for SmartEditingSystem {
    fn default() -> Self {
        Self {
            alignment_guides: AlignmentGuideManager::new(),
            magnetism: MagnetismManager::new(),
            spacing_guides: SpacingGuideManager::new(),
            learning_system: SmartEditingLearningSystem::new(),
            enabled: true,
            active_tab: SmartEditingTab::Overview,
        }
    }
}

impl SmartEditingSystem {
    /// Create a new smart editing system
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Update all smart editing systems based on component layout
    pub fn update(&mut self, positions: &HashMap<usize, Pos2>, sizes: &HashMap<usize, Vec2>) {
        if !self.enabled {
            return;
        }
        
        // Update alignment guides
        self.alignment_guides.update_guides(positions, sizes);
        
        // Update magnetism zones
        self.magnetism.update_zones(positions, sizes);
        
        // Update spacing guides
        self.spacing_guides.update_guides(positions, sizes);
    }
    
    /// Apply smart editing to a component position
    pub fn apply_smart_editing(
        &mut self,
        position: Pos2,
        component_id: Option<usize>,
        context: &learning_system::ActivationContext,
    ) -> SmartEditingResult {
        if !self.enabled {
            return SmartEditingResult {
                original_position: position,
                final_position: position,
                position_changed: false,
                active_alignment_guides: vec![],
                active_magnetism_zones: vec![],
                active_spacing_guides: vec![],
                confidence: 0.0,
            };
        }
        
        let mut current_position = position;
        let mut position_changed = false;
        let mut confidence_factors = Vec::new();
        
        // Apply alignment guide snapping
        let aligned_position = self.alignment_guides.snap_to_guide(current_position);
        if (aligned_position - current_position).length() > 1.0 {
            current_position = aligned_position;
            position_changed = true;
            confidence_factors.push(0.8);
            
            // Record activation for learning
            self.learning_system.record_activation(learning_system::GuideActivation {
                timestamp: chrono::Utc::now(),
                guide_type: learning_system::GuideType::AlignmentGuide,
                user_action: learning_system::UserAction::Accepted, // Assume accepted for now
                context: context.clone(),
            });
        }
        
        // Apply magnetism
        let magnetism_result = self.magnetism.apply_magnetism(current_position, component_id);
        if magnetism_result.snapped {
            current_position = magnetism_result.snapped_position;
            position_changed = true;
            confidence_factors.push(0.7);
            
            // Record activation for learning
            self.learning_system.record_activation(learning_system::GuideActivation {
                timestamp: chrono::Utc::now(),
                guide_type: learning_system::GuideType::MagnetismZone,
                user_action: learning_system::UserAction::Accepted,
                context: context.clone(),
            });
        }
        
        // Calculate overall confidence
        let confidence = if confidence_factors.is_empty() {
            0.0
        } else {
            confidence_factors.iter().sum::<f32>() / confidence_factors.len() as f32
        };
        
        // Activate guides near the final position for visual feedback
        self.activate_guides_near(current_position);
        
        SmartEditingResult {
            original_position: position,
            final_position: current_position,
            position_changed,
            active_alignment_guides: self.get_active_alignment_guide_indices(),
            active_magnetism_zones: self.get_active_magnetism_zone_indices(),
            active_spacing_guides: self.get_active_spacing_guide_indices(),
            confidence,
        }
    }
    
    /// Activate guides near a position for visual feedback
    pub fn activate_guides_near(&mut self, position: Pos2) {
        self.alignment_guides.activate_guides_near(position);
        self.magnetism.activate_zones_near(position);
        self.spacing_guides.activate_guides_near(position);
    }
    
    /// Deactivate all guides
    pub fn deactivate_all_guides(&mut self) {
        self.alignment_guides.deactivate_all_guides();
        self.magnetism.deactivate_all_zones();
        self.spacing_guides.deactivate_all_guides();
    }
    
    /// Get indices of active alignment guides
    fn get_active_alignment_guide_indices(&self) -> Vec<usize> {
        self.alignment_guides.guides
            .iter()
            .enumerate()
            .filter(|(_, guide)| guide.active)
            .map(|(i, _)| i)
            .collect()
    }
    
    /// Get indices of active magnetism zones
    fn get_active_magnetism_zone_indices(&self) -> Vec<usize> {
        self.magnetism.zones
            .iter()
            .enumerate()
            .filter(|(_, zone)| zone.active)
            .map(|(i, _)| i)
            .collect()
    }
    
    /// Get indices of active spacing guides
    fn get_active_spacing_guide_indices(&self) -> Vec<usize> {
        self.spacing_guides.guides
            .iter()
            .enumerate()
            .filter(|(_, guide)| guide.active)
            .map(|(i, _)| i)
            .collect()
    }
    
    /// Render all smart editing guides
    pub fn render_guides(&self, ui: &mut Ui, canvas_rect: Rect) {
        if !self.enabled {
            return;
        }
        
        // Render alignment guides
        self.alignment_guides.render_guides(ui, canvas_rect);
        
        // Render spacing guides
        self.spacing_guides.render_guides(ui);
        
        // Render magnetism zones (if debug mode is enabled)
        // self.magnetism.render_zones(ui, false);
    }
    
    /// Analyze spacing patterns in current layout
    pub fn analyze_spacing(&self, positions: &HashMap<usize, Pos2>, sizes: &HashMap<usize, Vec2>) -> spacing_guides::SpacingAnalysis {
        self.spacing_guides.analyze_spacing(positions, sizes)
    }
    
    /// Get recommendations for component positioning
    pub fn get_positioning_recommendations(
        &self,
        component_id: usize,
        current_position: Pos2,
        context: &learning_system::ActivationContext,
    ) -> Vec<PositionRecommendation> {
        let mut recommendations = Vec::new();
        
        // Get alignment recommendations
        if let Some(h_guide) = self.alignment_guides.find_closest_guide(current_position, alignment_guides::GuideDirection::Horizontal) {
            recommendations.push(PositionRecommendation {
                position: Pos2::new(current_position.x, h_guide.position),
                reason: "Horizontal alignment".to_string(),
                confidence: 0.8,
                recommendation_type: RecommendationType::Alignment,
            });
        }
        
        if let Some(v_guide) = self.alignment_guides.find_closest_guide(current_position, alignment_guides::GuideDirection::Vertical) {
            recommendations.push(PositionRecommendation {
                position: Pos2::new(v_guide.position, current_position.y),
                reason: "Vertical alignment".to_string(),
                confidence: 0.8,
                recommendation_type: RecommendationType::Alignment,
            });
        }
        
        // Get magnetism recommendations
        let magnetism_result = self.magnetism.apply_magnetism(current_position, Some(component_id));
        if magnetism_result.snapped {
            recommendations.push(PositionRecommendation {
                position: magnetism_result.snapped_position,
                reason: "Component magnetism".to_string(),
                confidence: 0.7,
                recommendation_type: RecommendationType::Magnetism,
            });
        }
        
        // Get spacing recommendations based on learned preferences
        let preferred_spacings = self.learning_system.get_spacing_recommendations(context);
        for &spacing in &preferred_spacings[..3.min(preferred_spacings.len())] {
            recommendations.push(PositionRecommendation {
                position: Pos2::new(current_position.x + spacing, current_position.y),
                reason: format!("Learned spacing preference: {}px", spacing),
                confidence: 0.6,
                recommendation_type: RecommendationType::Spacing,
            });
        }
        
        // Sort by confidence
        recommendations.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        
        // Limit to top 5 recommendations
        recommendations.truncate(5);
        
        recommendations
    }
    
    /// Render smart editing UI
    pub fn render_ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            // Header with enable/disable toggle
            ui.horizontal(|ui| {
                ui.heading("Smart Editing");
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.checkbox(&mut self.enabled, "Enabled");
                });
            });
            
            ui.separator();
            
            // Tab bar
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.active_tab, SmartEditingTab::Overview, "📊 Overview");
                ui.selectable_value(&mut self.active_tab, SmartEditingTab::AlignmentGuides, "📏 Alignment");
                ui.selectable_value(&mut self.active_tab, SmartEditingTab::Magnetism, "🧲 Magnetism");
                ui.selectable_value(&mut self.active_tab, SmartEditingTab::SpacingGuides, "📐 Spacing");
                ui.selectable_value(&mut self.active_tab, SmartEditingTab::Learning, "🧠 Learning");
                ui.selectable_value(&mut self.active_tab, SmartEditingTab::Settings, "⚙ Settings");
            });
            
            ui.separator();
            
            // Tab content
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    match self.active_tab {
                        SmartEditingTab::Overview => self.render_overview_tab(ui),
                        SmartEditingTab::AlignmentGuides => self.render_alignment_tab(ui),
                        SmartEditingTab::Magnetism => self.render_magnetism_tab(ui),
                        SmartEditingTab::SpacingGuides => self.render_spacing_tab(ui),
                        SmartEditingTab::Learning => self.render_learning_tab(ui),
                        SmartEditingTab::Settings => self.render_settings_tab(ui),
                    }
                });
        });
    }
    
    /// Render overview tab
    fn render_overview_tab(&self, ui: &mut Ui) {
        ui.heading("Smart Editing Overview");
        
        // System status
        ui.group(|ui| {
            ui.label("System Status");
            ui.horizontal(|ui| {
                let status_color = if self.enabled { Color32::GREEN } else { Color32::RED };
                let status_text = if self.enabled { "Active" } else { "Disabled" };
                ui.colored_label(status_color, status_text);
            });
        });
        
        ui.separator();
        
        // Statistics
        ui.group(|ui| {
            ui.label("Statistics");
            
            let alignment_stats = self.alignment_guides.get_statistics();
            let magnetism_stats = self.magnetism.get_statistics();
            let spacing_stats = self.spacing_guides.get_statistics();
            let learning_stats = self.learning_system.get_statistics();
            
            egui::Grid::new("stats_grid")
                .num_columns(2)
                .spacing([10.0, 5.0])
                .show(ui, |ui| {
                    ui.label("Alignment Guides:");
                    ui.label(format!("{} total, {} active", alignment_stats.total_guides, alignment_stats.active_guides));
                    ui.end_row();
                    
                    ui.label("Magnetism Zones:");
                    ui.label(format!("{} total, {} active", magnetism_stats.total_zones, magnetism_stats.active_zones));
                    ui.end_row();
                    
                    ui.label("Spacing Guides:");
                    ui.label(format!("{} total, {} active", spacing_stats.total_guides, spacing_stats.active_guides));
                    ui.end_row();
                    
                    ui.label("Learning Data:");
                    ui.label(format!("{} activations, {:.1}% acceptance", learning_stats.total_activations, learning_stats.acceptance_rate * 100.0));
                    ui.end_row();
                });
        });
    }
    
    /// Render alignment guides tab
    fn render_alignment_tab(&mut self, ui: &mut Ui) {
        ui.heading("Alignment Guides");
        
        ui.checkbox(&mut self.alignment_guides.enabled, "Enable alignment guides");
        
        ui.horizontal(|ui| {
            ui.label("Detection threshold:");
            ui.add(egui::Slider::new(&mut self.alignment_guides.detection_threshold, 1.0..=20.0).suffix("px"));
        });
        
        ui.separator();
        
        // Guide style settings
        ui.collapsing("Visual Style", |ui| {
            ui.horizontal(|ui| {
                ui.label("Line color:");
                ui.color_edit_button_srgba(&mut self.alignment_guides.style.line_color);
            });
            
            ui.horizontal(|ui| {
                ui.label("Line width:");
                ui.add(egui::Slider::new(&mut self.alignment_guides.style.line_width, 0.5..=5.0));
            });
        });
        
        // Current guides
        ui.collapsing("Active Guides", |ui| {
            if self.alignment_guides.guides.is_empty() {
                ui.label("No guides currently active");
            } else {
                for (i, guide) in self.alignment_guides.guides.iter().enumerate() {
                    ui.horizontal(|ui| {
                        let direction = match guide.direction {
                            alignment_guides::GuideDirection::Horizontal => "H",
                            alignment_guides::GuideDirection::Vertical => "V",
                        };
                        ui.label(format!("{} {}: {:.1}px", i, direction, guide.position));
                        ui.label(format!("({} components)", guide.aligned_components.len()));
                    });
                }
            }
        });
    }
    
    /// Render magnetism tab
    fn render_magnetism_tab(&mut self, ui: &mut Ui) {
        ui.heading("Component Magnetism");
        
        ui.checkbox(&mut self.magnetism.enabled, "Enable magnetism");
        
        ui.horizontal(|ui| {
            ui.label("Global strength:");
            ui.add(egui::Slider::new(&mut self.magnetism.global_strength, 0.0..=2.0));
        });
        
        ui.horizontal(|ui| {
            ui.label("Detection radius:");
            ui.add(egui::Slider::new(&mut self.magnetism.detection_radius, 5.0..=50.0).suffix("px"));
        });
        
        ui.separator();
        
        // Magnetism preferences
        ui.collapsing("Preferences", |ui| {
            ui.checkbox(&mut self.magnetism.preferences.edge_magnetism, "Edge magnetism");
            ui.checkbox(&mut self.magnetism.preferences.center_magnetism, "Center magnetism");
            ui.checkbox(&mut self.magnetism.preferences.corner_magnetism, "Corner magnetism");
            ui.checkbox(&mut self.magnetism.preferences.grid_magnetism, "Grid magnetism");
            
            if self.magnetism.preferences.grid_magnetism {
                ui.horizontal(|ui| {
                    ui.label("Grid size:");
                    ui.add(egui::Slider::new(&mut self.magnetism.preferences.grid_size, 10.0..=50.0).suffix("px"));
                });
            }
        });
        
        // Zone statistics
        let stats = self.magnetism.get_statistics();
        ui.collapsing("Statistics", |ui| {
            egui::Grid::new("magnetism_stats")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("Total zones:");
                    ui.label(stats.total_zones.to_string());
                    ui.end_row();
                    
                    ui.label("Edge zones:");
                    ui.label(stats.edge_zones.to_string());
                    ui.end_row();
                    
                    ui.label("Center zones:");
                    ui.label(stats.center_zones.to_string());
                    ui.end_row();
                    
                    ui.label("Active zones:");
                    ui.label(stats.active_zones.to_string());
                    ui.end_row();
                });
        });
    }
    
    /// Render spacing guides tab
    fn render_spacing_tab(&mut self, ui: &mut Ui) {
        ui.heading("Spacing Guides");
        
        ui.checkbox(&mut self.spacing_guides.enabled, "Enable spacing guides");
        
        ui.horizontal(|ui| {
            ui.label("Default spacing:");
            ui.add(egui::Slider::new(&mut self.spacing_guides.preferences.default_spacing, 4.0..=64.0).suffix("px"));
        });
        
        ui.horizontal(|ui| {
            ui.label("Tolerance:");
            ui.add(egui::Slider::new(&mut self.spacing_guides.preferences.tolerance, 1.0..=10.0).suffix("px"));
        });
        
        ui.separator();
        
        // Preferred spacings
        ui.collapsing("Preferred Spacings", |ui| {
            ui.label("Edit preferred spacing values:");
            
            let mut spacings_text = self.spacing_guides.preferences.preferred_spacings
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            
            if ui.text_edit_singleline(&mut spacings_text).changed() {
                // Parse the spacing values
                let new_spacings: Vec<f32> = spacings_text
                    .split(',')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();
                
                if !new_spacings.is_empty() {
                    self.spacing_guides.preferences.preferred_spacings = new_spacings;
                }
            }
            
            ui.small("Separate values with commas (e.g., 8, 16, 24, 32)");
        });
        
        // Current guides
        let stats = self.spacing_guides.get_statistics();
        ui.collapsing("Statistics", |ui| {
            egui::Grid::new("spacing_stats")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("Total guides:");
                    ui.label(stats.total_guides.to_string());
                    ui.end_row();
                    
                    ui.label("Horizontal guides:");
                    ui.label(stats.horizontal_guides.to_string());
                    ui.end_row();
                    
                    ui.label("Vertical guides:");
                    ui.label(stats.vertical_guides.to_string());
                    ui.end_row();
                    
                    ui.label("Average variance:");
                    ui.label(format!("{:.1}px", stats.average_variance));
                    ui.end_row();
                });
        });
    }
    
    /// Render learning system tab
    fn render_learning_tab(&mut self, ui: &mut Ui) {
        ui.heading("Learning System");
        
        ui.checkbox(&mut self.learning_system.config.auto_update, "Auto-update preferences");
        
        ui.horizontal(|ui| {
            ui.label("Learning rate:");
            ui.add(egui::Slider::new(&mut self.learning_system.config.learning_rate, 0.01..=1.0));
        });
        
        ui.separator();
        
        // Learning statistics
        let stats = self.learning_system.get_statistics();
        ui.group(|ui| {
            ui.label("Learning Statistics");
            
            egui::Grid::new("learning_stats")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("Total activations:");
                    ui.label(stats.total_activations.to_string());
                    ui.end_row();
                    
                    ui.label("Acceptance rate:");
                    ui.label(format!("{:.1}%", stats.acceptance_rate * 100.0));
                    ui.end_row();
                    
                    ui.label("Learned guide types:");
                    ui.label(stats.learned_guide_types.to_string());
                    ui.end_row();
                    
                    ui.label("Workflow patterns:");
                    ui.label(stats.workflow_patterns.to_string());
                    ui.end_row();
                });
        });
        
        ui.separator();
        
        // Data management
        ui.horizontal(|ui| {
            if ui.button("Export Learning Data").clicked() {
                if let Ok(json) = self.learning_system.export_learning_data() {
                    // In a real implementation, this would save to file or clipboard
                    println!("Learning data exported: {} bytes", json.len());
                }
            }
            
            if ui.button("Reset Learning Data").clicked() {
                self.learning_system = SmartEditingLearningSystem::new();
            }
        });
    }
    
    /// Render settings tab
    fn render_settings_tab(&mut self, ui: &mut Ui) {
        ui.heading("Smart Editing Settings");
        
        // Global settings
        ui.group(|ui| {
            ui.label("Global Settings");
            
            ui.checkbox(&mut self.enabled, "Enable smart editing system");
        });
        
        ui.separator();
        
        // Export/Import settings
        ui.group(|ui| {
            ui.label("Data Management");
            
            ui.horizontal(|ui| {
                if ui.button("Export All Settings").clicked() {
                    // Export all smart editing configuration
                }
                
                if ui.button("Import Settings").clicked() {
                    // Import smart editing configuration
                }
                
                if ui.button("Reset to Defaults").clicked() {
                    *self = SmartEditingSystem::new();
                }
            });
        });
    }
}

/// Position recommendation for component placement
#[derive(Debug, Clone)]
pub struct PositionRecommendation {
    /// Recommended position
    pub position: Pos2,
    /// Human-readable reason for recommendation
    pub reason: String,
    /// Confidence in recommendation (0.0 to 1.0)
    pub confidence: f32,
    /// Type of recommendation
    pub recommendation_type: RecommendationType,
}

/// Type of position recommendation
#[derive(Debug, Clone, PartialEq)]
pub enum RecommendationType {
    Alignment,
    Magnetism,
    Spacing,
    Learning,
}