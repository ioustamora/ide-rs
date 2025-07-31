//! Integration test for RustAnalyzerClient
use ide_rs::editor::rust_analyzer::RustAnalyzerClient;

#[test]
fn test_rust_analyzer_analyze() {
    let analyzer = RustAnalyzerClient::new();
    let diagnostics = analyzer.analyze("fn main() {}\n");
    assert_eq!(diagnostics, vec!["No errors (mocked)".to_string()]);
}
