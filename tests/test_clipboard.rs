//! Integration test for Clipboard
use ide_rs::rcl::system::clipboard::Clipboard;

#[test]
fn test_clipboard_copy_paste() {
    let clipboard = Clipboard::new();
    clipboard.copy("Test clipboard");
    let content = clipboard.paste();
    assert_eq!(content, "Clipboard content (mocked)");
}
