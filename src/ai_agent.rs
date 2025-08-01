//! # AI Agent System - Intelligent Development Assistant
//!
//! This module implements a sophisticated AI assistant specifically designed for software
//! development tasks within the RAD IDE. The AI agent provides context-aware assistance
//! across multiple development domains including code generation, debugging, architecture 
//! design, and UI development.
//!
//! ## Core Capabilities
//!
//! - **Code Generation**: Generates code from natural language descriptions
//! - **Bug Analysis**: Analyzes error messages and suggests fixes
//! - **Code Review**: Provides code quality feedback and improvement suggestions  
//! - **Architecture Guidance**: Suggests architectural patterns and best practices
//! - **UI Design**: Assists with component selection and user interface design
//! - **Test Generation**: Creates unit tests and integration test scenarios
//!
//! ## Context Awareness
//!
//! The AI agent maintains rich context about the current development session:
//! - Project structure and dependencies
//! - Currently open files and selected code
//! - Recent error messages and compiler output
//! - Conversation history for contextual responses
//! - Specialized knowledge for Rust and RAD development
//!
//! ## Integration with Ollama
//!
//! The system integrates with Ollama for local AI model execution, providing:
//! - Privacy-preserving local AI processing
//! - Customizable model selection (Code Llama, Llama 2, etc.)
//! - Offline capability for secure development environments
//! - Specialized prompt engineering for development tasks

use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;
use std::collections::HashMap;

/// Enhanced AI Agent with specialized capabilities for software development
#[allow(dead_code)]
pub struct AiAgent {
    ollama: Ollama,
    context: AiContext,
    conversation_history: Vec<AiConversation>,
    specialized_prompts: HashMap<AiTaskType, String>,
}

/// AI conversation entry for maintaining context
#[derive(Clone, Debug)]
pub struct AiConversation {
    pub user_input: String,
    pub ai_response: String,
    pub task_type: AiTaskType,
    pub timestamp: std::time::SystemTime,
}

/// Types of AI tasks with specialized handling
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum AiTaskType {
    CodeGeneration,
    BugFixing,
    CodeReview,
    Architecture,
    UIDesign,
    Testing,
    Documentation,
    Refactoring,
    Performance,
    Security,
}

/// Context information for better AI responses
#[derive(Clone, Debug)]
pub struct AiContext {
    pub project_name: String,
    pub current_file: Option<String>,
    pub selected_code: Option<String>,
    pub error_messages: Vec<String>,
    pub project_dependencies: Vec<String>,
    pub component_context: Vec<String>,
}

impl Default for AiContext {
    fn default() -> Self {
        Self {
            project_name: "Rust Project".to_string(),
            current_file: None,
            selected_code: None,
            error_messages: Vec::new(),
            project_dependencies: vec![
                "egui".to_string(),
                "eframe".to_string(),
                "serde".to_string(),
                "anyhow".to_string(),
            ],
            component_context: Vec::new(),
        }
    }
}

#[allow(dead_code)]
impl AiAgent {
    pub fn new() -> Self {
        let mut agent = Self {
            ollama: Ollama::default(),
            context: AiContext::default(),
            conversation_history: Vec::new(),
            specialized_prompts: HashMap::new(),
        };
        agent.initialize_specialized_prompts();
        agent
    }

    /// Initialize specialized prompts for different AI tasks
    fn initialize_specialized_prompts(&mut self) {
        self.specialized_prompts.insert(
            AiTaskType::CodeGeneration,
            "You are an expert Rust developer. Generate clean, idiomatic Rust code that follows best practices. Include appropriate error handling, documentation, and consider performance implications.".to_string()
        );

        self.specialized_prompts.insert(
            AiTaskType::BugFixing,
            "You are a Rust debugging expert. Analyze the provided code and error messages to identify the root cause. Provide a clear explanation and specific fix with reasoning.".to_string()
        );

        self.specialized_prompts.insert(
            AiTaskType::CodeReview,
            "You are a senior Rust code reviewer. Analyze the code for correctness, performance, security, and maintainability. Provide constructive feedback with specific improvements.".to_string()
        );

        self.specialized_prompts.insert(
            AiTaskType::Architecture,
            "You are a software architect specializing in Rust applications. Design scalable, maintainable architecture considering performance, modularity, and best practices.".to_string()
        );

        self.specialized_prompts.insert(
            AiTaskType::UIDesign,
            "You are a UI/UX expert working with egui and Rust. Create intuitive, responsive user interfaces that follow modern design principles and accessibility guidelines.".to_string()
        );

        self.specialized_prompts.insert(
            AiTaskType::Testing,
            "You are a Rust testing expert. Create comprehensive test cases including unit tests, integration tests, and property-based tests using appropriate testing frameworks.".to_string()
        );
    }

