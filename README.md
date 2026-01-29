# rs-torn-client

A high-performance, async Rust client for the [Torn API](https://www.torn.com/api.html),
generated from the official OpenAPI spec.

> **Status: Production Ready** 🚀
> Complete implementation with 183 typed endpoints (100% API v2 coverage), 545 generated models, comprehensive tests, and examples.

## Architecture

This workspace contains two crates:

| Crate | Purpose | Code type |
|-------|---------|-----------|
| `torn_models` | Data types (structs, enums) for all API responses | **Generated** from OpenAPI spec |
| `torn_client` | Ergonomic async client with rate limiting and key balancing | Handwritten |

### Design Principles

- **Async-first** — Tokio + reqwest
- **100% regeneratable** — All models are generated from the latest Torn OpenAPI spec
- **Feature parity** — Matches the [TypeScript Torn client](https://github.com/neon0404/torn-client) feature-for-feature
- **CI-enforced** — Regeneration produces zero diff; integration tests run against the real API

## Quick Start

```rust
use torn_client::TornClient;

#[tokio::main]
async fn main() -> Result<(), torn_client::Error> {
    // Single key
    let client = TornClient::new("YOUR_API_KEY");

    // Fetch your user info
    let user = client.user().basic().await?;
    println!("User: {} (Level {})", user.name, user.level);

    Ok(())
}
```

## Usage

### Basic Client Construction

```rust
use torn_client::TornClient;

// Single API key
let client = TornClient::new("YOUR_API_KEY");

// Multiple keys with round-robin balancing
let client = TornClient::with_keys(vec![
    "KEY_1".to_string(),
    "KEY_2".to_string(),
    "KEY_3".to_string(),
]);
```

### Builder Pattern

For more control, use the builder pattern:

```rust
use torn_client::{TornClient, ApiKeyBalancing, RateLimitMode};

let client = TornClient::builder()
    .api_key("YOUR_API_KEY")
    .api_key("ANOTHER_API_KEY")
    .api_key_balancing(ApiKeyBalancing::RoundRobin)
    .rate_limit_mode(RateLimitMode::Auto)
    .comment("my-app")
    .verbose(true)
    .build()?;
```

### Making API Calls

The client provides typed endpoint methods organized by context:

```rust
// User endpoints (self-user) - 72 endpoints
let user = client.user().basic().await?;
let bars = client.user().bars().await?;
let attacks = client.user().attacks().await?;

// Faction endpoints (your faction) - 46 endpoints
let faction = client.faction().basic().await?;
let members = client.faction().members().await?;

// Torn endpoints (global data) - 30 endpoints
let items = client.torn().items().await?;
let honors = client.torn().honors().await?;

// Market endpoints - 11 endpoints
let bazaar = client.market().bazaar(Default::default()).await?;

// Racing endpoints - 9 endpoints
let cars = client.racing().cars().await?;
let race = client.racing().with_race_id(123).race().await?;

// Forum endpoints - 8 endpoints
let categories = client.forum().categories(None).await?;
let posts = client.forum().with_thread_id(456).posts(Default::default()).await?;

// Property endpoints - 4 endpoints
let lookup = client.property().lookup().await?;

// Key endpoints - 3 endpoints
let key_info = client.key().info().await?;
```

### ID-Scoped Lookups

Many endpoints support looking up data for specific IDs:

```rust
// Look up another user by ID
let other_user = client.user().with_id(12345).basic().await?;

// Look up another faction by ID
let other_faction = client.faction().with_id(5678).basic().await?;

// Get item market data by item ID
let item_market = client
    .market()
    .with_item_id(1)
    .item_market(Default::default())
    .await?;
```

### Pagination

Many endpoints return paginated results. Use `.next()` to fetch subsequent pages:

```rust
// Get first page of attacks
let mut page = client.user().attacks().await?;

println!("Page 1: {} attacks", page.data.attacks.len());

// Fetch next page
if page.has_next() {
    let next_page = page.next().await?;
    if let Some(page2) = next_page {
        println!("Page 2: {} attacks", page2.data.attacks.len());
    }
}

// Or use the page stream for automatic iteration
let first_page = client.user().attacks().await?;
let mut pages = first_page.pages();

while let Some(page_result) = pages.next_page().await {
    let page = page_result?;
    println!("Got {} attacks", page.data.attacks.len());
}
```

### Error Handling

The client uses a typed error enum for precise error handling:

```rust
use torn_client::Error;

match client.user().basic().await {
    Ok(user) => println!("User: {}", user.name),
    Err(e) => match e {
        Error::Api { code, message } => {
            eprintln!("API error {}: {}", code, message);
        }
        Error::RateLimited => {
            eprintln!("Rate limit exceeded, please wait");
        }
        Error::Http(err) => {
            eprintln!("HTTP error: {}", err);
        }
        Error::Json(err) => {
            eprintln!("JSON parsing error: {}", err);
        }
        Error::NoKeys => {
            eprintln!("No API keys configured");
        }
        Error::Request(msg) => {
            eprintln!("Request failed: {}", msg);
        }
    }
}
```

### Rate Limiting

The client automatically manages rate limits:
- **100 requests per minute per API key**
- **1000 requests per minute per IP address**

With multiple keys, the client:
- Rotates between keys using the configured balancing strategy (round-robin by default)
- Tracks per-key rate limits independently
- Automatically waits when limits are approached

### Examples

See the `examples/` directory for complete working examples:

- **`basic.rs`** - Minimal usage example
- **`multi_key.rs`** - Multiple API keys with balancing and verbose logging
- **`pagination.rs`** - Iterating through paginated results
- **`faction.rs`** - Faction lookups and ID-scoped queries
- **`error_handling.rs`** - Error handling patterns
- **`market.rs`** - Market API queries

Run an example:

```bash
TORN_API_KEY=your_key cargo run --example basic
```

## Foundational Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Async runtime | Tokio | De facto standard for async Rust |
| HTTP client | reqwest | Mature, async, widely used |
| Serialization | serde + serde_json | Universal Rust JSON support |
| Error handling | thiserror | Typed errors with derive macros |
| Key balancing default | Round-robin | Matches TS client behavior |
| Rate limits | 100/min per key, 1000/min per IP | Torn API documented limits |
| Generated code | `crates/torn_models/src/generated/` | Isolated, never hand-edited |

## Regeneration Pipeline

All generated code is produced by a single command:

```bash
python scripts/regen.py
```

This:
1. Fetches the latest OpenAPI spec from `https://www.torn.com/swagger/openapi.json`
2. Builds `openapi/spec_map.json` (tag → endpoint mapping)
3. Builds `openapi/pagination_map.json` (paginated endpoint metadata)
4. Generates Rust model code into `crates/torn_models/src/generated/`

CI enforces that running regen produces zero diff. See [GENERATED_POLICY.md](GENERATED_POLICY.md).

## Testing

```bash
# Unit tests (no API key needed)
cargo test --workspace

# Integration tests (requires API key)
TORN_API_KEY=your-key cargo test --workspace
```

See [TESTING.md](TESTING.md) for full details.

## Project Structure

```
rs-torn-client/
├── Cargo.toml                    # Workspace root
├── crates/
│   ├── torn_models/              # GENERATED models crate
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       └── generated/        # ← ALL generated code lives here
│   │           └── mod.rs
│   └── torn_client/              # Handwritten client crate
│       ├── Cargo.toml
│       ├── src/
│       │   ├── lib.rs            # TornClient struct + public API
│       │   └── error.rs          # Typed error enum
│       └── tests/
│           └── integration.rs    # Integration test harness
├── scripts/
│   ├── fetch_spec.py             # Download latest OpenAPI spec
│   └── regen.py                  # Full regeneration pipeline
├── openapi/
│   ├── latest.json               # Fetched spec (committed)
│   ├── spec_map.json             # Tag → endpoint mapping
│   └── pagination_map.json       # Paginated endpoint metadata
├── .github/workflows/
│   ├── regen_check.yml           # CI: regen produces no diff
│   └── tests.yml                 # CI: unit + integration + lint
├── GENERATED_POLICY.md           # Rules for generated vs handwritten code
├── TESTING.md                    # Test setup and env var guide
└── README.md                     # This file
```

## Features

- **Complete API Coverage**: All 183 Torn API v2 endpoints implemented across 8 endpoint groups
- **Type Safety**: 545 generated Rust types from the official OpenAPI spec
- **Async/Await**: Built on Tokio and reqwest for high-performance async operations
- **Rate Limiting**: Automatic rate limit management (100/min per key, 1000/min per IP)
- **Multi-Key Support**: Round-robin and random key balancing strategies
- **Pagination**: First-class support for paginated endpoints with `.next()`, `.prev()`, and streaming
- **Error Handling**: Comprehensive typed error enum for precise error handling
- **Well Tested**: 50+ unit tests and 31+ integration tests
- **Examples**: 6 complete example programs demonstrating all major features

## License

MIT
