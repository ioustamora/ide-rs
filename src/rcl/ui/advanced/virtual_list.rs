//! Virtual List Component
//!
//! High-performance virtualized list for handling large datasets efficiently.
//! Only renders visible items to maintain smooth scrolling and low memory usage.

use egui::*;
use std::ops::Range;
use std::sync::Arc;

/// Virtual list component for efficient rendering of large datasets
pub struct VirtualList<T> {
    /// All items in the list
    pub items: Arc<Vec<T>>,
    /// Height of each item in pixels
    pub item_height: f32,
    /// Current scroll position
    pub scroll_position: f32,
    /// Visible area height
    pub viewport_height: f32,
    /// Number of items to render outside visible area (buffer)
    pub overscan: usize,
    /// Currently visible range
    pub visible_range: Range<usize>,
    /// Custom item renderer
    pub item_renderer: Option<Box<dyn Fn(&mut Ui, usize, &T)>>,
    /// List styling
    pub style: VirtualListStyle,
    /// Selection state
    pub selection: VirtualListSelection,
    /// Search and filtering
    pub filter: Option<VirtualListFilter<T>>,
    /// Performance monitoring
    pub performance: VirtualListPerformance,
}

/// Virtual list styling configuration
#[derive(Clone)]
pub struct VirtualListStyle {
    /// Background color
    pub background_color: Color32,
    /// Item background color (normal)
    pub item_background: Color32,
    /// Item background color (hover)
    pub item_hover_background: Color32,
    /// Item background color (selected)
    pub item_selected_background: Color32,
    /// Item text color
    pub item_text_color: Color32,
    /// Item padding
    pub item_padding: Margin,
    /// Scrollbar styling
    pub scrollbar_style: ScrollbarStyle,
    /// Separator line color
    pub separator_color: Option<Color32>,
}

#[derive(Clone)]
pub struct ScrollbarStyle {
    pub width: f32,
    pub background_color: Color32,
    pub thumb_color: Color32,
    pub thumb_hover_color: Color32,
}

/// Selection management for virtual list
#[derive(Default)]
pub struct VirtualListSelection {
    /// Currently selected indices
    pub selected_indices: std::collections::HashSet<usize>,
    /// Allow multiple selection
    pub multi_select: bool,
    /// Last clicked index for range selection
    pub last_clicked: Option<usize>,
    /// Selection callbacks
    pub on_selection_changed: Option<Box<dyn Fn(&std::collections::HashSet<usize>)>>,
}

/// Filtering functionality for virtual list
pub struct VirtualListFilter<T> {
    /// Filter predicate
    pub predicate: Box<dyn Fn(&T) -> bool>,
    /// Filtered indices (cached)
    pub filtered_indices: Vec<usize>,
    /// Filter is dirty and needs recalculation
    pub dirty: bool,
}

/// Performance monitoring for optimization
#[derive(Default)]
pub struct VirtualListPerformance {
    /// Number of rendered items in last frame
    pub rendered_items: usize,
    /// Render time in milliseconds
    pub render_time_ms: f32,
    /// Frame rate
    pub fps: f32,
    /// Memory usage estimate
    pub memory_usage_bytes: usize,
}

impl<T> VirtualList<T> {
    /// Create a new virtual list
    pub fn new(items: Arc<Vec<T>>) -> Self {
        Self {
            items,
            item_height: 24.0,
            scroll_position: 0.0,
            viewport_height: 400.0,
            overscan: 5,
            visible_range: 0..0,
            item_renderer: None,
            style: VirtualListStyle::default(),
            selection: VirtualListSelection::default(),
            filter: None,
            performance: VirtualListPerformance::default(),
        }
    }
    
    /// Set custom item renderer
    pub fn with_item_renderer<F>(mut self, renderer: F) -> Self
    where
        F: Fn(&mut Ui, usize, &T) + 'static,
    {
        self.item_renderer = Some(Box::new(renderer));
        self
    }
    
