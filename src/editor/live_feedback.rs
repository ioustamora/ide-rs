//! Live Visual Feedback System for Property Changes
//!
//! This module provides real-time visual feedback when component properties
//! are modified, including live preview and visual synchronization.

use egui::*;
use crate::rcl::ui::component::Component;
use crate::editor::inspector::{PropertyValue, PropertyInspector};
use crate::editor::visual_designer::VisualDesigner;
use std::collections::HashMap;

/// System for providing live visual feedback during property editing
pub struct LiveFeedbackSystem {
    /// Whether live feedback is enabled
    pub enabled: bool,
    /// Preview values that override component properties during editing
    pub preview_values: HashMap<String, PropertyValue>,
    /// Visual feedback overlays
    pub feedback_overlays: Vec<FeedbackOverlay>,
    /// Animation states for smooth transitions
    pub animations: HashMap<String, AnimationState>,
}

/// Visual feedback overlay for property changes
#[derive(Clone)]
pub struct FeedbackOverlay {
    /// Position of the overlay
    pub position: Pos2,
    /// Size of the overlay
    pub size: Vec2,
    /// Overlay type
    pub overlay_type: OverlayType,
    /// Duration remaining for the overlay
    pub duration: f32,
    /// Fade animation progress
    pub fade: f32,
}

/// Types of visual feedback overlays
#[derive(Clone)]
pub enum OverlayType {
    /// Property value tooltip
    PropertyTooltip { property: String, value: String },
    /// Size preview during resize
    SizePreview { width: f32, height: f32 },
    /// Position preview during move
    PositionPreview { x: f32, y: f32 },
    /// Color preview
    ColorPreview { color: Color32 },
    /// Alignment guide
    AlignmentGuide { direction: GuideDirection },
    /// Interactive control for direct manipulation
    InteractiveControl { component_id: usize, control_type: InteractiveControlType },
}

/// Types of interactive controls for direct property manipulation
#[derive(Clone, Copy)]
pub enum InteractiveControlType {
    /// Color picker overlay
    ColorPicker,
    /// Numeric slider overlay
    NumericSlider { min: f32, max: f32 },
    /// Text input overlay
    TextInput,
    /// Checkbox overlay
    Checkbox,
    /// Dropdown overlay
    Dropdown,
}

/// Direction of alignment guides
#[derive(Clone, Copy)]
pub enum GuideDirection {
    Horizontal,
    Vertical,
    Both,
}

/// Animation state for smooth property transitions
#[derive(Clone)]
pub struct AnimationState {
    /// Start value
    pub from: PropertyValue,
    /// Target value
    pub to: PropertyValue,
    /// Animation progress (0.0 to 1.0)
    pub progress: f32,
    /// Animation duration in seconds
    pub duration: f32,
    /// Animation start time
    pub start_time: std::time::Instant,
    /// Animation curve type
    pub curve: AnimationCurve,
}

/// Animation curve types for smooth transitions
#[derive(Clone, Copy)]
pub enum AnimationCurve {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}

impl Default for LiveFeedbackSystem {
    fn default() -> Self {
        Self {
            enabled: true,
            preview_values: HashMap::new(),
            feedback_overlays: Vec::new(),
            animations: HashMap::new(),
        }
    }
}

impl LiveFeedbackSystem {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Apply live property changes to components
    pub fn apply_live_changes(&mut self, 
                             components: &mut [Box<dyn Component>], 
                             visual_designer: &mut VisualDesigner,
                             property_inspector: &PropertyInspector) {
        if !self.enabled {
            return;
        }
        
        // Update component properties based on property inspector values
        for (component_idx, component) in components.iter_mut().enumerate() {
            self.apply_component_properties(component_idx, component, property_inspector, visual_designer);
        }
        
        // Update animations
        self.update_animations();
        
        // Update overlays
        self.update_overlays();
    }
    
