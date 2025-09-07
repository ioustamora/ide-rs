//! Code Folding System
//!
//! Provides hierarchical code folding capabilities for better code navigation
//! and organization, supporting multiple languages and custom folding regions.

use egui::*;
use std::collections::{HashMap, BTreeSet};

/// Code folding manager
pub struct CodeFoldingManager {
    /// Foldable regions indexed by line number
    pub foldable_regions: HashMap<usize, FoldableRegion>,
    /// Currently folded regions
    pub folded_regions: BTreeSet<usize>,
    /// Fold level cache for performance
    pub fold_level_cache: HashMap<usize, usize>,
    /// Language-specific folding rules
    pub folding_rules: HashMap<String, Box<dyn FoldingProvider>>,
    /// Manual fold markers
    pub manual_folds: Vec<ManualFoldRegion>,
}

/// Represents a foldable code region
#[derive(Debug, Clone)]
pub struct FoldableRegion {
    /// Starting line (0-based)
    pub start_line: usize,
    /// Ending line (0-based)
    pub end_line: usize,
    /// Nesting level
    pub level: usize,
    /// Region type (function, class, block, etc.)
    pub region_type: FoldRegionType,
    /// Display text when folded
    pub fold_text: String,
    /// Whether this region can be folded
    pub foldable: bool,
}

/// Types of foldable regions
#[derive(Debug, Clone, PartialEq)]
pub enum FoldRegionType {
    /// Function definitions
    Function,
    /// Class/struct definitions
    Class,
    /// Code blocks (if, for, while, etc.)
    Block,
    /// Comments (single-line and multi-line)
    Comment,
    /// Import/use statements
    Imports,
    /// Documentation
    Documentation,
    /// Custom regions marked by user
    Custom,
    /// Braced regions
    Braces,
}

/// Manual fold region created by user
#[derive(Debug, Clone)]
pub struct ManualFoldRegion {
    /// Start line
    pub start_line: usize,
    /// End line  
    pub end_line: usize,
    /// Custom display text
    pub display_text: String,
}

/// Language-specific folding provider
pub trait FoldingProvider: Send + Sync {
    /// Analyze code and return foldable regions
    fn find_foldable_regions(&self, content: &str) -> Vec<FoldableRegion>;
    
    /// Check if line can start a foldable region
    fn can_start_fold(&self, line: &str, line_number: usize) -> Option<FoldRegionType>;
    
    /// Check if line can end a foldable region
    fn can_end_fold(&self, line: &str, line_number: usize, region_type: &FoldRegionType) -> bool;
    
    /// Get default fold text for region type
    fn get_fold_text(&self, region_type: &FoldRegionType, content: &str) -> String;
}

/// Generic folding provider for brace-based languages
#[derive(Debug, Clone)]
pub struct BraceFoldingProvider {
    /// Language name
    pub language: String,
    /// Comment patterns
    pub comment_patterns: Vec<CommentPattern>,
}

/// Comment pattern for folding
#[derive(Debug, Clone)]
pub struct CommentPattern {
    /// Single-line comment prefix
    pub single_line: Option<String>,
    /// Multi-line comment start
    pub multi_start: Option<String>,
    /// Multi-line comment end
    pub multi_end: Option<String>,
}

impl CodeFoldingManager {
    /// Create a new code folding manager
    pub fn new() -> Self {
        let mut manager = Self {
            foldable_regions: HashMap::new(),
            folded_regions: BTreeSet::new(),
            fold_level_cache: HashMap::new(),
            folding_rules: HashMap::new(),
            manual_folds: Vec::new(),
        };

        // Register default folding providers
        manager.register_default_providers();
        manager
    }

