//! HTTP client component for RCL networking
use anyhow::Error;

#[allow(dead_code)]
pub struct HttpClient {}

#[allow(dead_code)]
impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn get(&self, _url: &str) -> Result<String, Error> {
        Ok(format!("GET request: {}", _url))
    }
}
