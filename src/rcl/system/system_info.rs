//! System info component for RCL system

#[allow(dead_code)]
pub struct SystemInfo {}

#[allow(dead_code)]
impl SystemInfo {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_os(&self) -> String {
        // Placeholder: return mock OS info
        "Windows 10 (mocked)".to_string()
    }

    pub fn get_cpu(&self) -> String {
        // Placeholder: return mock CPU info
        "Intel Core i7 (mocked)".to_string()
    }
}
