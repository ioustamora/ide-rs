//! Advanced Data Grid Component
//!
//! High-performance data grid with sorting, filtering, grouping, editing, and virtualization.
//! Supports large datasets with customizable column types and cell renderers.

use egui::*;
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use serde::{Serialize, Deserialize};

/// Advanced data grid component
pub struct DataGrid<T: Clone> {
    /// Grid data
    pub data: Vec<T>,
    /// Column definitions
    pub columns: Vec<ColumnDefinition<T>>,
    /// Grid state
    pub state: DataGridState,
    /// Grid configuration
    pub config: DataGridConfig,
    /// Selection manager
    pub selection: SelectionManager,
    /// Filter manager
    pub filters: FilterManager<T>,
    /// Sort manager
    pub sorting: SortManager<T>,
    /// Edit manager
    pub editing: EditManager<T>,
    /// Virtualization settings
    pub virtualization: VirtualizationConfig,
    /// Styling
    pub style: DataGridStyle,
}

/// Column definition for the data grid
pub struct ColumnDefinition<T> {
    /// Column identifier
    pub id: String,
    /// Column display title
    pub title: String,
    /// Column width configuration
    pub width: ColumnWidth,
    /// Whether column is sortable
    pub sortable: bool,
    /// Whether column is filterable
    pub filterable: bool,
    /// Whether column is editable
    pub editable: bool,
    /// Whether column is resizable
    pub resizable: bool,
    /// Whether column can be hidden
    pub hideable: bool,
    /// Whether column is currently visible
    pub visible: bool,
    /// Data accessor function
    pub accessor: Box<dyn Fn(&T) -> CellValue>,
    /// Custom cell renderer
    pub renderer: Option<Box<dyn Fn(&mut Ui, &T, &CellValue, &CellContext) -> Response>>,
    /// Cell editor
    pub editor: Option<Box<dyn Fn(&mut Ui, &mut T, &CellValue) -> (CellValue, bool)>>,
    /// Column validator
    pub validator: Option<Box<dyn Fn(&CellValue) -> ValidationResult>>,
    /// Column aggregator (for grouping)
    pub aggregator: Option<ColumnAggregator>,
    /// Minimum column width
    pub min_width: f32,
    /// Maximum column width
    pub max_width: f32,
}

/// Column width configuration
#[derive(Clone)]
pub enum ColumnWidth {
    /// Fixed width in pixels
    Fixed(f32),
    /// Percentage of available width
    Percentage(f32),
    /// Flexible width with weight
    Flex(f32),
    /// Auto-size based on content
    Auto,
    /// Minimum content width
    MinContent,
    /// Maximum content width
    MaxContent,
}

/// Cell value types
#[derive(Clone, Debug, PartialEq)]
pub enum CellValue {
    Text(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),
    Date(String), // ISO date string
    Time(String), // ISO time string
    DateTime(String), // ISO datetime string
    Currency(f64, String), // Amount and currency code
    Percentage(f64),
    Link(String, String), // URL and display text
    Image(String), // Image URL or base64
    Json(String), // JSON string
    Custom(String), // Custom serialized data
    Null,
}

/// Context information for cell rendering
pub struct CellContext {
    pub row_index: usize,
    pub column_id: String,
    pub is_selected: bool,
    pub is_editing: bool,
    pub is_hovered: bool,
    pub validation_state: Option<ValidationResult>,
}

/// Validation result for cell values
#[derive(Clone, Debug)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub error_message: Option<String>,
    pub warning_message: Option<String>,
}

/// Column aggregator for grouping
pub enum ColumnAggregator {
    Count,
    Sum,
    Average,
    Min,
    Max,
    Custom(Box<dyn Fn(&[CellValue]) -> CellValue>),
}

/// Data grid state
#[derive(Default)]
pub struct DataGridState {
    /// Currently visible rows (after filtering/sorting)
    pub visible_rows: Vec<usize>,
    /// Total row count
    pub total_rows: usize,
    /// Current scroll position
    pub scroll_position: f32,
    /// Viewport dimensions
    pub viewport_size: Vec2,
    /// Column widths (calculated)
    pub column_widths: HashMap<String, f32>,
    /// Column positions
    pub column_positions: HashMap<String, f32>,
    /// Row heights (for dynamic height)
    pub row_heights: HashMap<usize, f32>,
    /// Grid is dirty and needs refresh
    pub dirty: bool,
}

