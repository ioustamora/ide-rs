//! Animated UI Components
//! 
//! This module provides animated UI widgets for enhanced user experience,
//! including smooth dropdown animations and component movement effects.

use egui::*;
use std::collections::HashMap;

/// Animation state for a collapsible section
#[derive(Clone, Debug)]
pub struct CollapseAnimation {
    /// Current animation progress (0.0 = collapsed, 1.0 = expanded)
    pub progress: f32,
    /// Target state (true = expanded, false = collapsed)
    pub target_expanded: bool,
    /// Animation speed (progress change per second)
    pub speed: f32,
    /// Whether the animation is currently running
    pub animating: bool,
}

impl CollapseAnimation {
    pub fn new(expanded: bool) -> Self {
        Self {
            progress: if expanded { 1.0 } else { 0.0 },
            target_expanded: expanded,
            speed: 5.0, // Slower, more stable animation
            animating: false,
        }
    }
    
    /// Update the animation state
    pub fn update(&mut self, ctx: &Context) {
        if self.animating {
            let delta_time = ctx.input(|i| i.stable_dt).min(0.016); // Cap at 60fps for stability
            let target = if self.target_expanded { 1.0 } else { 0.0 };
            
            if (self.progress - target).abs() > 0.01 {  // Increased threshold for stability
                // Move towards target with easing
                let diff = target - self.progress;
                self.progress += diff * self.speed * delta_time;
                self.progress = self.progress.clamp(0.0, 1.0);
                
                // Request repaint for next frame
                ctx.request_repaint();
            } else {
                // Animation complete
                self.progress = target;
                self.animating = false;
            }
        }
    }
    
    /// Set the target state and start animation if needed
    pub fn set_expanded(&mut self, expanded: bool) {
        if self.target_expanded != expanded {
            self.target_expanded = expanded;
            self.animating = true;
        }
    }
    
    /// Get the current easing value for smooth animation
    pub fn eased_progress(&self) -> f32 {
        // Use easeOutCubic for smooth feel
        let t = self.progress;
        1.0 - (1.0 - t).powi(3)
    }
}

/// Manager for all collapse animations
pub struct AnimationManager {
    /// Map of animation states by ID
    animations: HashMap<String, CollapseAnimation>,
}

impl AnimationManager {
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
        }
    }
    
    /// Get or create an animation state for the given ID
    pub fn get_or_create(&mut self, id: &str, default_expanded: bool) -> &mut CollapseAnimation {
        self.animations.entry(id.to_string())
            .or_insert_with(|| CollapseAnimation::new(default_expanded))
    }
    
    /// Update all animations
    pub fn update_all(&mut self, ctx: &Context) {
        for animation in self.animations.values_mut() {
            animation.update(ctx);
        }
    }
}

/// Animated collapsible header widget
pub struct AnimatedCollapsing<'a> {
    id: Id,
    text: String,
    animation_manager: &'a mut AnimationManager,
    default_open: bool,
}

