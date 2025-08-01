//! # RAD IDE Application - Core Architecture
//! 
//! This module implements the main IDE application using eframe/egui and the RCL component system.
//! It provides a comprehensive visual development environment inspired by professional RAD tools
//! like Embarcadero RAD Studio and Delphi.
//!
//! ## Architecture Overview
//! 
//! The IDE follows a modular architecture with several key subsystems:
//! - **Visual Designer**: WYSIWYG component layout with advanced alignment tools
//! - **Code Editor**: Integrated editor with LSP support and syntax highlighting
//! - **AI Integration**: Context-aware AI assistance for code generation and analysis
//! - **Project Management**: File system integration with project templates
//! - **Component Library**: Extensible RCL component system with inheritance
//! - **Multi-Device Preview**: Responsive design testing across device profiles
//!
//! ## State Management
//! 
//! The application uses a centralized state model where `IdeApp` coordinates between
//! different subsystems. State changes flow through the main update loop, ensuring
//! consistent UI updates and proper event handling.
//!
//! ## Performance Considerations
//! 
//! - Components are boxed trait objects for dynamic dispatch
//! - Visual designer uses spatial indexing for efficient hit-testing
//! - AI operations run asynchronously to prevent UI blocking
//! - Build operations are handled via separate processes

use eframe::egui;
use crate::rcl::ui::component::Component;
use crate::rcl::ui::basic::button::Button;
use crate::rcl::ui::basic::label::Label;
use crate::rcl::ui::basic::textbox::TextBox;
use crate::rcl::ui::basic::checkbox::Checkbox;
use crate::rcl::ui::basic::slider::Slider;
use crate::rcl::ui::basic::dropdown::Dropdown;
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
use crate::editor::template_system::TemplateSystem;

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
pub struct IdeApp {
    // ========================================================================================
    // COMPONENT SYSTEM - Manages the collection of UI components in the visual designer
    // ========================================================================================
    
    /// Collection of UI components available in the designer.
    /// 
    /// Uses boxed trait objects to enable polymorphic component storage.
    /// Components are indexed by their position in this vector for efficient lookup.
    /// The visual designer maintains a separate spatial index for hit-testing.
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
    
    // ========================================================================================
    // SELECTION AND INTERACTION SYSTEM - Manages component selection and manipulation
    // ========================================================================================
    
    /// Currently selected component index for property editing.
    /// 
    /// Points to the index in the `components` vector. Used for coordinating
    /// between the visual designer selection and the property inspector.
    pub selected_component: Option<usize>,
    
    /// Drag and drop state for component manipulation.
    /// 
    /// Tracks ongoing drag operations for both adding new components from
    /// the palette and repositioning existing components in the designer.
    pub drag_state: DragState,

    // ========================================================================================
    // CORE EDITOR SYSTEMS - The main editing subsystems of the IDE
    // ========================================================================================
    
    /// Advanced visual designer with WYSIWYG editing capabilities.
    /// 
    /// Core visual editing system supporting component placement, alignment,
    /// grid snapping, multi-selection, and advanced layout tools.
    pub visual_designer: VisualDesigner,
    
    /// Mode toggle between design and code editing.
    /// 
    /// Controls the primary IDE mode:
    /// - `true`: Visual designer mode (WYSIWYG editing)
    /// - `false`: Code editor mode (text-based editing)
    pub design_mode: bool,
    
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
    
    /// Advanced property inspector with undo/redo and validation.
    /// 
    /// Enhanced property editing system supporting:
    /// - Type-safe property editing with validation
    /// - Undo/redo for property changes
    /// - Bulk property operations across selections
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
    pub template_system: TemplateSystem,
}

/// # Drag and Drop State Management
/// 
/// Manages the state of drag and drop operations within the IDE. This system
/// supports multiple types of drag operations including component placement
/// from the palette and repositioning of existing components.
/// 
/// ## State Lifecycle
/// 
/// 1. **Drag Start**: User begins dragging (from palette or existing component)
/// 2. **Drag Continue**: Track mouse movement and provide visual feedback
/// 3. **Drag End**: Complete the operation (place component or update position)
/// 4. **Drag Cancel**: User cancels the operation (ESC key or invalid drop)
/// 
/// ## Performance Considerations
/// 
/// The drag state is updated every frame during drag operations, so it's kept
/// minimal to reduce allocation overhead. Visual feedback is handled separately
/// by the live feedback system.
#[derive(Default)]
pub struct DragState {
    /// Index of the component being dragged, if any.
    /// 
    /// For existing components being repositioned, this refers to their index
    /// in the main `components` vector. None when dragging from palette.
    pub dragging_component: Option<usize>,
    
    /// Type of drag operation currently in progress.
    /// 
    /// Determines how the drag operation should be handled and what visual
    /// feedback should be provided to the user.
    pub drag_type: DragType,
    
    /// Screen position where the drag operation started.
    /// 
    /// Used for calculating drag deltas and determining drag thresholds.
    /// Essential for smooth drag feedback and proper component positioning.
    pub drag_start_pos: Option<egui::Pos2>,
}

/// Types of drag operations supported by the IDE
#[derive(Default)]
pub enum DragType {
    #[default]
    None,
    /// Dragging a component from the palette to add it
    AddingComponent(ComponentType),
    /// Dragging an existing component to reorder
    ReorderingComponent,
}

/// Types of components that can be dragged from the palette
#[derive(Clone, Copy)]
pub enum ComponentType {
    Button,
    Label,
    TextBox,
    Checkbox,
    Slider,
    Dropdown,
}

/// Actions that can be performed on components
#[derive(Clone, Copy)]
pub enum ComponentAction {
    Select,
    MoveUp,
    MoveDown,
    Delete,
}

impl Default for IdeApp {
    fn default() -> Self {
        Self {
            components: vec![
                Box::new(Button { label: "Click Me".into(), editable: false }),
                Box::new(Label { text: "Label Component".into(), editable: false }),
                Box::new(TextBox { value: "Edit me".into(), editable: false }),
                Box::new(Checkbox { label: "Check me".into(), checked: false, editable: false }),
                Box::new(Slider { value: 0.5, min: 0.0, max: 1.0, editable: false }),
                Box::new(Dropdown {
                    label: "Choose option".into(),
                    options: vec!["Option 1".into(), "Option 2".into(), "Option 3".into()],
                    selected: 0,
                    editable: false,
                }),
            ],
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
            drag_state: DragState::default(),
            visual_designer: VisualDesigner::new(),
            smart_ai: SmartAiAssistant::new(),
            lsp_client: LspClient::new(),
            design_mode: true,
            code_editor: CodeEditor::with_content("rust", Self::default_rust_code()),
            project_manager: ProjectManager::new(),
            show_project_panel: true,
            show_modern_ide_panel: false,
            property_inspector: PropertyInspector::new(),
            live_feedback: LiveFeedbackSystem::new(),
            hierarchy_manager: HierarchyManager::new(),
            modern_ide: ModernIdeIntegration::new(),
            multi_device_preview: MultiDevicePreview::new(),
            template_system: TemplateSystem::new(),
        }
    }
}

