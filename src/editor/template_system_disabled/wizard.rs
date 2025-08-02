//! Template Creation Wizard
//!
//! Interactive wizard for creating and editing component templates.

use egui::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::template::{ComponentTemplate, TemplateProperty, PropertyType, PropertyValue, PropertyValidation};

/// Template creation wizard state
#[derive(Debug, Clone)]
pub struct TemplateWizard {
    /// Current wizard step
    pub current_step: WizardStep,
    /// Template being created/edited
    pub template: ComponentTemplate,
    /// Wizard UI state
    pub ui_state: WizardUiState,
    /// Available parent templates
    pub available_parents: Vec<String>,
    /// Property being edited
    pub editing_property: Option<PropertyEditor>,
    /// Validation results
    pub validation_results: ValidationResults,
    /// Whether wizard is open
    pub is_open: bool,
}

/// Wizard steps
#[derive(Debug, Clone, PartialEq)]
pub enum WizardStep {
    /// Basic template information
    BasicInfo,
    /// Parent template selection
    ParentSelection,
    /// Property definition
    PropertyDefinition,
    /// Layout configuration
    LayoutConfiguration,
    /// Visual settings
    VisualSettings,
    /// Behavior settings
    BehaviorSettings,
    /// Review and finish
    Review,
}

/// Wizard UI state
#[derive(Debug, Clone)]
pub struct WizardUiState {
    /// Current tab in property editor
    pub property_tab: PropertyTab,
    /// Selected property for editing
    pub selected_property: Option<String>,
    /// Show advanced options
    pub show_advanced: bool,
    /// Filter text for parent selection
    pub parent_filter: String,
    /// Preview mode enabled
    pub preview_enabled: bool,
}

/// Property editor tabs
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyTab {
    Definition,
    Validation,
    Documentation,
    Advanced,
}

/// Property editor state
#[derive(Debug, Clone)]
pub struct PropertyEditor {
    /// Property being edited
    pub property: TemplateProperty,
    /// Original property name (for renames)
    pub original_name: Option<String>,
    /// Whether this is a new property
    pub is_new: bool,
    /// Property preview value
    pub preview_value: PropertyValue,
}

/// Validation results for wizard
#[derive(Debug, Clone)]
pub struct ValidationResults {
    /// Template name validation
    pub name_valid: bool,
    /// Template ID validation
    pub id_valid: bool,
    /// Property validations
    pub property_validations: HashMap<String, PropertyValidationResult>,
    /// Overall validation status
    pub is_valid: bool,
    /// Validation messages
    pub messages: Vec<ValidationMessage>,
}

/// Property validation result
#[derive(Debug, Clone)]
pub struct PropertyValidationResult {
    /// Property is valid
    pub valid: bool,
    /// Validation errors
    pub errors: Vec<String>,
    /// Validation warnings
    pub warnings: Vec<String>,
}

/// Validation message
#[derive(Debug, Clone)]
pub struct ValidationMessage {
    /// Message type
    pub message_type: MessageType,
    /// Message text
    pub message: String,
    /// Associated field
    pub field: Option<String>,
}

/// Message types
#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    Info,
    Warning,
    Error,
}

impl TemplateWizard {
    /// Create a new template wizard
    pub fn new() -> Self {
        Self {
            current_step: WizardStep::BasicInfo,
            template: ComponentTemplate::new("".to_string(), "".to_string()),
            ui_state: WizardUiState::new(),
            available_parents: Vec::new(),
            editing_property: None,
            validation_results: ValidationResults::new(),
            is_open: false,
        }
    }
    
    /// Start creating a new template
    pub fn start_new_template(&mut self) {
        self.template = ComponentTemplate::new("".to_string(), "".to_string());
        self.current_step = WizardStep::BasicInfo;
        self.editing_property = None;
        self.validation_results = ValidationResults::new();
        self.is_open = true;
    }
    
    /// Start editing an existing template
    pub fn start_edit_template(&mut self, template: ComponentTemplate) {
        self.template = template;
        self.current_step = WizardStep::BasicInfo;
        self.editing_property = None;
        self.validation_results = ValidationResults::new();
        self.is_open = true;
        self.validate_current_step();
    }
    
