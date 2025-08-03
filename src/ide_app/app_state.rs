//! # Application State Management
//! 
//! Core application state structures and management for the RAD IDE.
//! This module contains the main application state and basic UI management functionality.

use crate::rcl::ui::component::Component;
use crate::ai_agent::AiAgent;
use crate::editor::menu::IdeMenu;
use crate::editor::visual_designer::VisualDesigner;
use crate::editor::smart_ai_assistant::SmartAiAssistant;
use crate::editor::lsp_integration::LspClient;
use crate::editor::code_editor::CodeEditor;
use crate::editor::project_manager::ProjectManager;
use crate::editor::inspector::PropertyInspector;
use crate::editor::live_feedback::LiveFeedbackSystem;
use crate::editor::hierarchy_manager::HierarchyManager;
use crate::editor::modern_ide_integration::ModernIdeIntegration;
use crate::editor::multi_device_preview::MultiDevicePreview;
use crate::editor::template_system_simple::ComponentTemplate;
use crate::editor::file_manager::FileManager;
use crate::editor::realtime_sync::RealtimeSync;

/// # Main IDE Application State
/// 
/// Central state container for the entire RAD IDE application. This struct orchestrates
/// all subsystems and maintains the overall application state. The design follows a
/// centralized state pattern where all major IDE components are owned by this struct.
/// 
/// ## Architecture Pattern
/// 
/// The IDE uses composition over inheritance, combining multiple specialized systems:
/// - **Component Management**: Manages the collection of UI components in the designer
/// - **AI Integration**: Handles asynchronous AI operations with proper task management
/// - **Panel Management**: Controls visibility and state of various IDE panels
/// - **Mode Switching**: Coordinates between design and code editing modes
/// 
/// ## Memory Management
/// 
/// Components are stored as boxed trait objects (`Box<dyn Component>`) to enable
/// polymorphic behavior while maintaining a homogeneous collection. This allows
/// for runtime component type determination and dynamic dispatch.
pub struct IdeAppState {
    // ========================================================================================
    // COMPONENT SYSTEM - Manages the collection of UI components in the visual designer
    // ========================================================================================
    
    /// Root form component that serves as the base container for all other components.
    /// 
    /// The form is always present and acts as the background/container for the design.
    /// It has its own properties (background color, size, etc.) that can be edited.
    pub root_form: crate::rcl::ui::basic::form::Form,

    /// Collection of UI components available in the designer.
    /// 
    /// Uses boxed trait objects to enable polymorphic component storage.
    /// Components are indexed by their position in this vector for efficient lookup.
    /// The visual designer maintains a separate spatial index for hit-testing.
    /// All these components are placed on top of the root form.
    pub components: Vec<Box<dyn Component>>,

    // ========================================================================================
    // AI INTEGRATION SYSTEM - Handles asynchronous AI operations and context management
    // ========================================================================================
    
    /// AI agent for code assistance and automation.
    /// 
    /// Optional to allow operation without AI capabilities. When present, provides
    /// context-aware code generation, bug analysis, and intelligent suggestions.
    pub ai_agent: Option<AiAgent>,
    
    /// Current AI prompt being processed by the user.
    /// 
    /// Stores the user's input before sending to the AI agent. This allows for
    /// prompt editing and validation before submission.
    pub ai_prompt: String,
    
    /// Latest AI response received from the agent.
    /// 
    /// Cached response for display in the AI panel. Persists until the next
    /// AI operation completes to provide user feedback.
    pub ai_response: String,
    
    /// Flag indicating if an AI request is currently in progress.
    /// 
    /// Used to prevent multiple concurrent AI requests and provide appropriate
    /// UI feedback (loading indicators, disabled controls).
    pub ai_pending: bool,
    
    /// Async task handle for AI operations.
    /// 
    /// Maintains a reference to the current AI task to enable cancellation
    /// and proper cleanup. The complex type reflects the async nature of AI operations.
    pub ai_task: Option<std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<String>> + Send>>>,

