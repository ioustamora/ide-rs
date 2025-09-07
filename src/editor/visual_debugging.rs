//! Visual Debugging and Inspection Tools
//!
//! This module provides advanced visual debugging capabilities including:
//! - Interactive variable inspection
//! - Real-time value watches
//! - Memory visualization
//! - Call stack visualization
//! - Interactive breakpoint management

use std::collections::{HashMap, VecDeque};
use serde::{Deserialize, Serialize};
use egui::{Ui, Vec2, Color32, Rect, Pos2, Stroke, RichText, FontId, Sense};

/// Main visual debugging engine
#[derive(Debug, Clone)]
pub struct VisualDebuggingEngine {
    /// Active debugging session
    active_session: Option<DebugSession>,
    /// Variable watchers
    watchers: Vec<VariableWatcher>,
    /// Debug visualizers
    visualizers: HashMap<String, Box<dyn DebugVisualizer>>,
    /// Breakpoint manager
    breakpoint_manager: BreakpointManager,
    /// Memory inspector
    memory_inspector: MemoryInspector,
    /// Call stack viewer
    call_stack: CallStackViewer,
    /// Debug settings
    settings: DebugSettings,
    /// Debug metrics
    metrics: DebugMetrics,
}

/// Active debugging session
#[derive(Debug, Clone)]
pub struct DebugSession {
    /// Session ID
    pub id: String,
    /// Target process/program
    pub target: String,
    /// Current execution state
    pub state: ExecutionState,
    /// Current line/position
    pub current_position: Option<SourcePosition>,
    /// Session start time
    pub start_time: std::time::Instant,
    /// Session configuration
    pub config: SessionConfig,
    /// Local variables in current scope
    pub local_variables: HashMap<String, DebugValue>,
    /// Global variables
    pub global_variables: HashMap<String, DebugValue>,
}

/// Execution state of the debugging session
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionState {
    Running,
    Paused,
    Stopped,
    StepInto,
    StepOver,
    StepOut,
    Crashed(String),
}

/// Source code position
#[derive(Debug, Clone)]
pub struct SourcePosition {
    pub file_path: String,
    pub line: usize,
    pub column: usize,
    pub function_name: Option<String>,
}

/// Debug session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Auto-watch variables
    pub auto_watch: bool,
    /// Update frequency (ms)
    pub update_frequency: u64,
    /// Maximum watch history
    pub max_history: usize,
    /// Enable memory visualization
    pub memory_visualization: bool,
    /// Show advanced data structures
    pub show_advanced_structures: bool,
}

/// Variable watcher for tracking value changes
#[derive(Debug, Clone)]
pub struct VariableWatcher {
    /// Variable name/expression
    pub expression: String,
    /// Display name
    pub display_name: String,
    /// Current value
    pub current_value: Option<DebugValue>,
    /// Value history for trend analysis
    pub value_history: VecDeque<ValueSnapshot>,
    /// Whether watcher is enabled
    pub enabled: bool,
    /// Update condition
    pub condition: Option<String>,
    /// Visualization style
    pub visualization: WatchVisualization,
}

/// Snapshot of a variable value at a point in time
#[derive(Debug, Clone)]
pub struct ValueSnapshot {
    pub value: DebugValue,
    pub timestamp: std::time::Instant,
    pub context: String,
}

/// Debug value representation
#[derive(Debug, Clone)]
pub struct DebugValue {
    /// Raw value as string
    pub raw_value: String,
    /// Type information
    pub type_info: TypeInfo,
    /// Memory address (if applicable)
    pub memory_address: Option<usize>,
    /// Size in bytes
    pub size: usize,
    /// Whether value has changed since last update
    pub changed: bool,
    /// Child values (for complex types)
    pub children: Vec<DebugValue>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Type information for debug values
#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub name: String,
    pub kind: TypeKind,
    pub is_pointer: bool,
    pub is_reference: bool,
    pub is_mutable: bool,
    pub generic_parameters: Vec<String>,
}

