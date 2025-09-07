//! Intelligent Code Completion and Suggestions System
//!
//! This module provides advanced code completion capabilities including:
//! - Context-aware autocompletion
//! - Smart import suggestions
//! - Code snippet expansion
//! - AI-powered suggestions
//! - Semantic analysis-based completion

use std::collections::{HashMap, VecDeque};
use serde::{Deserialize, Serialize};
use egui::{Ui, Vec2, Color32, Rect, Pos2, Stroke, RichText};

/// Main intelligent completion engine
#[derive(Debug, Clone)]
pub struct IntelligentCompletionEngine {
    /// Available completion providers
    providers: Vec<CompletionProvider>,
    /// Completion cache for performance
    completion_cache: CompletionCache,
    /// User preferences and settings
    settings: CompletionSettings,
    /// Learning system for improving suggestions
    learning_system: CompletionLearningSystem,
    /// Active completion session
    active_session: Option<CompletionSession>,
    /// Performance metrics
    metrics: CompletionMetrics,
}

/// Individual completion provider
#[derive(Debug, Clone)]
pub struct CompletionProvider {
    /// Provider identifier
    pub id: String,
    /// Provider name
    pub name: String,
    /// Supported languages
    pub supported_languages: Vec<String>,
    /// Provider priority (higher = more important)
    pub priority: i32,
    /// Whether provider is enabled
    pub enabled: bool,
    /// Provider-specific configuration
    pub config: HashMap<String, String>,
}

/// Completion cache for performance optimization
#[derive(Debug, Clone)]
pub struct CompletionCache {
    /// Cached completions by context
    cache: HashMap<String, CachedCompletion>,
    /// Maximum cache size
    max_size: usize,
    /// Cache hit statistics
    hit_count: usize,
    /// Cache miss statistics
    miss_count: usize,
}

/// Cached completion entry
#[derive(Debug, Clone)]
struct CachedCompletion {
    /// Cached completion items
    items: Vec<CompletionItem>,
    /// Cache timestamp
    timestamp: std::time::Instant,
    /// Cache expiry duration
    ttl: std::time::Duration,
}

/// Completion settings and preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionSettings {
    /// Whether completion is enabled
    pub enabled: bool,
    /// Auto-trigger completion after typing delay (ms)
    pub auto_trigger_delay: u64,
    /// Minimum characters to trigger completion
    pub min_trigger_chars: usize,
    /// Maximum completion items to show
    pub max_items: usize,
    /// Whether to show documentation in popup
    pub show_documentation: bool,
    /// Whether to include snippets
    pub include_snippets: bool,
    /// Whether to include AI suggestions
    pub include_ai_suggestions: bool,
    /// Case sensitivity for filtering
    pub case_sensitive: bool,
    /// Fuzzy matching enabled
    pub fuzzy_matching: bool,
    /// Sort completion items by relevance
    pub sort_by_relevance: bool,
}

/// Learning system for improving completion quality
#[derive(Debug, Clone)]
pub struct CompletionLearningSystem {
    /// User acceptance history
    acceptance_history: VecDeque<CompletionAcceptance>,
    /// Frequently used completions
    frequent_completions: HashMap<String, f32>,
    /// Context patterns
    context_patterns: HashMap<String, Vec<String>>,
    /// Learning enabled
    enabled: bool,
}

/// Completion acceptance record for learning
#[derive(Debug, Clone)]
struct CompletionAcceptance {
    /// Accepted completion item
    item: CompletionItem,
    /// Context where it was accepted
    context: String,
    /// Timestamp of acceptance
    timestamp: std::time::Instant,
    /// Time taken to accept (user hesitation metric)
    acceptance_time: std::time::Duration,
}

/// Active completion session
#[derive(Debug, Clone)]
pub struct CompletionSession {
    /// Session ID
    pub id: String,
    /// Trigger position in text
    pub trigger_position: usize,
    /// Current completion items
    pub items: Vec<CompletionItem>,
    /// Currently selected item index
    pub selected_index: usize,
    /// Filter text (what user has typed)
    pub filter_text: String,
    /// Session start time
    pub start_time: std::time::Instant,
    /// Context information
    pub context: CompletionContext,
}

