//! # Performance Monitor
//!
//! Tracks and monitors performance metrics for the code editor.
//! Provides real-time monitoring of render times, memory usage, and cache performance.

use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Performance metrics collector and analyzer
#[derive(Debug, Clone)]
pub struct PerformanceMonitor {
    /// Frame timing metrics
    pub frame_metrics: FrameMetrics,
    /// Memory usage metrics
    pub memory_metrics: MemoryMetrics,
    /// Cache performance metrics
    pub cache_metrics: CacheMetrics,
    /// Render pipeline metrics
    pub render_metrics: RenderMetrics,
    /// Settings for monitoring
    pub settings: MonitorSettings,
}

/// Frame timing and rendering performance
#[derive(Debug, Clone)]
pub struct FrameMetrics {
    /// Recent frame times (rolling window)
    frame_times: VecDeque<Duration>,
    /// Current frame start time
    current_frame_start: Option<Instant>,
    /// Average frame time
    pub average_frame_time: Duration,
    /// Maximum frame time in window
    pub max_frame_time: Duration,
    /// Minimum frame time in window
    pub min_frame_time: Duration,
    /// Current FPS
    pub fps: f32,
    /// Target FPS
    pub target_fps: f32,
    /// Total frames rendered
    pub total_frames: u64,
}

/// Memory usage tracking
#[derive(Debug, Clone)]
pub struct MemoryMetrics {
    /// Current memory usage estimate (bytes)
    pub current_usage: usize,
    /// Peak memory usage
    pub peak_usage: usize,
    /// Memory allocations
    pub allocations: u64,
    /// Memory deallocations
    pub deallocations: u64,
    /// Memory usage history
    usage_history: VecDeque<usize>,
}

/// Cache performance metrics
#[derive(Debug, Clone)]
pub struct CacheMetrics {
    /// Syntax highlight cache hit rate
    pub syntax_cache_hit_rate: f32,
    /// Line cache hit rate
    pub line_cache_hit_rate: f32,
    /// Total cache requests
    pub total_cache_requests: u64,
    /// Total cache hits
    pub total_cache_hits: u64,
    /// Cache memory usage
    pub cache_memory_usage: usize,
}

/// Render pipeline performance
#[derive(Debug, Clone)]
pub struct RenderMetrics {
    /// Lines rendered per frame
    pub lines_rendered: usize,
    /// Characters rendered per frame
    pub characters_rendered: usize,
    /// Syntax highlighting time per frame
    pub syntax_highlight_time: Duration,
    /// Text layout time per frame
    pub text_layout_time: Duration,
    /// Paint time per frame
    pub paint_time: Duration,
    /// Total render time per frame
    pub total_render_time: Duration,
}

/// Performance monitoring settings
#[derive(Debug, Clone)]
pub struct MonitorSettings {
    /// Size of rolling window for metrics
    pub window_size: usize,
    /// Enable detailed profiling
    pub detailed_profiling: bool,
    /// Enable memory tracking
    pub memory_tracking: bool,
    /// Performance alert thresholds
    pub alert_thresholds: AlertThresholds,
}

/// Alert thresholds for performance issues
#[derive(Debug, Clone)]
pub struct AlertThresholds {
    /// Maximum acceptable frame time (ms)
    pub max_frame_time: Duration,
    /// Minimum acceptable FPS
    pub min_fps: f32,
    /// Maximum acceptable memory usage (MB)
    pub max_memory_mb: usize,
    /// Minimum acceptable cache hit rate
    pub min_cache_hit_rate: f32,
}

/// Performance alert types
#[derive(Debug, Clone)]
pub enum PerformanceAlert {
    /// Frame time exceeded threshold
    SlowFrame {
        frame_time: Duration,
        threshold: Duration,
    },
    /// FPS dropped below threshold
    LowFps {
        current_fps: f32,
        threshold: f32,
    },
    /// Memory usage exceeded threshold
    HighMemoryUsage {
        current_mb: usize,
        threshold_mb: usize,
    },
    /// Cache hit rate dropped below threshold
    LowCacheHitRate {
        current_rate: f32,
        threshold: f32,
    },
}

/// Comprehensive performance metrics
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    frame_metrics: FrameMetrics,
    memory_metrics: MemoryMetrics,
    cache_metrics: CacheMetrics,
    render_metrics: RenderMetrics,
    settings: MonitorSettings,
    alerts: Vec<PerformanceAlert>,
}

impl PerformanceMetrics {
    /// Create new performance metrics
    pub fn new() -> Self {
        Self {
            frame_metrics: FrameMetrics::new(),
            memory_metrics: MemoryMetrics::new(),
            cache_metrics: CacheMetrics::new(),
            render_metrics: RenderMetrics::new(),
            settings: MonitorSettings::default(),
            alerts: Vec::new(),
        }
    }

