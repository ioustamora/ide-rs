//! # Memory Optimizer
//!
//! Provides memory optimization and monitoring for the code editor.
//! Helps manage memory usage for large files and reduces allocation pressure.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Memory optimizer for reducing allocation pressure and optimizing memory usage
pub struct MemoryOptimizer {
    /// String interning pool for commonly used strings
    string_pool: StringPool,
    /// Buffer pool for reusing allocations
    buffer_pool: BufferPool,
    /// Memory statistics
    stats: MemoryStats,
    /// Optimization settings
    settings: OptimizerSettings,
}

/// String interning pool to reduce duplicate string allocations
pub struct StringPool {
    /// Interned strings
    pool: HashMap<String, usize>,
    /// Reference counts
    ref_counts: HashMap<String, usize>,
    /// Maximum pool size
    max_size: usize,
}

/// Buffer pool for reusing byte buffers and reducing allocations
pub struct BufferPool {
    /// Available buffers by size
    buffers: HashMap<usize, Vec<Vec<u8>>>,
    /// Buffer usage statistics
    allocations: u64,
    /// Buffer reuses
    reuses: u64,
    /// Maximum buffers per size
    max_buffers_per_size: usize,
}

/// Memory usage statistics and tracking
#[derive(Debug, Clone)]
pub struct MemoryStats {
    /// Current estimated heap usage
    pub heap_usage: usize,
    /// Peak heap usage
    pub peak_heap_usage: usize,
    /// Total allocations
    pub total_allocations: u64,
    /// Total deallocations
    pub total_deallocations: u64,
    /// String pool statistics
    pub string_pool_stats: StringPoolStats,
    /// Buffer pool statistics
    pub buffer_pool_stats: BufferPoolStats,
    /// Memory usage by category
    pub usage_by_category: HashMap<String, usize>,
    /// Last updated timestamp
    pub last_updated: Instant,
}

/// String pool statistics
#[derive(Debug, Clone)]
pub struct StringPoolStats {
    /// Number of unique strings
    pub unique_strings: usize,
    /// Total references
    pub total_references: usize,
    /// Memory saved by interning
    pub memory_saved: usize,
    /// Pool hit rate
    pub hit_rate: f32,
}

/// Buffer pool statistics
#[derive(Debug, Clone)]
pub struct BufferPoolStats {
    /// Total buffers in pool
    pub total_buffers: usize,
    /// Reuse rate
    pub reuse_rate: f32,
    /// Memory in pool
    pub memory_in_pool: usize,
    /// Allocations avoided
    pub allocations_avoided: u64,
}

/// Memory optimization settings
#[derive(Debug, Clone)]
pub struct OptimizerSettings {
    /// Enable string interning
    pub enable_string_interning: bool,
    /// Enable buffer pooling
    pub enable_buffer_pooling: bool,
    /// Maximum string pool size
    pub max_string_pool_size: usize,
    /// Maximum buffer pool memory
    pub max_buffer_pool_memory: usize,
    /// Cleanup interval
    pub cleanup_interval: Duration,
    /// Memory tracking categories
    pub track_categories: Vec<String>,
}

impl MemoryOptimizer {
    /// Create a new memory optimizer
    pub fn new() -> Self {
        Self {
            string_pool: StringPool::new(),
            buffer_pool: BufferPool::new(),
            stats: MemoryStats::new(),
            settings: OptimizerSettings::default(),
        }
    }

    /// Create with custom settings
    pub fn with_settings(settings: OptimizerSettings) -> Self {
        Self {
            string_pool: StringPool::with_capacity(settings.max_string_pool_size),
            buffer_pool: BufferPool::new(),
            stats: MemoryStats::new(),
            settings,
        }
    }

    /// Intern a string to reduce memory usage
    pub fn intern_string(&mut self, s: &str) -> String {
        if !self.settings.enable_string_interning {
            return s.to_string();
        }

        self.string_pool.intern(s)
    }

    /// Get a buffer from the pool or allocate a new one
    pub fn get_buffer(&mut self, size: usize) -> Vec<u8> {
        if !self.settings.enable_buffer_pooling {
            return vec![0; size];
        }

        self.buffer_pool.get_buffer(size)
    }

    /// Return a buffer to the pool for reuse
    pub fn return_buffer(&mut self, mut buffer: Vec<u8>) {
        if !self.settings.enable_buffer_pooling {
            return;
        }

        // Clear the buffer before returning to pool
        buffer.clear();
        self.buffer_pool.return_buffer(buffer);
    }