/// Individual completion item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItem {
    /// Text to insert
    pub insert_text: String,
    /// Display label (may differ from insert text)
    pub label: String,
    /// Detailed description
    pub detail: Option<String>,
    /// Documentation
    pub documentation: Option<String>,
    /// Completion kind
    pub kind: CompletionKind,
    /// Sort priority (lower = higher priority)
    pub sort_priority: f32,
    /// Relevance score (0.0 to 1.0)
    pub relevance: f32,
    /// Additional text edits (for imports, etc.)
    pub additional_edits: Vec<TextEdit>,
    /// Snippet placeholders
    pub snippet_placeholders: Vec<SnippetPlaceholder>,
    /// Provider that generated this item
    pub provider_id: String,
}

/// Types of completion items
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompletionKind {
    Text,
    Method,
    Function,
    Constructor,
    Field,
    Variable,
    Class,
    Interface,
    Module,
    Property,
    Unit,
    Value,
    Enum,
    Keyword,
    Snippet,
    Color,
    File,
    Reference,
    Folder,
    EnumMember,
    Constant,
    Struct,
    Event,
    Operator,
    TypeParameter,
    AI_Suggestion,
}

/// Context information for completion
#[derive(Debug, Clone)]
pub struct CompletionContext {
    /// Current file path
    pub file_path: String,
    /// Programming language
    pub language: String,
    /// Current line content
    pub current_line: String,
    /// Cursor position in line
    pub cursor_position: usize,
    /// Previous lines for context
    pub previous_lines: Vec<String>,
    /// Following lines for context
    pub following_lines: Vec<String>,
    /// Current scope information
    pub scope_info: ScopeInfo,
    /// Available imports
    pub available_imports: Vec<String>,
    /// Project dependencies
    pub project_dependencies: Vec<String>,
}

/// Scope information for context-aware completion
#[derive(Debug, Clone)]
pub struct ScopeInfo {
    /// Current function/method name
    pub current_function: Option<String>,
    /// Current struct/class name
    pub current_struct: Option<String>,
    /// Current module path
    pub current_module: Vec<String>,
    /// Local variables in scope
    pub local_variables: Vec<VariableInfo>,
    /// Available functions in scope
    pub available_functions: Vec<FunctionInfo>,
    /// Available types in scope
    pub available_types: Vec<TypeInfo>,
}

/// Variable information
#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub name: String,
    pub var_type: String,
    pub is_mutable: bool,
    pub scope_level: usize,
}

/// Function information
#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub parameters: Vec<ParameterInfo>,
    pub return_type: Option<String>,
    pub is_async: bool,
    pub visibility: String,
}

/// Parameter information
#[derive(Debug, Clone)]
pub struct ParameterInfo {
    pub name: String,
    pub param_type: String,
    pub is_optional: bool,
    pub default_value: Option<String>,
}

/// Type information
#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub name: String,
    pub kind: TypeKind,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<FunctionInfo>,
}

/// Type kinds
#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    Struct,
    Enum,
    Trait,
    Type,
    Union,
}

/// Field information
#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub field_type: String,
    pub visibility: String,
    pub is_mutable: bool,
}

/// Text edit for additional modifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEdit {
    /// Range to replace (start, end)
    pub range: (usize, usize),
    /// New text to insert
    pub new_text: String,
}

/// Snippet placeholder for interactive editing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnippetPlaceholder {
    /// Placeholder index (for tab ordering)
    pub index: usize,
    /// Default text
    pub default_text: String,
    /// Placeholder description
    pub description: Option<String>,
    /// Valid choices (for choice placeholders)
    pub choices: Vec<String>,
}

/// Performance metrics for completion system
#[derive(Debug, Clone)]
pub struct CompletionMetrics {
    /// Total completion requests
    pub total_requests: usize,
    /// Average response time (ms)
    pub avg_response_time: f32,
    /// Cache hit rate
    pub cache_hit_rate: f32,
    /// User acceptance rate
    pub acceptance_rate: f32,
    /// Most used completion types
    pub popular_kinds: HashMap<CompletionKind, usize>,
}

impl Default for IntelligentCompletionEngine {
    fn default() -> Self {
        Self {
            providers: Vec::new(),
            completion_cache: CompletionCache::new(1000),
            settings: CompletionSettings::default(),
            learning_system: CompletionLearningSystem::new(),
            active_session: None,
            metrics: CompletionMetrics::default(),
        }
    }
}