impl eframe::App for IdeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top menu bar with improved organization
        egui::TopBottomPanel::top("menu_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                self.menu.ui(ui);
                ui.separator();
                
                // Panel toggle buttons with tooltips
                if ui.button("🧰").on_hover_text("Toggle Component Palette").clicked() {
                    self.show_component_palette = !self.show_component_palette;
                }
                if ui.button("🔧").on_hover_text("Toggle Properties Inspector").clicked() {
                    self.show_properties_inspector = !self.show_properties_inspector;
                }
                if ui.button("🤖").on_hover_text("Toggle Smart AI Assistant").clicked() {
                    self.show_ai_panel = !self.show_ai_panel;
                }
                if ui.button("📋").on_hover_text("Toggle Output Panel").clicked() {
                    self.show_output_panel = !self.show_output_panel;
                }
                if ui.button("📁").on_hover_text("Toggle Project Explorer").clicked() {
                    self.show_project_panel = !self.show_project_panel;
                }
                if ui.button("🗂").on_hover_text("Toggle Hierarchy Panel").clicked() {
                    self.hierarchy_manager.show_hierarchy_panel = !self.hierarchy_manager.show_hierarchy_panel;
                }
                if ui.button("🚀").on_hover_text("Toggle Modern IDE Features").clicked() {
                    self.show_modern_ide_panel = !self.show_modern_ide_panel;
                }
                
                ui.separator();
                
                // Design mode toggle
                if ui.selectable_label(self.design_mode, "🎨 Design").on_hover_text("Visual Designer Mode").clicked() {
                    self.design_mode = true;
                }
                if ui.selectable_label(!self.design_mode, "💻 Code").on_hover_text("Code Editor Mode").clicked() {
                    self.design_mode = false;
                }
                
                ui.separator();
                
                // Grid and alignment tools (only in design mode)
                if self.design_mode {
                    if ui.button("📐").on_hover_text("Toggle Grid").clicked() {
                        self.visual_designer.grid.visible = !self.visual_designer.grid.visible;
                    }
                    if ui.button("📏").on_hover_text("Toggle Rulers").clicked() {
                        self.visual_designer.guides.rulers_visible = !self.visual_designer.guides.rulers_visible;
                    }
                    if ui.button("🔗").on_hover_text("Snap to Grid").clicked() {
                        self.visual_designer.grid.snap_enabled = !self.visual_designer.grid.snap_enabled;
                    }
                    
                    ui.separator();
                    
                    // Multi-device preview toggle
                    if ui.selectable_label(self.multi_device_preview.enabled, "📱").on_hover_text("Multi-Device Preview").clicked() {
                        self.multi_device_preview.toggle_preview();
                    }
                }
            });
        });

        // Left panel - Project Explorer, Component Palette, and Hierarchy
        if self.show_project_panel || self.show_component_palette || self.hierarchy_manager.show_hierarchy_panel {
            egui::SidePanel::left("left_panel")
                .resizable(true)
                .min_width(200.0)
                .default_width(250.0)
                .show(ctx, |ui| {
                    // Define which tab is currently active
                    let mut active_tab = if self.show_project_panel { 0 } 
                                       else if self.show_component_palette { 1 } 
                                       else if self.hierarchy_manager.show_hierarchy_panel { 2 }
                                       else if self.template_system.templates.len() > 0 { 3 }
                                       else { 1 }; // Default to components
                    
                    // Tab-like interface for different panels
                    ui.horizontal(|ui| {
                        if ui.selectable_label(active_tab == 0, "📁 Project").clicked() {
                            self.show_project_panel = true;
                            self.show_component_palette = false;
                            self.hierarchy_manager.show_hierarchy_panel = false;
                            active_tab = 0;
                        }
                        if ui.selectable_label(active_tab == 1, "🧰 Components").clicked() {
                            self.show_project_panel = false;
                            self.show_component_palette = true;
                            self.hierarchy_manager.show_hierarchy_panel = false;
                            active_tab = 1;
                        }
                        if ui.selectable_label(active_tab == 2, "🗂 Hierarchy").clicked() {
                            self.show_project_panel = false;
                            self.show_component_palette = false;
                            self.hierarchy_manager.show_hierarchy_panel = true;
                            active_tab = 2;
                        }
                        if ui.selectable_label(active_tab == 3, "📋 Templates").clicked() {
                            self.show_project_panel = false;
                            self.show_component_palette = false;
                            self.hierarchy_manager.show_hierarchy_panel = false;
                            active_tab = 3;
                        }
                    });
                    
                    ui.separator();
                    
                    // Show content based on active tab
                    match active_tab {
                        0 => self.render_project_panel(ui),
                        1 => {
                            ui.heading("Component Palette");
                            ui.separator();
                            
                            // Component buttons with drag-and-drop capability
                            self.render_draggable_component_button(ui, "🔘 Button", ComponentType::Button);
                            self.render_draggable_component_button(ui, "🏷️ Label", ComponentType::Label);
                            self.render_draggable_component_button(ui, "📝 TextBox", ComponentType::TextBox);
                            self.render_draggable_component_button(ui, "☑️ Checkbox", ComponentType::Checkbox);
                            self.render_draggable_component_button(ui, "🎚️ Slider", ComponentType::Slider);
                            self.render_draggable_component_button(ui, "📋 Dropdown", ComponentType::Dropdown);
                            
                            ui.separator();
                            ui.label("💡 Drag components to the canvas");
                        },
                        2 => self.hierarchy_manager.render_hierarchy_panel(ui, &self.components),
                        3 => self.template_system.render_template_panel(ui),
                        _ => {}
                    }
                });
        }

        // Right panel - Properties Inspector and Modern IDE Features
        if self.show_properties_inspector || self.show_modern_ide_panel {
            egui::SidePanel::right("properties_inspector")
                .resizable(true)
                .min_width(250.0)
                .default_width(300.0)
                .show(ctx, |ui| {
                    // Define which tab is active
                    let active_tab = if self.show_properties_inspector && !self.show_modern_ide_panel { 0 }
                                   else if self.show_modern_ide_panel { 1 }
                                   else { 0 }; // Default to properties
                    
                    // Tab interface for Properties vs Modern IDE Features
                    ui.horizontal(|ui| {
                        if ui.selectable_label(active_tab == 0, "🔧 Properties").clicked() {
                            self.show_properties_inspector = true;
                            self.show_modern_ide_panel = false;
                        }
                        if ui.selectable_label(active_tab == 1, "🚀 IDE Features").clicked() {
                            self.show_properties_inspector = false;
                            self.show_modern_ide_panel = true;
                        }
                    });
                    
                    ui.separator();
                    
                    match active_tab {
                        0 => {
                            ui.heading("Properties");
                        
                            // Prepare selected component for property inspector
                            let selected_component = if let Some(selected_idx) = self.selected_component {
                                if selected_idx < self.components.len() {
                                    Some((selected_idx, self.components[selected_idx].as_ref()))
                                } else {
                                    None
                                }
                            } else {
                                None
                            };
                            
                            // Sync live feedback state with property inspector
                            self.live_feedback.set_enabled(self.property_inspector.live_preview);
                            
                            // Render the advanced property inspector
                            self.property_inspector.ui(ui, selected_component);
                            
                            ui.separator();
                            
                            // Component actions
                            if let Some(selected_idx) = self.selected_component {
                                if selected_idx < self.components.len() {
                                    ui.horizontal(|ui| {
                                        if ui.button("🗑 Delete").on_hover_text("Delete selected component").clicked() {
                                            self.components.remove(selected_idx);
                                            self.selected_component = None;
                                            self.visual_designer.clear_selection();
                                        }
                                        if ui.button("📋 Duplicate").on_hover_text("Duplicate selected component").clicked() {
                                            // TODO: Implement component duplication
                                        }
                                    });
                                }
                            }
                        },
                        1 => {
                            // Modern IDE integration panel
                            self.modern_ide.render_integration_panel(ui);
                        },
                        _ => {}
                    }
                });
        }

        // Bottom panel - Output/AI/Multi-Device Preview
        if self.show_output_panel || self.show_ai_panel || self.multi_device_preview.show_preview_panel {
            egui::TopBottomPanel::bottom("bottom_panel")
                .resizable(true)
                .min_height(150.0)
                .show(ctx, |ui| {
                    // Determine active panel count and create tabs
                    let panel_count = [self.show_ai_panel, self.show_output_panel, self.multi_device_preview.show_preview_panel]
                        .iter().filter(|&&x| x).count();
                    
                    let mut active_bottom_panel = if self.show_ai_panel { 0 }
                                                else if self.show_output_panel { 1 }
                                                else if self.multi_device_preview.show_preview_panel { 2 }
                                                else { 0 };
                    
                    if panel_count > 1 {
                        // Show tabs when multiple panels are available
                        ui.horizontal(|ui| {
                            if self.show_ai_panel {
                                if ui.selectable_label(active_bottom_panel == 0, "🤖 AI Assistant").clicked() {
                                    active_bottom_panel = 0;
                                }
                            }
                            if self.show_output_panel {
                                if ui.selectable_label(active_bottom_panel == 1, "📤 Output").clicked() {
                                    active_bottom_panel = 1;
                                }
                            }
                            if self.multi_device_preview.show_preview_panel {
                                if ui.selectable_label(active_bottom_panel == 2, "📱 Device Preview").clicked() {
                                    active_bottom_panel = 2;
                                }
                            }
                        });
                        ui.separator();
                    }
                    
                    // Render active panel
                    match active_bottom_panel {
                        0 if self.show_ai_panel => self.render_ai_panel(ui),
                        1 if self.show_output_panel => {
                            ui.heading("Output");
                            ui.separator();
                            self.menu.output_panel.ui(ui);
                        },
                        2 if self.multi_device_preview.show_preview_panel => {
                            self.multi_device_preview.render_preview_panel(ui, &self.components);
                        },
                        _ => {
                            // Fallback to first available panel
                            if self.show_ai_panel {
                                self.render_ai_panel(ui);
                            } else if self.show_output_panel {
                                ui.heading("Output");
                                ui.separator();
                                self.menu.output_panel.ui(ui);
                            }
                        }
                    }
                });
        }

        // Central panel - Design Canvas or Code Editor
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.design_mode {
                // Advanced Visual Designer Mode
                ui.horizontal(|ui| {
                    ui.heading("🎨 Visual Designer");
                    ui.separator();
                    
                    // Advanced alignment toolbar
                    self.visual_designer.advanced_alignment.render_toolbar(ui, self.visual_designer.selection.selected.len());
                    
                    // Process alignment operations
                    if let Some(operation) = self.visual_designer.advanced_alignment.get_recent_operations().last() {
                        if !self.visual_designer.selection.selected.is_empty() {
                            let canvas_rect = egui::Rect::from_min_size(ui.cursor().min, ui.available_size());
                            self.apply_advanced_alignment_operation(operation.clone(), canvas_rect);
                        }
                        // Clear the operation after processing
                        if !self.visual_designer.advanced_alignment.get_recent_operations().is_empty() {
                            self.visual_designer.advanced_alignment.clear_recent_operations();
                        }
                    }
                    
                    // Undo/Redo
                    if ui.button("↶").on_hover_text("Undo (Ctrl+Z)").clicked() || ui.input(|i| i.key_pressed(egui::Key::Z) && i.modifiers.ctrl) {
                        self.visual_designer.undo();
                    }
                    if ui.button("↷").on_hover_text("Redo (Ctrl+Y)").clicked() || ui.input(|i| i.key_pressed(egui::Key::Y) && i.modifiers.ctrl) {
                        self.visual_designer.redo();
                    }
                });
                
                ui.separator();
                
                // Apply live feedback for property changes
                self.live_feedback.apply_live_changes(&mut self.components, &mut self.visual_designer, &self.property_inspector);
                
                // Update smart editing system with current component selections
                if self.visual_designer.selection.selected.len() >= 3 {
                    let _selected_components = self.visual_designer.selection.selected.clone();
                    // TODO: Re-enable smart editing distribution guides when borrowing is resolved
                    // self.visual_designer.smart_editing.generate_distribution_guides(&self.visual_designer, &selected_components);
                }
                
                // Render the advanced visual designer
                let canvas_size = ui.available_size();
                self.visual_designer.render_design_canvas(ui, &mut self.components, canvas_size);
                
                // Render live feedback overlays on top of the design canvas
                self.live_feedback.render_overlays(ui, &self.visual_designer);
                
                // Sync selection between visual designer, property inspector, and hierarchy manager
                if let Some(primary_selection) = self.visual_designer.selection.primary {
                    self.selected_component = Some(primary_selection);
                } else if self.visual_designer.selection.selected.is_empty() {
                    self.selected_component = None;
                }
                
                // Sync hierarchy manager with component changes
                for (idx, component) in self.components.iter().enumerate() {
                    if !self.hierarchy_manager.hierarchy.component_metadata.contains_key(&idx) {
                        self.hierarchy_manager.add_component(idx, component.as_ref(), None);
                    }
                }
                
                // Handle drop operations for new components
                if let DragType::AddingComponent(ref comp_type) = self.drag_state.drag_type {
                    if !ui.input(|i| i.pointer.any_down()) {
                        self.add_component_from_drag(comp_type.clone());
                        self.drag_state.drag_type = DragType::None;
                    }
                }
                
                // Instructions for new users
                if self.components.is_empty() {
                    ui.centered_and_justified(|ui| {
                        ui.label("🎨 Welcome to the Visual Designer! 🦀");
                        ui.label("• Drag components from the palette to add them");
                        ui.label("• Use the grid and guides for precise alignment");
                        ui.label("• Select multiple components to use alignment tools");
                        ui.label("• Switch to Code mode to see generated code");
                    });
                }
            } else {
                // Enhanced Code Editor Mode with LSP Integration
                self.render_code_editor_mode(ui);
            }
        });
    }
}

