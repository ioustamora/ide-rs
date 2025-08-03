//! Advanced Property Inspector for editing component properties
//!
//! This module provides a comprehensive property editing system with:
//! - Typed property system with validation
//! - Real-time visual updates
//! - Component-specific property panels
//! - Property grouping and categorization
//! - Undo/redo support for property changes

use egui::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::rcl::ui::component::Component;

/// Advanced property inspector with typed property system
pub struct PropertyInspector {
    /// Property definitions for different component types
    pub property_definitions: HashMap<String, Vec<PropertyDefinition>>,
    /// Current property values for selected components
    pub current_values: HashMap<String, PropertyValue>,
    /// Property change history for undo/redo
    pub change_history: Vec<PropertyChange>,
    /// Current history position
    pub history_index: usize,
    /// Whether to show advanced properties
    pub show_advanced: bool,
    /// Property search filter
    pub search_filter: String,
    /// Expanded property groups
    pub expanded_groups: HashMap<String, bool>,
    /// Live preview mode
    pub live_preview: bool,
    /// Property validation errors
    pub validation_errors: HashMap<String, String>,
}

/// Definition of a component property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyDefinition {
    /// Property name
    pub name: String,
    /// Property display label
    pub label: String,
    /// Property description/tooltip
    pub description: String,
    /// Property type and constraints
    pub property_type: PropertyType,
    /// Property group/category
    pub group: String,
    /// Whether property is advanced (hidden by default)
    pub advanced: bool,
    /// Default value
    pub default_value: PropertyValue,
    /// Whether property is read-only
    pub read_only: bool,
}

/// Types of properties with their constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyType {
    /// Text string with optional max length
    String { max_length: Option<usize> },
    /// Numeric value with range
    Number { min: f64, max: f64, step: f64 },
    /// Integer value with range
    Integer { min: i32, max: i32, step: i32 },
    /// Boolean checkbox
    Boolean,
    /// Color picker
    Color,
    /// Font selection
    Font,
    /// Enum selection with options
    Enum { options: Vec<String> },
    /// File path selection
    FilePath { extensions: Vec<String> },
    /// Position (x, y)
    Position,
    /// Size (width, height)
    Size,
    /// Rectangle (x, y, width, height)
    Rectangle,
    /// Custom property type
    Custom { type_name: String },
}

/// Property value container
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PropertyValue {
    String(String),
    Number(f64),
    Integer(i32),
    Boolean(bool),
    Color([u8; 4]), // RGBA
    Font(String),
    Enum(String),
    FilePath(String),
    Position(f32, f32),
    Size(f32, f32),
    Rectangle(f32, f32, f32, f32),
    Custom(String),
    None,
}

/// Property change for undo/redo
#[derive(Debug, Clone)]
pub struct PropertyChange {
    pub component_id: usize,
    pub property_name: String,
    pub old_value: PropertyValue,
    pub new_value: PropertyValue,
    pub timestamp: std::time::Instant,
}

/// Property groups for organization
#[derive(Debug, Clone)]
pub enum PropertyGroup {
    Appearance,
    Layout,
    Behavior,
    Data,
    Animation,
    Advanced,
    Custom(String),
}

impl PropertyGroup {
    fn name(&self) -> &str {
        match self {
            PropertyGroup::Appearance => "Appearance",
            PropertyGroup::Layout => "Layout",
            PropertyGroup::Behavior => "Behavior",
            PropertyGroup::Data => "Data",
            PropertyGroup::Animation => "Animation",
            PropertyGroup::Advanced => "Advanced",
            PropertyGroup::Custom(name) => name,
        }
    }
    
    fn icon(&self) -> &str {
        match self {
            PropertyGroup::Appearance => "🎨",
            PropertyGroup::Layout => "📐",
            PropertyGroup::Behavior => "⚡",
            PropertyGroup::Data => "📊",
            PropertyGroup::Animation => "🎬",
            PropertyGroup::Advanced => "🔧",
            PropertyGroup::Custom(_) => "📋",
        }
    }
    
    fn from_name(name: &str) -> Self {
        match name {
            "Appearance" => PropertyGroup::Appearance,
            "Layout" => PropertyGroup::Layout,
            "Behavior" => PropertyGroup::Behavior,
            "Data" => PropertyGroup::Data,
            "Animation" => PropertyGroup::Animation,
            "Advanced" => PropertyGroup::Advanced,
            custom => PropertyGroup::Custom(custom.to_string()),
        }
    }
}

