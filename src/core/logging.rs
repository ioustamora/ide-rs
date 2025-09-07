//! # Logging Infrastructure
//!
//! Provides structured logging capabilities for the IDE using the tracing ecosystem.
//! Logging is feature-gated and can be enabled with the "logging" feature flag.

use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize the logging system
/// 
/// This should be called early in the application startup. It sets up structured
/// logging with environment-based filtering and formatting suitable for development.
/// 
/// # Features
/// 
/// This function is only available when the "logging" feature is enabled.
/// Without the feature, logging calls become no-ops.
#[cfg(feature = "logging")]
pub fn init_logging() {
    INIT.call_once(|| {
        use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
        
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "ide_rs=debug,info".into()),
            )
            .with(tracing_subscriber::fmt::layer().pretty())
            .init();
        
        tracing::info!("Logging initialized");
    });
}

/// Initialize the logging system (no-op when logging feature is disabled)
#[cfg(not(feature = "logging"))]
pub fn init_logging() {
    // No-op when logging is disabled
}

/// Convenience macro for logging debug information
/// 
/// This provides a consistent interface whether logging is enabled or not.
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        #[cfg(feature = "logging")]
        tracing::debug!($($arg)*);
    };
}

/// Convenience macro for logging informational messages
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        #[cfg(feature = "logging")]
        tracing::info!($($arg)*);
    };
}

/// Convenience macro for logging warnings
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        #[cfg(feature = "logging")]
        tracing::warn!($($arg)*);
    };
}

/// Convenience macro for logging errors
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        #[cfg(feature = "logging")]
        tracing::error!($($arg)*);
    };
}

/// Span creation helper for tracing operation lifecycle
#[macro_export]
macro_rules! trace_span {
    ($name:expr) => {
        #[cfg(feature = "logging")]
        {
            tracing::info_span!($name)
        }
        #[cfg(not(feature = "logging"))]
        {
            NoOpSpan
        }
    };
    ($name:expr, $($field:tt)*) => {
        #[cfg(feature = "logging")]
        {
            tracing::info_span!($name, $($field)*)
        }
        #[cfg(not(feature = "logging"))]
        {
            NoOpSpan
        }
    };
}

/// No-op span for when logging is disabled
#[cfg(not(feature = "logging"))]
pub struct NoOpSpan;

#[cfg(not(feature = "logging"))]
impl NoOpSpan {
    pub fn enter(&self) -> NoOpGuard {
        NoOpGuard
    }
}

#[cfg(not(feature = "logging"))]
pub struct NoOpGuard;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging_init_no_panic() {
        // Should not panic even when called multiple times
        init_logging();
        init_logging();
    }

    #[test]
    fn test_logging_macros_compile() {
        log_debug!("Debug message");
        log_info!("Info message");
        log_warn!("Warning message");
        log_error!("Error message");
        
        let _span = trace_span!("test_operation");
        let _span_with_field = trace_span!("test_operation", field = "value");
    }
}