//! Toolbar for IDE actions

#[allow(dead_code)]
pub struct IdeToolbar {
    pub build_debug: bool,
    pub build_release: bool,
    pub run_debug: bool,
    pub run_release: bool,
    pub ai_chat: bool,
    pub ai_fix: bool,
    pub package_components: bool,
    pub export_project: bool,
    pub open_settings: bool,
    pub format_code: bool,
}

#[allow(dead_code)]
impl IdeToolbar {
    pub fn new() -> Self {
        Self {
            build_debug: false,
            build_release: false,
            run_debug: false,
            run_release: false,
            ai_chat: false,
            ai_fix: false,
            package_components: false,
            export_project: false,
            open_settings: false,
            format_code: false,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, actions: &mut crate::editor::actions::Actions, output_panel: &mut crate::editor::output_panel::OutputPanel) {
        ui.horizontal(|ui| {
            if ui.button("🔨").on_hover_text("Build Debug (Ctrl+B)").clicked() {
                actions.build_debug(output_panel);
            }
            if ui.button("🚀").on_hover_text("Build Release (Ctrl+Shift+B)").clicked() {
                actions.build_release(output_panel);
            }
            ui.separator();
            if ui.button("▶️").on_hover_text("Run Debug (F5)").clicked() {
                actions.run_debug(output_panel);
            }
            if ui.button("🏁").on_hover_text("Run Release (Ctrl+F5)").clicked() {
                actions.run_release(output_panel);
            }
            ui.separator();
            if ui.button("🤖").on_hover_text("AI Chat (Ctrl+Alt+A)").clicked() {
                actions.ai_chat(ui);
            }
            if ui.button("🛠️").on_hover_text("Fix with AI (Alt+F)").clicked() {
                actions.ai_fix(ui);
            }
            ui.separator();
            if ui.button("📦").on_hover_text("Package Components (Ctrl+P)").clicked() {
                actions.package_components(output_panel);
            }
            if ui.button("📤").on_hover_text("Export Project (Ctrl+E)").clicked() {
                actions.export_project(output_panel);
            }
            if ui.button("🧹").on_hover_text("Format Code (Ctrl+Shift+F)").clicked() {
                actions.format_code(output_panel);
            }
            ui.separator();
            if ui.button("⚙️").on_hover_text("Settings (Ctrl+,)").clicked() {
                actions.open_settings(ui);
            }
        });
    }
}
