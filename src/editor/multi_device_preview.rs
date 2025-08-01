//! Multi-Device Preview System
//!
//! Inspired by Embarcadero FireMonkey's Multi-Device Designer, this module provides
//! real-time preview of UI designs across different device form factors and platforms.
//! 
//! Features:
//! - Side-by-side device preview
//! - Responsive layout testing
//! - Platform-specific styling
//! - Orientation support
//! - Custom device profiles

use egui::*;
use std::collections::HashMap;

/// Multi-device preview manager
pub struct MultiDevicePreview {
    /// Available device profiles
    pub device_profiles: Vec<DeviceProfile>,
    /// Currently selected devices for preview
    pub active_previews: Vec<usize>,
    /// Preview layout configuration
    pub layout_config: PreviewLayout,
    /// Zoom levels for each preview
    pub zoom_levels: HashMap<usize, f32>,
    /// Whether preview is enabled
    pub enabled: bool,
    /// Preview panel visibility
    pub show_preview_panel: bool,
}

/// Device profile definitions
#[derive(Debug, Clone)]
pub struct DeviceProfile {
    /// Device name
    pub name: String,
    /// Screen resolution
    pub resolution: Vec2,
    /// Device type category
    pub device_type: DeviceType,
    /// Platform information
    pub platform: Platform,
    /// DPI scaling factor
    pub dpi_scale: f32,
    /// Safe area insets (for mobile devices)
    pub safe_area: SafeArea,
    /// Device orientation
    pub orientation: Orientation,
    /// Custom styling overrides
    pub style_overrides: HashMap<String, String>,
}

/// Device type categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Phone,
    Tablet,
    Desktop,
    Laptop,
    Watch,
    TV,
    Custom,
}

/// Platform types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
    IOS,
    Android,
    Web,
    Custom,
}

/// Device orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Portrait,
    Landscape,
    Auto,
}

/// Safe area configuration for mobile devices
#[derive(Debug, Clone)]
pub struct SafeArea {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

/// Preview layout configuration
#[derive(Debug, Clone)]
pub struct PreviewLayout {
    /// Layout mode (grid, list, custom)
    pub mode: PreviewLayoutMode,
    /// Number of columns in grid mode
    pub columns: usize,
    /// Spacing between previews
    pub spacing: f32,
    /// Whether to show device labels
    pub show_labels: bool,
    /// Whether to show device frames
    pub show_frames: bool,
}

/// Preview layout modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewLayoutMode {
    Grid,
    HorizontalList,
    VerticalList,
    Custom,
}

/// Responsive breakpoint system
#[derive(Debug, Clone)]
pub struct ResponsiveBreakpoints {
    pub breakpoints: Vec<Breakpoint>,
}

/// Individual breakpoint definition
#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub name: String,
    pub min_width: f32,
    pub max_width: Option<f32>,
    pub style_overrides: HashMap<String, String>,
}

impl Default for MultiDevicePreview {
    fn default() -> Self {
        Self {
            device_profiles: Self::create_default_profiles(),
            active_previews: vec![0, 1, 2], // Default to phone, tablet, desktop
            layout_config: PreviewLayout::default(),
            zoom_levels: HashMap::new(),
            enabled: false,
            show_preview_panel: false,
        }
    }
}

impl Default for PreviewLayout {
    fn default() -> Self {
        Self {
            mode: PreviewLayoutMode::Grid,
            columns: 3,
            spacing: 10.0,
            show_labels: true,
            show_frames: true,
        }
    }
}

impl Default for SafeArea {
    fn default() -> Self {
        Self {
            top: 0.0,
            bottom: 0.0,
            left: 0.0,
            right: 0.0,
        }
    }
}

impl MultiDevicePreview {
    /// Create new multi-device preview system
    pub fn new() -> Self {
        Self::default()
    }