    /// Start a new frame measurement
    pub fn frame_start(&mut self) {
        self.frame_metrics.frame_start();
    }

    /// End frame measurement and record timing
    pub fn frame_end(&mut self) {
        if let Some(frame_time) = self.frame_metrics.frame_end() {
            self.check_frame_alerts(frame_time);
        }
    }

    /// Record frame time directly
    pub fn record_frame_time(&mut self, duration: Duration) {
        self.frame_metrics.record_frame_time(duration);
        self.check_frame_alerts(duration);
    }

    /// Update memory usage
    pub fn update_memory_usage(&mut self, usage_bytes: usize) {
        self.memory_metrics.update_usage(usage_bytes);
        self.check_memory_alerts();
    }

    /// Update cache metrics
    pub fn update_cache_metrics(&mut self, hit_rate: f32, total_requests: u64, total_hits: u64) {
        self.cache_metrics.syntax_cache_hit_rate = hit_rate;
        self.cache_metrics.total_cache_requests = total_requests;
        self.cache_metrics.total_cache_hits = total_hits;
        self.check_cache_alerts();
    }

    /// Update render metrics
    pub fn update_render_metrics(&mut self, metrics: RenderMetrics) {
        self.render_metrics = metrics;
    }

    /// Check for performance alerts
    fn check_frame_alerts(&mut self, frame_time: Duration) {
        if frame_time > self.settings.alert_thresholds.max_frame_time {
            self.alerts.push(PerformanceAlert::SlowFrame {
                frame_time,
                threshold: self.settings.alert_thresholds.max_frame_time,
            });
        }

        if self.frame_metrics.fps < self.settings.alert_thresholds.min_fps {
            self.alerts.push(PerformanceAlert::LowFps {
                current_fps: self.frame_metrics.fps,
                threshold: self.settings.alert_thresholds.min_fps,
            });
        }
    }

    /// Check memory alerts
    fn check_memory_alerts(&mut self) {
        let current_mb = self.memory_metrics.current_usage / (1024 * 1024);
        if current_mb > self.settings.alert_thresholds.max_memory_mb {
            self.alerts.push(PerformanceAlert::HighMemoryUsage {
                current_mb,
                threshold_mb: self.settings.alert_thresholds.max_memory_mb,
            });
        }
    }

    /// Check cache alerts
    fn check_cache_alerts(&mut self) {
        if self.cache_metrics.syntax_cache_hit_rate < self.settings.alert_thresholds.min_cache_hit_rate {
            self.alerts.push(PerformanceAlert::LowCacheHitRate {
                current_rate: self.cache_metrics.syntax_cache_hit_rate,
                threshold: self.settings.alert_thresholds.min_cache_hit_rate,
            });
        }
    }

    /// Get current alerts and clear them
    pub fn take_alerts(&mut self) -> Vec<PerformanceAlert> {
        std::mem::take(&mut self.alerts)
    }

    /// Get performance summary
    pub fn get_summary(&self) -> PerformanceSummary {
        PerformanceSummary {
            average_fps: self.frame_metrics.fps,
            average_frame_time_ms: self.frame_metrics.average_frame_time.as_millis() as f32,
            memory_usage_mb: self.memory_metrics.current_usage / (1024 * 1024),
            cache_hit_rate: self.cache_metrics.syntax_cache_hit_rate,
            total_frames: self.frame_metrics.total_frames,
            performance_score: self.calculate_performance_score(),
        }
    }

    /// Calculate overall performance score (0-100)
    fn calculate_performance_score(&self) -> f32 {
        let fps_score = (self.frame_metrics.fps / 60.0).min(1.0) * 25.0;
        let memory_score = if self.memory_metrics.current_usage < 1024 * 1024 * 1024 { 25.0 } else { 0.0 };
        let cache_score = self.cache_metrics.syntax_cache_hit_rate * 25.0;
        let render_score = if self.render_metrics.total_render_time < Duration::from_millis(16) { 25.0 } else { 0.0 };

        fps_score + memory_score + cache_score + render_score
    }
}

impl FrameMetrics {
    fn new() -> Self {
        Self {
            frame_times: VecDeque::new(),
            current_frame_start: None,
            average_frame_time: Duration::ZERO,
            max_frame_time: Duration::ZERO,
            min_frame_time: Duration::from_secs(1),
            fps: 0.0,
            target_fps: 60.0,
            total_frames: 0,
        }
    }

    fn frame_start(&mut self) {
        self.current_frame_start = Some(Instant::now());
    }

    fn frame_end(&mut self) -> Option<Duration> {
        if let Some(start) = self.current_frame_start.take() {
            let frame_time = start.elapsed();
            self.record_frame_time(frame_time);
            Some(frame_time)
        } else {
            None
        }
    }

