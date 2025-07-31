//! Logic for auto-creating custom components from edits

#[allow(dead_code)]
pub struct CustomComponent {
    pub name: String,
    pub code: String,
}

#[allow(dead_code)]
impl CustomComponent {
    pub fn new(name: &str, code: &str) -> Self {
        Self {
            name: name.to_string(),
            code: code.to_string(),
        }
    }
    // Placeholder: logic to save to src/rcl/ui/custom/
}
