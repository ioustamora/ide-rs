//! Enhanced Property Inspector - Modular Architecture
//!
//! This module provides an advanced property inspector with modern UX features:
//! - Multi-selection property editing
//! - Design system integration with token suggestions
//! - AI-powered property suggestions and optimization
//! - Property animation and binding systems
//! - Contextual panels and smart workflows

pub mod multi_selection;
pub mod design_system_integration;
pub mod ai_suggestions;

// Re-export main types for convenience
pub use multi_selection::{MultiSelectionManager, ComponentSelection, CommonProperty, MultiEditResult};
pub use design_system_integration::{DesignSystemIntegration, DesignTokens, ColorToken, TypographyToken};
pub use ai_suggestions::{AiPropertySuggestions, PropertySuggestion, SuggestionCategory};

use egui::*;
use std::collections::{HashMap, HashSet};
use crate::rcl::ui::component::Component;
use crate::editor::inspector::{PropertyValue, PropertyType};

/// Enhanced property inspector with modern UX features
pub struct EnhancedPropertyInspector {
    /// Multi-selection property editing
    pub multi_selection: MultiSelectionManager,
    /// Design system integration
    pub design_system: DesignSystemIntegration,
    /// AI-powered suggestions
    pub ai_suggestions: AiPropertySuggestions,
    /// Property animation system
    pub animation_system: PropertyAnimationSystem,
    /// Contextual property panels
    pub contextual_panels: ContextualPanelManager,
    /// Property binding system
    pub property_bindings: PropertyBindingSystem,
    /// Current active tab
    active_tab: InspectorTab,
    /// Whether enhanced features are enabled
    pub enhanced_features_enabled: bool,
}

/// Property animation system for smooth transitions
pub struct PropertyAnimationSystem {
    /// Active animations
    pub active_animations: Vec<PropertyAnimation>,
    /// Animation presets
    pub presets: HashMap<String, AnimationPreset>,
    /// Transition settings
    pub transition_settings: TransitionSettings,
}

/// Property animation definition
#[derive(Clone, Debug)]
pub struct PropertyAnimation {
    /// Target component ID
    pub component_id: usize,
    /// Property being animated
    pub property_name: String,
    /// Start value
    pub start_value: PropertyValue,
    /// End value
    pub end_value: PropertyValue,
    /// Animation duration in milliseconds
    pub duration: u64,
    /// Animation start time
    pub start_time: std::time::Instant,
    /// Easing function
    pub easing: EasingFunction,
}

/// Animation preset for common transitions
#[derive(Clone, Debug)]
pub struct AnimationPreset {
    /// Preset name
    pub name: String,
    /// Default duration
    pub duration: u64,
    /// Default easing
    pub easing: EasingFunction,
    /// Properties this preset applies to
    pub applicable_properties: Vec<String>,
}

/// Easing function for animations
#[derive(Clone, Debug)]
pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bounce,
    Elastic,
}

/// Transition settings for property changes
#[derive(Clone, Debug)]
pub struct TransitionSettings {
    /// Whether transitions are enabled
    pub enabled: bool,
    /// Default transition duration
    pub default_duration: u64,
    /// Properties that should be animated
    pub animated_properties: HashSet<String>,
}

/// Contextual property panel manager
pub struct ContextualPanelManager {
    /// Available contextual panels
    pub panels: HashMap<String, ContextualPanel>,
    /// Current active panels
    pub active_panels: Vec<String>,
    /// Panel visibility rules
    pub visibility_rules: Vec<PanelVisibilityRule>,
}

/// Contextual property panel
pub struct ContextualPanel {
    /// Panel name
    pub name: String,
    /// Panel description
    pub description: String,
    /// Component types this panel applies to
    pub applicable_components: Vec<String>,
    /// Properties shown in this panel
    pub properties: Vec<ContextualProperty>,
    /// Panel priority for display order
    pub priority: u32,
    /// Whether panel is collapsible
    pub collapsible: bool,
}

/// Property in a contextual panel
#[derive(Clone, Debug)]
pub struct ContextualProperty {
    /// Property name
    pub name: String,
    /// Display label
    pub label: String,
    /// Property description
    pub description: String,
    /// Property type
    pub property_type: PropertyType,
    /// Whether property is required
    pub required: bool,
    /// Default value
    pub default_value: Option<PropertyValue>,
    /// Property group within panel
    pub group: String,
}

