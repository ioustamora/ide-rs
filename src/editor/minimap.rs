//! Code Minimap
//!
//! Provides a bird's-eye view of the entire code file with navigation
//! capabilities, syntax highlighting, and visual indicators for errors,
//! breakpoints, and other annotations.

use egui::*;
use std::collections::HashMap;

/// Minimap component for code editor
pub struct Minimap {
    /// Whether minimap is enabled
    pub enabled: bool,
    /// Minimap width in pixels
    pub width: f32,
    /// Scale factor for rendering (pixels per line)
    pub scale_factor: f32,
    /// Content cache for performance
    pub content_cache: Vec<String>,
    /// Syntax highlighting cache
    pub highlight_cache: HashMap<usize, Vec<(TextRange, Color32)>>,
    /// Visual annotations (errors, warnings, etc.)
    pub annotations: HashMap<usize, Vec<MinimapAnnotation>>,
    /// Current viewport (visible area)
    pub viewport: MinimapViewport,
    /// Minimap settings
    pub settings: MinimapSettings,
}

/// Minimap annotation (error, warning, breakpoint, etc.)
#[derive(Debug, Clone)]
pub struct MinimapAnnotation {
    /// Annotation type
    pub annotation_type: AnnotationType,
    /// Color for this annotation
    pub color: Color32,
    /// Priority (higher priority annotations are drawn on top)
    pub priority: u8,
    /// Optional tooltip text
    pub tooltip: Option<String>,
}

/// Types of minimap annotations
#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationType {
    /// Compilation error
    Error,
    /// Warning
    Warning,
    /// Information/hint
    Info,
    /// Breakpoint
    Breakpoint,
    /// Search result
    SearchMatch,
    /// TODO comment
    Todo,
    /// FIXME comment
    Fixme,
    /// Bookmark
    Bookmark,
    /// Git change (added/modified/deleted)
    GitChange(GitChangeType),
}

/// Git change types
#[derive(Debug, Clone, PartialEq)]
pub enum GitChangeType {
    Added,
    Modified,
    Deleted,
}

/// Text range for highlighting
#[derive(Debug, Clone)]
pub struct TextRange {
    /// Start column
    pub start: usize,
    /// End column
    pub end: usize,
}

/// Minimap viewport information
#[derive(Debug, Clone)]
pub struct MinimapViewport {
    /// First visible line
    pub first_visible_line: usize,
    /// Last visible line
    pub last_visible_line: usize,
    /// Total number of lines
    pub total_lines: usize,
    /// Viewport rect in minimap coordinates
    pub rect: Rect,
}

/// Minimap configuration settings
#[derive(Debug, Clone)]
pub struct MinimapSettings {
    /// Show syntax highlighting in minimap
    pub show_syntax_highlighting: bool,
    /// Show annotations (errors, warnings, etc.)
    pub show_annotations: bool,
    /// Show scroll indicators
    pub show_scroll_indicators: bool,
    /// Maximum width of minimap
    pub max_width: f32,
    /// Minimum width of minimap
    pub min_width: f32,
    /// Font size for minimap text
    pub font_size: f32,
    /// Background color
    pub background_color: Color32,
    /// Viewport indicator color
    pub viewport_color: Color32,
    /// Whether to show line numbers
    pub show_line_numbers: bool,
}

impl Default for MinimapSettings {
    fn default() -> Self {
        Self {
            show_syntax_highlighting: true,
            show_annotations: true,
            show_scroll_indicators: true,
            max_width: 150.0,
            min_width: 80.0,
            font_size: 6.0,
            background_color: Color32::from_gray(25),
            viewport_color: Color32::from_rgba_unmultiplied(100, 150, 255, 50),
            show_line_numbers: false,
        }
    }
}

impl Default for MinimapViewport {
    fn default() -> Self {
        Self {
            first_visible_line: 0,
            last_visible_line: 0,
            total_lines: 0,
            rect: Rect::NOTHING,
        }
    }
}