impl Default for CompletionSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_trigger_delay: 300,
            min_trigger_chars: 1,
            max_items: 50,
            show_documentation: true,
            include_snippets: true,
            include_ai_suggestions: true,
            case_sensitive: false,
            fuzzy_matching: true,
            sort_by_relevance: true,
        }
    }
}

impl IntelligentCompletionEngine {
    /// Create a new completion engine
    pub fn new() -> Self {
        let mut engine = Self::default();
        engine.initialize_default_providers();
        engine
    }

    /// Initialize default completion providers
    fn initialize_default_providers(&mut self) {
        // Rust language provider
        self.providers.push(CompletionProvider {
            id: "rust".to_string(),
            name: "Rust Language Server".to_string(),
            supported_languages: vec!["rust".to_string()],
            priority: 100,
            enabled: true,
            config: HashMap::new(),
        });

        // Snippet provider
        self.providers.push(CompletionProvider {
            id: "snippets".to_string(),
            name: "Code Snippets".to_string(),
            supported_languages: vec!["*".to_string()],
            priority: 80,
            enabled: true,
            config: HashMap::new(),
        });

        // Keyword provider
        self.providers.push(CompletionProvider {
            id: "keywords".to_string(),
            name: "Language Keywords".to_string(),
            supported_languages: vec!["rust".to_string(), "javascript".to_string(), "python".to_string()],
            priority: 60,
            enabled: true,
            config: HashMap::new(),
        });

        // AI provider
        self.providers.push(CompletionProvider {
            id: "ai".to_string(),
            name: "AI-Powered Suggestions".to_string(),
            supported_languages: vec!["*".to_string()],
            priority: 40,
            enabled: true,
            config: HashMap::new(),
        });
    }

    /// Trigger completion at current position
    pub async fn trigger_completion(&mut self, context: CompletionContext) -> Result<CompletionSession, String> {
        if !self.settings.enabled {
            return Err("Completion is disabled".to_string());
        }

        let start_time = std::time::Instant::now();
        self.metrics.total_requests += 1;

        // Check cache first
        let cache_key = self.generate_cache_key(&context);
        if let Some(cached) = self.completion_cache.get(&cache_key) {
            let session = CompletionSession {
                id: uuid::Uuid::new_v4().to_string(),
                trigger_position: context.cursor_position,
                items: cached.clone(),
                selected_index: 0,
                filter_text: String::new(),
                start_time,
                context,
            };
            self.active_session = Some(session.clone());
            return Ok(session);
        }

        // Generate completions from providers
        let mut all_items = Vec::new();
        
        for provider in &self.providers {
            if !provider.enabled || !self.provider_supports_language(provider, &context.language) {
                continue;
            }

            let items = self.get_completions_from_provider(provider, &context).await?;
            all_items.extend(items);
        }

        // Filter and sort items
        self.filter_and_sort_items(&mut all_items, &context);
        
        // Limit items
        all_items.truncate(self.settings.max_items);

        // Cache results
        self.completion_cache.insert(cache_key, all_items.clone(), std::time::Duration::from_secs(60));

        // Update metrics
        let response_time = start_time.elapsed().as_millis() as f32;
        self.update_response_time(response_time);

        let session = CompletionSession {
            id: uuid::Uuid::new_v4().to_string(),
            trigger_position: context.cursor_position,
            items: all_items,
            selected_index: 0,
            filter_text: String::new(),
            start_time,
            context,
        };

        self.active_session = Some(session.clone());
        Ok(session)
    }

    /// Get completions from a specific provider
    async fn get_completions_from_provider(&self, provider: &CompletionProvider, context: &CompletionContext) -> Result<Vec<CompletionItem>, String> {
        match provider.id.as_str() {
            "rust" => self.get_rust_completions(context).await,
            "snippets" => self.get_snippet_completions(context),
            "keywords" => self.get_keyword_completions(context),
            "ai" => self.get_ai_completions(context).await,
            _ => Ok(Vec::new()),
        }
    }