    /// Register default folding providers for common languages
    fn register_default_providers(&mut self) {
        // Rust folding provider
        let rust_provider = BraceFoldingProvider {
            language: "rust".to_string(),
            comment_patterns: vec![
                CommentPattern {
                    single_line: Some("//".to_string()),
                    multi_start: Some("/*".to_string()),
                    multi_end: Some("*/".to_string()),
                }
            ],
        };
        self.folding_rules.insert("rust".to_string(), Box::new(rust_provider));

        // JavaScript/TypeScript folding provider
        let js_provider = BraceFoldingProvider {
            language: "javascript".to_string(),
            comment_patterns: vec![
                CommentPattern {
                    single_line: Some("//".to_string()),
                    multi_start: Some("/*".to_string()),
                    multi_end: Some("*/".to_string()),
                }
            ],
        };
        self.folding_rules.insert("javascript".to_string(), Box::new(js_provider.clone()));
        self.folding_rules.insert("typescript".to_string(), Box::new(js_provider));
    }

    /// Analyze content and update foldable regions
    pub fn analyze_content(&mut self, content: &str, language: &str) {
        self.foldable_regions.clear();
        self.fold_level_cache.clear();

        if let Some(provider) = self.folding_rules.get(language) {
            let regions = provider.find_foldable_regions(content);
            for region in regions {
                self.foldable_regions.insert(region.start_line, region);
            }
        }

        // Add manual fold regions
        for manual_fold in &self.manual_folds {
            let region = FoldableRegion {
                start_line: manual_fold.start_line,
                end_line: manual_fold.end_line,
                level: 0,
                region_type: FoldRegionType::Custom,
                fold_text: manual_fold.display_text.clone(),
                foldable: true,
            };
            self.foldable_regions.insert(manual_fold.start_line, region);
        }

        self.calculate_fold_levels();
    }

    /// Calculate nesting levels for all foldable regions
    fn calculate_fold_levels(&mut self) {
        let mut level_stack = Vec::new();
        let mut current_level = 0;

        for line_number in 0..self.get_max_line() {
            // Check if a new region starts here
            if let Some(region) = self.foldable_regions.get_mut(&line_number) {
                region.level = current_level;
                level_stack.push((line_number, region.end_line));
                current_level += 1;
            }

            // Check if any regions end here
            level_stack.retain(|(_, end_line)| {
                if *end_line == line_number && current_level > 0 {
                    current_level -= 1;
                    false
                } else {
                    true
                }
            });

            self.fold_level_cache.insert(line_number, current_level);
        }
    }

    /// Toggle fold state for a line
    pub fn toggle_fold(&mut self, line_number: usize) {
        if self.foldable_regions.contains_key(&line_number) {
            if self.folded_regions.contains(&line_number) {
                self.unfold_region(line_number);
            } else {
                self.fold_region(line_number);
            }
        }
    }

    /// Fold a region
    pub fn fold_region(&mut self, line_number: usize) {
        self.folded_regions.insert(line_number);
    }

    /// Unfold a region
    pub fn unfold_region(&mut self, line_number: usize) {
        self.folded_regions.remove(&line_number);
    }

    /// Check if a line is folded
    pub fn is_line_folded(&self, line_number: usize) -> bool {
        // Check if this line is inside any folded region
        for &fold_start in &self.folded_regions {
            if let Some(region) = self.foldable_regions.get(&fold_start) {
                if line_number > fold_start && line_number <= region.end_line {
                    return true;
                }
            }
        }
        false
    }

    /// Check if a line starts a foldable region
    pub fn is_foldable_line(&self, line_number: usize) -> bool {
        self.foldable_regions.contains_key(&line_number)
    }

    /// Check if a line starts a folded region
    pub fn is_folded_line(&self, line_number: usize) -> bool {
        self.folded_regions.contains(&line_number)
    }

