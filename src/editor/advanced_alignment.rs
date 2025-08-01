//! Advanced Alignment and Distribution Tools
//!
//! Professional-grade alignment tools inspired by RAD Studio's alignment palette.
//! Provides comprehensive component alignment, distribution, and sizing operations.

use egui::*;

/// Advanced alignment system for professional UI design
pub struct AdvancedAlignment {
    /// Available alignment operations
    pub operations: Vec<AlignmentTool>,
    /// Quick access toolbar enabled
    pub show_toolbar: bool,
    /// Alignment guides settings
    pub guides: AlignmentGuides,
    /// Component spacing settings
    pub spacing: SpacingSettings,
    /// Recently used operations
    pub recent_operations: Vec<AlignmentOperation>,
}

/// Individual alignment tool definition
#[derive(Debug, Clone)]
pub struct AlignmentTool {
    /// Tool name
    pub name: String,
    /// Tool icon
    pub icon: String,
    /// Tooltip description
    pub tooltip: String,
    /// Alignment operation
    pub operation: AlignmentOperation,
    /// Keyboard shortcut
    pub shortcut: Option<String>,
    /// Whether tool is enabled
    pub enabled: bool,
}

/// Comprehensive alignment operations
#[derive(Debug, Clone, PartialEq)]
pub enum AlignmentOperation {
    // Horizontal Alignment
    AlignLeft,
    AlignRight,
    AlignCenterHorizontal,
    
    // Vertical Alignment  
    AlignTop,
    AlignBottom,
    AlignCenterVertical,
    
    // Distribution
    DistributeHorizontally,
    DistributeVertically,
    DistributeSpaceHorizontally,
    DistributeSpaceVertically,
    
    // Sizing
    SameWidth,
    SameHeight,
    SameSize,
    FitToContainer,
    
    // Advanced Operations
    AlignToGrid,
    AlignToMargins,
    CenterInParent,
    ArrangeInGrid,
    
    // Spacing Operations
    SpaceEvenly,
    RemoveSpaces,
    IncreaseSpacing,
    DecreaseSpacing,
}

/// Alignment guide system
#[derive(Debug, Clone)]
pub struct AlignmentGuides {
    /// Show alignment guides
    pub enabled: bool,
    /// Guide line color
    pub guide_color: Color32,
    /// Guide line thickness
    pub line_thickness: f32,
    /// Show margin guides
    pub show_margins: bool,
    /// Show center guides
    pub show_center_guides: bool,
    /// Show distribution guides
    pub show_distribution_guides: bool,
    /// Guide snap distance
    pub snap_distance: f32,
}

/// Spacing and margin settings
#[derive(Debug, Clone)]
pub struct SpacingSettings {
    /// Default spacing between components
    pub default_spacing: f32,
    /// Minimum spacing allowed
    pub min_spacing: f32,
    /// Maximum spacing allowed
    pub max_spacing: f32,
    /// Spacing increment for operations
    pub spacing_increment: f32,
    /// Margin from container edges
    pub container_margin: Vec2,
}

/// Component bounds information
#[derive(Debug, Clone)]
pub struct ComponentBounds {
    /// Component position
    pub position: Pos2,
    /// Component size
    pub size: Vec2,
    /// Component index
    pub index: usize,
}

impl Default for AdvancedAlignment {
    fn default() -> Self {
        Self {
            operations: Self::create_default_tools(),
            show_toolbar: true,
            guides: AlignmentGuides::default(),
            spacing: SpacingSettings::default(),
            recent_operations: Vec::new(),
        }
    }
}

impl Default for AlignmentGuides {
    fn default() -> Self {
        Self {
            enabled: true,
            guide_color: Color32::from_rgba_unmultiplied(0, 150, 255, 128),
            line_thickness: 1.0,
            show_margins: true,
            show_center_guides: true,
            show_distribution_guides: false,
            snap_distance: 5.0,
        }
    }
}

