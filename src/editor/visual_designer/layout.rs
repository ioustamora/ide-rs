use egui;
use std::collections::HashMap;

/// Layout management for components
#[derive(Default, Clone)]
pub struct LayoutManager {
    /// Component positions
    pub positions: HashMap<usize, egui::Pos2>,
    /// Component sizes
    pub sizes: HashMap<usize, egui::Vec2>,
    /// Z-order (layering)
    pub z_order: Vec<usize>,
    /// Alignment helpers
    pub alignment: AlignmentTools,
}

impl LayoutManager {
    pub fn get_or_init_position(&mut self, idx: usize) -> egui::Pos2 {
        self.positions.get(&idx).copied().unwrap_or_else(|| {
            let columns = 3;
            let col = idx % columns;
            let row = idx / columns;
            let spacing_x = 150.0;
            let spacing_y = 60.0;
            let start_x = 50.0;
            let start_y = 50.0;
            let default_pos = egui::pos2(
                start_x + (col as f32 * spacing_x),
                start_y + (row as f32 * spacing_y)
            );
            self.positions.insert(idx, default_pos);
            default_pos
        })
    }

    pub fn get_or_init_size(&mut self, idx: usize, component_name: &str) -> egui::Vec2 {
        self.sizes.get(&idx).copied().unwrap_or_else(|| {
            let default_size = match component_name {
                "Button" => egui::vec2(100.0, 32.0),
                "Label" => egui::vec2(80.0, 24.0),
                "TextBox" => egui::vec2(140.0, 28.0),
                "Checkbox" => egui::vec2(120.0, 24.0),
                "Slider" => egui::vec2(140.0, 24.0),
                "Dropdown" => egui::vec2(120.0, 28.0),
                _ => egui::vec2(100.0, 32.0),
            };
            self.sizes.insert(idx, default_size);
            default_size
        })
    }
    
    /// Align selected components
    pub fn align_components(&mut self, component_indices: &[usize], operation: AlignmentOperation) {
        if component_indices.len() < 2 {
            return;
        }
        
        let mut component_bounds: Vec<ComponentBounds> = component_indices
            .iter()
            .filter_map(|&idx| {
                let pos = self.positions.get(&idx)?;
                let size = self.sizes.get(&idx)?;
                Some(ComponentBounds {
                    position: *pos,
                    size: *size,
                    index: idx,
                })
            })
            .collect();
        
        if component_bounds.is_empty() {
            return;
        }
        
        match operation {
            AlignmentOperation::AlignLeft => {
                let min_x = component_bounds
                    .iter()
                    .map(|b| b.position.x)
                    .fold(f32::INFINITY, f32::min);
                for bounds in &mut component_bounds {
                    bounds.position.x = min_x;
                    self.positions.insert(bounds.index, bounds.position);
                }
            }
            AlignmentOperation::AlignRight => {
                let max_x = component_bounds
                    .iter()
                    .map(|b| b.position.x + b.size.x)
                    .fold(f32::NEG_INFINITY, f32::max);
                for bounds in &mut component_bounds {
                    bounds.position.x = max_x - bounds.size.x;
                    self.positions.insert(bounds.index, bounds.position);
                }
            }
            AlignmentOperation::AlignTop => {
                let min_y = component_bounds
                    .iter()
                    .map(|b| b.position.y)
                    .fold(f32::INFINITY, f32::min);
                for bounds in &mut component_bounds {
                    bounds.position.y = min_y;
                    self.positions.insert(bounds.index, bounds.position);
                }
            }
            AlignmentOperation::AlignBottom => {
                let max_y = component_bounds
                    .iter()
                    .map(|b| b.position.y + b.size.y)
                    .fold(f32::NEG_INFINITY, f32::max);
                for bounds in &mut component_bounds {
                    bounds.position.y = max_y - bounds.size.y;
                    self.positions.insert(bounds.index, bounds.position);
                }
            }
            AlignmentOperation::AlignCenterHorizontal => {
                // Get the center of all selected components
                let total_x = component_bounds.iter().map(|b| b.position.x + b.size.x / 2.0).sum::<f32>();
                let center_x = total_x / component_bounds.len() as f32;
                for bounds in &mut component_bounds {
                    bounds.position.x = center_x - bounds.size.x / 2.0;
                    self.positions.insert(bounds.index, bounds.position);
                }
            }
            AlignmentOperation::AlignCenterVertical => {
                // Get the center of all selected components
                let total_y = component_bounds.iter().map(|b| b.position.y + b.size.y / 2.0).sum::<f32>();
                let center_y = total_y / component_bounds.len() as f32;
                for bounds in &mut component_bounds {
                    bounds.position.y = center_y - bounds.size.y / 2.0;
                    self.positions.insert(bounds.index, bounds.position);
                }
            }
            _ => {} // Other operations not implemented
        }
    }
    
