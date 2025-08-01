//! Component Template and Inheritance System
//!
//! Professional RAD template system inspired by Embarcadero RAD Studio's form inheritance.
//! Enables creation of reusable component templates and property inheritance chains.
//!
//! Features:
//! - Component template creation and management
//! - Property inheritance with override capabilities
//! - Template versioning and compatibility
//! - Visual template editor integration
//! - Form inheritance similar to Delphi VCL

use egui::*;
use std::collections::{HashMap, BTreeMap};
use serde::{Deserialize, Serialize};

/// Template system manager
pub struct TemplateSystem {
    /// Available component templates
    pub templates: HashMap<String, ComponentTemplate>,
    /// Template inheritance relationships
    pub inheritance_tree: InheritanceTree,
    /// Template creation wizard state
    pub template_wizard: TemplateWizard,
    /// Recently used templates
    pub recent_templates: Vec<String>,
    /// Template categories for organization
    pub categories: HashMap<String, Vec<String>>,
    /// Template validation results
    pub validation_cache: HashMap<String, ValidationResult>,
}

/// Component template definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentTemplate {
    /// Template unique identifier
    pub id: String,
    /// Template display name
    pub name: String,
    /// Template description
    pub description: String,
    /// Template category
    pub category: String,
    /// Template version
    pub version: String,
    /// Parent template ID (for inheritance)
    pub parent_template: Option<String>,
    /// Component properties definition
    pub properties: BTreeMap<String, TemplateProperty>,
    /// Default component layout
    pub layout: TemplateLayout,
    /// Template creation metadata
    pub metadata: TemplateMetadata,
    /// Visual appearance settings
    pub visual_settings: VisualSettings,
    /// Behavioral settings
    pub behavior_settings: BehaviorSettings,
    /// Template usage statistics
    pub usage_stats: UsageStatistics,
}

/// Template property definition with inheritance support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateProperty {
    /// Property name
    pub name: String,
    /// Property type
    pub property_type: PropertyType,
    /// Default value
    pub default_value: PropertyValue,
    /// Whether property can be overridden in child templates
    pub overridable: bool,
    /// Whether property is inherited from parent
    pub inherited: bool,
    /// Property validation rules
    pub validation: PropertyValidation,
    /// Property description/documentation
    pub description: String,
    /// Property category for grouping
    pub category: String,
    /// Property visibility in designer
    pub designer_visible: bool,
}

/// Property type system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyType {
    String,
    Integer,
    Float,
    Boolean,
    Color,
    Font,
    Size,
    Position,
    Alignment,
    List(Vec<String>),
    Custom(String),
}

/// Property value container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyValue {
    String(String),
    Integer(i32),
    Float(f32),
    Boolean(bool),
    Color([u8; 4]),
    List(Vec<String>),
    Custom(String),
}

/// Property validation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyValidation {
    /// Property is required
    pub required: bool,
    /// Minimum value (for numbers)
    pub min_value: Option<f32>,
    /// Maximum value (for numbers)
    pub max_value: Option<f32>,
    /// Valid string pattern (regex)
    pub pattern: Option<String>,
    /// Valid options (for enums)
    pub valid_options: Option<Vec<String>>,
    /// Custom validation expression
    pub custom_validation: Option<String>,
}

/// Template layout definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateLayout {
    /// Layout type
    pub layout_type: LayoutType,
    /// Child component templates
    pub children: Vec<ChildTemplate>,
    /// Layout constraints
    pub constraints: LayoutConstraints,
    /// Responsive behavior
    pub responsive: ResponsiveSettings,
}

/// Layout type options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayoutType {
    Fixed,
    Flow,
    Grid,
    Flex,
    Stack,
    Custom(String),
}

/// Child template reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildTemplate {
    /// Child template ID
    pub template_id: String,
    /// Position within parent (x, y)
    pub position: (f32, f32),
    /// Size override (width, height)
    pub size: Option<(f32, f32)>,
    /// Property overrides
    pub property_overrides: BTreeMap<String, PropertyValue>,
    /// Z-order/layer
    pub z_order: i32,
}