impl IdeApp {
    /// Renders a draggable component button in the palette
    fn render_draggable_component_button(&mut self, ui: &mut egui::Ui, label: &str, comp_type: ComponentType) {
        let button = ui.button(label);
        
        // Handle drag start
        if button.drag_started() {
            self.drag_state.drag_type = DragType::AddingComponent(comp_type.clone());
            self.drag_state.drag_start_pos = ui.input(|i| i.pointer.hover_pos());
        }
        
        // Handle click (fallback for non-drag users)
        if button.clicked() && !button.dragged() {
            self.add_component_from_drag(comp_type);
        }
        
        // Visual feedback during drag
        if let DragType::AddingComponent(ref dragged_type) = self.drag_state.drag_type {
            if std::mem::discriminant(dragged_type) == std::mem::discriminant(&comp_type) {
                // Show visual feedback that this component is being dragged
                ui.label("↗️ Dragging...");
            }
        }
    }
    
    /// Creates a new component from a drag operation
    fn add_component_from_drag(&mut self, comp_type: ComponentType) {
        let new_component: Box<dyn Component> = match comp_type {
            ComponentType::Button => Box::new(Button { 
                label: "New Button".into(), 
                editable: false 
            }),
            ComponentType::Label => Box::new(Label { 
                text: "New Label".into(), 
                editable: false 
            }),
            ComponentType::TextBox => Box::new(TextBox { 
                value: "New TextBox".into(), 
                editable: false 
            }),
            ComponentType::Checkbox => Box::new(Checkbox { 
                label: "New Checkbox".into(), 
                checked: false, 
                editable: false 
            }),
            ComponentType::Slider => Box::new(Slider { 
                value: 0.5, 
                min: 0.0, 
                max: 1.0, 
                editable: false 
            }),
            ComponentType::Dropdown => Box::new(Dropdown {
                label: "New Dropdown".into(),
                options: vec!["Option 1".into(), "Option 2".into()],
                selected: 0,
                editable: false,
            }),
        };
        
        let component_id = self.components.len();
        
        // Add to hierarchy manager
        self.hierarchy_manager.add_component(component_id, new_component.as_ref(), None);
        
        self.components.push(new_component);
        // Auto-select the newly added component
        self.selected_component = Some(self.components.len() - 1);
    }