impl Default for PropertyInspector {
    fn default() -> Self {
        let mut inspector = Self {
            property_definitions: HashMap::new(),
            current_values: HashMap::new(),
            change_history: Vec::new(),
            history_index: 0,
            show_advanced: false,
            search_filter: String::new(),
            expanded_groups: HashMap::new(),
            live_preview: true,
            validation_errors: HashMap::new(),
        };
        
        // Initialize with default property definitions
        inspector.initialize_default_properties();
        inspector
    }
}

impl PropertyInspector {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Initialize default property definitions for built-in components
    fn initialize_default_properties(&mut self) {
        // Button properties
        self.add_component_properties("Button", vec![
            PropertyDefinition {
                name: "label".to_string(),
                label: "Label".to_string(),
                description: "Text displayed on the button".to_string(),
                property_type: PropertyType::String { max_length: Some(100) },
                group: "Appearance".to_string(),
                advanced: false,
                default_value: PropertyValue::String("Button".to_string()),
                read_only: false,
            },
            PropertyDefinition {
                name: "enabled".to_string(),
                label: "Enabled".to_string(),
                description: "Whether the button can be clicked".to_string(),
                property_type: PropertyType::Boolean,
                group: "Behavior".to_string(),
                advanced: false,
                default_value: PropertyValue::Boolean(true),
                read_only: false,
            },
            PropertyDefinition {
                name: "width".to_string(),
                label: "Width".to_string(),
                description: "Button width in pixels".to_string(),
                property_type: PropertyType::Number { min: 10.0, max: 1000.0, step: 1.0 },
                group: "Layout".to_string(),
                advanced: false,
                default_value: PropertyValue::Number(80.0),
                read_only: false,
            },
            PropertyDefinition {
                name: "height".to_string(),
                label: "Height".to_string(),
                description: "Button height in pixels".to_string(),
                property_type: PropertyType::Number { min: 10.0, max: 200.0, step: 1.0 },
                group: "Layout".to_string(),
                advanced: false,
                default_value: PropertyValue::Number(30.0),
                read_only: false,
            },
            PropertyDefinition {
                name: "background_color".to_string(),
                label: "Background Color".to_string(),
                description: "Button background color".to_string(),
                property_type: PropertyType::Color,
                group: "Appearance".to_string(),
                advanced: false,
                default_value: PropertyValue::Color([70, 130, 200, 255]),
                read_only: false,
            },
        ]);
        
        // Label properties
        self.add_component_properties("Label", vec![
            PropertyDefinition {
                name: "text".to_string(),
                label: "Text".to_string(),
                description: "Text content of the label".to_string(),
                property_type: PropertyType::String { max_length: Some(500) },
                group: "Appearance".to_string(),
                advanced: false,
                default_value: PropertyValue::String("Label".to_string()),
                read_only: false,
            },
            PropertyDefinition {
                name: "font_size".to_string(),
                label: "Font Size".to_string(),
                description: "Size of the text font".to_string(),
                property_type: PropertyType::Number { min: 8.0, max: 72.0, step: 1.0 },
                group: "Appearance".to_string(),
                advanced: false,
                default_value: PropertyValue::Number(14.0),
                read_only: false,
            },
            PropertyDefinition {
                name: "text_color".to_string(),
                label: "Text Color".to_string(),
                description: "Color of the text".to_string(),
                property_type: PropertyType::Color,
                group: "Appearance".to_string(),
                advanced: false,
                default_value: PropertyValue::Color([255, 255, 255, 255]),
                read_only: false,
            },
            PropertyDefinition {
                name: "alignment".to_string(),
                label: "Text Alignment".to_string(),
                description: "How the text is aligned".to_string(),
                property_type: PropertyType::Enum { 
                    options: vec!["Left".to_string(), "Center".to_string(), "Right".to_string()]
                },
                group: "Layout".to_string(),
                advanced: false,
                default_value: PropertyValue::Enum("Left".to_string()),
                read_only: false,
            },
        ]);
        
        // TextBox properties
        self.add_component_properties("TextBox", vec![
            PropertyDefinition {
                name: "value".to_string(),
                label: "Value".to_string(),
                description: "Current text value".to_string(),
                property_type: PropertyType::String { max_length: Some(1000) },
                group: "Data".to_string(),
                advanced: false,
                default_value: PropertyValue::String(String::new()),
                read_only: false,
            },
            PropertyDefinition {
                name: "placeholder".to_string(),
                label: "Placeholder".to_string(),
                description: "Placeholder text when empty".to_string(),
                property_type: PropertyType::String { max_length: Some(100) },
                group: "Appearance".to_string(),
                advanced: false,
                default_value: PropertyValue::String("Enter text...".to_string()),
                read_only: false,
            },
            PropertyDefinition {
                name: "multiline".to_string(),
                label: "Multiline".to_string(),
                description: "Allow multiple lines of text".to_string(),
                property_type: PropertyType::Boolean,
                group: "Behavior".to_string(),
                advanced: false,
                default_value: PropertyValue::Boolean(false),
                read_only: false,
            },
            PropertyDefinition {
                name: "readonly".to_string(),
                label: "Read Only".to_string(),
                description: "Prevent text editing".to_string(),
                property_type: PropertyType::Boolean,
                group: "Behavior".to_string(),
                advanced: false,
                default_value: PropertyValue::Boolean(false),
                read_only: false,
            },
        ]);
        
        // Form properties (root container)
        self.add_component_properties("Form", vec![
            PropertyDefinition {
                name: "title".to_string(),
                label: "Title".to_string(),
                description: "Form title or name".to_string(),
                property_type: PropertyType::String { max_length: Some(100) },
                group: "Appearance".to_string(),
                advanced: false,
                default_value: PropertyValue::String("Form".to_string()),
                read_only: false,
            },
            PropertyDefinition {
                name: "background_color".to_string(),
                label: "Background Color".to_string(),
                description: "Form background color".to_string(),
                property_type: PropertyType::Enum { 
                    options: vec!["white".to_string(), "black".to_string(), "red".to_string(), 
                                "green".to_string(), "blue".to_string(), "gray".to_string(), "light_gray".to_string()]
                },
                group: "Appearance".to_string(),
                advanced: false,
                default_value: PropertyValue::Enum("white".to_string()),
                read_only: false,
            },
            PropertyDefinition {
                name: "border_color".to_string(),
                label: "Border Color".to_string(),
                description: "Form border color".to_string(),
                property_type: PropertyType::Enum { 
                    options: vec!["white".to_string(), "black".to_string(), "red".to_string(), 
                                "green".to_string(), "blue".to_string(), "gray".to_string(), "light_gray".to_string()]
                },
                group: "Appearance".to_string(),
                advanced: false,
                default_value: PropertyValue::Enum("light_gray".to_string()),
                read_only: false,
            },
            PropertyDefinition {
                name: "border_width".to_string(),
                label: "Border Width".to_string(),
                description: "Width of the form border".to_string(),
                property_type: PropertyType::Number { min: 0.0, max: 10.0, step: 0.5 },
                group: "Appearance".to_string(),
                advanced: false,
                default_value: PropertyValue::Number(1.0),
                read_only: false,
            },
            PropertyDefinition {
                name: "show_border".to_string(),
                label: "Show Border".to_string(),
                description: "Whether to display the form border".to_string(),
                property_type: PropertyType::Boolean,
                group: "Appearance".to_string(),
                advanced: false,
                default_value: PropertyValue::Boolean(true),
                read_only: false,
            },
            PropertyDefinition {
                name: "corner_radius".to_string(),
                label: "Corner Radius".to_string(),
                description: "Roundness of form corners".to_string(),
                property_type: PropertyType::Number { min: 0.0, max: 50.0, step: 1.0 },
                group: "Appearance".to_string(),
                advanced: false,
                default_value: PropertyValue::Number(4.0),
                read_only: false,
            },
            PropertyDefinition {
                name: "width".to_string(),
                label: "Width".to_string(),
                description: "Form width in pixels".to_string(),
                property_type: PropertyType::Number { min: 200.0, max: 2000.0, step: 10.0 },
                group: "Layout".to_string(),
                advanced: false,
                default_value: PropertyValue::Number(400.0),
                read_only: false,
            },
            PropertyDefinition {
                name: "height".to_string(),
                label: "Height".to_string(),
                description: "Form height in pixels".to_string(),
                property_type: PropertyType::Number { min: 150.0, max: 1500.0, step: 10.0 },
                group: "Layout".to_string(),
                advanced: false,
                default_value: PropertyValue::Number(300.0),
                read_only: false,
            },
            PropertyDefinition {
                name: "padding".to_string(),
                label: "Padding".to_string(),
                description: "Internal spacing around form content".to_string(),
                property_type: PropertyType::Number { min: 0.0, max: 100.0, step: 1.0 },
                group: "Layout".to_string(),
                advanced: false,
                default_value: PropertyValue::Number(16.0),
                read_only: false,
            },
            PropertyDefinition {
                name: "visible".to_string(),
                label: "Visible".to_string(),
                description: "Whether the form is visible".to_string(),
                property_type: PropertyType::Boolean,
                group: "Behavior".to_string(),
                advanced: false,
                default_value: PropertyValue::Boolean(true),
                read_only: false,
            },
        ]);
        
        // Initialize expanded groups
        self.expanded_groups.insert("Appearance".to_string(), true);
        self.expanded_groups.insert("Layout".to_string(), true);
        self.expanded_groups.insert("Behavior".to_string(), false);
        self.expanded_groups.insert("Data".to_string(), false);
    }
    
