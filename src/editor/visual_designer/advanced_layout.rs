//! Advanced Layout Management System
//!
//! Provides modern layout capabilities including flexbox, grid, and auto-layout
//! similar to modern design tools and web technologies.

use egui::*;
use std::collections::HashMap;

/// Advanced layout types supported by the designer
#[derive(Debug, Clone, PartialEq)]
pub enum LayoutType {
    /// Absolute positioning (traditional)
    Absolute,
    /// Flexbox layout (horizontal or vertical)
    Flexbox {
        direction: FlexDirection,
        justify_content: JustifyContent,
        align_items: AlignItems,
        wrap: FlexWrap,
    },
    /// Grid layout with rows and columns
    Grid {
        columns: usize,
        rows: usize,
        column_gap: f32,
        row_gap: f32,
    },
    /// Auto-layout (automatic sizing and spacing)
    Auto {
        direction: AutoLayoutDirection,
        spacing: f32,
        padding: Padding,
    },
}

/// Flexbox direction
#[derive(Debug, Clone, PartialEq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

/// Flexbox justify-content property
#[derive(Debug, Clone, PartialEq)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

/// Flexbox align-items property
#[derive(Debug, Clone, PartialEq)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

/// Flexbox wrap property
#[derive(Debug, Clone, PartialEq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

/// Auto-layout direction
#[derive(Debug, Clone, PartialEq)]
pub enum AutoLayoutDirection {
    Horizontal,
    Vertical,
}

/// Padding values for layouts
#[derive(Debug, Clone, PartialEq)]
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Padding {
    pub fn uniform(value: f32) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    pub fn symmetric(horizontal: f32, vertical: f32) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }
}

/// Layout container that can hold and arrange child components
#[derive(Clone)]
pub struct LayoutContainer {
    /// Container ID
    pub id: usize,
    /// Layout type and configuration
    pub layout_type: LayoutType,
    /// Child component IDs
    pub children: Vec<usize>,
    /// Container bounds
    pub bounds: Rect,
    /// Whether this container is selected
    pub selected: bool,
    /// Container styling
    pub style: ContainerStyle,
}

/// Visual styling for layout containers
#[derive(Debug, Clone)]
pub struct ContainerStyle {
    /// Background color
    pub background: Option<Color32>,
    /// Border color and width
    pub border: Option<(Color32, f32)>,
    /// Corner radius
    pub corner_radius: f32,
    /// Shadow settings
    pub shadow: Option<Shadow>,
}

/// Shadow configuration
#[derive(Debug, Clone)]
pub struct Shadow {
    pub color: Color32,
    pub offset: Vec2,
    pub blur: f32,
}

/// Advanced layout manager
pub struct AdvancedLayoutManager {
    /// Layout containers
    pub containers: HashMap<usize, LayoutContainer>,
    /// Component positions calculated by layout
    pub calculated_positions: HashMap<usize, Pos2>,
    /// Component sizes calculated by layout
    pub calculated_sizes: HashMap<usize, Vec2>,
    /// Layout constraints for components
    pub constraints: HashMap<usize, LayoutConstraints>,
    /// Next container ID
    pub next_container_id: usize,
}

/// Layout constraints for individual components
#[derive(Debug, Clone)]
pub struct LayoutConstraints {
    /// Minimum size
    pub min_size: Option<Vec2>,
    /// Maximum size
    pub max_size: Option<Vec2>,
    /// Preferred size
    pub preferred_size: Option<Vec2>,
    /// Flex grow factor (for flexbox)
    pub flex_grow: f32,
    /// Flex shrink factor (for flexbox)
    pub flex_shrink: f32,
    /// Flex basis (for flexbox)
    pub flex_basis: Option<f32>,
    /// Grid area (for grid layout)
    pub grid_area: Option<GridArea>,
}

/// Grid area specification
#[derive(Debug, Clone)]
pub struct GridArea {
    pub start_row: usize,
    pub end_row: usize,
    pub start_column: usize,
    pub end_column: usize,
}

