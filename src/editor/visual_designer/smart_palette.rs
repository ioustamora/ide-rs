//! Smart Component Palette
//!
//! Provides an intelligent component palette with categorization,
//! search, favorites, recent components, and smart suggestions.

use egui::*;
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

/// Smart component palette with advanced features
pub struct SmartPalette {
    /// Component organization
    pub categories: Vec<ComponentCategory>,
    pub components: HashMap<String, PaletteComponent>,
    pub component_index: ComponentIndex,
    
    /// User interaction tracking
    pub recent_components: VecDeque<String>,
    pub favorite_components: Vec<String>,
    pub usage_statistics: UsageStatistics,
    
    /// Search and filtering
    pub search_query: String,
    pub active_filters: Vec<ComponentFilter>,
    pub search_results: Vec<String>,
    pub smart_suggestions: Vec<SmartSuggestion>,
    
    /// UI state
    pub expanded_categories: HashMap<String, bool>,
    pub selected_component: Option<String>,
    pub preview_mode: PreviewMode,
    pub palette_view: PaletteView,
    
    /// Advanced features
    pub custom_components: Vec<CustomComponent>,
    pub component_templates: Vec<ComponentTemplate>,
    pub ai_suggestions: AISuggestions,
    pub collaborative_palette: CollaborativePalette,
}

/// Component category with metadata
#[derive(Clone)]
pub struct ComponentCategory {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub color: Color32,
    pub components: Vec<String>,
    pub subcategories: Vec<ComponentCategory>,
    pub display_order: i32,
    pub is_expandable: bool,
}

/// Rich palette component definition
#[derive(Clone)]
pub struct PaletteComponent {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub subcategory: Option<String>,
    pub icon: ComponentIcon,
    pub preview: ComponentPreview,
    
    /// Metadata
    pub tags: Vec<String>,
    pub keywords: Vec<String>,
    pub difficulty_level: DifficultyLevel,
    pub compatibility: Vec<String>,
    pub version: String,
    
    /// Usage data
    pub usage_count: u32,
    pub last_used: Option<Instant>,
    pub rating: f32,
    pub user_notes: String,
    
    /// Component properties
    pub default_properties: HashMap<String, PropertyValue>,
    pub configurable_properties: Vec<ConfigurableProperty>,
    pub constraints: ComponentConstraints,
    pub responsive_behavior: ResponsiveBehavior,
    
    /// Advanced features
    pub has_animations: bool,
    pub has_interactions: bool,
    pub has_data_binding: bool,
    pub accessibility_features: Vec<AccessibilityFeature>,
}

/// Component indexing for fast search
pub struct ComponentIndex {
    pub name_index: HashMap<String, Vec<String>>,
    pub tag_index: HashMap<String, Vec<String>>,
    pub keyword_index: HashMap<String, Vec<String>>,
    pub category_index: HashMap<String, Vec<String>>,
    pub full_text_index: HashMap<String, Vec<String>>,
}

/// Usage statistics tracking
pub struct UsageStatistics {
    pub total_components_used: u32,
    pub session_usage: HashMap<String, u32>,
    pub daily_usage: HashMap<String, u32>,
    pub weekly_usage: HashMap<String, u32>,
    pub usage_patterns: Vec<UsagePattern>,
    pub peak_usage_times: Vec<PeakUsageTime>,
}

/// Smart filtering system
#[derive(Clone)]
pub struct ComponentFilter {
    pub filter_type: FilterType,
    pub criteria: FilterCriteria,
    pub is_active: bool,
    pub display_name: String,
}

/// AI-powered suggestions
pub struct SmartSuggestion {
    pub component_id: String,
    pub suggestion_type: SuggestionType,
    pub confidence: f32,
    pub reason: String,
    pub context: SuggestionContext,
}

/// Component preview system
#[derive(Clone)]
pub struct ComponentPreview {
    pub thumbnail: PreviewThumbnail,
    pub interactive_preview: bool,
    pub preview_size: Vec2,
    pub preview_states: Vec<PreviewState>,
    pub animation_preview: bool,
}

/// Custom user components
#[derive(Clone)]
pub struct CustomComponent {
    pub base_component: PaletteComponent,
    pub creation_date: Instant,
    pub author: String,
    pub is_shared: bool,
    pub sharing_permissions: SharingPermissions,
    pub version_history: Vec<ComponentVersion>,
}

/// Component templates
#[derive(Clone)]
pub struct ComponentTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub components: Vec<TemplateComponent>,
    pub layout: TemplateLayout,
    pub properties: HashMap<String, PropertyValue>,
    pub preview: ComponentPreview,
}

/// AI suggestion system
pub struct AISuggestions {
    pub enabled: bool,
    pub suggestion_engine: SuggestionEngine,
    pub learning_data: LearningData,
    pub personalization: PersonalizationData,
    pub context_analysis: ContextAnalysis,
}

/// Collaborative features
pub struct CollaborativePalette {
    pub shared_components: Vec<SharedComponent>,
    pub team_favorites: Vec<String>,
    pub collaboration_history: Vec<CollaborationEvent>,
    pub sharing_enabled: bool,
}

// Enums for various systems
#[derive(Clone, PartialEq)]
pub enum PreviewMode {
    Thumbnail,
    Detailed,
    Interactive,
    LivePreview,
}

