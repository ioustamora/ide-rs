//! # Event Bus System
//!
//! Provides a decoupled communication mechanism for IDE components using an event-driven
//! architecture. Components can publish events and subscribe to events of interest without
//! direct coupling.
//!
//! This implementation follows the observer pattern and supports typed events with
//! efficient dispatch and subscription management.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Weak, Mutex};
use uuid::Uuid;

/// Unique identifier for event subscribers
pub type SubscriberId = Uuid;

/// Core event types that flow through the IDE
#[derive(Debug, Clone)]
pub enum IdeEvent {
    /// Project lifecycle events
    ProjectLoaded { path: std::path::PathBuf },
    ProjectClosed { path: std::path::PathBuf },
    
    /// File operations
    FileOpened { path: std::path::PathBuf, buffer_id: Option<Uuid> },
    FileSaved { path: std::path::PathBuf, buffer_id: Option<Uuid> },
    FileClosed { path: std::path::PathBuf, buffer_id: Option<Uuid> },
    FileModified { path: std::path::PathBuf, external: bool },
    
    /// Buffer and editor events
    BufferChanged { buffer_id: Uuid, version: u64 },
    BufferCreated { buffer_id: Uuid, path: Option<std::path::PathBuf> },
    BufferDestroyed { buffer_id: Uuid },
    
    /// Visual designer events
    ComponentAdded { id: Uuid, component_type: String, parent_id: Option<Uuid> },
    ComponentRemoved { id: Uuid, component_type: String },
    ComponentSelected { id: Uuid, multi_select: bool },
    ComponentDeselected { id: Uuid },
    ComponentPropertyChanged { id: Uuid, property: String, old_value: String, new_value: String },
    ComponentMoved { id: Uuid, old_parent: Option<Uuid>, new_parent: Option<Uuid> },
    
    /// Code generation events
    CodeGenerationRequested { reason: String, target_files: Vec<std::path::PathBuf> },
    CodeGenerationCompleted { files_updated: Vec<std::path::PathBuf>, success: bool },
    CodeGenerationFailed { error: String, target_files: Vec<std::path::PathBuf> },
    
    /// Language server and diagnostics
    DiagnosticsUpdated { path: std::path::PathBuf, diagnostics_count: usize },
    LanguageServerStarted { language: String, server_id: String },
    LanguageServerStopped { language: String, server_id: String },
    LanguageServerError { language: String, error: String },
    
    /// Terminal events
    TerminalCreated { terminal_id: Uuid, name: String },
    TerminalClosed { terminal_id: Uuid },
    TerminalOutput { terminal_id: Uuid, line: String, is_error: bool },
    
    /// Task and build system events
    TaskStarted { task_id: Uuid, name: String, command: String },
    TaskCompleted { task_id: Uuid, name: String, exit_code: i32, duration_ms: u64 },
    TaskFailed { task_id: Uuid, name: String, error: String },
    
    /// UI state events
    PanelOpened { panel_name: String },
    PanelClosed { panel_name: String },
    ThemeChanged { theme_name: String },
    SettingsChanged { category: String, key: String },
    
    /// General application events
    ApplicationStarted,
    ApplicationShutdown,
    WorkspaceIndexingStarted,
    WorkspaceIndexingCompleted { files_indexed: usize, duration_ms: u64 },
}

/// Trait for event handlers
/// 
/// Components implement this trait to handle specific event types.
/// The event bus will call the handle_event method when subscribed events occur.
pub trait EventHandler: Send + Sync {
    /// Handle an incoming event
    /// 
    /// Implementations should pattern match on the event type and handle
    /// events of interest. Unknown events should be ignored.
    fn handle_event(&self, event: &IdeEvent);
    
    /// Optional method to provide a descriptive name for debugging
    fn handler_name(&self) -> &'static str {
        "AnonymousHandler"
    }
}

/// Type-erased event handler for internal storage
type BoxedHandler = Box<dyn EventHandler>;

/// Event subscription information
#[derive(Debug)]
struct Subscription {
    handler: Weak<BoxedHandler>,
    subscriber_id: SubscriberId,
    event_filter: Option<EventFilter>,
}