impl Default for LayoutConstraints {
    fn default() -> Self {
        Self {
            min_size: None,
            max_size: None,
            preferred_size: None,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: None,
            grid_area: None,
        }
    }
}

impl AdvancedLayoutManager {
    /// Create a new advanced layout manager
    pub fn new() -> Self {
        Self {
            containers: HashMap::new(),
            calculated_positions: HashMap::new(),
            calculated_sizes: HashMap::new(),
            constraints: HashMap::new(),
            next_container_id: 1,
        }
    }

    /// Create a new layout container
    pub fn create_container(&mut self, layout_type: LayoutType, bounds: Rect) -> usize {
        let id = self.next_container_id;
        self.next_container_id += 1;

        let container = LayoutContainer {
            id,
            layout_type,
            children: Vec::new(),
            bounds,
            selected: false,
            style: ContainerStyle {
                background: Some(Color32::from_rgba_unmultiplied(240, 240, 240, 50)),
                border: Some((Color32::from_rgba_unmultiplied(200, 200, 200, 100), 1.0)),
                corner_radius: 4.0,
                shadow: None,
            },
        };

        self.containers.insert(id, container);
        id
    }

    /// Add a child component to a container
    pub fn add_child(&mut self, container_id: usize, child_id: usize) {
        if let Some(container) = self.containers.get_mut(&container_id) {
            if !container.children.contains(&child_id) {
                container.children.push(child_id);
                self.recalculate_layout(container_id);
            }
        }
    }

    /// Remove a child component from a container
    pub fn remove_child(&mut self, container_id: usize, child_id: usize) {
        if let Some(container) = self.containers.get_mut(&container_id) {
            container.children.retain(|&id| id != child_id);
            self.recalculate_layout(container_id);
        }
    }

    /// Recalculate layout for a container
    pub fn recalculate_layout(&mut self, container_id: usize) {
        if let Some(container) = self.containers.get(&container_id).cloned() {
            match &container.layout_type {
                LayoutType::Absolute => {
                    // No automatic layout calculation needed
                }
                LayoutType::Flexbox { direction, justify_content, align_items, .. } => {
                    self.calculate_flexbox_layout(container_id, direction, justify_content, align_items);
                }
                LayoutType::Grid { columns, rows, column_gap, row_gap } => {
                    self.calculate_grid_layout(container_id, *columns, *rows, *column_gap, *row_gap);
                }
                LayoutType::Auto { direction, spacing, padding } => {
                    self.calculate_auto_layout(container_id, direction, *spacing, padding);
                }
            }
        }
    }