#[derive(Clone, PartialEq)]
pub enum PaletteView {
    Grid,
    List,
    Compact,
    Tree,
}

#[derive(Clone, PartialEq)]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Clone)]
pub enum ComponentIcon {
    Unicode(String),
    Image(String),
    Svg(String),
    Generated(IconGenerator),
}

#[derive(Clone)]
pub enum FilterType {
    Category,
    Tag,
    Difficulty,
    RecentlyUsed,
    Favorites,
    Rating,
    Compatibility,
    Features,
}

#[derive(Clone)]
pub enum SuggestionType {
    Similar,
    Complementary,
    Popular,
    Trending,
    Contextual,
    AIRecommended,
}

// Supporting structures
#[derive(Clone)]
pub struct FilterCriteria {
    pub values: Vec<String>,
    pub operator: FilterOperator,
    pub case_sensitive: bool,
}

#[derive(Clone)]
pub enum FilterOperator {
    Equals,
    Contains,
    StartsWith,
    EndsWith,
    GreaterThan,
    LessThan,
    InRange,
}

#[derive(Clone)]
pub struct UsagePattern {
    pub pattern_type: String,
    pub frequency: f32,
    pub components: Vec<String>,
    pub time_range: TimeRange,
}

#[derive(Clone)]
pub struct TimeRange {
    pub start: Instant,
    pub end: Instant,
}

#[derive(Clone)]
pub struct PeakUsageTime {
    pub hour: u8,
    pub day_of_week: u8,
    pub usage_count: u32,
}

#[derive(Clone)]
pub struct SuggestionContext {
    pub current_selection: Vec<String>,
    pub canvas_content: Vec<String>,
    pub project_type: String,
    pub user_preferences: Vec<String>,
}

#[derive(Clone)]
pub struct PreviewThumbnail {
    pub image_data: Vec<u8>,
    pub size: Vec2,
    pub format: ThumbnailFormat,
    pub quality: f32,
}

#[derive(Clone)]
pub enum ThumbnailFormat {
    Png,
    Jpeg,
    Svg,
    WebP,
}

#[derive(Clone)]
pub struct PreviewState {
    pub state_name: String,
    pub thumbnail: PreviewThumbnail,
    pub description: String,
}

#[derive(Clone)]
pub struct ConfigurableProperty {
    pub property_name: String,
    pub property_type: PropertyType,
    pub default_value: PropertyValue,
    pub constraints: PropertyConstraints,
    pub description: String,
    pub is_required: bool,
}

#[derive(Clone)]
pub enum PropertyType {
    String,
    Number,
    Boolean,
    Color,
    Size,
    Position,
    Font,
    Image,
    List,
    Object,
}

#[derive(Clone)]
pub struct PropertyConstraints {
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub allowed_values: Vec<PropertyValue>,
    pub pattern: Option<String>,
    pub validation_rules: Vec<ValidationRule>,
}

#[derive(Clone)]
pub struct ComponentConstraints {
    pub min_size: Vec2,
    pub max_size: Vec2,
    pub allowed_parents: Vec<String>,
    pub allowed_children: Vec<String>,
    pub layout_requirements: Vec<LayoutRequirement>,
}

#[derive(Clone)]
pub struct ResponsiveBehavior {
    pub is_responsive: bool,
    pub breakpoints: Vec<ResponsiveBreakpoint>,
    pub scaling_strategy: ScalingStrategy,
}

#[derive(Clone)]
pub struct AccessibilityFeature {
    pub feature_type: String,
    pub description: String,
    pub wcag_compliance: WcagLevel,
}

#[derive(Clone)]
pub enum WcagLevel {
    A,
    AA,
    AAA,
}

impl SmartPalette {
    pub fn new() -> Self {
        let mut palette = Self {
            categories: Self::create_default_categories(),
            components: HashMap::new(),
            component_index: ComponentIndex::new(),
            
            recent_components: VecDeque::with_capacity(20),
            favorite_components: Vec::new(),
            usage_statistics: UsageStatistics::new(),
            
            search_query: String::new(),
            active_filters: Vec::new(),
            search_results: Vec::new(),
            smart_suggestions: Vec::new(),
            
            expanded_categories: HashMap::new(),
            selected_component: None,
            preview_mode: PreviewMode::Thumbnail,
            palette_view: PaletteView::Grid,
            
            custom_components: Vec::new(),
            component_templates: Vec::new(),
            ai_suggestions: AISuggestions::new(),
            collaborative_palette: CollaborativePalette::new(),
        };
        
        palette.populate_default_components();
        palette
    }
    