    /// Get visible lines (excluding folded content)
    pub fn get_visible_lines(&self, total_lines: usize) -> Vec<usize> {
        let mut visible_lines = Vec::new();
        let mut skip_until = None;

        for line_number in 0..total_lines {
            // Check if we're skipping folded content
            if let Some(skip_end) = skip_until {
                if line_number <= skip_end {
                    continue;
                } else {
                    skip_until = None;
                }
            }

            // Check if this line starts a folded region
            if let Some(region) = self.foldable_regions.get(&line_number) {
                if self.folded_regions.contains(&line_number) {
                    visible_lines.push(line_number); // Include the fold line itself
                    skip_until = Some(region.end_line);
                    continue;
                }
            }

            visible_lines.push(line_number);
        }

        visible_lines
    }

    /// Get fold text for a folded region
    pub fn get_fold_text(&self, line_number: usize) -> Option<String> {
        if let Some(region) = self.foldable_regions.get(&line_number) {
            if self.folded_regions.contains(&line_number) {
                return Some(region.fold_text.clone());
            }
        }
        None
    }

    /// Add manual fold region
    pub fn add_manual_fold(&mut self, start_line: usize, end_line: usize, display_text: String) {
        let manual_fold = ManualFoldRegion {
            start_line,
            end_line,
            display_text,
        };
        self.manual_folds.push(manual_fold);
    }

    /// Remove manual fold region
    pub fn remove_manual_fold(&mut self, start_line: usize) {
        self.manual_folds.retain(|fold| fold.start_line != start_line);
        self.foldable_regions.remove(&start_line);
    }

    /// Get fold level for a line
    pub fn get_fold_level(&self, line_number: usize) -> usize {
        self.fold_level_cache.get(&line_number).copied().unwrap_or(0)
    }

    /// Fold all regions of a specific type
    pub fn fold_all_of_type(&mut self, region_type: FoldRegionType) {
        let lines_to_fold: Vec<usize> = self.foldable_regions
            .iter()
            .filter(|(_, region)| region.region_type == region_type)
            .map(|(&line, _)| line)
            .collect();

        for line in lines_to_fold {
            self.fold_region(line);
        }
    }

    /// Unfold all regions
    pub fn unfold_all(&mut self) {
        self.folded_regions.clear();
    }

    /// Render fold markers in the editor gutter
    pub fn render_fold_markers(&self, ui: &mut Ui, painter: &Painter, line_ranges: &[(usize, Rect)]) {
        for &(line_number, line_rect) in line_ranges {
            if let Some(region) = self.foldable_regions.get(&line_number) {
                let is_folded = self.folded_regions.contains(&line_number);
                let marker_rect = Rect::from_center_size(
                    Pos2::new(line_rect.min.x - 8.0, line_rect.center().y),
                    Vec2::splat(12.0)
                );

                // Draw fold marker background
                painter.rect_filled(
                    marker_rect,
                    2.0,
                    if is_folded { Color32::from_gray(100) } else { Color32::from_gray(80) }
                );

                // Draw fold marker icon
                let icon = if is_folded { "+" } else { "-" };
                painter.text(
                    marker_rect.center(),
                    Align2::CENTER_CENTER,
                    icon,
                    FontId::monospace(10.0),
                    Color32::WHITE,
                );

                // Handle clicks on fold markers
                let response = ui.allocate_rect(marker_rect, Sense::click());
                if response.clicked() {
                    // Note: This would need to be handled by the editor
                    // since we can't mutate self here
                }

                // Draw fold level indicator
                if region.level > 0 {
                    let level_color = Color32::from_rgb(
                        (100 + (region.level * 30) % 155) as u8,
                        (50 + (region.level * 50) % 155) as u8,
                        (200 - (region.level * 20) % 100) as u8,
                    );
                    
                    painter.line_segment(
                        [
                            Pos2::new(line_rect.min.x - 16.0, line_rect.min.y),
                            Pos2::new(line_rect.min.x - 16.0, line_rect.max.y),
                        ],
                        Stroke::new(2.0, level_color),
                    );
                }
            }
        }
    }

    /// Get maximum line number (helper function)
    fn get_max_line(&self) -> usize {
        self.foldable_regions
            .values()
            .map(|region| region.end_line)
            .max()
            .unwrap_or(0) + 1
    }
}