    /// Add property definitions for a component type
    pub fn add_component_properties(&mut self, component_type: &str, properties: Vec<PropertyDefinition>) {
        self.property_definitions.insert(component_type.to_string(), properties);
    }
    
    /// Render the property inspector UI
    pub fn ui(&mut self, ui: &mut Ui, selected_component: Option<(usize, &dyn Component)>) {
        ui.vertical(|ui| {
            // Header with search and controls
            self.render_header(ui);
            
            ui.separator();
            
            if let Some((component_id, component)) = selected_component {
                // Component info
                ui.horizontal(|ui| {
                    ui.label("📦");
                    ui.strong(component.name());
                    ui.label(format!("(ID: {})", component_id));
                });
                
                ui.separator();
                
                // Property panels grouped by category
                self.render_property_groups(ui, component_id, component);
            } else {
                // No selection state
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.label("🔍");
                    ui.label("No component selected");
                    ui.add_space(10.0);
                    ui.label("Select a component in the design canvas\nto edit its properties");
                });
            }
        });
    }
    
    /// Render the header with search and controls
    fn render_header(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("🔧");
            ui.heading("Properties");
            
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                // Live preview toggle
                if ui.selectable_label(self.live_preview, "👁").on_hover_text("Live Preview").clicked() {
                    self.live_preview = !self.live_preview;
                }
                
                // Advanced properties toggle
                if ui.selectable_label(self.show_advanced, "⚙").on_hover_text("Show Advanced").clicked() {
                    self.show_advanced = !self.show_advanced;
                }
            });
        });
        
        // Search filter
        ui.horizontal(|ui| {
            ui.label("🔍");
            ui.text_edit_singleline(&mut self.search_filter);
            if ui.button("✖").on_hover_text("Clear Search").clicked() {
                self.search_filter.clear();
            }
        });
    }
    
    /// Render property groups for the selected component
    fn render_property_groups(&mut self, ui: &mut Ui, component_id: usize, component: &dyn Component) {
        let component_type = component.name();
        
        if let Some(properties) = self.property_definitions.get(component_type).cloned() {
            // Group properties by category
            let mut grouped_properties: HashMap<String, Vec<PropertyDefinition>> = HashMap::new();
            
            for prop in properties {
                // Apply search filter
                if !self.search_filter.is_empty() {
                    let search_lower = self.search_filter.to_lowercase();
                    if !prop.name.to_lowercase().contains(&search_lower) &&
                       !prop.label.to_lowercase().contains(&search_lower) &&
                       !prop.description.to_lowercase().contains(&search_lower) {
                        continue;
                    }
                }
                
                // Skip advanced properties if not shown
                if prop.advanced && !self.show_advanced {
                    continue;
                }
                
                grouped_properties.entry(prop.group.clone()).or_insert_with(Vec::new).push(prop);
            }
            
            // Render each group
            for (group_name, group_properties) in &grouped_properties {
                self.render_property_group(ui, component_id, group_name, group_properties);
            }
        } else {
            ui.label(format!("No properties defined for {}", component_type));
        }
    }
    
    /// Render a single property group with animated collapse
    fn render_property_group(&mut self, ui: &mut Ui, component_id: usize, group_name: &str, properties: &[PropertyDefinition], animation_manager: &mut crate::ide_app::animated_ui::AnimationManager) {
        use crate::ide_app::animated_ui::AnimatedCollapsing;
        
        let group = PropertyGroup::from_name(group_name);
        let group_id = egui::Id::new(format!("prop_group_{}", group_name));
        
        AnimatedCollapsing::new(
            group_id,
            format!("{} {}", group.icon(), group.name()),
            animation_manager
        )
        .default_open(true)
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 4.0; // Consistent spacing between properties
            
            for property in properties {
                self.render_property_editor(ui, component_id, property);
                ui.add_space(2.0); // Small space between properties
            }
        });
        
        ui.add_space(4.0); // Space between groups
    }
    
    /// Render editor for a single property
    fn render_property_editor(&mut self, ui: &mut Ui, component_id: usize, property: &PropertyDefinition) {
        let property_key = format!("{}_{}", component_id, property.name);
        let current_value = self.current_values.get(&property_key).cloned()
            .unwrap_or_else(|| property.default_value.clone());
        
        // Use stable ID for each property row to prevent animation issues
        let row_id = egui::Id::new(format!("prop_row_{}", property_key));
        
        ui.push_id(row_id, |ui| {
            // Set fixed height for each property row to prevent vertical shifting
            let row_height = ui.spacing().button_padding.y * 2.0 + ui.text_style_height(&egui::TextStyle::Body);
            ui.allocate_ui_with_layout(
                egui::Vec2::new(ui.available_width(), row_height),
                Layout::left_to_right(Align::Center),
                |ui| {
                    // Property label with fixed width to prevent layout shifts
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(120.0, row_height),
                        Layout::left_to_right(Align::Center),
                        |ui| {
                            ui.label(&property.label)
                                .on_hover_text(&property.description);
                        }
                    );
                    
                    // Property editor with stable layout
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(ui.available_width().max(150.0), row_height),
                        Layout::left_to_right(Align::Center),
                        |ui| {
                            // Property editor based on type with stable ID
                            let editor_id = egui::Id::new(format!("prop_editor_{}", property_key));
                            ui.push_id(editor_id, |ui| {
                                let new_value = self.render_property_value_editor(ui, &property.property_type, &current_value, &property_key);
                                
                                // Only update value if it actually changed to avoid constant redraws
                                if new_value != current_value {
                                    self.set_property_value(component_id, &property.name, new_value);
                                }
                                
                                // Show validation error if any with fixed layout
                                if let Some(error) = self.validation_errors.get(&property_key) {
                                    ui.allocate_ui_with_layout(
                                        egui::Vec2::new(20.0, row_height),
                                        Layout::right_to_left(Align::Center),
                                        |ui| {
                                            ui.label("⚠")
                                                .on_hover_text(error)
                                                .on_hover_ui(|ui| {
                                                    ui.colored_label(egui::Color32::RED, error);
                                                });
                                        }
                                    );
                                }
                            });
                        }
                    );
                }
            );
        });
    }
    
    /// Render value editor based on property type
    fn render_property_value_editor(&self, ui: &mut Ui, property_type: &PropertyType, current_value: &PropertyValue, property_key: &str) -> PropertyValue {
        match (property_type, current_value) {
            (PropertyType::String { max_length }, PropertyValue::String(value)) => {
                let mut text = value.clone();
                let _response = ui.add(egui::TextEdit::singleline(&mut text)
                    .desired_width(150.0)  // Fixed width to prevent layout shifts
                    .id(egui::Id::new(format!("text_{}", property_key))));
                
                // Apply max length constraint
                if let Some(max_len) = max_length {
                    if text.len() > *max_len {
                        text.truncate(*max_len);
                    }
                }
                
                PropertyValue::String(text)
            }
            
            (PropertyType::Number { min, max, step }, PropertyValue::Number(value)) => {
                let mut num_value = *value;
                ui.add(Slider::new(&mut num_value, *min..=*max)
                    .step_by(*step)
                    .fixed_decimals(1)
                    .drag_value_speed(0.1));
                PropertyValue::Number(num_value)
            }
            
            (PropertyType::Integer { min, max, step }, PropertyValue::Integer(value)) => {
                let mut int_value = *value;
                ui.add(Slider::new(&mut int_value, *min..=*max)
                    .step_by(*step as f64)
                    .fixed_decimals(0)
                    .drag_value_speed(0.5));
                PropertyValue::Integer(int_value)
            }
            
            (PropertyType::Boolean, PropertyValue::Boolean(value)) => {
                let mut bool_value = *value;
                ui.add(egui::Checkbox::new(&mut bool_value, ""));
                PropertyValue::Boolean(bool_value)
            }
            
            (PropertyType::Color, PropertyValue::Color(rgba)) => {
                let mut color = Color32::from_rgba_unmultiplied(rgba[0], rgba[1], rgba[2], rgba[3]);
                ui.horizontal(|ui| {
                    ui.color_edit_button_srgba(&mut color);
                    ui.label(format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b()));
                });
                let [r, g, b, a] = color.to_array();
                PropertyValue::Color([r, g, b, a])
            }
            
            (PropertyType::Enum { options }, PropertyValue::Enum(value)) => {
                let mut selected = value.clone();
                ComboBox::from_id_source(format!("enum_{}", property_key))
                    .selected_text(&selected)
                    .width(150.0)  // Fixed width to prevent layout shifts
                    .show_ui(ui, |ui| {
                        for option in options {
                            ui.selectable_value(&mut selected, option.clone(), option);
                        }
                    });
                PropertyValue::Enum(selected)
            }
            
            _ => {
                ui.label("Unsupported property type");
                current_value.clone()
            }
        }
    }
    
    /// Set a property value with validation and history
    pub fn set_property_value(&mut self, component_id: usize, property_name: &str, new_value: PropertyValue) {
        let property_key = format!("{}_{}", component_id, property_name);
        let old_value = self.current_values.get(&property_key).cloned().unwrap_or(PropertyValue::None);
        
        // Validate the new value
        if let Err(error) = self.validate_property_value(component_id, property_name, &new_value) {
            self.validation_errors.insert(property_key.clone(), error);
            return;
        } else {
            self.validation_errors.remove(&property_key);
        }
        
        // Store the change in history
        let change = PropertyChange {
            component_id,
            property_name: property_name.to_string(),
            old_value,
            new_value: new_value.clone(),
            timestamp: std::time::Instant::now(),
        };
        
        // Truncate history if we're not at the end
        self.change_history.truncate(self.history_index);
        self.change_history.push(change);
        self.history_index = self.change_history.len();
        
        // Update current value
        self.current_values.insert(property_key, new_value);
    }
    
    /// Validate a property value
    fn validate_property_value(&self, _component_id: usize, property_name: &str, value: &PropertyValue) -> Result<(), String> {
        // Basic validation - can be extended with more sophisticated rules
        match (property_name, value) {
            ("width" | "height", PropertyValue::Number(n)) if *n <= 0.0 => {
                Err("Size must be positive".to_string())
            }
            ("font_size", PropertyValue::Number(n)) if *n < 1.0 => {
                Err("Font size must be at least 1".to_string())
            }
            _ => Ok(())
        }
    }
    
    /// Get current property value
    pub fn get_property_value(&self, component_id: usize, property_name: &str) -> Option<&PropertyValue> {
        let property_key = format!("{}_{}", component_id, property_name);
        self.current_values.get(&property_key)
    }
    
    /// Undo last property change
    pub fn undo(&mut self) -> bool {
        if self.history_index == 0 {
            return false;
        }
        
        self.history_index -= 1;
        let change = &self.change_history[self.history_index];
        
        let property_key = format!("{}_{}", change.component_id, change.property_name);
        self.current_values.insert(property_key, change.old_value.clone());
        
        true
    }
    
    /// Redo next property change
    pub fn redo(&mut self) -> bool {
        if self.history_index >= self.change_history.len() {
            return false;
        }
        
        let change = &self.change_history[self.history_index];
        let property_key = format!("{}_{}", change.component_id, change.property_name);
        self.current_values.insert(property_key, change.new_value.clone());
        
        self.history_index += 1;
        true
    }

    /// Render component properties (wrapper for UI method)
    pub fn render_component_properties(&mut self, ui: &mut Ui, component: &mut Box<dyn Component>) {
        // Get the actual component index for proper identification
        let component_id = 0; // This would normally be passed in, but for now use 0
        self.ui(ui, Some((component_id, component.as_ref())));
    }
}