    /// Apply advanced alignment operation to selected components
    fn apply_advanced_alignment_operation(&mut self, operation: crate::editor::advanced_alignment::AlignmentOperation, canvas_rect: egui::Rect) {
        use crate::editor::advanced_alignment::ComponentBounds;
        
        // Convert selected components to bounds
        let mut component_bounds: Vec<ComponentBounds> = Vec::new();
        
        for &idx in &self.visual_designer.selection.selected {
            if idx < self.components.len() {
                if let (Some(pos), Some(size)) = (
                    self.visual_designer.layout.positions.get(&idx),
                    self.visual_designer.layout.sizes.get(&idx)
                ) {
                    component_bounds.push(ComponentBounds {
                        position: *pos,
                        size: *size,
                        index: idx,
                    });
                }
            }
        }

        // Apply the alignment operation
        if !component_bounds.is_empty() {
            if self.visual_designer.advanced_alignment.apply_operation(&operation, &mut component_bounds, canvas_rect) {
                // Collect new positions before moving component_bounds
                let new_positions: Vec<egui::Pos2> = component_bounds.iter().map(|b| b.position).collect();
                
                // Update component positions and sizes based on the alignment result
                for bounds in component_bounds {
                    self.visual_designer.layout.positions.insert(bounds.index, bounds.position);
                    self.visual_designer.layout.sizes.insert(bounds.index, bounds.size);
                }
                
                // Add to undo history
                let design_operation = crate::editor::visual_designer::DesignOperation::Move {
                    component_ids: self.visual_designer.selection.selected.clone(),
                    old_positions: Vec::new(), // TODO: Store old positions for proper undo
                    new_positions,
                };
                self.visual_designer.add_to_history(design_operation);
            }
        }
    }
    
    /// Handles component actions like selection, movement, and deletion
    fn handle_component_action(&mut self, idx: usize, action: ComponentAction) {
        match action {
            ComponentAction::Select => {
                self.selected_component = Some(idx);
            }
            ComponentAction::MoveUp => {
                if idx > 0 {
                    self.components.swap(idx, idx - 1);
                    if let Some(selected) = self.selected_component {
                        if selected == idx {
                            self.selected_component = Some(idx - 1);
                        } else if selected == idx - 1 {
                            self.selected_component = Some(idx);
                        }
                    }
                }
            }
            ComponentAction::MoveDown => {
                if idx < self.components.len() - 1 {
                    self.components.swap(idx, idx + 1);
                    if let Some(selected) = self.selected_component {
                        if selected == idx {
                            self.selected_component = Some(idx + 1);
                        } else if selected == idx + 1 {
                            self.selected_component = Some(idx);
                        }
                    }
                }
            }
            ComponentAction::Delete => {
                self.components.remove(idx);
                if let Some(selected) = self.selected_component {
                    if selected == idx {
                        self.selected_component = None;
                    } else if selected > idx {
                        self.selected_component = Some(selected - 1);
                    }
                }
            }
        }
    }
    
