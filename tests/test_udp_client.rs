//! Integration test for UdpClient
use ide_rs::rcl::network::udp_client::UdpClient;

#[tokio::test]
async fn test_udp_client_send_to() {
    let client = UdpClient::new();
    let result = client.send_to("127.0.0.1:8080", b"hello").await;
    assert!(result.is_ok());
}
