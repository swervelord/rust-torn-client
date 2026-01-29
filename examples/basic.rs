//! Basic usage of the Torn client.
//!
//! This example demonstrates the simplest way to use the Torn client:
//! - Creating a client with a single API key
//! - Making a basic API call
//! - Handling the response
//!
//! Run with:
//! ```bash
//! TORN_API_KEY=your_key cargo run --example basic
//! ```

use torn_client::TornClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = std::env::var("TORN_API_KEY")
        .expect("Set TORN_API_KEY environment variable");

    // Create a new client with a single API key
    let client = TornClient::new(api_key);

    println!("Fetching your basic user info...");

    // Fetch your own basic info
    let user = client.user().basic().await?;
    println!("User: {:#?}", user);

    println!("\nSuccess! The client is working correctly.");

    Ok(())
}
