//! Advanced Code Completion System
//!
//! Provides intelligent code completion with context awareness, documentation,
//! snippets, and AI-powered suggestions.

use egui::*;
use std::collections::HashMap;
use crate::editor::lsp_integration::{CompletionItem, Position};

/// Advanced code completion engine
pub struct CodeCompletionEngine {
    /// Completion providers for different languages
    pub providers: HashMap<String, Box<dyn CompletionProvider>>,
    /// Current completion context
    pub current_context: Option<CompletionContext>,
    /// Completion cache for performance
    pub completion_cache: CompletionCache,
    /// Snippet manager for code templates
    pub snippet_manager: SnippetManager,
    /// Settings for completion behavior
    pub settings: CompletionSettings,
}

/// Context information for code completion
#[derive(Debug, Clone)]
pub struct CompletionContext {
    /// Current file language
    pub language: String,
    /// Cursor position in document
    pub position: Position,
    /// Line content up to cursor
    pub line_prefix: String,
    /// Word being typed
    pub current_word: String,
    /// Surrounding code context
    pub surrounding_lines: Vec<String>,
    /// Whether we're inside a comment
    pub in_comment: bool,
    /// Whether we're inside a string
    pub in_string: bool,
    /// Current scope/function context
    pub scope_context: Option<ScopeContext>,
}

/// Scope context for better completions
#[derive(Debug, Clone)]
pub struct ScopeContext {
    /// Function name if inside a function
    pub function_name: Option<String>,
    /// Local variables in scope
    pub local_variables: Vec<Variable>,
    /// Available imports
    pub imports: Vec<ImportInfo>,
    /// Current struct/class if inside one
    pub current_type: Option<String>,
}

/// Variable information
#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub var_type: String,
    pub is_mutable: bool,
    pub scope_level: usize,
}

/// Import information
#[derive(Debug, Clone)]
pub struct ImportInfo {
    pub module_path: String,
    pub imported_items: Vec<String>,
    pub alias: Option<String>,
}

/// Completion provider trait
pub trait CompletionProvider: Send + Sync {
    /// Get completions for the given context
    fn get_completions(&self, context: &CompletionContext) -> Vec<EnhancedCompletionItem>;
    
    /// Get signature help for function calls
    fn get_signature_help(&self, context: &CompletionContext) -> Option<SignatureHelp>;
    
    /// Validate if this provider can handle the context
    fn can_handle(&self, language: &str) -> bool;
}

/// Enhanced completion item with rich information
#[derive(Debug, Clone)]
pub struct EnhancedCompletionItem {
    /// Basic completion item from LSP
    pub base_item: CompletionItem,
    /// Detailed documentation
    pub documentation: Option<Documentation>,
    /// Code snippet template
    pub snippet: Option<CodeSnippet>,
    /// Completion priority/score
    pub priority: f32,
    /// Whether this requires additional imports
    pub requires_import: Option<ImportRequirement>,
    /// Usage examples
    pub examples: Vec<String>,
    /// Deprecation information
    pub deprecated: Option<DeprecationInfo>,
}

/// Documentation for completion items
#[derive(Debug, Clone)]
pub struct Documentation {
    /// Brief description
    pub summary: String,
    /// Detailed description
    pub details: Option<String>,
    /// Return type information
    pub returns: Option<String>,
    /// Parameter information
    pub parameters: Vec<ParameterDoc>,
    /// Links to external documentation
    pub links: Vec<String>,
}

/// Parameter documentation
#[derive(Debug, Clone)]
pub struct ParameterDoc {
    pub name: String,
    pub param_type: String,
    pub description: String,
    pub optional: bool,
}

/// Code snippet template
#[derive(Debug, Clone)]
pub struct CodeSnippet {
    /// Template with placeholders
    pub template: String,
    /// Cursor positions in template
    pub cursor_positions: Vec<usize>,
    /// Placeholder descriptions
    pub placeholders: HashMap<String, String>,
}

/// Import requirement for completion
#[derive(Debug, Clone)]
pub struct ImportRequirement {
    /// Module to import from
    pub module: String,
    /// Specific items to import
    pub items: Vec<String>,
    /// Whether to import the whole module
    pub import_all: bool,
}

/// Deprecation information
#[derive(Debug, Clone)]
pub struct DeprecationInfo {
    /// Deprecation reason
    pub reason: String,
    /// Suggested alternative
    pub alternative: Option<String>,
    /// Version when deprecated
    pub since_version: Option<String>,
}

