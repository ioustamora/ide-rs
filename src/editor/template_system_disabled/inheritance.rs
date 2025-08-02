//! Template Inheritance System
//!
//! Manages template inheritance relationships and property resolution.

use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use super::template::{ComponentTemplate, TemplateProperty, PropertyValue};

/// Template inheritance tree manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InheritanceTree {
    /// Parent-child relationships
    pub relationships: HashMap<String, Vec<String>>,
    /// Cached inheritance chains
    pub inheritance_cache: HashMap<String, Vec<String>>,
    /// Circular dependency detection
    pub dependency_graph: HashMap<String, HashSet<String>>,
}

/// Property inheritance resolution result
#[derive(Debug, Clone)]
pub struct InheritanceResult {
    /// Resolved properties
    pub properties: HashMap<String, ResolvedProperty>,
    /// Inheritance chain used
    pub inheritance_chain: Vec<String>,
    /// Resolution conflicts
    pub conflicts: Vec<PropertyConflict>,
}

/// Resolved property with inheritance information
#[derive(Debug, Clone)]
pub struct ResolvedProperty {
    /// Final property value
    pub property: TemplateProperty,
    /// Source template ID
    pub source_template: String,
    /// Whether property was overridden
    pub overridden: bool,
    /// Inheritance level (0 = direct, 1 = parent, etc.)
    pub inheritance_level: usize,
}

/// Property inheritance conflict
#[derive(Debug, Clone)]
pub struct PropertyConflict {
    /// Property name
    pub property_name: String,
    /// Conflicting templates
    pub conflicting_templates: Vec<String>,
    /// Conflict type
    pub conflict_type: ConflictType,
    /// Resolution strategy used
    pub resolution: ConflictResolution,
}

/// Types of inheritance conflicts
#[derive(Debug, Clone, PartialEq)]
pub enum ConflictType {
    /// Property defined in multiple templates
    MultipleDefinitions,
    /// Property marked as non-overridable but overridden
    NonOverridableOverride,
    /// Circular inheritance dependency
    CircularDependency,
    /// Type mismatch in overridden property
    TypeMismatch,
}

/// Conflict resolution strategies
#[derive(Debug, Clone, PartialEq)]
pub enum ConflictResolution {
    /// Use most derived (child) version
    UseMostDerived,
    /// Use base (parent) version
    UseBase,
    /// Merge values (for compatible types)
    Merge,
    /// Error - manual resolution required
    RequiresManualResolution,
}

impl InheritanceTree {
    /// Create a new inheritance tree
    pub fn new() -> Self {
        Self {
            relationships: HashMap::new(),
            inheritance_cache: HashMap::new(),
            dependency_graph: HashMap::new(),
        }
    }
    
    /// Add a parent-child relationship
    pub fn add_relationship(&mut self, parent_id: String, child_id: String) -> Result<(), InheritanceError> {
        // Check for circular dependencies
        if self.would_create_cycle(&parent_id, &child_id) {
            return Err(InheritanceError::CircularDependency(child_id, parent_id));
        }
        
        // Add relationship
        self.relationships
            .entry(parent_id.clone())
            .or_insert_with(Vec::new)
            .push(child_id.clone());
        
        // Update dependency graph
        self.dependency_graph
            .entry(child_id.clone())
            .or_insert_with(HashSet::new)
            .insert(parent_id);
        
        // Invalidate cache for affected templates
        self.invalidate_cache_for(&child_id);
        
        Ok(())
    }
    
    /// Remove a parent-child relationship
    pub fn remove_relationship(&mut self, parent_id: &str, child_id: &str) {
        if let Some(children) = self.relationships.get_mut(parent_id) {
            children.retain(|c| c != child_id);
            if children.is_empty() {
                self.relationships.remove(parent_id);
            }
        }
        
        if let Some(parents) = self.dependency_graph.get_mut(child_id) {
            parents.remove(parent_id);
            if parents.is_empty() {
                self.dependency_graph.remove(child_id);
            }
        }
        
        self.invalidate_cache_for(child_id);
    }
    
    /// Get inheritance chain for a template
    pub fn get_inheritance_chain(&mut self, template_id: &str) -> Vec<String> {
        if let Some(cached) = self.inheritance_cache.get(template_id) {
            return cached.clone();
        }
        
        let chain = self.build_inheritance_chain(template_id);
        self.inheritance_cache.insert(template_id.to_string(), chain.clone());
        chain
    }
    