    /// Render the smart palette UI
    pub fn render(&mut self, ui: &mut Ui, available_rect: Rect) {
        ui.allocate_ui_at_rect(available_rect, |ui| {
            ui.vertical(|ui| {
                // Render palette header with search and controls
                self.render_palette_header(ui);
                
                ui.separator();
                
                // Render active filters
                if !self.active_filters.is_empty() {
                    self.render_active_filters(ui);
                    ui.separator();
                }
                
                // Render smart suggestions if available
                if !self.smart_suggestions.is_empty() {
                    self.render_smart_suggestions(ui);
                    ui.separator();
                }
                
                // Main palette content
                ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        match self.palette_view {
                            PaletteView::Grid => self.render_grid_view(ui),
                            PaletteView::List => self.render_list_view(ui),
                            PaletteView::Compact => self.render_compact_view(ui),
                            PaletteView::Tree => self.render_tree_view(ui),
                        }
                    });
            });
        });
    }
    
    fn render_palette_header(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            // Search box
            ui.label("🔍");
            let search_response = ui.text_edit_singleline(&mut self.search_query);
            if search_response.changed() {
                self.update_search_results();
            }
            
            // Clear search button
            if !self.search_query.is_empty() && ui.small_button("✖").clicked() {
                self.search_query.clear();
                self.update_search_results();
            }
            
            ui.separator();
            
            // View mode selector
            ui.label("View:");
            if ui.selectable_label(self.palette_view == PaletteView::Grid, "⊞").clicked() {
                self.palette_view = PaletteView::Grid;
            }
            if ui.selectable_label(self.palette_view == PaletteView::List, "☰").clicked() {
                self.palette_view = PaletteView::List;
            }
            if ui.selectable_label(self.palette_view == PaletteView::Compact, "▤").clicked() {
                self.palette_view = PaletteView::Compact;
            }
            if ui.selectable_label(self.palette_view == PaletteView::Tree, "🌳").clicked() {
                self.palette_view = PaletteView::Tree;
            }
            
            ui.separator();
            
            // Preview mode selector
            ui.label("Preview:");
            ComboBox::from_id_source("preview_mode")
                .selected_text(format!("{:?}", self.preview_mode))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.preview_mode, PreviewMode::Thumbnail, "Thumbnail");
                    ui.selectable_value(&mut self.preview_mode, PreviewMode::Detailed, "Detailed");
                    ui.selectable_value(&mut self.preview_mode, PreviewMode::Interactive, "Interactive");
                    ui.selectable_value(&mut self.preview_mode, PreviewMode::LivePreview, "Live Preview");
                });
        });
        
        // Filter bar
        ui.horizontal_wrapped(|ui| {
            ui.label("Filters:");
            
            // Quick filter buttons
            if ui.small_button("Recent").clicked() {
                self.toggle_filter(ComponentFilter::recent_filter());
            }
            if ui.small_button("Favorites").clicked() {
                self.toggle_filter(ComponentFilter::favorites_filter());
            }
            if ui.small_button("Popular").clicked() {
                self.toggle_filter(ComponentFilter::popular_filter());
            }
            
            // Category filters
            for category in &self.categories.clone() {
                if ui.small_button(&category.name).clicked() {
                    self.toggle_filter(ComponentFilter::category_filter(&category.id));
                }
            }
            
            // Advanced filter button
            if ui.small_button("Advanced...").clicked() {
                self.show_advanced_filters();
            }
        });
    }
    
    fn render_active_filters(&mut self, ui: &mut Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.label("Active filters:");
            
            let mut filters_to_remove = Vec::new();
            for (index, filter) in self.active_filters.iter().enumerate() {
                if filter.is_active {
                    ui.horizontal(|ui| {
                        ui.label(&filter.display_name);
                        if ui.small_button("✖").clicked() {
                            filters_to_remove.push(index);
                        }
                    });
                }
            }
            
            // Remove filters
            for &index in filters_to_remove.iter().rev() {
                self.active_filters.remove(index);
            }
            
            if !filters_to_remove.is_empty() {
                self.update_search_results();
            }
            
            // Clear all filters
            if !self.active_filters.is_empty() && ui.small_button("Clear All").clicked() {
                self.active_filters.clear();
                self.update_search_results();
            }
        });
    }
    
    fn render_smart_suggestions(&mut self, ui: &mut Ui) {
        ui.collapsing("💡 Smart Suggestions", |ui| {
            ui.horizontal_wrapped(|ui| {
                for suggestion in &self.smart_suggestions {
                    if let Some(component) = self.components.get(&suggestion.component_id) {
                        if ui.small_button(&component.name).clicked() {
                            self.select_component(&suggestion.component_id);
                        }
                        
                        if ui.response().hovered() {
                            ui.ctx().show_tooltip_text(
                                &format!("{}\nConfidence: {:.1}%", suggestion.reason, suggestion.confidence * 100.0)
                            );
                        }
                    }
                }
            });
        });
    }
    
    fn render_grid_view(&mut self, ui: &mut Ui) {
        let components_to_show = self.get_filtered_components();
        let grid_size = match self.preview_mode {
            PreviewMode::Thumbnail => Vec2::new(80.0, 80.0),
            PreviewMode::Detailed => Vec2::new(120.0, 120.0),
            PreviewMode::Interactive => Vec2::new(150.0, 150.0),
            PreviewMode::LivePreview => Vec2::new(180.0, 180.0),
        };
        
        let columns = (ui.available_width() / (grid_size.x + 10.0)).floor() as usize;
        let columns = columns.max(1);
        
        // Collect component data to avoid borrowing issues
        let mut component_data: Vec<(String, PaletteComponent)> = Vec::new();
        for component_id in &components_to_show {
            if let Some(component) = self.components.get(component_id) {
                component_data.push((component_id.clone(), component.clone()));
            }
        }
        
        Grid::new("component_grid")
            .num_columns(columns)
            .spacing([5.0, 5.0])
            .show(ui, |ui| {
                for (index, (component_id, component)) in component_data.iter().enumerate() {
                    self.render_component_card_data(ui, component_id, component, grid_size);
                    
                    if (index + 1) % columns == 0 {
                        ui.end_row();
                    }
                }
            });
    }
    
    fn render_list_view(&mut self, ui: &mut Ui) {
        let components_to_show = self.get_filtered_components();
        
        for component_id in components_to_show {
            if let Some(component) = self.components.get(&component_id) {
                ui.horizontal(|ui| {
                    // Component icon
                    self.render_component_icon(ui, &component.icon, Vec2::new(32.0, 32.0));
                    
                    ui.vertical(|ui| {
                        // Component name
                        ui.heading(&component.name);
                        
                        // Component description
                        ui.label(&component.description);
                        
                        // Tags
                        ui.horizontal_wrapped(|ui| {
                            for tag in &component.tags {
                                ui.small_button(tag);
                            }
                        });
                        
                        // Usage info
                        if component.usage_count > 0 {
                            ui.label(format!("Used {} times", component.usage_count));
                        }
                    });
                    
                    // Actions
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.button("Add").clicked() {
                            self.add_component_to_canvas(component_id);
                        }
                        
                        if ui.small_button("⭐").clicked() {
                            self.toggle_favorite(component_id);
                        }
                        
                        if ui.small_button("ℹ").clicked() {
                            self.show_component_details(component_id);
                        }
                    });
                });
                
                ui.separator();
            }
        }
    }
    
    fn render_compact_view(&mut self, ui: &mut Ui) {
        let components_to_show = self.get_filtered_components();
        
        ui.horizontal_wrapped(|ui| {
            for component_id in components_to_show {
                if let Some(component) = self.components.get(&component_id) {
                    if ui.small_button(&component.name).clicked() {
                        self.add_component_to_canvas(component_id);
                    }
                    
                    if ui.response().hovered() {
                        ui.ctx().show_tooltip_text(&component.description);
                    }
                }
            }
        });
    }
    
    fn render_tree_view(&mut self, ui: &mut Ui) {
        for category in &self.categories.clone() {
            let is_expanded = self.expanded_categories.get(&category.id).copied().unwrap_or(true);
            
            CollapsingHeader::new(&category.name)
                .default_open(is_expanded)
                .icon(Self::category_icon)
                .show(ui, |ui| {
                    self.render_category_components(ui, category);
                });
        }
    }
    
    fn render_component_card_data(&mut self, ui: &mut Ui, component_id: &str, component: &PaletteComponent, size: Vec2) {
        let response = ui.allocate_response(size, Sense::click_and_drag());
        let rect = response.rect;
        
        // Draw card background
        let bg_color = if self.selected_component.as_ref() == Some(component_id) {
            Color32::from_rgb(70, 130, 200)
        } else if response.hovered() {
            Color32::from_rgb(60, 60, 70)
        } else {
            Color32::from_rgb(40, 40, 50)
        };
        
        ui.painter().rect_filled(rect, 4.0, bg_color);
        
        // Draw component preview
        let preview_rect = Rect::from_min_size(rect.min + Vec2::new(5.0, 5.0), size - Vec2::new(10.0, 25.0));
        self.render_component_preview(ui, component, preview_rect);
        
        // Draw component name
        let text_rect = Rect::from_min_size(
            Pos2::new(rect.min.x + 2.0, rect.max.y - 20.0),
            Vec2::new(size.x - 4.0, 15.0),
        );
        
        ui.painter().text(
            text_rect.center(),
            Anchor2::CENTER_CENTER,
            &component.name,
            FontId::proportional(10.0),
            Color32::WHITE,
        );
        
        // Handle interactions
        if response.clicked() {
            self.select_component(component_id);
        }
        
        if response.double_clicked() {
            self.add_component_to_canvas(component_id);
        }
        
        if response.dragged() {
            // Start drag operation
            self.start_component_drag(component_id, response.drag_delta());
        }
        
        // Show context menu
        response.context_menu(|ui| {
            if ui.button("Add to Canvas").clicked() {
                self.add_component_to_canvas(component_id);
                ui.close_menu();
            }
            
            let favorite_text = if self.favorite_components.contains(&component_id.to_string()) {
                "Remove from Favorites"
            } else {
                "Add to Favorites"
            };
            
            if ui.button(favorite_text).clicked() {
                self.toggle_favorite(component_id);
                ui.close_menu();
            }
            
            if ui.button("Component Details").clicked() {
                self.show_component_details(component_id);
                ui.close_menu();
            }
            
            if ui.button("Create Template").clicked() {
                self.create_template_from_component(component_id);
                ui.close_menu();
            }
        });
    }
    
    fn render_component_card(&mut self, ui: &mut Ui, component: &PaletteComponent, size: Vec2) {
        let response = ui.allocate_response(size, Sense::click_and_drag());
        let rect = response.rect;
        
        // Draw card background
        let bg_color = if self.selected_component.as_ref() == Some(&component.id) {
            Color32::from_rgb(70, 130, 200)
        } else if response.hovered() {
            Color32::from_rgb(60, 60, 70)
        } else {
            Color32::from_rgb(40, 40, 50)
        };
        
        ui.painter().rect_filled(rect, 4.0, bg_color);
        
        // Draw component preview
        let preview_rect = Rect::from_min_size(rect.min + Vec2::new(5.0, 5.0), size - Vec2::new(10.0, 25.0));
        self.render_component_preview(ui, component, preview_rect);
        
        // Draw component name
        let text_rect = Rect::from_min_size(
            Pos2::new(rect.min.x + 2.0, rect.max.y - 20.0),
            Vec2::new(size.x - 4.0, 15.0),
        );
        
        ui.painter().text(
            text_rect.center(),
            Anchor2::CENTER_CENTER,
            &component.name,
            FontId::proportional(10.0),
            Color32::WHITE,
        );
        
        // Handle interactions
        if response.clicked() {
            self.select_component(&component.id);
        }
        
        if response.double_clicked() {
            self.add_component_to_canvas(&component.id);
        }
        
        if response.dragged() {
            // Start drag operation
            self.start_component_drag(&component.id, response.drag_delta());
        }
        
        // Show context menu
        response.context_menu(|ui| {
            if ui.button("Add to Canvas").clicked() {
                self.add_component_to_canvas(&component.id);
                ui.close_menu();
            }
            
            let favorite_text = if self.favorite_components.contains(&component.id) {
                "Remove from Favorites"
            } else {
                "Add to Favorites"
            };
            
            if ui.button(favorite_text).clicked() {
                self.toggle_favorite(&component.id);
                ui.close_menu();
            }
            
            if ui.button("Component Details").clicked() {
                self.show_component_details(&component.id);
                ui.close_menu();
            }
            
            if ui.button("Create Template").clicked() {
                self.create_template_from_component(&component.id);
                ui.close_menu();
            }
        });
    }
    
    fn render_component_preview(&self, ui: &mut Ui, component: &PaletteComponent, rect: Rect) {
        let painter = ui.painter();
        
        match self.preview_mode {
            PreviewMode::Thumbnail => {
                // Simple icon/thumbnail
                self.render_component_icon(ui, &component.icon, rect.size());
            }
            PreviewMode::Detailed => {
                // Detailed preview with more information
                painter.rect_filled(rect, 2.0, Color32::from_gray(30));
                self.render_component_icon(ui, &component.icon, Vec2::new(32.0, 32.0));
                
                // Show some properties
                painter.text(
                    rect.center() + Vec2::new(0.0, 20.0),
                    Anchor2::CENTER_CENTER,
                    &format!("{}x{}", component.default_properties.len(), component.configurable_properties.len()),
                    FontId::proportional(8.0),
                    Color32::LIGHT_GRAY,
                );
            }
            PreviewMode::Interactive => {
                // Interactive preview
                painter.rect_stroke(rect, 2.0, Stroke::new(1.0, Color32::BLUE));
                self.render_component_icon(ui, &component.icon, rect.size() * 0.7);
                
                if component.has_interactions {
                    painter.text(
                        rect.max - Vec2::new(5.0, 5.0),
                        Anchor2::RIGHT_BOTTOM,
                        "🖱",
                        FontId::proportional(12.0),
                        Color32::YELLOW,
                    );
                }
            }
            PreviewMode::LivePreview => {
                // Live preview - would show actual component rendering
                painter.rect_filled(rect, 2.0, Color32::from_gray(20));
                painter.text(
                    rect.center(),
                    Anchor2::CENTER_CENTER,
                    "Live",
                    FontId::proportional(10.0),
                    Color32::GREEN,
                );
            }
        }
    }
    
    fn render_component_icon(&self, ui: &mut Ui, icon: &ComponentIcon, size: Vec2) {
        let painter = ui.painter();
        let rect = Rect::from_center_size(ui.cursor().center(), size);
        
        match icon {
            ComponentIcon::Unicode(unicode) => {
                painter.text(
                    rect.center(),
                    Anchor2::CENTER_CENTER,
                    unicode,
                    FontId::proportional(size.y * 0.7),
                    Color32::WHITE,
                );
            }
            ComponentIcon::Image(_path) => {
                // Would load and render image
                painter.rect_filled(rect, 2.0, Color32::from_gray(100));
                painter.text(
                    rect.center(),
                    Anchor2::CENTER_CENTER,
                    "IMG",
                    FontId::proportional(size.y * 0.3),
                    Color32::BLACK,
                );
            }
            ComponentIcon::Svg(_svg) => {
                // Would render SVG
                painter.rect_filled(rect, 2.0, Color32::from_gray(120));
                painter.text(
                    rect.center(),
                    Anchor2::CENTER_CENTER,
                    "SVG",
                    FontId::proportional(size.y * 0.3),
                    Color32::BLACK,
                );
            }
            ComponentIcon::Generated(_generator) => {
                // Generate icon procedurally
                painter.circle_filled(rect.center(), size.x * 0.4, Color32::BLUE);
            }
        }
    }
    
    fn render_category_components(&mut self, ui: &mut Ui, category: &ComponentCategory) {
        // Collect component data to avoid borrowing issues
        let mut component_data: Vec<(String, String, ComponentIcon)> = Vec::new();
        for component_id in &category.components {
            if let Some(component) = self.components.get(component_id) {
                component_data.push((component.id.clone(), component.name.clone(), component.icon.clone()));
            }
        }
        
        for (component_id, component_name, component_icon) in component_data {
            ui.horizontal(|ui| {
                self.render_component_icon(ui, &component_icon, Vec2::new(24.0, 24.0));
                
                if ui.selectable_label(
                    self.selected_component.as_ref() == Some(&component_id),
                    &component_name,
                ).clicked() {
                    self.select_component(&component_id);
                }
                
                if ui.small_button("+").clicked() {
                    self.add_component_to_canvas(&component_id);
                }
            });
        }
        
        // Render subcategories
        for subcategory in &category.subcategories.clone() {
            ui.indent(subcategory.id.as_str(), |ui| {
                CollapsingHeader::new(&subcategory.name)
                    .default_open(false)
                    .show(ui, |ui| {
                        self.render_category_components(ui, &subcategory);
                    });
            });
        }
    }
    
    // Helper methods
    fn create_default_categories() -> Vec<ComponentCategory> {
        vec![
            ComponentCategory {
                id: "basic".to_string(),
                name: "Basic Controls".to_string(),
                description: "Essential UI components for user interfaces".to_string(),
                icon: "🧩".to_string(),
                color: Color32::from_rgb(100, 150, 200),
                components: vec!["button".to_string(), "label".to_string(), "textbox".to_string()],
                subcategories: vec![],
                display_order: 1,
                is_expandable: true,
            },
            ComponentCategory {
                id: "layout".to_string(),
                name: "Layout".to_string(),
                description: "Containers and layout components".to_string(),
                icon: "📐".to_string(),
                color: Color32::from_rgb(150, 100, 200),
                components: vec!["panel".to_string(), "grid".to_string(), "stackpanel".to_string()],
                subcategories: vec![],
                display_order: 2,
                is_expandable: true,
            },
            ComponentCategory {
                id: "data".to_string(),
                name: "Data Display".to_string(),
                description: "Components for displaying and editing data".to_string(),
                icon: "📊".to_string(),
                color: Color32::from_rgb(200, 150, 100),
                components: vec!["datagrid".to_string(), "list".to_string(), "tree".to_string()],
                subcategories: vec![],
                display_order: 3,
                is_expandable: true,
            },
            ComponentCategory {
                id: "navigation".to_string(),
                name: "Navigation".to_string(),
                description: "Navigation and menu components".to_string(),
                icon: "🧭".to_string(),
                color: Color32::from_rgb(100, 200, 150),
                components: vec!["menu".to_string(), "toolbar".to_string(), "tabs".to_string()],
                subcategories: vec![],
                display_order: 4,
                is_expandable: true,
            },
        ]
    }
    
    fn populate_default_components(&mut self) {
        // Create some default components
        self.add_default_component("button", "Button", "Basic button control", "basic", "🔘");
        self.add_default_component("label", "Label", "Text label", "basic", "🏷️");
        self.add_default_component("textbox", "TextBox", "Text input field", "basic", "📝");
        self.add_default_component("panel", "Panel", "Container panel", "layout", "⬜");
        self.add_default_component("grid", "Grid", "Grid layout", "layout", "⊞");
        self.add_default_component("datagrid", "DataGrid", "Data table", "data", "📊");
        self.add_default_component("menu", "Menu", "Application menu", "navigation", "☰");
    }
    
    fn add_default_component(&mut self, id: &str, name: &str, description: &str, category: &str, icon: &str) {
        let component = PaletteComponent {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            category: category.to_string(),
            subcategory: None,
            icon: ComponentIcon::Unicode(icon.to_string()),
            preview: ComponentPreview::default(),
            
            tags: vec![category.to_string()],
            keywords: vec![name.to_lowercase()],
            difficulty_level: DifficultyLevel::Beginner,
            compatibility: vec!["all".to_string()],
            version: "1.0.0".to_string(),
            
            usage_count: 0,
            last_used: None,
            rating: 5.0,
            user_notes: String::new(),
            
            default_properties: HashMap::new(),
            configurable_properties: vec![],
            constraints: ComponentConstraints::default(),
            responsive_behavior: ResponsiveBehavior::default(),
            
            has_animations: false,
            has_interactions: name == "Button",
            has_data_binding: matches!(name, "TextBox" | "DataGrid"),
            accessibility_features: vec![],
        };
        
        self.components.insert(id.to_string(), component);
        self.component_index.add_component(id, name, &[category], &[name.to_lowercase()]);
    }
    
    fn category_icon(_openness: f32, ui: &mut Ui, _rect: Rect) {
        // Custom category icon rendering
        ui.label("📁");
    }
    
    fn update_search_results(&mut self) {
        if self.search_query.is_empty() && self.active_filters.is_empty() {
            self.search_results.clear();
            return;
        }
        
        self.search_results = self.component_index.search(&self.search_query, &self.active_filters);
        self.update_smart_suggestions();
    }
    
    fn get_filtered_components(&self) -> Vec<String> {
        if self.search_results.is_empty() && (self.search_query.is_empty() && self.active_filters.is_empty()) {
            self.components.keys().cloned().collect()
        } else {
            self.search_results.clone()
        }
    }
    
    fn toggle_filter(&mut self, filter: ComponentFilter) {
        if let Some(existing) = self.active_filters.iter_mut().find(|f| f.filter_type == filter.filter_type) {
            existing.is_active = !existing.is_active;
        } else {
            self.active_filters.push(filter);
        }
        self.update_search_results();
    }
    
    fn select_component(&mut self, component_id: &str) {
        self.selected_component = Some(component_id.to_string());
    }
    
    fn add_component_to_canvas(&mut self, component_id: &str) {
        // Record usage
        if let Some(component) = self.components.get_mut(component_id) {
            component.usage_count += 1;
            component.last_used = Some(Instant::now());
        }
        
        // Add to recent components
        self.recent_components.push_front(component_id.to_string());
        if self.recent_components.len() > 20 {
            self.recent_components.pop_back();
        }
        
        // Update usage statistics
        self.usage_statistics.record_usage(component_id);
        
        // Trigger canvas add event (would be handled by parent)
        println!("Adding component to canvas: {}", component_id);
    }
    
    fn toggle_favorite(&mut self, component_id: &str) {
        if let Some(pos) = self.favorite_components.iter().position(|id| id == component_id) {
            self.favorite_components.remove(pos);
        } else {
            self.favorite_components.push(component_id.to_string());
        }
    }
    
    fn show_component_details(&self, component_id: &str) {
        println!("Showing details for component: {}", component_id);
    }
    
    fn show_advanced_filters(&self) {
        println!("Showing advanced filter dialog");
    }
    
    fn start_component_drag(&mut self, component_id: &str, _delta: Vec2) {
        println!("Starting drag for component: {}", component_id);
    }
    
    fn create_template_from_component(&mut self, component_id: &str) {
        println!("Creating template from component: {}", component_id);
    }
    
    fn update_smart_suggestions(&mut self) {
        // AI-powered suggestion logic would go here
        self.smart_suggestions.clear();
        
        // For now, suggest popular components
        let mut popular_components: Vec<_> = self.components
            .iter()
            .filter(|(id, _)| !self.get_filtered_components().contains(id))
            .collect();
        popular_components.sort_by(|a, b| b.1.usage_count.cmp(&a.1.usage_count));
        
        for (component_id, _) in popular_components.iter().take(3) {
            self.smart_suggestions.push(SmartSuggestion {
                component_id: component_id.to_string(),
                suggestion_type: SuggestionType::Popular,
                confidence: 0.8,
                reason: "Popular component".to_string(),
                context: SuggestionContext::default(),
            });
        }
    }
}