    /// Enhanced ask method with context and task-specific handling
    pub async fn ask_with_context(&mut self, prompt: &str, task_type: AiTaskType) -> anyhow::Result<String> {
        let enhanced_prompt = self.build_contextual_prompt(prompt, &task_type);
        let response = self.ask(&enhanced_prompt).await?;
        
        // Store conversation for context
        self.conversation_history.push(AiConversation {
            user_input: prompt.to_string(),
            ai_response: response.clone(),
            task_type,
            timestamp: std::time::SystemTime::now(),
        });

        // Limit conversation history to prevent context explosion
        if self.conversation_history.len() > 10 {
            self.conversation_history.remove(0);
        }

        Ok(response)
    }

    /// Build a contextual prompt with project information
    /// 
    /// This algorithm constructs a sophisticated, context-aware prompt that maximizes the AI's
    /// effectiveness by providing comprehensive project state, error context, and conversation
    /// history. The prompt engineering follows best practices for getting high-quality responses
    /// from language models while maintaining conversation continuity and relevance.
    fn build_contextual_prompt(&self, user_prompt: &str, task_type: &AiTaskType) -> String {
        let mut prompt = String::new();

        // System Role and Task-Specific Instructions
        // Each task type gets a specialized system prompt to prime the AI with appropriate
        // knowledge, perspective, and response style for that specific type of request
        if let Some(system_prompt) = self.specialized_prompts.get(task_type) {
            prompt.push_str(&format!("System: {}\n\n", system_prompt));
        }

        // Project Metadata Section
        // Provides essential project context that helps the AI understand the environment
        // and constraints it's working within for more targeted suggestions
        prompt.push_str(&format!("Project: {}\n", self.context.project_name));
        prompt.push_str(&format!("Dependencies: {}\n", self.context.project_dependencies.join(", ")));

        // Current File Context (if available)
        // Gives the AI awareness of the specific file being worked on
        // This enables more targeted suggestions and code generation
        if let Some(current_file) = &self.context.current_file {
            prompt.push_str(&format!("Current file: {}\n", current_file));
        }

        // Selected Code Context (if available)
        // When user has selected specific code, include it for precise context
        // This allows the AI to provide suggestions specific to the selected region
        if let Some(selected_code) = &self.context.selected_code {
            prompt.push_str(&format!("Selected code:\n```rust\n{}\n```\n", selected_code));
        }

        // Error Context Integration
        // Recent errors provide crucial debugging context and help the AI understand
        // what problems the user is encountering for more relevant assistance
        if !self.context.error_messages.is_empty() {
            prompt.push_str(&format!("Recent errors:\n{}\n", self.context.error_messages.join("\n")));
        }

        // Component Context Section
        // Provides awareness of available UI components and their relationships
        // This helps the AI make more informed suggestions about component usage and interactions
        if !self.context.component_context.is_empty() {
            prompt.push_str(&format!("Available components: {}\n", self.context.component_context.join(", ")));
        }

        // Conversation History for Continuity
        // Recent conversation context helps maintain coherence across multiple interactions
        // Limited to 3 recent exchanges to balance context richness with prompt length
        if !self.conversation_history.is_empty() {
            prompt.push_str("\nRecent conversation:\n");
            // Process conversations in chronological order (oldest to newest of the selected 3)
            // First reverse to get most recent 3, then reverse again for chronological order
            for conv in self.conversation_history.iter().rev().take(3).rev() {
                // Truncate long inputs/responses to prevent prompt bloat while preserving key context
                // 100 chars for user input, 200 chars for AI response provides good summary
                prompt.push_str(&format!("User: {}\nAssistant: {}\n\n", 
                    conv.user_input.chars().take(100).collect::<String>(),    // Truncated user input
                    conv.ai_response.chars().take(200).collect::<String>()    // Truncated AI response
                ));
            }
        }

        // Current User Request
        // Clearly delineated user prompt to distinguish it from contextual information
        prompt.push_str(&format!("\nUser request: {}\n", user_prompt));
        // Final instruction to reinforce expectation for quality, actionable responses
        prompt.push_str("\nPlease provide a detailed, practical response:");

        prompt
    }

    /// Original ask method for backward compatibility
    pub async fn ask(&self, prompt: &str) -> anyhow::Result<String> {
        let req = GenerationRequest::new("llama2".to_string(), prompt);
        let resp = self.ollama.generate(req).await?;
        Ok(resp.response)
    }

