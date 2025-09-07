//! Toast Notification System
//!
//! Modern toast notifications with animations, queueing, and customizable styling.
//! Provides non-intrusive user feedback with automatic dismissal and interaction support.

use egui::*;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Toast notification manager
pub struct ToastManager {
    /// Active toast notifications
    pub toasts: VecDeque<Toast>,
    /// Maximum number of toasts to show simultaneously
    pub max_toasts: usize,
    /// Default toast duration
    pub default_duration: Duration,
    /// Toast positioning
    pub position: ToastPosition,
    /// Animation settings
    pub animation: ToastAnimation,
    /// Global styling
    pub style: ToastStyle,
    /// Performance settings
    pub performance: ToastPerformance,
}

/// Individual toast notification
#[derive(Clone)]
pub struct Toast {
    /// Unique toast ID
    pub id: String,
    /// Toast content
    pub content: ToastContent,
    /// Toast type for styling
    pub toast_type: ToastType,
    /// When the toast was created
    pub created_at: Instant,
    /// How long to show the toast
    pub duration: Duration,
    /// Whether the toast persists until manually dismissed
    pub persistent: bool,
    /// Current animation state
    pub animation_state: ToastAnimationState,
    /// Whether the toast has been interacted with
    pub interacted: bool,
    /// Custom actions
    pub actions: Vec<ToastAction>,
    /// Progress indicator (for long-running operations)
    pub progress: Option<ToastProgress>,
}

/// Toast content types
#[derive(Clone)]
pub enum ToastContent {
    /// Simple text message
    Text(String),
    /// Rich text with formatting
    RichText { 
        title: String, 
        message: String, 
        icon: Option<String> 
    },
    /// Custom widget content
    Custom {
        render_fn: String, // Function name for custom rendering
        data: String,      // JSON data for rendering
    },
    /// Loading state with spinner
    Loading {
        message: String,
        progress: Option<f32>,
    },
}

/// Toast notification types
#[derive(Clone, PartialEq)]
pub enum ToastType {
    /// Informational message
    Info,
    /// Success notification
    Success,
    /// Warning message
    Warning,
    /// Error notification
    Error,
    /// Loading/progress notification
    Loading,
    /// Custom type with custom styling
    Custom(String),
}

/// Toast positioning options
#[derive(Clone)]
pub enum ToastPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    Custom { x: f32, y: f32 },
}

/// Toast animation configuration
#[derive(Clone)]
pub struct ToastAnimation {
    /// Animation duration for entrance
    pub enter_duration: Duration,
    /// Animation duration for exit
    pub exit_duration: Duration,
    /// Animation easing function
    pub easing: AnimationEasing,
    /// Animation type
    pub animation_type: AnimationType,
    /// Enable/disable animations
    pub enabled: bool,
}

/// Animation easing functions
#[derive(Clone)]
pub enum AnimationEasing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bounce,
    Spring,
}

/// Animation types
#[derive(Clone)]
pub enum AnimationType {
    Slide,
    Fade,
    Scale,
    SlideAndFade,
    Custom,
}

/// Current animation state of a toast
#[derive(Clone)]
pub struct ToastAnimationState {
    /// Animation phase
    pub phase: AnimationPhase,
    /// Animation start time
    pub start_time: Instant,
    /// Current animation progress (0.0 to 1.0)
    pub progress: f32,
    /// Target position
    pub target_position: Vec2,
    /// Current position
    pub current_position: Vec2,
    /// Opacity
    pub opacity: f32,
    /// Scale
    pub scale: f32,
}

#[derive(Clone, PartialEq)]
pub enum AnimationPhase {
    Entering,
    Visible,
    Exiting,
    Hidden,
}

/// Toast styling configuration
#[derive(Clone)]
pub struct ToastStyle {
    /// Base toast dimensions
    pub width: f32,
    pub min_height: f32,
    pub max_height: f32,
    /// Spacing between toasts
    pub toast_spacing: f32,
    /// Margin from screen edges
    pub screen_margin: Margin,
    
    /// Colors for different toast types
    pub info_color: Color32,
    pub success_color: Color32,
    pub warning_color: Color32,
    pub error_color: Color32,
    pub loading_color: Color32,
    
    /// Background styling
    pub background_color: Color32,
    pub border_color: Option<Color32>,
    pub border_width: f32,
    pub corner_radius: f32,
    pub shadow: ToastShadow,
    
    /// Text styling
    pub title_font_size: f32,
    pub message_font_size: f32,
    pub text_color: Color32,
    pub secondary_text_color: Color32,
    
