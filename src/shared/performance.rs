//! Performance Utilities
//!
//! Performance monitoring, profiling, and optimization utilities
//! for tracking and improving IDE performance.

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

/// Performance profiler for monitoring system performance
pub struct PerformanceProfiler {
    /// Active profiling sessions
    pub sessions: HashMap<String, ProfilingSession>,
    /// Performance metrics history
    pub metrics_history: VecDeque<RenderMetrics>,
    /// Maximum history size
    pub max_history_size: usize,
    /// Performance thresholds
    pub thresholds: PerformanceThresholds,
    /// Profiling enabled
    pub enabled: bool,
}

/// Individual profiling session
#[derive(Clone, Debug)]
pub struct ProfilingSession {
    /// Session name
    pub name: String,
    /// Session start time
    pub start_time: Instant,
    /// Session measurements
    pub measurements: Vec<Measurement>,
    /// Session metadata
    pub metadata: HashMap<String, String>,
}

/// Performance measurement
#[derive(Clone, Debug)]
pub struct Measurement {
    /// Measurement name
    pub name: String,
    /// Measurement duration
    pub duration: Duration,
    /// Measurement timestamp
    pub timestamp: Instant,
    /// Additional measurement data
    pub data: HashMap<String, f64>,
}

/// Rendering performance metrics
#[derive(Clone, Debug)]
pub struct RenderMetrics {
    /// Frame render time
    pub frame_time: Duration,
    /// UI update time
    pub ui_update_time: Duration,
    /// Layout calculation time
    pub layout_time: Duration,
    /// Paint time
    pub paint_time: Duration,
    /// Component count
    pub component_count: usize,
    /// Memory usage (bytes)
    pub memory_usage: usize,
    /// CPU usage percentage
    pub cpu_usage: f32,
    /// FPS (frames per second)
    pub fps: f32,
    /// Timestamp
    pub timestamp: Instant,
}

/// Performance thresholds for warnings
#[derive(Clone, Debug)]
pub struct PerformanceThresholds {
    /// Maximum acceptable frame time (ms)
    pub max_frame_time: Duration,
    /// Maximum UI update time (ms)
    pub max_ui_update_time: Duration,
    /// Maximum layout time (ms)
    pub max_layout_time: Duration,
    /// Maximum component count
    pub max_component_count: usize,
    /// Maximum memory usage (MB)
    pub max_memory_usage: usize,
    /// Minimum acceptable FPS
    pub min_fps: f32,
}

/// Memory usage tracker
pub struct MemoryTracker {
    /// Current memory usage by category
    pub usage_by_category: HashMap<String, usize>,
    /// Memory usage history
    pub usage_history: VecDeque<MemorySnapshot>,
    /// Maximum history size
    pub max_history_size: usize,
}

/// Memory usage snapshot
#[derive(Clone, Debug)]
pub struct MemorySnapshot {
    /// Total memory usage (bytes)
    pub total_usage: usize,
    /// Usage by category
    pub category_usage: HashMap<String, usize>,
    /// Timestamp
    pub timestamp: Instant,
}

/// Performance analyzer for identifying bottlenecks
pub struct PerformanceAnalyzer {
    /// Collected metrics
    pub metrics: Vec<RenderMetrics>,
    /// Analysis results
    pub analysis_results: Vec<AnalysisResult>,
    /// Optimization suggestions
    pub suggestions: Vec<OptimizationSuggestion>,
}

/// Performance analysis result
#[derive(Clone, Debug)]
pub struct AnalysisResult {
    /// Analysis type
    pub analysis_type: AnalysisType,
    /// Severity level
    pub severity: PerformanceSeverity,
    /// Description
    pub description: String,
    /// Affected metrics
    pub affected_metrics: Vec<String>,
    /// Potential impact
    pub impact: f32,
}

/// Type of performance analysis
#[derive(Clone, Debug, PartialEq)]
pub enum AnalysisType {
    FrameTimeSpike,
    MemoryLeak,
    ExcessiveComponents,
    SlowLayout,
    SlowPaint,
    LowFps,
    HighCpuUsage,
}