impl<'a> AnimatedCollapsing<'a> {
    pub fn new(id: Id, text: String, animation_manager: &'a mut AnimationManager) -> Self {
        Self {
            id,
            text,
            animation_manager,
            default_open: false,
        }
    }
    
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }
    
    pub fn show<R: Clone>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> CollapsingResponse<R> {
        let id_str = format!("{:?}", self.id);
        
        // Update all animations first
        self.animation_manager.update_all(ui.ctx());
        
        // Get or create animation state
        let animation = self.animation_manager.get_or_create(&id_str, self.default_open);
        
        // Create header
        let header_response = ui.horizontal(|ui| {
            let progress = animation.eased_progress();
            
            // Animated arrow (simplified)
            ui.scope(|ui| {
                let (rect, response) = ui.allocate_exact_size(
                    Vec2::splat(ui.spacing().icon_width), 
                    Sense::click()
                );
                
                if response.clicked() {
                    animation.set_expanded(!animation.target_expanded);
                }
                
                // Draw animated arrow
                let center = rect.center();
                let painter = ui.painter();
                
                painter.text(
                    center,
                    Align2::CENTER_CENTER,
                    if progress > 0.5 { "▼" } else { "▶" },
                    FontId::default(),
                    ui.visuals().text_color(),
                );
                
                response
            }).inner
        });
        
        // Add the text
        let text_response = ui.horizontal(|ui| {
            ui.label(&self.text)
        }).response;
        
        // Combine responses for header
        let header_response = header_response.response.union(text_response);
        
        // Handle header clicks
        if header_response.clicked() {
            let animation = self.animation_manager.get_or_create(&id_str, self.default_open);
            animation.set_expanded(!animation.target_expanded);
        }
        
        // Animated content area
        let animation = self.animation_manager.get_or_create(&id_str, self.default_open);
        let progress = animation.eased_progress();
        
        let body_response = if progress > 0.0 {
            // Calculate content size
            let available_rect = ui.available_rect_before_wrap();
            let _content_ui_id = self.id.with("content");
            
            // Create a temporary UI to measure content size
            let mut content_size = Vec2::ZERO;
            let response = ui.allocate_ui_at_rect(available_rect, |ui| {
                ui.set_clip_rect(available_rect);
                
                // Animate the height
                let content_response = ui.allocate_ui_with_layout(
                    Vec2::new(available_rect.width(), available_rect.height() * progress),
                    Layout::top_down(Align::LEFT),
                    |ui| {
                        ui.set_clip_rect(Rect::from_min_size(
                            available_rect.min,
                            Vec2::new(available_rect.width(), available_rect.height() * progress)
                        ));
                        
                        add_contents(ui)
                    }
                );
                
                content_size = content_response.response.rect.size();
                content_response.inner
            });
            
            Some(response)
        } else {
            None
        };
        
        let body_returned = body_response.as_ref().map(|r| r.inner.clone());
        
        CollapsingResponse {
            header_response,
            body_response,
            body_returned,
            openness: progress,
        }
    }
}

/// Response from an animated collapsing widget
pub struct CollapsingResponse<R> {
    pub header_response: Response,
    pub body_response: Option<InnerResponse<R>>,
    pub body_returned: Option<R>,
    pub openness: f32,
}

/// Component movement animation state
#[derive(Clone, Debug)]
pub struct MoveAnimation {
    /// Current position
    pub current_pos: Pos2,
    /// Target position
    pub target_pos: Pos2,
    /// Animation speed (pixels per second)
    pub speed: f32,
    /// Whether the animation is running
    pub animating: bool,
}

impl MoveAnimation {
    pub fn new(pos: Pos2) -> Self {
        Self {
            current_pos: pos,
            target_pos: pos,
            speed: 800.0, // pixels per second
            animating: false,
        }
    }
    
    /// Update the movement animation
    pub fn update(&mut self, ctx: &Context) {
        if self.animating {
            let delta_time = ctx.input(|i| i.stable_dt);
            let distance = self.target_pos - self.current_pos;
            let distance_length = distance.length();
            
            if distance_length > 1.0 {
                // Move towards target
                let move_distance = self.speed * delta_time;
                let direction = distance / distance_length;
                self.current_pos += direction * move_distance.min(distance_length);
                
                // Request repaint for next frame
                ctx.request_repaint();
            } else {
                // Animation complete
                self.current_pos = self.target_pos;
                self.animating = false;
            }
        }
    }
    
    /// Set new target position and start animation
    pub fn move_to(&mut self, target: Pos2) {
        if (self.target_pos - target).length() > 1.0 {
            self.target_pos = target;
            self.animating = true;
        }
    }
    
    /// Set position immediately without animation
    pub fn set_pos(&mut self, pos: Pos2) {
        self.current_pos = pos;
        self.target_pos = pos;
        self.animating = false;
    }
}

/// Manager for component movement animations
#[derive(Default)]
pub struct MovementManager {
    /// Map of movement animations by component ID
    movements: HashMap<usize, MoveAnimation>,
}

impl MovementManager {
    pub fn new() -> Self {
        Self {
            movements: HashMap::new(),
        }
    }
    
    /// Get or create a movement animation for the given component ID
    pub fn get_or_create(&mut self, component_id: usize, default_pos: Pos2) -> &mut MoveAnimation {
        self.movements.entry(component_id)
            .or_insert_with(|| MoveAnimation::new(default_pos))
    }
    
    /// Update all movement animations
    pub fn update_all(&mut self, ctx: &Context) {
        for animation in self.movements.values_mut() {
            animation.update(ctx);
        }
    }
    
    /// Remove animation for a component
    pub fn remove(&mut self, component_id: usize) {
        self.movements.remove(&component_id);
    }
}