/// Filter for events based on type or content
#[derive(Debug, Clone)]
pub enum EventFilter {
    /// Only events matching the specific type
    EventType(EventType),
    /// Events matching a path pattern (for file events)
    PathPattern(String),
    /// Events matching component type (for visual designer events)
    ComponentType(String),
    /// Custom filter function
    Custom(fn(&IdeEvent) -> bool),
}

/// Event type enumeration for filtering
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    Project,
    File,
    Buffer,
    Component,
    CodeGeneration,
    Diagnostics,
    LanguageServer,
    Terminal,
    Task,
    UI,
    Application,
}

impl EventType {
    /// Check if an event matches this type
    pub fn matches(&self, event: &IdeEvent) -> bool {
        match (self, event) {
            (EventType::Project, IdeEvent::ProjectLoaded { .. }) => true,
            (EventType::Project, IdeEvent::ProjectClosed { .. }) => true,
            
            (EventType::File, IdeEvent::FileOpened { .. }) => true,
            (EventType::File, IdeEvent::FileSaved { .. }) => true,
            (EventType::File, IdeEvent::FileClosed { .. }) => true,
            (EventType::File, IdeEvent::FileModified { .. }) => true,
            
            (EventType::Buffer, IdeEvent::BufferChanged { .. }) => true,
            (EventType::Buffer, IdeEvent::BufferCreated { .. }) => true,
            (EventType::Buffer, IdeEvent::BufferDestroyed { .. }) => true,
            
            (EventType::Component, IdeEvent::ComponentAdded { .. }) => true,
            (EventType::Component, IdeEvent::ComponentRemoved { .. }) => true,
            (EventType::Component, IdeEvent::ComponentSelected { .. }) => true,
            (EventType::Component, IdeEvent::ComponentDeselected { .. }) => true,
            (EventType::Component, IdeEvent::ComponentPropertyChanged { .. }) => true,
            (EventType::Component, IdeEvent::ComponentMoved { .. }) => true,
            
            (EventType::CodeGeneration, IdeEvent::CodeGenerationRequested { .. }) => true,
            (EventType::CodeGeneration, IdeEvent::CodeGenerationCompleted { .. }) => true,
            (EventType::CodeGeneration, IdeEvent::CodeGenerationFailed { .. }) => true,
            
            (EventType::Diagnostics, IdeEvent::DiagnosticsUpdated { .. }) => true,
            
            (EventType::LanguageServer, IdeEvent::LanguageServerStarted { .. }) => true,
            (EventType::LanguageServer, IdeEvent::LanguageServerStopped { .. }) => true,
            (EventType::LanguageServer, IdeEvent::LanguageServerError { .. }) => true,
            
            (EventType::Terminal, IdeEvent::TerminalCreated { .. }) => true,
            (EventType::Terminal, IdeEvent::TerminalClosed { .. }) => true,
            (EventType::Terminal, IdeEvent::TerminalOutput { .. }) => true,
            
            (EventType::Task, IdeEvent::TaskStarted { .. }) => true,
            (EventType::Task, IdeEvent::TaskCompleted { .. }) => true,
            (EventType::Task, IdeEvent::TaskFailed { .. }) => true,
            
            (EventType::UI, IdeEvent::PanelOpened { .. }) => true,
            (EventType::UI, IdeEvent::PanelClosed { .. }) => true,
            (EventType::UI, IdeEvent::ThemeChanged { .. }) => true,
            (EventType::UI, IdeEvent::SettingsChanged { .. }) => true,
            
            (EventType::Application, IdeEvent::ApplicationStarted) => true,
            (EventType::Application, IdeEvent::ApplicationShutdown) => true,
            (EventType::Application, IdeEvent::WorkspaceIndexingStarted) => true,
            (EventType::Application, IdeEvent::WorkspaceIndexingCompleted { .. }) => true,
            
            _ => false,
        }
    }
}

