//! # Syntax Highlight Cache
//!
//! Provides intelligent caching and background highlighting for syntax highlighting.
//! Uses LRU cache with smart invalidation and async highlighting for performance.

use egui::Color32;
use std::collections::{HashMap, BTreeSet};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::time::{Duration, Instant};

use crate::editor::syntax_highlighter::SyntaxHighlighter;

/// Cache for syntax highlighted lines with LRU eviction
pub struct SyntaxHighlightCache {
    /// Cached highlighted lines (line_number -> highlighted_line)
    cache: lru::LruCache<usize, HighlightedLine>,
    /// Cache hit rate tracking
    hit_rate: f32,
    /// Total cache requests
    total_requests: u64,
    /// Cache hits
    cache_hits: u64,
    /// Lines marked as dirty (need re-highlighting)
    dirty_lines: BTreeSet<usize>,
    /// Background highlighter
    background_highlighter: Option<BackgroundHighlighter>,
    /// Maximum cache size
    max_cache_size: usize,
}

/// A syntax highlighted line with color segments
#[derive(Debug, Clone)]
pub struct HighlightedLine {
    /// Text segments with their colors
    pub segments: Vec<(String, Color32)>,
    /// Line number
    pub line_number: usize,
    /// Timestamp when highlighted
    pub highlighted_at: Instant,
    /// Language used for highlighting
    pub language: String,
}

/// Background syntax highlighter for async processing
pub struct BackgroundHighlighter {
    /// Sender for highlight requests
    request_sender: Sender<HighlightRequest>,
    /// Receiver for highlight results
    result_receiver: Receiver<HighlightResult>,
    /// Background thread handle
    thread_handle: Option<JoinHandle<()>>,
    /// Whether the highlighter is running
    is_running: Arc<Mutex<bool>>,
}

/// Request for background highlighting
#[derive(Debug, Clone)]
pub struct HighlightRequest {
    /// Line number to highlight
    pub line_number: usize,
    /// Line content
    pub content: String,
    /// Programming language
    pub language: String,
    /// Request timestamp
    pub timestamp: Instant,
}

/// Result from background highlighting
#[derive(Debug, Clone)]
pub struct HighlightResult {
    /// Line number
    pub line_number: usize,
    /// Highlighted line
    pub highlighted_line: HighlightedLine,
    /// Processing time
    pub processing_time: Duration,
}

impl SyntaxHighlightCache {
    /// Create a new syntax highlight cache
    pub fn new() -> Self {
        Self::with_capacity(1000)
    }

    /// Create cache with specific capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            cache: lru::LruCache::new(std::num::NonZeroUsize::new(capacity).unwrap()),
            hit_rate: 0.0,
            total_requests: 0,
            cache_hits: 0,
            dirty_lines: BTreeSet::new(),
            background_highlighter: Some(BackgroundHighlighter::new()),
            max_cache_size: capacity,
        }
    }

    /// Get or highlight a line (sync)
    pub fn get_or_highlight_line(
        &mut self,
        line_number: usize,
        content: &str,
        language: &str,
    ) -> HighlightedLine {
        self.total_requests += 1;

        // Check if line is dirty
        if self.dirty_lines.contains(&line_number) {
            self.cache.pop(&line_number);
            self.dirty_lines.remove(&line_number);
        }

        // Try to get from cache first
        if let Some(cached_line) = self.cache.get(&line_number) {
            // Verify language matches
            if cached_line.language == language {
                self.cache_hits += 1;
                self.update_hit_rate();
                return cached_line.clone();
            }
        }

        // Not in cache or language mismatch, highlight now
        let highlighted_line = self.highlight_line_immediate(line_number, content, language);
        self.cache.put(line_number, highlighted_line.clone());

        self.update_hit_rate();
        highlighted_line
    }

    /// Prefetch line highlighting (async)
    pub fn prefetch_line(&mut self, line_number: usize, content: &str, language: &str) {
        // Skip if already cached and not dirty
        if !self.dirty_lines.contains(&line_number) && self.cache.contains(&line_number) {
            return;
        }

        // Send to background highlighter
        if let Some(ref mut highlighter) = self.background_highlighter {
            let request = HighlightRequest {
                line_number,
                content: content.to_string(),
                language: language.to_string(),
                timestamp: Instant::now(),
            };

            if highlighter.send_request(request).is_err() {
                // Fallback to immediate highlighting if background failed
                let highlighted = self.highlight_line_immediate(line_number, content, language);
                self.cache.put(line_number, highlighted);
            }
        }
    }

    /// Process results from background highlighter
    pub fn process_background_results(&mut self) {
        if let Some(ref mut highlighter) = self.background_highlighter {
            let results = highlighter.collect_results();

            for result in results {
                // Only cache if line isn't dirty (hasn't been modified since request)
                if !self.dirty_lines.contains(&result.line_number) {
                    self.cache.put(result.line_number, result.highlighted_line);
                }
            }
        }
    }

    /// Highlight line immediately (sync)
    fn highlight_line_immediate(
        &self,
        line_number: usize,
        content: &str,
        language: &str,
    ) -> HighlightedLine {
        let highlighter = SyntaxHighlighter::new("base16-ocean.dark");
        let segments = highlighter.highlight_line(content, language);

        HighlightedLine {
            segments,
            line_number,
            highlighted_at: Instant::now(),
            language: language.to_string(),
        }
    }

    /// Mark line range as dirty (needs re-highlighting)
    pub fn invalidate_range(&mut self, start_line: usize, end_line: usize) {
        for line in start_line..=end_line {
            self.dirty_lines.insert(line);
            self.cache.pop(&line);
        }
    }

    /// Mark single line as dirty
    pub fn invalidate_line(&mut self, line_number: usize) {
        self.dirty_lines.insert(line_number);
        self.cache.pop(&line_number);
    }

    /// Clear entire cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
        self.dirty_lines.clear();
        self.total_requests = 0;
        self.cache_hits = 0;
        self.hit_rate = 0.0;
    }

    /// Update hit rate calculation
    fn update_hit_rate(&mut self) {
        if self.total_requests > 0 {
            self.hit_rate = self.cache_hits as f32 / self.total_requests as f32;
        }
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> CacheStats {
        CacheStats {
            hit_rate: self.hit_rate,
            total_requests: self.total_requests,
            cache_hits: self.cache_hits,
            cache_size: self.cache.len(),
            max_cache_size: self.max_cache_size,
            dirty_lines_count: self.dirty_lines.len(),
        }
    }

    /// Resize cache
    pub fn resize_cache(&mut self, new_size: usize) {
        if let Ok(new_capacity) = std::num::NonZeroUsize::try_from(new_size) {
            self.cache.resize(new_capacity);
            self.max_cache_size = new_size;
        }
    }
}