/// Data grid configuration
#[derive(Clone)]
pub struct DataGridConfig {
    /// Default row height
    pub row_height: f32,
    /// Enable dynamic row heights
    pub dynamic_row_heights: bool,
    /// Show header row
    pub show_header: bool,
    /// Show footer row (for aggregations)
    pub show_footer: bool,
    /// Enable row numbers column
    pub show_row_numbers: bool,
    /// Enable horizontal scrolling
    pub horizontal_scrolling: bool,
    /// Enable vertical scrolling
    pub vertical_scrolling: bool,
    /// Minimum column width
    pub min_column_width: f32,
    /// Maximum column width
    pub max_column_width: f32,
    /// Grid border width
    pub border_width: f32,
    /// Enable zebra striping
    pub zebra_striping: bool,
    /// Enable hover highlighting
    pub hover_highlighting: bool,
    /// Enable keyboard navigation
    pub keyboard_navigation: bool,
    /// Page size for pagination
    pub page_size: Option<usize>,
}

/// Selection manager
#[derive(Default)]
pub struct SelectionManager {
    /// Selected row indices
    pub selected_rows: HashSet<usize>,
    /// Selected cell coordinates
    pub selected_cells: HashSet<(usize, String)>,
    /// Selection mode
    pub selection_mode: SelectionMode,
    /// Last selected for range selection
    pub anchor_selection: Option<usize>,
    /// Selection callbacks
    pub on_selection_changed: Option<Box<dyn Fn(&HashSet<usize>)>>,
}

/// Selection modes
#[derive(Clone, PartialEq, Default)]
pub enum SelectionMode {
    #[default]
    None,
    SingleRow,
    MultipleRows,
    SingleCell,
    MultipleCells,
    RowAndCell,
}

/// Filter manager
pub struct FilterManager<T> {
    /// Active filters by column
    pub filters: HashMap<String, ColumnFilter>,
    /// Global search filter
    pub global_filter: Option<String>,
    /// Quick filters
    pub quick_filters: Vec<QuickFilter<T>>,
    /// Filter logic (AND/OR)
    pub filter_logic: FilterLogic,
    /// Filtered row indices
    pub filtered_rows: Vec<usize>,
    /// Filter is dirty
    pub dirty: bool,
}

/// Column filter types
pub enum ColumnFilter {
    Text { 
        value: String, 
        mode: TextFilterMode 
    },
    Number { 
        min: Option<f64>, 
        max: Option<f64>, 
        exact: Option<f64> 
    },
    Date { 
        from: Option<String>, 
        to: Option<String> 
    },
    Boolean { 
        value: Option<bool> 
    },
    List { 
        selected_values: HashSet<String> 
    },
    Custom { 
        predicate: Box<dyn Fn(&CellValue) -> bool> 
    },
}

#[derive(Clone)]
pub enum TextFilterMode {
    Contains,
    StartsWith,
    EndsWith,
    Equals,
    Regex,
}

/// Quick filter for common operations
pub struct QuickFilter<T> {
    pub name: String,
    pub predicate: Box<dyn Fn(&T) -> bool>,
    pub active: bool,
}

#[derive(Clone)]
pub enum FilterLogic {
    And,
    Or,
}

/// Sort manager
pub struct SortManager<T> {
    /// Current sort configuration
    pub sort_columns: Vec<SortColumn>,
    /// Sort is dirty
    pub dirty: bool,
    /// Custom comparators
    pub comparators: HashMap<String, Box<dyn Fn(&T, &T) -> Ordering>>,
}

#[derive(Clone)]
pub struct SortColumn {
    pub column_id: String,
    pub direction: SortDirection,
    pub priority: usize,
}

#[derive(Clone, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

/// Edit manager
pub struct EditManager<T> {
    /// Type marker
    _phantom: std::marker::PhantomData<T>,
    /// Currently editing cell
    pub editing_cell: Option<(usize, String)>,
    /// Pending changes
    pub pending_changes: HashMap<(usize, String), CellValue>,
    /// Validation errors
    pub validation_errors: HashMap<(usize, String), ValidationResult>,
    /// Edit mode
    pub edit_mode: EditMode,
    /// Auto-save changes
    pub auto_save: bool,
}