impl Default for SpacingSettings {
    fn default() -> Self {
        Self {
            default_spacing: 8.0,
            min_spacing: 0.0,
            max_spacing: 100.0,
            spacing_increment: 4.0,
            container_margin: Vec2::new(16.0, 16.0),
        }
    }
}

impl AdvancedAlignment {
    /// Create new advanced alignment system
    pub fn new() -> Self {
        Self::default()
    }

    /// Create default alignment tools
    fn create_default_tools() -> Vec<AlignmentTool> {
        vec![
            // Horizontal Alignment
            AlignmentTool {
                name: "Align Left".to_string(),
                icon: "◀".to_string(),
                tooltip: "Align selected components to the left".to_string(),
                operation: AlignmentOperation::AlignLeft,
                shortcut: Some("Ctrl+Shift+L".to_string()),
                enabled: true,
            },
            AlignmentTool {
                name: "Align Center Horizontal".to_string(),
                icon: "▐".to_string(),
                tooltip: "Align selected components to horizontal center".to_string(),
                operation: AlignmentOperation::AlignCenterHorizontal,
                shortcut: Some("Ctrl+Shift+C".to_string()),
                enabled: true,
            },
            AlignmentTool {
                name: "Align Right".to_string(),
                icon: "▶".to_string(),
                tooltip: "Align selected components to the right".to_string(),
                operation: AlignmentOperation::AlignRight,
                shortcut: Some("Ctrl+Shift+R".to_string()),
                enabled: true,
            },
            
            // Vertical Alignment
            AlignmentTool {
                name: "Align Top".to_string(),
                icon: "▲".to_string(),
                tooltip: "Align selected components to the top".to_string(),
                operation: AlignmentOperation::AlignTop,
                shortcut: Some("Ctrl+Shift+T".to_string()),
                enabled: true,
            },
            AlignmentTool {
                name: "Align Center Vertical".to_string(),
                icon: "▬".to_string(),
                tooltip: "Align selected components to vertical center".to_string(),
                operation: AlignmentOperation::AlignCenterVertical,
                shortcut: Some("Ctrl+Shift+M".to_string()),
                enabled: true,
            },
            AlignmentTool {
                name: "Align Bottom".to_string(),
                icon: "▼".to_string(),
                tooltip: "Align selected components to the bottom".to_string(),
                operation: AlignmentOperation::AlignBottom,
                shortcut: Some("Ctrl+Shift+B".to_string()),
                enabled: true,
            },
            
            // Distribution
            AlignmentTool {
                name: "Distribute Horizontally".to_string(),
                icon: "⟷".to_string(),
                tooltip: "Distribute selected components horizontally".to_string(),
                operation: AlignmentOperation::DistributeHorizontally,
                shortcut: Some("Ctrl+Shift+H".to_string()),
                enabled: true,
            },
            AlignmentTool {
                name: "Distribute Vertically".to_string(),
                icon: "↕".to_string(),
                tooltip: "Distribute selected components vertically".to_string(),
                operation: AlignmentOperation::DistributeVertically,
                shortcut: Some("Ctrl+Shift+V".to_string()),
                enabled: true,
            },
            
            // Sizing
            AlignmentTool {
                name: "Same Width".to_string(),
                icon: "═".to_string(),
                tooltip: "Make selected components the same width".to_string(),
                operation: AlignmentOperation::SameWidth,
                shortcut: Some("Ctrl+Shift+W".to_string()),
                enabled: true,
            },
            AlignmentTool {
                name: "Same Height".to_string(),
                icon: "║".to_string(),
                tooltip: "Make selected components the same height".to_string(),
                operation: AlignmentOperation::SameHeight,
                shortcut: Some("Ctrl+Shift+E".to_string()),
                enabled: true,
            },
            AlignmentTool {
                name: "Center in Parent".to_string(),
                icon: "⊞".to_string(),
                tooltip: "Center selected components in container".to_string(),
                operation: AlignmentOperation::CenterInParent,
                shortcut: Some("Ctrl+Alt+C".to_string()),
                enabled: true,
            },
        ]
    }