    /// Calculate flexbox layout
    fn calculate_flexbox_layout(
        &mut self,
        container_id: usize,
        direction: &FlexDirection,
        justify_content: &JustifyContent,
        _align_items: &AlignItems,
    ) {
        let container = match self.containers.get(&container_id) {
            Some(c) => c.clone(),
            None => return,
        };

        let is_row = matches!(direction, FlexDirection::Row | FlexDirection::RowReverse);
        let content_area = container.bounds;
        
        // Calculate available space
        let available_size = if is_row {
            content_area.width()
        } else {
            content_area.height()
        };

        // Determine component sizes based on flex properties
        let mut total_flex_grow = 0.0;
        let mut fixed_size = 0.0;

        for &child_id in &container.children {
            if let Some(constraints) = self.constraints.get(&child_id) {
                total_flex_grow += constraints.flex_grow;
                if let Some(basis) = constraints.flex_basis {
                    fixed_size += basis;
                } else if let Some(preferred) = constraints.preferred_size {
                    fixed_size += if is_row { preferred.x } else { preferred.y };
                } else {
                    fixed_size += if is_row { 100.0 } else { 30.0 }; // Default size
                }
            } else {
                fixed_size += if is_row { 100.0 } else { 30.0 }; // Default size
            }
        }

        // Calculate positions based on justify_content
        let remaining_space = available_size - fixed_size;
        let flex_unit = if total_flex_grow > 0.0 {
            remaining_space / total_flex_grow
        } else {
            0.0
        };

        let mut current_pos = match justify_content {
            JustifyContent::FlexStart => 0.0,
            JustifyContent::FlexEnd => remaining_space.max(0.0),
            JustifyContent::Center => (remaining_space / 2.0).max(0.0),
            _ => 0.0,
        };

        // Position children
        for &child_id in &container.children {
            let constraints = self.constraints.get(&child_id).cloned()
                .unwrap_or_default();
                
            let child_size = if let Some(basis) = constraints.flex_basis {
                basis + (constraints.flex_grow * flex_unit)
            } else if let Some(preferred) = constraints.preferred_size {
                let base_size = if is_row { preferred.x } else { preferred.y };
                base_size + (constraints.flex_grow * flex_unit)
            } else {
                let default_size = if is_row { 100.0 } else { 30.0 };
                default_size + (constraints.flex_grow * flex_unit)
            };

            let child_pos = if is_row {
                Pos2::new(content_area.min.x + current_pos, content_area.min.y)
            } else {
                Pos2::new(content_area.min.x, content_area.min.y + current_pos)
            };

            let child_full_size = if is_row {
                Vec2::new(child_size, content_area.height())
            } else {
                Vec2::new(content_area.width(), child_size)
            };

            self.calculated_positions.insert(child_id, child_pos);
            self.calculated_sizes.insert(child_id, child_full_size);

            current_pos += child_size;
        }
    }

    /// Calculate grid layout
    fn calculate_grid_layout(&mut self, container_id: usize, columns: usize, rows: usize, column_gap: f32, row_gap: f32) {
        let container = match self.containers.get(&container_id) {
            Some(c) => c.clone(),
            None => return,
        };

        let content_area = container.bounds;
        let cell_width = (content_area.width() - (column_gap * (columns - 1) as f32)) / columns as f32;
        let cell_height = (content_area.height() - (row_gap * (rows - 1) as f32)) / rows as f32;

        for (index, &child_id) in container.children.iter().enumerate() {
            let row = index / columns;
            let col = index % columns;

            if row >= rows {
                break; // Component doesn't fit in grid
            }

            let x = content_area.min.x + (col as f32 * (cell_width + column_gap));
            let y = content_area.min.y + (row as f32 * (cell_height + row_gap));

            self.calculated_positions.insert(child_id, Pos2::new(x, y));
            self.calculated_sizes.insert(child_id, Vec2::new(cell_width, cell_height));
        }
    }

    /// Calculate auto layout
    fn calculate_auto_layout(&mut self, container_id: usize, direction: &AutoLayoutDirection, spacing: f32, padding: &Padding) {
        let container = match self.containers.get(&container_id) {
            Some(c) => c.clone(),
            None => return,
        };

        let content_area = Rect::from_min_size(
            container.bounds.min + Vec2::new(padding.left, padding.top),
            container.bounds.size() - Vec2::new(padding.left + padding.right, padding.top + padding.bottom),
        );

        let mut current_pos = content_area.min;

        for &child_id in &container.children {
            let constraints = self.constraints.get(&child_id).cloned()
                .unwrap_or_default();

            let child_size = constraints.preferred_size.unwrap_or_else(|| {
                match direction {
                    AutoLayoutDirection::Horizontal => Vec2::new(100.0, content_area.height()),
                    AutoLayoutDirection::Vertical => Vec2::new(content_area.width(), 30.0),
                }
            });

            self.calculated_positions.insert(child_id, current_pos);
            self.calculated_sizes.insert(child_id, child_size);

            match direction {
                AutoLayoutDirection::Horizontal => {
                    current_pos.x += child_size.x + spacing;
                }
                AutoLayoutDirection::Vertical => {
                    current_pos.y += child_size.y + spacing;
                }
            }
        }
    }