    /// Interactive elements
    pub close_button_size: f32,
    pub action_button_style: ButtonStyle,
}

#[derive(Clone)]
pub struct ButtonStyle {
    pub background_color: Color32,
    pub hover_color: Color32,
    pub text_color: Color32,
    pub border_radius: f32,
}

#[derive(Clone)]
pub struct ToastShadow {
    pub enabled: bool,
    pub offset: Vec2,
    pub blur: f32,
    pub color: Color32,
}

/// Toast action (button)
#[derive(Clone)]
pub struct ToastAction {
    /// Action ID
    pub id: String,
    /// Button text
    pub text: String,
    /// Button style
    pub style: ToastActionStyle,
    /// Callback data
    pub callback_data: String,
}

#[derive(Clone)]
pub enum ToastActionStyle {
    Primary,
    Secondary,
    Danger,
    Link,
}

/// Progress indicator for toasts
#[derive(Clone)]
pub struct ToastProgress {
    /// Progress value (0.0 to 1.0)
    pub progress: f32,
    /// Whether to show percentage text
    pub show_percentage: bool,
    /// Progress bar color
    pub color: Option<Color32>,
    /// Indeterminate progress (spinner)
    pub indeterminate: bool,
}

/// Performance settings for toast system
#[derive(Clone)]
pub struct ToastPerformance {
    /// Maximum number of toasts in queue
    pub max_queue_size: usize,
    /// Update frequency for animations (FPS)
    pub animation_fps: u32,
    /// Enable GPU acceleration for animations
    pub hardware_acceleration: bool,
}

impl ToastManager {
    /// Create a new toast manager
    pub fn new() -> Self {
        Self {
            toasts: VecDeque::new(),
            max_toasts: 5,
            default_duration: Duration::from_secs(4),
            position: ToastPosition::TopRight,
            animation: ToastAnimation::default(),
            style: ToastStyle::default(),
            performance: ToastPerformance::default(),
        }
    }
    
    /// Show a simple text toast
    pub fn show_text(&mut self, message: impl Into<String>, toast_type: ToastType) -> String {
        let toast = Toast::new_text(message.into(), toast_type, self.default_duration);
        let id = toast.id.clone();
        self.add_toast(toast);
        id
    }
    
    /// Show a rich text toast with title and message
    pub fn show_rich(&mut self, title: impl Into<String>, message: impl Into<String>, toast_type: ToastType) -> String {
        let toast = Toast::new_rich(title.into(), message.into(), None, toast_type, self.default_duration);
        let id = toast.id.clone();
        self.add_toast(toast);
        id
    }
    
    /// Show a loading toast
    pub fn show_loading(&mut self, message: impl Into<String>) -> String {
        let toast = Toast::new_loading(message.into(), None);
        let id = toast.id.clone();
        self.add_toast(toast);
        id
    }
    
    /// Show a toast with progress
    pub fn show_progress(&mut self, message: impl Into<String>, progress: f32) -> String {
        let toast = Toast::new_loading(message.into(), Some(progress));
        let id = toast.id.clone();
        self.add_toast(toast);
        id
    }
    
    /// Add a custom toast
    pub fn add_toast(&mut self, toast: Toast) {
        // Remove oldest toasts if at max capacity
        while self.toasts.len() >= self.max_toasts {
            self.toasts.pop_front();
        }
        
        self.toasts.push_back(toast);
    }
    
    /// Update a toast (useful for progress updates)
    pub fn update_toast(&mut self, id: &str, update: impl FnOnce(&mut Toast)) {
        if let Some(toast) = self.toasts.iter_mut().find(|t| t.id == id) {
            update(toast);
        }
    }
    
    /// Dismiss a toast by ID
    pub fn dismiss_toast(&mut self, id: &str) {
        if let Some(toast) = self.toasts.iter_mut().find(|t| t.id == id) {
            toast.animation_state.phase = AnimationPhase::Exiting;
            toast.animation_state.start_time = Instant::now();
        }
    }
    
    /// Clear all toasts
    pub fn clear_all(&mut self) {
        self.toasts.clear();
    }
    
    /// Update animations and remove expired toasts
    pub fn update(&mut self) {
        let now = Instant::now();
        
        // Update animation states
        for toast in &mut self.toasts {
            toast.update_animation(now, &self.animation);
        }
        
        // Remove finished toasts
        self.toasts.retain(|toast| {
            match toast.animation_state.phase {
                AnimationPhase::Hidden => false,
                AnimationPhase::Visible => {
                    if !toast.persistent {
                        now.duration_since(toast.created_at) < toast.duration
                    } else {
                        true
                    }
                }
                _ => true,
            }
        });
    }
    
