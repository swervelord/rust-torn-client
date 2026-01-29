//! Iterating through paginated results.
//!
//! This example demonstrates:
//! - Fetching paginated endpoints
//! - Checking if more pages are available
//! - Using .next() to fetch subsequent pages
//! - Using the page stream for automatic iteration
//! - Counting total results across all pages
//!
//! Run with:
//! ```bash
//! TORN_API_KEY=your_key cargo run --example pagination
//! ```

use torn_client::TornClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("TORN_API_KEY")
        .expect("Set TORN_API_KEY environment variable");

    let client = TornClient::builder()
        .api_key(api_key)
        .verbose(true)
        .build()?;

    println!("=== Pagination Example ===\n");

    // Example 1: Manual pagination with .next()
    println!("1. Manual pagination using .next():");
    println!("   Fetching your attack history...\n");

    let mut page = client.user().attacks().await?;
    let mut page_count = 1;
    let mut total_attacks = 0;

    loop {
        let attacks_count = page.data.attacks.len();
        total_attacks += attacks_count;

        println!("   Page {}: {} attacks", page_count, attacks_count);

        // Show info about first attack on this page
        if let Some(first) = page.data.attacks.first() {
            println!("      First attack ID: {}", first.code);
        }

        // Check if there's a next page
        if page.has_next() {
            println!("      -> More pages available, fetching next...");
            match page.next().await? {
                Some(next_page) => {
                    page = next_page;
                    page_count += 1;
                }
                None => break,
            }
        } else {
            println!("      -> No more pages");
            break;
        }

        // Limit to 3 pages for demo purposes
        if page_count >= 3 {
            println!("      (Stopping after 3 pages for demo)");
            break;
        }
    }

    println!("\n   Total: {} attacks across {} page(s)\n", total_attacks, page_count);

    // Example 2: Automatic pagination with page stream
    println!("2. Automatic pagination using page stream:");
    println!("   Fetching your events...\n");

    let first_page = client.user().events().await?;
    let mut pages = first_page.pages();
    let mut page_num = 0;
    let mut total_events = 0;

    // Limit to 3 pages for demo
    while let Some(page_result) = pages.next_page().await {
        page_num += 1;
        if page_num > 3 {
            println!("   (Stopping after 3 pages for demo)");
            break;
        }

        let page = page_result?;
        let events_count = page.data.events.len();
        total_events += events_count;

        println!("   Page {}: {} events", page_num, events_count);

        if let Some(first) = page.data.events.first() {
            println!("      First event ID: {}", first.event);
        }
    }

    println!("\n   Total: {} events across {} page(s)\n", total_events, page_num);

    // Example 3: Checking pagination metadata
    println!("3. Pagination metadata:");
    let response = client.user().attacks().await?;

    println!("   Has next page: {}", response.has_next());
    println!("   Has previous page: {}", response.has_prev());

    if let Some(next_url) = response.next_url() {
        println!("   Next URL: {}", next_url);
    }

    if let Some(prev_url) = response.prev_url() {
        println!("   Previous URL: {}", prev_url);
    }

    println!("\n=== Pagination Complete ===");

    Ok(())
}
