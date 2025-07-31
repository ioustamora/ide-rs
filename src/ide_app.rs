//! Main RAD IDE application using eframe/egui and RCL
//! 
//! This module contains the core IDE application structure with improved layout
//! supporting multiple panels, drag-and-drop component editing, and integrated AI assistance.

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

/// Main IDE application state containing all UI components, panels, and tools
pub struct IdeApp {
    /// Collection of UI components available in the designer
    pub components: Vec<Box<dyn Component>>,
    /// AI agent for code assistance and automation
    pub ai_agent: Option<AiAgent>,
    /// Current AI prompt being processed
    pub ai_prompt: String,
    /// Latest AI response
    pub ai_response: String,
    /// Flag indicating if AI request is pending
    pub ai_pending: bool,
    /// Async task handle for AI operations
    pub ai_task: Option<std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<String>> + Send>>>,
    /// Main IDE menu and toolbar system
    pub menu: IdeMenu,
    /// Panel visibility states for improved UX
    pub show_component_palette: bool,
    pub show_properties_inspector: bool,
    pub show_ai_panel: bool,
    pub show_output_panel: bool,
    /// Selected component index for property editing
    pub selected_component: Option<usize>,
    /// Drag and drop state for component manipulation
    pub drag_state: DragState,
    /// Advanced visual designer
    pub visual_designer: VisualDesigner,
    /// Smart AI assistant with context awareness
    pub smart_ai: SmartAiAssistant,
    /// LSP client for code intelligence
    pub lsp_client: LspClient,
    /// Design mode toggle
    pub design_mode: bool,
}

/// State management for drag and drop operations
#[derive(Default)]
pub struct DragState {
    /// Index of component being dragged, if any
    pub dragging_component: Option<usize>,
    /// Type of drag operation in progress
    pub drag_type: DragType,
    /// Position where drag started
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
                }
            });
        });

        // Left panel - Component Palette with drag support
        if self.show_component_palette {
            egui::SidePanel::left("component_palette")
                .resizable(true)
                .min_width(150.0)
                .show(ctx, |ui| {
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
                });
        }

        // Right panel - Properties Inspector
        if self.show_properties_inspector {
            egui::SidePanel::right("properties_inspector")
                .resizable(true)
                .min_width(200.0)
                .show(ctx, |ui| {
                    ui.heading("Properties");
                    ui.separator();
                    
                    if let Some(selected_idx) = self.selected_component {
                        if selected_idx < self.components.len() {
                            ui.label(format!("Selected: {}", self.components[selected_idx].name()));
                            ui.separator();
                            
                            // Component-specific property editing would go here
                            ui.label("Properties editing coming soon...");
                            
                            if ui.button("Delete Component").clicked() {
                                self.components.remove(selected_idx);
                                self.selected_component = None;
                            }
                        }
                    } else {
                        ui.label("No component selected");
                        ui.label("Click on a component in the designer to edit its properties.");
                    }
                });
        }

        // Bottom panel - Output/AI
        if self.show_output_panel || self.show_ai_panel {
            egui::TopBottomPanel::bottom("bottom_panel")
                .resizable(true)
                .min_height(150.0)
                .show(ctx, |ui| {
                    if self.show_ai_panel && self.show_output_panel {
                        // Show tabs when both panels are visible
                        ui.horizontal(|ui| {
                            ui.selectable_label(true, "AI Assistant");
                            ui.selectable_label(false, "Output");
                        });
                    }
                    
                    if self.show_ai_panel {
                        self.render_ai_panel(ui);
                    } else if self.show_output_panel {
                        ui.heading("Output");
                        ui.separator();
                        self.menu.output_panel.ui(ui);
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
                    
                    // Alignment tools for selected components
                    if self.visual_designer.selection.selected.len() >= 2 {
                        if ui.button("◀").on_hover_text("Align Left").clicked() {
                            self.visual_designer.apply_alignment(crate::editor::visual_designer::AlignmentOperation::AlignLeft);
                        }
                        if ui.button("▶").on_hover_text("Align Right").clicked() {
                            self.visual_designer.apply_alignment(crate::editor::visual_designer::AlignmentOperation::AlignRight);
                        }
                        if ui.button("▲").on_hover_text("Align Top").clicked() {
                            self.visual_designer.apply_alignment(crate::editor::visual_designer::AlignmentOperation::AlignTop);
                        }
                        if ui.button("▼").on_hover_text("Align Bottom").clicked() {
                            self.visual_designer.apply_alignment(crate::editor::visual_designer::AlignmentOperation::AlignBottom);
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
                
                // Render the advanced visual designer
                let canvas_size = ui.available_size();
                self.visual_designer.render_design_canvas(ui, &mut self.components, canvas_size);
                
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
                // Code Editor Mode
                ui.horizontal(|ui| {
                    ui.heading("💻 Code Editor");
                    ui.separator();
                    
                    if ui.button("💡").on_hover_text("Smart AI Assistance").clicked() {
                        self.show_ai_panel = true;
                    }
                    if ui.button("🔍").on_hover_text("Find & Replace").clicked() {
                        // TODO: Implement find/replace
                    }
                    if ui.button("📝").on_hover_text("Format Code").clicked() {
                        // TODO: Implement code formatting
                    }
                });
                
                ui.separator();
                
                // Code editor placeholder - this would be replaced with a proper code editor
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.text_edit_multiline(&mut self.generate_code_preview());
                });
                
                // Show LSP diagnostics if any
                self.render_diagnostics(ui);
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
        
        self.components.push(new_component);
        // Auto-select the newly added component
        self.selected_component = Some(self.components.len() - 1);
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
    
    /// Generate code preview from visual components
    fn generate_code_preview(&mut self) -> String {
        let mut code = String::from("// Generated UI Code from Visual Designer\n\n");
        code.push_str("use eframe::egui;\n");
        code.push_str("use crate::rcl::ui::component::Component;\n\n");
        code.push_str("pub fn render_ui(ui: &mut egui::Ui) {\n");
        
        for (idx, component) in self.components.iter().enumerate() {
            let _component_name = component.name().to_lowercase();
            code.push_str(&format!("    // Component {}: {}\n", idx, component.name()));
            
            match component.name() {
                "Button" => {
                    code.push_str("    if ui.button(\"Button Text\").clicked() {\n");
                    code.push_str("        // Handle button click\n");
                    code.push_str("    }\n");
                }
                "Label" => {
                    code.push_str("    ui.label(\"Label Text\");\n");
                }
                "TextBox" => {
                    code.push_str("    ui.text_edit_singleline(&mut text_value);\n");
                }
                "Checkbox" => {
                    code.push_str("    ui.checkbox(&mut checkbox_value, \"Checkbox Text\");\n");
                }
                "Slider" => {
                    code.push_str("    ui.add(egui::Slider::new(&mut slider_value, 0.0..=1.0));\n");
                }
                "Dropdown" => {
                    code.push_str("    egui::ComboBox::from_label(\"Select\")\n");
                    code.push_str("        .selected_text(\"Option\")\n");
                    code.push_str("        .show_ui(ui, |ui| {\n");
                    code.push_str("            ui.selectable_value(&mut selection, 0, \"Option 1\");\n");
                    code.push_str("        });\n");
                }
                _ => {
                    code.push_str(&format!("    // {} component\n", component.name()));
                }
            }
            code.push('\n');
        }
        
        code.push_str("}\n");
        code
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
