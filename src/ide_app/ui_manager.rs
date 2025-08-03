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
        app_state.project_manager.render_project_ui(ui, &mut app_state.menu.output_panel);
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
    
    /// Render form properties when form is selected
    fn render_form_properties(app_state: &mut IdeAppState, ui: &mut egui::Ui) {
        ui.label("Form Properties");
        ui.separator();
        
        // Form basic properties
        ui.horizontal(|ui| {
            ui.label("Title:");
            ui.text_edit_singleline(&mut app_state.root_form.title);
        });
        
        ui.horizontal(|ui| {
            ui.label("Width:");
            ui.add(egui::DragValue::new(&mut app_state.root_form.size.x).speed(1.0).clamp_range(100.0..=2000.0));
        });
        
        ui.horizontal(|ui| {
            ui.label("Height:");
            ui.add(egui::DragValue::new(&mut app_state.root_form.size.y).speed(1.0).clamp_range(100.0..=1500.0));
        });
        
        ui.horizontal(|ui| {
            ui.label("Background Color:");
            ui.color_edit_button_srgba(&mut app_state.root_form.background_color);
        });
        
        ui.checkbox(&mut app_state.root_form.visible, "Visible");
        ui.checkbox(&mut app_state.root_form.show_border, "Show Border");
        
        ui.separator();
        ui.label("Form Layout");
        ui.horizontal(|ui| {
            ui.label("Padding:");
            ui.add(egui::DragValue::new(&mut app_state.root_form.padding).speed(0.1).clamp_range(0.0..=50.0));
        });
        
        ui.horizontal(|ui| {
            ui.label("Corner Radius:");
            ui.add(egui::DragValue::new(&mut app_state.root_form.corner_radius).speed(0.1).clamp_range(0.0..=25.0));
        });
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
}