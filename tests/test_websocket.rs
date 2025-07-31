//! Integration test for WebSocketClient
use ide_rs::rcl::network::websocket::WebSocketClient;

#[tokio::test]
async fn test_websocket_client_connect() {
    let client = WebSocketClient::new();
    let result = client.connect("ws://localhost:8080").await;
    assert!(result.is_ok());
}