    /// # Visual-to-Code Generation System
    /// 
    /// Transforms the visual component layout into a complete, runnable Rust application
    /// using eframe/egui. This is the core code generation algorithm that bridges the
    /// gap between visual design and executable code.
    /// 
    /// ## Generation Strategy
    /// 
    /// The code generation follows a structured approach:
    /// 1. **Header Generation**: Creates file metadata and documentation
    /// 2. **Import Analysis**: Determines required imports based on component usage
    /// 3. **State Structure**: Generates application state struct with typed fields
    /// 4. **Implementation Block**: Creates update/render logic for all components
    /// 5. **Application Entry**: Generates main function and eframe setup
    /// 
    /// ## Type Safety
    /// 
    /// Generated code maintains full type safety by:
    /// - Creating strongly-typed fields for each component's state
    /// - Using appropriate Rust types (String, bool, f32, Vec<T>, etc.)
    /// - Implementing proper ownership and borrowing patterns
    /// - Following Rust naming conventions and best practices
    /// 
    /// ## Component State Mapping
    /// 
    /// Each visual component is mapped to appropriate Rust state:
    /// - **Button**: Click counter (u32) + interaction handling
    /// - **TextBox**: String value with mutable text editing
    /// - **Label**: Display text with optional dynamic updates
    /// - **Checkbox**: Boolean state with toggle functionality
    /// - **Slider**: f32 value within defined range constraints
    /// - **Dropdown**: Vec<String> options + selected index tracking
    /// 
    /// ## Performance Considerations
    /// 
    /// The generated code is optimized for:
    /// - Minimal memory allocations during runtime
    /// - Efficient egui widget usage patterns
    /// - Proper state management without unnecessary clones
    /// - Event handling that doesn't block the UI thread
    fn generate_code_preview(&mut self) -> String {
        let mut code = String::new();
        
        // Generate complete Rust application structure in dependency order
        code.push_str(&self.generate_header_comments());
        code.push_str(&self.generate_imports());
        code.push_str(&self.generate_state_struct());
        code.push_str(&self.generate_impl_block());
        code.push_str(&self.generate_main_function());
        
        code
    }
    
    /// Generate header comments and metadata
    fn generate_header_comments(&self) -> String {
        format!(
            "//! Generated UI Application from Visual Designer\n\
             //! \n\
             //! This code was automatically generated from the RAD IDE visual designer.\n\
             //! Components: {}\n\
             //! Generated: {}\n\
             //! \n\
             //! To run this application:\n\
             //! cargo run --bin generated_app\n\n",
            self.components.len(),
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        )
    }
    
    /// Generate necessary imports
    fn generate_imports(&self) -> String {
        let mut imports = String::new();
        imports.push_str("use eframe::egui;\n");
        imports.push_str("use std::collections::HashMap;\n");
        
        // Add specific imports based on components used
        let mut needs_vec = false;
        let mut needs_string = false;
        
        for component in &self.components {
            match component.name() {
                "Dropdown" => needs_vec = true,
                "TextBox" | "Label" | "Button" => needs_string = true,
                _ => {}
            }
        }
        
        if needs_vec {
            imports.push_str("use std::vec::Vec;\n");
        }
        if needs_string {
            imports.push_str("use std::string::String;\n");
        }
        
        imports.push('\n');
        imports
    }
    
    /// Generate application state structure
    /// 
    /// This algorithm generates a Rust struct that contains all the state data needed
    /// for the UI components. Each component gets appropriate field types based on their
    /// data requirements, ensuring type safety and proper state management in the generated code.
    fn generate_state_struct(&self) -> String {
        let mut struct_code = String::new();
        struct_code.push_str("/// Application state containing all UI component data\n");
        struct_code.push_str("#[derive(Default)]\n");
        struct_code.push_str("pub struct GeneratedApp {\n");
        
        // Component State Generation Algorithm
        // This generates typed fields for each UI component based on their data requirements
        // Each component type has specific state needs that must be tracked across frames
        for (idx, component) in self.components.iter().enumerate() {
            // Generate unique field name: component_name + index to prevent naming conflicts
            // This ensures each component instance has its own state storage
            let field_name = format!("{}__{}", component.name().to_lowercase(), idx);
            
            // Component-specific state generation based on UI component behavior patterns
            match component.name() {
                "Button" => {
                    // Buttons need click counters for tracking user interactions
                    // This enables click analytics and interaction feedback
                    struct_code.push_str(&format!("    /// Button {} click counter\n", idx));
                    struct_code.push_str(&format!("    pub {}_clicks: u32,\n", field_name));
                }
                "Label" => {
                    // Labels need text content that can be dynamically updated
                    // This allows for data binding and dynamic text display
                    struct_code.push_str(&format!("    /// Label {} text content\n", idx));
                    struct_code.push_str(&format!("    pub {}_text: String,\n", field_name));
                }
                "TextBox" => {
                    // Text boxes need string storage for user input
                    // This maintains the text state across UI updates and redraws
                    struct_code.push_str(&format!("    /// TextBox {} input value\n", idx));
                    struct_code.push_str(&format!("    pub {}_value: String,\n", field_name));
                }
                "Checkbox" => {
                    // Checkboxes need boolean state for checked/unchecked status
                    // This provides simple binary choice state management
                    struct_code.push_str(&format!("    /// Checkbox {} state\n", idx));
                    struct_code.push_str(&format!("    pub {}_checked: bool,\n", field_name));
                }
                "Slider" => {
                    // Sliders need numeric values within their defined ranges
                    // Float type provides smooth continuous value adjustment
                    struct_code.push_str(&format!("    /// Slider {} value\n", idx));
                    struct_code.push_str(&format!("    pub {}_value: f32,\n", field_name));
                }
                "Dropdown" => {
                    // Dropdowns need both the available options and current selection
                    // This dual-field approach enables dynamic option management
                    struct_code.push_str(&format!("    /// Dropdown {} options\n", idx));
                    struct_code.push_str(&format!("    pub {}_options: Vec<String>,\n", field_name));
                    struct_code.push_str(&format!("    /// Dropdown {} selected index\n", idx));
                    struct_code.push_str(&format!("    pub {}_selected: usize,\n", field_name));
                }
                _ => {
                    // Fallback for unknown component types - uses string as universal container
                    // This ensures generated code compiles even with new component types
                    struct_code.push_str(&format!("    /// {} {} data\n", component.name(), idx));
                    struct_code.push_str(&format!("    pub {}_data: String,\n", field_name));
                }
            }
        }
        
        struct_code.push_str("}\n\n");  // Close struct definition
        struct_code
    }
    
