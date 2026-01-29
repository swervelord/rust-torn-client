//! Faction API lookups.
//!
//! This example demonstrates:
//! - Getting your own faction information
//! - Looking up another faction by ID
//! - Accessing faction members
//! - Using ID-scoped faction endpoints
//!
//! Run with:
//! ```bash
//! TORN_API_KEY=your_key cargo run --example faction
//! ```

use torn_client::TornClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("TORN_API_KEY")
        .expect("Set TORN_API_KEY environment variable");

    let client = TornClient::new(api_key);

    println!("=== Faction Example ===\n");

    // Example 1: Get your own faction's basic info
    println!("1. Fetching your own faction's basic information...");
    let my_faction = client.faction().basic().await?;
    println!("   Faction: {:#?}\n", my_faction);

    // Example 2: Get your faction's members
    println!("2. Fetching your faction's members...");
    let members = client.faction().members().await?;
    let member_count = members.data.members.len();
    println!("   Found {} members", member_count);

    if let Some(first_member) = members.data.members.first() {
        println!("   First member: {:#?}", first_member);
    }
    println!();

    // Example 3: Look up another faction by ID
    // Using a well-known faction ID (you can change this)
    let faction_id = 1; // Replace with a real faction ID

    println!("3. Looking up faction with ID {}...", faction_id);
    match client.faction().with_id(faction_id).basic().await {
        Ok(other_faction) => {
            println!("   Faction found: {:#?}\n", other_faction);
        }
        Err(e) => {
            println!("   Error looking up faction: {}", e);
            println!("   (This is expected if the faction ID doesn't exist)\n");
        }
    }

    // Example 4: Get members of a specific faction by ID
    println!("4. Getting members of faction ID {}...", faction_id);
    match client.faction().with_id(faction_id).members().await {
        Ok(faction_members) => {
            let count = faction_members.data.members.len();
            println!("   Found {} members in faction {}", count, faction_id);

            if let Some(member) = faction_members.data.members.first() {
                println!("   First member: {:#?}", member);
            }
        }
        Err(e) => {
            println!("   Error: {}", e);
            println!("   (This is expected if the faction ID doesn't exist)");
        }
    }
    println!();

    // Example 5: Get your faction's chain status
    println!("5. Checking your faction's current chain...");
    match client.faction().chain().await {
        Ok(chain) => {
            println!("   Chain: {:#?}", chain);
        }
        Err(e) => {
            println!("   Error: {}", e);
            println!("   (This might fail if you're not in a faction or no active chain)");
        }
    }
    println!();

    // Example 6: Get faction attacks (paginated)
    println!("6. Fetching your faction's attack history...");
    match client.faction().attacks().await {
        Ok(attacks) => {
            let attack_count = attacks.data.attacks.len();
            println!("   Found {} attacks on first page", attack_count);
            println!("   Has more pages: {}", attacks.has_next());

            if let Some(attack) = attacks.data.attacks.first() {
                println!("   First attack: {:#?}", attack);
            }
        }
        Err(e) => {
            println!("   Error: {}", e);
        }
    }

    println!("\n=== Faction Example Complete ===");

    Ok(())
}