/// Signature help for function calls
#[derive(Debug, Clone)]
pub struct SignatureHelp {
    /// Available signatures (overloads)
    pub signatures: Vec<FunctionSignature>,
    /// Currently active signature
    pub active_signature: usize,
    /// Currently active parameter
    pub active_parameter: usize,
}

/// Function signature information
#[derive(Debug, Clone)]
pub struct FunctionSignature {
    /// Function name
    pub name: String,
    /// Function parameters
    pub parameters: Vec<Parameter>,
    /// Return type
    pub return_type: Option<String>,
    /// Function documentation
    pub documentation: Option<String>,
}

/// Function parameter
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: String,
    pub optional: bool,
    pub default_value: Option<String>,
}

/// Completion cache for performance
pub struct CompletionCache {
    /// Cached completions by context hash
    cache: HashMap<u64, Vec<EnhancedCompletionItem>>,
    /// Cache timestamps for invalidation
    timestamps: HashMap<u64, std::time::Instant>,
    /// Maximum cache age in seconds
    max_age: u64,
    /// Maximum cache size
    max_size: usize,
}

impl CompletionCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            timestamps: HashMap::new(),
            max_age: 300, // 5 minutes
            max_size: 1000,
        }
    }

    pub fn get(&mut self, context_hash: u64) -> Option<Vec<EnhancedCompletionItem>> {
        // Check if entry exists and is not expired
        if let Some(timestamp) = self.timestamps.get(&context_hash) {
            if timestamp.elapsed().as_secs() < self.max_age {
                return self.cache.get(&context_hash).cloned();
            } else {
                // Remove expired entry
                self.cache.remove(&context_hash);
                self.timestamps.remove(&context_hash);
            }
        }
        None
    }

    pub fn insert(&mut self, context_hash: u64, completions: Vec<EnhancedCompletionItem>) {
        // Limit cache size
        if self.cache.len() >= self.max_size {
            // Remove oldest entries
            let mut entries: Vec<_> = self.timestamps.iter().collect();
            entries.sort_by_key(|(_, timestamp)| *timestamp);
            
            if let Some((oldest_key, _)) = entries.first() {
                let key = **oldest_key;
                self.cache.remove(&key);
                self.timestamps.remove(&key);
            }
        }

        self.cache.insert(context_hash, completions);
        self.timestamps.insert(context_hash, std::time::Instant::now());
    }

    pub fn clear(&mut self) {
        self.cache.clear();
        self.timestamps.clear();
    }
}

/// Snippet manager for code templates
pub struct SnippetManager {
    /// Language-specific snippets
    pub snippets: HashMap<String, Vec<Snippet>>,
    /// Global snippets (available in all languages)
    pub global_snippets: Vec<Snippet>,
    /// User-defined custom snippets
    pub custom_snippets: Vec<Snippet>,
}

/// Code snippet definition
#[derive(Debug, Clone)]
pub struct Snippet {
    /// Snippet name/trigger
    pub name: String,
    /// Brief description
    pub description: String,
    /// Code template
    pub template: String,
    /// Languages where this snippet is available
    pub languages: Vec<String>,
    /// Snippet category
    pub category: SnippetCategory,
}

/// Snippet categories
#[derive(Debug, Clone, PartialEq)]
pub enum SnippetCategory {
    Control,      // if, for, while, etc.
    Function,     // function definitions
    Class,        // class/struct definitions
    Import,       // import statements
    Documentation, // doc comments
    Test,         // test templates
    Custom,       // user-defined
}

impl SnippetManager {
    pub fn new() -> Self {
        let mut manager = Self {
            snippets: HashMap::new(),
            global_snippets: Vec::new(),
            custom_snippets: Vec::new(),
        };
        
        manager.load_default_snippets();
        manager
    }

    /// Load default language-specific snippets
    fn load_default_snippets(&mut self) {
        // Rust snippets
        let rust_snippets = vec![
            Snippet {
                name: "fn".to_string(),
                description: "Function definition".to_string(),
                template: "fn ${1:name}(${2:params}) ${3:-> ReturnType} {\n    ${4:// body}\n}".to_string(),
                languages: vec!["rust".to_string()],
                category: SnippetCategory::Function,
            },
            Snippet {
                name: "if".to_string(),
                description: "If statement".to_string(),
                template: "if ${1:condition} {\n    ${2:// body}\n}".to_string(),
                languages: vec!["rust".to_string()],
                category: SnippetCategory::Control,
            },
            Snippet {
                name: "match".to_string(),
                description: "Match expression".to_string(),
                template: "match ${1:expr} {\n    ${2:pattern} => ${3:value},\n    _ => ${4:default},\n}".to_string(),
                languages: vec!["rust".to_string()],
                category: SnippetCategory::Control,
            },
            Snippet {
                name: "struct".to_string(),
                description: "Struct definition".to_string(),
                template: "struct ${1:Name} {\n    ${2:field}: ${3:Type},\n}".to_string(),
                languages: vec!["rust".to_string()],
                category: SnippetCategory::Class,
            },
        ];

        self.snippets.insert("rust".to_string(), rust_snippets);
    }