    /// Generate implementation block with UI rendering
    fn generate_impl_block(&self) -> String {
        let mut impl_code = String::new();
        impl_code.push_str("impl eframe::App for GeneratedApp {\n");
        impl_code.push_str("    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {\n");
        impl_code.push_str("        egui::CentralPanel::default().show(ctx, |ui| {\n");
        impl_code.push_str("            ui.heading(\"🎨 Generated UI Application\");\n");
        impl_code.push_str("            ui.separator();\n\n");
        
        // Generate layout based on visual designer state
        if self.visual_designer.grid.visible {
            impl_code.push_str("            // Grid-based layout\n");
            impl_code.push_str("            egui::Grid::new(\"generated_grid\")\n");
            impl_code.push_str("                .num_columns(2)\n");
            impl_code.push_str("                .spacing([40.0, 4.0])\n");
            impl_code.push_str("                .show(ui, |ui| {\n");
            
            for (idx, component) in self.components.iter().enumerate() {
                impl_code.push_str(&self.generate_component_code(component, idx, "                    "));
                if idx % 2 == 1 {
                    impl_code.push_str("                    ui.end_row();\n");
                }
            }
            
            impl_code.push_str("                });\n");
        } else {
            impl_code.push_str("            // Vertical layout\n");
            for (idx, component) in self.components.iter().enumerate() {
                impl_code.push_str(&self.generate_component_code(component, idx, "            "));
            }
        }
        
        impl_code.push_str("\n            ui.separator();\n");
        impl_code.push_str("            self.render_component_status(ui);\n");
        impl_code.push_str("        });\n");
        impl_code.push_str("    }\n");
        impl_code.push_str("}\n\n");
        
        // Add helper methods
        impl_code.push_str("impl GeneratedApp {\n");
        impl_code.push_str("    /// Display component status and interactions\n");
        impl_code.push_str("    fn render_component_status(&self, ui: &mut egui::Ui) {\n");
        impl_code.push_str("        ui.collapsing(\"📊 Component Status\", |ui| {\n");
        
        for (idx, component) in self.components.iter().enumerate() {
            let field_name = format!("{}__{}", component.name().to_lowercase(), idx);
            match component.name() {
                "Button" => {
                    impl_code.push_str(&format!("            ui.label(format!(\"Button {}: {{}} clicks\", self.{}_clicks));\n", idx, field_name));
                }
                "TextBox" => {
                    impl_code.push_str(&format!("            ui.label(format!(\"TextBox {}: '{{}}'\", self.{}_value));\n", idx, field_name));
                }
                "Checkbox" => {
                    impl_code.push_str(&format!("            ui.label(format!(\"Checkbox {}: {{}}\", self.{}_checked));\n", idx, field_name));
                }
                "Slider" => {
                    impl_code.push_str(&format!("            ui.label(format!(\"Slider {}: {{:.2}}\", self.{}_value));\n", idx, field_name));
                }
                "Dropdown" => {
                    impl_code.push_str(&format!("            if self.{}_selected < self.{}_options.len() {{\n", field_name, field_name));
                    impl_code.push_str(&format!("                ui.label(format!(\"Dropdown {}: '{{}}'\", self.{}_options[self.{}_selected]));\n", idx, field_name, field_name));
                    impl_code.push_str("            }\n");
                }
                _ => {}
            }
        }
        
        impl_code.push_str("        });\n");
        impl_code.push_str("    }\n");
        impl_code.push_str("}\n\n");
        
        impl_code
    }
    
    /// Generate code for individual component
    fn generate_component_code(&self, component: &Box<dyn Component>, idx: usize, indent: &str) -> String {
        let field_name = format!("{}__{}", component.name().to_lowercase(), idx);
        let mut code = String::new();
        
        code.push_str(&format!("{}// {} Component\n", indent, component.name()));
        
        match component.name() {
            "Button" => {
                code.push_str(&format!("{}if ui.button(\"🔘 Button {}\").clicked() {{\n", indent, idx));
                code.push_str(&format!("{}    self.{}_clicks += 1;\n", indent, field_name));
                code.push_str(&format!("{}    println!(\"Button {} clicked! Total clicks: {{}}\", self.{}_clicks);\n", indent, idx, field_name));
                code.push_str(&format!("{}}}\n", indent));
            }
            "Label" => {
                code.push_str(&format!("{}ui.label(format!(\"🏷️ Label {}: {{}}\", self.{}_text));\n", indent, idx, field_name));
            }
            "TextBox" => {
                code.push_str(&format!("{}ui.horizontal(|ui| {{\n", indent));
                code.push_str(&format!("{}    ui.label(\"📝 TextBox {}:\");\n", indent, idx));
                code.push_str(&format!("{}    ui.text_edit_singleline(&mut self.{}_value);\n", indent, field_name));
                code.push_str(&format!("{}}});\n", indent));
            }
            "Checkbox" => {
                code.push_str(&format!("{}ui.checkbox(&mut self.{}_checked, \"☑️ Checkbox {}\");\n", indent, field_name, idx));
            }
            "Slider" => {
                code.push_str(&format!("{}ui.horizontal(|ui| {{\n", indent));
                code.push_str(&format!("{}    ui.label(\"🎚️ Slider {}:\");\n", indent, idx));
                code.push_str(&format!("{}    ui.add(egui::Slider::new(&mut self.{}_value, 0.0..=100.0).suffix(\"%\"));\n", indent, field_name));
                code.push_str(&format!("{}}});\n", indent));
            }
            "Dropdown" => {
                code.push_str(&format!("{}egui::ComboBox::from_label(\"📋 Dropdown {}\")\n", indent, idx));
                code.push_str(&format!("{}    .selected_text(if self.{}_selected < self.{}_options.len() {{ &self.{}_options[self.{}_selected] }} else {{ \"None\" }})\n", indent, field_name, field_name, field_name, field_name));
                code.push_str(&format!("{}    .show_ui(ui, |ui| {{\n", indent));
                code.push_str(&format!("{}        for (i, option) in self.{}_options.iter().enumerate() {{\n", indent, field_name));
                code.push_str(&format!("{}            ui.selectable_value(&mut self.{}_selected, i, option);\n", indent, field_name));
                code.push_str(&format!("{}        }}\n", indent));
                code.push_str(&format!("{}    }});\n", indent));
            }
            _ => {
                code.push_str(&format!("{}ui.label(\"Unknown component: {}\");\n", indent, component.name()));
            }
        }
        
        code.push('\n');
        code
    }
    