    fn record_frame_time(&mut self, duration: Duration) {
        const MAX_SAMPLES: usize = 120; // 2 seconds at 60fps

        self.frame_times.push_back(duration);
        if self.frame_times.len() > MAX_SAMPLES {
            self.frame_times.pop_front();
        }

        self.total_frames += 1;
        self.update_stats();
    }

    fn update_stats(&mut self) {
        if self.frame_times.is_empty() {
            return;
        }

        // Calculate average
        let total: Duration = self.frame_times.iter().sum();
        self.average_frame_time = total / self.frame_times.len() as u32;

        // Calculate min/max
        self.max_frame_time = self.frame_times.iter().max().copied().unwrap_or(Duration::ZERO);
        self.min_frame_time = self.frame_times.iter().min().copied().unwrap_or(Duration::from_secs(1));

        // Calculate FPS
        if self.average_frame_time > Duration::ZERO {
            self.fps = 1.0 / self.average_frame_time.as_secs_f32();
        }
    }
}

impl MemoryMetrics {
    fn new() -> Self {
        Self {
            current_usage: 0,
            peak_usage: 0,
            allocations: 0,
            deallocations: 0,
            usage_history: VecDeque::new(),
        }
    }

    fn update_usage(&mut self, usage: usize) {
        self.current_usage = usage;
        self.peak_usage = self.peak_usage.max(usage);

        const MAX_HISTORY: usize = 60;
        self.usage_history.push_back(usage);
        if self.usage_history.len() > MAX_HISTORY {
            self.usage_history.pop_front();
        }
    }
}

impl CacheMetrics {
    fn new() -> Self {
        Self {
            syntax_cache_hit_rate: 0.0,
            line_cache_hit_rate: 0.0,
            total_cache_requests: 0,
            total_cache_hits: 0,
            cache_memory_usage: 0,
        }
    }
}

impl RenderMetrics {
    fn new() -> Self {
        Self {
            lines_rendered: 0,
            characters_rendered: 0,
            syntax_highlight_time: Duration::ZERO,
            text_layout_time: Duration::ZERO,
            paint_time: Duration::ZERO,
            total_render_time: Duration::ZERO,
        }
    }
}

impl Default for MonitorSettings {
    fn default() -> Self {
        Self {
            window_size: 120,
            detailed_profiling: false,
            memory_tracking: true,
            alert_thresholds: AlertThresholds {
                max_frame_time: Duration::from_millis(33), // ~30fps
                min_fps: 30.0,
                max_memory_mb: 2048, // 2GB
                min_cache_hit_rate: 0.7, // 70%
            },
        }
    }
}

/// Performance summary for display
#[derive(Debug, Clone)]
pub struct PerformanceSummary {
    pub average_fps: f32,
    pub average_frame_time_ms: f32,
    pub memory_usage_mb: usize,
    pub cache_hit_rate: f32,
    pub total_frames: u64,
    pub performance_score: f32,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_metrics() {
        let mut metrics = FrameMetrics::new();

        // Record some frame times
        metrics.record_frame_time(Duration::from_millis(16)); // 60fps
        metrics.record_frame_time(Duration::from_millis(20)); // 50fps
        metrics.record_frame_time(Duration::from_millis(12)); // 83fps

        assert!(metrics.fps > 0.0);
        assert!(metrics.average_frame_time > Duration::ZERO);
        assert_eq!(metrics.total_frames, 3);
    }

    #[test]
    fn test_memory_metrics() {
        let mut metrics = MemoryMetrics::new();

        metrics.update_usage(1024 * 1024); // 1MB
        assert_eq!(metrics.current_usage, 1024 * 1024);
        assert_eq!(metrics.peak_usage, 1024 * 1024);

        metrics.update_usage(2 * 1024 * 1024); // 2MB
        assert_eq!(metrics.current_usage, 2 * 1024 * 1024);
        assert_eq!(metrics.peak_usage, 2 * 1024 * 1024);

        metrics.update_usage(512 * 1024); // 512KB
        assert_eq!(metrics.current_usage, 512 * 1024);
        assert_eq!(metrics.peak_usage, 2 * 1024 * 1024); // Peak unchanged
    }

    #[test]
    fn test_performance_alerts() {
        let mut metrics = PerformanceMetrics::new();

        // Trigger slow frame alert
        metrics.record_frame_time(Duration::from_millis(100)); // Very slow frame

        let alerts = metrics.take_alerts();
        assert!(!alerts.is_empty());

        match &alerts[0] {
            PerformanceAlert::SlowFrame { frame_time, .. } => {
                assert_eq!(*frame_time, Duration::from_millis(100));
            }
            _ => panic!("Expected SlowFrame alert"),
        }
    }
}