impl EventFilter {
    /// Check if an event passes this filter
    pub fn matches(&self, event: &IdeEvent) -> bool {
        match self {
            EventFilter::EventType(event_type) => event_type.matches(event),
            EventFilter::PathPattern(pattern) => {
                // Simple pattern matching for file paths
                match event {
                    IdeEvent::FileOpened { path, .. } |
                    IdeEvent::FileSaved { path, .. } |
                    IdeEvent::FileClosed { path, .. } |
                    IdeEvent::FileModified { path, .. } |
                    IdeEvent::DiagnosticsUpdated { path, .. } => {
                        path.to_string_lossy().contains(pattern)
                    }
                    _ => false,
                }
            }
            EventFilter::ComponentType(component_type) => {
                match event {
                    IdeEvent::ComponentAdded { component_type: ct, .. } |
                    IdeEvent::ComponentRemoved { component_type: ct, .. } => {
                        ct == component_type
                    }
                    _ => false,
                }
            }
            EventFilter::Custom(filter_fn) => filter_fn(event),
        }
    }
}

/// Main event bus implementation
/// 
/// The EventBus provides a centralized message passing system for the IDE.
/// Components can subscribe to events and publish events without tight coupling.
pub struct EventBus {
    subscriptions: Mutex<Vec<Subscription>>,
    event_history: Mutex<Vec<IdeEvent>>,
    max_history_size: usize,
}

impl EventBus {
    /// Create a new event bus
    pub fn new() -> Self {
        Self {
            subscriptions: Mutex::new(Vec::new()),
            event_history: Mutex::new(Vec::new()),
            max_history_size: 1000, // Keep last 1000 events for debugging
        }
    }
    
    /// Create a new event bus with custom history size
    pub fn with_history_size(max_history_size: usize) -> Self {
        Self {
            subscriptions: Mutex::new(Vec::new()),
            event_history: Mutex::new(Vec::new()),
            max_history_size,
        }
    }
    
    /// Subscribe to events with a handler
    /// 
    /// Returns a unique subscriber ID that can be used to unsubscribe later.
    /// The handler will be called for all events unless a filter is specified.
    pub fn subscribe<H>(&self, handler: H) -> SubscriberId
    where
        H: EventHandler + 'static,
    {
        self.subscribe_with_filter(handler, None)
    }
    
    /// Subscribe to events with a filter
    /// 
    /// Only events matching the filter will be delivered to the handler.
    pub fn subscribe_with_filter<H>(&self, handler: H, filter: Option<EventFilter>) -> SubscriberId
    where
        H: EventHandler + 'static,
    {
        let subscriber_id = Uuid::new_v4();
        let boxed_handler: Arc<BoxedHandler> = Arc::new(Box::new(handler));
        
        let subscription = Subscription {
            handler: Arc::downgrade(&boxed_handler),
            subscriber_id,
            event_filter: filter,
        };
        
        {
            let mut subscriptions = self.subscriptions.lock().unwrap();
            subscriptions.push(subscription);
        }
        
        crate::log_debug!("EventBus: Subscribed handler with ID: {}", subscriber_id);
        
        // Keep the Arc alive by storing it temporarily
        // In a real implementation, you'd want a better way to manage handler lifetimes
        std::mem::forget(boxed_handler);
        
        subscriber_id
    }
    
    /// Subscribe to specific event types
    pub fn subscribe_to_type<H>(&self, handler: H, event_type: EventType) -> SubscriberId
    where
        H: EventHandler + 'static,
    {
        self.subscribe_with_filter(handler, Some(EventFilter::EventType(event_type)))
    }
    
    /// Unsubscribe a handler using its subscriber ID
    pub fn unsubscribe(&self, subscriber_id: SubscriberId) -> bool {
        let mut subscriptions = self.subscriptions.lock().unwrap();
        let initial_len = subscriptions.len();
        subscriptions.retain(|sub| sub.subscriber_id != subscriber_id);
        let removed = subscriptions.len() < initial_len;
        
        if removed {
            crate::log_debug!("EventBus: Unsubscribed handler with ID: {}", subscriber_id);
        }
        
        removed
    }
    
