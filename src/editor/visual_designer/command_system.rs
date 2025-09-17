//! Enhanced Command System for Visual Designer Undo/Redo
//!
//! This module implements a comprehensive command pattern for the visual designer,
//! providing sophisticated undo/redo capabilities with command composition,
//! macro commands, and integration with the IDE event system.

use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::mpsc::Sender;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::core::event_bus::{IdeEvent, global_event_bus};
use crate::rcl::ui::component::Component;
use super::layout::LayoutManager;

/// Core Command trait that all operations must implement
pub trait Command {
    /// Execute the command
    fn execute(&mut self, context: &mut CommandContext) -> CommandResult;
    
    /// Undo the command (reverse the execute operation)
    fn undo(&mut self, context: &mut CommandContext) -> CommandResult;
    
    /// Get a human-readable description of the command
    fn description(&self) -> String;
    
    /// Get the command's unique identifier
    fn id(&self) -> Uuid;
    
    /// Check if this command can be merged with another command
    /// This is useful for combining multiple similar operations (e.g., continuous resizing)
    fn can_merge_with(&self, other: &dyn Command) -> bool {
        false // Default: no merging
    }
    
    /// Merge this command with another compatible command
    fn merge_with(&mut self, other: Box<dyn Command>) -> Result<(), Box<dyn Command>> {
        Err(other) // Default: cannot merge
    }
    
    /// Check if the command is still valid (e.g., target component still exists)
    fn is_valid(&self, context: &CommandContext) -> bool {
        true // Default: always valid
    }
    
    /// Get command metadata for history display and filtering
    fn metadata(&self) -> CommandMetadata {
        CommandMetadata::default()
    }
    
    /// Downcast to Any for type checking and conversion
    fn as_any(&self) -> &dyn std::any::Any;
    
    /// Convert Box<dyn Command> to Box<dyn Any>
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any>;
}

/// Result of command execution
#[derive(Debug, Clone)]
pub enum CommandResult {
    /// Command executed successfully
    Success,
    /// Command executed with warnings
    Warning(String),
    /// Command failed with error
    Error(String),
    /// Command was cancelled
    Cancelled,
}

/// Command execution context containing all necessary state
pub struct CommandContext {
    /// Layout manager for positions and sizes
    pub layout: LayoutManager,
    /// Component vector
    pub components: Vec<Box<dyn Component>>,
    /// Current selection
    pub selection: std::collections::HashSet<usize>,
    /// Primary selected component
    pub primary_selection: Option<usize>,
    /// Event sender for notifications
    pub event_sender: Option<Sender<IdeEvent>>,
    /// Command execution timestamp
    pub timestamp: Instant,
    /// Additional context data
    pub metadata: HashMap<String, String>,
}

impl CommandContext {
    pub fn new(
        layout: LayoutManager,
        components: Vec<Box<dyn Component>>,
        selection: std::collections::HashSet<usize>,
        primary_selection: Option<usize>,
    ) -> Self {
        Self {
            layout,
            components,
            selection,
            primary_selection,
            event_sender: None,
            timestamp: Instant::now(),
            metadata: HashMap::new(),
        }
    }
    
    /// Send an IDE event if event sender is available
    pub fn emit_event(&self, event: IdeEvent) {
        if let Some(sender) = &self.event_sender {
            let _ = sender.send(event);
        } else {
            global_event_bus().publish(event);
        }
    }
    
    /// Get component by index safely
    pub fn get_component(&self, index: usize) -> Option<&Box<dyn Component>> {
        self.components.get(index)
    }
    
    /// Get mutable component by index safely
    pub fn get_component_mut(&mut self, index: usize) -> Option<&mut Box<dyn Component>> {
        self.components.get_mut(index)
    }
    
    /// Check if a component index is valid
    pub fn is_valid_component(&self, index: usize) -> bool {
        index < self.components.len()
    }
}

/// Command metadata for history and filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandMetadata {
    /// Command category (e.g., "Layout", "Properties", "Selection")
    pub category: String,
    /// Affected component indices
    pub affected_components: Vec<usize>,
    /// Command priority (higher priority commands are harder to merge/replace)
    pub priority: u32,
    /// Whether the command should be logged for debugging
    pub should_log: bool,
    /// Custom tags for filtering and grouping
    pub tags: Vec<String>,
}

impl Default for CommandMetadata {
    fn default() -> Self {
        Self {
            category: "General".to_string(),
            affected_components: Vec::new(),
            priority: 0,
            should_log: false,
            tags: Vec::new(),
        }
    }
}

