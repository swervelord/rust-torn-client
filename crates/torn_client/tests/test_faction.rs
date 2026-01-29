//! Integration tests for faction endpoints.
//!
//! Tests key faction API methods against the real Torn API.
//! All tests skip gracefully when TORN_API_KEY is not set.

mod common;

#[tokio::test]
async fn live_faction_basic() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_faction_basic");
        return;
    };

    let result = client.faction().basic().await;

    // This might fail if the user is not in a faction, which is okay
    if let Err(e) = result {
        eprintln!("faction.basic() failed (user may not be in a faction): {:?}", e);
        return;
    }

    let data = result.unwrap();
    // If successful, should have faction data
    assert!(data.data.is_some(), "faction.basic() missing data");
}

#[tokio::test]
async fn live_faction_members() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_faction_members");
        return;
    };

    let result = client.faction().members().await;

    // This might fail if the user is not in a faction
    if let Err(e) = result {
        eprintln!("faction.members() failed (user may not be in a faction): {:?}", e);
        return;
    }

    let data = result.unwrap();
    assert!(data.data.is_some(), "faction.members() missing data");
}

#[tokio::test]
async fn live_faction_timestamp() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_faction_timestamp");
        return;
    };

    let result = client.faction().timestamp().await;

    // Timestamp should work even if not in a faction
    assert!(result.is_ok(), "faction.timestamp() failed: {:?}", result.err());

    let data = result.unwrap();
    assert!(data.data.is_some(), "faction.timestamp() missing data");
}

#[tokio::test]
async fn live_faction_with_id_basic() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_faction_with_id_basic");
        return;
    };

    // Test with a known faction ID (10000 is a well-known faction)
    let result = client.faction().with_id(10000).basic().await;
    assert!(result.is_ok(), "faction(10000).basic() failed: {:?}", result.err());

    let data = result.unwrap();
    assert!(data.data.is_some(), "faction(10000).basic() missing data");
}

#[tokio::test]
async fn live_faction_with_id_members() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_faction_with_id_members");
        return;
    };

    // Test with a known faction ID
    let result = client.faction().with_id(10000).members().await;
    assert!(result.is_ok(), "faction(10000).members() failed: {:?}", result.err());

    let data = result.unwrap();
    assert!(data.data.is_some(), "faction(10000).members() missing data");
}