/// Performance issue severity
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum PerformanceSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// Optimization suggestion
#[derive(Clone, Debug)]
pub struct OptimizationSuggestion {
    /// Suggestion title
    pub title: String,
    /// Detailed description
    pub description: String,
    /// Expected performance improvement
    pub expected_improvement: f32,
    /// Implementation difficulty
    pub difficulty: OptimizationDifficulty,
    /// Suggestion category
    pub category: OptimizationCategory,
    /// Implementation steps
    pub steps: Vec<String>,
}

/// Optimization implementation difficulty
#[derive(Clone, Debug, PartialEq)]
pub enum OptimizationDifficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

/// Optimization category
#[derive(Clone, Debug, PartialEq)]
pub enum OptimizationCategory {
    Rendering,
    Memory,
    Layout,
    ComponentCount,
    DataStructures,
    Algorithms,
    Caching,
}

/// Render timing tracker
pub struct RenderTimer {
    /// Timer start time
    start_time: Option<Instant>,
    /// Current phase being timed
    current_phase: Option<String>,
    /// Phase timings
    phase_timings: HashMap<String, Duration>,
}

impl Default for PerformanceProfiler {
    fn default() -> Self {
        Self {
            sessions: HashMap::new(),
            metrics_history: VecDeque::new(),
            max_history_size: 1000,
            thresholds: PerformanceThresholds::default(),
            enabled: true,
        }
    }
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            max_frame_time: Duration::from_millis(16), // 60 FPS
            max_ui_update_time: Duration::from_millis(5),
            max_layout_time: Duration::from_millis(3),
            max_component_count: 1000,
            max_memory_usage: 100 * 1024 * 1024, // 100MB
            min_fps: 30.0,
        }
    }
}

impl Default for MemoryTracker {
    fn default() -> Self {
        Self {
            usage_by_category: HashMap::new(),
            usage_history: VecDeque::new(),
            max_history_size: 500,
        }
    }
}

impl PerformanceProfiler {
    /// Create a new performance profiler
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Start a new profiling session
    pub fn start_session(&mut self, name: String) {
        let session = ProfilingSession {
            name: name.clone(),
            start_time: Instant::now(),
            measurements: Vec::new(),
            metadata: HashMap::new(),
        };
        self.sessions.insert(name, session);
    }
    
    /// End a profiling session
    pub fn end_session(&mut self, name: &str) -> Option<ProfilingSession> {
        self.sessions.remove(name)
    }
    
    /// Record a measurement in an active session
    pub fn record_measurement(&mut self, session_name: &str, measurement_name: String, duration: Duration) {
        if let Some(session) = self.sessions.get_mut(session_name) {
            session.measurements.push(Measurement {
                name: measurement_name,
                duration,
                timestamp: Instant::now(),
                data: HashMap::new(),
            });
        }
    }
    
    /// Record render metrics
    pub fn record_render_metrics(&mut self, metrics: RenderMetrics) {
        if !self.enabled {
            return;
        }
        
        self.metrics_history.push_back(metrics);
        
        // Maintain history size limit
        if self.metrics_history.len() > self.max_history_size {
            self.metrics_history.pop_front();
        }
    }
    
