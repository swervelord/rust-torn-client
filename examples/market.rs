//! Market API queries.
//!
//! This example demonstrates:
//! - Fetching bazaar listings
//! - Querying item market data
//! - Looking up specific items by ID
//! - Using market parameters
//!
//! Run with:
//! ```bash
//! TORN_API_KEY=your_key cargo run --example market
//! ```

use torn_client::TornClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("TORN_API_KEY")
        .expect("Set TORN_API_KEY environment variable");

    let client = TornClient::new(api_key);

    println!("=== Market Example ===\n");

    // Example 1: Get bazaar listings
    println!("1. Fetching bazaar listings...");
    match client.market().bazaar(Default::default()).await {
        Ok(bazaar) => {
            println!("   Bazaar response: {:#?}", bazaar);
            println!("   Has next page: {}", bazaar.has_next());
        }
        Err(e) => {
            println!("   Error fetching bazaar: {}", e);
        }
    }
    println!();

    // Example 2: Get auction house listings
    println!("2. Fetching auction house listings...");
    match client.market().auction_house(Default::default()).await {
        Ok(auction) => {
            println!("   Auction house response: {:#?}", auction);
            println!("   Has next page: {}", auction.has_next());
        }
        Err(e) => {
            println!("   Error fetching auction house: {}", e);
        }
    }
    println!();

    // Example 3: Look up market lookup data
    println!("3. Fetching market lookup data...");
    match client.market().lookup(None).await {
        Ok(lookup) => {
            println!("   Market lookup: {:#?}", lookup);
        }
        Err(e) => {
            println!("   Error fetching market lookup: {}", e);
        }
    }
    println!();

    // Example 4: Get item market by item ID
    println!("4. Looking up specific item market data...");
    let item_id = 1; // Replace with a real item ID

    match client
        .market()
        .with_item_id(item_id)
        .item_market(Default::default())
        .await
    {
        Ok(item_market) => {
            println!("   Item market for ID {}: {:#?}", item_id, item_market);
        }
        Err(e) => {
            println!("   Error fetching item market: {}", e);
            println!("   (This might fail if the item ID doesn't exist)");
        }
    }
    println!();

    // Example 5: Get bazaar for specific item
    println!("5. Fetching bazaar for specific item...");
    match client
        .market()
        .with_item_id(item_id)
        .bazaar(None)
        .await
    {
        Ok(item_bazaar) => {
            println!("   Item bazaar: {:#?}", item_bazaar);
        }
        Err(e) => {
            println!("   Error fetching item bazaar: {}", e);
        }
    }
    println!();

    // Example 6: Get server timestamp
    println!("6. Getting server timestamp...");
    match client.market().timestamp(None).await {
        Ok(timestamp) => {
            println!("   Server timestamp: {:#?}", timestamp);
        }
        Err(e) => {
            println!("   Error fetching timestamp: {}", e);
        }
    }

    println!("\n=== Market Example Complete ===");

    Ok(())
}
