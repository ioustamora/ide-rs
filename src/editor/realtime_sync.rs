//! Real-Time Sync Between Visual Designer and Code
//!
//! Provides bidirectional synchronization between the visual designer and generated code,
//! ensuring changes are reflected immediately in both views.

use std::collections::HashMap;
use std::time::{Duration, Instant};
use crate::editor::visual_designer::VisualDesigner;
use crate::editor::code_editor::CodeEditor;
use crate::editor::modern_ide_integration_modules::code_generation::{
    CodeGenerator, GenerationContext, ComponentGenerationData
};
use crate::rcl::ui::component::Component;
use crate::rcl::ui::basic::form::Form;

/// Real-time synchronization manager
pub struct RealtimeSync {
    /// Code generator for creating code from visual design
    pub code_generator: CodeGenerator,
    /// Last known visual designer state hash
    pub last_designer_hash: u64,
    /// Last known code content hash  
    pub last_code_hash: u64,
    /// Debounce timer to prevent excessive updates
    pub last_update: Instant,
    /// Minimum time between updates (debouncing)
    pub debounce_duration: Duration,
    /// Whether sync is currently enabled
    pub sync_enabled: bool,
    /// Sync direction preference
    pub sync_direction: SyncDirection,
    /// Component change listeners
    pub change_listeners: Vec<Box<dyn ComponentChangeListener>>,
}

/// Synchronization direction
#[derive(Debug, Clone, PartialEq)]
pub enum SyncDirection {
    /// Visual designer changes update code
    DesignerToCode,
    /// Code changes update visual designer
    CodeToDesigner,
    /// Bidirectional sync (last changed wins)
    Bidirectional,
}

/// Component change event
#[derive(Debug, Clone)]
pub enum ComponentChangeEvent {
    /// Component was added to the designer
    ComponentAdded {
        component_index: usize,
        component_type: String,
    },
    /// Component was removed from the designer
    ComponentRemoved {
        component_index: usize,
    },
    /// Component properties were modified
    ComponentModified {
        component_index: usize,
        property_name: String,
        old_value: String,
        new_value: String,
    },
    /// Component was moved/repositioned
    ComponentMoved {
        component_index: usize,
        old_position: egui::Pos2,
        new_position: egui::Pos2,
    },
    /// Component was resized
    ComponentResized {
        component_index: usize,
        old_size: egui::Vec2,
        new_size: egui::Vec2,
    },
    /// Form properties changed
    FormModified {
        property_name: String,
        old_value: String,
        new_value: String,
    },
}

/// Trait for listening to component changes
pub trait ComponentChangeListener {
    /// Called when a component change occurs
    fn on_component_change(&mut self, event: &ComponentChangeEvent);
}

/// Sync result containing generated code and metadata
#[derive(Debug, Clone)]
pub struct SyncResult {
    /// Generated code content
    pub code: String,
    /// Target language/framework
    pub language: String,
    /// Whether sync was successful
    pub success: bool,
    /// Error message if sync failed
    pub error: Option<String>,
    /// Timestamp of sync
    pub timestamp: Instant,
}

impl RealtimeSync {
    /// Create a new real-time sync manager
    pub fn new() -> Self {
        Self {
            code_generator: CodeGenerator::new(),
            last_designer_hash: 0,
            last_code_hash: 0,
            last_update: Instant::now(),
            debounce_duration: Duration::from_millis(300), // 300ms debounce
            sync_enabled: true,
            sync_direction: SyncDirection::DesignerToCode,
            change_listeners: Vec::new(),
        }
    }
    
    /// Enable or disable real-time sync
    pub fn set_sync_enabled(&mut self, enabled: bool) {
        self.sync_enabled = enabled;
    }
    
    /// Set sync direction
    pub fn set_sync_direction(&mut self, direction: SyncDirection) {
        self.sync_direction = direction;
    }
    
    /// Add a component change listener
    pub fn add_change_listener(&mut self, listener: Box<dyn ComponentChangeListener>) {
        self.change_listeners.push(listener);
    }
    
    /// Notify listeners of a component change
    pub fn notify_change(&mut self, event: ComponentChangeEvent) {
        for listener in &mut self.change_listeners {
            listener.on_component_change(&event);
        }
        
        // Mark that an update is needed
        self.last_update = Instant::now();
    }
    