    /// Get Rust-specific completions
    async fn get_rust_completions(&self, context: &CompletionContext) -> Result<Vec<CompletionItem>, String> {
        let mut items = Vec::new();

        // Local variables
        for var in &context.scope_info.local_variables {
            items.push(CompletionItem {
                insert_text: var.name.clone(),
                label: var.name.clone(),
                detail: Some(format!("{}: {}", if var.is_mutable { "mut" } else { "let" }, var.var_type)),
                documentation: None,
                kind: CompletionKind::Variable,
                sort_priority: 10.0,
                relevance: 0.9,
                additional_edits: Vec::new(),
                snippet_placeholders: Vec::new(),
                provider_id: "rust".to_string(),
            });
        }

        // Functions in scope
        for func in &context.scope_info.available_functions {
            let params_str = func.parameters.iter()
                .map(|p| format!("{}: {}", p.name, p.param_type))
                .collect::<Vec<_>>()
                .join(", ");
            
            let insert_text = if func.parameters.is_empty() {
                format!("{}()", func.name)
            } else {
                format!("{}($1)", func.name)
            };

            items.push(CompletionItem {
                insert_text,
                label: func.name.clone(),
                detail: Some(format!("fn({}){}", params_str, 
                    func.return_type.as_ref().map(|t| format!(" -> {}", t)).unwrap_or_default())),
                documentation: Some(format!("Function: {}", func.name)),
                kind: CompletionKind::Function,
                sort_priority: 20.0,
                relevance: 0.8,
                additional_edits: Vec::new(),
                snippet_placeholders: if !func.parameters.is_empty() {
                    vec![SnippetPlaceholder {
                        index: 1,
                        default_text: params_str,
                        description: Some("Parameters".to_string()),
                        choices: Vec::new(),
                    }]
                } else {
                    Vec::new()
                },
                provider_id: "rust".to_string(),
            });
        }

        // Types in scope
        for type_info in &context.scope_info.available_types {
            items.push(CompletionItem {
                insert_text: type_info.name.clone(),
                label: type_info.name.clone(),
                detail: Some(format!("{:?}", type_info.kind)),
                documentation: Some(format!("Type: {}", type_info.name)),
                kind: match type_info.kind {
                    TypeKind::Struct => CompletionKind::Struct,
                    TypeKind::Enum => CompletionKind::Enum,
                    TypeKind::Trait => CompletionKind::Interface,
                    _ => CompletionKind::Class,
                },
                sort_priority: 30.0,
                relevance: 0.7,
                additional_edits: Vec::new(),
                snippet_placeholders: Vec::new(),
                provider_id: "rust".to_string(),
            });
        }

        Ok(items)
    }

    /// Get snippet completions
    fn get_snippet_completions(&self, context: &CompletionContext) -> Result<Vec<CompletionItem>, String> {
        let mut items = Vec::new();

        // Rust-specific snippets
        if context.language == "rust" {
            items.extend(vec![
                CompletionItem {
                    insert_text: "fn ${1:function_name}(${2:parameters}) ${3:-> ReturnType }{\n    ${4:// TODO: implement}\n}".to_string(),
                    label: "fn".to_string(),
                    detail: Some("Function definition".to_string()),
                    documentation: Some("Create a new function with parameters and return type".to_string()),
                    kind: CompletionKind::Snippet,
                    sort_priority: 5.0,
                    relevance: 0.9,
                    additional_edits: Vec::new(),
                    snippet_placeholders: vec![
                        SnippetPlaceholder {
                            index: 1,
                            default_text: "function_name".to_string(),
                            description: Some("Function name".to_string()),
                            choices: Vec::new(),
                        },
                        SnippetPlaceholder {
                            index: 2,
                            default_text: "".to_string(),
                            description: Some("Parameters".to_string()),
                            choices: Vec::new(),
                        },
                    ],
                    provider_id: "snippets".to_string(),
                },
                CompletionItem {
                    insert_text: "struct ${1:StructName} {\n    ${2:field}: ${3:Type},\n}".to_string(),
                    label: "struct".to_string(),
                    detail: Some("Struct definition".to_string()),
                    documentation: Some("Create a new struct with fields".to_string()),
                    kind: CompletionKind::Snippet,
                    sort_priority: 5.0,
                    relevance: 0.8,
                    additional_edits: Vec::new(),
                    snippet_placeholders: Vec::new(),
                    provider_id: "snippets".to_string(),
                },
                CompletionItem {
                    insert_text: "impl ${1:StructName} {\n    ${2:// methods}\n}".to_string(),
                    label: "impl".to_string(),
                    detail: Some("Implementation block".to_string()),
                    documentation: Some("Create an implementation block for a type".to_string()),
                    kind: CompletionKind::Snippet,
                    sort_priority: 5.0,
                    relevance: 0.8,
                    additional_edits: Vec::new(),
                    snippet_placeholders: Vec::new(),
                    provider_id: "snippets".to_string(),
                },
            ]);
        }

        Ok(items)
    }