    // ========================================================================================
    // UI MANAGEMENT SYSTEM - Controls IDE layout, panels, and user interface state
    // ========================================================================================
    
    /// Main IDE menu and toolbar system.
    /// 
    /// Manages the top-level menu bar, toolbars, and associated actions.
    /// Provides a centralized interface for IDE commands and operations.
    pub menu: IdeMenu,
    
    // Panel visibility flags - control which IDE panels are currently shown
    /// Component palette panel visibility (left panel).
    pub show_component_palette: bool,
    /// Properties inspector panel visibility (right panel).
    pub show_properties_inspector: bool,
    /// AI assistance panel visibility (bottom panel).
    pub show_ai_panel: bool,
    /// Build output and console panel visibility (bottom panel).
    pub show_output_panel: bool,
    /// Project explorer panel visibility (left panel).
    pub show_project_panel: bool,
    /// Modern IDE features panel visibility (right panel).
    pub show_modern_ide_panel: bool,
    /// Active tab in left panel
    pub active_left_tab: String,
    /// Active tab in right panel
    pub active_right_tab: String,
    
    // ========================================================================================
    // SELECTION AND INTERACTION SYSTEM - Manages component selection and manipulation
    // ========================================================================================
    
    /// Currently selected component index for property editing.
    /// 
    /// Points to the index in the `components` vector. Used for coordinating
    /// between the visual designer selection and the property inspector.
    pub selected_component: Option<usize>,
    
    /// Mode toggle between design and code editing.
    /// 
    /// Controls the primary IDE mode:
    /// - `true`: Visual designer mode (WYSIWYG editing)
    /// - `false`: Code editor mode (text-based editing)
    pub design_mode: bool,

    // ========================================================================================
    // CORE EDITOR SYSTEMS - The main editing subsystems of the IDE
    // ========================================================================================
    
    /// Advanced visual designer with WYSIWYG editing capabilities.
    /// 
    /// Core visual editing system supporting component placement, alignment,
    /// grid snapping, multi-selection, and advanced layout tools.
    pub visual_designer: VisualDesigner,
    
    /// Enhanced code editor with syntax highlighting and LSP support.
    /// 
    /// Provides advanced text editing capabilities including:
    /// - Syntax highlighting for multiple languages
    /// - Code folding and bracket matching
    /// - Integration with build and debug systems
    pub code_editor: CodeEditor,

    // ========================================================================================
    // INTELLIGENT ASSISTANCE SYSTEMS - AI and language server integration
    // ========================================================================================
    
    /// Context-aware AI assistant for intelligent code assistance.
    /// 
    /// Provides advanced AI capabilities beyond the basic AI agent:
    /// - Project context awareness
    /// - Code analysis and suggestions
    /// - Intelligent refactoring assistance
    pub smart_ai: SmartAiAssistant,
    
    /// LSP client for real-time code intelligence.
    /// 
    /// Manages communication with language servers to provide:
    /// - Real-time error checking and diagnostics
    /// - Code completion and IntelliSense
    /// - Go-to-definition and symbol search
    pub lsp_client: LspClient,

    // ========================================================================================
    // PROJECT AND FILE MANAGEMENT - Handles project lifecycle and file operations
    // ========================================================================================
    
    /// Project manager for file system integration and project operations.
    /// 
    /// Manages project structure, file operations, and build coordination.
    /// Handles project templates, serialization, and workspace management.
    pub project_manager: ProjectManager,

    // ========================================================================================  
    // SPECIALIZED IDE FEATURES - Advanced IDE capabilities and tooling
    // ========================================================================================
    
    /// Property inspector for component configuration.
    /// 
    /// Property editing system supporting:
    /// - Type-safe property editing
    /// - Component property validation
    /// - Real-time property updates
    pub property_inspector: PropertyInspector,
    