/// Rule for panel visibility
pub struct PanelVisibilityRule {
    /// Rule name
    pub name: String,
    /// Panel this rule applies to
    pub panel_name: String,
    /// Condition for visibility
    pub condition: VisibilityCondition,
}

/// Condition for panel visibility
#[derive(Clone, Debug)]
pub enum VisibilityCondition {
    /// Component type matches
    ComponentType(String),
    /// Property exists
    PropertyExists(String),
    /// Property has specific value
    PropertyValue(String, PropertyValue),
    /// Multiple conditions (AND)
    And(Vec<VisibilityCondition>),
    /// Alternative conditions (OR)
    Or(Vec<VisibilityCondition>),
    /// Inverted condition
    Not(Box<VisibilityCondition>),
}

/// Property context for intelligent suggestions
#[derive(Clone, Debug)]
pub struct PropertyContext {
    /// Component being edited
    pub component_id: usize,
    /// Component type
    pub component_type: String,
    /// Parent component context
    pub parent_context: Option<Box<PropertyContext>>,
    /// Sibling components
    pub siblings: Vec<ComponentReference>,
    /// Current design mode
    pub design_mode: DesignMode,
    /// User intent (if detected)
    pub detected_intent: Option<UserIntent>,
}

/// Reference to a component
#[derive(Clone, Debug)]
pub struct ComponentReference {
    /// Component ID
    pub id: usize,
    /// Component type
    pub component_type: String,
    /// Spatial relationship
    pub relationship: SpatialRelationship,
}

/// Spatial relationship between components
#[derive(Clone, Debug, PartialEq)]
pub enum SpatialRelationship {
    Above,
    Below,
    LeftOf,
    RightOf,
    Inside,
    Contains,
    Adjacent,
}

/// Current design mode context
#[derive(Clone, Debug, PartialEq)]
pub enum DesignMode {
    Layout,
    Styling,
    Content,
    Interaction,
    Animation,
    Responsive,
}

/// Detected user intent
#[derive(Clone, Debug)]
pub enum UserIntent {
    CreateLayout,
    StyleComponent,
    AddInteraction,
    OptimizePerformance,
    ImproveAccessibility,
    CreateAnimation,
}

/// Property binding system for dynamic values
pub struct PropertyBindingSystem {
    /// Active property bindings
    pub bindings: HashMap<String, PropertyBinding>,
    /// Available data sources
    pub data_sources: HashMap<String, DataSource>,
    /// Binding validators
    pub validators: Vec<BindingValidator>,
}

/// Property binding definition
#[derive(Clone, Debug)]
pub struct PropertyBinding {
    /// Source component ID
    pub source_component: usize,
    /// Source property name
    pub source_property: String,
    /// Target component ID
    pub target_component: usize,
    /// Target property name
    pub target_property: String,
    /// Binding transformation function
    pub transform: Option<String>,
    /// Whether binding is bidirectional
    pub bidirectional: bool,
}

/// Data source for property binding
#[derive(Clone, Debug)]
pub struct DataSource {
    /// Data source name
    pub name: String,
    /// Data source type
    pub source_type: DataSourceType,
    /// Available data fields
    pub fields: Vec<DataField>,
    /// Connection status
    pub connected: bool,
}

/// Type of data source
#[derive(Clone, Debug, PartialEq)]
pub enum DataSourceType {
    StaticData,
    ApiEndpoint,
    LocalStorage,
    UserInput,
    SystemState,
    ComponentState,
}

/// Data field in a data source
#[derive(Clone, Debug)]
pub struct DataField {
    /// Field name
    pub name: String,
    /// Field type
    pub field_type: String,
    /// Field description
    pub description: String,
    /// Whether field is required
    pub required: bool,
}

/// Binding validator for checking binding validity
pub struct BindingValidator {
    /// Validator name
    pub name: String,
    /// Validation rules
    pub rules: Vec<ValidationRule>,
}

/// Validation rule for bindings
#[derive(Clone, Debug)]
pub struct ValidationRule {
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Rule severity
    pub severity: ValidationSeverity,
}

/// Validation severity levels
#[derive(Clone, Debug, PartialEq)]
pub enum ValidationSeverity {
    Error,
    Warning,
    Info,
}

/// Tabs for enhanced property inspector UI
#[derive(Clone, Debug, PartialEq)]
enum InspectorTab {
    Properties,
    MultiSelection,
    DesignSystem,
    AiSuggestions,
    Animations,
    Bindings,
    Context,
}

