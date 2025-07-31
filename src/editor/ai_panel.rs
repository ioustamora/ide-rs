//! AI chat/help panel for IDE
use egui::*;

pub struct AiPanel {
    pub chat_history: Vec<String>,
    pub input: String,
    pub suggestions: Vec<String>,
    pub refactor_code: String,
}

impl AiPanel {
    pub fn new() -> Self {
        Self {
            chat_history: vec![],
            input: String::new(),
            suggestions: vec![],
            refactor_code: String::new(),
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label("AI Chat & Help Panel");
        for msg in &self.chat_history {
            ui.label(msg);
        }
        ui.text_edit_singleline(&mut self.input);
        if ui.button("Send").clicked() {
            // Placeholder: send to AI agent
            self.chat_history.push(format!("You: {}", self.input));
            // Simulate AI suggestions and refactoring
            self.suggestions = vec![
                "Consider using a match statement.".to_string(),
                "Refactor to use async/await.".to_string()
            ];
            self.refactor_code = "// Refactored code example\nfn main() {}".to_string();
            self.input.clear();
        }
        if !self.suggestions.is_empty() {
            ui.separator();
            ui.label("AI Suggestions:");
            for suggestion in &self.suggestions {
                ui.label(suggestion);
            }
        }
        if !self.refactor_code.is_empty() {
            ui.separator();
            ui.label("AI Refactored Code:");
            ui.code(&self.refactor_code);
        }
    }
}