    /// Live visual feedback system for real-time design assistance.
    /// 
    /// Provides immediate visual feedback during design operations:
    /// - Alignment guides and snap indicators
    /// - Distance measurements and spacing aids
    /// - Color and style feedback overlays
    pub live_feedback: LiveFeedbackSystem,
    
    /// Component hierarchy manager for tree-based component organization.
    /// 
    /// Manages the hierarchical structure of components:
    /// - Parent-child relationships
    /// - Z-order and layering
    /// - Bulk operations on component trees
    pub hierarchy_manager: HierarchyManager,
    
    /// Modern IDE integration with design systems and framework export.
    /// 
    /// Professional IDE features including:
    /// - Design token management
    /// - Component library integration  
    /// - Framework-specific code export (React, Vue, etc.)
    pub modern_ide: ModernIdeIntegration,
    
    /// Multi-device preview system for responsive design testing.
    /// 
    /// FireMonkey-inspired responsive design testing:
    /// - Real-time preview across device profiles
    /// - Responsive breakpoint management
    /// - Platform-specific styling preview
    pub multi_device_preview: MultiDevicePreview,
    
    /// Component template and inheritance system.
    /// 
    /// Professional template system supporting:
    /// - Component template creation and management
    /// - Template inheritance with property overrides
    /// - Template library and reuse patterns
    pub template_system: Vec<ComponentTemplate>,
    
    // ========================================================================================
    // ANIMATION SYSTEMS - Smooth animations for enhanced user experience
    // ========================================================================================
    
    /// Animation manager for collapsible UI elements
    /// 
    /// Provides smooth dropdown animations for:
    /// - Component palette categories
    /// - Property groups
    /// - Panel sections
    pub animation_manager: super::animated_ui::AnimationManager,
    
    /// Movement animation manager for component dragging
    /// 
    /// Handles smooth component movement animations:
    /// - Drag and drop operations
    /// - Component repositioning
    /// - Snap-to-grid animations
    pub movement_manager: super::animated_ui::MovementManager,
    
    // ========================================================================================
    // FILE MANAGEMENT SYSTEM - Multi-file editing with VS Code-inspired interface
    // ========================================================================================
    
    /// Multi-file management with tab system
    /// 
    /// VS Code-inspired file management providing:
    /// - Multiple file tabs with easy switching
    /// - File type recognition for mode selection
    /// - Automatic switching between code and visual designer
    pub file_manager: FileManager,
    
    /// Real-time synchronization between visual designer and code
    /// 
    /// Ensures changes are reflected immediately:
    /// - Visual designer changes update generated code
    /// - Component additions/modifications trigger code regeneration
    /// - Bidirectional sync with debouncing for performance
    pub realtime_sync: RealtimeSync,
    
    // ========================================================================================
    // PROJECT CREATION SYSTEM - New GUI project creation with cargo integration
    // ========================================================================================
    
    /// New project name for GUI project creation dialog
    /// 
    /// Stores the user-entered project name for cargo new integration.
    /// Validated for cargo compatibility before project creation.
    pub new_project_name: String,
    
    /// New project location path for GUI project creation dialog
    /// 
    /// Stores the selected directory path where the new project will be created.
    /// Defaults to current working directory if not specified.
    pub new_project_location: String,
}