    /// Create default device profiles (FireMonkey-inspired)
    fn create_default_profiles() -> Vec<DeviceProfile> {
        vec![
            // Mobile Phones
            DeviceProfile {
                name: "iPhone 15".to_string(),
                resolution: Vec2::new(393.0, 852.0),
                device_type: DeviceType::Phone,
                platform: Platform::IOS,
                dpi_scale: 3.0,
                safe_area: SafeArea { top: 47.0, bottom: 34.0, left: 0.0, right: 0.0 },
                orientation: Orientation::Portrait,
                style_overrides: HashMap::new(),
            },
            DeviceProfile {
                name: "Samsung Galaxy S24".to_string(),
                resolution: Vec2::new(384.0, 854.0),
                device_type: DeviceType::Phone,
                platform: Platform::Android,
                dpi_scale: 2.75,
                safe_area: SafeArea::default(),
                orientation: Orientation::Portrait,
                style_overrides: HashMap::new(),
            },
            
            // Tablets
            DeviceProfile {
                name: "iPad Pro 12.9\"".to_string(),
                resolution: Vec2::new(1024.0, 1366.0),
                device_type: DeviceType::Tablet,
                platform: Platform::IOS,
                dpi_scale: 2.0,
                safe_area: SafeArea::default(),
                orientation: Orientation::Portrait,
                style_overrides: HashMap::new(),
            },
            DeviceProfile {
                name: "Surface Pro".to_string(),
                resolution: Vec2::new(1368.0, 912.0),
                device_type: DeviceType::Tablet,
                platform: Platform::Windows,
                dpi_scale: 1.5,
                safe_area: SafeArea::default(),
                orientation: Orientation::Landscape,
                style_overrides: HashMap::new(),
            },
            
            // Desktop
            DeviceProfile {
                name: "Windows Desktop".to_string(),
                resolution: Vec2::new(1920.0, 1080.0),
                device_type: DeviceType::Desktop,
                platform: Platform::Windows,
                dpi_scale: 1.0,
                safe_area: SafeArea::default(),
                orientation: Orientation::Landscape,
                style_overrides: HashMap::new(),
            },
            DeviceProfile {
                name: "macOS Desktop".to_string(),
                resolution: Vec2::new(1920.0, 1080.0),
                device_type: DeviceType::Desktop,
                platform: Platform::MacOS,
                dpi_scale: 2.0,
                safe_area: SafeArea::default(),
                orientation: Orientation::Landscape,
                style_overrides: HashMap::new(),
            },
            
            // Watch
            DeviceProfile {
                name: "Apple Watch".to_string(),
                resolution: Vec2::new(176.0, 215.0),
                device_type: DeviceType::Watch,
                platform: Platform::IOS,
                dpi_scale: 2.0,
                safe_area: SafeArea::default(),
                orientation: Orientation::Portrait,
                style_overrides: HashMap::new(),
            },
        ]
    }

    /// Toggle multi-device preview
    pub fn toggle_preview(&mut self) {
        self.enabled = !self.enabled;
        self.show_preview_panel = self.enabled;
    }

    /// Add device to active previews
    pub fn add_device_to_preview(&mut self, device_index: usize) {
        if device_index < self.device_profiles.len() && !self.active_previews.contains(&device_index) {
            self.active_previews.push(device_index);
            self.zoom_levels.insert(device_index, 1.0);
        }
    }

    /// Remove device from active previews
    pub fn remove_device_from_preview(&mut self, device_index: usize) {
        self.active_previews.retain(|&x| x != device_index);
        self.zoom_levels.remove(&device_index);
    }

    /// Set zoom level for specific device preview
    pub fn set_zoom_level(&mut self, device_index: usize, zoom: f32) {
        self.zoom_levels.insert(device_index, zoom.clamp(0.1, 5.0));
    }

    /// Get zoom level for device
    pub fn get_zoom_level(&self, device_index: usize) -> f32 {
        self.zoom_levels.get(&device_index).copied().unwrap_or(1.0)
    }

    /// Render preview panel
    pub fn render_preview_panel(&mut self, ui: &mut Ui, components: &[Box<dyn crate::rcl::ui::component::Component>]) {
        if !self.show_preview_panel {
            return;
        }

        ui.vertical(|ui| {
            // Preview controls
            ui.horizontal(|ui| {
                ui.heading("📱 Multi-Device Preview");
                
                if ui.button("⚙️ Settings").clicked() {
                    // TODO: Open preview settings dialog
                }
                
                if ui.button("➕ Add Device").clicked() {
                    // TODO: Open device selection dialog
                }
                
                ui.separator();
                
                // Layout mode selector
                ui.label("Layout:");
                if ui.selectable_label(
                    self.layout_config.mode == PreviewLayoutMode::Grid, 
                    "Grid"
                ).clicked() {
                    self.layout_config.mode = PreviewLayoutMode::Grid;
                }
                if ui.selectable_label(
                    self.layout_config.mode == PreviewLayoutMode::HorizontalList, 
                    "Horizontal"
                ).clicked() {
                    self.layout_config.mode = PreviewLayoutMode::HorizontalList;
                }
            });

            ui.separator();

            // Render device previews
            match self.layout_config.mode {
                PreviewLayoutMode::Grid => self.render_grid_layout(ui, components),
                PreviewLayoutMode::HorizontalList => self.render_horizontal_layout(ui, components),
                PreviewLayoutMode::VerticalList => self.render_vertical_layout(ui, components),
                PreviewLayoutMode::Custom => self.render_custom_layout(ui, components),
            }
        });
    }

