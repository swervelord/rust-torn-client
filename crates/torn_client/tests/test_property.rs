//! Integration tests for property endpoints.
//!
//! Tests key property API methods against the real Torn API.
//! All tests skip gracefully when TORN_API_KEY is not set.

mod common;

#[tokio::test]
async fn live_property_lookup() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_property_lookup");
        return;
    };

    let result = client.property().lookup().await;
    assert!(result.is_ok(), "property.lookup() failed: {:?}", result.err());

    let _data = result.unwrap();
    // Just verify the call succeeded
}

#[tokio::test]
async fn live_property_timestamp() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_property_timestamp");
        return;
    };

    let result = client.property().timestamp().await;
    assert!(result.is_ok(), "property.timestamp() failed: {:?}", result.err());

    let data = result.unwrap();
    // Timestamp should be valid
    assert!(data.timestamp > 1700000000, "property.timestamp() returned invalid timestamp");
}
