//! Integration test for FloatingPanel
use ide_rs::rcl::ui::advanced::floating_panel::FloatingPanel;

#[test]
fn test_floating_panel_initialization() {
    let panel = FloatingPanel::new("Test Panel", "Panel Content");
    assert_eq!(panel.title, "Test Panel");
    assert_eq!(panel.content, "Panel Content");
    assert!(panel.open);
}