    /// Apply property changes to a specific component
    fn apply_component_properties(&mut self, 
                                 component_idx: usize,
                                 component: &mut Box<dyn Component>,
                                 property_inspector: &PropertyInspector,
                                 visual_designer: &mut VisualDesigner) {
        let component_type = component.name();
        
        // Apply basic layout properties to visual designer with live preview
        if let Some(width_value) = property_inspector.get_property_value(component_idx, "width") {
            if let PropertyValue::Number(width) = width_value {
                let current_size = visual_designer.layout.sizes.get(&component_idx).copied()
                    .unwrap_or(Vec2::new(100.0, 30.0));
                let new_size = Vec2::new(*width as f32, current_size.y);
                visual_designer.layout.sizes.insert(component_idx, new_size);
                
                // Add size preview overlay
                if let Some(pos) = visual_designer.layout.positions.get(&component_idx) {
                    self.add_size_preview_overlay(*pos, new_size);
                }
            }
        }
        
        if let Some(height_value) = property_inspector.get_property_value(component_idx, "height") {
            if let PropertyValue::Number(height) = height_value {
                let current_size = visual_designer.layout.sizes.get(&component_idx).copied()
                    .unwrap_or(Vec2::new(100.0, 30.0));
                let new_size = Vec2::new(current_size.x, *height as f32);
                visual_designer.layout.sizes.insert(component_idx, new_size);
                
                // Add size preview overlay
                if let Some(pos) = visual_designer.layout.positions.get(&component_idx) {
                    self.add_size_preview_overlay(*pos, new_size);
                }
            }
        }
        
        // Apply position changes with live preview
        if let Some(x_value) = property_inspector.get_property_value(component_idx, "x") {
            if let PropertyValue::Number(x) = x_value {
                let current_pos = visual_designer.layout.positions.get(&component_idx).copied()
                    .unwrap_or(Pos2::new(0.0, 0.0));
                let new_pos = Pos2::new(*x as f32, current_pos.y);
                visual_designer.layout.positions.insert(component_idx, new_pos);
                
                // Add position preview overlay
                self.add_position_preview_overlay(new_pos);
            }
        }
        
        if let Some(y_value) = property_inspector.get_property_value(component_idx, "y") {
            if let PropertyValue::Number(y) = y_value {
                let current_pos = visual_designer.layout.positions.get(&component_idx).copied()
                    .unwrap_or(Pos2::new(0.0, 0.0));
                let new_pos = Pos2::new(current_pos.x, *y as f32);
                visual_designer.layout.positions.insert(component_idx, new_pos);
                
                // Add position preview overlay
                self.add_position_preview_overlay(new_pos);
            }
        }
        
        // Component-specific property application
        match component_type {
            "Button" => self.apply_button_properties(component_idx, component, property_inspector),
            "Label" => self.apply_label_properties(component_idx, component, property_inspector),
            "TextBox" => self.apply_textbox_properties(component_idx, component, property_inspector),
            _ => {}
        }
    }
    
    /// Apply properties specific to Button components
    fn apply_button_properties(&mut self, 
                              component_idx: usize,
                              _component: &mut Box<dyn Component>,
                              property_inspector: &PropertyInspector) {
        // This would require extending the Component trait to allow property setting
        // For now, we'll add visual feedback overlays
        
        if let Some(PropertyValue::String(label)) = property_inspector.get_property_value(component_idx, "label") {
            // TODO: Update button label when Component trait supports it
            self.add_property_feedback(component_idx, "label", label.clone());
        }
        
        if let Some(PropertyValue::Color(color)) = property_inspector.get_property_value(component_idx, "background_color") {
            let egui_color = Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
            self.add_color_feedback(component_idx, egui_color);
        }
    }
    
