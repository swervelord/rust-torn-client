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

    let data = result.unwrap();
    // Basic validation: response should have a player_id
    assert!(data.player_id.is_some(), "user.basic() missing player_id");
}

#[tokio::test]
async fn live_user_bars() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_user_bars");
        return;
    };

    let result = client.user().bars().await;
    assert!(result.is_ok(), "user.bars() failed: {:?}", result.err());

    let data = result.unwrap();
    // Bars response should have the main bar fields
    assert!(data.energy.is_some() || data.nerve.is_some() || data.happy.is_some(),
        "user.bars() missing expected bar fields");
}

#[tokio::test]
async fn live_user_attacks() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_user_attacks");
        return;
    };

    let result = client.user().attacks().await;
    assert!(result.is_ok(), "user.attacks() failed: {:?}", result.err());

    let response = result.unwrap();
    // Verify it's a paginated response
    assert!(response.data.attacks.is_some(), "user.attacks() missing attacks data");
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

    let data = result.unwrap();
    // Profile should have basic player information
    assert!(data.player_id.is_some(), "user.profile() missing player_id");
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

    let data = result.unwrap();
    assert!(data.player_id.is_some(), "user(4).basic() missing player_id");
}
