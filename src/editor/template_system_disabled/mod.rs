//! Component Template and Inheritance System
//!
//! Professional RAD template system inspired by Embarcadero RAD Studio's form inheritance.
//! Enables creation of reusable component templates and property inheritance chains.
//!
//! ## Features
//! - Component template creation and management
//! - Property inheritance with override capabilities
//! - Template versioning and compatibility
//! - Visual template editor integration
//! - Form inheritance similar to Delphi VCL
//!
//! ## Architecture
//! 
//! The template system is organized into focused modules:
//! - **template**: Core template and property definitions
//! - **inheritance**: Template inheritance tree and property resolution
//! - **wizard**: Interactive template creation wizard
//! - **validation**: Template and property validation system
//! - **serialization**: Template serialization and persistence
//! - **builtin_templates**: Default templates and components

pub mod template;
pub mod inheritance;
pub mod wizard;
pub mod validation;
pub mod serialization;
pub mod builtin_templates;

use std::collections::HashMap;
use egui::*;

// Re-export main types
pub use template::{ComponentTemplate, TemplateProperty, PropertyType, PropertyValue};
pub use inheritance::{InheritanceTree, InheritanceResult, InheritanceError};
pub use wizard::{TemplateWizard, WizardStep};
pub use validation::{ValidationResult, TemplateValidator};
pub use serialization::{TemplateSerializer, SerializationFormat};

/// Template system manager
/// 
/// Central coordinator for all template system functionality.
/// This is a much lighter version of the original monolithic structure.
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
    /// Template validation results cache
    pub validation_cache: HashMap<String, ValidationResult>,
    /// Template serializer
    pub serializer: TemplateSerializer,
    /// Template validator
    pub validator: TemplateValidator,
}

impl TemplateSystem {
    /// Create a new template system
    pub fn new() -> Self {
        let mut system = Self {
            templates: HashMap::new(),
            inheritance_tree: InheritanceTree::new(),
            template_wizard: TemplateWizard::new(),
            recent_templates: Vec::new(),
            categories: HashMap::new(),
            validation_cache: HashMap::new(),
            serializer: TemplateSerializer::new(),
            validator: TemplateValidator::new(),
        };
        
        // Load built-in templates
        system.load_builtin_templates();
        
        system
    }
    
    /// Load built-in templates
    fn load_builtin_templates(&mut self) {
        let builtin_templates = builtin_templates::create_builtin_templates();
        for template in builtin_templates {
            self.add_template(template);
        }
    }
    
    /// Add a template to the system
    pub fn add_template(&mut self, template: ComponentTemplate) {
        let template_id = template.id.clone();
        
        // Add to templates collection
        self.templates.insert(template_id.clone(), template.clone());
        
        // Update inheritance tree if template has parent
        if let Some(parent_id) = &template.parent_template {
            if let Err(err) = self.inheritance_tree.add_relationship(parent_id.clone(), template_id.clone()) {
                eprintln!("Failed to add inheritance relationship: {}", err);
            }
        }
        
        // Update categories
        self.categories
            .entry(template.category.clone())
            .or_insert_with(Vec::new)
            .push(template_id.clone());
        
        // Invalidate validation cache
        self.validation_cache.remove(&template_id);
    }
    
    /// Remove a template from the system
    pub fn remove_template(&mut self, template_id: &str) -> Option<ComponentTemplate> {
        if let Some(template) = self.templates.remove(template_id) {
            // Remove from inheritance tree
            if let Some(parent_id) = &template.parent_template {
                self.inheritance_tree.remove_relationship(parent_id, template_id);
            }
            
            // Remove from categories
            if let Some(templates) = self.categories.get_mut(&template.category) {
                templates.retain(|id| id != template_id);
                if templates.is_empty() {
                    self.categories.remove(&template.category);
                }
            }
            
            // Remove from recent templates
            self.recent_templates.retain(|id| id != template_id);
            
            // Remove from validation cache
            self.validation_cache.remove(template_id);
            
            Some(template)
        } else {
            None
        }
    }
    
    /// Get a template by ID
    pub fn get_template(&self, template_id: &str) -> Option<&ComponentTemplate> {
        self.templates.get(template_id)
    }
    
