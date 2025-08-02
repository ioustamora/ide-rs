
#[derive(Clone, Debug, Default)]
pub struct MemoryUsage {
    pub components: usize,
    pub cache: usize,
    pub total: usize,
}

#[derive(Default)]
pub struct PerformanceMetrics {
    pub frame_times: Vec<f32>,
    pub render_calls: usize,
    pub cache_hit_ratio: f32,
    pub memory_usage: MemoryUsage,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Default)]
pub struct PerformanceReport {
    pub average_frame_time: f32,
    pub fps: f32,
    pub render_calls: usize,
    pub cache_hit_ratio: f32,
    pub memory_usage: MemoryUsage,
}
/// Performance metrics and render cache
///
/// Tracks performance and optimizes rendering.

impl PerformanceMetrics {
    pub fn add_frame_time(&mut self, time: f32) {
        self.frame_times.push(time);
        if self.frame_times.len() > 1000 {
            self.frame_times.remove(0);
        }
    }

    pub fn increment_render_calls(&mut self) {
        self.render_calls += 1;
    }

    pub fn update_cache_hit_ratio(&mut self, hits: usize, total: usize) {
        self.cache_hit_ratio = if total > 0 {
            hits as f32 / total as f32
        } else {
            0.0
        };
    }

    pub fn update_memory_usage(&mut self, components: usize, cache: usize) {
        self.memory_usage.components = components;
        self.memory_usage.cache = cache;
        self.memory_usage.total = components + cache;
    }

    pub fn report(&self) -> PerformanceReport {
        let average_frame_time = if self.frame_times.is_empty() {
            0.0
        } else {
            self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32
        };
        let fps = if average_frame_time > 0.0 {
            1000.0 / average_frame_time
        } else {
            0.0
        };
        PerformanceReport {
            average_frame_time,
            fps,
            render_calls: self.render_calls,
            cache_hit_ratio: self.cache_hit_ratio,
            memory_usage: self.memory_usage.clone(),
        }
    }
}