    /// Render all toasts
    pub fn show(&mut self, ui: &mut Ui) {
        self.update();
        
        let screen_rect = ui.ctx().screen_rect();
        let base_position = self.calculate_base_position(&screen_rect);
        
        for (index, toast) in self.toasts.iter_mut().enumerate() {
            let toast_position = self.calculate_toast_position(base_position, index);
            toast.show_at_position(ui, toast_position, &self.style);
        }
    }
    
    /// Calculate base position for toasts
    fn calculate_base_position(&self, screen_rect: &Rect) -> Vec2 {
        match self.position {
            ToastPosition::TopLeft => pos2(
                self.style.screen_margin.left,
                self.style.screen_margin.top
            ),
            ToastPosition::TopCenter => pos2(
                screen_rect.center().x,
                self.style.screen_margin.top
            ),
            ToastPosition::TopRight => pos2(
                screen_rect.max.x - self.style.screen_margin.right - self.style.width,
                self.style.screen_margin.top
            ),
            ToastPosition::BottomLeft => pos2(
                self.style.screen_margin.left,
                screen_rect.max.y - self.style.screen_margin.bottom
            ),
            ToastPosition::BottomCenter => pos2(
                screen_rect.center().x,
                screen_rect.max.y - self.style.screen_margin.bottom
            ),
            ToastPosition::BottomRight => pos2(
                screen_rect.max.x - self.style.screen_margin.right - self.style.width,
                screen_rect.max.y - self.style.screen_margin.bottom
            ),
            ToastPosition::Custom { x, y } => pos2(x, y),
        }.to_vec2()
    }
    
    /// Calculate position for individual toast
    fn calculate_toast_position(&self, base_position: Vec2, index: usize) -> Vec2 {
        let offset_y = index as f32 * (self.style.min_height + self.style.toast_spacing);
        
        match self.position {
            ToastPosition::BottomLeft | ToastPosition::BottomCenter | ToastPosition::BottomRight => {
                base_position - vec2(0.0, offset_y)
            }
            _ => {
                base_position + vec2(0.0, offset_y)
            }
        }
    }
}

impl Toast {
    /// Create a new text toast
    pub fn new_text(message: String, toast_type: ToastType, duration: Duration) -> Self {
        Self {
            id: format!("toast_{}", uuid::Uuid::new_v4().to_string()),
            content: ToastContent::Text(message),
            toast_type,
            created_at: Instant::now(),
            duration,
            persistent: false,
            animation_state: ToastAnimationState::new(),
            interacted: false,
            actions: Vec::new(),
            progress: None,
        }
    }
    
    /// Create a new rich text toast
    pub fn new_rich(title: String, message: String, icon: Option<String>, toast_type: ToastType, duration: Duration) -> Self {
        Self {
            id: format!("toast_{}", uuid::Uuid::new_v4().to_string()),
            content: ToastContent::RichText { title, message, icon },
            toast_type,
            created_at: Instant::now(),
            duration,
            persistent: false,
            animation_state: ToastAnimationState::new(),
            interacted: false,
            actions: Vec::new(),
            progress: None,
        }
    }
    
    /// Create a new loading toast
    pub fn new_loading(message: String, progress: Option<f32>) -> Self {
        Self {
            id: format!("toast_{}", uuid::Uuid::new_v4().to_string()),
            content: ToastContent::Loading { message, progress },
            toast_type: ToastType::Loading,
            created_at: Instant::now(),
            duration: Duration::MAX, // Loading toasts are persistent by default
            persistent: true,
            animation_state: ToastAnimationState::new(),
            interacted: false,
            actions: Vec::new(),
            progress: progress.map(|p| ToastProgress {
                progress: p,
                show_percentage: true,
                color: None,
                indeterminate: false,
            }),
        }
    }
    
