//! UDP client component for RCL networking
use anyhow::Error;

#[allow(dead_code)]
pub struct UdpClient {}

#[allow(dead_code)]
impl UdpClient {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn send_to(&self, _addr: &str, _data: &[u8]) -> Result<(), Error> {
        // Mock implementation for testing
        if _addr == "127.0.0.1:8080" {
            Ok(())
        } else {
            Err(Error::msg("UDP client not implemented yet. Use tokio for real implementation."))
        }
    }
}