    /// Enable multi-selection
    pub fn with_multi_select(mut self, enabled: bool) -> Self {
        self.selection.multi_select = enabled;
        self
    }
    
    /// Set item height
    pub fn with_item_height(mut self, height: f32) -> Self {
        self.item_height = height;
        self
    }
    
    /// Set viewport height
    pub fn with_viewport_height(mut self, height: f32) -> Self {
        self.viewport_height = height;
        self
    }
    
    /// Calculate visible range based on scroll position
    fn calculate_visible_range(&self, available_height: f32) -> Range<usize> {
        let total_items = self.get_filtered_count();
        if total_items == 0 {
            return 0..0;
        }
        
        let start_index = (self.scroll_position / self.item_height) as usize;
        let visible_items = (available_height / self.item_height).ceil() as usize + 1;
        
        let start_with_overscan = start_index.saturating_sub(self.overscan);
        let end_with_overscan = (start_index + visible_items + self.overscan).min(total_items);
        
        start_with_overscan..end_with_overscan
    }
    
    /// Get the number of items after filtering
    fn get_filtered_count(&self) -> usize {
        if let Some(ref filter) = self.filter {
            filter.filtered_indices.len()
        } else {
            self.items.len()
        }
    }
    
    /// Get item at filtered index
    fn get_filtered_item(&self, filtered_index: usize) -> Option<(usize, &T)> {
        if let Some(ref filter) = self.filter {
            filter.filtered_indices.get(filtered_index)
                .and_then(|&real_index| self.items.get(real_index).map(|item| (real_index, item)))
        } else {
            self.items.get(filtered_index).map(|item| (filtered_index, item))
        }
    }
    
    /// Update filter if dirty
    fn update_filter(&mut self) {
        if let Some(ref mut filter) = self.filter {
            if filter.dirty {
                filter.filtered_indices.clear();
                for (index, item) in self.items.iter().enumerate() {
                    if (filter.predicate)(item) {
                        filter.filtered_indices.push(index);
                    }
                }
                filter.dirty = false;
            }
        }
    }
    
    /// Render the virtual list
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let start_time = std::time::Instant::now();
        
        // Update filter if necessary
        self.update_filter();
        
        let total_items = self.get_filtered_count();
        let total_height = total_items as f32 * self.item_height;
        
        let available_rect = ui.available_rect_before_wrap();
        self.viewport_height = available_rect.height();
        
        // Calculate visible range
        let visible_range = self.calculate_visible_range(self.viewport_height);
        self.visible_range = visible_range.clone();
        
        let response = ScrollArea::vertical()
            .max_height(self.viewport_height)
            .show(ui, |ui| {
                // Create a large invisible spacer for the total content height
                ui.allocate_space(vec2(available_rect.width(), total_height));
                
                // Render only visible items
                for filtered_index in visible_range {
                    if let Some((real_index, item)) = self.get_filtered_item(filtered_index) {
                        let item_rect = Rect::from_min_size(
                            pos2(available_rect.min.x, filtered_index as f32 * self.item_height),
                            vec2(available_rect.width(), self.item_height),
                        );
                        
                        // Check if item is selected
                        let is_selected = self.selection.selected_indices.contains(&real_index);
                        let is_hovered = ui.ctx().pointer_hover_pos()
                            .map_or(false, |pos| item_rect.contains(pos));
                        
                        // Draw item background
                        let bg_color = if is_selected {
                            self.style.item_selected_background
                        } else if is_hovered {
                            self.style.item_hover_background
                        } else {
                            self.style.item_background
                        };
                        
                        ui.painter().rect_filled(item_rect, 0.0, bg_color);
                        
                        // Render item content
                        let mut item_ui = ui.child_ui(item_rect, Layout::left_to_right(Align::Center));
                        
                        if let Some(ref renderer) = self.item_renderer {
                            renderer(&mut item_ui, real_index, item);
                        } else {
                            // Default renderer
                            item_ui.label(format!("Item {}", real_index));
                        }
                        
                        // Handle selection
                        let item_response = item_ui.interact(item_rect, Id::new(("virtual_list_item", real_index)), Sense::click());
                        if item_response.clicked() {
                            self.handle_item_click(real_index, ui.input(|i| i.modifiers.shift || i.modifiers.ctrl));
                        }
                        
                        // Draw separator if enabled
                        if let Some(separator_color) = self.style.separator_color {
                            ui.painter().line_segment(
                                [
                                    pos2(item_rect.min.x, item_rect.max.y),
                                    pos2(item_rect.max.x, item_rect.max.y),
                                ],
                                Stroke::new(1.0, separator_color),
                            );
                        }
                    }
                }
            }).inner;
        