    /// Check if visual designer has changed and sync if needed
    pub fn check_and_sync_designer(
        &mut self, 
        visual_designer: &VisualDesigner,
        form: &Form,
        components: &[Box<dyn Component>],
        code_editor: &mut CodeEditor,
    ) -> Option<SyncResult> {
        if !self.sync_enabled {
            return None;
        }
        
        // Check debounce timer
        if self.last_update.elapsed() < self.debounce_duration {
            return None;
        }
        
        // Calculate current visual designer state hash
        let current_hash = self.calculate_designer_hash(visual_designer, form, components);
        
        // Check if visual designer has changed
        if current_hash != self.last_designer_hash {
            self.last_designer_hash = current_hash;
            
            match self.sync_direction {
                SyncDirection::DesignerToCode | SyncDirection::Bidirectional => {
                    return self.sync_designer_to_code(visual_designer, form, components, code_editor);
                }
                SyncDirection::CodeToDesigner => {
                    // Visual designer changed but we're only syncing code->designer
                    return None;
                }
            }
        }
        
        None
    }
    
    /// Check if code has changed and sync if needed
    pub fn check_and_sync_code(
        &mut self,
        code_editor: &CodeEditor,
        visual_designer: &mut VisualDesigner,
        form: &mut Form,
        components: &mut Vec<Box<dyn Component>>,
    ) -> Option<SyncResult> {
        if !self.sync_enabled {
            return None;
        }
        
        // Check debounce timer
        if self.last_update.elapsed() < self.debounce_duration {
            return None;
        }
        
        // Calculate current code hash
        let current_hash = self.calculate_code_hash(code_editor);
        
        // Check if code has changed
        if current_hash != self.last_code_hash {
            self.last_code_hash = current_hash;
            
            match self.sync_direction {
                SyncDirection::CodeToDesigner | SyncDirection::Bidirectional => {
                    return self.sync_code_to_designer(code_editor, visual_designer, form, components);
                }
                SyncDirection::DesignerToCode => {
                    // Code changed but we're only syncing designer->code
                    return None;
                }
            }
        }
        
        None
    }
    
    /// Sync visual designer changes to code
    fn sync_designer_to_code(
        &mut self,
        visual_designer: &VisualDesigner,
        form: &Form,
        components: &[Box<dyn Component>],
        code_editor: &mut CodeEditor,
    ) -> Option<SyncResult> {
        // Create generation context from visual designer state
        let context = self.create_generation_context(visual_designer, form, components);
        
        // Generate code using the selected template
        let template_id = self.get_template_for_language(&code_editor.language);
        
        match self.code_generator.generate_code(&template_id, &context) {
            Ok(generated_code) => {
                // Update code editor with generated code
                code_editor.code = generated_code.content.clone();
                code_editor.mark_dirty();
                
                // Update our hash to prevent immediate re-sync
                self.last_code_hash = self.calculate_code_hash(code_editor);
                
                Some(SyncResult {
                    code: generated_code.content,
                    language: generated_code.language,
                    success: true,
                    error: None,
                    timestamp: Instant::now(),
                })
            }
            Err(e) => {
                Some(SyncResult {
                    code: String::new(),
                    language: code_editor.language.clone(),
                    success: false,
                    error: Some(e.to_string()),
                    timestamp: Instant::now(),
                })
            }
        }
    }
    
    /// Sync code changes to visual designer (not fully implemented - complex parsing required)
    fn sync_code_to_designer(
        &mut self,
        _code_editor: &CodeEditor,
        _visual_designer: &mut VisualDesigner,
        _form: &mut Form,
        _components: &mut Vec<Box<dyn Component>>,
    ) -> Option<SyncResult> {
        // This would require parsing the code and extracting component information
        // This is a complex feature that would require AST parsing for each language
        // For now, return a not-implemented result
        
        Some(SyncResult {
            code: "Code to designer sync not yet implemented".to_string(),
            language: "rust".to_string(),
            success: false,
            error: Some("Code to designer synchronization requires AST parsing implementation".to_string()),
            timestamp: Instant::now(),
        })
    }
    
    /// Create generation context from visual designer state
    fn create_generation_context(
        &self,
        visual_designer: &VisualDesigner,
        form: &Form,
        components: &[Box<dyn Component>],
    ) -> GenerationContext {
        // Convert form and components to generation data
        let mut component_data = Vec::new();
        
        for (index, component) in components.iter().enumerate() {
            let position = visual_designer.layout.positions.get(&index).copied();
            let size = visual_designer.layout.sizes.get(&index).copied();
            
            // Extract component properties
            let mut properties = HashMap::new();
            for prop_name in component.get_property_names() {
                if let Some(prop_value) = component.get_property(&prop_name) {
                    properties.insert(prop_name, crate::editor::inspector::PropertyValue::String(prop_value));
                }
            }
            
            component_data.push(ComponentGenerationData {
                name: format!("component_{}", index),
                component_type: component.name().to_string(),
                properties,
                children: Vec::new(), // TODO: Handle nested components
                layout: Some(crate::editor::modern_ide_integration_modules::code_generation::LayoutInfo {
                    position,
                    size,
                    z_index: None,
                    constraints: HashMap::new(),
                }),
            });
        }
        
        // Create root component data for the form
        let root_component = ComponentGenerationData {
            name: form.title.clone(),
            component_type: "Form".to_string(),
            properties: HashMap::new(), // TODO: Extract form properties
            children: component_data,
            layout: None,
        };
        
        // Create context variables
        let mut variables = HashMap::new();
        variables.insert("use_typescript".to_string(), 
            crate::editor::modern_ide_integration_modules::code_generation::ContextValue::Boolean(true));
        
        GenerationContext {
            component: root_component,
            framework: "react".to_string(), // TODO: Make configurable
            variables,
        }
    }
    