    /// Track memory allocation for a category
    pub fn track_allocation(&mut self, category: &str, size: usize) {
        self.stats.total_allocations += 1;
        self.stats.heap_usage += size;
        self.stats.peak_heap_usage = self.stats.peak_heap_usage.max(self.stats.heap_usage);

        *self.stats.usage_by_category.entry(category.to_string()).or_insert(0) += size;
        self.stats.last_updated = Instant::now();
    }

    /// Track memory deallocation for a category
    pub fn track_deallocation(&mut self, category: &str, size: usize) {
        self.stats.total_deallocations += 1;
        self.stats.heap_usage = self.stats.heap_usage.saturating_sub(size);

        if let Some(category_usage) = self.stats.usage_by_category.get_mut(category) {
            *category_usage = category_usage.saturating_sub(size);
        }
        self.stats.last_updated = Instant::now();
    }

    /// Perform cleanup and optimization
    pub fn cleanup(&mut self) {
        self.string_pool.cleanup();
        self.buffer_pool.cleanup();
        self.update_stats();
    }

    /// Update internal statistics
    fn update_stats(&mut self) {
        self.stats.string_pool_stats = self.string_pool.get_stats();
        self.stats.buffer_pool_stats = self.buffer_pool.get_stats();
    }

    /// Get current memory statistics
    pub fn get_stats(&self) -> &MemoryStats {
        &self.stats
    }

    /// Get memory usage report
    pub fn get_memory_report(&self) -> MemoryReport {
        let total_tracked = self.stats.usage_by_category.values().sum::<usize>();

        MemoryReport {
            total_heap_usage: self.stats.heap_usage,
            peak_heap_usage: self.stats.peak_heap_usage,
            tracked_usage: total_tracked,
            untracked_usage: self.stats.heap_usage.saturating_sub(total_tracked),
            string_pool_savings: self.stats.string_pool_stats.memory_saved,
            buffer_pool_reuse_rate: self.stats.buffer_pool_stats.reuse_rate,
            categories: self.stats.usage_by_category.clone(),
            optimization_score: self.calculate_optimization_score(),
        }
    }

    /// Calculate optimization effectiveness score (0-100)
    fn calculate_optimization_score(&self) -> f32 {
        let string_score = if self.stats.string_pool_stats.hit_rate > 0.8 { 25.0 } else { 0.0 };
        let buffer_score = if self.stats.buffer_pool_stats.reuse_rate > 0.7 { 25.0 } else { 0.0 };
        let memory_score = if self.stats.heap_usage < 1024 * 1024 * 1024 { 25.0 } else { 0.0 }; // <1GB
        let allocation_score = if self.stats.total_allocations > 0 &&
            self.stats.total_deallocations as f32 / self.stats.total_allocations as f32 > 0.9 { 25.0 } else { 0.0 };

        string_score + buffer_score + memory_score + allocation_score
    }
}

impl StringPool {
    fn new() -> Self {
        Self::with_capacity(10000)
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            pool: HashMap::with_capacity(capacity),
            ref_counts: HashMap::with_capacity(capacity),
            max_size: capacity,
        }
    }

    fn intern(&mut self, s: &str) -> String {
        if let Some(count) = self.ref_counts.get_mut(s) {
            *count += 1;
            s.to_string()
        } else if self.pool.len() < self.max_size {
            let string = s.to_string();
            self.pool.insert(string.clone(), 1);
            self.ref_counts.insert(string.clone(), 1);
            string
        } else {
            // Pool is full, return new string without interning
            s.to_string()
        }
    }

    fn cleanup(&mut self) {
        // Remove strings with no references
        let to_remove: Vec<String> = self.ref_counts
            .iter()
            .filter(|(_, &count)| count == 0)
            .map(|(s, _)| s.clone())
            .collect();

        for string in to_remove {
            self.pool.remove(&string);
            self.ref_counts.remove(&string);
        }
    }

    fn get_stats(&self) -> StringPoolStats {
        let total_refs: usize = self.ref_counts.values().sum();
        let memory_saved = self.pool.iter()
            .map(|(s, _)| s.len() * (self.ref_counts.get(s).unwrap_or(&1) - 1))
            .sum();

        StringPoolStats {
            unique_strings: self.pool.len(),
            total_references: total_refs,
            memory_saved,
            hit_rate: if total_refs > 0 {
                (total_refs - self.pool.len()) as f32 / total_refs as f32
            } else {
                0.0
            },
        }
    }
}

impl BufferPool {
    fn new() -> Self {
        Self {
            buffers: HashMap::new(),
            allocations: 0,
            reuses: 0,
            max_buffers_per_size: 10,
        }
    }