/// Layout constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutConstraints {
    /// Minimum size (width, height)
    pub min_size: Option<(f32, f32)>,
    /// Maximum size (width, height)
    pub max_size: Option<(f32, f32)>,
    /// Aspect ratio constraints
    pub aspect_ratio: Option<f32>,
    /// Anchor points
    pub anchors: AnchorSettings,
}

/// Component anchor settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnchorSettings {
    pub left: bool,
    pub top: bool,
    pub right: bool,
    pub bottom: bool,
}

/// Responsive behavior settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsiveSettings {
    /// Enable responsive behavior
    pub enabled: bool,
    /// Breakpoint rules
    pub breakpoints: Vec<ResponsiveBreakpoint>,
    /// Auto-scaling behavior
    pub auto_scale: bool,
}

/// Responsive breakpoint definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsiveBreakpoint {
    /// Breakpoint name
    pub name: String,
    /// Minimum width
    pub min_width: f32,
    /// Property overrides at this breakpoint
    pub property_overrides: BTreeMap<String, PropertyValue>,
}

/// Template metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMetadata {
    /// Template author
    pub author: String,
    /// Creation date
    pub created_date: String,
    /// Last modified date
    pub modified_date: String,
    /// Template tags
    pub tags: Vec<String>,
    /// Compatibility version
    pub min_ide_version: String,
    /// Template dependencies
    pub dependencies: Vec<String>,
}

/// Visual appearance settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualSettings {
    /// Template icon
    pub icon: Option<String>,
    /// Preview thumbnail
    pub thumbnail: Option<String>,
    /// Designer grid settings
    pub grid_settings: GridSettings,
    /// Selection appearance
    pub selection_style: SelectionStyle,
}

/// Grid settings for template designer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridSettings {
    /// Enable grid
    pub enabled: bool,
    /// Grid size
    pub size: f32,
    /// Grid color
    pub color: [u8; 4],
    /// Snap to grid
    pub snap_enabled: bool,
}

/// Selection style in designer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionStyle {
    /// Selection border color
    pub border_color: [u8; 4],
    /// Selection border width
    pub border_width: f32,
    /// Selection handles color
    pub handle_color: [u8; 4],
    /// Handle size
    pub handle_size: f32,
}

/// Behavioral settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorSettings {
    /// Event handlers
    pub event_handlers: HashMap<String, String>,
    /// Animation settings
    pub animations: Vec<AnimationSetting>,
    /// State management
    pub state_properties: Vec<String>,
    /// Validation behavior
    pub validation_behavior: ValidationBehavior,
}

/// Animation setting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationSetting {
    /// Animation name
    pub name: String,
    /// Animation type
    pub animation_type: String,
    /// Duration in milliseconds
    pub duration: u32,
    /// Animation properties
    pub properties: HashMap<String, String>,
}

/// Validation behavior settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationBehavior {
    /// Validate on change
    pub validate_on_change: bool,
    /// Show validation errors inline
    pub inline_errors: bool,
    /// Validation error style
    pub error_style: String,
}

/// Template usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStatistics {
    /// Times used
    pub usage_count: u32,
    /// Last used date
    pub last_used: Option<String>,
    /// Average rating
    pub rating: f32,
    /// Usage contexts
    pub contexts: Vec<String>,
}

/// Template inheritance tree
pub struct InheritanceTree {
    /// Parent-child relationships
    pub relationships: HashMap<String, Vec<String>>,
    /// Root templates (no parents)
    pub root_templates: Vec<String>,
    /// Inheritance depth cache
    pub depth_cache: HashMap<String, usize>,
}

/// Template creation wizard
pub struct TemplateWizard {
    /// Current wizard step
    pub current_step: WizardStep,
    /// Template being created
    pub working_template: ComponentTemplate,
    /// Wizard state
    pub state: WizardState,
    /// Property editor state
    pub property_editor: PropertyEditor,
}

/// Wizard steps
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WizardStep {
    BasicInfo,
    ParentSelection,
    PropertyDefinition,
    LayoutDesign,
    VisualSettings,
    BehaviorSettings,
    Preview,
    Finish,
}

/// Wizard state data
#[derive(Debug, Clone)]
pub struct WizardState {
    /// Selected parent template
    pub selected_parent: Option<String>,
    /// Available parent templates
    pub available_parents: Vec<String>,
    /// Wizard completion progress
    pub completion_progress: f32,
    /// Validation errors
    pub validation_errors: Vec<String>,
}