impl IdeAppState {
    /// Create a new IDE application state with default values
    pub fn new() -> Self {
        
        Self {
            // Initialize with a default form as the root component
            root_form: crate::rcl::ui::basic::form::Form::new("Form1".to_string()),
            // Start with no components - just the blank form
            components: vec![],
            ai_agent: Some(AiAgent::new()),
            ai_prompt: String::new(),
            ai_response: String::new(),
            ai_pending: false,
            ai_task: None,
            menu: IdeMenu::new(),
            // Initialize panel visibility - start with key panels visible
            show_component_palette: true,
            show_properties_inspector: true,
            show_ai_panel: false,
            show_output_panel: false,
            selected_component: None,
            visual_designer: VisualDesigner::new(),
            smart_ai: SmartAiAssistant::new(),
            lsp_client: LspClient::new(),
            design_mode: true,
            code_editor: CodeEditor::with_content("rust", Self::default_rust_code()),
            project_manager: ProjectManager::new(),
            show_project_panel: true,
            show_modern_ide_panel: false,
            active_left_tab: "project".to_string(),
            active_right_tab: "properties".to_string(),
            property_inspector: PropertyInspector::new(),
            live_feedback: LiveFeedbackSystem::new(),
            hierarchy_manager: HierarchyManager::new(),
            modern_ide: ModernIdeIntegration::new(),
            multi_device_preview: MultiDevicePreview::new(),
            template_system: Vec::new(),
            animation_manager: super::animated_ui::AnimationManager::new(),
            movement_manager: super::animated_ui::MovementManager::new(),
            file_manager: FileManager::new(),
            realtime_sync: RealtimeSync::new(),
            new_project_name: String::new(),
            new_project_location: String::new(),
        }
    }

    /// Add a component and trigger real-time sync
    pub fn add_component_with_sync(&mut self, component: Box<dyn Component>) {
        let component_index = self.components.len();
        let component_type = component.name().to_string();
        
        // Add component to the collection
        self.components.push(component);
        
        // Notify real-time sync of the addition
        self.realtime_sync.notify_change(
            crate::editor::realtime_sync::ComponentChangeEvent::ComponentAdded {
                component_index,
                component_type,
            }
        );
    }
    
    /// Remove a component and trigger real-time sync
    pub fn remove_component_with_sync(&mut self, component_index: usize) -> bool {
        if component_index < self.components.len() {
            self.components.remove(component_index);
            
            // Notify real-time sync of the removal
            self.realtime_sync.notify_change(
                crate::editor::realtime_sync::ComponentChangeEvent::ComponentRemoved {
                    component_index,
                }
            );
            
            true
        } else {
            false
        }
    }
    
    /// Update component property and trigger real-time sync
    pub fn update_component_property_with_sync(
        &mut self, 
        component_index: usize, 
        property_name: String, 
        new_value: String
    ) -> bool {
        if let Some(component) = self.components.get_mut(component_index) {
            let old_value = component.get_property(&property_name)
                .unwrap_or_else(|| "".to_string());
            
            if component.set_property(&property_name, &new_value) {
                // Notify real-time sync of the change
                self.realtime_sync.notify_change(
                    crate::editor::realtime_sync::ComponentChangeEvent::ComponentModified {
                        component_index,
                        property_name,
                        old_value,
                        new_value,
                    }
                );
                return true;
            }
        }
        false
    }
    
    /// Check and perform real-time sync if needed
    pub fn update_realtime_sync(&mut self) {
        // Check if visual designer has changed and sync to code
        if let Some(active_tab) = self.file_manager.get_active_tab_mut() {
            if let Some(code_editor) = &mut active_tab.code_editor {
                if let Some(_sync_result) = self.realtime_sync.check_and_sync_designer(
                    &self.visual_designer,
                    &self.root_form,
                    &self.components,
                    code_editor,
                ) {
                    // Code was updated, mark tab as dirty
                    active_tab.mark_dirty();
                }
            }
        }
    }

    /// Default Rust code template for new projects
    fn default_rust_code() -> String {
        r#"fn main() {
    println!("Hello, RAD IDE!");
}

// Sample component structure
struct MyComponent {
    value: i32,
}

impl MyComponent {
    fn new(value: i32) -> Self {
        Self { value }
    }
    
    fn render(&self) {
        println!("Component value: {}", self.value);
    }
}
"#.to_string()
    }
}

impl Default for IdeAppState {
    fn default() -> Self {
        Self::new()
    }
}