    /// Get snippets for a specific language
    pub fn get_snippets_for_language(&self, language: &str) -> Vec<&Snippet> {
        let mut snippets = Vec::new();
        
        // Add language-specific snippets
        if let Some(lang_snippets) = self.snippets.get(language) {
            snippets.extend(lang_snippets.iter());
        }
        
        // Add global snippets
        snippets.extend(self.global_snippets.iter());
        
        // Add custom snippets
        snippets.extend(
            self.custom_snippets
                .iter()
                .filter(|s| s.languages.is_empty() || s.languages.contains(&language.to_string()))
        );
        
        snippets
    }

    /// Add a custom snippet
    pub fn add_custom_snippet(&mut self, snippet: Snippet) {
        self.custom_snippets.push(snippet);
    }
}

/// Completion settings
#[derive(Debug, Clone)]
pub struct CompletionSettings {
    /// Enable automatic completion popup
    pub auto_popup: bool,
    /// Minimum characters to trigger completion
    pub min_chars: usize,
    /// Maximum number of completion items to show
    pub max_items: usize,
    /// Enable fuzzy matching
    pub fuzzy_matching: bool,
    /// Enable snippet completion
    pub enable_snippets: bool,
    /// Enable AI-powered completions
    pub enable_ai: bool,
    /// Sort by relevance vs alphabetical
    pub sort_by_relevance: bool,
}

impl Default for CompletionSettings {
    fn default() -> Self {
        Self {
            auto_popup: true,
            min_chars: 2,
            max_items: 50,
            fuzzy_matching: true,
            enable_snippets: true,
            enable_ai: true,
            sort_by_relevance: true,
        }
    }
}

