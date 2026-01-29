//! Integration tests for market endpoints.
//!
//! Tests key market API methods against the real Torn API.
//! All tests skip gracefully when TORN_API_KEY is not set.

mod common;

use torn_client::endpoints::market::BazaarParams;

#[tokio::test]
async fn live_market_bazaar() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_market_bazaar");
        return;
    };

    let params = BazaarParams::default();
    let result = client.market().bazaar(params).await;
    assert!(result.is_ok(), "market.bazaar() failed: {:?}", result.err());

    let data = result.unwrap();
    // Bazaar should return some data
    assert!(data.data.bazaar.is_some(), "market.bazaar() missing bazaar data");
}

#[tokio::test]
async fn live_market_lookup() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_market_lookup");
        return;
    };

    let result = client.market().lookup(None).await;
    assert!(result.is_ok(), "market.lookup() failed: {:?}", result.err());

    let data = result.unwrap();
    // Lookup should return available selections
    assert!(data.data.selections.is_some(), "market.lookup() missing selections");
}

#[tokio::test]
async fn live_market_timestamp() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_market_timestamp");
        return;
    };

    let result = client.market().timestamp(None).await;
    assert!(result.is_ok(), "market.timestamp() failed: {:?}", result.err());

    let data = result.unwrap();
    // Timestamp should be valid
    assert!(data.data.timestamp > 1700000000, "market.timestamp() returned invalid timestamp");
}
