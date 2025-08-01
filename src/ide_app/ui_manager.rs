//! # UI Manager
//! 
//! Manages the IDE's user interface layout, panel visibility, and UI orchestration.
//! This module handles the top-level UI structure and coordinates between different panels.

use eframe::egui;
use super::app_state::IdeAppState;

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
        if ui.selectable_label(app_state.design_mode, "🎨 Design").on_hover_text("Visual Designer Mode").clicked() {
            app_state.design_mode = true;
        }
        if ui.selectable_label(!app_state.design_mode, "💻 Code").on_hover_text("Code Editor Mode").clicked() {
            app_state.design_mode = false;
        }
    }
    
    /// Render design mode specific controls
    fn render_design_controls(app_state: &mut IdeAppState, ui: &mut egui::Ui) {
        if ui.button("📐").on_hover_text("Toggle Grid").clicked() {
            app_state.visual_designer.grid.visible = !app_state.visual_designer.grid.visible;
        }
        if ui.button("📏").on_hover_text("Toggle Rulers").clicked() {
            app_state.visual_designer.guides.rulers_visible = !app_state.visual_designer.guides.rulers_visible;
        }
        if ui.button("🔗").on_hover_text("Snap to Grid").clicked() {
            app_state.visual_designer.grid.snap_enabled = !app_state.visual_designer.grid.snap_enabled;
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
        // Determine active tab priorities
        let mut active_tab = "project";
        if app_state.show_component_palette { active_tab = "palette"; }
        if app_state.hierarchy_manager.show_hierarchy_panel { active_tab = "hierarchy"; }
        
        // Tab headers
        ui.horizontal(|ui| {
            if app_state.show_project_panel {
                if ui.selectable_label(active_tab == "project", "📁 Project").clicked() {
                    active_tab = "project";
                }
            }
            if app_state.show_component_palette {
                if ui.selectable_label(active_tab == "palette", "🧰 Components").clicked() {
                    active_tab = "palette";
                }
            }
            if app_state.hierarchy_manager.show_hierarchy_panel {
                if ui.selectable_label(active_tab == "hierarchy", "🗂 Hierarchy").clicked() {
                    active_tab = "hierarchy";
                }
            }
        });
        
        ui.separator();
        
        // Tab content
        match active_tab {
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
        app_state.project_manager.render_file_browser(ui, &mut app_state.menu.output_panel);
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
    fn render_palette_components(_app_state: &mut IdeAppState, ui: &mut egui::Ui) {
        use super::drag_drop::ComponentType;
        
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
        ];
        
        for component_type in &component_types {
            ui.horizontal(|ui| {
                ui.label(component_type.icon());
                if ui.button(component_type.display_name()).clicked() {
                    // Start drag operation for this component type
                    // TODO: Integrate with drag system
                }
            });
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
        let mut active_tab = "properties";
        if app_state.show_modern_ide_panel { active_tab = "modern"; }
        
        // Tab headers
        ui.horizontal(|ui| {
            if app_state.show_properties_inspector {
                if ui.selectable_label(active_tab == "properties", "🔧 Properties").clicked() {
                    active_tab = "properties";
                }
            }
            if app_state.show_modern_ide_panel {
                if ui.selectable_label(active_tab == "modern", "🚀 Modern IDE").clicked() {
                    active_tab = "modern";
                }
            }
        });
        
        ui.separator();
        
        // Tab content
        match active_tab {
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
    
    /// Render the properties inspector
    fn render_properties_inspector(app_state: &mut IdeAppState, ui: &mut egui::Ui) {
        ui.heading("Properties");
        ui.separator();
        
        if let Some(selected_idx) = app_state.selected_component {
            if selected_idx < app_state.components.len() {
                app_state.property_inspector.render_component_properties(ui, &mut app_state.components[selected_idx]);
            } else {
                ui.label("Invalid component selection");
            }
        } else {
            ui.label("No component selected");
            ui.label("Select a component to edit its properties");
        }
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
}