    /// Get templates by category
    pub fn get_templates_by_category(&self, category: &str) -> Vec<&ComponentTemplate> {
        self.categories
            .get(category)
            .map(|template_ids| {
                template_ids
                    .iter()
                    .filter_map(|id| self.templates.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Get all template categories
    pub fn get_categories(&self) -> Vec<String> {
        self.categories.keys().cloned().collect()
    }
    
    /// Resolve template properties including inheritance
    pub fn resolve_template_properties(&mut self, template_id: &str) -> Result<inheritance::InheritanceResult, InheritanceError> {
        self.inheritance_tree.resolve_properties(template_id, &self.templates)
    }
    
    /// Validate a template
    pub fn validate_template(&mut self, template_id: &str) -> ValidationResult {
        if let Some(cached) = self.validation_cache.get(template_id) {
            return cached.clone();
        }
        
        if let Some(template) = self.templates.get(template_id) {
            let result = self.validator.validate_template(template, &self.templates, &self.inheritance_tree);
            self.validation_cache.insert(template_id.to_string(), result.clone());
            result
        } else {
            ValidationResult::error(format!("Template '{}' not found", template_id))
        }
    }
    
    /// Start template creation wizard
    pub fn start_new_template_wizard(&mut self) {
        let available_parents: Vec<String> = self.templates.keys().cloned().collect();
        self.template_wizard.available_parents = available_parents;
        self.template_wizard.start_new_template();
    }
    
    /// Start template editing wizard
    pub fn start_edit_template_wizard(&mut self, template_id: &str) {
        if let Some(template) = self.templates.get(template_id).cloned() {
            let available_parents: Vec<String> = self.templates.keys().cloned().collect();
            self.template_wizard.available_parents = available_parents;
            self.template_wizard.start_edit_template(template);
        }
    }
    
    /// Add template to recent list
    pub fn add_to_recent(&mut self, template_id: String) {
        self.recent_templates.retain(|id| id != &template_id);
        self.recent_templates.insert(0, template_id);
        
        // Keep only last 10 recent templates
        if self.recent_templates.len() > 10 {
            self.recent_templates.truncate(10);
        }
    }
    
    /// Get recent templates
    pub fn get_recent_templates(&self) -> Vec<&ComponentTemplate> {
        self.recent_templates
            .iter()
            .filter_map(|id| self.templates.get(id))
            .collect()
    }
    
    /// Search templates by name or description
    pub fn search_templates(&self, query: &str) -> Vec<&ComponentTemplate> {
        let query_lower = query.to_lowercase();
        self.templates
            .values()
            .filter(|template| {
                template.name.to_lowercase().contains(&query_lower) ||
                template.description.to_lowercase().contains(&query_lower) ||
                template.category.to_lowercase().contains(&query_lower)
            })
            .collect()
    }
    
    /// Export template to file
    pub fn export_template(&self, template_id: &str, format: SerializationFormat) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(template) = self.templates.get(template_id) {
            self.serializer.serialize_template(template, format)
        } else {
            Err(format!("Template '{}' not found", template_id).into())
        }
    }
    
    /// Import template from string
    pub fn import_template(&mut self, data: &str, format: SerializationFormat) -> Result<String, Box<dyn std::error::Error>> {
        let template = self.serializer.deserialize_template(data, format)?;
        let template_id = template.id.clone();
        self.add_template(template);
        Ok(template_id)
    }
    
    /// Render template system UI
    pub fn render_ui(&mut self, ctx: &Context) {
        // Render wizard if open
        self.template_wizard.render(ctx);
        
        // Template manager window
        Window::new("Template Manager")
            .resizable(true)
            .default_width(400.0)
            .default_height(600.0)
            .show(ctx, |ui| {
                self.render_template_manager(ui);
            });
    }
    
    /// Render template manager UI
    fn render_template_manager(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("New Template").clicked() {
                self.start_new_template_wizard();
            }
            if ui.button("Import").clicked() {
                // TODO: Open file dialog
            }
        });
        
        ui.separator();
        
        // Category tabs
        let categories = self.get_categories();
        let mut selected_category = "All".to_string();
        
        ui.horizontal(|ui| {
            ui.selectable_value(&mut selected_category, "All".to_string(), "All");
            for category in &categories {
                ui.selectable_value(&mut selected_category, category.clone(), category);
            }
        });
        
        ui.separator();
        
        // Template list
        ScrollArea::vertical().show(ui, |ui| {
            let templates: Vec<&ComponentTemplate> = if selected_category == "All" {
                self.templates.values().collect()
            } else {
                self.get_templates_by_category(&selected_category)
            };
            
            for template in templates {
                ui.horizontal(|ui| {
                    ui.label(&template.name);
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.small_button("Edit").clicked() {
                            self.start_edit_template_wizard(&template.id);
                        }
                        if ui.small_button("Delete").clicked() {
                            self.remove_template(&template.id);
                        }
                        if ui.small_button("Export").clicked() {
                            // TODO: Export template
                        }
                    });
                });
                
                ui.label(format!("Category: {}", template.category));
                ui.label(format!("Properties: {}", template.properties.len()));
                
                if let Some(parent) = &template.parent_template {
                    ui.label(format!("Parent: {}", parent));
                }
                
                ui.separator();
            }
        });
    }
    
    /// Get template statistics
    pub fn get_statistics(&self) -> TemplateStatistics {
        TemplateStatistics {
            total_templates: self.templates.len(),
            categories: self.categories.len(),
            inheritance_relationships: self.inheritance_tree.get_all_templates().len(),
            recent_templates: self.recent_templates.len(),
        }
    }
}

/// Template system statistics
#[derive(Debug, Clone)]
pub struct TemplateStatistics {
    pub total_templates: usize,
    pub categories: usize,
    pub inheritance_relationships: usize,
    pub recent_templates: usize,
}

impl Default for TemplateSystem {
    fn default() -> Self {
        Self::new()
    }
}