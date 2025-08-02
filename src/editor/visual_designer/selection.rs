/// Component selection and interaction logic

use eframe::egui;
use std::collections::HashSet;

#[derive(Clone)]
pub struct ComponentSelection {
    pub selected: HashSet<usize>,
    pub primary: Option<usize>,
    pub selection_rect: Option<egui::Rect>,
    pub multi_select_mode: bool,
    pub dragging: Option<DragOperation>,
    pub hover_component: Option<usize>,
}

#[derive(Clone)]
pub struct DragOperation {
    pub component_indices: Vec<usize>,
    pub original_positions: Vec<egui::Pos2>,
    pub drag_offset: egui::Vec2,
    pub start_pos: egui::Pos2,
    pub drag_type: DragOperationType,
}

#[derive(Clone, Copy)]
pub enum DragOperationType {
    Move,
    Resize { handle: ResizeHandle },
    SelectionRect,
}

#[derive(Clone, Copy)]
pub enum ResizeHandle {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    TopCenter,
    BottomCenter,
    LeftCenter,
    RightCenter,
}

impl Default for ComponentSelection {
    fn default() -> Self {
        Self {
            selected: HashSet::new(),
            primary: None,
            selection_rect: None,
            multi_select_mode: false,
            dragging: None,
            hover_component: None,
        }
    }
}
impl ComponentSelection {
    pub fn select_single_component(&mut self, component_idx: usize) {
        self.selected.clear();
        self.selected.insert(component_idx);
        self.primary = Some(component_idx);
    }

    pub fn toggle_component_selection(&mut self, component_idx: usize) {
        if self.selected.contains(&component_idx) {
            self.selected.remove(&component_idx);
            if Some(component_idx) == self.primary {
                self.primary = self.selected.iter().next().copied();
            }
        } else {
            self.selected.insert(component_idx);
            if self.primary.is_none() {
                self.primary = Some(component_idx);
            }
        }
    }

    pub fn clear_selection(&mut self) {
        self.selected.clear();
        self.primary = None;
    }

    pub fn select_component(&mut self, component_idx: usize) {
        self.select_single_component(component_idx);
    }

    pub fn select_components_in_rect(&mut self, selection_rect: egui::Rect, layout: &super::LayoutManager) {
        self.selected.clear();
        for (idx, pos) in &layout.positions {
            let size = layout.sizes.get(idx).cloned().unwrap_or(egui::vec2(100.0, 30.0));
            let component_rect = egui::Rect::from_min_size(*pos, size);
            if selection_rect.intersects(component_rect) {
                self.selected.insert(*idx);
            }
        }
        self.primary = self.selected.iter().next().copied();
    }

    pub fn start_component_drag(&mut self, component_idx: usize, start_pos: egui::Pos2, layout: &super::LayoutManager) {
        if !self.selected.contains(&component_idx) {
            self.select_single_component(component_idx);
        }
        let original_positions: Vec<egui::Pos2> = self.selected
            .iter()
            .filter_map(|&idx| layout.positions.get(&idx).copied())
            .collect();
        self.dragging = Some(DragOperation {
            component_indices: self.selected.iter().copied().collect(),
            original_positions,
            drag_offset: egui::Vec2::ZERO,
            start_pos,
            drag_type: DragOperationType::Move,
        });
    }

    pub fn start_resize_drag(&mut self, component_idx: usize, handle: ResizeHandle, start_pos: egui::Pos2, layout: &super::LayoutManager) {
        if let Some(original_pos) = layout.positions.get(&component_idx).copied() {
            self.dragging = Some(DragOperation {
                component_indices: vec![component_idx],
                original_positions: vec![original_pos],
                drag_offset: egui::Vec2::ZERO,
                start_pos,
                drag_type: DragOperationType::Resize { handle },
            });
        }
    }

    pub fn start_selection_rect_drag(&mut self, start_pos: egui::Pos2) {
        self.dragging = Some(DragOperation {
            component_indices: Vec::new(),
            original_positions: Vec::new(),
            drag_offset: egui::Vec2::ZERO,
            start_pos,
            drag_type: DragOperationType::SelectionRect,
        });
        self.selection_rect = Some(egui::Rect::from_min_size(start_pos, egui::Vec2::ZERO));
    }

    pub fn check_resize_handle_hit(component_rect: egui::Rect, pointer_pos: egui::Pos2) -> Option<ResizeHandle> {
        let handle_size = 6.0;
        let handles = [
            (component_rect.min, ResizeHandle::TopLeft),
            (egui::pos2(component_rect.max.x, component_rect.min.y), ResizeHandle::TopRight),
            (component_rect.max, ResizeHandle::BottomRight),
            (egui::pos2(component_rect.min.x, component_rect.max.y), ResizeHandle::BottomLeft),
            (egui::pos2(component_rect.center().x, component_rect.min.y), ResizeHandle::TopCenter),
            (egui::pos2(component_rect.max.x, component_rect.center().y), ResizeHandle::RightCenter),
            (egui::pos2(component_rect.center().x, component_rect.max.y), ResizeHandle::BottomCenter),
            (egui::pos2(component_rect.min.x, component_rect.center().y), ResizeHandle::LeftCenter),
        ];
        for (handle_pos, handle_type) in handles {
            let handle_rect = egui::Rect::from_center_size(handle_pos, egui::vec2(handle_size, handle_size));
            if handle_rect.contains(pointer_pos) {
                return Some(handle_type);
            }
        }
        None
    }

    // Additional selection/interaction methods can be added here.
}