    /// Render alignment toolbar
    pub fn render_toolbar(&mut self, ui: &mut Ui, selected_count: usize) {
        if !self.show_toolbar {
            return;
        }

        ui.horizontal(|ui| {
            ui.label("Align:");
            
            // Enable/disable tools based on selection count
            let can_align = selected_count >= 2;
            let can_distribute = selected_count >= 3;
            let can_size = selected_count >= 2;
            
            for tool in &self.operations {
                let enabled = match tool.operation {
                    AlignmentOperation::DistributeHorizontally | 
                    AlignmentOperation::DistributeVertically |
                    AlignmentOperation::DistributeSpaceHorizontally |
                    AlignmentOperation::DistributeSpaceVertically => can_distribute,
                    AlignmentOperation::SameWidth |
                    AlignmentOperation::SameHeight |
                    AlignmentOperation::SameSize => can_size,
                    AlignmentOperation::CenterInParent => selected_count >= 1,
                    _ => can_align,
                };

                ui.add_enabled_ui(enabled, |ui| {
                    if ui.small_button(&tool.icon)
                        .on_hover_text(&tool.tooltip)
                        .clicked() {
                        self.recent_operations.push(tool.operation.clone());
                        // Keep only last 10 recent operations
                        if self.recent_operations.len() > 10 {
                            self.recent_operations.remove(0);
                        }
                    }
                });
            }
            
            ui.separator();
            
            // Quick spacing controls
            if ui.small_button("➕").on_hover_text("Increase spacing").clicked() {
                self.recent_operations.push(AlignmentOperation::IncreaseSpacing);
            }
            if ui.small_button("➖").on_hover_text("Decrease spacing").clicked() {
                self.recent_operations.push(AlignmentOperation::DecreaseSpacing);
            }
        });
    }

    /// Apply alignment operation to selected components
    pub fn apply_operation(
        &mut self, 
        operation: &AlignmentOperation, 
        components: &mut [ComponentBounds],
        container_rect: Rect
    ) -> bool {
        if components.is_empty() {
            return false;
        }

        match operation {
            AlignmentOperation::AlignLeft => self.align_left(components),
            AlignmentOperation::AlignRight => self.align_right(components),
            AlignmentOperation::AlignCenterHorizontal => self.align_center_horizontal(components),
            AlignmentOperation::AlignTop => self.align_top(components),
            AlignmentOperation::AlignBottom => self.align_bottom(components),
            AlignmentOperation::AlignCenterVertical => self.align_center_vertical(components),
            AlignmentOperation::DistributeHorizontally => self.distribute_horizontally(components),
            AlignmentOperation::DistributeVertically => self.distribute_vertically(components),
            AlignmentOperation::SameWidth => self.make_same_width(components),
            AlignmentOperation::SameHeight => self.make_same_height(components),
            AlignmentOperation::SameSize => self.make_same_size(components),
            AlignmentOperation::CenterInParent => self.center_in_container(components, container_rect),
            AlignmentOperation::SpaceEvenly => self.space_evenly(components),
            AlignmentOperation::IncreaseSpacing => self.adjust_spacing(components, self.spacing.spacing_increment),
            AlignmentOperation::DecreaseSpacing => self.adjust_spacing(components, -self.spacing.spacing_increment),
            _ => return false, // Operation not implemented yet
        }
        
        true
    }

    /// Align components to the left
    fn align_left(&self, components: &mut [ComponentBounds]) {
        if let Some(leftmost) = components.iter().map(|c| c.position.x).min_by(|a, b| a.partial_cmp(b).unwrap()) {
            for component in components {
                component.position.x = leftmost;
            }
        }
    }

    /// Align components to the right
    fn align_right(&self, components: &mut [ComponentBounds]) {
        if let Some(rightmost) = components.iter().map(|c| c.position.x + c.size.x).max_by(|a, b| a.partial_cmp(b).unwrap()) {
            for component in components {
                component.position.x = rightmost - component.size.x;
            }
        }
    }