    /// Get average metrics over recent frames
    pub fn get_average_metrics(&self, frame_count: usize) -> Option<RenderMetrics> {
        if self.metrics_history.is_empty() {
            return None;
        }
        
        let count = frame_count.min(self.metrics_history.len());
        let recent_metrics: Vec<&RenderMetrics> = self.metrics_history
            .iter()
            .rev()
            .take(count)
            .collect();
        
        if recent_metrics.is_empty() {
            return None;
        }
        
        let total_frame_time: Duration = recent_metrics.iter()
            .map(|m| m.frame_time)
            .sum();
        
        let total_ui_update_time: Duration = recent_metrics.iter()
            .map(|m| m.ui_update_time)
            .sum();
        
        let total_layout_time: Duration = recent_metrics.iter()
            .map(|m| m.layout_time)
            .sum();
        
        let total_paint_time: Duration = recent_metrics.iter()
            .map(|m| m.paint_time)
            .sum();
        
        let avg_component_count = recent_metrics.iter()
            .map(|m| m.component_count)
            .sum::<usize>() / count;
        
        let avg_memory_usage = recent_metrics.iter()
            .map(|m| m.memory_usage)
            .sum::<usize>() / count;
        
        let avg_cpu_usage = recent_metrics.iter()
            .map(|m| m.cpu_usage)
            .sum::<f32>() / count as f32;
        
        let avg_fps = recent_metrics.iter()
            .map(|m| m.fps)
            .sum::<f32>() / count as f32;
        
        Some(RenderMetrics {
            frame_time: total_frame_time / count as u32,
            ui_update_time: total_ui_update_time / count as u32,
            layout_time: total_layout_time / count as u32,
            paint_time: total_paint_time / count as u32,
            component_count: avg_component_count,
            memory_usage: avg_memory_usage,
            cpu_usage: avg_cpu_usage,
            fps: avg_fps,
            timestamp: Instant::now(),
        })
    }
    
    /// Check for performance threshold violations
    pub fn check_thresholds(&self, metrics: &RenderMetrics) -> Vec<AnalysisResult> {
        let mut results = Vec::new();
        
        if metrics.frame_time > self.thresholds.max_frame_time {
            results.push(AnalysisResult {
                analysis_type: AnalysisType::FrameTimeSpike,
                severity: PerformanceSeverity::High,
                description: format!(
                    "Frame time {}ms exceeds threshold {}ms",
                    metrics.frame_time.as_millis(),
                    self.thresholds.max_frame_time.as_millis()
                ),
                affected_metrics: vec!["frame_time".to_string()],
                impact: (metrics.frame_time.as_millis() as f32 / self.thresholds.max_frame_time.as_millis() as f32) - 1.0,
            });
        }
        
        if metrics.component_count > self.thresholds.max_component_count {
            results.push(AnalysisResult {
                analysis_type: AnalysisType::ExcessiveComponents,
                severity: PerformanceSeverity::Medium,
                description: format!(
                    "Component count {} exceeds threshold {}",
                    metrics.component_count,
                    self.thresholds.max_component_count
                ),
                affected_metrics: vec!["component_count".to_string()],
                impact: (metrics.component_count as f32 / self.thresholds.max_component_count as f32) - 1.0,
            });
        }
        
        if metrics.memory_usage > self.thresholds.max_memory_usage {
            results.push(AnalysisResult {
                analysis_type: AnalysisType::MemoryLeak,
                severity: PerformanceSeverity::High,
                description: format!(
                    "Memory usage {}MB exceeds threshold {}MB",
                    metrics.memory_usage / (1024 * 1024),
                    self.thresholds.max_memory_usage / (1024 * 1024)
                ),
                affected_metrics: vec!["memory_usage".to_string()],
                impact: (metrics.memory_usage as f32 / self.thresholds.max_memory_usage as f32) - 1.0,
            });
        }
        
        if metrics.fps < self.thresholds.min_fps {
            results.push(AnalysisResult {
                analysis_type: AnalysisType::LowFps,
                severity: PerformanceSeverity::Critical,
                description: format!(
                    "FPS {:.1} is below minimum threshold {:.1}",
                    metrics.fps,
                    self.thresholds.min_fps
                ),
                affected_metrics: vec!["fps".to_string()],
                impact: (self.thresholds.min_fps - metrics.fps) / self.thresholds.min_fps,
            });
        }
        
        results
    }
}

impl MemoryTracker {
    /// Create a new memory tracker
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Update memory usage for a category
    pub fn update_category_usage(&mut self, category: String, usage: usize) {
        self.usage_by_category.insert(category, usage);
    }
    
