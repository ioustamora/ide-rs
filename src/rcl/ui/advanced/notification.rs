//! Enhanced Notification System component for RCL advanced UI
//!
//! This component provides a comprehensive notification system with support for
//! different notification types, animations, and automatic dismissal.

use egui::{Ui, RichText, Color32};
use crate::rcl::ui::component::Component;
use std::time::{Duration, Instant};

/// Types of notifications with different visual styles
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NotificationKind {
    Info,
    Success,
    Warning,
    Error,
}

/// Individual notification with enhanced features
pub struct Notification {
    pub message: String,
    pub kind: NotificationKind,
    pub editable: bool,
    /// When the notification was created
    pub created_at: Instant,
    /// How long to show the notification (None = permanent)
    pub timeout: Option<Duration>,
    /// Whether this notification can be dismissed
    pub dismissible: bool,
    /// Show icon with the notification
    pub show_icon: bool,
}

impl Default for Notification {
    fn default() -> Self {
        Self {
            message: "Sample notification message".to_string(),
            kind: NotificationKind::Info,
            editable: false,
            created_at: Instant::now(),
            timeout: Some(Duration::from_secs(5)),
            dismissible: true,
            show_icon: true,
        }
    }
}

impl Component for Notification {
    fn name(&self) -> &str {
        "Notification"
    }
    
    fn render(&mut self, ui: &mut Ui) {
        // Check if notification should be auto-dismissed
        let should_dismiss = if let Some(timeout) = self.timeout {
            self.created_at.elapsed() >= timeout
        } else {
            false
        };
        
        if should_dismiss && !self.editable {
            ui.label("⏰ Notification expired");
            return;
        }
        
        if self.editable {
            // Edit mode - show configuration options
            ui.heading("📝 Edit Notification");
            ui.separator();
            
            // Message editing
            ui.horizontal(|ui| {
                ui.label("Message:");
                ui.text_edit_singleline(&mut self.message);
            });
            
            // Kind selection
            ui.horizontal(|ui| {
                ui.label("Type:");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", self.kind))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.kind, NotificationKind::Info, "ℹ️ Info");
                        ui.selectable_value(&mut self.kind, NotificationKind::Success, "✅ Success");
                        ui.selectable_value(&mut self.kind, NotificationKind::Warning, "⚠️ Warning");
                        ui.selectable_value(&mut self.kind, NotificationKind::Error, "❌ Error");
                    });
            });
            
            // Timeout configuration
            ui.horizontal(|ui| {
                ui.label("Auto-dismiss:");
                let mut has_timeout = self.timeout.is_some();
                if ui.checkbox(&mut has_timeout, "Enable").changed() {
                    if has_timeout {
                        self.timeout = Some(Duration::from_secs(5));
                    } else {
                        self.timeout = None;
                    }
                }
                
                if let Some(ref mut timeout) = self.timeout {
                    let mut seconds = timeout.as_secs() as f32;
                    if ui.add(egui::Slider::new(&mut seconds, 1.0..=30.0).suffix("s")).changed() {
                        *timeout = Duration::from_secs(seconds as u64);
                    }
                }
            });
            
            // Display options
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.dismissible, "Dismissible");
                ui.checkbox(&mut self.show_icon, "Show icon");
            });
            
            // Reset timer button
            if ui.button("🔄 Reset Timer").clicked() {
                self.created_at = Instant::now();
            }
            
        } else {
            // Display mode - show the notification
            let (color, icon) = self.get_style();
            let frame = egui::Frame::none()
                .fill(Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 30))
                .stroke(egui::Stroke::new(1.0, color))
                .rounding(4.0)
                .inner_margin(8.0);
            
            let created_at = self.created_at; // Copy the timestamp to avoid borrow issues
            let timeout = self.timeout;
            
            frame.show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Icon
                    if self.show_icon {
                        ui.label(RichText::new(icon).color(color).size(16.0));
                    }
                    
                    // Message
                    ui.label(RichText::new(&self.message).color(color));
                    
                    // Dismiss button
                    if self.dismissible {
                        if ui.small_button("✕").on_hover_text("Dismiss").clicked() {
                            // In a real implementation, this would remove the notification
                            // For now, we'll just restart the timer
                            // Note: This would need to be handled differently in a real app
                        }
                    }
                });
                
                // Show remaining time if applicable
                if let Some(timeout) = timeout {
                    let elapsed = created_at.elapsed();
                    let remaining = timeout.saturating_sub(elapsed);
                    if remaining > Duration::ZERO {
                        let progress = elapsed.as_secs_f32() / timeout.as_secs_f32();
                        ui.add(egui::ProgressBar::new(progress).show_percentage());
                        ui.label(format!("Auto-dismiss in {}s", remaining.as_secs()));
                    }
                }
            });
        }
        
        // Edit toggle button
        if ui.button(if self.editable { "Preview" } else { "Edit" }).clicked() {
            self.editable = !self.editable;
            if !self.editable {
                // Reset timer when switching to preview mode
                self.created_at = Instant::now();
            }
        }
    }
}

impl Notification {
    /// Create a new notification with the specified message and kind
    pub fn new(message: String, kind: NotificationKind) -> Self {
        Self {
            message,
            kind,
            created_at: Instant::now(),
            ..Default::default()
        }
    }
    
    /// Create an info notification
    pub fn info(message: String) -> Self {
        Self::new(message, NotificationKind::Info)
    }
    
    /// Create a success notification
    pub fn success(message: String) -> Self {
        Self::new(message, NotificationKind::Success)
    }
    
    /// Create a warning notification
    pub fn warning(message: String) -> Self {
        Self::new(message, NotificationKind::Warning)
    }
    
    /// Create an error notification
    pub fn error(message: String) -> Self {
        let mut notification = Self::new(message, NotificationKind::Error);
        notification.timeout = None; // Error notifications don't auto-dismiss
        notification
    }
    
    /// Get the color and icon for this notification type
    fn get_style(&self) -> (Color32, &str) {
        match self.kind {
            NotificationKind::Info => (Color32::LIGHT_BLUE, "ℹ️"),
            NotificationKind::Success => (Color32::LIGHT_GREEN, "✅"),
            NotificationKind::Warning => (Color32::YELLOW, "⚠️"),
            NotificationKind::Error => (Color32::LIGHT_RED, "❌"),
        }
    }
    
    /// Check if the notification has expired
    pub fn is_expired(&self) -> bool {
        if let Some(timeout) = self.timeout {
            self.created_at.elapsed() >= timeout
        } else {
            false
        }
    }
    
    /// Get the remaining time before auto-dismissal
    pub fn remaining_time(&self) -> Option<Duration> {
        if let Some(timeout) = self.timeout {
            let elapsed = self.created_at.elapsed();
            if elapsed < timeout {
                Some(timeout - elapsed)
            } else {
                None
            }
        } else {
            None
        }
    }
}