// Implementation for supporting structures
impl ComponentIndex {
    fn new() -> Self {
        Self {
            name_index: HashMap::new(),
            tag_index: HashMap::new(),
            keyword_index: HashMap::new(),
            category_index: HashMap::new(),
            full_text_index: HashMap::new(),
        }
    }
    
    fn add_component(&mut self, id: &str, name: &str, tags: &[&str], keywords: &[String]) {
        // Add to name index
        let name_words: Vec<String> = name.to_lowercase().split_whitespace().map(|s| s.to_string()).collect();
        for word in name_words {
            self.name_index.entry(word).or_insert_with(Vec::new).push(id.to_string());
        }
        
        // Add to tag index
        for tag in tags {
            self.tag_index.entry(tag.to_string()).or_insert_with(Vec::new).push(id.to_string());
        }
        
        // Add to keyword index
        for keyword in keywords {
            self.keyword_index.entry(keyword.clone()).or_insert_with(Vec::new).push(id.to_string());
        }
    }
    
    fn search(&self, query: &str, filters: &[ComponentFilter]) -> Vec<String> {
        let mut results = std::collections::HashSet::new();
        
        if !query.is_empty() {
            let query_lower = query.to_lowercase();
            
            // Search in names
            for (word, components) in &self.name_index {
                if word.contains(&query_lower) {
                    results.extend(components.iter().cloned());
                }
            }
            
            // Search in keywords
            for (keyword, components) in &self.keyword_index {
                if keyword.contains(&query_lower) {
                    results.extend(components.iter().cloned());
                }
            }
        }
        
        // Apply filters
        for filter in filters {
            if filter.is_active {
                results = self.apply_filter(&results, filter);
            }
        }
        
        results.into_iter().collect()
    }
    
