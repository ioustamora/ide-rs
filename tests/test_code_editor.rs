//! Integration test for CodeEditor
use ide_rs::editor::code_editor::CodeEditor;

#[test]
fn test_code_editor_initialization() {
    let editor = CodeEditor::new("rust");
    assert_eq!(editor.language, "rust");
    assert_eq!(editor.code, "");
}