    /// Distribute components evenly
    pub fn distribute_components(&mut self, component_indices: &[usize], horizontal: bool) {
        if component_indices.len() < 3 {
            return;
        }
        
        let mut component_bounds: Vec<ComponentBounds> = component_indices
            .iter()
            .filter_map(|&idx| {
                let pos = self.positions.get(&idx)?;
                let size = self.sizes.get(&idx)?;
                Some(ComponentBounds {
                    position: *pos,
                    size: *size,
                    index: idx,
                })
            })
            .collect();
        
        if component_bounds.len() < 3 {
            return;
        }
        
        if horizontal {
            // Sort by X position
            component_bounds.sort_by(|a, b| a.position.x.partial_cmp(&b.position.x).unwrap());
            
            let first_x = component_bounds.first().unwrap().position.x;
            let last_x = component_bounds.last().unwrap().position.x + component_bounds.last().unwrap().size.x;
            let total_width = last_x - first_x;
            
            // Calculate spacing
            let component_widths: f32 = component_bounds.iter().map(|b| b.size.x).sum();
            let available_space = total_width - component_widths;
            let spacing = available_space / (component_bounds.len() - 1) as f32;
            
            // Distribute components
            let mut current_x = first_x;
            for bounds in &mut component_bounds {
                bounds.position.x = current_x;
                self.positions.insert(bounds.index, bounds.position);
                current_x += bounds.size.x + spacing;
            }
        } else {
            // Sort by Y position
            component_bounds.sort_by(|a, b| a.position.y.partial_cmp(&b.position.y).unwrap());
            
            let first_y = component_bounds.first().unwrap().position.y;
            let last_y = component_bounds.last().unwrap().position.y + component_bounds.last().unwrap().size.y;
            let total_height = last_y - first_y;
            
            // Calculate spacing
            let component_heights: f32 = component_bounds.iter().map(|b| b.size.y).sum();
            let available_space = total_height - component_heights;
            let spacing = available_space / (component_bounds.len() - 1) as f32;
            
            // Distribute components
            let mut current_y = first_y;
            for bounds in &mut component_bounds {
                bounds.position.y = current_y;
                self.positions.insert(bounds.index, bounds.position);
                current_y += bounds.size.y + spacing;
            }
        }
    }
}

/// Alignment and distribution tools
#[derive(Default, Clone)]
pub struct AlignmentTools {
    /// Last alignment operation
    pub last_operation: Option<AlignmentOperation>,
    /// Recent operations for processing
    pub recent_operations: Vec<AlignmentOperation>,
}

impl AlignmentTools {
    pub fn render_toolbar(&mut self, ui: &mut egui::Ui, selected_count: usize) {
        if selected_count > 1 {
            ui.horizontal(|ui| {
                ui.label("Align:");
                if ui.button("←").on_hover_text("Align Left").clicked() {
                    self.recent_operations.push(AlignmentOperation::AlignLeft);
                }
                if ui.button("→").on_hover_text("Align Right").clicked() {
                    self.recent_operations.push(AlignmentOperation::AlignRight);
                }
                if ui.button("↑").on_hover_text("Align Top").clicked() {
                    self.recent_operations.push(AlignmentOperation::AlignTop);
                }
                if ui.button("↓").on_hover_text("Align Bottom").clicked() {
                    self.recent_operations.push(AlignmentOperation::AlignBottom);
                }
                if ui.button("⊢").on_hover_text("Center Horizontal").clicked() {
                    self.recent_operations.push(AlignmentOperation::AlignCenterHorizontal);
                }
                if ui.button("⊥").on_hover_text("Center Vertical").clicked() {
                    self.recent_operations.push(AlignmentOperation::AlignCenterVertical);
                }
            });
        }
    }

    pub fn get_recent_operations(&self) -> &Vec<AlignmentOperation> {
        &self.recent_operations
    }

    pub fn clear_recent_operations(&mut self) {
        self.recent_operations.clear();
    }

