//! Integration test for HttpClient
use ide_rs::rcl::network::http_client::HttpClient;

#[tokio::test]
async fn test_http_client_get() {
    let client = HttpClient::new();
    let result = client.get("http://example.com").await;
    assert!(result.is_ok());
    assert!(result.unwrap().contains("GET request"));
}