    /// Apply properties specific to Label components
    fn apply_label_properties(&mut self,
                             component_idx: usize, 
                             _component: &mut Box<dyn Component>,
                             property_inspector: &PropertyInspector) {
        if let Some(PropertyValue::String(text)) = property_inspector.get_property_value(component_idx, "text") {
            self.add_property_feedback(component_idx, "text", text.clone());
        }
        
        if let Some(PropertyValue::Number(font_size)) = property_inspector.get_property_value(component_idx, "font_size") {
            self.add_property_feedback(component_idx, "font_size", format!("{}px", font_size));
        }
    }
    
    /// Apply properties specific to TextBox components
    fn apply_textbox_properties(&mut self,
                               component_idx: usize,
                               _component: &mut Box<dyn Component>, 
                               property_inspector: &PropertyInspector) {
        if let Some(PropertyValue::String(value)) = property_inspector.get_property_value(component_idx, "value") {
            self.add_property_feedback(component_idx, "value", value.clone());
        }
        
        if let Some(PropertyValue::String(placeholder)) = property_inspector.get_property_value(component_idx, "placeholder") {
            self.add_property_feedback(component_idx, "placeholder", placeholder.clone());
        }
    }
    
    /// Add property feedback overlay
    fn add_property_feedback(&mut self, component_idx: usize, property: &str, value: String) {
        let overlay = FeedbackOverlay {
            position: Pos2::new(0.0, 0.0), // Will be positioned based on component
            size: Vec2::new(150.0, 30.0),
            overlay_type: OverlayType::PropertyTooltip { 
                property: property.to_string(), 
                value 
            },
            duration: 2.0,
            fade: 1.0,
        };
        self.feedback_overlays.push(overlay);
    }
    
    /// Add color feedback overlay
    fn add_color_feedback(&mut self, component_idx: usize, color: Color32) {
        let overlay = FeedbackOverlay {
            position: Pos2::new(0.0, 0.0), // Will be positioned based on component
            size: Vec2::new(20.0, 20.0),
            overlay_type: OverlayType::ColorPreview { color },
            duration: 1.5,
            fade: 1.0,
        };
        self.feedback_overlays.push(overlay);
    }
    
    /// Add size preview overlay
    fn add_size_preview_overlay(&mut self, position: Pos2, size: Vec2) {
        let overlay = FeedbackOverlay {
            position,
            size,
            overlay_type: OverlayType::SizePreview { 
                width: size.x, 
                height: size.y 
            },
            duration: 1.0,
            fade: 1.0,
        };
        self.feedback_overlays.push(overlay);
    }
    
    /// Add position preview overlay
    fn add_position_preview_overlay(&mut self, position: Pos2) {
        let overlay = FeedbackOverlay {
            position,
            size: Vec2::new(20.0, 20.0),
            overlay_type: OverlayType::PositionPreview { 
                x: position.x, 
                y: position.y 
            },
            duration: 1.0,
            fade: 1.0,
        };
        self.feedback_overlays.push(overlay);
    }
    
    /// Render feedback overlays on the design canvas
    pub fn render_overlays(&mut self, ui: &mut Ui, _visual_designer: &VisualDesigner) {
        // First render all painter-based overlays
        {
            let painter = ui.painter();
            for overlay in &self.feedback_overlays {
                match &overlay.overlay_type {
                    OverlayType::PropertyTooltip { property, value } => {
                        self.render_property_tooltip(&painter, overlay, property, value);
                    }
                    OverlayType::SizePreview { width, height } => {
                        self.render_size_preview(&painter, overlay, *width, *height);
                    }
                    OverlayType::PositionPreview { x, y } => {
                        self.render_position_preview(&painter, overlay, *x, *y);
                    }
                    OverlayType::ColorPreview { color } => {
                        self.render_color_preview(&painter, overlay, *color);
                    }
                    OverlayType::AlignmentGuide { direction } => {
                        self.render_alignment_guide(&painter, overlay, *direction);
                    }
                    _ => {} // Interactive controls rendered separately
                }
            }
        }
        
        // Then render interactive controls that need mutable UI access
        let interactive_overlays: Vec<_> = self.feedback_overlays.iter()
            .filter_map(|overlay| {
                if let OverlayType::InteractiveControl { component_id, control_type } = &overlay.overlay_type {
                    Some((overlay.clone(), *component_id, *control_type))
                } else {
                    None
                }
            })
            .collect();
            
        for (overlay, component_id, control_type) in interactive_overlays {
            self.render_interactive_control(ui, &overlay, component_id, control_type);
        }
    }
    