#[derive(Clone, PartialEq)]
pub enum EditMode {
    None,
    SingleCell,
    InlineRow,
    Modal,
}

/// Virtualization configuration
#[derive(Clone)]
pub struct VirtualizationConfig {
    /// Enable row virtualization
    pub enable_row_virtualization: bool,
    /// Enable column virtualization
    pub enable_column_virtualization: bool,
    /// Number of overscan rows
    pub row_overscan: usize,
    /// Number of overscan columns
    pub column_overscan: usize,
    /// Estimated row height for calculations
    pub estimated_row_height: f32,
    /// Estimated column width for calculations
    pub estimated_column_width: f32,
}

/// Data grid styling
#[derive(Clone)]
pub struct DataGridStyle {
    /// Header styling
    pub header_style: HeaderStyle,
    /// Cell styling
    pub cell_style: CellStyle,
    /// Row styling
    pub row_style: RowStyle,
    /// Border styling
    pub border_style: BorderStyle,
    /// Color scheme
    pub colors: ColorScheme,
}

#[derive(Clone)]
pub struct HeaderStyle {
    pub background_color: Color32,
    pub text_color: Color32,
    pub font_size: f32,
    pub height: f32,
    pub padding: Margin,
    pub sort_indicator_color: Color32,
    pub filter_indicator_color: Color32,
}

#[derive(Clone)]
pub struct CellStyle {
    pub background_color: Color32,
    pub text_color: Color32,
    pub font_size: f32,
    pub padding: Margin,
    pub alignment: Align,
}

#[derive(Clone)]
pub struct RowStyle {
    pub even_row_color: Color32,
    pub odd_row_color: Color32,
    pub hover_color: Color32,
    pub selected_color: Color32,
    pub height: f32,
}

#[derive(Clone)]
pub struct BorderStyle {
    pub color: Color32,
    pub width: f32,
    pub header_border_color: Color32,
    pub cell_border_color: Color32,
}

#[derive(Clone)]
pub struct ColorScheme {
    pub primary: Color32,
    pub secondary: Color32,
    pub accent: Color32,
    pub success: Color32,
    pub warning: Color32,
    pub error: Color32,
    pub info: Color32,
}

impl<T: Clone> DataGrid<T> {
    /// Create a new data grid
    pub fn new(data: Vec<T>) -> Self {
        Self {
            data,
            columns: Vec::new(),
            state: DataGridState::default(),
            config: DataGridConfig::default(),
            selection: SelectionManager::default(),
            filters: FilterManager::new(),
            sorting: SortManager::new(),
            editing: EditManager::new(),
            virtualization: VirtualizationConfig::default(),
            style: DataGridStyle::default(),
        }
    }
    
    /// Add a column to the grid
    pub fn add_column(mut self, column: ColumnDefinition<T>) -> Self {
        self.columns.push(column);
        self.state.dirty = true;
        self
    }
    
    /// Set selection mode
    pub fn with_selection_mode(mut self, mode: SelectionMode) -> Self {
        self.selection.selection_mode = mode;
        self
    }
    
    /// Enable sorting
    pub fn with_sorting(mut self, enabled: bool) -> Self {
        for column in &mut self.columns {
            column.sortable = enabled;
        }
        self
    }
    
    /// Enable filtering
    pub fn with_filtering(mut self, enabled: bool) -> Self {
        for column in &mut self.columns {
            column.filterable = enabled;
        }
        self
    }
    
    /// Set page size for pagination
    pub fn with_page_size(mut self, page_size: usize) -> Self {
        self.config.page_size = Some(page_size);
        self
    }
    
    /// Update grid data
    pub fn set_data(&mut self, data: Vec<T>) {
        self.data = data;
        self.state.dirty = true;
        self.filters.dirty = true;
        self.sorting.dirty = true;
    }
    
    /// Get selected row data
    pub fn get_selected_data(&self) -> Vec<&T> {
        self.selection.selected_rows
            .iter()
            .filter_map(|&index| self.data.get(index))
            .collect()
    }
    
