//! Integration test for FtpClient
use ide_rs::rcl::network::ftp_client::FtpClient;

#[tokio::test]
async fn test_ftp_client_connect() {
    let client = FtpClient::new();
    let result = client.connect("ftp://localhost:21").await;
    assert!(result.is_ok());
}