    /// Update animation state
    fn update_animation(&mut self, now: Instant, animation: &ToastAnimation) {
        if !animation.enabled {
            self.animation_state.phase = AnimationPhase::Visible;
            self.animation_state.opacity = 1.0;
            self.animation_state.scale = 1.0;
            return;
        }
        
        let elapsed = now.duration_since(self.animation_state.start_time);
        
        match self.animation_state.phase {
            AnimationPhase::Entering => {
                if elapsed >= animation.enter_duration {
                    self.animation_state.phase = AnimationPhase::Visible;
                    self.animation_state.opacity = 1.0;
                    self.animation_state.scale = 1.0;
                } else {
                    let progress = elapsed.as_secs_f32() / animation.enter_duration.as_secs_f32();
                    let eased_progress = Self::apply_easing(progress, &animation.easing);
                    
                    self.animation_state.opacity = eased_progress;
                    self.animation_state.scale = 0.8 + (0.2 * eased_progress);
                }
            }
            AnimationPhase::Exiting => {
                if elapsed >= animation.exit_duration {
                    self.animation_state.phase = AnimationPhase::Hidden;
                } else {
                    let progress = elapsed.as_secs_f32() / animation.exit_duration.as_secs_f32();
                    let eased_progress = Self::apply_easing(progress, &animation.easing);
                    
                    self.animation_state.opacity = 1.0 - eased_progress;
                    self.animation_state.scale = 1.0 - (0.2 * eased_progress);
                }
            }
            _ => {}
        }
    }
    
    /// Apply easing function to progress
    fn apply_easing(progress: f32, easing: &AnimationEasing) -> f32 {
        match easing {
            AnimationEasing::Linear => progress,
            AnimationEasing::EaseIn => progress * progress,
            AnimationEasing::EaseOut => 1.0 - (1.0 - progress) * (1.0 - progress),
            AnimationEasing::EaseInOut => {
                if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
                }
            }
            AnimationEasing::Bounce => {
                if progress < 1.0 / 2.75 {
                    7.5625 * progress * progress
                } else if progress < 2.0 / 2.75 {
                    let p = progress - 1.5 / 2.75;
                    7.5625 * p * p + 0.75
                } else if progress < 2.5 / 2.75 {
                    let p = progress - 2.25 / 2.75;
                    7.5625 * p * p + 0.9375
                } else {
                    let p = progress - 2.625 / 2.75;
                    7.5625 * p * p + 0.984375
                }
            }
            AnimationEasing::Spring => {
                let spring_constant = 4.0;
                let damping = 0.8;
                let t = progress * std::f32::consts::PI;
                (-spring_constant * t).exp() * (t * damping).sin() + 1.0
            }
        }
    }
    
    /// Render toast at specific position
    fn show_at_position(&mut self, ui: &mut Ui, position: Vec2, style: &ToastStyle) {
        if self.animation_state.phase == AnimationPhase::Hidden {
            return;
        }
        
        // Calculate toast color based on type
        let toast_color = self.get_toast_color(style);
        let background_color = Color32::from_rgba_unmultiplied(
            style.background_color.r(),
            style.background_color.g(),
            style.background_color.b(),
            (style.background_color.a() as f32 * self.animation_state.opacity) as u8,
        );
        
        // Create toast area
        let toast_rect = Rect::from_min_size(
            position.to_pos2(),
            vec2(style.width * self.animation_state.scale, style.min_height * self.animation_state.scale),
        );
        
        // Draw toast background
        ui.painter().rect_filled(
            toast_rect,
            style.corner_radius,
            background_color,
        );
        
        // Draw border if enabled
        if let Some(border_color) = style.border_color {
            ui.painter().rect_stroke(
                toast_rect,
                style.corner_radius,
                Stroke::new(style.border_width, border_color),
            );
        }
        
        // Draw accent bar
        let accent_rect = Rect::from_min_size(
            toast_rect.min,
            vec2(4.0, toast_rect.height()),
        );
        ui.painter().rect_filled(accent_rect, 2.0, toast_color);
        
        // Render content
        let content_rect = toast_rect.shrink(8.0);
        let mut content_ui = ui.child_ui(content_rect, Layout::top_down(Align::Left));
        
        match &self.content {
            ToastContent::Text(text) => {
                content_ui.label(RichText::new(text).color(style.text_color));
            }
            ToastContent::RichText { title, message, icon } => {
                if let Some(icon) = icon {
                    content_ui.horizontal(|ui| {
                        ui.label(RichText::new(icon).size(style.title_font_size));
                        ui.vertical(|ui| {
                            ui.label(RichText::new(title).size(style.title_font_size).color(style.text_color));
                            ui.label(RichText::new(message).size(style.message_font_size).color(style.secondary_text_color));
                        });
                    });
                } else {
                    content_ui.label(RichText::new(title).size(style.title_font_size).color(style.text_color));
                    content_ui.label(RichText::new(message).size(style.message_font_size).color(style.secondary_text_color));
                }
            }
            ToastContent::Loading { message, progress } => {
                content_ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label(RichText::new(message).color(style.text_color));
                });
                
                if let Some(progress_val) = progress {
                    let progress_bar = ProgressBar::new(*progress_val)
                        .show_percentage()
                        .desired_width(style.width - 16.0);
                    content_ui.add(progress_bar);
                }
            }
            ToastContent::Custom { .. } => {
                // Custom content rendering would go here
                content_ui.label("Custom content");
            }
        }
        
        // Render actions
        if !self.actions.is_empty() {
            content_ui.horizontal(|ui| {
                for action in &self.actions {
                    if ui.button(&action.text).clicked() {
                        // Handle action click
                        self.interacted = true;
                    }
                }
            });
        }
        
        // Close button
        let close_button_rect = Rect::from_center_size(
            pos2(toast_rect.max.x - 16.0, toast_rect.min.y + 16.0),
            vec2(style.close_button_size, style.close_button_size),
        );
        
        let close_response = ui.interact(close_button_rect, Id::new(("toast_close", &self.id)), Sense::click());
        if close_response.clicked() {
            self.animation_state.phase = AnimationPhase::Exiting;
            self.animation_state.start_time = Instant::now();
        }
        
        // Draw close button
        ui.painter().circle_filled(close_button_rect.center(), style.close_button_size / 2.0, Color32::from_rgba_unmultiplied(255, 255, 255, 40));
        ui.painter().text(
            close_button_rect.center(),
            Align2::CENTER_CENTER,
            "×",
            FontId::proportional(12.0),
            style.text_color,
        );
    }
    
    /// Get toast color based on type
    fn get_toast_color(&self, style: &ToastStyle) -> Color32 {
        match self.toast_type {
            ToastType::Info => style.info_color,
            ToastType::Success => style.success_color,
            ToastType::Warning => style.warning_color,
            ToastType::Error => style.error_color,
            ToastType::Loading => style.loading_color,
            ToastType::Custom(_) => style.info_color, // Default to info color
        }
    }
}