    /// Align components to horizontal center
    fn align_center_horizontal(&self, components: &mut [ComponentBounds]) {
        if components.is_empty() { return; }
        
        // Find the center of all components
        let total_center: f32 = components.iter()
            .map(|c| c.position.x + c.size.x / 2.0)
            .sum::<f32>() / components.len() as f32;
            
        for component in components {
            component.position.x = total_center - component.size.x / 2.0;
        }
    }

    /// Align components to the top
    fn align_top(&self, components: &mut [ComponentBounds]) {
        if let Some(topmost) = components.iter().map(|c| c.position.y).min_by(|a, b| a.partial_cmp(b).unwrap()) {
            for component in components {
                component.position.y = topmost;
            }
        }
    }

    /// Align components to the bottom
    fn align_bottom(&self, components: &mut [ComponentBounds]) {
        if let Some(bottommost) = components.iter().map(|c| c.position.y + c.size.y).max_by(|a, b| a.partial_cmp(b).unwrap()) {
            for component in components {
                component.position.y = bottommost - component.size.y;
            }
        }
    }

    /// Align components to vertical center
    fn align_center_vertical(&self, components: &mut [ComponentBounds]) {
        if components.is_empty() { return; }
        
        // Find the center of all components
        let total_center: f32 = components.iter()
            .map(|c| c.position.y + c.size.y / 2.0)
            .sum::<f32>() / components.len() as f32;
            
        for component in components {
            component.position.y = total_center - component.size.y / 2.0;
        }
    }

    /// Distribute components horizontally
    /// 
    /// This algorithm implements professional horizontal distribution similar to Adobe Illustrator
    /// or Figma. It evenly spaces components between the leftmost and rightmost components,
    /// calculating optimal spacing while preserving the overall layout bounds.
    fn distribute_horizontally(&self, components: &mut [ComponentBounds]) {
        // Need at least 3 components for distribution to make sense
        // With 2 components, there's only one gap, so distribution is meaningless
        if components.len() < 3 { return; }

        // Sort components by their left edge position (x coordinate)
        // This ensures we process them from left to right in the correct order
        components.sort_by(|a, b| a.position.x.partial_cmp(&b.position.x).unwrap());
        
        // Calculate the total distribution area boundaries
        // Leftmost edge: x position of the first component
        let leftmost = components[0].position.x;
        // Rightmost edge: x position + width of the last component
        let rightmost = components.last().unwrap().position.x + components.last().unwrap().size.x;
        // Total area we have to work with
        let total_width = rightmost - leftmost;
        
        // Calculate how much space is taken up by the components themselves
        // This is the sum of all component widths
        let total_component_width: f32 = components.iter().map(|c| c.size.x).sum();
        
        // Calculate available space for gaps between components
        // This is the space we need to distribute evenly
        let available_space = total_width - total_component_width;
        
        // Calculate equal spacing between components
        // We have (n-1) gaps between n components, so divide available space accordingly
        let spacing = available_space / (components.len() - 1) as f32;
        
        // Position each component with calculated equal spacing
        let mut current_x = leftmost;  // Start at the leftmost position
        for component in components {
            // Place component at current position
            component.position.x = current_x;
            // Move to next position: current component width + calculated spacing
            current_x += component.size.x + spacing;
        }
    }