impl Minimap {
    /// Create a new minimap
    pub fn new() -> Self {
        Self {
            enabled: true,
            width: 120.0,
            scale_factor: 2.0,
            content_cache: Vec::new(),
            highlight_cache: HashMap::new(),
            annotations: HashMap::new(),
            viewport: MinimapViewport::default(),
            settings: MinimapSettings::default(),
        }
    }

    /// Update minimap content
    pub fn update_content(&mut self, content: &str) {
        self.content_cache = content.lines().map(|line| line.to_string()).collect();
        self.highlight_cache.clear(); // Clear cache when content changes
    }

    /// Update viewport information
    pub fn update_viewport(&mut self, first_visible: usize, last_visible: usize, total_lines: usize) {
        self.viewport.first_visible_line = first_visible;
        self.viewport.last_visible_line = last_visible;
        self.viewport.total_lines = total_lines;
    }

    /// Add annotation to a line
    pub fn add_annotation(&mut self, line: usize, annotation: MinimapAnnotation) {
        self.annotations.entry(line).or_default().push(annotation);
    }

    /// Remove annotations of a specific type from a line
    pub fn remove_annotations(&mut self, line: usize, annotation_type: AnnotationType) {
        if let Some(annotations) = self.annotations.get_mut(&line) {
            annotations.retain(|ann| ann.annotation_type != annotation_type);
            if annotations.is_empty() {
                self.annotations.remove(&line);
            }
        }
    }

    /// Clear all annotations
    pub fn clear_annotations(&mut self) {
        self.annotations.clear();
    }

    /// Clear annotations of a specific type
    pub fn clear_annotations_of_type(&mut self, annotation_type: AnnotationType) {
        for annotations in self.annotations.values_mut() {
            annotations.retain(|ann| ann.annotation_type != annotation_type);
        }
        self.annotations.retain(|_, annotations| !annotations.is_empty());
    }

    /// Render the minimap
    pub fn render(&mut self, ui: &mut Ui, available_rect: Rect) -> MinimapResponse {
        if !self.enabled || self.content_cache.is_empty() {
            return MinimapResponse::default();
        }

        // Calculate minimap dimensions
        let minimap_rect = Rect::from_min_size(
            Pos2::new(available_rect.max.x - self.width, available_rect.min.y),
            Vec2::new(self.width, available_rect.height())
        );

        let mut response = MinimapResponse::default();

        ui.allocate_ui_at_rect(minimap_rect, |ui| {
            // Draw minimap background
            let painter = ui.painter();
            painter.rect_filled(minimap_rect, 0.0, self.settings.background_color);

            // Calculate metrics
            let line_height = self.scale_factor;
            let total_content_height = self.content_cache.len() as f32 * line_height;
            let visible_height = minimap_rect.height();
            
            // Calculate scroll ratio if content is larger than minimap
            let scroll_ratio = if total_content_height > visible_height {
                visible_height / total_content_height
            } else {
                1.0
            };

            // Render visible content
            let start_y = minimap_rect.min.y;
            for (line_index, line_content) in self.content_cache.iter().enumerate() {
                let y_pos = start_y + (line_index as f32 * line_height * scroll_ratio);
                
                if y_pos > minimap_rect.max.y {
                    break; // Outside visible area
                }

                let line_rect = Rect::from_min_size(
                    Pos2::new(minimap_rect.min.x, y_pos),
                    Vec2::new(self.width, line_height * scroll_ratio)
                );

                // Render line content (simplified)
                if self.settings.show_syntax_highlighting {
                    self.render_line_with_highlighting(painter, line_rect, line_index, line_content);
                } else {
                    self.render_plain_line(painter, line_rect, line_content);
                }

                // Render annotations for this line
                if self.settings.show_annotations {
                    self.render_line_annotations(painter, line_rect, line_index);
                }
            }

            // Render viewport indicator
            self.render_viewport_indicator(painter, minimap_rect, scroll_ratio);

            // Handle mouse interactions
            let minimap_response = ui.allocate_rect(minimap_rect, Sense::click_and_drag());
            
            if minimap_response.clicked() || minimap_response.dragged() {
                // Calculate which line was clicked
                let click_y = minimap_response.interact_pointer_pos()
                    .unwrap_or_default().y;
                let relative_y = (click_y - minimap_rect.min.y) / minimap_rect.height();
                let target_line = (relative_y * self.content_cache.len() as f32) as usize;
                
                response.clicked_line = Some(target_line.min(self.content_cache.len().saturating_sub(1)));
            }

            if minimap_response.hovered() {
                // Show hover tooltip with line information
                let hover_y = ui.input(|i| i.pointer.hover_pos().unwrap_or_default().y);
                let relative_y = (hover_y - minimap_rect.min.y) / minimap_rect.height();
                let hover_line = (relative_y * self.content_cache.len() as f32) as usize;
                
                if hover_line < self.content_cache.len() {
                    response.hovered_line = Some(hover_line);
                }
            }
        });

        response
    }

