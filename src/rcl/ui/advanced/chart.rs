//! Chart and Data Visualization Components for RCL
//!
//! This module provides comprehensive charting capabilities including
//! line charts, bar charts, pie charts, and real-time data visualization.

use egui::{Ui, Vec2, Pos2, Color32, Stroke, Rect};
use crate::rcl::ui::component::Component;
// use std::collections::HashMap;

/// Main chart component with multiple chart types
pub struct Chart {
    /// Chart data points
    pub data: ChartData,
    /// Chart configuration
    pub config: ChartConfig,
    /// Whether the chart is in edit mode
    pub editable: bool,
    /// Animation state for smooth transitions
    pub animation: ChartAnimation,
}

/// Chart data container
#[derive(Clone, Debug)]
pub struct ChartData {
    /// Data series for multi-series charts
    pub series: Vec<DataSeries>,
    /// X-axis labels
    pub x_labels: Vec<String>,
    /// Y-axis range (min, max)
    pub y_range: (f32, f32),
}

/// Individual data series
#[derive(Clone, Debug)]
pub struct DataSeries {
    /// Series name
    pub name: String,
    /// Data points (x, y)
    pub points: Vec<(f32, f32)>,
    /// Series color
    pub color: Color32,
    /// Series style
    pub style: SeriesStyle,
}

/// Chart configuration
#[derive(Clone, Debug)]
pub struct ChartConfig {
    /// Type of chart
    pub chart_type: ChartType,
    /// Chart title
    pub title: String,
    /// X-axis label
    pub x_label: String,
    /// Y-axis label
    pub y_label: String,
    /// Show grid lines
    pub show_grid: bool,
    /// Show legend
    pub show_legend: bool,
    /// Chart size
    pub size: Vec2,
    /// Background color
    pub background_color: Color32,
    /// Grid color
    pub grid_color: Color32,
}

/// Types of charts supported
#[derive(Clone, Debug, PartialEq)]
pub enum ChartType {
    Line,
    Bar,
    Pie,
    Scatter,
    Area,
    Histogram,
}

/// Series visual styles
#[derive(Clone, Debug)]
pub enum SeriesStyle {
    Line { thickness: f32, dashed: bool },
    Bar { width: f32, filled: bool },
    Point { radius: f32, shape: PointShape },
    Area { alpha: f32 },
}

/// Point shapes for scatter plots
#[derive(Clone, Debug)]
pub enum PointShape {
    Circle,
    Square,
    Triangle,
    Diamond,
}

/// Animation configuration
#[derive(Clone, Debug)]
pub struct ChartAnimation {
    /// Animation enabled
    pub enabled: bool,
    /// Animation progress (0.0 to 1.0)
    pub progress: f32,
    /// Animation duration in seconds
    pub duration: f32,
    /// Animation start time
    pub start_time: Option<std::time::Instant>,
}

impl Default for Chart {
    fn default() -> Self {
        // Create sample data
        let sample_series = vec![
            DataSeries {
                name: "Series 1".to_string(),
                points: vec![
                    (0.0, 10.0), (1.0, 25.0), (2.0, 15.0), (3.0, 40.0), 
                    (4.0, 30.0), (5.0, 50.0), (6.0, 35.0)
                ],
                color: Color32::BLUE,
                style: SeriesStyle::Line { thickness: 2.0, dashed: false },
            },
            DataSeries {
                name: "Series 2".to_string(),
                points: vec![
                    (0.0, 5.0), (1.0, 20.0), (2.0, 25.0), (3.0, 30.0), 
                    (4.0, 45.0), (5.0, 35.0), (6.0, 55.0)
                ],
                color: Color32::RED,
                style: SeriesStyle::Line { thickness: 2.0, dashed: true },
            },
        ];

        let data = ChartData {
            series: sample_series,
            x_labels: vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul"]
                .into_iter().map(String::from).collect(),
            y_range: (0.0, 60.0),
        };

        let config = ChartConfig {
            chart_type: ChartType::Line,
            title: "Sample Chart".to_string(),
            x_label: "Time".to_string(),
            y_label: "Value".to_string(),
            show_grid: true,
            show_legend: true,
            size: Vec2::new(400.0, 300.0),
            background_color: Color32::WHITE,
            grid_color: Color32::LIGHT_GRAY,
        };

        Self {
            data,
            config,
            editable: false,
            animation: ChartAnimation {
                enabled: true,
                progress: 1.0,
                duration: 1.0,
                start_time: None,
            },
        }
    }
}