    /// Distribute components vertically
    /// 
    /// Vertical distribution algorithm - mirrors the horizontal logic but works on Y-axis.
    /// This creates evenly spaced components from top to bottom, maintaining the overall
    /// vertical bounds while calculating optimal spacing between components.
    fn distribute_vertically(&self, components: &mut [ComponentBounds]) {
        // Need at least 3 components for meaningful vertical distribution
        if components.len() < 3 { return; }

        // Sort components by their top edge position (y coordinate)
        // This ensures we process them from top to bottom in the correct order
        components.sort_by(|a, b| a.position.y.partial_cmp(&b.position.y).unwrap());
        
        // Calculate the total vertical distribution area boundaries
        // Topmost edge: y position of the first (highest) component
        let topmost = components[0].position.y;
        // Bottommost edge: y position + height of the last (lowest) component
        let bottommost = components.last().unwrap().position.y + components.last().unwrap().size.y;
        // Total vertical area we have to work with
        let total_height = bottommost - topmost;
        
        // Calculate total space occupied by component heights
        // This is the sum of all component heights
        let total_component_height: f32 = components.iter().map(|c| c.size.y).sum();
        
        // Calculate available vertical space for gaps between components
        // This space will be divided evenly between all gaps
        let available_space = total_height - total_component_height;
        
        // Calculate equal vertical spacing between components
        // With n components, we have (n-1) gaps to fill with equal spacing
        let spacing = available_space / (components.len() - 1) as f32;
        
        // Position each component with calculated equal vertical spacing
        let mut current_y = topmost;  // Start at the topmost position
        for component in components {
            // Place component at current vertical position
            component.position.y = current_y;
            // Move to next vertical position: current component height + calculated spacing
            current_y += component.size.y + spacing;
        }
    }

    /// Make components the same width
    fn make_same_width(&self, components: &mut [ComponentBounds]) {
        if let Some(max_width) = components.iter().map(|c| c.size.x).max_by(|a, b| a.partial_cmp(b).unwrap()) {
            for component in components {
                component.size.x = max_width;
            }
        }
    }

    /// Make components the same height
    fn make_same_height(&self, components: &mut [ComponentBounds]) {
        if let Some(max_height) = components.iter().map(|c| c.size.y).max_by(|a, b| a.partial_cmp(b).unwrap()) {
            for component in components {
                component.size.y = max_height;
            }
        }
    }

    /// Make components the same size
    fn make_same_size(&self, components: &mut [ComponentBounds]) {
        self.make_same_width(components);
        self.make_same_height(components);
    }

    /// Center components in container
    fn center_in_container(&self, components: &mut [ComponentBounds], container_rect: Rect) {
        for component in components {
            component.position.x = container_rect.center().x - component.size.x / 2.0;
            component.position.y = container_rect.center().y - component.size.y / 2.0;
        }
    }

    /// Space components evenly
    fn space_evenly(&self, components: &mut [ComponentBounds]) {
        if components.len() < 2 { return; }
        
        for i in 1..components.len() {
            let spacing = self.spacing.default_spacing;
            components[i].position.x = components[i-1].position.x + components[i-1].size.x + spacing;
        }
    }

    /// Adjust spacing between components
    /// 
    /// This algorithm implements incremental spacing adjustment - allowing users to fine-tune
    /// the spacing between components by a specific amount. It maintains the relative order
    /// of components while adjusting gaps, with built-in protection against negative spacing.
    fn adjust_spacing(&self, components: &mut [ComponentBounds], adjustment: f32) {
        // Need at least 2 components to have any spacing to adjust
        if components.len() < 2 { return; }
        
        // Create index array for sorting without modifying original component order
        // This preserves component relationships while allowing spatial sorting
        let mut sorted_indices: Vec<usize> = (0..components.len()).collect();
        // Sort indices by component X position (left to right)
        sorted_indices.sort_by(|&a, &b| components[a].position.x.partial_cmp(&components[b].position.x).unwrap());
        
        // Process each component pair from left to right
        for i in 1..sorted_indices.len() {
            let prev_idx = sorted_indices[i-1];  // Previous component index
            let curr_idx = sorted_indices[i];    // Current component index
            
            // Calculate current spacing between these two components
            // Spacing = left edge of current - right edge of previous
            let current_spacing = components[curr_idx].position.x - 
                                 (components[prev_idx].position.x + components[prev_idx].size.x);
            
            // Apply the adjustment while respecting minimum spacing constraints
            // This prevents components from overlapping or having negative gaps
            let new_spacing = (current_spacing + adjustment).max(self.spacing.min_spacing);
            
            // Reposition the current component to achieve the new spacing
            // New position = previous component's right edge + desired spacing
            components[curr_idx].position.x = components[prev_idx].position.x + 
                                             components[prev_idx].size.x + new_spacing;
        }
    }