    /// Render grid layout of device previews
    fn render_grid_layout(&mut self, ui: &mut Ui, components: &[Box<dyn crate::rcl::ui::component::Component>]) {
        let available_width = ui.available_width();
        let columns = self.layout_config.columns;
        let spacing = self.layout_config.spacing;
        let preview_width = (available_width - (spacing * (columns - 1) as f32)) / columns as f32;

        ui.spacing_mut().item_spacing = Vec2::splat(spacing);

        // Group previews into rows
        for row_start in (0..self.active_previews.len()).step_by(columns) {
            ui.horizontal(|ui| {
                for col in 0..columns {
                    let preview_index = row_start + col;
                    if preview_index < self.active_previews.len() {
                        let device_index = self.active_previews[preview_index];
                        ui.allocate_ui_with_layout(
                            Vec2::new(preview_width, ui.available_height()),
                            Layout::top_down(Align::Center),
                            |ui| {
                                self.render_device_preview(ui, device_index, components);
                            },
                        );
                    }
                }
            });
        }
    }

    /// Render horizontal list layout
    fn render_horizontal_layout(&mut self, ui: &mut Ui, components: &[Box<dyn crate::rcl::ui::component::Component>]) {
        let active_previews = self.active_previews.clone();
        let spacing = self.layout_config.spacing;
        
        ScrollArea::horizontal().show(ui, |ui| {
            ui.horizontal(|ui| {
                for device_index in active_previews {
                    ui.vertical(|ui| {
                        self.render_device_preview(ui, device_index, components);
                    });
                    ui.add_space(spacing);
                }
            });
        });
    }

    /// Render vertical list layout
    fn render_vertical_layout(&mut self, ui: &mut Ui, components: &[Box<dyn crate::rcl::ui::component::Component>]) {
        let active_previews = self.active_previews.clone();
        let spacing = self.layout_config.spacing;
        
        ScrollArea::vertical().show(ui, |ui| {
            for device_index in active_previews {
                self.render_device_preview(ui, device_index, components);
                ui.add_space(spacing);
            }
        });
    }

    /// Render custom layout (placeholder)
    fn render_custom_layout(&mut self, ui: &mut Ui, components: &[Box<dyn crate::rcl::ui::component::Component>]) {
        // Fallback to grid layout for now
        self.render_grid_layout(ui, components);
    }