impl Default for EnhancedPropertyInspector {
    fn default() -> Self {
        Self {
            multi_selection: MultiSelectionManager::new(),
            design_system: DesignSystemIntegration::new(),
            ai_suggestions: AiPropertySuggestions::new(),
            animation_system: PropertyAnimationSystem::default(),
            contextual_panels: ContextualPanelManager::default(),
            property_bindings: PropertyBindingSystem::default(),
            active_tab: InspectorTab::Properties,
            enhanced_features_enabled: true,
        }
    }
}

impl Default for PropertyAnimationSystem {
    fn default() -> Self {
        let mut system = Self {
            active_animations: Vec::new(),
            presets: HashMap::new(),
            transition_settings: TransitionSettings::default(),
        };
        
        system.initialize_default_presets();
        system
    }
}

impl Default for TransitionSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            default_duration: 300,
            animated_properties: HashSet::from([
                "opacity".to_string(),
                "color".to_string(),
                "background_color".to_string(),
                "width".to_string(),
                "height".to_string(),
            ]),
        }
    }
}

impl Default for ContextualPanelManager {
    fn default() -> Self {
        let mut manager = Self {
            panels: HashMap::new(),
            active_panels: Vec::new(),
            visibility_rules: Vec::new(),
        };
        
        manager.initialize_default_panels();
        manager
    }
}

impl Default for PropertyBindingSystem {
    fn default() -> Self {
        Self {
            bindings: HashMap::new(),
            data_sources: HashMap::new(),
            validators: Vec::new(),
        }
    }
}

impl PropertyAnimationSystem {
    /// Initialize default animation presets
    fn initialize_default_presets(&mut self) {
        self.presets.insert("fade".to_string(), AnimationPreset {
            name: "Fade".to_string(),
            duration: 300,
            easing: EasingFunction::EaseInOut,
            applicable_properties: vec!["opacity".to_string()],
        });
        
        self.presets.insert("slide".to_string(), AnimationPreset {
            name: "Slide".to_string(),
            duration: 400,
            easing: EasingFunction::EaseOut,
            applicable_properties: vec!["position".to_string(), "transform".to_string()],
        });
        
        self.presets.insert("bounce".to_string(), AnimationPreset {
            name: "Bounce".to_string(),
            duration: 600,
            easing: EasingFunction::Bounce,
            applicable_properties: vec!["scale".to_string(), "transform".to_string()],
        });
    }
    
    /// Start a property animation
    pub fn animate_property(
        &mut self,
        component_id: usize,
        property_name: String,
        from_value: PropertyValue,
        to_value: PropertyValue,
        duration: Option<u64>,
    ) {
        let animation_duration = duration.unwrap_or(self.transition_settings.default_duration);
        
        self.active_animations.push(PropertyAnimation {
            component_id,
            property_name,
            start_value: from_value,
            end_value: to_value,
            duration: animation_duration,
            start_time: std::time::Instant::now(),
            easing: EasingFunction::EaseInOut,
        });
    }
    
    /// Update animations and return completed ones
    pub fn update_animations(&mut self) -> Vec<PropertyAnimation> {
        let now = std::time::Instant::now();
        let mut completed = Vec::new();
        
        self.active_animations.retain(|animation| {
            let elapsed = now.duration_since(animation.start_time).as_millis() as u64;
            
            if elapsed >= animation.duration {
                completed.push(animation.clone());
                false
            } else {
                true
            }
        });
        
        completed
    }
}

impl ContextualPanelManager {
    /// Initialize default contextual panels
    fn initialize_default_panels(&mut self) {
        // Layout panel for layout-related properties
        self.panels.insert("layout".to_string(), ContextualPanel {
            name: "Layout".to_string(),
            description: "Position and size properties".to_string(),
            applicable_components: vec![], // Apply to all
            properties: vec![
                ContextualProperty {
                    name: "width".to_string(),
                    label: "Width".to_string(),
                    description: "Component width".to_string(),
                    property_type: PropertyType::Number { min: 0.0, max: 2000.0, step: 1.0 },
                    required: false,
                    default_value: Some(PropertyValue::Number(100.0)),
                    group: "Size".to_string(),
                },
                ContextualProperty {
                    name: "height".to_string(),
                    label: "Height".to_string(),
                    description: "Component height".to_string(),
                    property_type: PropertyType::Number { min: 0.0, max: 2000.0, step: 1.0 },
                    required: false,
                    default_value: Some(PropertyValue::Number(30.0)),
                    group: "Size".to_string(),
                },
            ],
            priority: 100,
            collapsible: true,
        });
        
        // Appearance panel for visual properties
        self.panels.insert("appearance".to_string(), ContextualPanel {
            name: "Appearance".to_string(),
            description: "Visual styling properties".to_string(),
            applicable_components: vec![], // Apply to all
            properties: vec![
                ContextualProperty {
                    name: "background_color".to_string(),
                    label: "Background".to_string(),
                    description: "Background color".to_string(),
                    property_type: PropertyType::Color,
                    required: false,
                    default_value: Some(PropertyValue::Color([255, 255, 255, 255])),
                    group: "Colors".to_string(),
                },
                ContextualProperty {
                    name: "border_radius".to_string(),
                    label: "Border Radius".to_string(),
                    description: "Corner roundness".to_string(),
                    property_type: PropertyType::Number { min: 0.0, max: 50.0, step: 1.0 },
                    required: false,
                    default_value: Some(PropertyValue::Number(0.0)),
                    group: "Borders".to_string(),
                },
            ],
            priority: 90,
            collapsible: true,
        });
    }
    