    /// Get keyword completions
    fn get_keyword_completions(&self, context: &CompletionContext) -> Result<Vec<CompletionItem>, String> {
        let mut items = Vec::new();

        if context.language == "rust" {
            let rust_keywords = vec![
                "let", "mut", "const", "static", "fn", "struct", "enum", "trait", "impl",
                "use", "mod", "pub", "priv", "extern", "crate", "super", "self", "Self",
                "if", "else", "match", "loop", "while", "for", "in", "break", "continue",
                "return", "yield", "async", "await", "move", "ref", "dyn", "unsafe",
                "where", "type", "as", "true", "false", "Some", "None", "Ok", "Err",
            ];

            for keyword in rust_keywords {
                items.push(CompletionItem {
                    insert_text: keyword.to_string(),
                    label: keyword.to_string(),
                    detail: Some("Rust keyword".to_string()),
                    documentation: Some(format!("Rust language keyword: {}", keyword)),
                    kind: CompletionKind::Keyword,
                    sort_priority: 40.0,
                    relevance: 0.6,
                    additional_edits: Vec::new(),
                    snippet_placeholders: Vec::new(),
                    provider_id: "keywords".to_string(),
                });
            }
        }

        Ok(items)
    }

    /// Get AI-powered completions
    async fn get_ai_completions(&self, _context: &CompletionContext) -> Result<Vec<CompletionItem>, String> {
        // This would integrate with an AI service like Copilot
        // For now, return placeholder AI suggestions
        Ok(vec![
            CompletionItem {
                insert_text: "// AI suggests: Consider using a match statement here".to_string(),
                label: "AI: Match statement".to_string(),
                detail: Some("AI-powered suggestion".to_string()),
                documentation: Some("The AI suggests using a match statement for better pattern matching".to_string()),
                kind: CompletionKind::AI_Suggestion,
                sort_priority: 50.0,
                relevance: 0.5,
                additional_edits: Vec::new(),
                snippet_placeholders: Vec::new(),
                provider_id: "ai".to_string(),
            },
        ])
    }

    /// Filter and sort completion items
    fn filter_and_sort_items(&self, items: &mut Vec<CompletionItem>, context: &CompletionContext) {
        // Apply learning system adjustments
        if self.learning_system.enabled {
            for item in items.iter_mut() {
                if let Some(frequency) = self.learning_system.frequent_completions.get(&item.label) {
                    item.relevance += frequency * 0.2; // Boost frequently used items
                }
            }
        }

        // Sort by relevance and priority
        if self.settings.sort_by_relevance {
            items.sort_by(|a, b| {
                let a_score = a.relevance - (a.sort_priority / 100.0);
                let b_score = b.relevance - (b.sort_priority / 100.0);
                b_score.partial_cmp(&a_score).unwrap_or(std::cmp::Ordering::Equal)
            });
        }
    }

    /// Filter items based on current input
    pub fn filter_items(&mut self, filter_text: &str) {
        if let Some(session) = &mut self.active_session {
            session.filter_text = filter_text.to_string();
            
            if filter_text.is_empty() {
                return;
            }

            // Apply fuzzy matching if enabled
            if self.settings.fuzzy_matching {
                session.items.retain(|item| self.fuzzy_match(&item.label, filter_text));
            } else {
                let filter_lower = if self.settings.case_sensitive {
                    filter_text.to_string()
                } else {
                    filter_text.to_lowercase()
                };

                session.items.retain(|item| {
                    let label = if self.settings.case_sensitive {
                        item.label.clone()
                    } else {
                        item.label.to_lowercase()
                    };
                    label.contains(&filter_lower)
                });
            }

            // Reset selection
            session.selected_index = 0;
        }
    }