    /// Take a memory snapshot
    pub fn take_snapshot(&mut self) {
        let total_usage = self.usage_by_category.values().sum();
        
        let snapshot = MemorySnapshot {
            total_usage,
            category_usage: self.usage_by_category.clone(),
            timestamp: Instant::now(),
        };
        
        self.usage_history.push_back(snapshot);
        
        // Maintain history size limit
        if self.usage_history.len() > self.max_history_size {
            self.usage_history.pop_front();
        }
    }
    
    /// Get memory usage trend
    pub fn get_usage_trend(&self, duration: Duration) -> Option<f32> {
        if self.usage_history.len() < 2 {
            return None;
        }
        
        let now = Instant::now();
        let cutoff_time = now - duration;
        
        let recent_snapshots: Vec<&MemorySnapshot> = self.usage_history
            .iter()
            .filter(|snapshot| snapshot.timestamp >= cutoff_time)
            .collect();
        
        if recent_snapshots.len() < 2 {
            return None;
        }
        
        let first = recent_snapshots.first()?.total_usage as f32;
        let last = recent_snapshots.last()?.total_usage as f32;
        
        Some((last - first) / first)
    }
}

impl PerformanceAnalyzer {
    /// Create a new performance analyzer
    pub fn new() -> Self {
        Self {
            metrics: Vec::new(),
            analysis_results: Vec::new(),
            suggestions: Vec::new(),
        }
    }
    
    /// Analyze collected metrics
    pub fn analyze(&mut self, metrics: Vec<RenderMetrics>) {
        self.metrics = metrics;
        self.analysis_results.clear();
        self.suggestions.clear();
        
        self.analyze_frame_times();
        self.analyze_memory_usage();
        self.analyze_component_count();
        self.generate_suggestions();
    }
    
    /// Analyze frame time patterns
    fn analyze_frame_times(&mut self) {
        if self.metrics.is_empty() {
            return;
        }
        
        let avg_frame_time = self.metrics.iter()
            .map(|m| m.frame_time.as_millis() as f32)
            .sum::<f32>() / self.metrics.len() as f32;
        
        let spikes = self.metrics.iter()
            .filter(|m| m.frame_time.as_millis() as f32 > avg_frame_time * 2.0)
            .count();
        
        if spikes > self.metrics.len() / 10 {
            self.analysis_results.push(AnalysisResult {
                analysis_type: AnalysisType::FrameTimeSpike,
                severity: PerformanceSeverity::High,
                description: format!("Frequent frame time spikes detected ({} spikes)", spikes),
                affected_metrics: vec!["frame_time".to_string()],
                impact: spikes as f32 / self.metrics.len() as f32,
            });
        }
    }
    
    /// Analyze memory usage patterns
    fn analyze_memory_usage(&mut self) {
        if self.metrics.len() < 10 {
            return;
        }
        
        let memory_values: Vec<f32> = self.metrics.iter()
            .map(|m| m.memory_usage as f32)
            .collect();
        
        // Simple trend analysis
        let first_half_avg = memory_values[..memory_values.len()/2].iter().sum::<f32>() / (memory_values.len()/2) as f32;
        let second_half_avg = memory_values[memory_values.len()/2..].iter().sum::<f32>() / (memory_values.len()/2) as f32;
        
        let growth_rate = (second_half_avg - first_half_avg) / first_half_avg;
        
        if growth_rate > 0.1 {
            self.analysis_results.push(AnalysisResult {
                analysis_type: AnalysisType::MemoryLeak,
                severity: PerformanceSeverity::Critical,
                description: format!("Potential memory leak detected ({}% growth)", growth_rate * 100.0),
                affected_metrics: vec!["memory_usage".to_string()],
                impact: growth_rate,
            });
        }
    }
    
