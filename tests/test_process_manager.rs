//! Integration test for ProcessManager
use ide_rs::rcl::system::process_manager::ProcessManager;

#[test]
fn test_process_manager_list_kill() {
    let pm = ProcessManager::new();
    let processes = pm.list_processes();
    assert!(!processes.is_empty());
    let result = pm.kill_process(1234);
    assert!(result.is_ok());
}