    /// Simple fuzzy matching algorithm
    fn fuzzy_match(&self, text: &str, pattern: &str) -> bool {
        let text_chars: Vec<char> = text.to_lowercase().chars().collect();
        let pattern_chars: Vec<char> = pattern.to_lowercase().chars().collect();
        
        let mut text_idx = 0;
        let mut pattern_idx = 0;
        
        while text_idx < text_chars.len() && pattern_idx < pattern_chars.len() {
            if text_chars[text_idx] == pattern_chars[pattern_idx] {
                pattern_idx += 1;
            }
            text_idx += 1;
        }
        
        pattern_idx == pattern_chars.len()
    }

    /// Render completion popup
    pub fn render_completion_popup(&mut self, ui: &mut Ui, cursor_pos: Pos2) -> Option<CompletionItem> {
        let session = self.active_session.as_ref()?;
        let mut selected_item = None;

        if session.items.is_empty() {
            return None;
        }

        let popup_size = Vec2::new(400.0, 300.0);
        let popup_pos = Pos2::new(
            cursor_pos.x,
            cursor_pos.y + 20.0
        );

        egui::Window::new("Code Completion")
            .fixed_pos(popup_pos)
            .fixed_size(popup_size)
            .resizable(false)
            .title_bar(false)
            .frame(egui::Frame::popup(ui.style()))
            .show(ui.ctx(), |ui| {
                egui::ScrollArea::vertical()
                    .max_height(250.0)
                    .show(ui, |ui| {
                        for (index, item) in session.items.iter().enumerate() {
                            let is_selected = index == session.selected_index;
                            
                            let response = ui.selectable_label(
                                is_selected,
                                self.format_completion_item(item)
                            );

                            if response.clicked() {
                                selected_item = Some(item.clone());
                            }

                            // Show documentation on hover
                            if response.hovered() && self.settings.show_documentation {
                                if let Some(doc) = &item.documentation {
                                    response.on_hover_text(doc);
                                }
                            }
                        }
                    });

                // Handle keyboard navigation
                ui.input(|i| {
                    if let Some(session) = &mut self.active_session {
                        if i.key_pressed(egui::Key::ArrowDown) {
                            session.selected_index = (session.selected_index + 1).min(session.items.len().saturating_sub(1));
                        } else if i.key_pressed(egui::Key::ArrowUp) {
                            session.selected_index = session.selected_index.saturating_sub(1);
                        } else if i.key_pressed(egui::Key::Enter) {
                            if let Some(item) = session.items.get(session.selected_index) {
                                selected_item = Some(item.clone());
                            }
                        } else if i.key_pressed(egui::Key::Escape) {
                            self.active_session = None;
                        }
                    }
                });
            });

        selected_item
    }

    /// Format completion item for display
    fn format_completion_item(&self, item: &CompletionItem) -> RichText {
        let mut text = RichText::new(&item.label);

        // Color code by completion kind
        text = match item.kind {
            CompletionKind::Function | CompletionKind::Method => text.color(Color32::from_rgb(220, 220, 170)),
            CompletionKind::Variable | CompletionKind::Field => text.color(Color32::from_rgb(156, 220, 254)),
            CompletionKind::Class | CompletionKind::Struct => text.color(Color32::from_rgb(78, 201, 176)),
            CompletionKind::Keyword => text.color(Color32::from_rgb(86, 156, 214)),
            CompletionKind::Snippet => text.color(Color32::from_rgb(206, 145, 120)),
            CompletionKind::AI_Suggestion => text.color(Color32::from_rgb(255, 215, 0)),
            _ => text,
        };

        // Add kind indicator
        let icon = self.get_completion_icon(&item.kind);
        RichText::new(format!("{} {}", icon, item.label))
            .color(text.color().unwrap_or(Color32::WHITE))
    }