    /// Apply filters
    fn apply_filters(&mut self) {
        if !self.filters.dirty {
            return;
        }
        
        self.filters.filtered_rows.clear();
        
        for (index, row) in self.data.iter().enumerate() {
            let mut include_row = true;
            
            // Apply column filters
            for (column_id, filter) in &self.filters.filters {
                if let Some(column) = self.columns.iter().find(|c| c.id == *column_id) {
                    let cell_value = (column.accessor)(row);
                    if !Self::apply_column_filter(filter, &cell_value) {
                        include_row = false;
                        break;
                    }
                }
            }
            
            // Apply global filter
            if include_row {
                if let Some(ref global_filter) = self.filters.global_filter {
                    let mut found_match = false;
                    for column in &self.columns {
                        let cell_value = (column.accessor)(row);
                        if Self::cell_value_contains_text(&cell_value, global_filter) {
                            found_match = true;
                            break;
                        }
                    }
                    include_row = found_match;
                }
            }
            
            // Apply quick filters
            if include_row {
                for quick_filter in &self.filters.quick_filters {
                    if quick_filter.active && !(quick_filter.predicate)(row) {
                        include_row = false;
                        break;
                    }
                }
            }
            
            if include_row {
                self.filters.filtered_rows.push(index);
            }
        }
        
        self.filters.dirty = false;
        self.state.dirty = true;
    }
    
    /// Apply column filter
    fn apply_column_filter(filter: &ColumnFilter, value: &CellValue) -> bool {
        match filter {
            ColumnFilter::Text { value: filter_value, mode } => {
                if let CellValue::Text(text) = value {
                    match mode {
                        TextFilterMode::Contains => text.to_lowercase().contains(&filter_value.to_lowercase()),
                        TextFilterMode::StartsWith => text.to_lowercase().starts_with(&filter_value.to_lowercase()),
                        TextFilterMode::EndsWith => text.to_lowercase().ends_with(&filter_value.to_lowercase()),
                        TextFilterMode::Equals => text.to_lowercase() == filter_value.to_lowercase(),
                        TextFilterMode::Regex => {
                            // Would implement regex matching here
                            text.contains(filter_value)
                        }
                    }
                } else {
                    false
                }
            }
            ColumnFilter::Number { min, max, exact } => {
                let num_value = match value {
                    CellValue::Number(n) => Some(*n),
                    CellValue::Integer(i) => Some(*i as f64),
                    CellValue::Currency(amount, _) => Some(*amount),
                    CellValue::Percentage(p) => Some(*p),
                    _ => None,
                };
                
                if let Some(num) = num_value {
                    if let Some(exact_val) = exact {
                        (num - exact_val).abs() < f64::EPSILON
                    } else {
                        let min_check = min.map_or(true, |m| num >= m);
                        let max_check = max.map_or(true, |m| num <= m);
                        min_check && max_check
                    }
                } else {
                    false
                }
            }
            ColumnFilter::Boolean { value: filter_value } => {
                if let CellValue::Boolean(b) = value {
                    filter_value.map_or(true, |fv| *b == fv)
                } else {
                    false
                }
            }
            ColumnFilter::List { selected_values } => {
                let string_value = Self::cell_value_to_string(value);
                selected_values.contains(&string_value)
            }
            ColumnFilter::Date { from, to } => {
                // Would implement date range filtering here
                true
            }
            ColumnFilter::Custom { predicate } => {
                predicate(value)
            }
        }
    }
    
    /// Check if cell value contains text
    fn cell_value_contains_text(value: &CellValue, text: &str) -> bool {
        let cell_text = Self::cell_value_to_string(value);
        cell_text.to_lowercase().contains(&text.to_lowercase())
    }
    
    /// Convert cell value to string
    fn cell_value_to_string(value: &CellValue) -> String {
        match value {
            CellValue::Text(s) => s.clone(),
            CellValue::Number(n) => n.to_string(),
            CellValue::Integer(i) => i.to_string(),
            CellValue::Boolean(b) => b.to_string(),
            CellValue::Date(d) => d.clone(),
            CellValue::Time(t) => t.clone(),
            CellValue::DateTime(dt) => dt.clone(),
            CellValue::Currency(amount, currency) => format!("{} {}", amount, currency),
            CellValue::Percentage(p) => format!("{}%", p * 100.0),
            CellValue::Link(_, text) => text.clone(),
            CellValue::Json(json) => json.clone(),
            CellValue::Custom(data) => data.clone(),
            CellValue::Image(_) => "[Image]".to_string(),
            CellValue::Null => "".to_string(),
        }
    }
    