    /// Render a line with syntax highlighting
    fn render_line_with_highlighting(&self, painter: &Painter, rect: Rect, line_index: usize, content: &str) {
        // Simple highlighting based on content patterns
        let mut x_offset = 0.0;
        let char_width = 1.0; // Very small character width for minimap
        
        for (i, ch) in content.chars().enumerate() {
            if x_offset >= rect.width() {
                break; // Line too long for minimap
            }

            let color = self.get_char_color(ch, content, i);
            let char_rect = Rect::from_min_size(
                Pos2::new(rect.min.x + x_offset, rect.min.y),
                Vec2::new(char_width, rect.height())
            );
            
            painter.rect_filled(char_rect, 0.0, color);
            x_offset += char_width;
        }
    }

    /// Render a plain line without highlighting
    fn render_plain_line(&self, painter: &Painter, rect: Rect, content: &str) {
        if !content.trim().is_empty() {
            let intensity = self.calculate_line_intensity(content);
            let color = Color32::from_gray((intensity * 255.0) as u8);
            painter.rect_filled(rect, 0.0, color);
        }
    }

    /// Calculate line intensity based on character density
    fn calculate_line_intensity(&self, content: &str) -> f32 {
        let non_space_chars = content.chars().filter(|c| !c.is_whitespace()).count();
        let total_chars = content.len().max(1);
        (non_space_chars as f32 / total_chars as f32 * 0.7).min(1.0)
    }

    /// Get color for a character based on simple syntax patterns
    fn get_char_color(&self, ch: char, line: &str, pos: usize) -> Color32 {
        // Very simple syntax highlighting for minimap
        if line.trim_start().starts_with("//") || line.trim_start().starts_with("#") {
            Color32::from_rgb(100, 150, 100) // Comments - green
        } else if ch == '"' || ch == '\'' {
            Color32::from_rgb(150, 100, 100) // Strings - red
        } else if ch.is_numeric() {
            Color32::from_rgb(100, 100, 150) // Numbers - blue
        } else if "{}[]()".contains(ch) {
            Color32::from_rgb(150, 150, 100) // Brackets - yellow
        } else if ch.is_alphabetic() {
            // Check if it's a keyword (very basic)
            if self.is_likely_keyword(line, pos) {
                Color32::from_rgb(150, 100, 150) // Keywords - purple
            } else {
                Color32::from_gray(180) // Regular text
            }
        } else {
            Color32::from_gray(120) // Other characters
        }
    }

    /// Simple keyword detection for minimap highlighting
    fn is_likely_keyword(&self, line: &str, pos: usize) -> bool {
        let keywords = ["fn", "let", "mut", "if", "else", "for", "while", "match", 
                       "struct", "impl", "trait", "enum", "use", "mod", "pub"];
        
        // Extract word around position
        let start = line[..pos].rfind(|c: char| !c.is_alphanumeric() && c != '_')
            .map(|i| i + 1)
            .unwrap_or(0);
        let end = line[pos..].find(|c: char| !c.is_alphanumeric() && c != '_')
            .map(|i| pos + i)
            .unwrap_or(line.len());
            
        let word = &line[start..end];
        keywords.contains(&word)
    }

