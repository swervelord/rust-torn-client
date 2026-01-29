//! Error handling patterns.
//!
//! This example demonstrates:
//! - Different error types (Api, RateLimited, Http, Json)
//! - Pattern matching on errors
//! - Handling API errors gracefully
//! - Dealing with rate limits
//!
//! Run with:
//! ```bash
//! TORN_API_KEY=your_key cargo run --example error_handling
//! ```

use torn_client::{Error, TornClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("TORN_API_KEY")
        .expect("Set TORN_API_KEY environment variable");

    let client = TornClient::new(api_key);

    println!("=== Error Handling Example ===\n");

    // Example 1: Successful request
    println!("1. Making a successful request...");
    match client.user().basic().await {
        Ok(user) => {
            println!("   Success! User: {} (ID: {})", user.name, user.player_id);
        }
        Err(e) => {
            println!("   Error: {}", e);
        }
    }
    println!();

    // Example 2: API error - invalid user ID
    println!("2. Attempting to fetch a non-existent user (will cause API error)...");
    let invalid_user_id = 999999999; // Very unlikely to exist

    match client.user().with_id(invalid_user_id).basic().await {
        Ok(user) => {
            println!("   Unexpectedly found user: {:?}", user);
        }
        Err(e) => {
            println!("   Caught error: {}", e);

            // Pattern match on specific error types
            match e {
                Error::Api { code, message } => {
                    println!("   -> This is a Torn API error");
                    println!("      Error code: {}", code);
                    println!("      Message: {}", message);

                    // Handle specific error codes
                    match code {
                        2 => println!("      (Code 2: Incorrect ID)"),
                        5 => println!("      (Code 5: Too many requests)"),
                        10 => println!("      (Code 10: Incorrect key)"),
                        _ => println!("      (Unknown error code)"),
                    }
                }
                Error::Http(http_err) => {
                    println!("   -> This is an HTTP transport error: {}", http_err);
                }
                Error::Json(json_err) => {
                    println!("   -> This is a JSON parsing error: {}", json_err);
                }
                Error::RateLimited => {
                    println!("   -> Rate limit exceeded!");
                }
                Error::NoKeys => {
                    println!("   -> No API keys configured!");
                }
                Error::Request(msg) => {
                    println!("   -> Request error: {}", msg);
                }
            }
        }
    }
    println!();

    // Example 3: Invalid API key error
    println!("3. Demonstrating invalid API key error...");
    let bad_client = TornClient::new("invalid-api-key-123");

    match bad_client.user().basic().await {
        Ok(_) => {
            println!("   Unexpected success with invalid key!");
        }
        Err(e) => {
            println!("   Caught error: {}", e);

            if let Error::Api { code, message } = &e {
                if *code == 10 {
                    println!("   -> As expected: Invalid API key (code 10)");
                    println!("   -> Message: {}", message);
                }
            }
        }
    }
    println!();

    // Example 4: Using Result combinators
    println!("4. Using Result combinators for cleaner error handling...");

    let user_result = client
        .user()
        .basic()
        .await
        .map(|user| {
            println!("   User {} has {} level", user.name, user.level);
            user
        })
        .map_err(|e| {
            eprintln!("   Failed to fetch user: {}", e);
            e
        });

    match user_result {
        Ok(_) => println!("   Request completed successfully"),
        Err(_) => println!("   Request failed"),
    }
    println!();

    // Example 5: Retry logic for transient errors
    println!("5. Implementing retry logic...");

    let max_retries = 3;
    let mut attempt = 0;

    loop {
        attempt += 1;
        println!("   Attempt {}/{}...", attempt, max_retries);

        match client.user().basic().await {
            Ok(user) => {
                println!("   Success on attempt {}! User: {}", attempt, user.name);
                break;
            }
            Err(e) => {
                println!("   Error: {}", e);

                // Retry on specific errors
                let should_retry = matches!(
                    e,
                    Error::Http(_) | Error::RateLimited | Error::Request(_)
                );

                if should_retry && attempt < max_retries {
                    println!("   -> Will retry after delay...");
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                } else {
                    println!("   -> Not retrying (max attempts reached or non-retryable error)");
                    break;
                }
            }
        }
    }
    println!();

    // Example 6: Error propagation with ?
    println!("6. Using ? operator for error propagation...");
    if let Err(e) = fetch_user_info(&client).await {
        println!("   Function returned error: {}", e);
    }

    println!("\n=== Error Handling Complete ===");

    Ok(())
}

// Helper function demonstrating error propagation
async fn fetch_user_info(client: &TornClient) -> Result<(), Error> {
    // The ? operator automatically propagates errors
    let user = client.user().basic().await?;
    println!("   Successfully fetched user: {}", user.name);

    let bars = client.user().bars().await?;
    println!("   User has {} energy", bars.energy.current);

    Ok(())
}