    fn apply_filter(&self, results: &std::collections::HashSet<String>, filter: &ComponentFilter) -> std::collections::HashSet<String> {
        // Filter implementation would go here
        results.clone()
    }
}

impl UsageStatistics {
    fn new() -> Self {
        Self {
            total_components_used: 0,
            session_usage: HashMap::new(),
            daily_usage: HashMap::new(),
            weekly_usage: HashMap::new(),
            usage_patterns: Vec::new(),
            peak_usage_times: Vec::new(),
        }
    }
    
    fn record_usage(&mut self, component_id: &str) {
        self.total_components_used += 1;
        *self.session_usage.entry(component_id.to_string()).or_insert(0) += 1;
        *self.daily_usage.entry(component_id.to_string()).or_insert(0) += 1;
        *self.weekly_usage.entry(component_id.to_string()).or_insert(0) += 1;
    }
}

impl ComponentFilter {
    fn recent_filter() -> Self {
        Self {
            filter_type: FilterType::RecentlyUsed,
            criteria: FilterCriteria {
                values: vec!["recent".to_string()],
                operator: FilterOperator::Equals,
                case_sensitive: false,
            },
            is_active: true,
            display_name: "Recent".to_string(),
        }
    }
    
    fn favorites_filter() -> Self {
        Self {
            filter_type: FilterType::Favorites,
            criteria: FilterCriteria {
                values: vec!["favorites".to_string()],
                operator: FilterOperator::Equals,
                case_sensitive: false,
            },
            is_active: true,
            display_name: "Favorites".to_string(),
        }
    }
    
