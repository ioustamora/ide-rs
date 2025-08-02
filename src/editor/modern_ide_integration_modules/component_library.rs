//! Component Library Management System
//!
//! This module handles the creation, storage, and reuse of custom components
//! and component templates in the IDE.

use egui::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::editor::inspector::PropertyValue;

/// Component library manager for reusable components
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComponentLibrary {
    /// User-defined component templates
    pub templates: HashMap<String, ComponentTemplate>,
    /// Pre-built component library
    pub user_components: HashMap<String, UserComponent>,
    /// Library metadata and organization
    pub metadata: LibraryMetadata,
    /// Component categories
    pub categories: Vec<String>,
    /// Component tags for search
    pub tags: HashMap<String, Vec<String>>,
}

/// Component template definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComponentTemplate {
    /// Template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Component type this template applies to
    pub component_type: String,
    /// Default property values
    pub default_properties: HashMap<String, PropertyValue>,
    /// Template preview image path
    pub preview_image: Option<String>,
    /// Template category
    pub category: String,
    /// Template tags
    pub tags: Vec<String>,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last modified timestamp
    pub modified_at: chrono::DateTime<chrono::Utc>,
}

/// User-defined component
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserComponent {
    /// Component name
    pub name: String,
    /// Component description
    pub description: String,
    /// Component layout and structure
    pub layout: ComponentLayout,
    /// Component properties
    pub properties: HashMap<String, PropertyValue>,
    /// Component preview
    pub preview_image: Option<String>,
    /// Component category
    pub category: String,
    /// Component tags
    pub tags: Vec<String>,
}

/// Library metadata and organization
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LibraryMetadata {
    /// Library name
    pub name: String,
    /// Library version
    pub version: String,
    /// Library author
    pub author: String,
    /// Library description
    pub description: String,
    /// Library icon path
    pub icon: Option<String>,
    /// Library creation date
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Library last modified date
    pub modified_at: chrono::DateTime<chrono::Utc>,
}

/// Component layout definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComponentLayout {
    /// Layout type (stack, grid, absolute, etc.)
    pub layout_type: LayoutType,
    /// Child components
    pub children: Vec<ComponentData>,
    /// Layout properties
    pub properties: HashMap<String, PropertyValue>,
}

/// Component data for serialization
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComponentData {
    /// Component type name
    pub component_type: String,
    /// Component properties
    pub properties: HashMap<String, PropertyValue>,
    /// Component position
    pub position: Option<Pos2>,
    /// Component size
    pub size: Option<Vec2>,
    /// Child components (for containers)
    pub children: Vec<ComponentData>,
}

/// Layout type enumeration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LayoutType {
    /// Absolute positioning
    Absolute,
    /// Vertical stack
    VerticalStack,
    /// Horizontal stack
    HorizontalStack,
    /// Grid layout
    Grid { columns: u32, rows: u32 },
    /// Flex layout
    Flex { direction: FlexDirection, wrap: bool },
}

/// Flex direction enumeration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

impl Default for ComponentLibrary {
    fn default() -> Self {
        Self {
            templates: HashMap::new(),
            user_components: HashMap::new(),
            metadata: LibraryMetadata::default(),
            categories: vec![
                "Basic".to_string(),
                "Layout".to_string(),
                "Input".to_string(),
                "Display".to_string(),
                "Navigation".to_string(),
                "Custom".to_string(),
            ],
            tags: HashMap::new(),
        }
    }
}

impl Default for LibraryMetadata {
    fn default() -> Self {
        Self {
            name: "User Library".to_string(),
            version: "1.0.0".to_string(),
            author: "User".to_string(),
            description: "User-defined component library".to_string(),
            icon: None,
            created_at: chrono::Utc::now(),
            modified_at: chrono::Utc::now(),
        }
    }
}

impl ComponentLibrary {
    /// Create a new component library
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a component template
    pub fn add_template(&mut self, template: ComponentTemplate) {
        self.templates.insert(template.name.clone(), template);
        self.metadata.modified_at = chrono::Utc::now();
    }
    
    /// Add a user component
    pub fn add_user_component(&mut self, component: UserComponent) {
        self.user_components.insert(component.name.clone(), component);
        self.metadata.modified_at = chrono::Utc::now();
    }
    
    /// Get template by name
    pub fn get_template(&self, name: &str) -> Option<&ComponentTemplate> {
        self.templates.get(name)
    }
    
    /// Get user component by name
    pub fn get_user_component(&self, name: &str) -> Option<&UserComponent> {
        self.user_components.get(name)
    }
    
    /// Search templates by tag
    pub fn search_templates_by_tag(&self, tag: &str) -> Vec<&ComponentTemplate> {
        self.templates.values()
            .filter(|template| template.tags.contains(&tag.to_string()))
            .collect()
    }
    
    /// Search templates by category
    pub fn search_templates_by_category(&self, category: &str) -> Vec<&ComponentTemplate> {
        self.templates.values()
            .filter(|template| template.category == category)
            .collect()
    }
    
    /// Export library as JSON
    pub fn export_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
    
    /// Import library from JSON
    pub fn import_json(&mut self, json: &str) -> serde_json::Result<()> {
        let imported: ComponentLibrary = serde_json::from_str(json)?;
        *self = imported;
        Ok(())
    }
    
    /// Render component library UI
    pub fn render_ui(&mut self, ui: &mut Ui) {
        ui.heading("Component Library");
        
        ui.horizontal(|ui| {
            if ui.button("+ New Template").clicked() {
                // Handle new template creation
            }
            
            if ui.button("+ New Component").clicked() {
                // Handle new component creation
            }
            
            if ui.button("Import").clicked() {
                // Handle library import
            }
            
            if ui.button("Export").clicked() {
                // Handle library export
            }
        });
        
        ui.separator();
        
        // Category tabs
        ui.horizontal(|ui| {
            for category in &self.categories {
                if ui.selectable_label(false, category).clicked() {
                    // Filter by category
                }
            }
        });
        
        ui.separator();
        
        // Component grid
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("component_grid")
                .num_columns(3)
                .spacing([10.0, 10.0])
                .show(ui, |ui| {
                    for (name, template) in &self.templates {
                        self.render_template_card(ui, name, template);
                        
                        // Start new row every 3 items
                        if ui.next_location().x == 0.0 {
                            ui.end_row();
                        }
                    }
                });
        });
    }
    
    /// Render a template card
    fn render_template_card(&self, ui: &mut Ui, name: &str, template: &ComponentTemplate) {
        egui::Frame::group(ui.style())
            .inner_margin(8.0)
            .show(ui, |ui| {
                ui.set_width(120.0);
                ui.set_height(100.0);
                
                // Template preview (placeholder)
                let rect = ui.allocate_space(Vec2::new(100.0, 60.0)).1;
                ui.painter().rect_filled(rect, 4.0, Color32::LIGHT_GRAY);
                ui.painter().text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    &template.component_type,
                    FontId::default(),
                    Color32::DARK_GRAY,
                );
                
                ui.label(name);
                ui.small(&template.description);
                
                if ui.small_button("Use").clicked() {
                    // Handle template usage
                }
            });
    }
}