        // Update scroll position
        if let Some(scroll_delta) = ui.input(|i| {
            if i.scroll_delta.y != 0.0 {
                Some(-i.scroll_delta.y)
            } else {
                None
            }
        }) {
            self.scroll_position = (self.scroll_position + scroll_delta).max(0.0);
        }
        
        // Update performance metrics
        self.performance.render_time_ms = start_time.elapsed().as_secs_f32() * 1000.0;
        self.performance.rendered_items = visible_range.len();
        self.performance.memory_usage_bytes = visible_range.len() * std::mem::size_of::<T>();
        
        response
    }
    
    /// Handle item click for selection
    fn handle_item_click(&mut self, index: usize, modifier_held: bool) {
        if self.selection.multi_select && modifier_held {
            // Toggle selection
            if self.selection.selected_indices.contains(&index) {
                self.selection.selected_indices.remove(&index);
            } else {
                self.selection.selected_indices.insert(index);
            }
        } else {
            // Single selection
            self.selection.selected_indices.clear();
            self.selection.selected_indices.insert(index);
        }
        
        self.selection.last_clicked = Some(index);
        
        // Call selection callback
        if let Some(ref callback) = self.selection.on_selection_changed {
            callback(&self.selection.selected_indices);
        }
    }
    
    /// Set filter predicate
    pub fn set_filter<F>(&mut self, predicate: F)
    where
        F: Fn(&T) -> bool + 'static,
    {
        self.filter = Some(VirtualListFilter {
            predicate: Box::new(predicate),
            filtered_indices: Vec::new(),
            dirty: true,
        });
    }
    
    /// Clear filter
    pub fn clear_filter(&mut self) {
        self.filter = None;
    }
    
    /// Get selected items
    pub fn get_selected_items(&self) -> Vec<&T> {
        self.selection.selected_indices
            .iter()
            .filter_map(|&index| self.items.get(index))
            .collect()
    }
    
    /// Scroll to item
    pub fn scroll_to_item(&mut self, index: usize) {
        let target_scroll = index as f32 * self.item_height;
        self.scroll_position = target_scroll;
    }
    
    /// Get performance statistics
    pub fn get_performance_stats(&self) -> &VirtualListPerformance {
        &self.performance
    }
}

impl Default for VirtualListStyle {
    fn default() -> Self {
        Self {
            background_color: Color32::from_rgb(32, 32, 32),
            item_background: Color32::TRANSPARENT,
            item_hover_background: Color32::from_rgba_unmultiplied(255, 255, 255, 20),
            item_selected_background: Color32::from_rgb(0, 120, 215),
            item_text_color: Color32::WHITE,
            item_padding: Margin::same(8.0),
            scrollbar_style: ScrollbarStyle {
                width: 12.0,
                background_color: Color32::from_rgb(60, 60, 60),
                thumb_color: Color32::from_rgb(100, 100, 100),
                thumb_hover_color: Color32::from_rgb(140, 140, 140),
            },
            separator_color: Some(Color32::from_rgb(60, 60, 60)),
        }
    }
}