/// Macro command that combines multiple commands into a single operation
pub struct MacroCommand {
    id: Uuid,
    commands: Vec<Box<dyn Command>>,
    description: String,
    metadata: CommandMetadata,
}

impl MacroCommand {
    pub fn new(description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            commands: Vec::new(),
            description,
            metadata: CommandMetadata {
                category: "Composite".to_string(),
                should_log: true,
                ..Default::default()
            },
        }
    }
    
    pub fn add_command(&mut self, command: Box<dyn Command>) {
        // Update metadata to include all affected components
        let command_metadata = command.metadata();
        self.metadata.affected_components.extend(command_metadata.affected_components);
        self.commands.push(command);
    }
    
    pub fn len(&self) -> usize {
        self.commands.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
}

impl Command for MacroCommand {
    fn execute(&mut self, context: &mut CommandContext) -> CommandResult {
        let mut results = Vec::new();
        
        for command in &mut self.commands {
            let result = command.execute(context);
            match result {
                CommandResult::Error(ref msg) => {
                    // If any command fails, we need to undo the already executed ones
                    self.undo_partial(context, results.len());
                    return CommandResult::Error(format!("Macro command failed: {}", msg));
                }
                _ => results.push(result),
            }
        }
        
        context.emit_event(IdeEvent::CommandExecuted { 
            command_id: self.id,
            description: self.description.clone()
        });
        
        // Return Success if all succeeded, Warning if any had warnings
        if results.iter().any(|r| matches!(r, CommandResult::Warning(_))) {
            CommandResult::Warning("Some operations completed with warnings".to_string())
        } else {
            CommandResult::Success
        }
    }
    
    fn undo(&mut self, context: &mut CommandContext) -> CommandResult {
        // Undo in reverse order
        for command in self.commands.iter_mut().rev() {
            if let CommandResult::Error(msg) = command.undo(context) {
                return CommandResult::Error(format!("Failed to undo macro command: {}", msg));
            }
        }
        
        context.emit_event(IdeEvent::CommandUndone { 
            command_id: self.id,
            description: self.description.clone()
        });
        
        CommandResult::Success
    }
    
    fn description(&self) -> String {
        if self.commands.len() == 1 {
            self.commands[0].description()
        } else {
            format!("{} ({} operations)", self.description, self.commands.len())
        }
    }
    
    fn id(&self) -> Uuid {
        self.id
    }
    
    fn is_valid(&self, context: &CommandContext) -> bool {
        self.commands.iter().all(|cmd| cmd.is_valid(context))
    }
    