/// Property editor for template properties
pub struct PropertyEditor {
    /// Currently editing property
    pub editing_property: Option<String>,
    /// Property values
    pub property_values: BTreeMap<String, PropertyValue>,
    /// Property categories
    pub categories: HashMap<String, Vec<String>>,
    /// Show advanced properties
    pub show_advanced: bool,
}

/// Template validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Validation passed
    pub valid: bool,
    /// Validation errors
    pub errors: Vec<ValidationError>,
    /// Validation warnings
    pub warnings: Vec<ValidationWarning>,
    /// Validation timestamp
    pub validated_at: String,
}

/// Validation error
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
    /// Error severity
    pub severity: ErrorSeverity,
    /// Related property/component
    pub context: String,
}

/// Validation warning
#[derive(Debug, Clone)]
pub struct ValidationWarning {
    /// Warning message
    pub message: String,
    /// Warning context
    pub context: String,
}

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    Critical,
    Error,
    Warning,
    Info,
}

impl Default for TemplateSystem {
    fn default() -> Self {
        Self {
            templates: HashMap::new(),
            inheritance_tree: InheritanceTree::default(),
            template_wizard: TemplateWizard::default(),
            recent_templates: Vec::new(),
            categories: Self::create_default_categories(),
            validation_cache: HashMap::new(),
        }
    }
}

impl Default for InheritanceTree {
    fn default() -> Self {
        Self {
            relationships: HashMap::new(),
            root_templates: Vec::new(),
            depth_cache: HashMap::new(),
        }
    }
}

impl Default for TemplateWizard {
    fn default() -> Self {
        Self {
            current_step: WizardStep::BasicInfo,
            working_template: ComponentTemplate::default(),
            state: WizardState::default(),
            property_editor: PropertyEditor::default(),
        }
    }
}

impl Default for WizardState {
    fn default() -> Self {
        Self {
            selected_parent: None,
            available_parents: Vec::new(),
            completion_progress: 0.0,
            validation_errors: Vec::new(),
        }
    }
}

impl Default for PropertyEditor {
    fn default() -> Self {
        Self {
            editing_property: None,
            property_values: BTreeMap::new(),
            categories: HashMap::new(),
            show_advanced: false,
        }
    }
}

impl Default for ComponentTemplate {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            description: String::new(),
            category: "General".to_string(),
            version: "1.0.0".to_string(),
            parent_template: None,
            properties: BTreeMap::new(),
            layout: TemplateLayout::default(),
            metadata: TemplateMetadata::default(),
            visual_settings: VisualSettings::default(),
            behavior_settings: BehaviorSettings::default(),
            usage_stats: UsageStatistics::default(),
        }
    }
}

impl Default for TemplateLayout {
    fn default() -> Self {
        Self {
            layout_type: LayoutType::Fixed,
            children: Vec::new(),
            constraints: LayoutConstraints::default(),
            responsive: ResponsiveSettings::default(),
        }
    }
}

impl Default for LayoutConstraints {
    fn default() -> Self {
        Self {
            min_size: None,
            max_size: None,
            aspect_ratio: None,
            anchors: AnchorSettings::default(),
        }
    }
}

impl Default for AnchorSettings {
    fn default() -> Self {
        Self {
            left: true,
            top: true,
            right: false,
            bottom: false,
        }
    }
}

impl Default for ResponsiveSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            breakpoints: Vec::new(),
            auto_scale: false,
        }
    }
}

impl Default for TemplateMetadata {
    fn default() -> Self {
        Self {
            author: "Unknown".to_string(),
            created_date: "".to_string(),
            modified_date: "".to_string(),
            tags: Vec::new(),
            min_ide_version: "1.0.0".to_string(),
            dependencies: Vec::new(),
        }
    }
}

impl Default for VisualSettings {
    fn default() -> Self {
        Self {
            icon: None,
            thumbnail: None,
            grid_settings: GridSettings::default(),
            selection_style: SelectionStyle::default(),
        }
    }
}

impl Default for GridSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            size: 8.0,
            color: [200, 200, 200, 128],
            snap_enabled: true,
        }
    }
}

