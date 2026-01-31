//! # torn_client
//!
//! Async Rust client for the [Torn API](https://www.torn.com/api.html).
//!
//! This is the **handwritten ergonomic layer** that wraps generated models
//! from [`torn_models`] and provides:
//!
//! ## Features
//!
//! - **Async-first** - Built on [tokio](https://tokio.rs) and [reqwest](https://docs.rs/reqwest) for high-performance async I/O
//! - **Type-safe endpoints** - Fully typed API methods with structured responses
//! - **Automatic rate limiting** - Respects Torn's rate limits (100/min per key, 1000/min per IP)
//! - **Multi-key support** - Round-robin or random balancing across multiple API keys
//! - **Pagination helpers** - Simple `.next()` / `.prev()` navigation and async page streaming
//! - **Comprehensive errors** - Typed error enum for precise error handling
//! - **ID-scoped lookups** - Ergonomic APIs for user/faction/item lookups by ID
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use torn_client::TornClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), torn_client::Error> {
//!     let client = TornClient::new("YOUR_API_KEY");
//!
//!     // Fetch your user info
//!     let user = client.user().basic().await?;
//!     println!("User: {} (Level {})", user.name, user.level);
//!
//!     // Get your current bars (energy, nerve, etc.)
//!     let bars = client.user().bars().await?;
//!     println!("Energy: {}/{}", bars.energy.current, bars.energy.maximum);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Multi-Key Configuration
//!
//! Use multiple API keys for higher throughput with automatic load balancing:
//!
//! ```rust,no_run
//! use torn_client::{TornClient, ApiKeyBalancing};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), torn_client::Error> {
//! let client = TornClient::builder()
//!     .api_key("KEY_1")
//!     .api_key("KEY_2")
//!     .api_key("KEY_3")
//!     .api_key_balancing(ApiKeyBalancing::RoundRobin)
//!     .comment("my-app")
//!     .verbose(true)
//!     .build()?;
//!
//! // The client will automatically rotate between keys
//! let user = client.user().basic().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Pagination
//!
//! Many endpoints return paginated results. Navigate pages with `.next()`:
//!
//! ```rust,no_run
//! # use torn_client::TornClient;
//! # async fn example(client: TornClient) -> Result<(), torn_client::Error> {
//! // Get first page
//! let mut page = client.user().attacks().await?;
//! println!("Page 1: {} attacks", page.data.attacks.len());
//!
//! // Fetch next page
//! if page.has_next() {
//!     if let Some(next) = page.next().await? {
//!         println!("Page 2: {} attacks", next.data.attacks.len());
//!     }
//! }
//!
//! // Or use the page stream
//! let first_page = client.user().attacks().await?;
//! let mut pages = first_page.pages();
//!
//! while let Some(page_result) = pages.next_page().await {
//!     let page = page_result?;
//!     // Process attacks...
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## ID-Scoped Lookups
//!
//! Look up data for specific users, factions, or items:
//!
//! ```rust,no_run
//! # use torn_client::TornClient;
//! # async fn example(client: TornClient) -> Result<(), torn_client::Error> {
//! // Look up another user by ID
//! let user = client.user().with_id(12345).basic().await?;
//!
//! // Look up another faction by ID
//! let faction = client.faction().with_id(5678).basic().await?;
//!
//! // Get item market data
//! let market = client.market().with_item_id(1).item_market(Default::default()).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Error Handling
//!
//! The client provides a comprehensive error enum for precise error handling:
//!
//! ```rust,no_run
//! # use torn_client::{TornClient, Error};
//! # async fn example(client: TornClient) -> Result<(), Box<dyn std::error::Error>> {
//! match client.user().basic().await {
//!     Ok(user) => println!("User: {}", user.name),
//!     Err(e) => match e {
//!         Error::Api { code, message } => {
//!             eprintln!("API error {}: {}", code, message);
//!         }
//!         Error::RateLimited => {
//!             eprintln!("Rate limit exceeded");
//!         }
//!         Error::Http(err) => {
//!             eprintln!("HTTP error: {}", err);
//!         }
//!         _ => {
//!             eprintln!("Other error: {}", e);
//!         }
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Examples
//!
//! See the [`examples/`](https://github.com/your-repo/rs-torn-client/tree/main/examples)
//! directory for complete working examples:
//!
//! - `basic.rs` - Minimal usage example
//! - `multi_key.rs` - Multiple API keys with balancing
//! - `pagination.rs` - Iterating through paginated results
//! - `faction.rs` - Faction lookups and ID-scoped queries
//! - `error_handling.rs` - Error handling patterns
//! - `market.rs` - Market API queries
//!
//! ## Rate Limiting
//!
//! The client automatically respects Torn's rate limits:
//! - **100 requests per minute per API key**
//! - **1000 requests per minute per IP address**
//!
//! When using multiple keys, the client tracks limits independently per key
//! and automatically waits when approaching limits.

// Lint suppressions for library code
#![allow(dead_code)]
#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::manual_strip)]

// Core modules
pub mod client;
pub mod config;
pub mod endpoints;
pub mod error;
pub mod http;
pub mod pagination;

// Internal modules (not public API)
pub(crate) mod key_pool;
pub(crate) mod rate_limit;

// Re-exports
pub use client::{TornClient, TornClientBuilder};
pub use config::{ApiKeyBalancing, RateLimitMode, TornClientConfig};
pub use error::Error;
pub use pagination::{PaginatedResponse, PaginationLinks, PaginationMetadata};

/// Re-export generated models for convenience.
pub use torn_models;