    /// Render the wizard UI
    pub fn render(&mut self, ctx: &Context) {
        if !self.is_open {
            return;
        }
        
        Window::new("Template Wizard")
            .resizable(true)
            .default_width(600.0)
            .default_height(500.0)
            .show(ctx, |ui| {
                self.render_wizard_content(ui);
            });
    }
    
    /// Render wizard content
    fn render_wizard_content(&mut self, ui: &mut Ui) {
        // Step indicator
        self.render_step_indicator(ui);
        ui.separator();
        
        // Current step content
        match self.current_step {
            WizardStep::BasicInfo => self.render_basic_info(ui),
            WizardStep::ParentSelection => self.render_parent_selection(ui),
            WizardStep::PropertyDefinition => self.render_property_definition(ui),
            WizardStep::LayoutConfiguration => self.render_layout_configuration(ui),
            WizardStep::VisualSettings => self.render_visual_settings(ui),
            WizardStep::BehaviorSettings => self.render_behavior_settings(ui),
            WizardStep::Review => self.render_review(ui),
        }
        
        ui.separator();
        
        // Navigation buttons
        self.render_navigation_buttons(ui);
    }
    
    /// Render step indicator
    fn render_step_indicator(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let steps = [
                ("Basic", WizardStep::BasicInfo),
                ("Parent", WizardStep::ParentSelection),
                ("Properties", WizardStep::PropertyDefinition),
                ("Layout", WizardStep::LayoutConfiguration),
                ("Visual", WizardStep::VisualSettings),
                ("Behavior", WizardStep::BehaviorSettings),
                ("Review", WizardStep::Review),
            ];
            
            for (i, (name, step)) in steps.iter().enumerate() {
                if i > 0 {
                    ui.label("→");
                }
                
                let is_current = &self.current_step == step;
                let is_completed = self.is_step_completed(step);
                
                let color = if is_current {
                    Color32::YELLOW
                } else if is_completed {
                    Color32::GREEN
                } else {
                    Color32::GRAY
                };
                
                ui.colored_label(color, name);
            }
        });
    }
    
    /// Check if a step is completed
    fn is_step_completed(&self, step: &WizardStep) -> bool {
        // TODO: Implement step completion logic
        false
    }
    
    /// Render basic info step
    fn render_basic_info(&mut self, ui: &mut Ui) {
        ui.heading("Basic Template Information");
        
        ui.horizontal(|ui| {
            ui.label("Template Name:");
            ui.text_edit_singleline(&mut self.template.name);
        });
        
        ui.horizontal(|ui| {
            ui.label("Template ID:");
            ui.text_edit_singleline(&mut self.template.id);
        });
        
        ui.horizontal(|ui| {
            ui.label("Category:");
            ui.text_edit_singleline(&mut self.template.category);
        });
        
        ui.horizontal(|ui| {
            ui.label("Version:");
            ui.text_edit_singleline(&mut self.template.version);
        });
        
        ui.label("Description:");
        ui.text_edit_multiline(&mut self.template.description);
        
        // Validation feedback
        if !self.validation_results.name_valid && !self.template.name.is_empty() {
            ui.colored_label(Color32::RED, "Invalid template name");
        }
        if !self.validation_results.id_valid && !self.template.id.is_empty() {
            ui.colored_label(Color32::RED, "Invalid template ID");
        }
    }
    
    /// Render parent selection step
    fn render_parent_selection(&mut self, ui: &mut Ui) {
        ui.heading("Parent Template Selection");
        
        ui.label("Select a parent template (optional):");
        
        // Filter input
        ui.horizontal(|ui| {
            ui.label("Filter:");
            ui.text_edit_singleline(&mut self.ui_state.parent_filter);
        });
        
        // Parent template list
        ScrollArea::vertical().show(ui, |ui| {
            ui.radio_value(&mut self.template.parent_template, None, "No parent (base template)");
            
            for parent_id in &self.available_parents {
                if self.ui_state.parent_filter.is_empty() || 
                   parent_id.to_lowercase().contains(&self.ui_state.parent_filter.to_lowercase()) {
                    ui.radio_value(
                        &mut self.template.parent_template,
                        Some(parent_id.clone()),
                        parent_id
                    );
                }
            }
        });
    }
    
    /// Render property definition step
    fn render_property_definition(&mut self, ui: &mut Ui) {
        ui.heading("Property Definition");
        
        ui.horizontal(|ui| {
            if ui.button("Add Property").clicked() {
                self.start_property_editor(None);
            }
            
            if ui.button("Remove Selected").clicked() {
                if let Some(selected) = &self.ui_state.selected_property {
                    self.template.remove_property(selected);
                    self.ui_state.selected_property = None;
                }
            }
        });
        
        // Property list
        ui.separator();
        ui.label("Properties:");
        
        ScrollArea::vertical().show(ui, |ui| {
            let property_names: Vec<String> = self.template.properties.keys().cloned().collect();
            for name in property_names {
                ui.horizontal(|ui| {
                    let is_selected = self.ui_state.selected_property.as_ref() == Some(&name);
                    if ui.selectable_label(is_selected, &name).clicked() {
                        self.ui_state.selected_property = Some(name.clone());
                    }
                    
                    if ui.small_button("Edit").clicked() {
                        self.start_property_editor(Some(name.clone()));
                    }
                });
            }
        });
        
        // Property editor
        if let Some(editor) = &mut self.editing_property {
            ui.separator();
            self.render_property_editor(ui, editor);
        }
    }
    
    /// Start property editor
    fn start_property_editor(&mut self, property_name: Option<String>) {
        if let Some(name) = property_name {
            if let Some(property) = self.template.get_property(&name) {
                self.editing_property = Some(PropertyEditor {
                    property: property.clone(),
                    original_name: Some(name),
                    is_new: false,
                    preview_value: property.default_value.clone(),
                });
            }
        } else {
            self.editing_property = Some(PropertyEditor {
                property: TemplateProperty {
                    name: "new_property".to_string(),
                    property_type: PropertyType::String,
                    default_value: PropertyValue::String("".to_string()),
                    overridable: true,
                    inherited: false,
                    validation: PropertyValidation::default(),
                    description: "".to_string(),
                    category: "General".to_string(),
                    designer_visible: true,
                },
                original_name: None,
                is_new: true,
                preview_value: PropertyValue::String("".to_string()),
            });
        }
    }
    
    /// Render property editor
    fn render_property_editor(&mut self, ui: &mut Ui, editor: &mut PropertyEditor) {
        ui.collapsing("Property Editor", |ui| {
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut editor.property.name);
            });
            
            ui.horizontal(|ui| {
                ui.label("Type:");
                ComboBox::from_label("")
                    .selected_text(format!("{:?}", editor.property.property_type))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut editor.property.property_type, PropertyType::String, "String");
                        ui.selectable_value(&mut editor.property.property_type, PropertyType::Integer, "Integer");
                        ui.selectable_value(&mut editor.property.property_type, PropertyType::Float, "Float");
                        ui.selectable_value(&mut editor.property.property_type, PropertyType::Boolean, "Boolean");
                        ui.selectable_value(&mut editor.property.property_type, PropertyType::Color, "Color");
                    });
            });
            
            ui.checkbox(&mut editor.property.overridable, "Overridable");
            ui.checkbox(&mut editor.property.designer_visible, "Visible in Designer");
            
            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    self.save_property_editor();
                }
                if ui.button("Cancel").clicked() {
                    self.editing_property = None;
                }
            });
        });
    }
    
    /// Save property editor changes
    fn save_property_editor(&mut self) {
        if let Some(editor) = &self.editing_property {
            self.template.add_property(editor.property.clone());
            self.editing_property = None;
        }
    }
    
    /// Render layout configuration step
    fn render_layout_configuration(&mut self, ui: &mut Ui) {
        ui.heading("Layout Configuration");
        ui.label("Layout configuration options will be implemented here");
    }
    
    /// Render visual settings step
    fn render_visual_settings(&mut self, ui: &mut Ui) {
        ui.heading("Visual Settings");
        ui.label("Visual settings configuration will be implemented here");
    }
    
    /// Render behavior settings step
    fn render_behavior_settings(&mut self, ui: &mut Ui) {
        ui.heading("Behavior Settings");
        ui.label("Behavior settings configuration will be implemented here");
    }
    
    /// Render review step
    fn render_review(&mut self, ui: &mut Ui) {
        ui.heading("Review Template");
        
        ui.label(format!("Name: {}", self.template.name));
        ui.label(format!("ID: {}", self.template.id));
        ui.label(format!("Category: {}", self.template.category));
        ui.label(format!("Properties: {}", self.template.properties.len()));
        
        if let Some(parent) = &self.template.parent_template {
            ui.label(format!("Parent: {}", parent));
        }
    }
    
    /// Render navigation buttons
    fn render_navigation_buttons(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("Cancel").clicked() {
                self.is_open = false;
            }
            
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let is_last_step = matches!(self.current_step, WizardStep::Review);
                
                if is_last_step {
                    if ui.button("Finish").clicked() {
                        self.finish_wizard();
                    }
                } else {
                    if ui.button("Next").clicked() {
                        self.next_step();
                    }
                }
                
                if !matches!(self.current_step, WizardStep::BasicInfo) {
                    if ui.button("Previous").clicked() {
                        self.previous_step();
                    }
                }
            });
        });
    }
    
    /// Move to next step
    fn next_step(&mut self) {
        self.validate_current_step();
        if !self.validation_results.is_valid {
            return;
        }
        
        self.current_step = match self.current_step {
            WizardStep::BasicInfo => WizardStep::ParentSelection,
            WizardStep::ParentSelection => WizardStep::PropertyDefinition,
            WizardStep::PropertyDefinition => WizardStep::LayoutConfiguration,
            WizardStep::LayoutConfiguration => WizardStep::VisualSettings,
            WizardStep::VisualSettings => WizardStep::BehaviorSettings,
            WizardStep::BehaviorSettings => WizardStep::Review,
            WizardStep::Review => WizardStep::Review, // Stay on last step
        };
    }
    
    /// Move to previous step
    fn previous_step(&mut self) {
        self.current_step = match self.current_step {
            WizardStep::BasicInfo => WizardStep::BasicInfo, // Stay on first step
            WizardStep::ParentSelection => WizardStep::BasicInfo,
            WizardStep::PropertyDefinition => WizardStep::ParentSelection,
            WizardStep::LayoutConfiguration => WizardStep::PropertyDefinition,
            WizardStep::VisualSettings => WizardStep::LayoutConfiguration,
            WizardStep::BehaviorSettings => WizardStep::VisualSettings,
            WizardStep::Review => WizardStep::BehaviorSettings,
        };
    }
    
    /// Validate current step
    fn validate_current_step(&mut self) {
        self.validation_results = ValidationResults::new();
        
        match self.current_step {
            WizardStep::BasicInfo => {
                self.validation_results.name_valid = !self.template.name.trim().is_empty();
                self.validation_results.id_valid = !self.template.id.trim().is_empty() &&
                    self.template.id.chars().all(|c| c.is_alphanumeric() || c == '_');
                
                self.validation_results.is_valid = 
                    self.validation_results.name_valid && self.validation_results.id_valid;
            }
            _ => {
                self.validation_results.is_valid = true;
            }
        }
    }
    
    /// Finish wizard and create template
    fn finish_wizard(&mut self) {
        // Final validation
        self.validate_current_step();
        if self.validation_results.is_valid {
            self.is_open = false;
            // Template is ready in self.template
        }
    }
}

impl WizardUiState {
    fn new() -> Self {
        Self {
            property_tab: PropertyTab::Definition,
            selected_property: None,
            show_advanced: false,
            parent_filter: String::new(),
            preview_enabled: false,
        }
    }
}

impl ValidationResults {
    fn new() -> Self {
        Self {
            name_valid: true,
            id_valid: true,
            property_validations: HashMap::new(),
            is_valid: true,
            messages: Vec::new(),
        }
    }
}

impl Default for PropertyValidation {
    fn default() -> Self {
        Self {
            required: false,
            min_value: None,
            max_value: None,
            pattern: None,
            valid_options: None,
            custom_validation: None,
        }
    }
}

impl Default for TemplateWizard {
    fn default() -> Self {
        Self::new()
    }
}