impl CodeCompletionEngine {
    /// Create a new code completion engine
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            current_context: None,
            completion_cache: CompletionCache::new(),
            snippet_manager: SnippetManager::new(),
            settings: CompletionSettings::default(),
        }
    }

    /// Register a completion provider for a language
    pub fn register_provider(&mut self, language: String, provider: Box<dyn CompletionProvider>) {
        self.providers.insert(language, provider);
    }

    /// Get completions for the current context
    pub fn get_completions(&mut self, context: CompletionContext) -> Vec<EnhancedCompletionItem> {
        // Generate context hash for caching
        let context_hash = self.hash_context(&context);
        
        // Check cache first
        if let Some(cached) = self.completion_cache.get(context_hash) {
            return cached;
        }

        let mut completions = Vec::new();

        // Get completions from LSP provider
        if let Some(provider) = self.providers.get(&context.language) {
            completions.extend(provider.get_completions(&context));
        }

        // Add snippet completions
        if self.settings.enable_snippets {
            completions.extend(self.get_snippet_completions(&context));
        }

        // Sort and filter completions
        self.filter_and_sort_completions(&mut completions, &context);

        // Cache the results
        self.completion_cache.insert(context_hash, completions.clone());

        completions
    }

    /// Get snippet completions
    fn get_snippet_completions(&self, context: &CompletionContext) -> Vec<EnhancedCompletionItem> {
        let snippets = self.snippet_manager.get_snippets_for_language(&context.language);
        
        snippets
            .into_iter()
            .filter(|snippet| {
                // Filter based on current word
                if !context.current_word.is_empty() {
                    snippet.name.starts_with(&context.current_word)
                } else {
                    true
                }
            })
            .map(|snippet| {
                EnhancedCompletionItem {
                    base_item: CompletionItem {
                        label: snippet.name.clone(),
                        kind: Some(crate::editor::lsp_integration::CompletionItemKind::Snippet),
                        detail: Some(snippet.description.clone()),
                        documentation: None,
                        sort_text: Some(snippet.name.clone()),
                        filter_text: Some(snippet.name.clone()),
                        insert_text: Some(snippet.template.clone()),
                        insert_text_format: Some(crate::editor::lsp_integration::InsertTextFormat::Snippet),
                    },
                    documentation: Some(Documentation {
                        summary: snippet.description.clone(),
                        details: None,
                        returns: None,
                        parameters: Vec::new(),
                        links: Vec::new(),
                    }),
                    snippet: Some(CodeSnippet {
                        template: snippet.template.clone(),
                        cursor_positions: Vec::new(),
                        placeholders: HashMap::new(),
                    }),
                    priority: 0.8, // High priority for snippets
                    requires_import: None,
                    examples: Vec::new(),
                    deprecated: None,
                }
            })
            .collect()
    }

    /// Filter and sort completions
    fn filter_and_sort_completions(&self, completions: &mut Vec<EnhancedCompletionItem>, context: &CompletionContext) {
        // Filter based on current word
        if !context.current_word.is_empty() && self.settings.fuzzy_matching {
            completions.retain(|item| {
                self.fuzzy_match(&item.base_item.label, &context.current_word)
            });
        }

        // Sort by relevance or alphabetically
        if self.settings.sort_by_relevance {
            completions.sort_by(|a, b| {
                b.priority.partial_cmp(&a.priority).unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| a.base_item.label.cmp(&b.base_item.label))
            });
        } else {
            completions.sort_by(|a, b| a.base_item.label.cmp(&b.base_item.label));
        }

        // Limit to max items
        completions.truncate(self.settings.max_items);
    }

    /// Simple fuzzy matching
    fn fuzzy_match(&self, text: &str, pattern: &str) -> bool {
        let text_lower = text.to_lowercase();
        let pattern_lower = pattern.to_lowercase();
        
        // Simple contains check for now - could be enhanced with more sophisticated fuzzy matching
        text_lower.contains(&pattern_lower)
    }

    /// Hash context for caching
    fn hash_context(&self, context: &CompletionContext) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        context.language.hash(&mut hasher);
        context.position.line.hash(&mut hasher);
        context.position.character.hash(&mut hasher);
        context.line_prefix.hash(&mut hasher);
        context.current_word.hash(&mut hasher);
        
        hasher.finish()
    }

    /// Render completion popup
    pub fn render_completion_popup(&mut self, ui: &mut Ui, available_rect: Rect) -> Option<String> {
        if let Some(context) = &self.current_context {
            let completions = self.get_completions(context.clone());
            
            if completions.is_empty() {
                return None;
            }

            let mut selected_completion = None;

            // Show completion popup
            let popup_size = Vec2::new(300.0, 200.0);
            let popup_rect = Rect::from_min_size(
                available_rect.min + Vec2::new(0.0, 20.0),
                popup_size,
            );

            ui.allocate_ui_at_rect(popup_rect, |ui| {
                Frame::popup(ui.style()).show(ui, |ui| {
                    ScrollArea::vertical()
                        .max_height(180.0)
                        .show(ui, |ui| {
                            for (i, completion) in completions.iter().enumerate() {
                                let is_selected = i == 0; // For now, always select first
                                
                                ui.horizontal(|ui| {
                                    // Icon based on completion kind
                                    let icon = match &completion.base_item.kind {
                                        Some(crate::editor::lsp_integration::CompletionItemKind::Function) => "🔧",
                                        Some(crate::editor::lsp_integration::CompletionItemKind::Variable) => "🔤",
                                        Some(crate::editor::lsp_integration::CompletionItemKind::Class) => "📦",
                                        Some(crate::editor::lsp_integration::CompletionItemKind::Snippet) => "📝",
                                        _ => "📄",
                                    };
                                    
                                    ui.label(icon);
                                    
                                    let label = if is_selected {
                                        RichText::new(&completion.base_item.label).color(Color32::WHITE)
                                    } else {
                                        RichText::new(&completion.base_item.label)
                                    };
                                    
                                    if ui.selectable_label(is_selected, label).clicked() {
                                        selected_completion = Some(completion.base_item.insert_text
                                            .as_ref()
                                            .unwrap_or(&completion.base_item.label)
                                            .clone()
                                        );
                                    }
                                    
                                    // Show type/detail if available
                                    if let Some(detail) = &completion.base_item.detail {
                                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                            ui.label(RichText::new(detail).color(Color32::GRAY));
                                        });
                                    }
                                });
                            }
                        });
                });
            });

            selected_completion
        } else {
            None
        }
    }
}

impl Default for CodeCompletionEngine {
    fn default() -> Self {
        Self::new()
    }
}