/// Type classification
#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    Primitive,
    Struct,
    Enum,
    Array,
    Vector,
    HashMap,
    String,
    Option,
    Result,
    Custom(String),
}

/// Visualization style for watched variables
#[derive(Debug, Clone, PartialEq)]
pub enum WatchVisualization {
    Text,
    Graph,
    Chart,
    Tree,
    Memory,
    Custom(String),
}

/// Trait for debug value visualizers
pub trait DebugVisualizer {
    fn name(&self) -> &str;
    fn can_visualize(&self, value: &DebugValue) -> bool;
    fn render(&mut self, ui: &mut Ui, value: &DebugValue) -> egui::Response;
    fn get_settings(&self) -> Vec<VisualizerSetting>;
    fn set_setting(&mut self, name: &str, value: &str) -> bool;
}

/// Visualizer setting
#[derive(Debug, Clone)]
pub struct VisualizerSetting {
    pub name: String,
    pub display_name: String,
    pub setting_type: SettingType,
    pub current_value: String,
    pub possible_values: Vec<String>,
}

/// Setting type for visualizers
#[derive(Debug, Clone, PartialEq)]
pub enum SettingType {
    Boolean,
    String,
    Number,
    Choice,
    Color,
}

/// Breakpoint management system
#[derive(Debug, Clone)]
pub struct BreakpointManager {
    /// All breakpoints
    pub breakpoints: HashMap<String, Breakpoint>,
    /// Conditional breakpoints
    pub conditional_breakpoints: Vec<ConditionalBreakpoint>,
    /// Log points (breakpoints that log instead of pause)
    pub log_points: Vec<LogPoint>,
    /// Breakpoint statistics
    pub statistics: BreakpointStatistics,
}

/// Individual breakpoint
#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub id: String,
    pub file_path: String,
    pub line: usize,
    pub enabled: bool,
    pub hit_count: usize,
    pub condition: Option<String>,
    pub log_message: Option<String>,
    pub created_at: std::time::Instant,
}

/// Conditional breakpoint with complex conditions
#[derive(Debug, Clone)]
pub struct ConditionalBreakpoint {
    pub breakpoint: Breakpoint,
    pub condition_expression: String,
    pub hit_count_condition: Option<HitCountCondition>,
}

/// Hit count condition for breakpoints
#[derive(Debug, Clone)]
pub struct HitCountCondition {
    pub condition_type: HitCountType,
    pub value: usize,
}

/// Hit count condition types
#[derive(Debug, Clone, PartialEq)]
pub enum HitCountType {
    Equal,
    GreaterThan,
    LessThan,
    Multiple,
}

/// Log point for non-intrusive debugging
#[derive(Debug, Clone)]
pub struct LogPoint {
    pub breakpoint: Breakpoint,
    pub log_expression: String,
    pub log_level: LogLevel,
}

/// Log levels for log points
#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Breakpoint usage statistics
#[derive(Debug, Clone)]
pub struct BreakpointStatistics {
    pub total_breakpoints: usize,
    pub active_breakpoints: usize,
    pub total_hits: usize,
    pub most_hit_breakpoint: Option<String>,
    pub average_hits_per_breakpoint: f32,
}

/// Memory inspector for low-level debugging
#[derive(Debug, Clone)]
pub struct MemoryInspector {
    /// Memory regions to inspect
    pub regions: Vec<MemoryRegion>,
    /// Current view settings
    pub view_settings: MemoryViewSettings,
    /// Memory search results
    pub search_results: Vec<MemorySearchResult>,
    /// Memory bookmarks
    pub bookmarks: Vec<MemoryBookmark>,
}

/// Memory region definition
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub name: String,
    pub start_address: usize,
    pub end_address: usize,
    pub permissions: MemoryPermissions,
    pub region_type: MemoryRegionType,
    pub data: Vec<u8>,
}