    /// Publish an event to all subscribers
    /// 
    /// The event will be delivered to all subscribed handlers that match
    /// any specified filters. Dead handlers (where the weak reference is invalid)
    /// will be automatically cleaned up.
    pub fn publish(&self, event: IdeEvent) {
        crate::log_debug!("EventBus: Publishing event: {:?}", event);
        
        // Add to history first
        {
            let mut history = self.event_history.lock().unwrap();
            history.push(event.clone());
            
            // Keep history within bounds
            if history.len() > self.max_history_size {
                history.remove(0);
            }
        }
        
        // Dispatch to subscribers
        let mut dead_subscriptions = Vec::new();
        let mut active_handlers = Vec::new();
        
        {
            let subscriptions = self.subscriptions.lock().unwrap();
            for (index, subscription) in subscriptions.iter().enumerate() {
                if let Some(handler) = subscription.handler.upgrade() {
                    // Check if event passes filter
                    let passes_filter = subscription.event_filter.as_ref()
                        .map(|filter| filter.matches(&event))
                        .unwrap_or(true); // No filter means all events pass
                    
                    if passes_filter {
                        active_handlers.push(handler);
                    }
                } else {
                    dead_subscriptions.push(index);
                }
            }
        }
        
        // Deliver events to active handlers
        for handler in active_handlers {
            handler.handle_event(&event);
        }
        
        // Clean up dead subscriptions
        if !dead_subscriptions.is_empty() {
            let mut subscriptions = self.subscriptions.lock().unwrap();
            for &index in dead_subscriptions.iter().rev() {
                if index < subscriptions.len() {
                    subscriptions.remove(index);
                }
            }
            crate::log_debug!("EventBus: Cleaned up {} dead subscriptions", dead_subscriptions.len());
        }
    }
    
    /// Get the number of active subscriptions
    pub fn subscription_count(&self) -> usize {
        let subscriptions = self.subscriptions.lock().unwrap();
        subscriptions.len()
    }
    
    /// Get recent event history
    /// 
    /// Returns up to `max_events` of the most recent events.
    pub fn get_recent_events(&self, max_events: usize) -> Vec<IdeEvent> {
        let history = self.event_history.lock().unwrap();
        let start_index = history.len().saturating_sub(max_events);
        history[start_index..].to_vec()
    }
    
    /// Clear event history
    pub fn clear_history(&self) {
        let mut history = self.event_history.lock().unwrap();
        history.clear();
    }
    
    /// Get statistics about the event bus
    pub fn get_stats(&self) -> EventBusStats {
        let subscriptions = self.subscriptions.lock().unwrap();
        let history = self.event_history.lock().unwrap();
        
        EventBusStats {
            active_subscriptions: subscriptions.len(),
            events_in_history: history.len(),
            max_history_size: self.max_history_size,
        }
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about the event bus state
#[derive(Debug, Clone)]
pub struct EventBusStats {
    pub active_subscriptions: usize,
    pub events_in_history: usize,
    pub max_history_size: usize,
}

/// Global event bus instance
/// 
/// This provides a singleton event bus that can be accessed throughout the application.
/// Use this for application-wide events and communication.
static EVENT_BUS: std::sync::OnceLock<Arc<EventBus>> = std::sync::OnceLock::new();

/// Get the global event bus instance
pub fn global_event_bus() -> Arc<EventBus> {
    EVENT_BUS
        .get_or_init(|| Arc::new(EventBus::new()))
        .clone()
}

/// Convenience function to publish to the global event bus
pub fn publish_global(event: IdeEvent) {
    global_event_bus().publish(event);
}

/// Convenience function to subscribe to the global event bus
pub fn subscribe_global<H>(handler: H) -> SubscriberId
where
    H: EventHandler + 'static,
{
    global_event_bus().subscribe(handler)
}

/// Convenience function to subscribe to specific event types on the global event bus
pub fn subscribe_global_type<H>(handler: H, event_type: EventType) -> SubscriberId
where
    H: EventHandler + 'static,
{
    global_event_bus().subscribe_to_type(handler, event_type)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    struct TestHandler {
        name: String,
        call_count: AtomicUsize,
        last_event: Mutex<Option<IdeEvent>>,
    }
    
    impl TestHandler {
        fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                call_count: AtomicUsize::new(0),
                last_event: Mutex::new(None),
            }
        }
        
        fn get_call_count(&self) -> usize {
            self.call_count.load(Ordering::Relaxed)
        }
        
        fn get_last_event(&self) -> Option<IdeEvent> {
            let guard = self.last_event.lock().unwrap();
            guard.clone()
        }
    }
    