    /// Get icon for completion kind
    fn get_completion_icon(&self, kind: &CompletionKind) -> &'static str {
        match kind {
            CompletionKind::Function | CompletionKind::Method => "ƒ",
            CompletionKind::Variable | CompletionKind::Field => "x",
            CompletionKind::Class | CompletionKind::Struct => "C",
            CompletionKind::Interface => "I",
            CompletionKind::Enum => "E",
            CompletionKind::Keyword => "K",
            CompletionKind::Snippet => "S",
            CompletionKind::AI_Suggestion => "✨",
            _ => "T",
        }
    }

    /// Accept a completion item
    pub fn accept_completion(&mut self, item: CompletionItem) {
        if let Some(session) = &self.active_session {
            // Record acceptance for learning
            self.learning_system.record_acceptance(
                item.clone(),
                session.context.current_line.clone(),
                session.start_time.elapsed(),
            );

            // Update metrics
            self.metrics.acceptance_rate = self.calculate_acceptance_rate();
            *self.metrics.popular_kinds.entry(item.kind.clone()).or_insert(0) += 1;
        }

        self.active_session = None;
    }

    /// Cancel current completion session
    pub fn cancel_completion(&mut self) {
        self.active_session = None;
    }

    /// Generate cache key for completion context
    fn generate_cache_key(&self, context: &CompletionContext) -> String {
        format!("{}:{}:{}:{}", 
            context.file_path,
            context.language,
            context.current_line,
            context.cursor_position
        )
    }

    /// Check if provider supports language
    fn provider_supports_language(&self, provider: &CompletionProvider, language: &str) -> bool {
        provider.supported_languages.contains(&"*".to_string()) ||
        provider.supported_languages.contains(&language.to_string())
    }

    /// Update response time metric
    fn update_response_time(&mut self, response_time: f32) {
        let total_time = self.metrics.avg_response_time * self.metrics.total_requests as f32;
        self.metrics.avg_response_time = (total_time + response_time) / (self.metrics.total_requests + 1) as f32;
    }

    /// Calculate acceptance rate
    fn calculate_acceptance_rate(&self) -> f32 {
        if self.learning_system.acceptance_history.is_empty() {
            return 0.0;
        }
        
        let recent_acceptances = self.learning_system.acceptance_history.len();
        let total_sessions = self.metrics.total_requests;
        
        if total_sessions == 0 {
            0.0
        } else {
            recent_acceptances as f32 / total_sessions as f32
        }
    }

    /// Get completion statistics
    pub fn get_statistics(&self) -> &CompletionMetrics {
        &self.metrics
    }

    /// Update settings
    pub fn update_settings(&mut self, settings: CompletionSettings) {
        self.settings = settings;
    }
}

impl CompletionCache {
    fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
            hit_count: 0,
            miss_count: 0,
        }
    }

    fn get(&mut self, key: &str) -> Option<Vec<CompletionItem>> {
        if let Some(cached) = self.cache.get(key) {
            if cached.timestamp.elapsed() < cached.ttl {
                self.hit_count += 1;
                return Some(cached.items.clone());
            } else {
                self.cache.remove(key);
            }
        }
        
        self.miss_count += 1;
        None
    }

    fn insert(&mut self, key: String, items: Vec<CompletionItem>, ttl: std::time::Duration) {
        if self.cache.len() >= self.max_size {
            // Remove oldest entry
            if let Some(oldest_key) = self.cache.keys().next().cloned() {
                self.cache.remove(&oldest_key);
            }
        }

        self.cache.insert(key, CachedCompletion {
            items,
            timestamp: std::time::Instant::now(),
            ttl,
        });
    }
}

impl CompletionLearningSystem {
    fn new() -> Self {
        Self {
            acceptance_history: VecDeque::new(),
            frequent_completions: HashMap::new(),
            context_patterns: HashMap::new(),
            enabled: true,
        }
    }

    fn record_acceptance(&mut self, item: CompletionItem, context: String, acceptance_time: std::time::Duration) {
        if !self.enabled {
            return;
        }

        // Record acceptance
        self.acceptance_history.push_back(CompletionAcceptance {
            item: item.clone(),
            context: context.clone(),
            timestamp: std::time::Instant::now(),
            acceptance_time,
        });

        // Limit history size
        if self.acceptance_history.len() > 1000 {
            self.acceptance_history.pop_front();
        }

        // Update frequency
        *self.frequent_completions.entry(item.label).or_insert(0.0) += 1.0;

        // Update context patterns
        self.context_patterns
            .entry(context)
            .or_insert_with(Vec::new)
            .push(item.label);
    }
}

impl Default for CompletionMetrics {
    fn default() -> Self {
        Self {
            total_requests: 0,
            avg_response_time: 0.0,
            cache_hit_rate: 0.0,
            acceptance_rate: 0.0,
            popular_kinds: HashMap::new(),
        }
    }
}

// Include uuid for session IDs
mod uuid {
    pub struct Uuid;
    impl Uuid {
        pub fn new_v4() -> Self { Self }
        pub fn to_string(&self) -> String {
            format!("session_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis())
        }
    }
}