/// Memory permissions
#[derive(Debug, Clone)]
pub struct MemoryPermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

/// Memory region types
#[derive(Debug, Clone, PartialEq)]
pub enum MemoryRegionType {
    Code,
    Data,
    Stack,
    Heap,
    Shared,
    Unknown,
}

/// Memory view settings
#[derive(Debug, Clone)]
pub struct MemoryViewSettings {
    pub bytes_per_row: usize,
    pub display_format: MemoryDisplayFormat,
    pub show_ascii: bool,
    pub highlight_changes: bool,
    pub address_format: AddressFormat,
}

/// Memory display formats
#[derive(Debug, Clone, PartialEq)]
pub enum MemoryDisplayFormat {
    Hex,
    Decimal,
    Binary,
    Float,
    Double,
}

/// Address display formats
#[derive(Debug, Clone, PartialEq)]
pub enum AddressFormat {
    Hex,
    Decimal,
    Relative,
}

/// Memory search result
#[derive(Debug, Clone)]
pub struct MemorySearchResult {
    pub address: usize,
    pub pattern: String,
    pub context: Vec<u8>,
}

/// Memory bookmark for important addresses
#[derive(Debug, Clone)]
pub struct MemoryBookmark {
    pub name: String,
    pub address: usize,
    pub description: String,
    pub color: Color32,
}

/// Call stack viewer
#[derive(Debug, Clone)]
pub struct CallStackViewer {
    /// Current call stack
    pub frames: Vec<StackFrame>,
    /// Selected frame index
    pub selected_frame: usize,
    /// Frame navigation history
    pub navigation_history: Vec<usize>,
    /// Display settings
    pub display_settings: CallStackDisplaySettings,
}

/// Individual stack frame
#[derive(Debug, Clone)]
pub struct StackFrame {
    pub function_name: String,
    pub file_path: String,
    pub line: usize,
    pub column: usize,
    pub local_variables: HashMap<String, DebugValue>,
    pub parameters: Vec<DebugValue>,
    pub return_address: Option<usize>,
    pub frame_pointer: Option<usize>,
}

/// Call stack display settings
#[derive(Debug, Clone)]
pub struct CallStackDisplaySettings {
    pub show_parameters: bool,
    pub show_local_variables: bool,
    pub show_addresses: bool,
    pub max_frames: usize,
    pub filter_system_frames: bool,
}

/// Debug settings configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugSettings {
    /// Enable visual debugging
    pub enabled: bool,
    /// Auto-attach to new processes
    pub auto_attach: bool,
    /// Update frequency for watches (ms)
    pub update_frequency: u64,
    /// Maximum history entries
    pub max_history_entries: usize,
    /// Enable advanced visualizations
    pub advanced_visualizations: bool,
    /// Memory inspection enabled
    pub memory_inspection_enabled: bool,
    /// Show system internals
    pub show_system_internals: bool,
}

/// Debug metrics and statistics
#[derive(Debug, Clone)]
pub struct DebugMetrics {
    pub total_sessions: usize,
    pub total_breakpoints_hit: usize,
    pub total_variables_watched: usize,
    pub average_session_duration: f32,
    pub most_watched_variables: HashMap<String, usize>,
    pub performance_impact: f32,
}

impl Default for VisualDebuggingEngine {
    fn default() -> Self {
        Self {
            active_session: None,
            watchers: Vec::new(),
            visualizers: HashMap::new(),
            breakpoint_manager: BreakpointManager::new(),
            memory_inspector: MemoryInspector::new(),
            call_stack: CallStackViewer::new(),
            settings: DebugSettings::default(),
            metrics: DebugMetrics::default(),
        }
    }
}

impl Default for DebugSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_attach: false,
            update_frequency: 100,
            max_history_entries: 1000,
            advanced_visualizations: true,
            memory_inspection_enabled: false,
            show_system_internals: false,
        }
    }
}

