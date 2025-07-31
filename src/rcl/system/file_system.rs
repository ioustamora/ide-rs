//! File system component for RCL system

#[allow(dead_code)]
pub struct FileSystem {}

#[allow(dead_code)]
impl FileSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read_file(&self, _path: &str) -> Result<String, anyhow::Error> {
        Ok(String::new())
    }

    pub fn write_file(&self, _path: &str, _content: &str) -> Result<(), anyhow::Error> {
        Ok(())
    }
}
