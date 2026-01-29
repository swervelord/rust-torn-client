//! Multiple API keys with load balancing.
//!
//! This example demonstrates:
//! - Using the builder pattern to configure the client
//! - Adding multiple API keys for round-robin balancing
//! - Enabling verbose logging to see which keys are used
//! - Observing rate limit behavior
//!
//! Run with:
//! ```bash
//! TORN_API_KEY_1=key1 TORN_API_KEY_2=key2 TORN_API_KEY_3=key3 cargo run --example multi_key
//! ```
//!
//! Or with a single key:
//! ```bash
//! TORN_API_KEY=your_key cargo run --example multi_key
//! ```

use torn_client::{ApiKeyBalancing, TornClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Try to get multiple keys from environment
    let mut keys = Vec::new();

    // Try TORN_API_KEY_1, TORN_API_KEY_2, TORN_API_KEY_3
    for i in 1..=3 {
        if let Ok(key) = std::env::var(format!("TORN_API_KEY_{}", i)) {
            keys.push(key);
        }
    }

    // Fall back to single TORN_API_KEY if no numbered keys found
    if keys.is_empty() {
        if let Ok(key) = std::env::var("TORN_API_KEY") {
            keys.push(key);
        } else {
            eprintln!("Error: No API keys found!");
            eprintln!("Set TORN_API_KEY_1, TORN_API_KEY_2, TORN_API_KEY_3");
            eprintln!("Or set TORN_API_KEY for a single key");
            std::process::exit(1);
        }
    }

    println!("Building client with {} API key(s)...", keys.len());

    // Build client with custom configuration
    let client = TornClient::builder()
        .api_keys(keys)
        .api_key_balancing(ApiKeyBalancing::RoundRobin) // Explicitly set round-robin
        .comment("multi-key-example") // Add a comment to all requests
        .verbose(true) // Enable verbose logging
        .build()?;

    println!("\nClient configured with:");
    println!("  - {} API key(s)", client.key_count());
    println!("  - Round-robin key balancing");
    println!("  - Verbose logging enabled");
    println!("\nMaking multiple requests to demonstrate key rotation...\n");

    // Make several requests to see key rotation in action
    for i in 1..=5 {
        println!("--- Request {} ---", i);
        let user = client.user().basic().await?;
        println!("Got user: {} (ID: {})\n", user.name, user.player_id);

        // Small delay to make logs easier to read
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    println!("All requests completed successfully!");
    println!("\nWith multiple keys, the client automatically:");
    println!("  - Rotates between keys using round-robin");
    println!("  - Tracks rate limits per key (100 requests/min)");
    println!("  - Tracks global IP rate limit (1000 requests/min)");
    println!("  - Waits when rate limits are approached");

    Ok(())
}