impl VisualDebuggingEngine {
    /// Create a new visual debugging engine
    pub fn new() -> Self {
        let mut engine = Self::default();
        engine.initialize_default_visualizers();
        engine
    }

    /// Initialize default visualizers
    fn initialize_default_visualizers(&mut self) {
        // Add built-in visualizers
        self.visualizers.insert("text".to_string(), Box::new(TextVisualizer::new()));
        self.visualizers.insert("graph".to_string(), Box::new(GraphVisualizer::new()));
        self.visualizers.insert("tree".to_string(), Box::new(TreeVisualizer::new()));
        self.visualizers.insert("memory".to_string(), Box::new(MemoryVisualizer::new()));
    }

    /// Start a new debugging session
    pub fn start_session(&mut self, target: String, config: SessionConfig) -> Result<String, String> {
        let session_id = format!("debug_session_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis());

        let session = DebugSession {
            id: session_id.clone(),
            target,
            state: ExecutionState::Running,
            current_position: None,
            start_time: std::time::Instant::now(),
            config,
            local_variables: HashMap::new(),
            global_variables: HashMap::new(),
        };

        self.active_session = Some(session);
        self.metrics.total_sessions += 1;

        Ok(session_id)
    }

    /// Stop the current debugging session
    pub fn stop_session(&mut self) {
        if let Some(session) = &self.active_session {
            let duration = session.start_time.elapsed().as_secs_f32();
            self.metrics.average_session_duration = 
                (self.metrics.average_session_duration * (self.metrics.total_sessions - 1) as f32 + duration) 
                / self.metrics.total_sessions as f32;
        }
        self.active_session = None;
    }

    /// Add a variable watcher
    pub fn add_watcher(&mut self, expression: String, display_name: String) -> Result<(), String> {
        let watcher = VariableWatcher {
            expression: expression.clone(),
            display_name,
            current_value: None,
            value_history: VecDeque::new(),
            enabled: true,
            condition: None,
            visualization: WatchVisualization::Text,
        };

        self.watchers.push(watcher);
        *self.metrics.most_watched_variables.entry(expression).or_insert(0) += 1;
        self.metrics.total_variables_watched += 1;

        Ok(())
    }

    /// Update all watchers with current values
    pub fn update_watchers(&mut self) -> Result<(), String> {
        if self.active_session.is_none() {
            return Err("No active debugging session".to_string());
        }

        // Collect expressions first to avoid borrowing conflicts
        let expressions: Vec<String> = self.watchers
            .iter()
            .filter(|w| w.enabled)
            .map(|w| w.expression.clone())
            .collect();

        for (i, expression) in expressions.iter().enumerate() {
            // Simulate getting value from debugger
            let new_value = self.get_variable_value(expression)?;
            
            // Find the corresponding watcher and update it
            if let Some(watcher) = self.watchers.get_mut(i) {
                // Check if value changed
                let changed = match &watcher.current_value {
                    Some(current) => current.raw_value != new_value.raw_value,
                    None => true,
                };

                if changed {
                    let snapshot = ValueSnapshot {
                        value: new_value.clone(),
                        timestamp: std::time::Instant::now(),
                        context: expression.clone(),
                    };

                    watcher.value_history.push_back(snapshot);
                    
                    // Limit history size
                    if watcher.value_history.len() > self.settings.max_history_entries {
                        watcher.value_history.pop_front();
                    }
                }

                watcher.current_value = Some(new_value);
            }
        }

        Ok(())
    }

    /// Get variable value (simulated)
    fn get_variable_value(&self, expression: &str) -> Result<DebugValue, String> {
        // This would interface with actual debugger
        Ok(DebugValue {
            raw_value: format!("value_of_{}", expression),
            type_info: TypeInfo {
                name: "i32".to_string(),
                kind: TypeKind::Primitive,
                is_pointer: false,
                is_reference: false,
                is_mutable: true,
                generic_parameters: Vec::new(),
            },
            memory_address: Some(0x1000),
            size: 4,
            changed: false,
            children: Vec::new(),
            metadata: HashMap::new(),
        })
    }

    /// Add breakpoint
    pub fn add_breakpoint(&mut self, file_path: String, line: usize) -> Result<String, String> {
        let breakpoint_id = format!("bp_{}_{}", file_path.replace('/', "_"), line);
        
        let breakpoint = Breakpoint {
            id: breakpoint_id.clone(),
            file_path,
            line,
            enabled: true,
            hit_count: 0,
            condition: None,
            log_message: None,
            created_at: std::time::Instant::now(),
        };

        self.breakpoint_manager.breakpoints.insert(breakpoint_id.clone(), breakpoint);
        self.breakpoint_manager.statistics.total_breakpoints += 1;
        self.breakpoint_manager.statistics.active_breakpoints += 1;

        Ok(breakpoint_id)
    }

    /// Remove breakpoint
    pub fn remove_breakpoint(&mut self, breakpoint_id: &str) -> Result<(), String> {
        if self.breakpoint_manager.breakpoints.remove(breakpoint_id).is_some() {
            if self.breakpoint_manager.statistics.active_breakpoints > 0 {
                self.breakpoint_manager.statistics.active_breakpoints -= 1;
            }
            Ok(())
        } else {
            Err(format!("Breakpoint not found: {}", breakpoint_id))
        }
    }

    /// Render debug interface
    pub fn render_debug_interface(&mut self, ui: &mut Ui) {
        if !self.settings.enabled {
            ui.label("Visual debugging is disabled");
            return;
        }

        ui.heading("Visual Debugging");

        // Session status
        self.render_session_status(ui);

        ui.separator();

        // Tabs for different debug views
        egui::TopBottomPanel::top("debug_tabs").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_label(false, "Variables");
                ui.selectable_label(false, "Call Stack");
                ui.selectable_label(false, "Breakpoints");
                ui.selectable_label(false, "Memory");
                ui.selectable_label(false, "Settings");
            });
        });

        // Variables panel
        self.render_variables_panel(ui);
    }

    /// Render session status
    fn render_session_status(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("Status:");
                
                if let Some(session) = &self.active_session {
                    let status_color = match session.state {
                        ExecutionState::Running => Color32::GREEN,
                        ExecutionState::Paused => Color32::YELLOW,
                        ExecutionState::Stopped => Color32::RED,
                        ExecutionState::Crashed(_) => Color32::DARK_RED,
                        _ => Color32::GRAY,
                    };

                    ui.colored_label(status_color, format!("{:?}", session.state));
                    ui.separator();
                    ui.label(format!("Target: {}", session.target));
                    
                    if let Some(pos) = &session.current_position {
                        ui.separator();
                        ui.label(format!("{}:{}", pos.file_path, pos.line));
                    }
                } else {
                    ui.colored_label(Color32::GRAY, "No active session");
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("⚙ Settings").clicked() {
                        // Open debug settings
                    }
                    
                    if self.active_session.is_some() {
                        if ui.button("⏸ Pause").clicked() {
                            if let Some(session) = &mut self.active_session {
                                session.state = ExecutionState::Paused;
                            }
                        }
                        if ui.button("⏹ Stop").clicked() {
                            self.stop_session();
                        }
                    } else {
                        if ui.button("▶ Start").clicked() {
                            let _ = self.start_session(
                                "demo_target".to_string(),
                                SessionConfig::default()
                            );
                        }
                    }
                });
            });
        });
    }

    /// Render variables panel
    fn render_variables_panel(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.heading("Variable Watchers");
            
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut String::new()); // Expression input
                if ui.button("Add Watch").clicked() {
                    // Add new watcher
                }
            });

            ui.separator();

            egui::ScrollArea::vertical()
                .max_height(300.0)
                .show(ui, |ui| {
                    for (i, watcher) in self.watchers.iter().enumerate() {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut true, "");
                                ui.label(&watcher.display_name);
                                
                                if let Some(value) = &watcher.current_value {
                                    ui.separator();
                                    ui.label(&value.raw_value);
                                    ui.label(format!("({})", value.type_info.name));
                                }

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.small_button("🗑").clicked() {
                                        // Remove watcher
                                    }
                                    if ui.small_button("📊").clicked() {
                                        // Show visualization
                                    }
                                });
                            });

                            // Show value history as mini-graph
                            if !watcher.value_history.is_empty() {
                                self.render_value_history(ui, watcher);
                            }
                        });
                    }
                });
        });
    }

    /// Render value history as a mini graph
    fn render_value_history(&self, ui: &mut Ui, watcher: &VariableWatcher) {
        let available_size = ui.available_size();
        let graph_size = Vec2::new(available_size.x.min(200.0), 40.0);
        
        let (rect, _) = ui.allocate_exact_size(graph_size, Sense::hover());
        
        if watcher.value_history.len() < 2 {
            return;
        }

        let painter = ui.painter();
        let stroke = Stroke::new(2.0, Color32::from_rgb(0, 150, 255));

        // Draw simple line graph of value changes
        let points: Vec<Pos2> = watcher.value_history
            .iter()
            .enumerate()
            .map(|(i, snapshot)| {
                let x = rect.min.x + (i as f32 / (watcher.value_history.len() - 1) as f32) * rect.width();
                let y = rect.center().y; // Simplified - would parse numeric values in real implementation
                Pos2::new(x, y)
            })
            .collect();

        if points.len() > 1 {
            painter.line_segment([points[0], points[1]], stroke);
        }
    }

    /// Get debug statistics
    pub fn get_statistics(&self) -> &DebugMetrics {
        &self.metrics
    }

    /// Update debug settings
    pub fn update_settings(&mut self, settings: DebugSettings) {
        self.settings = settings;
    }
}