impl BackgroundHighlighter {
    /// Create a new background highlighter
    pub fn new() -> Self {
        let (request_sender, request_receiver) = channel::<HighlightRequest>();
        let (result_sender, result_receiver) = channel::<HighlightResult>();
        let is_running = Arc::new(Mutex::new(true));
        let is_running_clone = is_running.clone();

        // Spawn background highlighting thread
        let thread_handle = thread::spawn(move || {
            let mut highlighter = SyntaxHighlighter::new("base16-ocean.dark");

            while *is_running_clone.lock().unwrap() {
                // Process requests with timeout to allow checking shutdown
                match request_receiver.recv_timeout(Duration::from_millis(100)) {
                    Ok(request) => {
                        let start_time = Instant::now();

                        // Highlight the line
                        let segments = highlighter.highlight_line(&request.content, &request.language);

                        let highlighted_line = HighlightedLine {
                            segments,
                            line_number: request.line_number,
                            highlighted_at: Instant::now(),
                            language: request.language,
                        };

                        let result = HighlightResult {
                            line_number: request.line_number,
                            highlighted_line,
                            processing_time: start_time.elapsed(),
                        };

                        // Send result back (ignore if receiver is dropped)
                        let _ = result_sender.send(result);
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                        // Continue loop to check shutdown condition
                        continue;
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                        // Channel disconnected, exit thread
                        break;
                    }
                }
            }
        });

        Self {
            request_sender,
            result_receiver,
            thread_handle: Some(thread_handle),
            is_running,
        }
    }

    /// Send a highlight request to background thread
    pub fn send_request(&self, request: HighlightRequest) -> Result<(), String> {
        self.request_sender
            .send(request)
            .map_err(|e| format!("Failed to send highlight request: {}", e))
    }

    /// Collect all available results
    pub fn collect_results(&self) -> Vec<HighlightResult> {
        let mut results = Vec::new();

        // Collect all available results without blocking
        while let Ok(result) = self.result_receiver.try_recv() {
            results.push(result);
        }

        results
    }

    /// Shutdown background highlighter
    pub fn shutdown(&mut self) {
        // Signal shutdown
        if let Ok(mut running) = self.is_running.lock() {
            *running = false;
        }

        // Wait for thread to finish
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for BackgroundHighlighter {
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Cache hit rate (0.0 to 1.0)
    pub hit_rate: f32,
    /// Total number of requests
    pub total_requests: u64,
    /// Number of cache hits
    pub cache_hits: u64,
    /// Current cache size
    pub cache_size: usize,
    /// Maximum cache size
    pub max_cache_size: usize,
    /// Number of dirty lines
    pub dirty_lines_count: usize,
}

impl Default for SyntaxHighlightCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_hit_miss() {
        let mut cache = SyntaxHighlightCache::new();

        // First request should be a miss
        let line1 = cache.get_or_highlight_line(0, "fn main() {", "rust");
        assert!(!line1.segments.is_empty());

        // Second request for same line should be a hit
        let line2 = cache.get_or_highlight_line(0, "fn main() {", "rust");
        assert_eq!(line1.segments.len(), line2.segments.len());

        let stats = cache.get_stats();
        assert_eq!(stats.total_requests, 2);
        assert_eq!(stats.cache_hits, 1);
        assert_eq!(stats.hit_rate, 0.5);
    }

    #[test]
    fn test_cache_invalidation() {
        let mut cache = SyntaxHighlightCache::new();

        // Cache a line
        let _line1 = cache.get_or_highlight_line(0, "fn main() {", "rust");

        // Invalidate it
        cache.invalidate_line(0);

        // Next request should be a miss
        let _line2 = cache.get_or_highlight_line(0, "fn main() {", "rust");

        let stats = cache.get_stats();
        assert_eq!(stats.total_requests, 2);
        assert_eq!(stats.cache_hits, 0);
    }

    #[test]
    fn test_language_mismatch() {
        let mut cache = SyntaxHighlightCache::new();

        // Cache for Rust
        let _line1 = cache.get_or_highlight_line(0, "fn main() {", "rust");

        // Request same line for different language should miss
        let _line2 = cache.get_or_highlight_line(0, "fn main() {", "javascript");

        let stats = cache.get_stats();
        assert_eq!(stats.cache_hits, 0);
    }
}