    /// Generate Rust code with specific requirements
    pub async fn generate_rust_code(&mut self, description: &str, requirements: Vec<String>) -> anyhow::Result<String> {
        let mut prompt = format!("Generate Rust code for: {}\n\nRequirements:\n", description);
        for req in requirements {
            prompt.push_str(&format!("- {}\n", req));
        }
        prompt.push_str("\nPlease provide complete, compilable Rust code with comments.");

        self.ask_with_context(&prompt, AiTaskType::CodeGeneration).await
    }

    /// Fix bugs with error context
    pub async fn fix_bug(&mut self, code: &str, error: &str) -> anyhow::Result<String> {
        let prompt = format!(
            "Fix this Rust code error:\n\nCode:\n```rust\n{}\n```\n\nError:\n{}\n\nProvide the corrected code with explanation.",
            code, error
        );

        self.ask_with_context(&prompt, AiTaskType::BugFixing).await
    }

    /// Review code for improvements
    pub async fn review_code(&mut self, code: &str) -> anyhow::Result<String> {
        let prompt = format!(
            "Review this Rust code for improvements:\n\n```rust\n{}\n```\n\nFocus on correctness, performance, readability, and best practices.",
            code
        );

        self.ask_with_context(&prompt, AiTaskType::CodeReview).await
    }

    /// Suggest UI components and layout
    pub async fn suggest_ui_design(&mut self, description: &str) -> anyhow::Result<String> {
        let prompt = format!(
            "Design a UI layout for: {}\n\nUsing egui components, suggest the best layout, components, and user experience patterns.",
            description
        );

        self.ask_with_context(&prompt, AiTaskType::UIDesign).await
    }

    /// Generate comprehensive tests
    pub async fn generate_tests(&mut self, code: &str) -> anyhow::Result<String> {
        let prompt = format!(
            "Generate comprehensive tests for this Rust code:\n\n```rust\n{}\n```\n\nInclude unit tests, edge cases, and integration tests where appropriate.",
            code
        );

        self.ask_with_context(&prompt, AiTaskType::Testing).await
    }

    /// Update AI context with project information
    pub fn update_context(&mut self, context: AiContext) {
        self.context = context;
    }

    /// Add error message to context
    /// 
    /// This algorithm maintains a rolling window of recent error messages that provide
    /// crucial debugging context to the AI. The bounded history prevents context bloat
    /// while ensuring the AI has access to recent error patterns that might inform
    /// better suggestions and solutions.
    pub fn add_error_context(&mut self, error: &str) {
        // Add the new error message to the context history
        // This gives the AI visibility into current problems and failure patterns
        self.context.error_messages.push(error.to_string());
        
        // Maintain a bounded history to prevent unbounded growth
        // Keep only the 5 most recent errors to balance context richness with memory usage
        // Older errors are likely less relevant to current debugging efforts
        if self.context.error_messages.len() > 5 {
            // Remove the oldest error message (FIFO queue behavior)
            // This ensures we always have the most recent and relevant error context
            self.context.error_messages.remove(0);
        }
    }

    /// Set current file context
    pub fn set_current_file(&mut self, file_path: &str) {
        self.context.current_file = Some(file_path.to_string());
    }

    /// Set selected code context
    pub fn set_selected_code(&mut self, code: &str) {
        self.context.selected_code = Some(code.to_string());
    }

    /// Add component to context
    pub fn add_component_context(&mut self, component: &str) {
        if !self.context.component_context.contains(&component.to_string()) {
            self.context.component_context.push(component.to_string());
        }
    }

    /// Get conversation history
    pub fn get_conversation_history(&self) -> &[AiConversation] {
        &self.conversation_history
    }

    /// Clear conversation history
    pub fn clear_history(&mut self) {
        self.conversation_history.clear();
    }

    /// Get AI statistics
    pub fn get_stats(&self) -> AiStats {
        let mut task_counts = HashMap::new();
        for conv in &self.conversation_history {
            *task_counts.entry(conv.task_type.clone()).or_insert(0) += 1;
        }

        AiStats {
            total_conversations: self.conversation_history.len(),
            task_distribution: task_counts,
            context_size: self.context.error_messages.len() + 
                         self.context.component_context.len() +
                         if self.context.current_file.is_some() { 1 } else { 0 },
        }
    }
}

/// AI usage statistics
#[derive(Debug)]
pub struct AiStats {
    pub total_conversations: usize,
    pub task_distribution: HashMap<AiTaskType, usize>,
    pub context_size: usize,
}
