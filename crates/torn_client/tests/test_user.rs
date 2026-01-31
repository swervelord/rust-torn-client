//! Integration tests for user endpoints.
//!
//! Tests key user API methods against the real Torn API.
//! All tests skip gracefully when TORN_API_KEY is not set.

mod common;

#[tokio::test]
async fn live_user_basic() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_user_basic");
        return;
    };

    let result = client.user().basic().await;
    assert!(result.is_ok(), "user.basic() failed: {:?}", result.err());

    let _data = result.unwrap();
    // Basic validation: just verify the call succeeded
}

#[tokio::test]
async fn live_user_bars() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_user_bars");
        return;
    };

    let result = client.user().bars().await;
    assert!(result.is_ok(), "user.bars() failed: {:?}", result.err());

    let _data = result.unwrap();
    // Just verify the call succeeded
}

#[tokio::test]
async fn live_user_attacks() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_user_attacks");
        return;
    };

    let result = client.user().attacks().await;
    assert!(result.is_ok(), "user.attacks() failed: {:?}", result.err());

    let _response = result.unwrap();
    // Just verify the call succeeded
}

#[tokio::test]
async fn live_user_timestamp() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_user_timestamp");
        return;
    };

    let result = client.user().timestamp().await;
    assert!(result.is_ok(), "user.timestamp() failed: {:?}", result.err());

    let data = result.unwrap();
    // Timestamp should be a recent Unix timestamp
    assert!(data.timestamp > 1700000000, "user.timestamp() returned invalid timestamp");
}

#[tokio::test]
async fn live_user_profile() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_user_profile");
        return;
    };

    let result = client.user().profile().await;
    assert!(result.is_ok(), "user.profile() failed: {:?}", result.err());

    let _data = result.unwrap();
    // Just verify the call succeeded
}

#[tokio::test]
async fn live_user_with_id_basic() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_user_with_id_basic");
        return;
    };

    // Test with a known user ID (ID 4 is Chedburn, a Torn staff member)
    let result = client.user().with_id(4).basic().await;
    assert!(result.is_ok(), "user(4).basic() failed: {:?}", result.err());

    let _data = result.unwrap();
    // Just verify the call succeeded
}