impl Default for SelectionStyle {
    fn default() -> Self {
        Self {
            border_color: [0, 150, 255, 255],
            border_width: 2.0,
            handle_color: [0, 150, 255, 255],
            handle_size: 6.0,
        }
    }
}

impl Default for BehaviorSettings {
    fn default() -> Self {
        Self {
            event_handlers: HashMap::new(),
            animations: Vec::new(),
            state_properties: Vec::new(),
            validation_behavior: ValidationBehavior::default(),
        }
    }
}

impl Default for ValidationBehavior {
    fn default() -> Self {
        Self {
            validate_on_change: true,
            inline_errors: true,
            error_style: "highlight".to_string(),
        }
    }
}

impl Default for UsageStatistics {
    fn default() -> Self {
        Self {
            usage_count: 0,
            last_used: None,
            rating: 0.0,
            contexts: Vec::new(),
        }
    }
}

impl TemplateSystem {
    /// Create new template system
    pub fn new() -> Self {
        let mut system = Self::default();
        system.load_builtin_templates();
        system
    }

    /// Create default template categories
    fn create_default_categories() -> HashMap<String, Vec<String>> {
        let mut categories = HashMap::new();
        categories.insert("General".to_string(), Vec::new());
        categories.insert("Layout".to_string(), Vec::new());
        categories.insert("Input".to_string(), Vec::new());
        categories.insert("Display".to_string(), Vec::new());
        categories.insert("Navigation".to_string(), Vec::new());
        categories.insert("Data".to_string(), Vec::new());
        categories.insert("Custom".to_string(), Vec::new());
        categories
    }

    /// Load built-in component templates
    fn load_builtin_templates(&mut self) {
        // Button template
        let button_template = ComponentTemplate {
            id: "builtin.button".to_string(),
            name: "Button".to_string(),
            description: "Standard button component".to_string(),
            category: "Input".to_string(),
            version: "1.0.0".to_string(),
            parent_template: None,
            properties: self.create_button_properties(),
            layout: TemplateLayout::default(),
            metadata: TemplateMetadata {
                author: "System".to_string(),
                created_date: "2024-01-01".to_string(),
                modified_date: "2024-01-01".to_string(),
                tags: vec!["button".to_string(), "input".to_string()],
                min_ide_version: "1.0.0".to_string(),
                dependencies: Vec::new(),
            },
            visual_settings: VisualSettings::default(),
            behavior_settings: BehaviorSettings::default(),
            usage_stats: UsageStatistics::default(),
        };

        self.add_template(button_template);

        // Label template
        let label_template = ComponentTemplate {
            id: "builtin.label".to_string(),
            name: "Label".to_string(),
            description: "Text label component".to_string(),
            category: "Display".to_string(),
            version: "1.0.0".to_string(),
            parent_template: None,
            properties: self.create_label_properties(),
            layout: TemplateLayout::default(),
            metadata: TemplateMetadata {
                author: "System".to_string(),
                created_date: "2024-01-01".to_string(),
                modified_date: "2024-01-01".to_string(),
                tags: vec!["label".to_string(), "text".to_string()],
                min_ide_version: "1.0.0".to_string(),
                dependencies: Vec::new(),
            },
            visual_settings: VisualSettings::default(),
            behavior_settings: BehaviorSettings::default(),
            usage_stats: UsageStatistics::default(),
        };

        self.add_template(label_template);

        // Panel template
        let panel_template = ComponentTemplate {
            id: "builtin.panel".to_string(),
            name: "Panel".to_string(),
            description: "Container panel component".to_string(),
            category: "Layout".to_string(),
            version: "1.0.0".to_string(),
            parent_template: None,
            properties: self.create_panel_properties(),
            layout: TemplateLayout::default(),
            metadata: TemplateMetadata {
                author: "System".to_string(),
                created_date: "2024-01-01".to_string(),
                modified_date: "2024-01-01".to_string(),
                tags: vec!["panel".to_string(), "container".to_string()],
                min_ide_version: "1.0.0".to_string(),
                dependencies: Vec::new(),
            },
            visual_settings: VisualSettings::default(),
            behavior_settings: BehaviorSettings::default(),
            usage_stats: UsageStatistics::default(),
        };

        self.add_template(panel_template);
    }