    pub fn apply_operation(
        &mut self,
        operation: &AlignmentOperation,
        component_bounds: &mut Vec<ComponentBounds>,
        canvas_rect: egui::Rect,
    ) -> bool {
        if component_bounds.is_empty() {
            return false;
        }

        match operation {
            AlignmentOperation::AlignLeft => {
                let min_x = component_bounds.iter().map(|b| b.position.x).fold(f32::INFINITY, f32::min);
                for bounds in component_bounds {
                    bounds.position.x = min_x;
                }
            }
            AlignmentOperation::AlignRight => {
                let max_x = component_bounds.iter()
                    .map(|b| b.position.x + b.size.x)
                    .fold(f32::NEG_INFINITY, f32::max);
                for bounds in component_bounds {
                    bounds.position.x = max_x - bounds.size.x;
                }
            }
            AlignmentOperation::AlignTop => {
                let min_y = component_bounds.iter().map(|b| b.position.y).fold(f32::INFINITY, f32::min);
                for bounds in component_bounds {
                    bounds.position.y = min_y;
                }
            }
            AlignmentOperation::AlignBottom => {
                let max_y = component_bounds.iter()
                    .map(|b| b.position.y + b.size.y)
                    .fold(f32::NEG_INFINITY, f32::max);
                for bounds in component_bounds {
                    bounds.position.y = max_y - bounds.size.y;
                }
            }
            AlignmentOperation::AlignCenterHorizontal => {
                let center_x = canvas_rect.center().x;
                for bounds in component_bounds {
                    bounds.position.x = center_x - bounds.size.x / 2.0;
                }
            }
            AlignmentOperation::AlignCenterVertical => {
                let center_y = canvas_rect.center().y;
                for bounds in component_bounds {
                    bounds.position.y = center_y - bounds.size.y / 2.0;
                }
            }
            _ => return false, // Other operations not implemented yet
        }
        
        self.last_operation = Some(*operation);
        true
    }
}
pub type ComponentId = usize;

/// Component bounds for alignment operations
#[derive(Clone, Debug)]
pub struct ComponentBounds {
    pub position: egui::Pos2,
    pub size: egui::Vec2,
    pub index: usize,
}

#[derive(Clone, Debug)]
pub struct ConstraintSystem {
    pub horizontal: HashMap<ComponentId, Vec<HorizontalConstraint>>,
    pub vertical: HashMap<ComponentId, Vec<VerticalConstraint>>,
    pub aspect_ratios: HashMap<ComponentId, f32>,
    pub size_constraints: HashMap<ComponentId, SizeConstraint>,
}

impl ConstraintSystem {
    pub fn new() -> Self {
        Self {
            horizontal: HashMap::new(),
            vertical: HashMap::new(),
            aspect_ratios: HashMap::new(),
            size_constraints: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum AutoLayoutMode {
    Stack {
        direction: StackDirection,
        spacing: f32,
        padding: EdgeInsets,
        alignment: StackAlignment,
    },
    Grid {
        columns: GridColumns,
        rows: GridRows,
        gap: Gap,
        alignment: GridAlignment,
    },
    Wrap {
        direction: WrapDirection,
        spacing: f32,
        alignment: WrapAlignment,
    },
    Absolute,
}

#[derive(Clone, Debug)]
pub enum GridColumns {
    Fixed(usize),
    Auto,
    FitContent,
    Stretch,
    Custom(Vec<GridTrack>),
}

#[derive(Clone, Debug)]
pub enum GridRows {
    Fixed(usize),
    Auto,
    FitContent,
    Custom(Vec<GridTrack>),
}

#[derive(Clone, Debug)]
pub enum GridTrack {
    Fixed(f32),
    Fraction(f32),
    MinMax(f32, f32),
    Auto,
}

#[derive(Clone, Debug)]
pub struct Gap {
    pub row: f32,
    pub column: f32,
}

#[derive(Clone, Debug)]
pub struct EdgeInsets {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Clone, Debug)]
pub enum StackDirection {
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug)]
pub enum StackAlignment {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Clone, Debug)]
pub enum WrapDirection {
    Row,
    Column,
}

#[derive(Clone, Debug)]
pub enum WrapAlignment {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
}

#[derive(Clone, Debug)]
pub enum GridAlignment {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Clone, Debug)]
pub enum SizeConstraint {
    Fixed(f32),
    Min(f32),
    Max(f32),
    Range(f32, f32),
    Fill,
    FitContent,
}

#[derive(Clone, Debug)]
pub enum HorizontalConstraint {
    LeftToLeft(ComponentId, f32),
    LeftToRight(ComponentId, f32),
    RightToLeft(ComponentId, f32),
    RightToRight(ComponentId, f32),
    CenterX(ComponentId, f32),
    LeftMargin(f32),
    RightMargin(f32),
}

#[derive(Clone, Debug)]
pub enum VerticalConstraint {
    TopToTop(ComponentId, f32),
    TopToBottom(ComponentId, f32),
    BottomToTop(ComponentId, f32),
    BottomToBottom(ComponentId, f32),
    CenterY(ComponentId, f32),
    TopMargin(f32),
    BottomMargin(f32),
}
/// Types of alignment operations
#[derive(Clone, Copy)]
pub enum AlignmentOperation {
    AlignLeft,
    AlignRight,
    AlignTop,
    AlignBottom,
    AlignCenterHorizontal,
    AlignCenterVertical,
    DistributeHorizontal,
    DistributeVertical,
    SameWidth,
    SameHeight,
    SameSize,
}