    /// Build inheritance chain by traversing parents
    fn build_inheritance_chain(&self, template_id: &str) -> Vec<String> {
        let mut chain = vec![template_id.to_string()];
        let mut visited = HashSet::new();
        let mut current = template_id;
        
        while let Some(parents) = self.dependency_graph.get(current) {
            if let Some(parent) = parents.iter().next() {
                if visited.contains(parent) {
                    // Circular dependency detected
                    break;
                }
                visited.insert(current.to_string());
                chain.push(parent.clone());
                current = parent;
            } else {
                break;
            }
        }
        
        chain
    }
    
    /// Resolve properties for a template including inheritance
    pub fn resolve_properties(
        &mut self,
        template_id: &str,
        templates: &HashMap<String, ComponentTemplate>,
    ) -> Result<InheritanceResult, InheritanceError> {
        let inheritance_chain = self.get_inheritance_chain(template_id);
        let mut resolved_properties = HashMap::new();
        let mut conflicts = Vec::new();
        
        // Process inheritance chain from most base to most derived
        for (level, tmpl_id) in inheritance_chain.iter().rev().enumerate() {
            if let Some(template) = templates.get(tmpl_id) {
                for (prop_name, property) in &template.properties {
                    match resolved_properties.get(prop_name) {
                        Some(existing) => {
                            // Handle property override
                            let conflict_result = self.handle_property_override(
                                prop_name,
                                existing,
                                property,
                                tmpl_id,
                                level,
                            );
                            
                            match conflict_result {
                                Ok(resolved) => {
                                    resolved_properties.insert(prop_name.clone(), resolved);
                                }
                                Err(conflict) => {
                                    conflicts.push(conflict);
                                }
                            }
                        }
                        None => {
                            // First definition of this property
                            resolved_properties.insert(
                                prop_name.clone(),
                                ResolvedProperty {
                                    property: property.clone(),
                                    source_template: tmpl_id.clone(),
                                    overridden: false,
                                    inheritance_level: level,
                                },
                            );
                        }
                    }
                }
            }
        }
        
        Ok(InheritanceResult {
            properties: resolved_properties,
            inheritance_chain,
            conflicts,
        })
    }
    
    /// Handle property override logic
    fn handle_property_override(
        &self,
        property_name: &str,
        existing: &ResolvedProperty,
        new_property: &TemplateProperty,
        template_id: &str,
        inheritance_level: usize,
    ) -> Result<ResolvedProperty, PropertyConflict> {
        // Check if property is overridable
        if !existing.property.overridable {
            return Err(PropertyConflict {
                property_name: property_name.to_string(),
                conflicting_templates: vec![existing.source_template.clone(), template_id.to_string()],
                conflict_type: ConflictType::NonOverridableOverride,
                resolution: ConflictResolution::RequiresManualResolution,
            });
        }
        
        // Check type compatibility
        if !self.are_types_compatible(&existing.property.property_type, &new_property.property_type) {
            return Err(PropertyConflict {
                property_name: property_name.to_string(),
                conflicting_templates: vec![existing.source_template.clone(), template_id.to_string()],
                conflict_type: ConflictType::TypeMismatch,
                resolution: ConflictResolution::RequiresManualResolution,
            });
        }
        
        // Override is valid - use most derived version
        Ok(ResolvedProperty {
            property: new_property.clone(),
            source_template: template_id.to_string(),
            overridden: true,
            inheritance_level,
        })
    }
    
    /// Check if two property types are compatible
    fn are_types_compatible(
        &self,
        type1: &super::template::PropertyType,
        type2: &super::template::PropertyType,
    ) -> bool {
        use super::template::PropertyType;
        
        match (type1, type2) {
            (PropertyType::String, PropertyType::String) => true,
            (PropertyType::Integer, PropertyType::Integer) => true,
            (PropertyType::Float, PropertyType::Float) => true,
            (PropertyType::Boolean, PropertyType::Boolean) => true,
            (PropertyType::Color, PropertyType::Color) => true,
            (PropertyType::Font, PropertyType::Font) => true,
            (PropertyType::Size, PropertyType::Size) => true,
            (PropertyType::Position, PropertyType::Position) => true,
            (PropertyType::Alignment, PropertyType::Alignment) => true,
            (PropertyType::List(ref l1), PropertyType::List(ref l2)) => l1 == l2,
            (PropertyType::Custom(ref c1), PropertyType::Custom(ref c2)) => c1 == c2,
            // Allow numeric type conversions
            (PropertyType::Integer, PropertyType::Float) => true,
            (PropertyType::Float, PropertyType::Integer) => true,
            _ => false,
        }
    }
    