    /// Create button template properties
    fn create_button_properties(&self) -> BTreeMap<String, TemplateProperty> {
        let mut properties = BTreeMap::new();

        properties.insert("text".to_string(), TemplateProperty {
            name: "text".to_string(),
            property_type: PropertyType::String,
            default_value: PropertyValue::String("Button".to_string()),
            overridable: true,
            inherited: false,
            validation: PropertyValidation {
                required: true,
                min_value: None,
                max_value: None,
                pattern: None,
                valid_options: None,
                custom_validation: None,
            },
            description: "Button text".to_string(),
            category: "Appearance".to_string(),
            designer_visible: true,
        });

        properties.insert("enabled".to_string(), TemplateProperty {
            name: "enabled".to_string(),
            property_type: PropertyType::Boolean,
            default_value: PropertyValue::Boolean(true),
            overridable: true,
            inherited: false,
            validation: PropertyValidation {
                required: false,
                min_value: None,
                max_value: None,
                pattern: None,
                valid_options: None,
                custom_validation: None,
            },
            description: "Whether button is enabled".to_string(),
            category: "Behavior".to_string(),
            designer_visible: true,
        });

        properties
    }

    /// Create label template properties
    fn create_label_properties(&self) -> BTreeMap<String, TemplateProperty> {
        let mut properties = BTreeMap::new();

        properties.insert("text".to_string(), TemplateProperty {
            name: "text".to_string(),
            property_type: PropertyType::String,
            default_value: PropertyValue::String("Label".to_string()),
            overridable: true,
            inherited: false,
            validation: PropertyValidation {
                required: true,
                min_value: None,
                max_value: None,
                pattern: None,
                valid_options: None,
                custom_validation: None,
            },
            description: "Label text".to_string(),
            category: "Appearance".to_string(),
            designer_visible: true,
        });

        properties.insert("alignment".to_string(), TemplateProperty {
            name: "alignment".to_string(),
            property_type: PropertyType::List(vec!["Left".to_string(), "Center".to_string(), "Right".to_string()]),
            default_value: PropertyValue::String("Left".to_string()),
            overridable: true,
            inherited: false,
            validation: PropertyValidation {
                required: false,
                min_value: None,
                max_value: None,
                pattern: None,
                valid_options: Some(vec!["Left".to_string(), "Center".to_string(), "Right".to_string()]),
                custom_validation: None,
            },
            description: "Text alignment".to_string(),
            category: "Appearance".to_string(),
            designer_visible: true,
        });

        properties
    }

    /// Create panel template properties
    fn create_panel_properties(&self) -> BTreeMap<String, TemplateProperty> {
        let mut properties = BTreeMap::new();

        properties.insert("background_color".to_string(), TemplateProperty {
            name: "background_color".to_string(),
            property_type: PropertyType::Color,
            default_value: PropertyValue::Color([240, 240, 240, 255]),
            overridable: true,
            inherited: false,
            validation: PropertyValidation {
                required: false,
                min_value: None,
                max_value: None,
                pattern: None,
                valid_options: None,
                custom_validation: None,
            },
            description: "Panel background color".to_string(),
            category: "Appearance".to_string(),
            designer_visible: true,
        });

        properties.insert("border_width".to_string(), TemplateProperty {
            name: "border_width".to_string(),
            property_type: PropertyType::Float,
            default_value: PropertyValue::Float(1.0),
            overridable: true,
            inherited: false,
            validation: PropertyValidation {
                required: false,
                min_value: Some(0.0),
                max_value: Some(10.0),
                pattern: None,
                valid_options: None,
                custom_validation: None,
            },
            description: "Border width in pixels".to_string(),
            category: "Appearance".to_string(),
            designer_visible: true,
        });

        properties
    }

    /// Add template to system
    pub fn add_template(&mut self, template: ComponentTemplate) {
        let template_id = template.id.clone();
        let category = template.category.clone();

        // Add to templates collection
        self.templates.insert(template_id.clone(), template);

        // Update category
        self.categories.entry(category).or_insert_with(Vec::new).push(template_id.clone());

        // Update inheritance tree
        self.inheritance_tree.add_template(&template_id);
    }

    /// Remove template from system
    pub fn remove_template(&mut self, template_id: &str) -> bool {
        if let Some(template) = self.templates.remove(template_id) {
            // Remove from category
            if let Some(category_templates) = self.categories.get_mut(&template.category) {
                category_templates.retain(|id| id != template_id);
            }

            // Update inheritance tree
            self.inheritance_tree.remove_template(template_id);

            // Remove from recent templates
            self.recent_templates.retain(|id| id != template_id);

            true
        } else {
            false
        }
    }