    /// Render property tooltip overlay
    fn render_property_tooltip(&self, painter: &egui::Painter, overlay: &FeedbackOverlay, property: &str, value: &str) {
        let background_color = Color32::from_rgba_unmultiplied(0, 0, 0, (180.0 * overlay.fade) as u8);
        let text_color = Color32::from_rgba_unmultiplied(255, 255, 255, (255.0 * overlay.fade) as u8);
        
        let tooltip_rect = Rect::from_min_size(overlay.position, overlay.size);
        
        // Background
        painter.rect_filled(tooltip_rect, 4.0, background_color);
        painter.rect_stroke(tooltip_rect, 4.0, Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, (100.0 * overlay.fade) as u8)));
        
        // Text
        painter.text(
            tooltip_rect.center(),
            Align2::CENTER_CENTER,
            format!("{}: {}", property, value),
            FontId::default(),
            text_color,
        );
    }
    
    /// Render size preview overlay
    fn render_size_preview(&self, painter: &egui::Painter, overlay: &FeedbackOverlay, width: f32, height: f32) {
        let preview_color = Color32::from_rgba_unmultiplied(0, 150, 255, (100.0 * overlay.fade) as u8);
        let stroke_color = Color32::from_rgba_unmultiplied(0, 150, 255, (200.0 * overlay.fade) as u8);
        
        let preview_rect = Rect::from_min_size(overlay.position, Vec2::new(width, height));
        
        // Preview rectangle
        painter.rect_filled(preview_rect, 2.0, preview_color);
        painter.rect_stroke(preview_rect, 2.0, Stroke::new(2.0, stroke_color));
        
        // Size label
        painter.text(
            preview_rect.center(),
            Align2::CENTER_CENTER,
            format!("{}×{}", width as i32, height as i32),
            FontId::default(),
            Color32::WHITE,
        );
    }
    
    /// Render position preview overlay
    fn render_position_preview(&self, painter: &egui::Painter, overlay: &FeedbackOverlay, x: f32, y: f32) {
        let crosshair_color = Color32::from_rgba_unmultiplied(255, 100, 0, (200.0 * overlay.fade) as u8);
        
        let center = Pos2::new(x, y);
        let size = 10.0;
        
        // Crosshair
        painter.line_segment(
            [Pos2::new(center.x - size, center.y), Pos2::new(center.x + size, center.y)],
            Stroke::new(2.0, crosshair_color),
        );
        painter.line_segment(
            [Pos2::new(center.x, center.y - size), Pos2::new(center.x, center.y + size)],
            Stroke::new(2.0, crosshair_color),
        );
        
        // Position label
        painter.text(
            center + Vec2::new(15.0, -15.0),
            Align2::LEFT_BOTTOM,
            format!("({}, {})", x as i32, y as i32),
            FontId::default(),
            Color32::WHITE,
        );
    }
    
    /// Render color preview overlay
    fn render_color_preview(&self, painter: &egui::Painter, overlay: &FeedbackOverlay, color: Color32) {
        let preview_size = 20.0;
        let color_rect = Rect::from_center_size(overlay.position, Vec2::splat(preview_size));
        
        // Color swatch
        painter.rect_filled(color_rect, 4.0, color);
        painter.rect_stroke(color_rect, 4.0, Stroke::new(2.0, Color32::WHITE));
        
        // Color value text
        painter.text(
            overlay.position + Vec2::new(25.0, 0.0),
            Align2::LEFT_CENTER,
            format!("#{:02x}{:02x}{:02x}", color.r(), color.g(), color.b()),
            FontId::default(),
            Color32::WHITE,
        );
    }
    
    /// Render alignment guide overlay
    fn render_alignment_guide(&self, painter: &egui::Painter, overlay: &FeedbackOverlay, direction: GuideDirection) {
        let guide_color = Color32::from_rgba_unmultiplied(255, 0, 150, (150.0 * overlay.fade) as u8);
        let stroke = Stroke::new(1.0, guide_color);
        
        match direction {
            GuideDirection::Horizontal => {
                painter.line_segment(
                    [overlay.position, overlay.position + Vec2::new(overlay.size.x, 0.0)],
                    stroke,
                );
            }
            GuideDirection::Vertical => {
                painter.line_segment(
                    [overlay.position, overlay.position + Vec2::new(0.0, overlay.size.y)],
                    stroke,
                );
            }
            GuideDirection::Both => {
                // Draw both horizontal and vertical guides
                painter.line_segment(
                    [overlay.position, overlay.position + Vec2::new(overlay.size.x, 0.0)],
                    stroke,
                );
                painter.line_segment(
                    [overlay.position, overlay.position + Vec2::new(0.0, overlay.size.y)],
                    stroke,
                );
            }
        }
    }
    
    /// Render interactive control overlay
    fn render_interactive_control(&mut self, ui: &mut Ui, overlay: &FeedbackOverlay, component_id: usize, control_type: InteractiveControlType) {
        let control_rect = Rect::from_min_size(overlay.position, overlay.size);
        let mut child_ui = ui.child_ui(control_rect, Layout::left_to_right(Align::Center));
        
        match control_type {
            InteractiveControlType::ColorPicker => {
                self.render_color_picker_control(&mut child_ui, component_id);
            }
            InteractiveControlType::NumericSlider { min, max } => {
                self.render_numeric_slider_control(&mut child_ui, component_id, min, max);
            }
            InteractiveControlType::TextInput => {
                self.render_text_input_control(&mut child_ui, component_id);
            }
            InteractiveControlType::Checkbox => {
                self.render_checkbox_control(&mut child_ui, component_id);
            }
            InteractiveControlType::Dropdown => {
                self.render_dropdown_control(&mut child_ui, component_id);
            }
        }
    }
    
    /// Render color picker control
    fn render_color_picker_control(&mut self, ui: &mut Ui, component_id: usize) {
        // Create a floating color picker
        let mut color = Color32::RED; // Default color
        
        // Get current color from preview values
        if let Some(PropertyValue::Color(rgba)) = self.get_preview_value(component_id, "background_color").or_else(|| self.get_preview_value(component_id, "color")) {
            color = Color32::from_rgba_unmultiplied(rgba[0], rgba[1], rgba[2], rgba[3]);
        }
        
        if ui.color_edit_button_srgba(&mut color).changed() {
            let [r, g, b, a] = color.to_array();
            let new_value = PropertyValue::Color([r, g, b, a]);
            self.update_property_with_preview(component_id, "background_color", new_value, Some(ui.cursor().min));
        }
    }
    
    /// Render numeric slider control
    fn render_numeric_slider_control(&mut self, ui: &mut Ui, component_id: usize, min: f32, max: f32) {
        let mut value = (min + max) / 2.0; // Default to middle
        
        // Get current value from preview values
        if let Some(PropertyValue::Number(num)) = self.get_preview_value(component_id, "width").or_else(|| self.get_preview_value(component_id, "height")) {
            value = *num as f32;
        }
        
        if ui.add(egui::Slider::new(&mut value, min..=max)).changed() {
            let new_value = PropertyValue::Number(value as f64);
            self.update_property_with_preview(component_id, "width", new_value, Some(ui.cursor().min));
        }
    }
    
    /// Render text input control
    fn render_text_input_control(&mut self, ui: &mut Ui, component_id: usize) {
        let mut text = String::new();
        
        // Get current text from preview values
        if let Some(PropertyValue::String(string)) = self.get_preview_value(component_id, "text").or_else(|| self.get_preview_value(component_id, "label")) {
            text = string.clone();
        }
        
        if ui.text_edit_singleline(&mut text).changed() {
            let new_value = PropertyValue::String(text);
            self.update_property_with_preview(component_id, "text", new_value, Some(ui.cursor().min));
        }
    }
    
    /// Render checkbox control
    fn render_checkbox_control(&mut self, ui: &mut Ui, component_id: usize) {
        let mut checked = false;
        
        // Get current value from preview values
        if let Some(PropertyValue::Boolean(bool_val)) = self.get_preview_value(component_id, "enabled").or_else(|| self.get_preview_value(component_id, "visible")) {
            checked = *bool_val;
        }
        
        if ui.checkbox(&mut checked, "").changed() {
            let new_value = PropertyValue::Boolean(checked);
            self.update_property_with_preview(component_id, "enabled", new_value, Some(ui.cursor().min));
        }
    }
    
    /// Render dropdown control
    fn render_dropdown_control(&mut self, ui: &mut Ui, component_id: usize) {
        let options = vec!["Option 1".to_string(), "Option 2".to_string(), "Option 3".to_string()];
        let mut selected = options[0].clone();
        
        // Get current value from preview values
        if let Some(PropertyValue::Enum(enum_val)) = self.get_preview_value(component_id, "alignment") {
            selected = enum_val.clone();
        }
        
        ComboBox::from_id_source(format!("dropdown_{}", component_id))
            .selected_text(&selected)
            .show_ui(ui, |ui| {
                for option in &options {
                    if ui.selectable_value(&mut selected, option.clone(), option).clicked() {
                        let new_value = PropertyValue::Enum(selected.clone());
                        self.update_property_with_preview(component_id, "alignment", new_value, Some(ui.cursor().min));
                    }
                }
            });
    }
    
    /// Update animation states
    fn update_animations(&mut self) {
        let current_time = std::time::Instant::now();
        
        self.animations.retain(|_key, animation| {
            let elapsed = current_time.duration_since(animation.start_time).as_secs_f32();
            animation.progress = (elapsed / animation.duration).min(1.0);
            
            // Apply animation curve
            animation.progress = match animation.curve {
                AnimationCurve::Linear => animation.progress,
                AnimationCurve::EaseIn => animation.progress * animation.progress,
                AnimationCurve::EaseOut => 1.0 - (1.0 - animation.progress).powi(2),
                AnimationCurve::EaseInOut => {
                    if animation.progress < 0.5 {
                        2.0 * animation.progress * animation.progress
                    } else {
                        1.0 - 2.0 * (1.0 - animation.progress).powi(2)
                    }
                }
            };
            
            // Keep animation if not finished
            animation.progress < 1.0
        });
    }
    
    /// Update overlay states
    fn update_overlays(&mut self) {
        let dt = 0.016; // Approximate frame time
        
        self.feedback_overlays.iter_mut().for_each(|overlay| {
            overlay.duration -= dt;
            overlay.fade = (overlay.duration / 2.0).min(1.0).max(0.0);
        });
        
        // Remove expired overlays
        self.feedback_overlays.retain(|overlay| overlay.duration > 0.0);
    }
    
    /// Start an animation for a property change
    pub fn animate_property_change(&mut self, 
                                  component_idx: usize,
                                  property: &str,
                                  from: PropertyValue,
                                  to: PropertyValue) {
        let animation_key = format!("{}_{}", component_idx, property);
        
        let animation = AnimationState {
            from,
            to,
            progress: 0.0,
            duration: 0.3, // 300ms animation
            start_time: std::time::Instant::now(),
            curve: AnimationCurve::EaseOut,
        };
        
        self.animations.insert(animation_key, animation);
    }
    
    /// Get current animated value for a property
    pub fn get_animated_value(&self, component_idx: usize, property: &str) -> Option<PropertyValue> {
        let animation_key = format!("{}_{}", component_idx, property);
        
        if let Some(animation) = self.animations.get(&animation_key) {
            Some(self.interpolate_property_value(&animation.from, &animation.to, animation.progress))
        } else {
            None
        }
    }
    
    /// Interpolate between two property values
    fn interpolate_property_value(&self, from: &PropertyValue, to: &PropertyValue, t: f32) -> PropertyValue {
        match (from, to) {
            (PropertyValue::Number(a), PropertyValue::Number(b)) => {
                PropertyValue::Number(a + (b - a) * t as f64)
            }
            (PropertyValue::Integer(a), PropertyValue::Integer(b)) => {
                PropertyValue::Integer(a + ((b - a) as f32 * t) as i32)
            }
            (PropertyValue::Color(a), PropertyValue::Color(b)) => {
                let r = (a[0] as f32 + (b[0] as f32 - a[0] as f32) * t) as u8;
                let g = (a[1] as f32 + (b[1] as f32 - a[1] as f32) * t) as u8;
                let b_val = (a[2] as f32 + (b[2] as f32 - a[2] as f32) * t) as u8;
                let alpha = (a[3] as f32 + (b[3] as f32 - a[3] as f32) * t) as u8;
                PropertyValue::Color([r, g, b_val, alpha])
            }
            _ => to.clone(), // For non-interpolatable types, just use the target value
        }
    }
    
    /// Add a feedback overlay
    pub fn add_overlay(&mut self, overlay: FeedbackOverlay) {
        self.feedback_overlays.push(overlay);
    }
    
    /// Clear all overlays
    pub fn clear_overlays(&mut self) {
        self.feedback_overlays.clear();
    }
    
    /// Enable or disable live feedback
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.clear_overlays();
            self.animations.clear();
        }
    }
    
    /// Add interactive control overlay for direct manipulation
    pub fn add_interactive_control(&mut self, component_id: usize, position: Pos2, size: Vec2, control_type: InteractiveControlType) {
        let overlay = FeedbackOverlay {
            position,
            size,
            overlay_type: OverlayType::InteractiveControl { 
                component_id, 
                control_type 
            },
            duration: f32::INFINITY, // Persistent until removed
            fade: 1.0,
        };
        self.feedback_overlays.push(overlay);
    }
    
    /// Remove interactive controls for a component
    pub fn remove_interactive_controls(&mut self, component_id: usize) {
        self.feedback_overlays.retain(|overlay| {
            !matches!(overlay.overlay_type, OverlayType::InteractiveControl { component_id: id, .. } if id == component_id)
        });
    }
    
    /// Update property value with live preview
    pub fn update_property_with_preview(&mut self, 
                                       component_id: usize, 
                                       property: &str, 
                                       value: PropertyValue,
                                       position: Option<Pos2>) {
        // Store preview value
        let key = format!("{}_{}", component_id, property);
        self.preview_values.insert(key, value.clone());
        
        // Add visual feedback
        if let Some(pos) = position {
            match &value {
                PropertyValue::Color(color) => {
                    let egui_color = Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
                    self.add_color_feedback(component_id, egui_color);
                }
                PropertyValue::Number(num) => {
                    self.add_property_feedback(component_id, property, format!("{:.1}", num));
                }
                PropertyValue::String(text) => {
                    self.add_property_feedback(component_id, property, text.clone());
                }
                _ => {}
            }
        }
    }
    
    /// Get preview value for a property (overrides actual component value during live editing)
    pub fn get_preview_value(&self, component_id: usize, property: &str) -> Option<&PropertyValue> {
        let key = format!("{}_{}", component_id, property);
        self.preview_values.get(&key)
    }
    
    /// Commit all preview values (make them permanent)
    pub fn commit_preview_values(&mut self) {
        self.preview_values.clear();
        self.clear_overlays();
    }
    
    /// Cancel all preview values (revert to original)
    pub fn cancel_preview_values(&mut self) {
        self.preview_values.clear();
        self.clear_overlays();
    }
}