    impl EventHandler for TestHandler {
        fn handle_event(&self, event: &IdeEvent) {
            self.call_count.fetch_add(1, Ordering::Relaxed);
            let mut guard = self.last_event.lock().unwrap();
            *guard = Some(event.clone());
        }
        
        fn handler_name(&self) -> &'static str {
            // We can't return &self.name as it's not 'static, so we'll use a default
            "TestHandler"
        }
    }
    
    #[test]
    fn test_event_bus_basic_publish_subscribe() {
        let event_bus = EventBus::new();
        let handler = TestHandler::new("test_handler");
        
        let subscriber_id = event_bus.subscribe(handler);
        
        // Publish an event
        let event = IdeEvent::ApplicationStarted;
        event_bus.publish(event.clone());
        
        // Give a moment for async processing if any
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        // Check that subscription exists
        assert_eq!(event_bus.subscription_count(), 1);
        
        // Unsubscribe
        assert!(event_bus.unsubscribe(subscriber_id));
        assert_eq!(event_bus.subscription_count(), 0);
    }
    
    #[test]
    fn test_event_type_filtering() {
        let file_event = IdeEvent::FileOpened {
            path: std::path::PathBuf::from("test.rs"),
            buffer_id: Some(Uuid::new_v4()),
        };
        
        let project_event = IdeEvent::ProjectLoaded {
            path: std::path::PathBuf::from("project"),
        };
        
        // Test file event type matching
        assert!(EventType::File.matches(&file_event));
        assert!(!EventType::File.matches(&project_event));
        
        // Test project event type matching
        assert!(EventType::Project.matches(&project_event));
        assert!(!EventType::Project.matches(&file_event));
    }
    
    #[test]
    fn test_event_filter() {
        let filter = EventFilter::EventType(EventType::File);
        
        let file_event = IdeEvent::FileOpened {
            path: std::path::PathBuf::from("test.rs"),
            buffer_id: Some(Uuid::new_v4()),
        };
        
        let app_event = IdeEvent::ApplicationStarted;
        
        assert!(filter.matches(&file_event));
        assert!(!filter.matches(&app_event));
    }
    
    #[test]
    fn test_path_pattern_filter() {
        let filter = EventFilter::PathPattern("test".to_string());
        
        let matching_event = IdeEvent::FileOpened {
            path: std::path::PathBuf::from("test.rs"),
            buffer_id: Some(Uuid::new_v4()),
        };
        
        let non_matching_event = IdeEvent::FileOpened {
            path: std::path::PathBuf::from("main.rs"),
            buffer_id: Some(Uuid::new_v4()),
        };
        
        assert!(filter.matches(&matching_event));
        assert!(!filter.matches(&non_matching_event));
    }
    
    #[test]
    fn test_event_history() {
        let event_bus = EventBus::with_history_size(3);
        
        // Publish some events
        event_bus.publish(IdeEvent::ApplicationStarted);
        event_bus.publish(IdeEvent::ProjectLoaded {
            path: std::path::PathBuf::from("test1"),
        });
        event_bus.publish(IdeEvent::ProjectLoaded {
            path: std::path::PathBuf::from("test2"),
        });
        event_bus.publish(IdeEvent::ProjectLoaded {
            path: std::path::PathBuf::from("test3"),
        });
        
        // History should be limited to max size
        let history = event_bus.get_recent_events(10);
        assert_eq!(history.len(), 3);
        
        // Should have the most recent events
        match &history[2] {
            IdeEvent::ProjectLoaded { path } => {
                assert_eq!(path, &std::path::PathBuf::from("test3"));
            }
            _ => panic!("Expected ProjectLoaded event"),
        }
    }
    
    #[test]
    fn test_global_event_bus() {
        let bus1 = global_event_bus();
        let bus2 = global_event_bus();
        
        // Should be the same instance
        assert!(Arc::ptr_eq(&bus1, &bus2));
    }
}