    /// Render individual device preview
    fn render_device_preview(&mut self, ui: &mut Ui, device_index: usize, _components: &[Box<dyn crate::rcl::ui::component::Component>]) {
        if device_index >= self.device_profiles.len() {
            return;
        }

        // Clone necessary data to avoid borrowing issues
        let device = self.device_profiles[device_index].clone();
        let zoom = self.get_zoom_level(device_index);
        let show_labels = self.layout_config.show_labels;
        let show_frames = self.layout_config.show_frames;

        ui.group(|ui| {
            // Device label
            if show_labels {
                ui.horizontal(|ui| {
                    ui.label(&device.name);
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        // Zoom controls
                        if ui.small_button("🔍+").clicked() {
                            let new_zoom = (zoom * 1.2).min(5.0);
                            self.set_zoom_level(device_index, new_zoom);
                        }
                        if ui.small_button("🔍-").clicked() {
                            let new_zoom = (zoom / 1.2).max(0.1);
                            self.set_zoom_level(device_index, new_zoom);
                        }
                        ui.label(format!("{:.0}%", zoom * 100.0));
                        
                        // Remove device button
                        if ui.small_button("❌").clicked() {
                            self.remove_device_from_preview(device_index);
                        }
                    });
                });
            }

            // Device frame
            let scaled_size = device.resolution * zoom;
            let max_preview_size = Vec2::new(300.0, 400.0); // Maximum preview size
            let display_size = scaled_size.min(max_preview_size);

            let (rect, _response) = ui.allocate_exact_size(display_size, Sense::hover());

            // Draw device frame
            if show_frames {
                let frame_color = match device.platform {
                    Platform::IOS => Color32::from_rgb(29, 29, 31),
                    Platform::Android => Color32::from_rgb(33, 150, 243),
                    Platform::Windows => Color32::from_rgb(0, 120, 215),
                    Platform::MacOS => Color32::from_rgb(99, 99, 102),
                    _ => Color32::GRAY,
                };

                ui.painter().rect_stroke(
                    rect,
                    Rounding::same(8.0),
                    Stroke::new(2.0, frame_color),
                );
            }

            // Draw screen content (placeholder)
            let screen_rect = rect.shrink(4.0);
            ui.painter().rect_filled(
                screen_rect,
                Rounding::same(4.0),
                Color32::from_rgb(245, 245, 245),
            );

            // Add platform-specific styling hints
            ui.painter().text(
                screen_rect.center(),
                Align2::CENTER_CENTER,
                format!("{}\n{}×{}\n{:?}", 
                    device.name, 
                    device.resolution.x as i32, 
                    device.resolution.y as i32,
                    device.platform
                ),
                FontId::monospace(10.0),
                Color32::DARK_GRAY,
            );

            // TODO: Render actual UI components scaled for this device
        });
    }

    /// Create responsive breakpoints from device profiles
    pub fn create_responsive_breakpoints(&self) -> ResponsiveBreakpoints {
        let mut breakpoints = vec![
            Breakpoint {
                name: "Mobile".to_string(),
                min_width: 0.0,
                max_width: Some(768.0),
                style_overrides: HashMap::new(),
            },
            Breakpoint {
                name: "Tablet".to_string(),
                min_width: 768.0,
                max_width: Some(1024.0),
                style_overrides: HashMap::new(),
            },
            Breakpoint {
                name: "Desktop".to_string(),
                min_width: 1024.0,
                max_width: None,
                style_overrides: HashMap::new(),
            },
        ];

        // Add device-specific breakpoints
        for device in &self.device_profiles {
            if device.device_type == DeviceType::Watch {
                breakpoints.push(Breakpoint {
                    name: "Watch".to_string(),
                    min_width: 0.0,
                    max_width: Some(200.0),
                    style_overrides: HashMap::new(),
                });
            }
        }

        ResponsiveBreakpoints { breakpoints }
    }

    /// Get appropriate device profile for given screen size
    pub fn get_best_device_for_size(&self, size: Vec2) -> Option<&DeviceProfile> {
        self.device_profiles
            .iter()
            .min_by(|a, b| {
                let a_diff = (a.resolution - size).length();
                let b_diff = (b.resolution - size).length();
                a_diff.partial_cmp(&b_diff).unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    /// Export device configuration (simplified)
    pub fn export_device_config(&self) -> String {
        format!("Device profiles: {} devices configured", self.device_profiles.len())
    }

    /// Add custom device profile
    pub fn add_custom_device(&mut self, profile: DeviceProfile) {
        self.device_profiles.push(profile);
    }
}

impl DeviceProfile {
    /// Get effective resolution considering orientation
    pub fn effective_resolution(&self) -> Vec2 {
        match self.orientation {
            Orientation::Portrait => self.resolution,
            Orientation::Landscape => Vec2::new(self.resolution.y, self.resolution.x),
            Orientation::Auto => self.resolution, // Default to portrait
        }
    }

    /// Get safe area adjusted resolution
    pub fn safe_area_resolution(&self) -> Vec2 {
        let effective = self.effective_resolution();
        Vec2::new(
            effective.x - self.safe_area.left - self.safe_area.right,
            effective.y - self.safe_area.top - self.safe_area.bottom,
        )
    }

    /// Check if device supports given feature
    pub fn supports_feature(&self, feature: &str) -> bool {
        match feature {
            "touch" => matches!(self.device_type, DeviceType::Phone | DeviceType::Tablet | DeviceType::Watch),
            "mouse" => matches!(self.device_type, DeviceType::Desktop | DeviceType::Laptop),
            "keyboard" => matches!(self.device_type, DeviceType::Desktop | DeviceType::Laptop),
            "camera" => matches!(self.device_type, DeviceType::Phone | DeviceType::Tablet),
            "gps" => matches!(self.device_type, DeviceType::Phone | DeviceType::Watch),
            _ => false,
        }
    }
}