    /// Apply sorting
    fn apply_sorting(&mut self) {
        if !self.sorting.dirty || self.sorting.sort_columns.is_empty() {
            return;
        }
        
        self.state.visible_rows.sort_by(|&a, &b| {
            for sort_column in &self.sorting.sort_columns {
                if let Some(column) = self.columns.iter().find(|c| c.id == sort_column.column_id) {
                    let value_a = (column.accessor)(&self.data[a]);
                    let value_b = (column.accessor)(&self.data[b]);
                    
                    let ordering = Self::compare_cell_values(&value_a, &value_b);
                    let final_ordering = match sort_column.direction {
                        SortDirection::Ascending => ordering,
                        SortDirection::Descending => ordering.reverse(),
                    };
                    
                    if final_ordering != Ordering::Equal {
                        return final_ordering;
                    }
                }
            }
            Ordering::Equal
        });
        
        self.sorting.dirty = false;
    }
    
    /// Compare cell values for sorting
    fn compare_cell_values(a: &CellValue, b: &CellValue) -> Ordering {
        match (a, b) {
            (CellValue::Text(a), CellValue::Text(b)) => a.cmp(b),
            (CellValue::Number(a), CellValue::Number(b)) => a.partial_cmp(b).unwrap_or(Ordering::Equal),
            (CellValue::Integer(a), CellValue::Integer(b)) => a.cmp(b),
            (CellValue::Boolean(a), CellValue::Boolean(b)) => a.cmp(b),
            (CellValue::Date(a), CellValue::Date(b)) => a.cmp(b),
            _ => Ordering::Equal,
        }
    }
    
    /// Render the data grid
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        // Update grid state
        self.apply_filters();
        self.update_visible_rows();
        self.apply_sorting();
        
        let available_rect = ui.available_rect_before_wrap();
        self.state.viewport_size = available_rect.size();
        
        let scroll_output = ScrollArea::both()
            .show(ui, |ui| {
                self.render_header(ui);
                self.render_rows(ui);
                self.render_footer(ui);
            });

