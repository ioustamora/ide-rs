//! # Network Communication Components
//!
//! This module provides comprehensive network communication capabilities for RCL
//! applications. It includes support for various protocols, connection management,
//! and network monitoring with async/await patterns for optimal performance.
//!
//! ## Protocol Support
//!
//! ### Web Protocols
//! - [`http_client`] - HTTP/HTTPS client with full REST API support
//! - [`websocket`] - WebSocket client for real-time bidirectional communication
//!
//! ### Transport Protocols
//! - [`tcp_client`] - TCP socket client for reliable stream connections
//! - [`udp_client`] - UDP socket client for lightweight datagram communication
//!
//! ### Application Protocols
//! - [`ftp_client`] - FTP client for file transfer operations
//! - [`dns_client`] - DNS resolution and lookup services
//!
//! ### Network Utilities
//! - [`network_monitor`] - Network connectivity monitoring and diagnostics
//!
//! ## Design Philosophy
//!
//! Network components are built with these principles:
//!
//! - **Async-First**: All network operations use async/await for non-blocking I/O
//! - **Error Resilience**: Comprehensive error handling with retry mechanisms
//! - **Security**: Built-in support for TLS/SSL and secure authentication
//! - **Performance**: Efficient connection pooling and resource management
//! - **Flexibility**: Configurable timeouts, headers, and connection parameters
//!
//! ## Connection Management
//!
//! All network components provide:
//! - Automatic connection pooling and reuse
//! - Configurable timeout and retry policies
//! - Graceful connection lifecycle management
//! - Resource cleanup and memory efficiency
//!
//! ## Security Features
//!
//! Network security is handled through:
//! - TLS/SSL encryption for secure protocols
//! - Certificate validation and trust management
//! - Authentication method support (Basic, Bearer, etc.)
//! - Input validation and sanitization
//!
//! ## Error Handling
//!
//! Network operations use structured error types that provide:
//! - Detailed error context and recovery suggestions
//! - Network-specific error categorization
//! - Retry-able vs. terminal error classification
//! - Debug information for troubleshooting
//!
//! # Examples
//!
//! ```ignore
//! use crate::rcl::network::{http_client::HttpClient, websocket::WebSocket};
//!
//! // HTTP client usage
//! let client = HttpClient::new();
//! let response = client.get("https://api.example.com/data").await?;
//! let json_data = response.json().await?;
//!
//! // WebSocket connection
//! let mut ws = WebSocket::connect("wss://api.example.com/ws").await?;
//! ws.send_text("Hello, Server!").await?;
//! let message = ws.receive().await?;
//! ```

/// HTTP/HTTPS client with comprehensive REST API support
/// 
/// Full-featured HTTP client supporting all standard methods, authentication,
/// custom headers, request/response body handling, and connection management.
pub mod http_client;

/// WebSocket client for real-time bidirectional communication
/// 
/// Async WebSocket implementation supporting text and binary messages,
/// connection lifecycle management, and automatic reconnection capabilities.
pub mod websocket;

/// TCP socket client for reliable stream-based communication
/// 
/// Low-level TCP client with connection management, configurable timeouts,
/// and efficient data streaming for custom protocol implementations.
pub mod tcp_client;

/// UDP socket client for lightweight datagram communication
/// 
/// UDP socket interface for connectionless communication with support
/// for broadcast, multicast, and peer-to-peer messaging patterns.
pub mod udp_client;

/// FTP client for file transfer protocol operations
/// 
/// Complete FTP client implementation supporting file upload/download,
/// directory operations, and both active and passive connection modes.
pub mod ftp_client;

/// Network connectivity monitoring and diagnostic services
/// 
/// Network health monitoring with connectivity testing, bandwidth measurement,
/// and network configuration detection for adaptive application behavior.
pub mod network_monitor;

/// DNS resolution and domain name lookup services
/// 
/// DNS client providing hostname resolution, reverse DNS lookups,
/// and comprehensive DNS record type support with caching capabilities.
pub mod dns_client;

// Re-export main client types for convenience
pub use dns_client::DnsClient;
pub use ftp_client::FtpClient;
pub use http_client::HttpClient;
pub use network_monitor::NetworkMonitor;
pub use tcp_client::TcpClient;
pub use udp_client::UdpClient;
pub use websocket::WebSocket;