    /// Update active panels based on selection
    pub fn update_active_panels(&mut self, selected_components: &[ComponentSelection]) {
        self.active_panels.clear();
        
        if selected_components.is_empty() {
            return;
        }
        
        // Determine which panels should be active
        for (panel_name, panel) in &self.panels {
            let mut should_show = true;
            
            // Check if panel applies to all selected components
            if !panel.applicable_components.is_empty() {
                should_show = selected_components.iter().all(|comp| {
                    panel.applicable_components.contains(&comp.component_type)
                });
            }
            
            if should_show {
                self.active_panels.push(panel_name.clone());
            }
        }
        
        // Sort by priority
        self.active_panels.sort_by(|a, b| {
            let priority_a = self.panels.get(a).map(|p| p.priority).unwrap_or(0);
            let priority_b = self.panels.get(b).map(|p| p.priority).unwrap_or(0);
            priority_b.cmp(&priority_a) // Higher priority first
        });
    }
}

impl EnhancedPropertyInspector {
    /// Create a new enhanced property inspector
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Update inspector with current selection
    pub fn update_selection(&mut self, selected_components: Vec<ComponentSelection>) {
        // Update multi-selection manager
        self.multi_selection.clear_selection();
        self.multi_selection.add_to_selection(selected_components.clone());
        
        // Update contextual panels
        self.contextual_panels.update_active_panels(&selected_components);
        
        // Generate AI suggestions for the selection
        if let Some(first_component) = selected_components.first() {
            for (property_name, property_value) in &first_component.properties {
                let context = ai_suggestions::SuggestionContext {
                    component_type: first_component.component_type.clone(),
                    nearby_components: vec![],
                    container_context: HashMap::new(),
                    recent_actions: vec![],
                    design_phase: ai_suggestions::DesignPhase::Styling,
                    target_platform: vec!["web".to_string()],
                };
                
                self.ai_suggestions.generate_suggestions(&context, property_name, property_value);
            }
        }
        
        // Validate against design system
        for component in &selected_components {
            self.design_system.validator.validate_component(
                component.component_id,
                &component.component_type,
                &component.properties,
                &self.design_system.design_tokens,
            );
        }
    }
    
    /// Set property for selected components
    pub fn set_property(&mut self, property_name: &str, value: PropertyValue) -> MultiEditResult {
        let result = self.multi_selection.set_property_for_all(property_name, value.clone());
        
        // Start animation if property supports it
        if self.animation_system.transition_settings.enabled &&
           self.animation_system.transition_settings.animated_properties.contains(property_name) {
            
            for component in &self.multi_selection.selected_components {
                if let Some(old_value) = component.properties.get(property_name) {
                    self.animation_system.animate_property(
                        component.component_id,
                        property_name.to_string(),
                        old_value.clone(),
                        value.clone(),
                        None,
                    );
                }
            }
        }
        
        result
    }
    