impl FoldingProvider for BraceFoldingProvider {
    fn find_foldable_regions(&self, content: &str) -> Vec<FoldableRegion> {
        let mut regions = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut brace_stack = Vec::new();
        let mut in_multiline_comment = false;
        let mut comment_start = None;

        for (line_number, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Handle multi-line comments
            for pattern in &self.comment_patterns {
                if let (Some(start), Some(end)) = (&pattern.multi_start, &pattern.multi_end) {
                    if !in_multiline_comment && trimmed.contains(start) {
                        in_multiline_comment = true;
                        comment_start = Some(line_number);
                    }
                    
                    if in_multiline_comment && trimmed.contains(end) {
                        in_multiline_comment = false;
                        if let Some(start_line) = comment_start {
                            if line_number > start_line {
                                regions.push(FoldableRegion {
                                    start_line,
                                    end_line: line_number,
                                    level: 0,
                                    region_type: FoldRegionType::Comment,
                                    fold_text: "/* ... */".to_string(),
                                    foldable: true,
                                });
                            }
                        }
                        comment_start = None;
                    }
                }
            }

            if in_multiline_comment {
                continue;
            }

            // Find function definitions (simple heuristic)
            if (trimmed.starts_with("fn ") || trimmed.starts_with("function ") || 
                trimmed.contains(" fn ") || trimmed.contains(" function ")) && trimmed.contains("{") {
                brace_stack.push((line_number, FoldRegionType::Function));
            }
            // Find class/struct definitions
            else if (trimmed.starts_with("struct ") || trimmed.starts_with("class ") || 
                     trimmed.starts_with("impl ")) && trimmed.contains("{") {
                brace_stack.push((line_number, FoldRegionType::Class));
            }
            // Find generic braced blocks
            else if trimmed.contains("{") && !trimmed.starts_with("//") {
                brace_stack.push((line_number, FoldRegionType::Block));
            }

            // Handle closing braces
            if trimmed.contains("}") && !trimmed.starts_with("//") {
                if let Some((start_line, region_type)) = brace_stack.pop() {
                    if line_number > start_line {
                        let fold_text = match region_type {
                            FoldRegionType::Function => "{ ... }".to_string(),
                            FoldRegionType::Class => "{ ... }".to_string(),
                            _ => "{ ... }".to_string(),
                        };

                        regions.push(FoldableRegion {
                            start_line,
                            end_line: line_number,
                            level: 0,
                            region_type,
                            fold_text,
                            foldable: true,
                        });
                    }
                }
            }
        }

        regions
    }

    fn can_start_fold(&self, line: &str, _line_number: usize) -> Option<FoldRegionType> {
        let trimmed = line.trim();
        
        if trimmed.starts_with("fn ") || trimmed.contains(" fn ") {
            Some(FoldRegionType::Function)
        } else if trimmed.starts_with("struct ") || trimmed.starts_with("class ") {
            Some(FoldRegionType::Class)
        } else if trimmed.contains("{") {
            Some(FoldRegionType::Block)
        } else {
            None
        }
    }

    fn can_end_fold(&self, line: &str, _line_number: usize, _region_type: &FoldRegionType) -> bool {
        line.trim().contains("}")
    }

    fn get_fold_text(&self, region_type: &FoldRegionType, _content: &str) -> String {
        match region_type {
            FoldRegionType::Function => "{ ... }".to_string(),
            FoldRegionType::Class => "{ ... }".to_string(),
            FoldRegionType::Block => "{ ... }".to_string(),
            FoldRegionType::Comment => "/* ... */".to_string(),
            FoldRegionType::Documentation => "/// ...".to_string(),
            FoldRegionType::Imports => "use ...".to_string(),
            FoldRegionType::Custom => "...".to_string(),
            FoldRegionType::Braces => "{ ... }".to_string(),
        }
    }
}

impl Default for CodeFoldingManager {
    fn default() -> Self {
        Self::new()
    }
}