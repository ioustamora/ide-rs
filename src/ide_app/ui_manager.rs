//! # UI Manager
//! 
//! Manages the IDE's user interface layout, panel visibility, and UI orchestration.
//! This module handles the top-level UI structure and coordinates between different panels.

use eframe::egui;
use super::app_state::IdeAppState;

/// Alignment types for component alignment
#[derive(Debug, Clone, Copy)]
enum AlignmentType {
    Left,
    Right,
    Top,
    Bottom,
    CenterHorizontal,
    CenterVertical,
}

/// # UI Manager
/// 
/// Handles the overall UI layout and panel management for the IDE.
/// Responsible for rendering the main UI structure and coordinating panel visibility.
pub struct UiManager;

impl UiManager {
    /// Render the top menu bar and toolbar
    pub fn render_top_panel(app_state: &mut IdeAppState, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_panel").show(ctx, |ui| {
            // Menu bar
            ui.horizontal(|ui| {
                app_state.menu.ui(ui);
            });
            
            // Toolbar
            app_state.menu.toolbar.ui(ui, crate::editor::actions::get_actions(), &mut app_state.menu.output_panel);
            
            // Panel toggles and mode switches
            ui.horizontal(|ui| {
                ui.separator();
                Self::render_panel_toggles(app_state, ui);
                ui.separator();
                Self::render_mode_switches(app_state, ui);
                ui.separator();
                if app_state.design_mode {
                    Self::render_design_controls(app_state, ui);
                }
            });
        });
    }
    
    /// Render panel toggle buttons
    fn render_panel_toggles(app_state: &mut IdeAppState, ui: &mut egui::Ui) {
        if ui.button("🧰").on_hover_text("Toggle Component Palette").clicked() {
            app_state.show_component_palette = !app_state.show_component_palette;
        }
        if ui.button("🔧").on_hover_text("Toggle Properties Inspector").clicked() {
            app_state.show_properties_inspector = !app_state.show_properties_inspector;
        }
        if ui.button("🤖").on_hover_text("Toggle Smart AI Assistant").clicked() {
            app_state.show_ai_panel = !app_state.show_ai_panel;
        }
        if ui.button("📋").on_hover_text("Toggle Output Panel").clicked() {
            app_state.show_output_panel = !app_state.show_output_panel;
        }
        if ui.button("📁").on_hover_text("Toggle Project Explorer").clicked() {
            app_state.show_project_panel = !app_state.show_project_panel;
        }
        if ui.button("🗂").on_hover_text("Toggle Hierarchy Panel").clicked() {
            app_state.hierarchy_manager.show_hierarchy_panel = !app_state.hierarchy_manager.show_hierarchy_panel;
        }
        if ui.button("🚀").on_hover_text("Toggle Modern IDE Features").clicked() {
            app_state.show_modern_ide_panel = !app_state.show_modern_ide_panel;
        }
    }
    
    /// Render mode switch buttons
    fn render_mode_switches(app_state: &mut IdeAppState, ui: &mut egui::Ui) {
        // Check if we have an active file tab to determine mode switching availability
        let can_switch_modes = app_state.file_manager.get_active_tab().is_some();
        
        // Determine current mode based on active file type or fallback to design mode
        let current_mode = if let Some(active_tab) = app_state.file_manager.get_active_tab() {
            match active_tab.file_type {
                crate::editor::file_manager::FileType::UIDesign => true,  // Design mode
                crate::editor::file_manager::FileType::Code(_) | 
                crate::editor::file_manager::FileType::Unknown => false, // Code mode
            }
        } else {
            app_state.design_mode // Fallback to current state when no file is open
        };
        
        ui.add_enabled_ui(can_switch_modes, |ui| {
            if ui.selectable_label(current_mode, "🎨 Design").on_hover_text("Visual Designer Mode").clicked() {
                if let Some(active_tab) = app_state.file_manager.get_active_tab() {
                    match active_tab.file_type {
                        crate::editor::file_manager::FileType::UIDesign => {
                            app_state.design_mode = true;
                        }
                        crate::editor::file_manager::FileType::Code(_) | 
                        crate::editor::file_manager::FileType::Unknown => {
                            // For code files, create a new UI design file
                            let _ = app_state.file_manager.open_file(
                                std::path::PathBuf::from("Untitled.ui"), 
                                String::new()
                            );
                            app_state.design_mode = true;
                        }
                    }
                } else {
                    // No file open, create new UI design
                    let _ = app_state.file_manager.open_file(
                        std::path::PathBuf::from("Untitled.ui"), 
                        String::new()
                    );
                    app_state.design_mode = true;
                }
            }
            
            if ui.selectable_label(!current_mode, "💻 Code").on_hover_text("Code Editor Mode").clicked() {
                if let Some(active_tab) = app_state.file_manager.get_active_tab() {
                    match active_tab.file_type {
                        crate::editor::file_manager::FileType::Code(_) | 
                        crate::editor::file_manager::FileType::Unknown => {
                            app_state.design_mode = false;
                        }
                        crate::editor::file_manager::FileType::UIDesign => {
                            // For UI files, create a new code file
                            let _ = app_state.file_manager.open_file(
                                std::path::PathBuf::from("Untitled.rs"), 
                                IdeAppState::default_rust_code()
                            );
                            app_state.design_mode = false;
                        }
                    }
                } else {
                    // No file open, create new code file
                    let _ = app_state.file_manager.open_file(
                        std::path::PathBuf::from("Untitled.rs"), 
                        IdeAppState::default_rust_code()
                    );
                    app_state.design_mode = false;
                }
            }
        });
        
        // Show hint when no file is open
        if !can_switch_modes {
            ui.label("Open a file to switch modes");
        }
    }
    
    /// Render design mode specific controls
    fn render_design_controls(app_state: &mut IdeAppState, ui: &mut egui::Ui) {
        // Grid and snap controls
        if ui.selectable_label(app_state.visual_designer.grid.visible, "📐").on_hover_text("Toggle Grid").clicked() {
            app_state.visual_designer.grid.visible = !app_state.visual_designer.grid.visible;
        }
        if ui.selectable_label(app_state.visual_designer.snap_to_grid, "🔗").on_hover_text("Snap to Grid").clicked() {
            app_state.visual_designer.toggle_snap_to_grid();
        }
        if ui.selectable_label(app_state.visual_designer.guides.rulers_visible, "📏").on_hover_text("Toggle Rulers").clicked() {
            app_state.visual_designer.guides.rulers_visible = !app_state.visual_designer.guides.rulers_visible;
        }
        if ui.selectable_label(app_state.visual_designer.show_alignment_guides, "📐").on_hover_text("Alignment Guides").clicked() {
            app_state.visual_designer.toggle_alignment_guides();
        }
        
        ui.separator();
        
        // Layout tools
        if ui.button("🎯").on_hover_text("Align Selected").clicked() {
            Self::show_alignment_options(app_state, ui);
        }
        if ui.button("🔄").on_hover_text("Auto Distribute").clicked() {
            Self::auto_distribute_components(app_state);
        }
        
        ui.separator();
        if ui.selectable_label(app_state.multi_device_preview.enabled, "📱").on_hover_text("Multi-Device Preview").clicked() {
            app_state.multi_device_preview.toggle_preview();
        }
    }
    
    /// Render the left panel with project explorer, component palette, and hierarchy
    pub fn render_left_panel(app_state: &mut IdeAppState, ctx: &egui::Context) {
        if app_state.show_project_panel || app_state.show_component_palette || app_state.hierarchy_manager.show_hierarchy_panel {
            egui::SidePanel::left("left_panel")
                .resizable(true)
                .min_width(200.0)
                .default_width(250.0)
                .show(ctx, |ui| {
                    Self::render_left_panel_tabs(app_state, ui);
                });
        }
    }
    
    /// Render the tabs in the left panel
    fn render_left_panel_tabs(app_state: &mut IdeAppState, ui: &mut egui::Ui) {
        // Tab headers
        ui.horizontal(|ui| {
            if app_state.show_project_panel {
                if ui.selectable_label(app_state.active_left_tab == "project", "📁 Project").clicked() {
                    app_state.active_left_tab = "project".to_string();
                }
            }
            if app_state.show_component_palette {
                if ui.selectable_label(app_state.active_left_tab == "palette", "🧰 Components").clicked() {
                    app_state.active_left_tab = "palette".to_string();
                }
            }
            if app_state.hierarchy_manager.show_hierarchy_panel {
                if ui.selectable_label(app_state.active_left_tab == "hierarchy", "🗂 Hierarchy").clicked() {
                    app_state.active_left_tab = "hierarchy".to_string();
                }
            }
        });
        
        ui.separator();
        
        // Tab content
        match app_state.active_left_tab.as_str() {
            "project" if app_state.show_project_panel => {
                Self::render_project_explorer(app_state, ui);
            }
            "palette" if app_state.show_component_palette => {
                Self::render_component_palette(app_state, ui);
            }
            "hierarchy" if app_state.hierarchy_manager.show_hierarchy_panel => {
                Self::render_hierarchy_panel(app_state, ui);
            }
            _ => {
                ui.label("No active panel");
            }
        }
    }
    
    /// Render the project explorer
    fn render_project_explorer(app_state: &mut IdeAppState, ui: &mut egui::Ui) {
        ui.heading("Project Explorer");
        ui.separator();
        
        // Add new GUI project creation button
        if ui.button("🚀 New GUI Project").clicked() {
            Self::show_new_gui_project_dialog(app_state, ui);
        }
        
        ui.separator();
        
        // Handle file opening from project explorer
        if let Some(file_path) = app_state.project_manager.render_project_ui(ui, &mut app_state.menu.output_panel) {
            Self::open_file_in_editor(app_state, file_path);
        }
    }
    
    /// Show new GUI project creation dialog
    fn show_new_gui_project_dialog(app_state: &mut IdeAppState, ui: &mut egui::Ui) {
        ui.label("Create New GUI Project");
        ui.separator();
        
        ui.horizontal(|ui| {
            ui.label("Project Name:");
            ui.text_edit_singleline(&mut app_state.new_project_name);
        });
        
        ui.horizontal(|ui| {
            ui.label("Location:");
            ui.text_edit_singleline(&mut app_state.new_project_location);
            if ui.button("Browse...").clicked() {
                // TODO: Integrate with file picker
                app_state.new_project_location = std::env::current_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| ".".to_string());
            }
        });
        
        ui.horizontal(|ui| {
            if ui.button("Create Project").clicked() && !app_state.new_project_name.is_empty() {
                Self::create_new_gui_project(app_state);
            }
            if ui.button("Cancel").clicked() {
                app_state.new_project_name.clear();
                app_state.new_project_location.clear();
            }
        });
    }
    
    /// Create new GUI project with cargo integration
    fn create_new_gui_project(app_state: &mut IdeAppState) {
        
        let project_name = app_state.new_project_name.clone();
        let location = if app_state.new_project_location.is_empty() {
            std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
        } else {
            std::path::PathBuf::from(&app_state.new_project_location)
        };
        
        // Create project using cargo new integration
        match app_state.project_manager.operations.create_gui_project_with_cargo(
            &project_name,
            &location,
            &mut app_state.menu.output_panel
        ) {
            Ok(project) => {
                app_state.menu.output_panel.log("✅ GUI project created successfully!");
                
                // Load project into IDE
                app_state.project_manager.current_project = Some(project.clone());
                
                // Automatically open in design mode with visual designer
                app_state.design_mode = true;
                
                // Load default components into visual designer
                app_state.components.clear();
                Self::load_project_components_to_designer(&project, app_state);
                
                // Open the ui.rs file in the visual designer
                let ui_file_path = project.metadata.root_path.join("src").join("ui.rs");
                if ui_file_path.exists() {
                    app_state.menu.output_panel.log("🎨 Opening ui.rs in visual designer...");
                    // TODO: Integrate with file manager to open the file
                }
                
                app_state.menu.output_panel.log("🚀 Project ready for visual design!");
            }
            Err(e) => {
                app_state.menu.output_panel.log(&format!("❌ Failed to create project: {}", e));
            }
        }
        
        // Clear form
        app_state.new_project_name.clear();
        app_state.new_project_location.clear();
    }
    
    /// Load project components into visual designer
    fn load_project_components_to_designer(project: &crate::editor::project_manager::project::IdeProject, app_state: &mut IdeAppState) {
        use crate::rcl::ui::component::Component;
        
        // Create UI components from project component data
        for comp_data in &project.designer_data.components {
            let component: Box<dyn Component> = match comp_data.component_type.as_str() {
                "Button" => {
                    let label = comp_data.properties.get("label")
                        .cloned()
                        .unwrap_or_else(|| "Button".to_string());
                    Box::new(crate::rcl::ui::basic::button::Button::new(label))
                }
                "Label" => {
                    let text = comp_data.properties.get("text")
                        .cloned()
                        .unwrap_or_else(|| "Label".to_string());
                    Box::new(crate::rcl::ui::basic::label::Label::new(text))
                }
                _ => {
                    // Default to button
                    Box::new(crate::rcl::ui::basic::button::Button::new("Component".to_string()))
                }
            };
            
            app_state.components.push(component);
            
            // Set position and size in visual designer
            let idx = app_state.components.len() - 1;
            app_state.visual_designer.layout.positions.insert(idx, egui::Pos2::new(comp_data.position.0, comp_data.position.1));
            app_state.visual_designer.layout.sizes.insert(idx, egui::Vec2::new(comp_data.size.0, comp_data.size.1));
        }
    }
    
    /// Render the component palette
    fn render_component_palette(app_state: &mut IdeAppState, ui: &mut egui::Ui) {
        ui.heading("Component Palette");
        ui.separator();
        Self::render_palette_components(app_state, ui);
    }
    
    /// Render the hierarchy panel
    fn render_hierarchy_panel(app_state: &mut IdeAppState, ui: &mut egui::Ui) {
        ui.heading("Component Hierarchy");
        ui.separator();
        app_state.hierarchy_manager.render_hierarchy_panel(ui, &app_state.components);
    }
    
    /// Render palette components with drag and drop support
    fn render_palette_components(app_state: &mut IdeAppState, ui: &mut egui::Ui) {
        use super::drag_drop::{ComponentType, ComponentCategory, DragType};
        use super::animated_ui::AnimatedCollapsing;
        use std::collections::BTreeMap;
        
        let component_types = [
            ComponentType::Button,
            ComponentType::Label,
            ComponentType::TextBox,
            ComponentType::Checkbox,
            ComponentType::Slider,
            ComponentType::Dropdown,
            ComponentType::Panel,
            ComponentType::Image,
            ComponentType::Chart,
            ComponentType::Table,
            ComponentType::Tree,
            ComponentType::Custom(1),  // Progress Bar
            ComponentType::Custom(2),  // Tab Control
            ComponentType::Custom(3),  // Menu Bar
            ComponentType::Custom(4),  // Toolbar
            ComponentType::Custom(5),  // Status Bar
            ComponentType::Custom(6),  // Split Container
            ComponentType::Custom(7),  // Calendar
            ComponentType::Custom(8),  // Color Picker
            ComponentType::Custom(9),  // File Picker
            ComponentType::Custom(10), // Rich Text Editor
            ComponentType::Custom(11), // Code Editor
        ];
        
        // Group components by category
        let mut categories: BTreeMap<ComponentCategory, Vec<ComponentType>> = BTreeMap::new();
        for &component_type in &component_types {
            categories.entry(component_type.category())
                     .or_insert_with(Vec::new)
                     .push(component_type);
        }
        
        // Collect component actions to avoid borrow checker issues
        let mut component_actions = Vec::new();
        
        // Render each category with smooth animations
        for (category, components) in categories {
            let category_id = egui::Id::new(format!("palette_category_{:?}", category));
            
            AnimatedCollapsing::new(
                category_id,
                format!("{} {}", category.icon(), category.display_name()),
                &mut app_state.animation_manager
            )
            .default_open(true)
            .show(ui, |ui| {
                for component_type in components {
                    ui.horizontal(|ui| {
                        ui.label(component_type.icon());
                        let button = ui.button(component_type.display_name());
                        
                        // Handle drag start from palette
                        if button.drag_started() {
                            component_actions.push((component_type, "drag", button.rect.center()));
                        } else if button.clicked() {
                            // Add component directly on click
                            component_actions.push((component_type, "click", egui::Pos2::new(100.0, 100.0)));
                        }
                    });
                }
            });
        }
        
        // Execute actions after borrowing is complete
        for (component_type, action, position) in component_actions {
            match action {
                "drag" => {
                    let drag_type = DragType::ComponentFromPalette(component_type);
                    app_state.visual_designer.drag_state.start_drag(drag_type, position);
                }
                "click" => {
                    Self::add_component_to_form(app_state, component_type, position);
                }
                _ => {}
            }
        }
    }
    
    /// Render the right panel with properties inspector and modern IDE features
    pub fn render_right_panel(app_state: &mut IdeAppState, ctx: &egui::Context) {
        if app_state.show_properties_inspector || app_state.show_modern_ide_panel {
            egui::SidePanel::right("right_panel")
                .resizable(true)
                .min_width(200.0)
                .default_width(300.0)
                .show(ctx, |ui| {
                    Self::render_right_panel_tabs(app_state, ui);
                });
        }
    }
    
    /// Render the tabs in the right panel
    fn render_right_panel_tabs(app_state: &mut IdeAppState, ui: &mut egui::Ui) {
        // Tab headers
        ui.horizontal(|ui| {
            if ui.selectable_label(app_state.active_right_tab == "objects", "🔍 Objects").clicked() {
                app_state.active_right_tab = "objects".to_string();
            }
            if app_state.show_properties_inspector {
                if ui.selectable_label(app_state.active_right_tab == "properties", "🔧 Properties").clicked() {
                    app_state.active_right_tab = "properties".to_string();
                }
            }
            if app_state.show_modern_ide_panel {
                if ui.selectable_label(app_state.active_right_tab == "modern", "🚀 Modern IDE").clicked() {
                    app_state.active_right_tab = "modern".to_string();
                }
            }
        });
        
        ui.separator();
        
        // Tab content
        match app_state.active_right_tab.as_str() {
            "objects" => {
                Self::render_object_inspector(app_state, ui);
            }
            "properties" if app_state.show_properties_inspector => {
                Self::render_properties_inspector(app_state, ui);
            }
            "modern" if app_state.show_modern_ide_panel => {
                Self::render_modern_ide_panel(app_state, ui);
            }
            _ => {
                ui.label("No active panel");
            }
        }
    }
    
    /// Render the Object Inspector
    fn render_object_inspector(app_state: &mut IdeAppState, ui: &mut egui::Ui) {
        // Get current visual designer selection
        let selected_components = &app_state.visual_designer.selection.selected;
        
        // Render the Object Inspector UI and handle component selection
        if let Some(clicked_component) = app_state.object_inspector.render_ui(
            ui,
            &mut app_state.components,
            &app_state.root_form,
            selected_components,
        ) {
            // Update visual designer selection based on Object Inspector click
            app_state.visual_designer.selection.selected.clear();
            
            if clicked_component == usize::MAX {
                // Form was clicked
                app_state.visual_designer.selection.selected.insert(usize::MAX);
                app_state.visual_designer.selection.primary = Some(usize::MAX);
                app_state.selected_component = Some(usize::MAX);
            } else if clicked_component < app_state.components.len() {
                // Component was clicked
                app_state.visual_designer.selection.selected.insert(clicked_component);
                app_state.visual_designer.selection.primary = Some(clicked_component);
                app_state.selected_component = Some(clicked_component);
            }
            
            // Update Object Inspector selection to match
            app_state.object_inspector.select_component(Some(clicked_component));
        }
        
        // Synchronize Object Inspector selection with visual designer
        if let Some(primary) = app_state.visual_designer.selection.primary {
            if app_state.object_inspector.get_selected_component() != Some(primary) {
                app_state.object_inspector.select_component(Some(primary));
            }
        } else if !app_state.visual_designer.selection.selected.is_empty() {
            // Select the first component if no primary selection
            let first_selected = *app_state.visual_designer.selection.selected.iter().next().unwrap();
            app_state.object_inspector.select_component(Some(first_selected));
        }
    }
    
    /// Render the properties inspector
    fn render_properties_inspector(app_state: &mut IdeAppState, ui: &mut egui::Ui) {
        ui.heading("Properties");
        ui.separator();
        
        if let Some(selected_idx) = app_state.selected_component {
            if selected_idx == usize::MAX {
                // Form is selected - show form properties
                Self::render_form_properties(app_state, ui);
            } else if selected_idx < app_state.components.len() {
                // Use the basic property inspector
                app_state.property_inspector.render_component_properties(ui, &mut app_state.components[selected_idx]);
            } else {
                ui.label("Invalid component selection");
            }
        } else {
            ui.label("No component selected");
            ui.label("Click on the form background or a component to edit its properties");
        }
    }
    
    /// Render form properties when form is selected using advanced property inspector
    fn render_form_properties(app_state: &mut IdeAppState, ui: &mut egui::Ui) {
        // Use the advanced property inspector for the form
        app_state.property_inspector.render_form_properties(ui, &mut app_state.root_form);
    }
    
    /// Render the modern IDE features panel
    fn render_modern_ide_panel(app_state: &mut IdeAppState, ui: &mut egui::Ui) {
        ui.heading("Modern IDE");
        ui.separator();
        app_state.modern_ide.render_integration_panel(ui);
    }
    
    /// Render the bottom panel with AI assistant and output
    pub fn render_bottom_panel(app_state: &mut IdeAppState, ctx: &egui::Context) {
        if app_state.show_ai_panel || app_state.show_output_panel {
            egui::TopBottomPanel::bottom("bottom_panel")
                .resizable(true)
                .min_height(100.0)
                .default_height(200.0)
                .show(ctx, |ui| {
                    Self::render_bottom_panel_tabs(app_state, ui);
                });
        }
    }
    
    /// Render the tabs in the bottom panel
    fn render_bottom_panel_tabs(app_state: &mut IdeAppState, ui: &mut egui::Ui) {
        let mut active_tab = "output";
        if app_state.show_ai_panel { active_tab = "ai"; }
        
        // Tab headers
        ui.horizontal(|ui| {
            if app_state.show_output_panel {
                if ui.selectable_label(active_tab == "output", "📋 Output").clicked() {
                    active_tab = "output";
                }
            }
            if app_state.show_ai_panel {
                if ui.selectable_label(active_tab == "ai", "🤖 AI Assistant").clicked() {
                    active_tab = "ai";
                }
            }
        });
        
        ui.separator();
        
        // Tab content
        match active_tab {
            "output" if app_state.show_output_panel => {
                Self::render_output_panel(app_state, ui);
            }
            "ai" if app_state.show_ai_panel => {
                Self::render_ai_panel(app_state, ui);
            }
            _ => {
                ui.label("No active panel");
            }
        }
    }
    
    /// Render the output panel
    fn render_output_panel(app_state: &mut IdeAppState, ui: &mut egui::Ui) {
        ui.heading("Build Output");
        ui.separator();
        app_state.menu.output_panel.ui(ui);
    }
    
    /// Render the AI assistant panel
    fn render_ai_panel(app_state: &mut IdeAppState, ui: &mut egui::Ui) {
        ui.heading("AI Assistant");
        ui.separator();
        
        // AI prompt input
        ui.horizontal(|ui| {
            ui.label("Prompt:");
            ui.text_edit_singleline(&mut app_state.ai_prompt);
            if ui.button("Send").clicked() && !app_state.ai_prompt.is_empty() {
                // TODO: Integrate with AI system
                app_state.ai_response = format!("AI response to: {}", app_state.ai_prompt);
                app_state.ai_prompt.clear();
            }
        });
        
        ui.separator();
        
        // AI response display
        if !app_state.ai_response.is_empty() {
            ui.label("Response:");
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(&app_state.ai_response);
            });
        }
        
        // Smart AI assistant integration
        app_state.smart_ai.render_ai_panel(ui);
    }
    
    /// Add a new component to the form
    fn add_component_to_form(app_state: &mut IdeAppState, component_type: super::drag_drop::ComponentType, position: egui::Pos2) {
        use crate::rcl::ui::component::Component;
        
        let component: Box<dyn Component> = match component_type {
            super::drag_drop::ComponentType::Button => {
                Box::new(crate::rcl::ui::basic::button::Button::new("Button".to_string()))
            }
            super::drag_drop::ComponentType::Label => {
                Box::new(crate::rcl::ui::basic::label::Label::new("Label".to_string()))
            }
            super::drag_drop::ComponentType::TextBox => {
                Box::new(crate::rcl::ui::basic::textbox::TextBox::new("".to_string()))
            }
            super::drag_drop::ComponentType::Checkbox => {
                Box::new(crate::rcl::ui::basic::checkbox::Checkbox::new("Checkbox".to_string(), false))
            }
            super::drag_drop::ComponentType::Slider => {
                Box::new(crate::rcl::ui::basic::slider::Slider::new(0.0, 0.0, 100.0))
            }
            _ => {
                // Default to button for unsupported types
                Box::new(crate::rcl::ui::basic::button::Button::new("New Component".to_string()))
            }
        };
        
        let component_idx = app_state.components.len();
        app_state.components.push(component);
        
        // Set the layout position using the visual designer
        app_state.visual_designer.layout.positions.insert(component_idx, position);
    }
    
    /// Show alignment options for selected components
    fn show_alignment_options(app_state: &mut IdeAppState, ui: &mut egui::Ui) {
        if app_state.visual_designer.selection.selected.is_empty() {
            return;
        }
        
        let button_response = ui.button("🎯");
        egui::popup::popup_below_widget(ui, egui::Id::new("alignment_popup"), &button_response, |ui| {
            ui.set_min_width(150.0);
            ui.label("Align Selected Components:");
            ui.separator();
            
            if ui.button("⬅️ Align Left").clicked() {
                Self::align_components(app_state, AlignmentType::Left);
                ui.close_menu();
            }
            if ui.button("➡️ Align Right").clicked() {
                Self::align_components(app_state, AlignmentType::Right);
                ui.close_menu();
            }
            if ui.button("⬆️ Align Top").clicked() {
                Self::align_components(app_state, AlignmentType::Top);
                ui.close_menu();
            }
            if ui.button("⬇️ Align Bottom").clicked() {
                Self::align_components(app_state, AlignmentType::Bottom);
                ui.close_menu();
            }
            if ui.button("↔️ Center Horizontal").clicked() {
                Self::align_components(app_state, AlignmentType::CenterHorizontal);
                ui.close_menu();
            }
            if ui.button("↕️ Center Vertical").clicked() {
                Self::align_components(app_state, AlignmentType::CenterVertical);
                ui.close_menu();
            }
        });
    }
    
    /// Auto-distribute components evenly
    fn auto_distribute_components(app_state: &mut IdeAppState) {
        let selected = &app_state.visual_designer.selection.selected;
        if selected.len() < 3 {
            return; // Need at least 3 components to distribute
        }
        
        // Get positions of selected components
        let mut positions: Vec<(usize, egui::Pos2)> = selected.iter()
            .filter_map(|&idx| {
                app_state.visual_designer.layout.positions.get(&idx)
                    .map(|&pos| (idx, pos))
            })
            .collect();
        
        // Sort by X position for horizontal distribution
        positions.sort_by(|a, b| a.1.x.partial_cmp(&b.1.x).unwrap());
        
        if positions.len() >= 3 {
            let first_x = positions[0].1.x;
            let last_x = positions[positions.len() - 1].1.x;
            let spacing = (last_x - first_x) / (positions.len() - 1) as f32;
            
            for (i, (idx, pos)) in positions.iter().enumerate() {
                if i > 0 && i < positions.len() - 1 {
                    let new_x = first_x + i as f32 * spacing;
                    let new_pos = egui::Pos2::new(new_x, pos.y);
                    app_state.visual_designer.layout.positions.insert(*idx, new_pos);
                }
            }
        }
    }
    
    
    /// Align selected components
    fn align_components(app_state: &mut IdeAppState, alignment: AlignmentType) {
        let selected = &app_state.visual_designer.selection.selected;
        if selected.len() < 2 {
            return;
        }
        
        // Get positions of selected components
        let positions: Vec<(usize, egui::Pos2)> = selected.iter()
            .filter_map(|&idx| {
                app_state.visual_designer.layout.positions.get(&idx)
                    .map(|&pos| (idx, pos))
            })
            .collect();
        
        if positions.is_empty() {
            return;
        }
        
        // Calculate alignment reference
        let reference = match alignment {
            AlignmentType::Left => positions.iter().map(|(_, pos)| pos.x).fold(f32::INFINITY, f32::min),
            AlignmentType::Right => positions.iter().map(|(_, pos)| pos.x).fold(f32::NEG_INFINITY, f32::max),
            AlignmentType::Top => positions.iter().map(|(_, pos)| pos.y).fold(f32::INFINITY, f32::min),
            AlignmentType::Bottom => positions.iter().map(|(_, pos)| pos.y).fold(f32::NEG_INFINITY, f32::max),
            AlignmentType::CenterHorizontal => {
                let sum: f32 = positions.iter().map(|(_, pos)| pos.x).sum();
                sum / positions.len() as f32
            }
            AlignmentType::CenterVertical => {
                let sum: f32 = positions.iter().map(|(_, pos)| pos.y).sum();
                sum / positions.len() as f32
            }
        };
        
        // Apply alignment
        for (idx, pos) in positions {
            let new_pos = match alignment {
                AlignmentType::Left | AlignmentType::Right | AlignmentType::CenterHorizontal => {
                    egui::Pos2::new(reference, pos.y)
                }
                AlignmentType::Top | AlignmentType::Bottom | AlignmentType::CenterVertical => {
                    egui::Pos2::new(pos.x, reference)
                }
            };
            
            // Apply snap-to-grid if enabled
            let snapped_pos = app_state.visual_designer.snap_to_grid(new_pos);
            app_state.visual_designer.layout.positions.insert(idx, snapped_pos);
        }
    }
    
    /// Open a file in the appropriate editor
    fn open_file_in_editor(app_state: &mut IdeAppState, file_path: std::path::PathBuf) {
        // Try to read the file content
        match std::fs::read_to_string(&file_path) {
            Ok(content) => {
                // Open the file in the file manager
                if let Err(e) = app_state.file_manager.open_file(file_path.clone(), content) {
                    app_state.menu.output_panel.log(&format!("❌ Failed to open file: {}", e));
                    return;
                }
                
                // Switch to appropriate mode based on file type
                if let Some(active_tab) = app_state.file_manager.get_active_tab() {
                    match active_tab.file_type {
                        crate::editor::file_manager::FileType::UIDesign => {
                            app_state.design_mode = true;
                            app_state.menu.output_panel.log(&format!("🎨 Opened UI design file: {}", file_path.display()));
                        }
                        crate::editor::file_manager::FileType::Code(_) | 
                        crate::editor::file_manager::FileType::Unknown => {
                            app_state.design_mode = false;
                            app_state.menu.output_panel.log(&format!("💻 Opened code file: {}", file_path.display()));
                        }
                    }
                }
            }
            Err(e) => {
                app_state.menu.output_panel.log(&format!("❌ Failed to read file {}: {}", file_path.display(), e));
            }
        }
    }
}