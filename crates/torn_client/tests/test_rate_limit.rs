//! Integration tests for rate limiting behavior.
//!
//! Tests that the rate limiter correctly tracks and manages API requests.
//! All tests skip gracefully when TORN_API_KEY is not set.

mod common;

use torn_client::{RateLimitMode, TornClient};

#[tokio::test]
async fn rate_limit_auto_delay_mode() {
    let Some(api_key) = std::env::var("TORN_API_KEY").ok() else {
        common::skip_message("rate_limit_auto_delay_mode");
        return;
    };

    // Create a client with AutoDelay mode (default)
    let client = TornClient::builder()
        .api_key(api_key)
        .rate_limit_mode(RateLimitMode::AutoDelay)
        .build()
        .unwrap();

    // Make a few requests to verify rate limiting is working
    for i in 0..5 {
        let result = client.user().timestamp().await;
        assert!(result.is_ok(), "Request {} failed: {:?}", i, result.err());
    }

    // All requests should succeed with AutoDelay mode
}

#[tokio::test]
async fn rate_limit_throw_on_limit_mode() {
    let Some(api_key) = std::env::var("TORN_API_KEY").ok() else {
        common::skip_message("rate_limit_throw_on_limit_mode");
        return;
    };

    // Create a client with ThrowOnLimit mode
    let client = TornClient::builder()
        .api_key(api_key)
        .rate_limit_mode(RateLimitMode::ThrowOnLimit)
        .build()
        .unwrap();

    // Make a few requests - should work fine under the limit
    for i in 0..5 {
        let result = client.user().timestamp().await;
        assert!(result.is_ok(), "Request {} failed: {:?}", i, result.err());
    }
}

#[tokio::test]
async fn rate_limit_ignore_mode() {
    let Some(api_key) = std::env::var("TORN_API_KEY").ok() else {
        common::skip_message("rate_limit_ignore_mode");
        return;
    };

    // Create a client with Ignore mode
    let client = TornClient::builder()
        .api_key(api_key)
        .rate_limit_mode(RateLimitMode::Ignore)
        .build()
        .unwrap();

    // Make a few requests - should always work with Ignore mode
    for i in 0..5 {
        let result = client.user().timestamp().await;
        assert!(result.is_ok(), "Request {} failed: {:?}", i, result.err());
    }
}

#[tokio::test]
async fn rate_limit_multi_key_balancing() {
    let Some(keys) = common::test_client_multi().map(|c| c.key_count()) else {
        common::skip_message("rate_limit_multi_key_balancing");
        return;
    };

    // If we have multiple keys, verify balancing works
    if keys > 1 {
        let client = common::test_client_multi().unwrap();

        // Make several requests and verify they succeed
        for i in 0..10 {
            let result = client.user().timestamp().await;
            assert!(result.is_ok(), "Request {} failed: {:?}", i, result.err());
        }
    } else {
        eprintln!("TORN_API_KEYS not set with multiple keys — skipping multi-key test");
    }
}
