//! Integration tests for torn endpoints.
//!
//! Tests key Torn API methods against the real Torn API.
//! All tests skip gracefully when TORN_API_KEY is not set.

mod common;

#[tokio::test]
async fn live_torn_items() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_torn_items");
        return;
    };

    let result = client.torn().items().await;
    assert!(result.is_ok(), "torn.items() failed: {:?}", result.err());

    let _data = result.unwrap();
    // Just verify the call succeeded
}

#[tokio::test]
async fn live_torn_honors() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_torn_honors");
        return;
    };

    let result = client.torn().honors().await;
    assert!(result.is_ok(), "torn.honors() failed: {:?}", result.err());

    let _data = result.unwrap();
    // Just verify the call succeeded
}

#[tokio::test]
async fn live_torn_medals() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_torn_medals");
        return;
    };

    let result = client.torn().medals().await;
    assert!(result.is_ok(), "torn.medals() failed: {:?}", result.err());

    let _data = result.unwrap();
    // Just verify the call succeeded
}

#[tokio::test]
async fn live_torn_timestamp() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_torn_timestamp");
        return;
    };

    let result = client.torn().timestamp().await;
    assert!(result.is_ok(), "torn.timestamp() failed: {:?}", result.err());

    let data = result.unwrap();
    // Timestamp should be valid
    assert!(data.data.timestamp > 1700000000, "torn.timestamp() returned invalid timestamp");
}

#[tokio::test]
async fn live_torn_lookup() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_torn_lookup");
        return;
    };

    let result = client.torn().lookup().await;
    assert!(result.is_ok(), "torn.lookup() failed: {:?}", result.err());

    let _data = result.unwrap();
    // Just verify the call succeeded
}
