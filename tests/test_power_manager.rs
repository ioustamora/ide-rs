//! Integration test for PowerManager
use ide_rs::rcl::system::power_manager::PowerManager;

#[test]
fn test_power_manager_battery_status() {
    let pm = PowerManager::new();
    let status = pm.get_battery_status();
    assert!(status.contains("Battery"));
}