    /// Analyze component count patterns
    fn analyze_component_count(&mut self) {
        if self.metrics.is_empty() {
            return;
        }
        
        let max_components = self.metrics.iter()
            .map(|m| m.component_count)
            .max()
            .unwrap_or(0);
        
        if max_components > 500 {
            self.analysis_results.push(AnalysisResult {
                analysis_type: AnalysisType::ExcessiveComponents,
                severity: PerformanceSeverity::Medium,
                description: format!("High component count detected ({})", max_components),
                affected_metrics: vec!["component_count".to_string()],
                impact: (max_components as f32 - 500.0) / 500.0,
            });
        }
    }
    
    /// Generate optimization suggestions based on analysis
    fn generate_suggestions(&mut self) {
        for result in &self.analysis_results {
            match result.analysis_type {
                AnalysisType::FrameTimeSpike => {
                    self.suggestions.push(OptimizationSuggestion {
                        title: "Optimize Rendering Pipeline".to_string(),
                        description: "Consider batching draw calls and reducing state changes".to_string(),
                        expected_improvement: 0.3,
                        difficulty: OptimizationDifficulty::Medium,
                        category: OptimizationCategory::Rendering,
                        steps: vec![
                            "Profile individual render operations".to_string(),
                            "Batch similar draw operations".to_string(),
                            "Minimize state changes".to_string(),
                        ],
                    });
                }
                AnalysisType::MemoryLeak => {
                    self.suggestions.push(OptimizationSuggestion {
                        title: "Fix Memory Leaks".to_string(),
                        description: "Investigate and fix memory leaks in component lifecycle".to_string(),
                        expected_improvement: 0.5,
                        difficulty: OptimizationDifficulty::Hard,
                        category: OptimizationCategory::Memory,
                        steps: vec![
                            "Use memory profiler to identify leaks".to_string(),
                            "Ensure proper cleanup in component destructors".to_string(),
                            "Check for circular references".to_string(),
                        ],
                    });
                }
                AnalysisType::ExcessiveComponents => {
                    self.suggestions.push(OptimizationSuggestion {
                        title: "Reduce Component Count".to_string(),
                        description: "Implement component pooling or virtualization".to_string(),
                        expected_improvement: 0.4,
                        difficulty: OptimizationDifficulty::Medium,
                        category: OptimizationCategory::ComponentCount,
                        steps: vec![
                            "Implement virtual scrolling".to_string(),
                            "Use component pooling".to_string(),
                            "Lazy load off-screen components".to_string(),
                        ],
                    });
                }
                _ => {}
            }
        }
    }
}

impl RenderTimer {
    /// Create a new render timer
    pub fn new() -> Self {
        Self {
            start_time: None,
            current_phase: None,
            phase_timings: HashMap::new(),
        }
    }
    
    /// Start timing a render phase
    pub fn start_phase(&mut self, phase: String) {
        if let Some(_current) = &self.current_phase {
            self.end_current_phase();
        }
        
        self.current_phase = Some(phase);
        self.start_time = Some(Instant::now());
    }
    
    /// End the current phase
    pub fn end_current_phase(&mut self) {
        if let (Some(phase), Some(start_time)) = (&self.current_phase, self.start_time) {
            let duration = start_time.elapsed();
            self.phase_timings.insert(phase.clone(), duration);
        }
        
        self.current_phase = None;
        self.start_time = None;
    }
    
    /// Get phase timing
    pub fn get_phase_timing(&self, phase: &str) -> Option<Duration> {
        self.phase_timings.get(phase).copied()
    }
    
    /// Get all phase timings
    pub fn get_all_timings(&self) -> &HashMap<String, Duration> {
        &self.phase_timings
    }
    
    /// Reset all timings
    pub fn reset(&mut self) {
        self.start_time = None;
        self.current_phase = None;
        self.phase_timings.clear();
    }
}

/// Create a scoped performance measurement
pub struct ScopedMeasurement {
    profiler: *mut PerformanceProfiler,
    session_name: String,
    measurement_name: String,
    start_time: Instant,
}

