//! Integration tests for key endpoints.
//!
//! Tests key API methods against the real Torn API.
//! All tests skip gracefully when TORN_API_KEY is not set.

mod common;

#[tokio::test]
async fn live_key_info() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_key_info");
        return;
    };

    let result = client.key().info().await;
    assert!(result.is_ok(), "key.info() failed: {:?}", result.err());

    let data = result.unwrap();
    // Key info should have access level
    assert!(data.access_level.is_some(), "key.info() missing access_level");
}