    /// Check if adding a relationship would create a cycle
    fn would_create_cycle(&self, parent_id: &str, child_id: &str) -> bool {
        if parent_id == child_id {
            return true;
        }
        
        let mut visited = HashSet::new();
        let mut stack = vec![parent_id];
        
        while let Some(current) = stack.pop() {
            if visited.contains(current) {
                continue;
            }
            visited.insert(current);
            
            if current == child_id {
                return true;
            }
            
            if let Some(parents) = self.dependency_graph.get(current) {
                for parent in parents {
                    stack.push(parent);
                }
            }
        }
        
        false
    }
    
    /// Invalidate cache for a template and its descendants
    fn invalidate_cache_for(&mut self, template_id: &str) {
        self.inheritance_cache.remove(template_id);
        
        // Also invalidate descendants
        if let Some(children) = self.relationships.get(template_id) {
            for child in children {
                self.invalidate_cache_for(child);
            }
        }
    }
    
    /// Get all children of a template
    pub fn get_children(&self, template_id: &str) -> Vec<String> {
        self.relationships
            .get(template_id)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Get all parents of a template
    pub fn get_parents(&self, template_id: &str) -> Vec<String> {
        self.dependency_graph
            .get(template_id)
            .map(|parents| parents.iter().cloned().collect())
            .unwrap_or_default()
    }
    
    /// Check if a template has any inheritance relationships
    pub fn has_relationships(&self, template_id: &str) -> bool {
        self.relationships.contains_key(template_id) || 
        self.dependency_graph.contains_key(template_id)
    }
    
    /// Get all templates in the inheritance tree
    pub fn get_all_templates(&self) -> HashSet<String> {
        let mut all_templates = HashSet::new();
        
        for (parent, children) in &self.relationships {
            all_templates.insert(parent.clone());
            for child in children {
                all_templates.insert(child.clone());
            }
        }
        
        all_templates
    }
    
    /// Validate the entire inheritance tree
    pub fn validate_tree(&self, templates: &HashMap<String, ComponentTemplate>) -> Vec<InheritanceError> {
        let mut errors = Vec::new();
        
        for template_id in self.get_all_templates() {
            // Check if template exists
            if !templates.contains_key(&template_id) {
                errors.push(InheritanceError::TemplateNotFound(template_id.clone()));
                continue;
            }
            
            // Check for circular dependencies
            if self.would_create_cycle(&template_id, &template_id) {
                errors.push(InheritanceError::CircularDependency(
                    template_id.clone(),
                    template_id,
                ));
            }
        }
        
        errors
    }
}

/// Inheritance system errors
#[derive(Debug, Clone)]
pub enum InheritanceError {
    /// Template not found in collection
    TemplateNotFound(String),
    /// Circular dependency detected
    CircularDependency(String, String),
    /// Property type mismatch
    PropertyTypeMismatch(String, String),
    /// Non-overridable property override attempt
    NonOverridableOverride(String, String),
    /// Invalid inheritance chain
    InvalidInheritanceChain(String),
}

impl std::fmt::Display for InheritanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InheritanceError::TemplateNotFound(id) => {
                write!(f, "Template '{}' not found", id)
            }
            InheritanceError::CircularDependency(child, parent) => {
                write!(f, "Circular dependency: '{}' cannot inherit from '{}'", child, parent)
            }
            InheritanceError::PropertyTypeMismatch(prop, template) => {
                write!(f, "Property type mismatch for '{}' in template '{}'", prop, template)
            }
            InheritanceError::NonOverridableOverride(prop, template) => {
                write!(f, "Cannot override non-overridable property '{}' in template '{}'", prop, template)
            }
            InheritanceError::InvalidInheritanceChain(template) => {
                write!(f, "Invalid inheritance chain for template '{}'", template)
            }
        }
    }
}

impl std::error::Error for InheritanceError {}

impl Default for InheritanceTree {
    fn default() -> Self {
        Self::new()
    }
}