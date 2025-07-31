//! Editor state for managing drag-and-drop and selection

#[allow(dead_code)]
pub struct EditorState {
    pub selected_component: Option<usize>,
    pub dragging_component: Option<usize>,
}

#[allow(dead_code)]
impl EditorState {
    pub fn new() -> Self {
        Self {
            selected_component: None,
            dragging_component: None,
        }
    }
}
