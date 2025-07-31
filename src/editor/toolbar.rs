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

    pub fn ui(&mut self, _ui: &mut egui::Ui) {}
}