    /// Get template by ID
    pub fn get_template(&self, template_id: &str) -> Option<&ComponentTemplate> {
        self.templates.get(template_id)
    }

    /// Get template with inherited properties resolved
    /// 
    /// This algorithm implements recursive template inheritance resolution, similar to class
    /// inheritance in object-oriented programming. It walks up the inheritance chain,
    /// merging properties from parent templates while allowing child templates to override
    /// specific properties. This enables powerful template reuse and customization.
    pub fn get_resolved_template(&self, template_id: &str) -> Option<ComponentTemplate> {
        // Get the base template - return None if template doesn't exist
        let template = self.templates.get(template_id)?;
        // Start with a copy of the child template as our working template
        let mut resolved = template.clone();

        // Resolve inheritance chain recursively
        // This implements depth-first inheritance resolution
        if let Some(parent_id) = &template.parent_template {
            // Recursively resolve the parent template (which may itself have parents)
            // This creates a fully resolved inheritance chain from root to child
            if let Some(parent_resolved) = self.get_resolved_template(parent_id) {
                // Merge the resolved parent with this child template
                // Child properties override parent properties where allowed
                resolved = self.merge_templates(&parent_resolved, &resolved);
            }
        }

        // Return the fully resolved template with all inheritance applied
        Some(resolved)
    }

    /// Merge parent and child templates (inheritance resolution)
    /// 
    /// This is the core template inheritance algorithm that combines a parent template
    /// with a child template, respecting property override rules and maintaining the
    /// inheritance hierarchy. It implements the "child wins" strategy where child
    /// templates can selectively override parent properties while inheriting others.
    fn merge_templates(&self, parent: &ComponentTemplate, child: &ComponentTemplate) -> ComponentTemplate {
        // Start with the parent template as the base - this ensures we inherit all parent properties
        let mut merged = parent.clone();

        // Override core template metadata with child values
        // These fields are always overridden by the child to maintain template identity
        merged.id = child.id.clone();                         // Child template ID takes precedence
        merged.name = child.name.clone();                     // Child template name
        merged.description = child.description.clone();       // Child template description
        merged.version = child.version.clone();               // Child template version
        merged.parent_template = child.parent_template.clone(); // Maintain inheritance reference

        // Property merging with override rules
        // This is the critical part where inheritance behavior is determined
        for (prop_name, child_prop) in &child.properties {
            // Only allow override if the property is marked as overridable
            // This provides fine-grained control over which properties can be customized
            if child_prop.overridable {
                // Child property completely replaces parent property
                // This allows child templates to customize specific aspects while inheriting others
                merged.properties.insert(prop_name.clone(), child_prop.clone());
            }
            // If property is not overridable, parent property is preserved automatically
            // This enforces template constraints and ensures certain properties remain consistent
        }

        // Override appearance and behavior settings with child values
        // These settings are considered part of the child template's identity
        merged.visual_settings = child.visual_settings.clone();     // Child visual appearance
        merged.behavior_settings = child.behavior_settings.clone(); // Child behavior rules
        merged.metadata = child.metadata.clone();                   // Child metadata (author, dates, etc.)

        // Return the merged template with combined parent/child characteristics
        merged
    }

    /// Create new template from existing component
    pub fn create_template_from_component(&mut self, _component_data: &str) -> Result<String, String> {
        // Initialize template wizard
        self.template_wizard.current_step = WizardStep::BasicInfo;
        self.template_wizard.working_template = ComponentTemplate::default();

        // Generate unique ID using simple approach
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let template_id = format!("template_{}", timestamp);
        self.template_wizard.working_template.id = template_id.clone();

        Ok(template_id)
    }