    fn metadata(&self) -> CommandMetadata {
        self.metadata.clone()
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

impl MacroCommand {
    /// Undo the first `count` commands (used when macro execution fails partially)
    fn undo_partial(&mut self, context: &mut CommandContext, count: usize) {
        for command in self.commands.iter_mut().take(count).rev() {
            let _ = command.undo(context);
        }
    }
}

/// Enhanced command history with sophisticated management
pub struct CommandHistory {
    /// Stack of executed commands
    executed: Vec<Box<dyn Command>>,
    /// Stack of undone commands (for redo)
    undone: Vec<Box<dyn Command>>,
    /// Current batch of commands being built
    current_batch: Option<MacroCommand>,
    /// Maximum number of commands to keep in history
    max_history: usize,
    /// Batch timeout - auto-end batch after this duration
    batch_timeout: Duration,
    /// Last command execution time
    last_execution: Option<Instant>,
    /// Whether to enable command merging
    enable_merging: bool,
    /// Command statistics
    stats: CommandStats,
}

#[derive(Debug, Default, Clone)]
pub struct CommandStats {
    pub total_executed: usize,
    pub total_undone: usize,
    pub total_redone: usize,
    pub batches_created: usize,
    pub commands_merged: usize,
}

impl Default for CommandHistory {
    fn default() -> Self {
        Self {
            executed: Vec::new(),
            undone: Vec::new(),
            current_batch: None,
            max_history: 200,
            batch_timeout: Duration::from_secs(2),
            last_execution: None,
            enable_merging: true,
            stats: CommandStats::default(),
        }
    }
}

impl CommandHistory {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Execute a command and add it to history
    pub fn execute_command(&mut self, mut command: Box<dyn Command>, context: &mut CommandContext) -> CommandResult {
        // Clear redo stack when executing new command
        self.undone.clear();
        
        // Try to merge with the last command if possible
        if self.enable_merging && !self.executed.is_empty() {
            if let Some(last_command) = self.executed.last_mut() {
                if last_command.can_merge_with(command.as_ref()) {
                    match last_command.merge_with(command) {
                        Ok(()) => {
                            self.stats.commands_merged += 1;
                            context.emit_event(IdeEvent::CommandMerged { 
                                description: last_command.description() 
                            });
                            return CommandResult::Success;
                        }
                        Err(returned_command) => {
                            command = returned_command;
                        }
                    }
                }
            }
        }
        
        // Execute the command
        let result = command.execute(context);
        
        match result {
            CommandResult::Success | CommandResult::Warning(_) => {
                // Add to current batch or create new single-command batch
                if let Some(ref mut batch) = self.current_batch {
                    batch.add_command(command);
                    self.maybe_end_batch_on_timeout();
                } else {
                    self.executed.push(command);
                    self.stats.total_executed += 1;
                }
                
                // Maintain history size limit
                if self.executed.len() > self.max_history {
                    self.executed.remove(0);
                }
                
                self.last_execution = Some(Instant::now());
            }
            _ => {
                // Command failed, don't add to history
            }
        }
        
        result
    }
    
    /// Start a new command batch
    pub fn begin_batch(&mut self, description: String) {
        self.end_batch(); // End any existing batch
        self.current_batch = Some(MacroCommand::new(description));
    }
    
    /// End the current command batch
    pub fn end_batch(&mut self) {
        if let Some(batch) = self.current_batch.take() {
            if !batch.is_empty() {
                self.executed.push(Box::new(batch));
                self.stats.batches_created += 1;
                self.stats.total_executed += 1;
            }
        }
    }
    
    /// Check if we should auto-end the current batch
    fn maybe_end_batch_on_timeout(&mut self) {
        if let Some(ref batch) = self.current_batch {
            if let Some(last_exec) = self.last_execution {
                if last_exec.elapsed() > self.batch_timeout {
                    self.end_batch();
                }
            }
        }
    }
    
    /// Undo the last command
    pub fn undo(&mut self, context: &mut CommandContext) -> CommandResult {
        self.end_batch(); // End any current batch before undoing
        
        if let Some(mut command) = self.executed.pop() {
            let result = command.undo(context);
            
            match result {
                CommandResult::Success | CommandResult::Warning(_) => {
                    self.undone.push(command);
                    self.stats.total_undone += 1;
                    CommandResult::Success
                }
                CommandResult::Error(msg) => {
                    // Put the command back if undo failed
                    self.executed.push(command);
                    CommandResult::Error(msg)
                }
                CommandResult::Cancelled => CommandResult::Cancelled,
            }
        } else {
            CommandResult::Error("Nothing to undo".to_string())
        }
    }
    
    /// Redo the last undone command
    pub fn redo(&mut self, context: &mut CommandContext) -> CommandResult {
        self.end_batch(); // End any current batch before redoing
        
        if let Some(mut command) = self.undone.pop() {
            let result = command.execute(context);
            
            match result {
                CommandResult::Success | CommandResult::Warning(_) => {
                    self.executed.push(command);
                    self.stats.total_redone += 1;
                    CommandResult::Success
                }
                CommandResult::Error(msg) => {
                    // Put the command back if redo failed
                    self.undone.push(command);
                    CommandResult::Error(msg)
                }
                CommandResult::Cancelled => CommandResult::Cancelled,
            }
        } else {
            CommandResult::Error("Nothing to redo".to_string())
        }
    }
    
    /// Get the number of commands that can be undone
    pub fn undo_count(&self) -> usize {
        self.executed.len()
    }
    
    /// Get the number of commands that can be redone
    pub fn redo_count(&self) -> usize {
        self.undone.len()
    }
    
    /// Get description of the next command to undo
    pub fn next_undo_description(&self) -> Option<String> {
        self.executed.last().map(|cmd| cmd.description())
    }
    
    /// Get description of the next command to redo
    pub fn next_redo_description(&self) -> Option<String> {
        self.undone.last().map(|cmd| cmd.description())
    }
    
    /// Clear all history
    pub fn clear(&mut self) {
        self.executed.clear();
        self.undone.clear();
        self.current_batch = None;
        self.last_execution = None;
    }
    
    /// Get command statistics
    pub fn stats(&self) -> &CommandStats {
        &self.stats
    }
    
    /// Get history snapshot for debugging/inspection
    pub fn get_history_snapshot(&self) -> Vec<String> {
        self.executed.iter()
            .map(|cmd| format!("{} ({})", cmd.description(), cmd.id()))
            .collect()
    }
    
    /// Validate all commands in history (remove invalid ones)
    pub fn validate_history(&mut self, context: &CommandContext) {
        self.executed.retain(|cmd| cmd.is_valid(context));
        self.undone.retain(|cmd| cmd.is_valid(context));
    }
    
    /// Set whether command merging is enabled
    pub fn set_merging_enabled(&mut self, enabled: bool) {
        self.enable_merging = enabled;
    }
    
    /// Set batch timeout duration
    pub fn set_batch_timeout(&mut self, timeout: Duration) {
        self.batch_timeout = timeout;
    }
    
    /// Set maximum history size
    pub fn set_max_history(&mut self, max_size: usize) {
        self.max_history = max_size;
        
        // Trim history if needed
        while self.executed.len() > self.max_history {
            self.executed.remove(0);
        }
    }
}

/// Event extensions for command system integration
impl IdeEvent {
    /// Create a command executed event
    pub fn command_executed(command_id: Uuid, description: String) -> Self {
        IdeEvent::CommandExecuted { command_id, description }
    }
    
    /// Create a command undone event
    pub fn command_undone(command_id: Uuid, description: String) -> Self {
        IdeEvent::CommandUndone { command_id, description }
    }
    
    /// Create a command merged event
    pub fn command_merged(description: String) -> Self {
        IdeEvent::CommandMerged { description }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rcl::ui::basic::button::Button;
    
    #[derive(Debug)]
    struct MockCommand {
        id: Uuid,
        description: String,
        executed: bool,
        undone: bool,
    }
    
    impl MockCommand {
        fn new(description: &str) -> Self {
            Self {
                id: Uuid::new_v4(),
                description: description.to_string(),
                executed: false,
                undone: false,
            }
        }
    }
    
    impl Command for MockCommand {
        fn execute(&mut self, _context: &mut CommandContext) -> CommandResult {
            self.executed = true;
            self.undone = false;
            CommandResult::Success
        }
        
        fn undo(&mut self, _context: &mut CommandContext) -> CommandResult {
            self.undone = true;
            self.executed = false;
            CommandResult::Success
        }
        
        fn description(&self) -> String {
            self.description.clone()
        }
        
        fn id(&self) -> Uuid {
            self.id
        }
    }
    
    fn create_test_context() -> CommandContext {
        let layout = LayoutManager::new();
        let components: Vec<Box<dyn Component>> = vec![
            Box::new(Button::new("Test Button".to_string())),
        ];
        let selection = std::collections::HashSet::new();
        
        CommandContext::new(layout, components, selection, None)
    }
    
    #[test]
    fn test_command_execution() {
        let mut history = CommandHistory::new();
        let mut context = create_test_context();
        
        let command = Box::new(MockCommand::new("Test Command"));
        let result = history.execute_command(command, &mut context);
        
        assert!(matches!(result, CommandResult::Success));
        assert_eq!(history.undo_count(), 1);
        assert_eq!(history.redo_count(), 0);
    }
    
    #[test]
    fn test_undo_redo() {
        let mut history = CommandHistory::new();
        let mut context = create_test_context();
        
        let command = Box::new(MockCommand::new("Test Command"));
        history.execute_command(command, &mut context);
        
        // Test undo
        let undo_result = history.undo(&mut context);
        assert!(matches!(undo_result, CommandResult::Success));
        assert_eq!(history.undo_count(), 0);
        assert_eq!(history.redo_count(), 1);
        
        // Test redo
        let redo_result = history.redo(&mut context);
        assert!(matches!(redo_result, CommandResult::Success));
        assert_eq!(history.undo_count(), 1);
        assert_eq!(history.redo_count(), 0);
    }
    
    #[test]
    fn test_macro_command() {
        let mut macro_cmd = MacroCommand::new("Test Macro".to_string());
        macro_cmd.add_command(Box::new(MockCommand::new("Sub Command 1")));
        macro_cmd.add_command(Box::new(MockCommand::new("Sub Command 2")));
        
        let mut context = create_test_context();
        let result = macro_cmd.execute(&mut context);
        
        assert!(matches!(result, CommandResult::Success));
        assert_eq!(macro_cmd.len(), 2);
    }
    
    #[test]
    fn test_batch_operations() {
        let mut history = CommandHistory::new();
        let mut context = create_test_context();
        
        history.begin_batch("Test Batch".to_string());
        history.execute_command(Box::new(MockCommand::new("Batch Cmd 1")), &mut context);
        history.execute_command(Box::new(MockCommand::new("Batch Cmd 2")), &mut context);
        history.end_batch();
        
        assert_eq!(history.undo_count(), 1); // Should be one batch
        
        let undo_result = history.undo(&mut context);
        assert!(matches!(undo_result, CommandResult::Success));
        assert_eq!(history.undo_count(), 0);
        assert_eq!(history.redo_count(), 1);
    }
}