impl ToastAnimationState {
    fn new() -> Self {
        Self {
            phase: AnimationPhase::Entering,
            start_time: Instant::now(),
            progress: 0.0,
            target_position: Vec2::ZERO,
            current_position: Vec2::ZERO,
            opacity: 0.0,
            scale: 0.8,
        }
    }
}

// Default implementations
impl Default for ToastAnimation {
    fn default() -> Self {
        Self {
            enter_duration: Duration::from_millis(300),
            exit_duration: Duration::from_millis(200),
            easing: AnimationEasing::EaseOut,
            animation_type: AnimationType::SlideAndFade,
            enabled: true,
        }
    }
}

impl Default for ToastStyle {
    fn default() -> Self {
        Self {
            width: 320.0,
            min_height: 60.0,
            max_height: 200.0,
            toast_spacing: 8.0,
            screen_margin: Margin::same(16.0),
            
            info_color: Color32::from_rgb(59, 130, 246),
            success_color: Color32::from_rgb(34, 197, 94),
            warning_color: Color32::from_rgb(251, 191, 36),
            error_color: Color32::from_rgb(239, 68, 68),
            loading_color: Color32::from_rgb(168, 85, 247),
            
            background_color: Color32::from_rgb(31, 41, 55),
            border_color: Some(Color32::from_rgb(55, 65, 81)),
            border_width: 1.0,
            corner_radius: 8.0,
            shadow: ToastShadow {
                enabled: true,
                offset: vec2(0.0, 4.0),
                blur: 12.0,
                color: Color32::from_rgba_unmultiplied(0, 0, 0, 25),
            },
            
            title_font_size: 14.0,
            message_font_size: 12.0,
            text_color: Color32::WHITE,
            secondary_text_color: Color32::from_rgb(156, 163, 175),
            
            close_button_size: 16.0,
            action_button_style: ButtonStyle {
                background_color: Color32::from_rgb(59, 130, 246),
                hover_color: Color32::from_rgb(37, 99, 235),
                text_color: Color32::WHITE,
                border_radius: 4.0,
            },
        }
    }
}

impl Default for ToastPerformance {
    fn default() -> Self {
        Self {
            max_queue_size: 50,
            animation_fps: 60,
            hardware_acceleration: true,
        }
    }
}

impl Default for ToastManager {
    fn default() -> Self {
        Self::new()
    }
}