        // Create a response from the scroll area
        ui.allocate_rect(scroll_output.inner_rect, Sense::hover())
    }
    
    /// Update visible rows based on filters
    fn update_visible_rows(&mut self) {
        if self.state.dirty {
            self.state.visible_rows = if self.filters.filtered_rows.is_empty() {
                (0..self.data.len()).collect()
            } else {
                self.filters.filtered_rows.clone()
            };
            self.state.total_rows = self.state.visible_rows.len();
            self.state.dirty = false;
        }
    }
    
    /// Render grid header
    fn render_header(&mut self, ui: &mut Ui) {
        if !self.config.show_header {
            return;
        }
        
        ui.horizontal(|ui| {
            ui.set_height(self.style.header_style.height);
            
            for column in &self.columns {
                if !column.visible {
                    continue;
                }
                
                let column_width = self.get_column_width(&column.id);
                let header_rect = Rect::from_min_size(
                    ui.cursor().min,
                    vec2(column_width, self.style.header_style.height),
                );
                
                // Draw header background
                ui.painter().rect_filled(
                    header_rect,
                    0.0,
                    self.style.header_style.background_color,
                );
                
                // Draw header content
                let mut header_ui = ui.child_ui(header_rect, Layout::left_to_right(Align::Center));
                header_ui.add_space(self.style.header_style.padding.left);
                
                // Column title
                header_ui.label(
                    RichText::new(&column.title)
                        .size(self.style.header_style.font_size)
                        .color(self.style.header_style.text_color)
                );
                
                // Sort indicator
                if column.sortable {
                    if let Some(sort_column) = self.sorting.sort_columns.iter().find(|sc| sc.column_id == column.id) {
                        let sort_icon = match sort_column.direction {
                            SortDirection::Ascending => "▲",
                            SortDirection::Descending => "▼",
                        };
                        header_ui.label(
                            RichText::new(sort_icon)
                                .color(self.style.header_style.sort_indicator_color)
                        );
                    }
                }
                
                // Filter indicator
                if column.filterable && self.filters.filters.contains_key(&column.id) {
                    header_ui.label(
                        RichText::new("🔍")
                            .color(self.style.header_style.filter_indicator_color)
                    );
                }
                
                ui.advance_cursor_after_rect(header_rect);
            }
        });
    }
    
    /// Render grid rows
    fn render_rows(&mut self, ui: &mut Ui) {
        let visible_range = self.calculate_visible_row_range();
        
        let visible_indices: Vec<usize> = self.state.visible_rows[visible_range].iter().copied().collect();
        for row_index in visible_indices {
            if let Some(row_data) = self.data.get(row_index).cloned() {
                self.render_row(ui, row_index, &row_data);
            }
        }
    }
    
    /// Calculate visible row range for virtualization
    fn calculate_visible_row_range(&self) -> std::ops::Range<usize> {
        if !self.virtualization.enable_row_virtualization {
            return 0..self.state.visible_rows.len();
        }
        
        let row_height = self.config.row_height;
        let start_row = (self.state.scroll_position / row_height) as usize;
        let visible_rows = (self.state.viewport_size.y / row_height).ceil() as usize + 1;
        
        let start_with_overscan = start_row.saturating_sub(self.virtualization.row_overscan);
        let end_with_overscan = (start_row + visible_rows + self.virtualization.row_overscan)
            .min(self.state.visible_rows.len());
        
        start_with_overscan..end_with_overscan
    }
    
    /// Render a single row
    fn render_row(&mut self, ui: &mut Ui, row_index: usize, row_data: &T) {
        let is_selected = self.selection.selected_rows.contains(&row_index);
        let is_even = row_index % 2 == 0;
        
        let row_color = if is_selected {
            self.style.row_style.selected_color
        } else if is_even {
            self.style.row_style.even_row_color
        } else {
            self.style.row_style.odd_row_color
        };
        
        ui.horizontal(|ui| {
            ui.set_height(self.config.row_height);
            
            for column in &self.columns {
                if !column.visible {
                    continue;
                }
                
                let column_width = self.get_column_width(&column.id);
                let cell_rect = Rect::from_min_size(
                    ui.cursor().min,
                    vec2(column_width, self.config.row_height),
                );
                
                // Draw cell background
                ui.painter().rect_filled(cell_rect, 0.0, row_color);
                
                // Draw cell content
                let cell_value = (column.accessor)(row_data);
                let cell_context = CellContext {
                    row_index,
                    column_id: column.id.clone(),
                    is_selected,
                    is_editing: false, // Would check editing state
                    is_hovered: false, // Would check hover state
                    validation_state: None,
                };
                
                let mut cell_ui = ui.child_ui(cell_rect, Layout::left_to_right(Align::Center));
                
                if let Some(ref renderer) = column.renderer {
                    renderer(&mut cell_ui, row_data, &cell_value, &cell_context);
                } else {
                    self.render_default_cell(&mut cell_ui, &cell_value);
                }
                
                ui.advance_cursor_after_rect(cell_rect);
            }
        });
    }
    
    /// Render default cell content
    fn render_default_cell(&self, ui: &mut Ui, value: &CellValue) {
        ui.add_space(self.style.cell_style.padding.left);
        
        match value {
            CellValue::Text(text) => {
                ui.label(
                    RichText::new(text)
                        .size(self.style.cell_style.font_size)
                        .color(self.style.cell_style.text_color)
                );
            }
            CellValue::Number(n) => {
                ui.label(
                    RichText::new(format!("{:.2}", n))
                        .size(self.style.cell_style.font_size)
                        .color(self.style.cell_style.text_color)
                );
            }
            CellValue::Boolean(b) => {
                ui.checkbox(&mut b.clone(), "");
            }
            CellValue::Currency(amount, currency) => {
                ui.label(
                    RichText::new(format!("{:.2} {}", amount, currency))
                        .size(self.style.cell_style.font_size)
                        .color(self.style.cell_style.text_color)
                );
            }
            _ => {
                let text = Self::cell_value_to_string(value);
                ui.label(
                    RichText::new(text)
                        .size(self.style.cell_style.font_size)
                        .color(self.style.cell_style.text_color)
                );
            }
        }
    }
    
    /// Render grid footer
    fn render_footer(&mut self, ui: &mut Ui) {
        if !self.config.show_footer {
            return;
        }
        
        ui.horizontal(|ui| {
            ui.label(format!("Total rows: {}", self.state.total_rows));
            ui.separator();
            ui.label(format!("Selected: {}", self.selection.selected_rows.len()));
        });
    }
    
    /// Get calculated column width
    fn get_column_width(&self, column_id: &str) -> f32 {
        self.state.column_widths
            .get(column_id)
            .copied()
            .unwrap_or(150.0) // Default width
    }
}

