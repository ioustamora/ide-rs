//! Integration test for TcpClient
use ide_rs::rcl::network::tcp_client::TcpClient;

#[tokio::test]
async fn test_tcp_client_connect() {
    let mut client = TcpClient::new();
    let result = client.connect("127.0.0.1:8080").await;
    assert!(result.is_ok());
}
