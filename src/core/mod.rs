//! # Core IDE Infrastructure
//!
//! This module provides the foundational components and services that power the IDE,
//! including event systems, logging, and cross-cutting concerns.

/// Logging infrastructure with feature gate
pub mod logging;

/// Event system for decoupled communication
pub mod event_bus;