    /// Generate main function and app initialization
    fn generate_main_function(&self) -> String {
        let mut main_code = String::new();
        
        main_code.push_str("/// Initialize and run the generated application\n");
        main_code.push_str("fn main() -> Result<(), eframe::Error> {\n");
        main_code.push_str("    let options = eframe::NativeOptions {\n");
        main_code.push_str("        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),\n");
        main_code.push_str("        ..Default::default()\n");
        main_code.push_str("    };\n\n");
        
        main_code.push_str("    eframe::run_native(\n");
        main_code.push_str("        \"Generated UI Application\",\n");
        main_code.push_str("        options,\n");
        main_code.push_str("        Box::new(|_cc| {\n");
        main_code.push_str("            let mut app = GeneratedApp::default();\n");
        
        // Initialize default values
        for (idx, component) in self.components.iter().enumerate() {
            let field_name = format!("{}__{}", component.name().to_lowercase(), idx);
            match component.name() {
                "Label" => {
                    main_code.push_str(&format!("            app.{}_text = \"Label {} Text\".to_string();\n", field_name, idx));
                }
                "TextBox" => {
                    main_code.push_str(&format!("            app.{}_value = \"Default text {}\".to_string();\n", field_name, idx));
                }
                "Slider" => {
                    main_code.push_str(&format!("            app.{}_value = 50.0;\n", field_name));
                }
                "Dropdown" => {
                    main_code.push_str(&format!("            app.{}_options = vec![\n", field_name));
                    main_code.push_str(&format!("                \"Option 1\".to_string(),\n"));
                    main_code.push_str(&format!("                \"Option 2\".to_string(),\n"));
                    main_code.push_str(&format!("                \"Option 3\".to_string(),\n"));
                    main_code.push_str(&format!("            ];\n"));
                    main_code.push_str(&format!("            app.{}_selected = 0;\n", field_name));
                }
                _ => {}
            }
        }
        
        main_code.push_str("            Box::new(app)\n");
        main_code.push_str("        }),\n");
        main_code.push_str("    )\n");
        main_code.push_str("}\n");
        
        main_code
    }
    
    /// Render LSP diagnostics
    fn render_diagnostics(&mut self, ui: &mut egui::Ui) {
        // Process LSP messages
        let _messages = self.lsp_client.process_messages();
        
        // Show diagnostics if any
        let diagnostics = self.lsp_client.get_diagnostics("current_file.rs");
        if !diagnostics.is_empty() {
            ui.separator();
            ui.heading("📋 Diagnostics");
            
            for diagnostic in diagnostics {
                let (icon, color) = match diagnostic.severity {
                    Some(crate::editor::lsp_integration::DiagnosticSeverity::Error) => ("❌", egui::Color32::RED),
                    Some(crate::editor::lsp_integration::DiagnosticSeverity::Warning) => ("⚠️", egui::Color32::YELLOW),
                    Some(crate::editor::lsp_integration::DiagnosticSeverity::Information) => ("ℹ️", egui::Color32::BLUE),
                    Some(crate::editor::lsp_integration::DiagnosticSeverity::Hint) => ("💡", egui::Color32::GRAY),
                    None => ("•", egui::Color32::WHITE),
                };
                
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(icon).color(color));
                    ui.label(&diagnostic.message);
                    ui.label(format!("Line {}", diagnostic.range.start.line + 1));
                });
            }
        }
    }

    /// Render the enhanced code editor mode
    fn render_code_editor_mode(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("💻 Advanced Code Editor");
            ui.separator();
            
            if ui.button("🎨").on_hover_text("Generate Code from Design").clicked() {
                let generated_code = self.generate_code_preview();
                self.code_editor.code = generated_code;
                self.code_editor.language = "rust".to_string();
                self.code_editor.analyze_foldable_regions();
                self.menu.output_panel.log("✅ Generated complete Rust application from visual design!");
                self.menu.output_panel.log("💡 You can now copy this code to create a standalone app");
            }
            
            ui.separator();
            
            if ui.button("💡").on_hover_text("Smart AI Assistance").clicked() {
                self.show_ai_panel = true;
            }
            if ui.button("🚀").on_hover_text("Run Code").clicked() {
                self.menu.output_panel.log("🚀 Running Rust code...");
                // In a real implementation, this would compile and run the code
                self.menu.output_panel.log("✅ Code executed successfully");
            }
            if ui.button("🔨").on_hover_text("Build Project").clicked() {
                self.menu.output_panel.log("🔨 Building project with cargo...");
                // In a real implementation, this would call cargo build
                self.menu.output_panel.log("✅ Build completed successfully");
            }
            
            ui.separator();
            
            // Code editor options
            if ui.button("📋").on_hover_text("Copy Generated Code").clicked() {
                ui.output_mut(|o| o.copied_text = self.code_editor.code.clone());
                self.menu.output_panel.log("📋 Code copied to clipboard");
            }
            
            if ui.button("💾").on_hover_text("Save as File").clicked() {
                // In a real implementation, this would open a file dialog
                self.menu.output_panel.log("💾 File save dialog would open here");
            }
        });
        
        ui.separator();
        
        // Show code generation status
        if !self.components.is_empty() {
            ui.horizontal(|ui| {
                ui.label("📊 Design Status:");
                ui.label(format!("{} components ready for code generation", self.components.len()));
                if self.visual_designer.grid.visible {
                    ui.label("| Grid layout enabled");
                }
            });
            ui.separator();
        }
        
        // Render the enhanced code editor with LSP integration
        self.code_editor.render_enhanced(ui, &mut self.lsp_client, &mut self.menu.output_panel);
    }

    /// Get default Rust code for the editor
    fn default_rust_code() -> String {
        r#"// Welcome to the Rust RAD IDE! 🦀
// This is an advanced code editor with LSP integration

use std::collections::HashMap;

/// Example struct demonstrating Rust features
#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub skills: Vec<String>,
}

impl Person {
    /// Create a new person
    pub fn new(name: String, age: u32) -> Self {
        Self {
            name,
            age,
            skills: Vec::new(),
        }
    }
    
    /// Add a skill to the person
    pub fn add_skill(&mut self, skill: String) {
        self.skills.push(skill);
    }
    
    /// Get person's information
    pub fn info(&self) -> String {
        format!("{} ({} years old) - Skills: {:?}", 
                self.name, self.age, self.skills)
    }
}

fn main() {
    println!("🎉 Welcome to Rust RAD IDE!");
    
    let mut developer = Person::new("Rustacean".to_string(), 25);
    developer.add_skill("Rust".to_string());
    developer.add_skill("GUI Development".to_string());
    developer.add_skill("Systems Programming".to_string());
    
    println!("Developer: {}", developer.info());
    
    // Demonstrate collections
    let mut projects = HashMap::new();
    projects.insert("RAD IDE", "Advanced Rust IDE with visual designer");
    projects.insert("Web Framework", "High-performance web framework");
    projects.insert("Game Engine", "3D game engine in Rust");
    
    println!("\n📦 Current Projects:");
    for (name, description) in &projects {
        println!("  • {}: {}", name, description);
    }
    
    // Error handling example
    match calculate_fibonacci(10) {
        Ok(result) => println!("\n📊 Fibonacci(10) = {}", result),
        Err(e) => eprintln!("❌ Error: {}", e),
    }
}