    fn popular_filter() -> Self {
        Self {
            filter_type: FilterType::Rating,
            criteria: FilterCriteria {
                values: vec!["4.0".to_string()],
                operator: FilterOperator::GreaterThan,
                case_sensitive: false,
            },
            is_active: true,
            display_name: "Popular".to_string(),
        }
    }
    
    fn category_filter(category_id: &str) -> Self {
        Self {
            filter_type: FilterType::Category,
            criteria: FilterCriteria {
                values: vec![category_id.to_string()],
                operator: FilterOperator::Equals,
                case_sensitive: false,
            },
            is_active: true,
            display_name: format!("Category: {}", category_id),
        }
    }
}

impl AISuggestions {
    fn new() -> Self {
        Self {
            enabled: true,
            suggestion_engine: SuggestionEngine::new(),
            learning_data: LearningData::new(),
            personalization: PersonalizationData::new(),
            context_analysis: ContextAnalysis::new(),
        }
    }
}

impl CollaborativePalette {
    fn new() -> Self {
        Self {
            shared_components: Vec::new(),
            team_favorites: Vec::new(),
            collaboration_history: Vec::new(),
            sharing_enabled: false,
        }
    }
}

impl Default for ComponentPreview {
    fn default() -> Self {
        Self {
            thumbnail: PreviewThumbnail::default(),
            interactive_preview: false,
            preview_size: Vec2::new(64.0, 64.0),
            preview_states: vec![],
            animation_preview: false,
        }
    }
}

