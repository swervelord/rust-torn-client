//! Legacy integration tests for the Torn client.
//!
//! These tests require real API credentials and are **skipped** when
//! the `TORN_API_KEY` environment variable is not set.
//!
//! # Environment Variables
//!
//! | Variable         | Required | Description                                  |
//! |------------------|----------|----------------------------------------------|
//! | `TORN_API_KEY`   | Yes      | A single Torn API key                        |
//! | `TORN_API_KEYS`  | No       | Comma-separated list of keys (for balancing) |
//!
//! See `TESTING.md` at the workspace root for full details.
//!
//! Note: Most integration tests have been moved to dedicated test files
//! (test_user.rs, test_faction.rs, etc.). This file contains basic scaffold
//! tests and spec validation.

mod common;

use torn_client::TornClient;

// ---------------------------------------------------------------------------
// Scaffold tests — these run even without API keys
// ---------------------------------------------------------------------------

#[test]
fn client_can_be_constructed_with_single_key() {
    let client = TornClient::new("dummy-key");
    assert_eq!(client.key_count(), 1);
}

#[test]
fn client_can_be_constructed_with_multiple_keys() {
    let client = TornClient::with_keys(vec![
        "key-a".into(),
        "key-b".into(),
    ]);
    assert_eq!(client.key_count(), 2);
}

// ---------------------------------------------------------------------------
// Live API tests — skipped without TORN_API_KEY
// ---------------------------------------------------------------------------

#[tokio::test]
async fn live_client_is_constructed() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_client_is_constructed");
        return;
    };
    assert_eq!(client.key_count(), 1);

    // Make a simple API call to verify the client works
    let result = client.user().timestamp().await;
    assert!(result.is_ok(), "timestamp() failed: {:?}", result.err());
}

#[tokio::test]
async fn live_client_multi_key() {
    let Some(client) = common::test_client_multi() else {
        common::skip_message("live_client_multi_key");
        return;
    };

    let key_count = client.key_count();
    assert!(key_count >= 1, "client should have at least one key");

    // Make a simple API call to verify the client works
    let result = client.user().timestamp().await;
    assert!(result.is_ok(), "timestamp() failed: {:?}", result.err());
}

// ---------------------------------------------------------------------------
// Spec validation (runs without API key, but needs spec file)
// ---------------------------------------------------------------------------

#[test]
fn spec_file_is_valid_json() {
    let spec_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../openapi/latest.json");
    let Ok(content) = std::fs::read_to_string(spec_path) else {
        eprintln!("openapi/latest.json not found — skipping spec validation");
        return;
    };
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&content);
    assert!(parsed.is_ok(), "openapi/latest.json is not valid JSON");

    let spec = parsed.unwrap();
    // Basic sanity: must have "paths" and "info"
    assert!(spec.get("paths").is_some(), "spec missing 'paths' key");
    assert!(spec.get("info").is_some(), "spec missing 'info' key");
}
