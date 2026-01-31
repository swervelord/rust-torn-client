//! Shared test helpers for integration tests.
//!
//! This module provides utilities for creating test clients and managing
//! API keys from environment variables.

#![allow(dead_code)]

use torn_client::TornClient;

/// Get a single API key from the environment, or return None.
///
/// This is used for tests that require a real API key.
/// Tests should skip gracefully when this returns None.
///
/// # Environment Variables
///
/// - `TORN_API_KEY` - A single Torn API key
pub fn test_client() -> Option<TornClient> {
    std::env::var("TORN_API_KEY")
        .ok()
        .map(TornClient::new)
}

/// Get a client with multiple API keys from the environment, or fallback to single key.
///
/// This is used for tests that want to test key balancing behavior.
///
/// # Environment Variables
///
/// - `TORN_API_KEYS` - Comma-separated list of API keys (preferred)
/// - `TORN_API_KEY` - Single API key (fallback)
pub fn test_client_multi() -> Option<TornClient> {
    std::env::var("TORN_API_KEYS")
        .ok()
        .and_then(|csv| {
            let keys: Vec<String> = csv.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            if keys.is_empty() { None } else { Some(TornClient::with_keys(keys)) }
        })
        .or_else(test_client)
}

/// Check if API key is set in the environment.
///
/// Useful for conditional test execution.
pub fn has_api_key() -> bool {
    std::env::var("TORN_API_KEY").is_ok()
}

/// Print a skip message for tests that require an API key.
///
/// This provides consistent messaging across all integration tests.
pub fn skip_message(test_name: &str) {
    eprintln!("TORN_API_KEY not set — skipping {}", test_name);
}