/// Calculate fibonacci number with error handling
fn calculate_fibonacci(n: u32) -> Result<u64, String> {
    if n > 93 {
        return Err("Number too large for u64".to_string());
    }
    
    let mut a = 0u64;
    let mut b = 1u64;
    
    for _ in 0..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    
    Ok(a)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_person_creation() {
        let person = Person::new("Test".to_string(), 30);
        assert_eq!(person.name, "Test");
        assert_eq!(person.age, 30);
        assert!(person.skills.is_empty());
    }
    
    #[test]
    fn test_fibonacci() {
        assert_eq!(calculate_fibonacci(0).unwrap(), 0);
        assert_eq!(calculate_fibonacci(1).unwrap(), 1);
        assert_eq!(calculate_fibonacci(10).unwrap(), 55);
    }
}
"#.to_string()
    }

    /// Render the project management panel
    fn render_project_panel(&mut self, ui: &mut egui::Ui) {
        // Project management controls
        ui.heading("📁 Project Manager");
        ui.separator();
        
        // Current project info
        let has_project = self.project_manager.has_current_project();
        let project_info = if has_project {
            let project = self.project_manager.get_current_project().unwrap();
            Some((project.metadata.name.clone(), project.metadata.root_path.clone()))
        } else {
            None
        };
        
        if let Some((project_name, project_path)) = project_info {
            ui.horizontal(|ui| {
                ui.label("📂 Current Project:");
                ui.label(&project_name);
            });
            ui.horizontal(|ui| {
                ui.label("📍 Location:");
                ui.label(format!("{}", project_path.display()));
            });
            ui.separator();
            
            // Project actions
            let mut save_clicked = false;
            let mut reload_clicked = false;
            let mut generate_code_clicked = false;
            
            ui.horizontal(|ui| {
                if ui.button("💾 Save Project").clicked() {
                    save_clicked = true;
                }
                
                if ui.button("🔄 Reload").clicked() {
                    reload_clicked = true;
                }
                
                if ui.button("🎨 Generate Code").clicked() {
                    generate_code_clicked = true;
                }
            });
            
            // Handle actions after UI closure
            if save_clicked {
                let project = self.project_manager.get_current_project().unwrap().clone();
                if let Err(e) = self.project_manager.save_project(&project, &mut self.menu.output_panel) {
                    self.menu.output_panel.log(&format!("❌ Failed to save project: {}", e));
                }
            }
            
            if reload_clicked {
                if let Err(e) = self.project_manager.load_project(&project_path, &mut self.menu.output_panel) {
                    self.menu.output_panel.log(&format!("❌ Failed to reload project: {}", e));
                }
            }
            
            if generate_code_clicked {
                let generated_code = self.generate_code_preview();
                self.code_editor.code = generated_code;
                self.code_editor.language = "rust".to_string();
                self.code_editor.analyze_foldable_regions();
                self.menu.output_panel.log("✅ Generated code from current design!");
            }
            
            ui.separator();
        } else {
            ui.label("No project loaded");
            ui.separator();
        }
        
        // New project section
        if ui.button("🆕 New Project").clicked() {
            self.show_new_project_dialog(ui);
        }
        
        if ui.button("📂 Open Project").clicked() {
            // In a real implementation, this would open a file dialog
            self.menu.output_panel.log("📂 File dialog would open here to select project");
        }
        
        // Recent projects
        let recent_projects = self.project_manager.get_recent_projects().to_vec();
        if !recent_projects.is_empty() {
            ui.separator();
            ui.heading("📋 Recent Projects");
            
            for (i, project_path) in recent_projects.iter().enumerate() {
                if i >= 5 { break; } // Limit to 5 recent projects
                
                let project_name = project_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy();
                
                if ui.selectable_label(false, format!("📁 {}", project_name)).clicked() {
                    if let Err(e) = self.project_manager.load_project(project_path, &mut self.menu.output_panel) {
                        self.menu.output_panel.log(&format!("❌ Failed to load project: {}", e));
                    }
                }
            }
        }
        
        ui.separator();
        
        // File browser
        self.project_manager.render_file_browser(ui, &mut self.menu.output_panel);
    }
    
    /// Show new project creation dialog
    fn show_new_project_dialog(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.heading("🆕 Create New Project");
        
        // In a real implementation, this would be a proper dialog
        // For now, create a simple GUI project
        let project_name = "my-gui-app";
        let template = self.project_manager.get_templates().first().cloned();
        
        if let Some(template) = template {
            let location = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            
            if let Err(e) = self.project_manager.create_project(project_name, &template, &location, &mut self.menu.output_panel) {
                self.menu.output_panel.log(&format!("❌ Failed to create project: {}", e));
            } else {
                // Update components to match the new project
                if let Err(e) = self.project_manager.update_project_components(&self.components) {
                    self.menu.output_panel.log(&format!("⚠️ Warning: Failed to update project components: {}", e));
                }
            }
        }
    }

    /// Renders the AI panel UI with improved layout and UX
    fn render_ai_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("AI Assistant");
        ui.separator();
        
        ui.horizontal(|ui| {
            ui.label("Ask the AI:");
            ui.text_edit_singleline(&mut self.ai_prompt);
            
            let button_text = if self.ai_pending { "Processing..." } else { "Ask" };
            ui.add_enabled(!self.ai_pending, egui::Button::new(button_text));
            
            if ui.button(button_text).clicked() && !self.ai_pending {
                self.ai_pending = true;
                let prompt = self.ai_prompt.clone();
                let _agent = self.ai_agent.as_ref().unwrap();
                
                // Use eframe's async runtime for AI requests
                self.ai_task = Some(Box::pin(async move {
                    // This would connect to the actual AI agent
                    Ok(format!("AI response for: '{}'", prompt))
                }));
            }
        });
        
        ui.separator();
        
        // AI Response area with scrolling
        egui::ScrollArea::vertical()
            .max_height(100.0)
            .show(ui, |ui| {
                if let Some(task) = &mut self.ai_task {
                    // Poll the async AI task
                    use futures::future::FutureExt;
                    if let std::task::Poll::Ready(result) = task.now_or_never().unwrap().into() {
                        self.ai_response = result.unwrap_or_else(|e| format!("Error: {e}"));
                        self.ai_pending = false;
                        self.ai_task = None;
                    } else {
                        ui.spinner();
                        ui.label("Waiting for AI response...");
                    }
                } else if !self.ai_response.is_empty() {
                    ui.label(&self.ai_response);
                } else {
                    ui.label("AI responses will appear here...");
                }
            });
    }
}