    /// Set layout constraints for a component
    pub fn set_constraints(&mut self, component_id: usize, constraints: LayoutConstraints) {
        self.constraints.insert(component_id, constraints);
        
        // Recalculate layouts for containers that contain this component
        let containers_to_update: Vec<usize> = self.containers
            .iter()
            .filter(|(_, container)| container.children.contains(&component_id))
            .map(|(&id, _)| id)
            .collect();
            
        for container_id in containers_to_update {
            self.recalculate_layout(container_id);
        }
    }

    /// Render layout containers with visual indicators
    pub fn render_containers(&self, _ui: &mut Ui, painter: &Painter) {
        for container in self.containers.values() {
            // Draw container background
            if let Some(bg_color) = container.style.background {
                painter.rect_filled(container.bounds, container.style.corner_radius, bg_color);
            }

            // Draw container border
            if let Some((border_color, border_width)) = container.style.border {
                painter.rect_stroke(container.bounds, container.style.corner_radius, 
                    Stroke::new(border_width, border_color));
            }

            // Draw selection indicator
            if container.selected {
                painter.rect_stroke(container.bounds, container.style.corner_radius,
                    Stroke::new(2.0, Color32::BLUE));
            }

            // Draw layout type indicator
            self.draw_layout_indicator(painter, container);
        }
    }

    /// Draw visual indicator for layout type
    fn draw_layout_indicator(&self, painter: &Painter, container: &LayoutContainer) {
        let icon_pos = container.bounds.min + Vec2::new(8.0, 8.0);
        let _icon_size = 16.0;
        
        match &container.layout_type {
            LayoutType::Flexbox { direction, .. } => {
                let icon = match direction {
                    FlexDirection::Row | FlexDirection::RowReverse => "↔",
                    FlexDirection::Column | FlexDirection::ColumnReverse => "↕",
                };
                painter.text(icon_pos, Align2::LEFT_TOP, icon, FontId::default(), Color32::BLUE);
            }
            LayoutType::Grid { .. } => {
                painter.text(icon_pos, Align2::LEFT_TOP, "⊞", FontId::default(), Color32::GREEN);
            }
            LayoutType::Auto { .. } => {
                painter.text(icon_pos, Align2::LEFT_TOP, "↻", FontId::default(), Color32::from_rgb(255, 165, 0));
            }
            LayoutType::Absolute => {
                // No indicator for absolute layout
            }
        }
    }

    /// Get calculated position for a component
    pub fn get_calculated_position(&self, component_id: usize) -> Option<Pos2> {
        self.calculated_positions.get(&component_id).copied()
    }

    /// Get calculated size for a component
    pub fn get_calculated_size(&self, component_id: usize) -> Option<Vec2> {
        self.calculated_sizes.get(&component_id).copied()
    }

    /// Create a quick flexbox container
    pub fn create_flexbox(&mut self, bounds: Rect, direction: FlexDirection) -> usize {
        self.create_container(
            LayoutType::Flexbox {
                direction,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                wrap: FlexWrap::NoWrap,
            },
            bounds,
        )
    }

    /// Create a quick grid container
    pub fn create_grid(&mut self, bounds: Rect, columns: usize, rows: usize) -> usize {
        self.create_container(
            LayoutType::Grid {
                columns,
                rows,
                column_gap: 8.0,
                row_gap: 8.0,
            },
            bounds,
        )
    }

    /// Create a quick auto-layout container
    pub fn create_auto_layout(&mut self, bounds: Rect, direction: AutoLayoutDirection) -> usize {
        self.create_container(
            LayoutType::Auto {
                direction,
                spacing: 8.0,
                padding: Padding::uniform(8.0),
            },
            bounds,
        )
    }
}

impl Default for AdvancedLayoutManager {
    fn default() -> Self {
        Self::new()
    }
}