    /// Render enhanced property inspector UI
    pub fn render_ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            // Header with enhanced features toggle
            ui.horizontal(|ui| {
                ui.heading("Enhanced Property Inspector");
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.checkbox(&mut self.enhanced_features_enabled, "Enhanced");
                });
            });
            
            ui.separator();
            
            if !self.enhanced_features_enabled {
                // Fall back to basic property inspector
                self.render_basic_properties(ui);
                return;
            }
            
            // Tab bar
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.active_tab, InspectorTab::Properties, "📝 Properties");
                ui.selectable_value(&mut self.active_tab, InspectorTab::MultiSelection, "⚏ Multi-Select");
                ui.selectable_value(&mut self.active_tab, InspectorTab::DesignSystem, "🎨 Design System");
                ui.selectable_value(&mut self.active_tab, InspectorTab::AiSuggestions, "🧠 AI Suggestions");
                ui.selectable_value(&mut self.active_tab, InspectorTab::Animations, "🎬 Animations");
                ui.selectable_value(&mut self.active_tab, InspectorTab::Bindings, "🔗 Bindings");
            });
            
            ui.separator();
            
            // Tab content
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    match self.active_tab {
                        InspectorTab::Properties => self.render_contextual_properties(ui),
                        InspectorTab::MultiSelection => self.multi_selection.render_ui(ui),
                        InspectorTab::DesignSystem => self.design_system.render_ui(ui),
                        InspectorTab::AiSuggestions => self.ai_suggestions.render_ui(ui),
                        InspectorTab::Animations => self.render_animations_tab(ui),
                        InspectorTab::Bindings => self.render_bindings_tab(ui),
                        InspectorTab::Context => self.render_context_tab(ui),
                    }
                });
        });
    }
    
    /// Render basic properties without enhanced features
    fn render_basic_properties(&self, ui: &mut Ui) {
        if self.multi_selection.selected_components.is_empty() {
            ui.label("No components selected");
            return;
        }
        
        // Simple property list
        for common_property in &self.multi_selection.common_properties {
            ui.horizontal(|ui| {
                ui.label(&common_property.name);
                ui.label(format!("{:?}", common_property.representative_value));
            });
        }
    }
    
    /// Render contextual properties panel
    fn render_contextual_properties(&mut self, ui: &mut Ui) {
        if self.contextual_panels.active_panels.is_empty() {
            ui.label("No contextual panels available");
            return;
        }
        
        for panel_name in &self.contextual_panels.active_panels.clone() {
            if let Some(panel) = self.contextual_panels.panels.get(panel_name) {
                ui.collapsing(&panel.name, |ui| {
                    ui.label(&panel.description);
                    
                    // Group properties by group
                    let mut groups: HashMap<String, Vec<&ContextualProperty>> = HashMap::new();
                    for property in &panel.properties {
                        groups.entry(property.group.clone()).or_default().push(property);
                    }
                    
                    for (group_name, properties) in groups {
                        if !group_name.is_empty() {
                            ui.group(|ui| {
                                ui.label(&group_name);
                                for property in properties {
                                    self.render_contextual_property(ui, property);
                                }
                            });
                        } else {
                            for property in properties {
                                self.render_contextual_property(ui, property);
                            }
                        }
                    }
                });
            }
        }
    }
    
    /// Render a single contextual property
    fn render_contextual_property(&mut self, ui: &mut Ui, property: &ContextualProperty) {
        ui.horizontal(|ui| {
            ui.label(&property.label);
            
            // Get current value from selection
            let current_value = self.multi_selection.get_common_property(&property.name)
                .map(|cp| cp.representative_value.clone())
                .unwrap_or_else(|| property.default_value.clone().unwrap_or(PropertyValue::String("".to_string())));
            
            // Render editor based on property type
            match &property.property_type {
                PropertyType::String { .. } => {
                    if let PropertyValue::String(text) = &current_value {
                        let mut text_edit = text.clone();
                        if ui.text_edit_singleline(&mut text_edit).changed() {
                            let _ = self.set_property(&property.name, PropertyValue::String(text_edit));
                        }
                    }
                }
                PropertyType::Number { min, max, step } => {
                    if let PropertyValue::Number(number) = &current_value {
                        let mut number_edit = *number;
                        if ui.add(egui::Slider::new(&mut number_edit, *min..=*max).step_by(*step as f64)).changed() {
                            let _ = self.set_property(&property.name, PropertyValue::Number(number_edit));
                        }
                    }
                }
                PropertyType::Boolean => {
                    if let PropertyValue::Boolean(boolean) = &current_value {
                        let mut boolean_edit = *boolean;
                        if ui.checkbox(&mut boolean_edit, "").changed() {
                            let _ = self.set_property(&property.name, PropertyValue::Boolean(boolean_edit));
                        }
                    }
                }
                PropertyType::Color => {
                    if let PropertyValue::Color(color) = &current_value {
                        let mut color32 = Color32::from_rgba_premultiplied(color[0], color[1], color[2], color[3]);
                        if ui.color_edit_button_srgba(&mut color32).changed() {
                            let color_array = [color32.r(), color32.g(), color32.b(), color32.a()];
                            let _ = self.set_property(&property.name, PropertyValue::Color(color_array));
                        }
                    }
                }
                PropertyType::Enum { options } => {
                    if let PropertyValue::Enum(selected) = &current_value {
                        egui::ComboBox::from_id_source(&property.name)
                            .selected_text(selected)
                            .show_ui(ui, |ui| {
                                for option in options {
                                    if ui.selectable_label(option == selected, option).clicked() {
                                        let _ = self.set_property(&property.name, PropertyValue::Enum(option.clone()));
                                    }
                                }
                            });
                    }
                }
            }
            
            // Show mixed value indicator
            if self.multi_selection.has_mixed_values(&property.name) {
                ui.colored_label(Color32::YELLOW, "Mixed");
            }
        });
    }
    
    /// Render animations tab
    fn render_animations_tab(&mut self, ui: &mut Ui) {
        ui.heading("Property Animations");
        
        // Animation settings
        ui.checkbox(&mut self.animation_system.transition_settings.enabled, "Enable property transitions");
        
        if self.animation_system.transition_settings.enabled {
            ui.horizontal(|ui| {
                ui.label("Default duration:");
                ui.add(egui::Slider::new(&mut self.animation_system.transition_settings.default_duration, 100..=2000).suffix("ms"));
            });
        }
        
        // Active animations
        if !self.animation_system.active_animations.is_empty() {
            ui.separator();
            ui.label("Active Animations:");
            
            for animation in &self.animation_system.active_animations {
                ui.horizontal(|ui| {
                    ui.label(format!("Component {}: {}", animation.component_id, animation.property_name));
                    
                    let elapsed = animation.start_time.elapsed().as_millis() as u64;
                    let progress = (elapsed as f32 / animation.duration as f32).min(1.0);
                    ui.add(egui::ProgressBar::new(progress));
                });
            }
        }
        
        // Animation presets
        ui.separator();
        ui.label("Animation Presets:");
        
        for (preset_name, preset) in &self.animation_system.presets {
            ui.horizontal(|ui| {
                ui.label(preset_name);
                ui.label(format!("{}ms", preset.duration));
                ui.label(format!("{:?}", preset.easing));
                
                if ui.button("Apply").clicked() {
                    // Apply preset to selected components
                }
            });
        }
    }
    
    /// Render property bindings tab
    fn render_bindings_tab(&mut self, ui: &mut Ui) {
        ui.heading("Property Bindings");
        
        // Data sources
        ui.collapsing("Data Sources", |ui| {
            if self.property_bindings.data_sources.is_empty() {
                ui.label("No data sources configured");
            } else {
                for (source_name, source) in &self.property_bindings.data_sources {
                    ui.horizontal(|ui| {
                        let status_color = if source.connected { Color32::GREEN } else { Color32::RED };
                        ui.colored_label(status_color, "●");
                        ui.label(source_name);
                        ui.label(format!("{:?}", source.source_type));
                    });
                }
            }
            
            if ui.button("+ Add Data Source").clicked() {
                // Add new data source
            }
        });
        
        // Active bindings
        ui.collapsing("Active Bindings", |ui| {
            if self.property_bindings.bindings.is_empty() {
                ui.label("No property bindings configured");
            } else {
                for (binding_name, binding) in &self.property_bindings.bindings {
                    ui.horizontal(|ui| {
                        ui.label(binding_name);
                        ui.label(format!("{} → {}", binding.source_property, binding.target_property));
                        
                        if binding.bidirectional {
                            ui.label("↔");
                        } else {
                            ui.label("→");
                        }
                        
                        if ui.button("Remove").clicked() {
                            // Remove binding
                        }
                    });
                }
            }
            
            if ui.button("+ Create Binding").clicked() {
                // Create new binding
            }
        });
    }
    
    /// Render context tab
    fn render_context_tab(&self, ui: &mut Ui) {
        ui.heading("Context Information");
        
        if let Some(component) = self.multi_selection.selected_components.first() {
            ui.group(|ui| {
                ui.label("Selected Component");
                ui.label(format!("Type: {}", component.component_type));
                ui.label(format!("ID: {}", component.component_id));
                ui.label(format!("Properties: {}", component.properties.len()));
            });
        }
        
        // Show contextual information
        ui.separator();
        ui.label("Design Context");
        ui.label("• Current phase: Styling");
        ui.label("• Target platform: Web");
        ui.label("• Responsive mode: Desktop");
    }
}