impl BreakpointManager {
    fn new() -> Self {
        Self {
            breakpoints: HashMap::new(),
            conditional_breakpoints: Vec::new(),
            log_points: Vec::new(),
            statistics: BreakpointStatistics {
                total_breakpoints: 0,
                active_breakpoints: 0,
                total_hits: 0,
                most_hit_breakpoint: None,
                average_hits_per_breakpoint: 0.0,
            },
        }
    }
}

impl MemoryInspector {
    fn new() -> Self {
        Self {
            regions: Vec::new(),
            view_settings: MemoryViewSettings {
                bytes_per_row: 16,
                display_format: MemoryDisplayFormat::Hex,
                show_ascii: true,
                highlight_changes: true,
                address_format: AddressFormat::Hex,
            },
            search_results: Vec::new(),
            bookmarks: Vec::new(),
        }
    }
}

impl CallStackViewer {
    fn new() -> Self {
        Self {
            frames: Vec::new(),
            selected_frame: 0,
            navigation_history: Vec::new(),
            display_settings: CallStackDisplaySettings {
                show_parameters: true,
                show_local_variables: true,
                show_addresses: false,
                max_frames: 50,
                filter_system_frames: true,
            },
        }
    }
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            auto_watch: true,
            update_frequency: 100,
            max_history: 1000,
            memory_visualization: false,
            show_advanced_structures: true,
        }
    }
}

