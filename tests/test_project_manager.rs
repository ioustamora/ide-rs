//! Integration test for ProjectManager
use ide_rs::editor::project_manager::ProjectManager;
use std::path::PathBuf;

#[test]
fn test_create_project() {
    let manager = ProjectManager::new();
    let tmp_dir = std::env::temp_dir();
    let name = "test_ide_rs_project";
    let result = manager.create_project(&tmp_dir, name);
    assert!(result.is_ok(), "Project creation should succeed");
    // Optionally, check if the directory exists
    let project_path = tmp_dir.join(name);
    assert!(project_path.exists(), "Project directory should exist");
    // Cleanup
    std::fs::remove_dir_all(project_path).unwrap();
}