impl ScopedMeasurement {
    /// Create a new scoped measurement
    pub fn new(
        profiler: &mut PerformanceProfiler,
        session_name: String,
        measurement_name: String,
    ) -> Self {
        Self {
            profiler,
            session_name,
            measurement_name,
            start_time: Instant::now(),
        }
    }
}

impl Drop for ScopedMeasurement {
    fn drop(&mut self) {
        let duration = self.start_time.elapsed();
        unsafe {
            if let Some(profiler) = self.profiler.as_mut() {
                profiler.record_measurement(&self.session_name, self.measurement_name.clone(), duration);
            }
        }
    }
}

/// Macro for easy performance measurement
#[macro_export]
macro_rules! measure_performance {
    ($profiler:expr, $session:expr, $name:expr, $code:block) => {
        {
            let _measurement = ScopedMeasurement::new($profiler, $session.to_string(), $name.to_string());
            $code
        }
    };
}

/// Performance cache for expensive operations
pub struct PerformanceCache<K, V> {
    /// Cached values
    cache: HashMap<K, CachedValue<V>>,
    /// Maximum cache size
    max_size: usize,
    /// Cache hit counter
    hit_count: usize,
    /// Cache miss counter
    miss_count: usize,
    /// Cache expiry duration
    expiry_duration: Duration,
}

/// Cached value with timestamp
#[derive(Clone, Debug)]
struct CachedValue<V> {
    value: V,
    timestamp: Instant,
    access_count: usize,
}