impl Default for DebugMetrics {
    fn default() -> Self {
        Self {
            total_sessions: 0,
            total_breakpoints_hit: 0,
            total_variables_watched: 0,
            average_session_duration: 0.0,
            most_watched_variables: HashMap::new(),
            performance_impact: 0.0,
        }
    }
}

// Built-in visualizers
pub struct TextVisualizer {
    settings: HashMap<String, String>,
}

impl TextVisualizer {
    fn new() -> Self {
        Self {
            settings: HashMap::new(),
        }
    }
}

impl DebugVisualizer for TextVisualizer {
    fn name(&self) -> &str {
        "Text"
    }

    fn can_visualize(&self, _value: &DebugValue) -> bool {
        true // Can visualize any value as text
    }

    fn render(&mut self, ui: &mut Ui, value: &DebugValue) -> egui::Response {
        ui.label(RichText::new(&value.raw_value)
            .color(if value.changed { Color32::YELLOW } else { Color32::WHITE }))
    }

    fn get_settings(&self) -> Vec<VisualizerSetting> {
        vec![
            VisualizerSetting {
                name: "font_size".to_string(),
                display_name: "Font Size".to_string(),
                setting_type: SettingType::Number,
                current_value: "12".to_string(),
                possible_values: Vec::new(),
            }
        ]
    }

