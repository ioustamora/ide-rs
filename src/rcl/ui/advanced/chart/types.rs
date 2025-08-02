//! Chart Data Types and Structures
//!
//! This module contains all the data structures used by the chart component.

use egui::{Vec2, Color32};

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