impl<K: std::hash::Hash + Eq + Clone, V: Clone> PerformanceCache<K, V> {
    /// Create a new performance cache
    pub fn new(max_size: usize, expiry_duration: Duration) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
            hit_count: 0,
            miss_count: 0,
            expiry_duration,
        }
    }

    /// Get a value from cache or compute it
    pub fn get_or_compute<F>(&mut self, key: K, compute_fn: F) -> V 
    where 
        F: FnOnce() -> V,
    {
        // Check if value exists and is not expired
        if let Some(cached) = self.cache.get_mut(&key) {
            if cached.timestamp.elapsed() < self.expiry_duration {
                cached.access_count += 1;
                self.hit_count += 1;
                return cached.value.clone();
            }
        }

        // Compute new value
        self.miss_count += 1;
        let value = compute_fn();
        
        // Store in cache
        self.insert(key, value.clone());
        value
    }

    /// Insert a value into cache
    pub fn insert(&mut self, key: K, value: V) {
        // Remove oldest entries if cache is full
        if self.cache.len() >= self.max_size {
            self.evict_oldest();
        }

        self.cache.insert(key, CachedValue {
            value,
            timestamp: Instant::now(),
            access_count: 0,
        });
    }

    /// Evict the oldest cache entry
    fn evict_oldest(&mut self) {
        if let Some(oldest_key) = self.cache.iter()
            .min_by_key(|(_, v)| v.timestamp)
            .map(|(k, _)| k.clone()) {
            self.cache.remove(&oldest_key);
        }
    }

    /// Get cache hit rate
    pub fn hit_rate(&self) -> f32 {
        let total = self.hit_count + self.miss_count;
        if total == 0 {
            0.0
        } else {
            self.hit_count as f32 / total as f32
        }
    }

    /// Clear expired entries
    pub fn clear_expired(&mut self) {
        let now = Instant::now();
        self.cache.retain(|_, v| now.duration_since(v.timestamp) < self.expiry_duration);
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> CacheStats {
        CacheStats {
            size: self.cache.len(),
            max_size: self.max_size,
            hit_count: self.hit_count,
            miss_count: self.miss_count,
            hit_rate: self.hit_rate(),
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub size: usize,
    pub max_size: usize,
    pub hit_count: usize,
    pub miss_count: usize,
    pub hit_rate: f32,
}

/// Resource pool for expensive objects
pub struct ResourcePool<T> {
    /// Available resources
    available: Vec<T>,
    /// Resource factory function
    factory: Box<dyn Fn() -> T>,
    /// Maximum pool size
    max_size: usize,
    /// Pool statistics
    stats: PoolStats,
}

/// Pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub created_count: usize,
    pub borrowed_count: usize,
    pub returned_count: usize,
    pub current_available: usize,
    pub peak_usage: usize,
}

impl<T> ResourcePool<T> {
    /// Create a new resource pool
    pub fn new<F>(factory: F, max_size: usize) -> Self 
    where 
        F: Fn() -> T + 'static,
    {
        Self {
            available: Vec::new(),
            factory: Box::new(factory),
            max_size,
            stats: PoolStats {
                created_count: 0,
                borrowed_count: 0,
                returned_count: 0,
                current_available: 0,
                peak_usage: 0,
            },
        }
    }

    /// Borrow a resource from the pool
    pub fn borrow(&mut self) -> T {
        if let Some(resource) = self.available.pop() {
            self.stats.borrowed_count += 1;
            self.stats.current_available = self.available.len();
            resource
        } else {
            self.stats.created_count += 1;
            self.stats.borrowed_count += 1;
            (self.factory)()
        }
    }

    /// Return a resource to the pool
    pub fn return_resource(&mut self, resource: T) {
        if self.available.len() < self.max_size {
            self.available.push(resource);
            self.stats.returned_count += 1;
            self.stats.current_available = self.available.len();
            self.stats.peak_usage = self.stats.peak_usage.max(self.available.len());
        }
        // If pool is full, resource is dropped
    }

    /// Get pool statistics
    pub fn get_stats(&self) -> &PoolStats {
        &self.stats
    }
}

/// Lazy evaluation wrapper for expensive computations
pub struct LazyValue<T> {
    value: Option<T>,
    compute_fn: Option<Box<dyn FnOnce() -> T>>,
}

impl<T> LazyValue<T> {
    /// Create a new lazy value
    pub fn new<F>(compute_fn: F) -> Self 
    where 
        F: FnOnce() -> T + 'static,
    {
        Self {
            value: None,
            compute_fn: Some(Box::new(compute_fn)),
        }
    }

    /// Get the value, computing it if necessary
    pub fn get(&mut self) -> &T {
        if self.value.is_none() {
            if let Some(compute_fn) = self.compute_fn.take() {
                self.value = Some(compute_fn());
            }
        }
        self.value.as_ref().expect("Value should be computed")
    }

    /// Check if value has been computed
    pub fn is_computed(&self) -> bool {
        self.value.is_some()
    }
}

/// Batch processor for grouping similar operations
pub struct BatchProcessor<T> {
    /// Pending items to process
    pending: Vec<T>,
    /// Batch size threshold
    batch_size: usize,
    /// Time threshold for batch processing
    time_threshold: Duration,
    /// Last batch process time
    last_process_time: Instant,
}

impl<T> BatchProcessor<T> {
    /// Create a new batch processor
    pub fn new(batch_size: usize, time_threshold: Duration) -> Self {
        Self {
            pending: Vec::new(),
            batch_size,
            time_threshold,
            last_process_time: Instant::now(),
        }
    }

    /// Add an item to the batch
    pub fn add(&mut self, item: T) {
        self.pending.push(item);
    }

    /// Process batch if conditions are met
    pub fn process_if_ready<F>(&mut self, processor: F) -> bool 
    where 
        F: FnOnce(Vec<T>),
    {
        let should_process = self.pending.len() >= self.batch_size || 
                           self.last_process_time.elapsed() >= self.time_threshold;

        if should_process && !self.pending.is_empty() {
            let items = std::mem::take(&mut self.pending);
            processor(items);
            self.last_process_time = Instant::now();
            true
        } else {
            false
        }
    }

    /// Force process all pending items
    pub fn flush<F>(&mut self, processor: F) 
    where 
        F: FnOnce(Vec<T>),
    {
        if !self.pending.is_empty() {
            let items = std::mem::take(&mut self.pending);
            processor(items);
            self.last_process_time = Instant::now();
        }
    }

    /// Get pending item count
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
}