impl Default for PreviewThumbnail {
    fn default() -> Self {
        Self {
            image_data: vec![],
            size: Vec2::new(64.0, 64.0),
            format: ThumbnailFormat::Png,
            quality: 0.8,
        }
    }
}

impl Default for ComponentConstraints {
    fn default() -> Self {
        Self {
            min_size: Vec2::new(10.0, 10.0),
            max_size: Vec2::new(1000.0, 1000.0),
            allowed_parents: vec![],
            allowed_children: vec![],
            layout_requirements: vec![],
        }
    }
}

impl Default for ResponsiveBehavior {
    fn default() -> Self {
        Self {
            is_responsive: false,
            breakpoints: vec![],
            scaling_strategy: ScalingStrategy::Proportional,
        }
    }
}

impl Default for SuggestionContext {
    fn default() -> Self {
        Self {
            current_selection: vec![],
            canvas_content: vec![],
            project_type: "general".to_string(),
            user_preferences: vec![],
        }
    }
}

// More placeholder implementations
#[derive(Clone)]
pub struct IconGenerator {
    pub generator_type: String,
}

#[derive(Clone)]
pub struct ValidationRule {
    pub rule_type: String,
    pub parameters: Vec<String>,
}

#[derive(Clone)]
pub struct LayoutRequirement {
    pub requirement_type: String,
    pub description: String,
}