// Implementations for associated types
impl<T> FilterManager<T> {
    fn new() -> Self {
        Self {
            filters: HashMap::new(),
            global_filter: None,
            quick_filters: Vec::new(),
            filter_logic: FilterLogic::And,
            filtered_rows: Vec::new(),
            dirty: false,
        }
    }
}

impl<T> SortManager<T> {
    fn new() -> Self {
        Self {
            sort_columns: Vec::new(),
            dirty: false,
            comparators: HashMap::new(),
        }
    }
}

impl<T> EditManager<T> {
    fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
            editing_cell: None,
            pending_changes: HashMap::new(),
            validation_errors: HashMap::new(),
            edit_mode: EditMode::None,
            auto_save: false,
        }
    }
}

// Default implementations
impl Default for DataGridConfig {
    fn default() -> Self {
        Self {
            row_height: 32.0,
            dynamic_row_heights: false,
            show_header: true,
            show_footer: true,
            show_row_numbers: false,
            horizontal_scrolling: true,
            vertical_scrolling: true,
            min_column_width: 50.0,
            max_column_width: 500.0,
            border_width: 1.0,
            zebra_striping: true,
            hover_highlighting: true,
            keyboard_navigation: true,
            page_size: None,
        }
    }
}

impl Default for VirtualizationConfig {
    fn default() -> Self {
        Self {
            enable_row_virtualization: true,
            enable_column_virtualization: false,
            row_overscan: 5,
            column_overscan: 2,
            estimated_row_height: 32.0,
            estimated_column_width: 150.0,
        }
    }
}

impl Default for DataGridStyle {
    fn default() -> Self {
        Self {
            header_style: HeaderStyle {
                background_color: Color32::from_rgb(70, 70, 70),
                text_color: Color32::WHITE,
                font_size: 14.0,
                height: 32.0,
                padding: Margin::symmetric(8.0, 4.0),
                sort_indicator_color: Color32::from_rgb(100, 200, 255),
                filter_indicator_color: Color32::from_rgb(255, 200, 100),
            },
            cell_style: CellStyle {
                background_color: Color32::TRANSPARENT,
                text_color: Color32::WHITE,
                font_size: 13.0,
                padding: Margin::symmetric(8.0, 4.0),
                alignment: Align::LEFT,
            },
            row_style: RowStyle {
                even_row_color: Color32::from_rgb(45, 45, 45),
                odd_row_color: Color32::from_rgb(50, 50, 50),
                hover_color: Color32::from_rgb(60, 60, 60),
                selected_color: Color32::from_rgb(0, 120, 215),
                height: 32.0,
            },
            border_style: BorderStyle {
                color: Color32::from_rgb(100, 100, 100),
                width: 1.0,
                header_border_color: Color32::from_rgb(120, 120, 120),
                cell_border_color: Color32::from_rgb(80, 80, 80),
            },
            colors: ColorScheme {
                primary: Color32::from_rgb(0, 120, 215),
                secondary: Color32::from_rgb(108, 117, 125),
                accent: Color32::from_rgb(255, 193, 7),
                success: Color32::from_rgb(40, 167, 69),
                warning: Color32::from_rgb(255, 193, 7),
                error: Color32::from_rgb(220, 53, 69),
                info: Color32::from_rgb(23, 162, 184),
            },
        }
    }
}