    /// Render template system UI
    pub fn render_template_panel(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.heading("📋 Component Templates");
                
                if ui.button("➕ New Template").clicked() {
                    self.start_template_wizard();
                }
                
                if ui.button("📁 Import").clicked() {
                    // TODO: Import template from file
                }
                
                if ui.button("💾 Export").clicked() {
                    // TODO: Export selected templates
                }
            });

            ui.separator();

            // Template categories
            ui.horizontal(|ui| {
                ui.label("Category:");
                for (category, templates) in &self.categories {
                    if !templates.is_empty() {
                        if ui.selectable_label(false, category).clicked() {
                            // TODO: Filter by category
                        }
                    }
                }
            });

            ui.separator();

            // Template list
            ScrollArea::vertical().show(ui, |ui| {
                let template_items: Vec<(String, ComponentTemplate)> = self.templates.iter()
                    .map(|(id, template)| (id.clone(), template.clone()))
                    .collect();
                
                for (template_id, template) in template_items {
                    self.render_template_item(ui, &template_id, &template);
                }
            });
        });
    }

    /// Render individual template item
    fn render_template_item(&mut self, ui: &mut Ui, template_id: &str, template: &ComponentTemplate) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                // Template icon or placeholder
                ui.label("🧩");
                
                ui.vertical(|ui| {
                    ui.strong(&template.name);
                    ui.label(&template.description);
                    ui.horizontal(|ui| {
                        ui.small(format!("v{}", template.version));
                        ui.small(&template.category);
                        if let Some(parent) = &template.parent_template {
                            ui.small(format!("↳ {}", parent));
                        }
                    });
                });

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.small_button("🗑").on_hover_text("Delete template").clicked() {
                        // Mark for deletion
                    }
                    if ui.small_button("✏").on_hover_text("Edit template").clicked() {
                        self.edit_template(template_id);
                    }
                    if ui.small_button("📋").on_hover_text("Use template").clicked() {
                        self.use_template(template_id);
                    }
                });
            });

            // Show properties summary
            if ui.small_button(format!("📊 {} properties", template.properties.len())).clicked() {
                // TODO: Expand properties view
            }
        });

        ui.add_space(4.0);
    }

    /// Start template creation wizard
    pub fn start_template_wizard(&mut self) {
        self.template_wizard.current_step = WizardStep::BasicInfo;
        self.template_wizard.working_template = ComponentTemplate::default();
        self.template_wizard.state = WizardState::default();
    }

    /// Edit existing template
    pub fn edit_template(&mut self, template_id: &str) {
        if let Some(template) = self.templates.get(template_id).cloned() {
            self.template_wizard.working_template = template;
            self.template_wizard.current_step = WizardStep::BasicInfo;
        }
    }

    /// Use template to create component instance
    pub fn use_template(&mut self, template_id: &str) {
        if let Some(_template) = self.get_resolved_template(template_id) {
            // Add to recent templates
            self.recent_templates.retain(|id| id != template_id);
            self.recent_templates.insert(0, template_id.to_string());
            if self.recent_templates.len() > 10 {
                self.recent_templates.truncate(10);
            }

            // TODO: Create component instance from template
        }
    }

    /// Validate template
    pub fn validate_template(&mut self, template_id: &str) -> ValidationResult {
        if let Some(template) = self.templates.get(template_id) {
            let mut errors = Vec::new();
            let mut warnings = Vec::new();

            // Basic validation
            if template.name.is_empty() {
                errors.push(ValidationError {
                    code: "EMPTY_NAME".to_string(),
                    message: "Template name cannot be empty".to_string(),
                    severity: ErrorSeverity::Error,
                    context: "name".to_string(),
                });
            }

            if template.properties.is_empty() {
                warnings.push(ValidationWarning {
                    message: "Template has no properties defined".to_string(),
                    context: "properties".to_string(),
                });
            }

            // Check inheritance chain
            if let Some(parent_id) = &template.parent_template {
                if !self.templates.contains_key(parent_id) {
                    errors.push(ValidationError {
                        code: "MISSING_PARENT".to_string(),
                        message: format!("Parent template '{}' not found", parent_id),
                        severity: ErrorSeverity::Error,
                        context: "parent_template".to_string(),
                    });
                }
            }

            let result = ValidationResult {
                valid: errors.is_empty(),
                errors,
                warnings,
                validated_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
                    .to_string(),
            };

            // Cache result
            self.validation_cache.insert(template_id.to_string(), result.clone());

            result
        } else {
            ValidationResult {
                valid: false,
                errors: vec![ValidationError {
                    code: "NOT_FOUND".to_string(),
                    message: format!("Template '{}' not found", template_id),
                    severity: ErrorSeverity::Critical,
                    context: "template".to_string(),
                }],
                warnings: Vec::new(),
                validated_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
                    .to_string(),
            }
        }
    }

    /// Get templates by category
    pub fn get_templates_by_category(&self, category: &str) -> Vec<&ComponentTemplate> {
        if let Some(template_ids) = self.categories.get(category) {
            template_ids
                .iter()
                .filter_map(|id| self.templates.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Search templates by name or description
    pub fn search_templates(&self, query: &str) -> Vec<&ComponentTemplate> {
        let query_lower = query.to_lowercase();
        self.templates
            .values()
            .filter(|template| {
                template.name.to_lowercase().contains(&query_lower)
                    || template.description.to_lowercase().contains(&query_lower)
                    || template.metadata.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .collect()
    }
}

impl InheritanceTree {
    /// Add template to inheritance tree
    pub fn add_template(&mut self, template_id: &str) {
        if !self.relationships.contains_key(template_id) {
            self.relationships.insert(template_id.to_string(), Vec::new());
        }
        
        // Update root templates
        if !self.root_templates.contains(&template_id.to_string()) {
            self.root_templates.push(template_id.to_string());
        }
        
        // Clear depth cache
        self.depth_cache.clear();
    }

    /// Remove template from inheritance tree
    pub fn remove_template(&mut self, template_id: &str) {
        self.relationships.remove(template_id);
        self.root_templates.retain(|id| id != template_id);
        
        // Remove as child from all parents
        for children in self.relationships.values_mut() {
            children.retain(|id| id != template_id);
        }
        
        // Clear depth cache
        self.depth_cache.clear();
    }

    /// Add inheritance relationship
    pub fn add_inheritance(&mut self, parent_id: &str, child_id: &str) {
        self.relationships
            .entry(parent_id.to_string())
            .or_insert_with(Vec::new)
            .push(child_id.to_string());
        
        // Remove child from root templates
        self.root_templates.retain(|id| id != child_id);
        
        // Clear depth cache
        self.depth_cache.clear();
    }

    /// Get inheritance depth
    pub fn get_depth(&mut self, template_id: &str) -> usize {
        if let Some(&depth) = self.depth_cache.get(template_id) {
            return depth;
        }

        let depth = self.calculate_depth(template_id, 0);
        self.depth_cache.insert(template_id.to_string(), depth);
        depth
    }

    /// Calculate inheritance depth recursively
    /// 
    /// This algorithm calculates the maximum depth of the inheritance tree rooted at a given
    /// template. It uses recursive traversal to find the deepest inheritance path, which is
    /// useful for understanding template complexity and detecting potential circular references.
    fn calculate_depth(&self, template_id: &str, current_depth: usize) -> usize {
        // Check if this template has any child templates
        if let Some(children) = self.relationships.get(template_id) {
            // Recursively calculate depth for each child template
            // Each child starts at current_depth + 1 (one level deeper)
            children
                .iter()
                .map(|child_id| self.calculate_depth(child_id, current_depth + 1))
                .max()  // Take the maximum depth among all children
                .unwrap_or(current_depth)  // If no children, use current depth
        } else {
            // Leaf node - no children, so depth is current level
            current_depth
        }
    }

    /// Get all descendants of a template
    pub fn get_descendants(&self, template_id: &str) -> Vec<String> {
        let mut descendants = Vec::new();
        self.collect_descendants(template_id, &mut descendants);
        descendants
    }

    /// Collect descendants recursively
    /// 
    /// This algorithm performs a depth-first traversal of the inheritance tree to collect
    /// all descendant templates. It's used for operations like "find all templates that
    /// inherit from X" or for cleanup when deleting a template that has children.
    fn collect_descendants(&self, template_id: &str, descendants: &mut Vec<String>) {
        // Check if this template has any direct children
        if let Some(children) = self.relationships.get(template_id) {
            // Process each direct child
            for child_id in children {
                // Add the child to our descendants collection
                descendants.push(child_id.clone());
                // Recursively collect the child's descendants (grandchildren, etc.)
                // This implements depth-first traversal to capture the entire subtree
                self.collect_descendants(child_id, descendants);
            }
        }
        // If no children exist, recursion naturally terminates
    }
}