    /// Draw alignment guides
    pub fn draw_guides(&self, ui: &mut Ui, components: &[ComponentBounds], canvas_rect: Rect) {
        if !self.guides.enabled || components.is_empty() {
            return;
        }

        let painter = ui.painter();
        let stroke = Stroke::new(self.guides.line_thickness, self.guides.guide_color);

        // Draw center guides
        if self.guides.show_center_guides {
            let center_x = canvas_rect.center().x;
            let center_y = canvas_rect.center().y;
            
            // Vertical center line
            painter.line_segment([
                Pos2::new(center_x, canvas_rect.top()),
                Pos2::new(center_x, canvas_rect.bottom())
            ], stroke);
            
            // Horizontal center line
            painter.line_segment([
                Pos2::new(canvas_rect.left(), center_y),
                Pos2::new(canvas_rect.right(), center_y)
            ], stroke);
        }

        // Draw margin guides
        if self.guides.show_margins {
            let margin = self.spacing.container_margin;
            let margin_rect = Rect::from_min_size(
                canvas_rect.min + margin,
                canvas_rect.size() - 2.0 * margin
            );
            
            painter.rect_stroke(margin_rect, Rounding::ZERO, stroke);
        }
    }

    /// Get most recent alignment operations
    pub fn get_recent_operations(&self) -> &[AlignmentOperation] {
        &self.recent_operations
    }

    /// Clear recent operations
    pub fn clear_recent_operations(&mut self) {
        self.recent_operations.clear();
    }

    /// Check if alignment guides should snap
    pub fn should_snap_to_guide(&self, position: Pos2, guide_position: f32, is_horizontal: bool) -> bool {
        let distance = if is_horizontal {
            (position.y - guide_position).abs()
        } else {
            (position.x - guide_position).abs()
        };
        
        distance <= self.guides.snap_distance
    }

    /// Snap position to nearest guide
    /// 
    /// This algorithm implements "magnetic" guide snapping that provides visual assistance
    /// during component positioning. It checks proximity to various guide lines and automatically
    /// snaps positions when they're close enough, similar to professional design tools.
    pub fn snap_to_guides(&self, mut position: Pos2, canvas_rect: Rect) -> Pos2 {
        // Early exit if guide snapping is disabled
        // This optimization avoids unnecessary calculations when guides aren't active
        if !self.guides.enabled {
            return position;
        }

        // Calculate key reference points for guide snapping
        let center_x = canvas_rect.center().x;  // Horizontal center line
        let center_y = canvas_rect.center().y;  // Vertical center line
        let margin = self.spacing.container_margin;  // Margin from edges

        // Snap to center guides - these provide primary alignment references
        if self.guides.show_center_guides {
            // Check horizontal center line snapping (vertical guide)
            // If position is close enough to center X, snap to it
            if self.should_snap_to_guide(position, center_x, false) {
                position.x = center_x;
            }
            // Check vertical center line snapping (horizontal guide)
            // If position is close enough to center Y, snap to it
            if self.should_snap_to_guide(position, center_y, true) {
                position.y = center_y;
            }
        }

        // Snap to margin guides - these help maintain consistent spacing from edges
        if self.guides.show_margins {
            // Left margin guide - snap to left edge + margin
            if self.should_snap_to_guide(position, canvas_rect.left() + margin.x, false) {
                position.x = canvas_rect.left() + margin.x;
            }
            // Right margin guide - snap to right edge - margin
            if self.should_snap_to_guide(position, canvas_rect.right() - margin.x, false) {
                position.x = canvas_rect.right() - margin.x;
            }
            // Top margin guide - snap to top edge + margin
            if self.should_snap_to_guide(position, canvas_rect.top() + margin.y, true) {
                position.y = canvas_rect.top() + margin.y;
            }
            // Bottom margin guide - snap to bottom edge - margin
            if self.should_snap_to_guide(position, canvas_rect.bottom() - margin.y, true) {
                position.y = canvas_rect.bottom() - margin.y;
            }
        }

        // Return the potentially snapped position
        position
    }
}