    fn get_buffer(&mut self, size: usize) -> Vec<u8> {
        self.allocations += 1;

        // Round size to nearest power of 2 for better pooling
        let rounded_size = size.next_power_of_two();

        if let Some(buffers) = self.buffers.get_mut(&rounded_size) {
            if let Some(mut buffer) = buffers.pop() {
                buffer.resize(size, 0);
                self.reuses += 1;
                return buffer;
            }
        }

        // No buffer available, allocate new one
        vec![0; size]
    }

    fn return_buffer(&mut self, buffer: Vec<u8>) {
        let capacity = buffer.capacity();
        let rounded_capacity = capacity.next_power_of_two();

        let buffers = self.buffers.entry(rounded_capacity).or_insert_with(Vec::new);

        if buffers.len() < self.max_buffers_per_size {
            buffers.push(buffer);
        }
        // If pool is full, buffer will be dropped
    }

    fn cleanup(&mut self) {
        // Remove empty buffer vectors
        self.buffers.retain(|_, buffers| !buffers.is_empty());
    }

    fn get_stats(&self) -> BufferPoolStats {
        let total_buffers: usize = self.buffers.values().map(|v| v.len()).sum();
        let memory_in_pool: usize = self.buffers
            .iter()
            .map(|(size, buffers)| size * buffers.len())
            .sum();

        BufferPoolStats {
            total_buffers,
            reuse_rate: if self.allocations > 0 {
                self.reuses as f32 / self.allocations as f32
            } else {
                0.0
            },
            memory_in_pool,
            allocations_avoided: self.reuses,
        }
    }
}

impl MemoryStats {
    fn new() -> Self {
        Self {
            heap_usage: 0,
            peak_heap_usage: 0,
            total_allocations: 0,
            total_deallocations: 0,
            string_pool_stats: StringPoolStats {
                unique_strings: 0,
                total_references: 0,
                memory_saved: 0,
                hit_rate: 0.0,
            },
            buffer_pool_stats: BufferPoolStats {
                total_buffers: 0,
                reuse_rate: 0.0,
                memory_in_pool: 0,
                allocations_avoided: 0,
            },
            usage_by_category: HashMap::new(),
            last_updated: Instant::now(),
        }
    }
}

impl Default for OptimizerSettings {
    fn default() -> Self {
        Self {
            enable_string_interning: true,
            enable_buffer_pooling: true,
            max_string_pool_size: 10000,
            max_buffer_pool_memory: 64 * 1024 * 1024, // 64MB
            cleanup_interval: Duration::from_secs(30),
            track_categories: vec![
                "syntax_highlighting".to_string(),
                "text_content".to_string(),
                "ui_rendering".to_string(),
                "cache".to_string(),
            ],
        }
    }
}

/// Memory usage report
#[derive(Debug, Clone)]
pub struct MemoryReport {
    pub total_heap_usage: usize,
    pub peak_heap_usage: usize,
    pub tracked_usage: usize,
    pub untracked_usage: usize,
    pub string_pool_savings: usize,
    pub buffer_pool_reuse_rate: f32,
    pub categories: HashMap<String, usize>,
    pub optimization_score: f32,
}

impl Default for MemoryOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_interning() {
        let mut pool = StringPool::new();

        let s1 = pool.intern("hello");
        let s2 = pool.intern("hello");
        let s3 = pool.intern("world");

        assert_eq!(s1, s2);
        assert_ne!(s1, s3);

        let stats = pool.get_stats();
        assert_eq!(stats.unique_strings, 2);
        assert_eq!(stats.total_references, 3);
        assert!(stats.memory_saved > 0);
    }

    #[test]
    fn test_buffer_pooling() {
        let mut pool = BufferPool::new();

        // Get a buffer
        let buffer1 = pool.get_buffer(1024);
        assert_eq!(buffer1.len(), 1024);

        // Return it to pool
        pool.return_buffer(buffer1);

        // Get another buffer of same size - should reuse
        let buffer2 = pool.get_buffer(1024);
        assert_eq!(buffer2.len(), 1024);

        let stats = pool.get_stats();
        assert!(stats.reuse_rate > 0.0);
        assert_eq!(stats.allocations_avoided, 1);
    }

    #[test]
    fn test_memory_tracking() {
        let mut optimizer = MemoryOptimizer::new();

        optimizer.track_allocation("test", 1024);
        optimizer.track_allocation("test", 512);

        let stats = optimizer.get_stats();
        assert_eq!(stats.heap_usage, 1536);
        assert_eq!(stats.total_allocations, 2);
        assert_eq!(stats.usage_by_category.get("test"), Some(&1536));

        optimizer.track_deallocation("test", 512);
        let stats = optimizer.get_stats();
        assert_eq!(stats.heap_usage, 1024);
        assert_eq!(stats.total_deallocations, 1);
    }
}