    /// Calculate hash of visual designer state
    fn calculate_designer_hash(
        &self,
        visual_designer: &VisualDesigner,
        form: &Form,
        components: &[Box<dyn Component>],
    ) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        
        // Hash form properties
        form.title.hash(&mut hasher);
        form.background_color.hash(&mut hasher);
        
        // Hash component count and positions
        components.len().hash(&mut hasher);
        for (index, component) in components.iter().enumerate() {
            component.name().hash(&mut hasher);
            
            // Hash position if available
            if let Some(pos) = visual_designer.layout.positions.get(&index) {
                (pos.x as i32).hash(&mut hasher);
                (pos.y as i32).hash(&mut hasher);
            }
            
            // Hash component properties
            for prop_name in component.get_property_names() {
                if let Some(prop_value) = component.get_property(&prop_name) {
                    prop_name.hash(&mut hasher);
                    prop_value.hash(&mut hasher);
                }
            }
        }
        
        hasher.finish()
    }
    
    /// Calculate hash of code content
    fn calculate_code_hash(&self, code_editor: &CodeEditor) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        code_editor.code.hash(&mut hasher);
        hasher.finish()
    }
    
    /// Get appropriate template ID for a language
    fn get_template_for_language(&self, language: &str) -> String {
        match language {
            "typescript" | "javascript" => "react-component".to_string(),
            "vue" => "vue-component".to_string(),
            _ => "react-component".to_string(), // Default fallback
        }
    }
    
    /// Render sync status UI
    pub fn render_sync_ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Real-time Sync:");
            
            // Enable/disable toggle
            let sync_text = if self.sync_enabled { "🟢 ON" } else { "🔴 OFF" };
            if ui.button(sync_text).clicked() {
                self.sync_enabled = !self.sync_enabled;
            }
            
            ui.separator();
            
            // Sync direction selector
            ui.label("Direction:");
            egui::ComboBox::from_id_source("sync_direction")
                .selected_text(match self.sync_direction {
                    SyncDirection::DesignerToCode => "Designer → Code",
                    SyncDirection::CodeToDesigner => "Code → Designer", 
                    SyncDirection::Bidirectional => "Bidirectional",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.sync_direction, SyncDirection::DesignerToCode, "Designer → Code");
                    ui.selectable_value(&mut self.sync_direction, SyncDirection::CodeToDesigner, "Code → Designer");
                    ui.selectable_value(&mut self.sync_direction, SyncDirection::Bidirectional, "Bidirectional");
                });
            
            ui.separator();
            
            // Debounce setting
            ui.label("Debounce (ms):");
            let mut debounce_ms = self.debounce_duration.as_millis() as u32;
            if ui.add(egui::Slider::new(&mut debounce_ms, 100..=1000)).changed() {
                self.debounce_duration = Duration::from_millis(debounce_ms as u64);
            }
        });
    }
}

impl Default for RealtimeSync {
    fn default() -> Self {
        Self::new()
    }
}

/// Example component change listener that logs changes
pub struct LoggingChangeListener;

impl ComponentChangeListener for LoggingChangeListener {
    fn on_component_change(&mut self, event: &ComponentChangeEvent) {
        match event {
            ComponentChangeEvent::ComponentAdded { component_index, component_type } => {
                println!("Component added: {} at index {}", component_type, component_index);
            }
            ComponentChangeEvent::ComponentRemoved { component_index } => {
                println!("Component removed at index {}", component_index);
            }
            ComponentChangeEvent::ComponentModified { component_index, property_name, old_value, new_value } => {
                println!("Component {} property '{}' changed: '{}' -> '{}'", 
                    component_index, property_name, old_value, new_value);
            }
            ComponentChangeEvent::ComponentMoved { component_index, old_position, new_position } => {
                println!("Component {} moved: {:?} -> {:?}", 
                    component_index, old_position, new_position);
            }
            ComponentChangeEvent::ComponentResized { component_index, old_size, new_size } => {
                println!("Component {} resized: {:?} -> {:?}", 
                    component_index, old_size, new_size);
            }
            ComponentChangeEvent::FormModified { property_name, old_value, new_value } => {
                println!("Form property '{}' changed: '{}' -> '{}'", 
                    property_name, old_value, new_value);
            }
        }
    }
}