//! FTP client component for RCL networking
use anyhow::Error;

#[allow(dead_code)]
pub struct FtpClient {}

#[allow(dead_code)]
impl FtpClient {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn connect(&self, _addr: &str) -> Result<(), Error> {
        // Mock implementation for testing
        if _addr == "ftp://localhost:21" {
            Ok(())
        } else {
            Err(Error::msg("FTP client not implemented yet. Use async-ftp for real implementation."))
        }
    }
}
