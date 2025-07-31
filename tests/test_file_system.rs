//! Integration test for FileSystem
use ide_rs::rcl::system::file_system::FileSystem;

#[test]
fn test_file_system_read_write() {
    let fs = FileSystem::new();
    let path = "test_file.txt";
    let content = "Hello, world!";
    let write_result = fs.write_file(path, content);
    assert!(write_result.is_ok());
    let read_result = fs.read_file(path);
    assert!(read_result.is_ok());
    // Cleanup (mocked, so no actual file)
}