    fn set_setting(&mut self, name: &str, value: &str) -> bool {
        self.settings.insert(name.to_string(), value.to_string());
        true
    }
}

pub struct GraphVisualizer {
    settings: HashMap<String, String>,
}

impl GraphVisualizer {
    fn new() -> Self {
        Self {
            settings: HashMap::new(),
        }
    }
}

impl DebugVisualizer for GraphVisualizer {
    fn name(&self) -> &str {
        "Graph"
    }

    fn can_visualize(&self, value: &DebugValue) -> bool {
        matches!(value.type_info.kind, TypeKind::Array | TypeKind::Vector)
    }

    fn render(&mut self, ui: &mut Ui, value: &DebugValue) -> egui::Response {
        let size = Vec2::new(200.0, 100.0);
        let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
        
        ui.painter().rect_stroke(rect, 0.0, Stroke::new(1.0, Color32::GRAY));
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            format!("Graph: {}", value.raw_value),
            FontId::default(),
            Color32::WHITE
        );

        response
    }

    fn get_settings(&self) -> Vec<VisualizerSetting> {
        Vec::new()
    }

    fn set_setting(&mut self, name: &str, value: &str) -> bool {
        self.settings.insert(name.to_string(), value.to_string());
        true
    }
}

pub struct TreeVisualizer {
    settings: HashMap<String, String>,
}

impl TreeVisualizer {
    fn new() -> Self {
        Self {
            settings: HashMap::new(),
        }
    }
}

impl DebugVisualizer for TreeVisualizer {
    fn name(&self) -> &str {
        "Tree"
    }

    fn can_visualize(&self, value: &DebugValue) -> bool {
        !value.children.is_empty()
    }

    fn render(&mut self, ui: &mut Ui, value: &DebugValue) -> egui::Response {
        ui.label(format!("🌳 Tree: {} children", value.children.len()))
    }

    fn get_settings(&self) -> Vec<VisualizerSetting> {
        Vec::new()
    }

    fn set_setting(&mut self, name: &str, value: &str) -> bool {
        self.settings.insert(name.to_string(), value.to_string());
        true
    }
}

pub struct MemoryVisualizer {
    settings: HashMap<String, String>,
}

impl MemoryVisualizer {
    fn new() -> Self {
        Self {
            settings: HashMap::new(),
        }
    }
}

impl DebugVisualizer for MemoryVisualizer {
    fn name(&self) -> &str {
        "Memory"
    }

    fn can_visualize(&self, value: &DebugValue) -> bool {
        value.memory_address.is_some()
    }

    fn render(&mut self, ui: &mut Ui, value: &DebugValue) -> egui::Response {
        if let Some(addr) = value.memory_address {
            ui.label(format!("📍 Memory: 0x{:x} ({} bytes)", addr, value.size))
        } else {
            ui.label("Memory: No address available")
        }
    }

    fn get_settings(&self) -> Vec<VisualizerSetting> {
        Vec::new()
    }

    fn set_setting(&mut self, name: &str, value: &str) -> bool {
        self.settings.insert(name.to_string(), value.to_string());
        true
    }
}