#[derive(Clone)]
pub struct ResponsiveBreakpoint {
    pub width: f32,
    pub properties: HashMap<String, PropertyValue>,
}

#[derive(Clone)]
pub enum ScalingStrategy {
    Proportional,
    Fixed,
    Adaptive,
}

#[derive(Clone)]
pub struct TemplateComponent {
    pub component_id: String,
    pub position: Vec2,
    pub properties: HashMap<String, PropertyValue>,
}

#[derive(Clone)]
pub struct TemplateLayout {
    pub layout_type: String,
    pub constraints: Vec<String>,
}

#[derive(Clone)]
pub struct SharedComponent {
    pub component: PaletteComponent,
    pub shared_by: String,
    pub permissions: SharingPermissions,
}

#[derive(Clone)]
pub struct SharingPermissions {
    pub can_view: bool,
    pub can_edit: bool,
    pub can_share: bool,
}

#[derive(Clone)]
pub struct ComponentVersion {
    pub version: String,
    pub changes: Vec<String>,
    pub timestamp: Instant,
}

#[derive(Clone)]
pub struct CollaborationEvent {
    pub event_type: String,
    pub user: String,
    pub timestamp: Instant,
    pub details: String,
}

pub struct SuggestionEngine {
    pub engine_type: String,
}

impl SuggestionEngine {
    fn new() -> Self {
        Self {
            engine_type: "ml_based".to_string(),
        }
    }
}

pub struct LearningData {
    pub usage_patterns: Vec<String>,
}

impl LearningData {
    fn new() -> Self {
        Self {
            usage_patterns: vec![],
        }
    }
}

pub struct PersonalizationData {
    pub preferences: HashMap<String, String>,
}

impl PersonalizationData {
    fn new() -> Self {
        Self {
            preferences: HashMap::new(),
        }
    }
}

pub struct ContextAnalysis {
    pub analysis_enabled: bool,
}

impl ContextAnalysis {
    fn new() -> Self {
        Self {
            analysis_enabled: true,
        }
    }
}

// Re-export PropertyValue from the enhanced canvas module for consistency
pub use super::enhanced_canvas::PropertyValue;