impl Component for Chart {
    fn name(&self) -> &str {
        "Chart"
    }
    
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            self.render_editor(ui);
        } else {
            self.render_chart(ui);
        }
        
        // Edit toggle
        if ui.button(if self.editable { "View Chart" } else { "Edit Chart" }).clicked() {
            self.editable = !self.editable;
            if !self.editable {
                self.start_animation();
            }
        }
    }
}

impl Chart {
    /// Render the chart editor interface
    fn render_editor(&mut self, ui: &mut Ui) {
        ui.heading("📊 Chart Editor");
        ui.separator();
        
        // Chart type selection
        ui.horizontal(|ui| {
            ui.label("Chart Type:");
            egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", self.config.chart_type))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.config.chart_type, ChartType::Line, "Line Chart");
                    ui.selectable_value(&mut self.config.chart_type, ChartType::Bar, "Bar Chart");
                    ui.selectable_value(&mut self.config.chart_type, ChartType::Pie, "Pie Chart");
                    ui.selectable_value(&mut self.config.chart_type, ChartType::Scatter, "Scatter Plot");
                    ui.selectable_value(&mut self.config.chart_type, ChartType::Area, "Area Chart");
                });
        });
        
        // Chart configuration
        ui.horizontal(|ui| {
            ui.label("Title:");
            ui.text_edit_singleline(&mut self.config.title);
        });
        
        ui.horizontal(|ui| {
            ui.label("X-Axis:");
            ui.text_edit_singleline(&mut self.config.x_label);
        });
        
        ui.horizontal(|ui| {
            ui.label("Y-Axis:");
            ui.text_edit_singleline(&mut self.config.y_label);
        });
        
        // Visual options
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.config.show_grid, "Show Grid");
            ui.checkbox(&mut self.config.show_legend, "Show Legend");
            ui.checkbox(&mut self.animation.enabled, "Animate");
        });
        
        // Size controls
        ui.horizontal(|ui| {
            ui.label("Size:");
            ui.add(egui::Slider::new(&mut self.config.size.x, 200.0..=800.0).suffix("px").text("Width"));
            ui.add(egui::Slider::new(&mut self.config.size.y, 150.0..=600.0).suffix("px").text("Height"));
        });
        
        ui.separator();
        
        // Data series editor
        ui.heading("Data Series");
        
        for (i, series) in self.data.series.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label(format!("Series {}:", i + 1));
                ui.text_edit_singleline(&mut series.name);
                
                // Color picker (simplified)
                if ui.button("🎨").clicked() {
                    series.color = match i % 6 {
                        0 => Color32::BLUE,
                        1 => Color32::RED,
                        2 => Color32::GREEN,
                        3 => Color32::YELLOW,
                        4 => Color32::LIGHT_BLUE,
                        _ => Color32::GRAY,
                    };
                }
                
                if ui.button("🗑").clicked() {
                    // Mark for removal (simplified)
                }
            });
        }
        
        if ui.button("➕ Add Series").clicked() {
            self.add_sample_series();
        }
        
        // Data generation buttons
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("📈 Generate Sample Data").clicked() {
                self.generate_sample_data();
            }
            if ui.button("🎲 Random Data").clicked() {
                self.generate_random_data();
            }
            if ui.button("📊 Reset Chart").clicked() {
                *self = Chart::default();
            }
        });
    }
    
    /// Render the actual chart
    fn render_chart(&mut self, ui: &mut Ui) {
        // Update animation
        self.update_animation();
        
        // Chart title
        if !self.config.title.is_empty() {
            ui.heading(&self.config.title);
        }
        
        // Main chart area
        let (response, painter) = ui.allocate_painter(self.config.size, egui::Sense::hover());
        let chart_rect = response.rect;
        
        // Background
        painter.rect_filled(chart_rect, 2.0, self.config.background_color);
        painter.rect_stroke(chart_rect, 2.0, Stroke::new(1.0, Color32::BLACK));
        
        // Calculate drawing area (leaving space for axes)
        let margin = 40.0;
        let plot_rect = Rect::from_min_size(
            chart_rect.min + Vec2::new(margin, margin),
            chart_rect.size() - Vec2::new(2.0 * margin, 2.0 * margin),
        );
        
        // Draw grid
        if self.config.show_grid {
            self.draw_grid(&painter, plot_rect);
        }
        
        // Draw axes
        self.draw_axes(&painter, chart_rect, plot_rect);
        
        // Draw data based on chart type
        match self.config.chart_type {
            ChartType::Line => self.draw_line_chart(&painter, plot_rect),
            ChartType::Bar => self.draw_bar_chart(&painter, plot_rect),
            ChartType::Pie => self.draw_pie_chart(&painter, plot_rect),
            ChartType::Scatter => self.draw_scatter_plot(&painter, plot_rect),
            ChartType::Area => self.draw_area_chart(&painter, plot_rect),
            ChartType::Histogram => self.draw_histogram(&painter, plot_rect),
        }
        
        // Draw legend
        if self.config.show_legend {
            self.draw_legend(&painter, chart_rect);
        }
        
        // Chart statistics
        ui.horizontal(|ui| {
            ui.label(format!("📊 Series: {}", self.data.series.len()));
            if let Some(first_series) = self.data.series.first() {
                ui.label(format!("📈 Points: {}", first_series.points.len()));
            }
            ui.label(format!("📏 Range: {:.1} - {:.1}", self.data.y_range.0, self.data.y_range.1));
        });
    }
    
    /// Draw grid lines
    fn draw_grid(&self, painter: &egui::Painter, rect: Rect) {
        let grid_stroke = Stroke::new(0.5, self.config.grid_color);
        
        // Vertical grid lines
        for i in 0..=10 {
            let x = rect.min.x + (rect.width() * i as f32 / 10.0);
            painter.line_segment(
                [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                grid_stroke,
            );
        }
        
        // Horizontal grid lines
        for i in 0..=8 {
            let y = rect.min.y + (rect.height() * i as f32 / 8.0);
            painter.line_segment(
                [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
                grid_stroke,
            );
        }
    }
    
    /// Draw chart axes with labels
    fn draw_axes(&self, painter: &egui::Painter, chart_rect: Rect, plot_rect: Rect) {
        let axis_stroke = Stroke::new(2.0, Color32::BLACK);
        
        // X-axis
        painter.line_segment(
            [Pos2::new(plot_rect.min.x, plot_rect.max.y), plot_rect.max],
            axis_stroke,
        );
        
        // Y-axis
        painter.line_segment(
            [plot_rect.min, Pos2::new(plot_rect.min.x, plot_rect.max.y)],
            axis_stroke,
        );
        
        // Axis labels
        if !self.config.x_label.is_empty() {
            painter.text(
                Pos2::new(chart_rect.center().x, chart_rect.max.y - 10.0),
                egui::Align2::CENTER_BOTTOM,
                &self.config.x_label,
                egui::FontId::default(),
                Color32::BLACK,
            );
        }
        
        if !self.config.y_label.is_empty() {
            // Y-axis label (rotated text would be better but simplified here)
            painter.text(
                Pos2::new(10.0, chart_rect.center().y),
                egui::Align2::LEFT_CENTER,
                &self.config.y_label,
                egui::FontId::default(),
                Color32::BLACK,
            );
        }
    }
    
    /// Draw line chart
    fn draw_line_chart(&self, painter: &egui::Painter, rect: Rect) {
        for series in &self.data.series {
            if let SeriesStyle::Line { thickness, dashed } = &series.style {
                let stroke = if *dashed {
                    Stroke::new(*thickness, series.color) // Simplified - real dashed lines need custom implementation
                } else {
                    Stroke::new(*thickness, series.color)
                };
                
                let points: Vec<Pos2> = series.points.iter()
                    .map(|(x, y)| self.data_to_screen(*x, *y, rect))
                    .collect();
                
                // Apply animation
                let animated_points = if self.animation.enabled {
                    let count = (points.len() as f32 * self.animation.progress) as usize;
                    points.into_iter().take(count.max(1)).collect()
                } else {
                    points
                };
                
                if animated_points.len() > 1 {
                    painter.add(egui::Shape::line(animated_points, stroke));
                }
            }
        }
    }
    
    /// Draw bar chart
    fn draw_bar_chart(&self, painter: &egui::Painter, rect: Rect) {
        let bar_width = rect.width() / (self.data.series.first().map_or(1, |s| s.points.len()) as f32 * 1.2);
        
        for (series_idx, series) in self.data.series.iter().enumerate() {
            for (_point_idx, (x, y)) in series.points.iter().enumerate() {
                let screen_pos = self.data_to_screen(*x, 0.0, rect);
                let bar_height = (*y / self.data.y_range.1) * rect.height() * self.animation.progress;
                
                let bar_rect = Rect::from_min_size(
                    Pos2::new(
                        screen_pos.x + (series_idx as f32 * bar_width * 0.3) - bar_width * 0.4,
                        rect.max.y - bar_height,
                    ),
                    Vec2::new(bar_width * 0.8, bar_height),
                );
                
                painter.rect_filled(bar_rect, 2.0, series.color);
                painter.rect_stroke(bar_rect, 2.0, Stroke::new(1.0, Color32::BLACK));
            }
        }
    }
    
    /// Draw pie chart
    fn draw_pie_chart(&self, painter: &egui::Painter, rect: Rect) {
        let center = rect.center();
        let radius = rect.width().min(rect.height()) * 0.4;
        
        if let Some(first_series) = self.data.series.first() {
            let total: f32 = first_series.points.iter().map(|(_, y)| *y).sum();
            let mut current_angle = 0.0;
            
            for (i, (_, value)) in first_series.points.iter().enumerate() {
                let slice_angle = (value / total) * 2.0 * std::f32::consts::PI * self.animation.progress;
                let color = match i % 6 {
                    0 => Color32::RED,
                    1 => Color32::BLUE,
                    2 => Color32::GREEN,
                    3 => Color32::YELLOW,
                    4 => Color32::LIGHT_BLUE,
                    _ => Color32::GRAY,
                };
                
                // Draw pie slice (simplified as triangle - real implementation would use arcs)
                let end_angle = current_angle + slice_angle;
                let start_pos = center + Vec2::angled(current_angle) * radius;
                let end_pos = center + Vec2::angled(end_angle) * radius;
                
                painter.add(egui::Shape::convex_polygon(
                    vec![center, start_pos, end_pos],
                    color,
                    Stroke::new(1.0, Color32::BLACK),
                ));
                
                current_angle = end_angle;
            }
        }
    }
    
    /// Draw scatter plot
    fn draw_scatter_plot(&self, painter: &egui::Painter, rect: Rect) {
        for series in &self.data.series {
            if let SeriesStyle::Point { radius, .. } = &series.style {
                for (x, y) in &series.points {
                    let pos = self.data_to_screen(*x, *y, rect);
                    let animated_radius = radius * self.animation.progress;
                    
                    painter.circle_filled(pos, animated_radius, series.color);
                    painter.circle_stroke(pos, animated_radius, Stroke::new(1.0, Color32::BLACK));
                }
            }
        }
    }
    
    /// Draw area chart
    fn draw_area_chart(&self, painter: &egui::Painter, rect: Rect) {
        for series in &self.data.series {
            let mut points: Vec<Pos2> = series.points.iter()
                .map(|(x, y)| self.data_to_screen(*x, *y, rect))
                .collect();
            
            // Close the area by adding baseline points
            if !points.is_empty() {
                let baseline_y = rect.max.y;
                points.push(Pos2::new(points.last().unwrap().x, baseline_y));
                points.push(Pos2::new(points.first().unwrap().x, baseline_y));
            }
            
            // Apply animation
            let animated_points = if self.animation.enabled {
                let count = (points.len() as f32 * self.animation.progress) as usize;
                points.into_iter().take(count.max(3)).collect()
            } else {
                points
            };
            
            if animated_points.len() > 2 {
                let mut area_color = series.color;
                area_color[3] = 100; // Semi-transparent
                painter.add(egui::Shape::convex_polygon(
                    animated_points,
                    area_color,
                    Stroke::new(2.0, series.color),
                ));
            }
        }
    }
    
    /// Draw histogram
    fn draw_histogram(&self, painter: &egui::Painter, rect: Rect) {
        // Simplified histogram - similar to bar chart but with calculated bins
        self.draw_bar_chart(painter, rect);
    }
    
    /// Draw legend
    fn draw_legend(&self, painter: &egui::Painter, chart_rect: Rect) {
        let legend_start = Pos2::new(chart_rect.max.x - 120.0, chart_rect.min.y + 20.0);
        
        for (i, series) in self.data.series.iter().enumerate() {
            let y_offset = i as f32 * 20.0;
            let legend_pos = legend_start + Vec2::new(0.0, y_offset);
            
            // Color box
            painter.rect_filled(
                Rect::from_min_size(legend_pos, Vec2::new(12.0, 12.0)),
                2.0,
                series.color,
            );
            
            // Series name
            painter.text(
                legend_pos + Vec2::new(18.0, 6.0),
                egui::Align2::LEFT_CENTER,
                &series.name,
                egui::FontId::default(),
                Color32::BLACK,
            );
        }
    }
    
    /// Convert data coordinates to screen coordinates
    fn data_to_screen(&self, x: f32, y: f32, rect: Rect) -> Pos2 {
        let x_range = self.data.series.first()
            .map(|s| s.points.len() as f32 - 1.0)
            .unwrap_or(1.0);
        
        let screen_x = rect.min.x + (x / x_range) * rect.width();
        let screen_y = rect.max.y - ((y - self.data.y_range.0) / 
                       (self.data.y_range.1 - self.data.y_range.0)) * rect.height();
        
        Pos2::new(screen_x, screen_y)
    }
    
    /// Start chart animation
    fn start_animation(&mut self) {
        if self.animation.enabled {
            self.animation.progress = 0.0;
            self.animation.start_time = Some(std::time::Instant::now());
        }
    }
    
    /// Update animation progress
    fn update_animation(&mut self) {
        if let Some(start_time) = self.animation.start_time {
            let elapsed = start_time.elapsed().as_secs_f32();
            self.animation.progress = (elapsed / self.animation.duration).min(1.0);
            
            if self.animation.progress >= 1.0 {
                self.animation.start_time = None;
            }
        }
    }
    
    /// Generate sample data for demonstration
    fn generate_sample_data(&mut self) {
        self.data.series.clear();
        
        // Sample line data
        let points1: Vec<(f32, f32)> = (0..10)
            .map(|i| (i as f32, (i as f32 * 0.5).sin() * 20.0 + 30.0))
            .collect();
            
        let points2: Vec<(f32, f32)> = (0..10)
            .map(|i| (i as f32, (i as f32 * 0.3).cos() * 15.0 + 25.0))
            .collect();
        
        self.data.series.push(DataSeries {
            name: "Sine Wave".to_string(),
            points: points1,
            color: Color32::BLUE,
            style: SeriesStyle::Line { thickness: 2.0, dashed: false },
        });
        
        self.data.series.push(DataSeries {
            name: "Cosine Wave".to_string(),
            points: points2,
            color: Color32::RED,
            style: SeriesStyle::Line { thickness: 2.0, dashed: true },
        });
        
        self.data.y_range = (0.0, 50.0);
        self.start_animation();
    }
    
    /// Generate random data
    fn generate_random_data(&mut self) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        // Simple pseudo-random number generation
        let mut hasher = DefaultHasher::new();
        std::time::SystemTime::now().hash(&mut hasher);
        let seed = hasher.finish();
        
        let points: Vec<(f32, f32)> = (0..15)
            .map(|i| {
                let pseudo_random = ((seed.wrapping_add(i as u64)) % 100) as f32;
                (i as f32, pseudo_random * 0.5 + 10.0)
            })
            .collect();
        
        self.data.series.clear();
        self.data.series.push(DataSeries {
            name: "Random Data".to_string(),
            points,
            color: Color32::GREEN,
            style: SeriesStyle::Line { thickness: 2.0, dashed: false },
        });
        
        self.data.y_range = (0.0, 60.0);
        self.start_animation();
    }
    
    /// Add a sample data series
    fn add_sample_series(&mut self) {
        let series_count = self.data.series.len();
        let color = match series_count % 6 {
            0 => Color32::BLUE,
            1 => Color32::RED,
            2 => Color32::GREEN,
            3 => Color32::YELLOW,
            4 => Color32::LIGHT_BLUE,
            _ => Color32::GRAY,
        };
        
        let points: Vec<(f32, f32)> = (0..8)
            .map(|i| (i as f32, (i as f32 + series_count as f32) * 5.0 + 10.0))
            .collect();
        
        self.data.series.push(DataSeries {
            name: format!("Series {}", series_count + 1),
            points,
            color,
            style: SeriesStyle::Line { thickness: 2.0, dashed: false },
        });
    }
}