//! Integration test for DnsClient
use ide_rs::rcl::network::dns_client::DnsClient;

#[tokio::test]
async fn test_dns_client_resolve() {
    let client = DnsClient::new();
    let result = client.resolve("example.com").await;
    assert!(result.is_ok());
    assert!(result.unwrap().contains("Resolved"));
}