    /// Render annotations for a line
    fn render_line_annotations(&self, painter: &Painter, rect: Rect, line_index: usize) {
        if let Some(annotations) = self.annotations.get(&line_index) {
            let mut sorted_annotations = annotations.clone();
            sorted_annotations.sort_by_key(|ann| ann.priority);
            
            for (i, annotation) in sorted_annotations.iter().enumerate() {
                let annotation_rect = Rect::from_min_size(
                    Pos2::new(rect.max.x - 4.0 - (i as f32 * 2.0), rect.min.y),
                    Vec2::new(3.0, rect.height())
                );
                
                painter.rect_filled(annotation_rect, 1.0, annotation.color);
            }
        }
    }

    /// Render viewport indicator showing currently visible area
    fn render_viewport_indicator(&self, painter: &Painter, minimap_rect: Rect, scroll_ratio: f32) {
        if self.viewport.total_lines == 0 {
            return;
        }

        let viewport_start = (self.viewport.first_visible_line as f32 / self.viewport.total_lines as f32) * minimap_rect.height();
        let viewport_height = ((self.viewport.last_visible_line - self.viewport.first_visible_line) as f32 / self.viewport.total_lines as f32) * minimap_rect.height();
        
        let viewport_rect = Rect::from_min_size(
            Pos2::new(minimap_rect.min.x, minimap_rect.min.y + viewport_start),
            Vec2::new(minimap_rect.width(), viewport_height.max(10.0)) // Minimum height
        );
        
        // Draw viewport background
        painter.rect_filled(viewport_rect, 2.0, self.settings.viewport_color);
        
        // Draw viewport border
        painter.rect_stroke(viewport_rect, 2.0, Stroke::new(1.0, Color32::from_gray(150)));
    }

    /// Toggle minimap visibility
    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    /// Set minimap width
    pub fn set_width(&mut self, width: f32) {
        self.width = width.clamp(self.settings.min_width, self.settings.max_width);
    }

    /// Add error annotation
    pub fn add_error(&mut self, line: usize, message: String) {
        self.add_annotation(line, MinimapAnnotation {
            annotation_type: AnnotationType::Error,
            color: Color32::RED,
            priority: 100,
            tooltip: Some(message),
        });
    }

    /// Add warning annotation  
    pub fn add_warning(&mut self, line: usize, message: String) {
        self.add_annotation(line, MinimapAnnotation {
            annotation_type: AnnotationType::Warning,
            color: Color32::YELLOW,
            priority: 80,
            tooltip: Some(message),
        });
    }

    /// Add breakpoint annotation
    pub fn add_breakpoint(&mut self, line: usize) {
        self.add_annotation(line, MinimapAnnotation {
            annotation_type: AnnotationType::Breakpoint,
            color: Color32::from_rgb(200, 50, 50),
            priority: 90,
            tooltip: Some("Breakpoint".to_string()),
        });
    }

    /// Add search match annotation
    pub fn add_search_match(&mut self, line: usize) {
        self.add_annotation(line, MinimapAnnotation {
            annotation_type: AnnotationType::SearchMatch,
            color: Color32::from_rgb(255, 200, 50),
            priority: 70,
            tooltip: Some("Search match".to_string()),
        });
    }
}

/// Response from minimap interaction
#[derive(Debug, Default)]
pub struct MinimapResponse {
    /// Line that was clicked (if any)
    pub clicked_line: Option<usize>,
    /// Line that is being hovered (if any)
    pub hovered_line: Option<usize>,
}

impl Default for Minimap {
    fn default() -> Self {
        Self::new()
    }
}