//! Clipboard component for RCL system

#[allow(dead_code)]
pub struct Clipboard {}

#[allow(dead_code)]
impl Clipboard {
    pub fn new() -> Self {
        Self {}
    }

    pub fn copy(&self, _text: &str) {
        // Placeholder: mock copy
    }

    pub fn paste(&self) -> String {
        // Placeholder: return mock clipboard content
        "Clipboard content (mocked)".to_string()
    }
}
