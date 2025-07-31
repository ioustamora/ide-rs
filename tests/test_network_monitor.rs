//! Integration test for NetworkMonitor
use ide_rs::rcl::network::network_monitor::NetworkMonitor;

#[test]
fn test_network_monitor_status() {
    let monitor = NetworkMonitor::new();
    let status = monitor.status();
    assert!(status.contains("Network status"));
}
