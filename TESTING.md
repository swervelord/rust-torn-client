# Testing Guide

This document describes how to run tests for `rs-torn-client`.

## Overview

The test suite includes:

- **Unit tests** - Built into library modules, test internal logic without network access
- **Integration tests** - Validate against the real Torn API, require API key
- **Spec validation** - Verify OpenAPI specification structure

All integration tests **skip gracefully** when API credentials are not configured.

## Test Categories

### Unit Tests

Unit tests live alongside the code in `#[cfg(test)]` modules and do not require
any environment variables or network access.

```bash
cargo test --workspace
```

### Integration Tests

Integration tests live in `crates/torn_client/tests/` and hit the **real Torn API**.
They are organized by endpoint group and test real API behavior.

#### Test Organization

```
crates/torn_client/tests/
├── common/
│   └── mod.rs              # Shared test helpers
├── test_user.rs            # User endpoint tests (6 tests)
├── test_faction.rs         # Faction endpoint tests (5 tests)
├── test_market.rs          # Market endpoint tests (3 tests)
├── test_property.rs        # Property endpoint tests (2 tests)
├── test_torn.rs            # Torn endpoint tests (5 tests)
├── test_key.rs             # Key endpoint tests (1 test)
├── test_rate_limit.rs      # Rate limiting tests (4 tests)
├── test_pagination.rs      # Pagination tests (5 tests)
└── integration.rs          # Legacy integration tests
```

#### Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `TORN_API_KEY` | Yes (for live tests) | A single Torn API key |
| `TORN_API_KEYS` | No | Comma-separated list of API keys for multi-key / balancing tests |

#### Setting Environment Variables

**Windows (PowerShell)**
```powershell
$env:TORN_API_KEY="your-api-key-here"
$env:TORN_API_KEYS="key1,key2,key3"
```

**Windows (CMD)**
```cmd
set TORN_API_KEY=your-api-key-here
set TORN_API_KEYS=key1,key2,key3
```

**Linux/macOS (Bash)**
```bash
export TORN_API_KEY="your-api-key-here"
export TORN_API_KEYS="key1,key2,key3"
```

#### Running Integration Tests Locally

**Run all tests:**
```bash
cargo test --workspace
```

**Run only integration tests:**
```bash
cargo test --test '*' -p torn_client
```

**Run specific test file:**
```bash
cargo test --test test_user -p torn_client
cargo test --test test_faction -p torn_client
cargo test --test test_pagination -p torn_client
```

**Run specific test:**
```bash
cargo test --test test_user live_user_basic -p torn_client
```

#### Skipping Behavior

When `TORN_API_KEY` is not set, tests skip gracefully:

```
TORN_API_KEY not set — skipping live_user_basic
TORN_API_KEY not set — skipping live_faction_members
```

- Live API tests print a message and return early (not a failure)
- Scaffold tests (client construction, etc.) always run
- Tests that require faction membership handle errors gracefully

### Spec Validation Tests

These tests verify that the checked-in OpenAPI spec is valid JSON with the
expected structure. They require `openapi/latest.json` to exist (run
`python scripts/fetch_spec.py` first).

## CI Configuration

### Unit Tests (`tests.yml` → `unit-tests` job)

- Runs on every push and PR
- No secrets required
- Runs `cargo test --workspace`

### Integration Tests (`tests.yml` → `integration-tests` job)

- Runs only on pushes to `main` (to protect API keys from fork PRs)
- Uses `TORN_API_KEY` and `TORN_API_KEYS` from GitHub Secrets
- Fetches latest spec before running tests

### Lint (`tests.yml` → `lint` job)

- Runs `cargo fmt --check` and `cargo clippy`
- No secrets required

### Regen Check (`regen_check.yml`)

- Runs `python scripts/regen.py` and checks for git diff
- Ensures generated code is up to date
- See `GENERATED_POLICY.md` for details

## Setting Up CI Secrets

In your GitHub repository settings, add these secrets:

1. **`TORN_API_KEY`** — A single Torn API key for integration tests
2. **`TORN_API_KEYS`** (optional) — Comma-separated keys for balancing tests

Example GitHub Actions workflow:

```yaml
- name: Run integration tests
  env:
    TORN_API_KEY: ${{ secrets.TORN_API_KEY }}
    TORN_API_KEYS: ${{ secrets.TORN_API_KEYS }}
  run: cargo test --workspace
```

## Troubleshooting

### Tests Skip Even With API Key Set

Make sure the environment variable is set in the same shell session where you run the tests:

```bash
echo $TORN_API_KEY  # Linux/macOS
echo %TORN_API_KEY%  # Windows CMD
echo $env:TORN_API_KEY  # Windows PowerShell
```

### Rate Limit Errors

If you hit rate limits:

1. Wait 60 seconds for the rate limit window to reset
2. Use multiple API keys via `TORN_API_KEYS`
3. Run tests with `RateLimitMode::AutoDelay` (default)
4. Run test files individually instead of all at once

Integration tests hit the real Torn API. Be aware of rate limits:

- **100 calls/minute** per API key
- **1000 calls/minute** per IP address

Tests are designed to stay well within these limits (~40 total calls).

### Permission Errors

Some endpoints require specific API key permissions:

- Faction endpoints require faction membership
- User private data requires appropriate access level

Tests are designed to handle permission errors gracefully and skip when needed.

## Test Coverage

### Endpoint Tests

Each endpoint group has 2-6 tests covering:

- **Basic retrieval** - Verify endpoint returns valid data
- **Structure validation** - Check response has expected fields
- **ID-scoped access** - Test endpoints with specific IDs (e.g., user 4, faction 10000)
- **Error handling** - Verify graceful handling of permission errors

### Rate Limit Tests

Tests verify that rate limiting works correctly:

- `AutoDelay` mode - Automatically waits when limit reached
- `ThrowOnLimit` mode - Returns error when limit exceeded
- `Ignore` mode - Bypasses rate limiting entirely
- Multi-key balancing - Distributes requests across multiple keys

### Pagination Tests

Tests verify pagination navigation:

- `has_next()` / `has_prev()` detection
- `next()` / `prev()` navigation
- URL extraction and parsing
- Page stream iteration (limited to 3 pages to avoid long tests)

## Rate Limit Considerations

The integration test suite is designed to be rate-limit friendly:

- Each test file makes ≤5 API calls
- Total suite makes ~40 calls when fully run with API key
- Tests are independent (don't depend on other test outcomes)
- Skip gracefully when API key is not set
- Use AutoDelay mode by default to handle rate limits

### Running Tests Without Hitting Rate Limits

If you're concerned about rate limits:

1. Run tests one file at a time
2. Use multiple API keys via `TORN_API_KEYS`
3. Add delays between test runs
4. Tests already use `RateLimitMode::AutoDelay` by default

## Common Test Helpers

The `common` module provides shared utilities:

```rust
mod common;

#[tokio::test]
async fn my_test() {
    let Some(client) = common::test_client() else {
        common::skip_message("my_test");
        return;
    };

    let result = client.user().basic().await;
    assert!(result.is_ok());
}
```

**Available helpers:**
- `test_client()` - Returns client with single API key, or None
- `test_client_multi()` - Returns client with multiple keys (for balancing tests)
- `skip_message()` - Prints consistent skip message

## Writing New Tests

When adding a new endpoint, add integration tests following this pattern:

```rust
mod common;

#[tokio::test]
async fn live_my_endpoint() {
    let Some(client) = common::test_client() else {
        common::skip_message("live_my_endpoint");
        return;
    };

    let result = client.my_endpoint().await;
    assert!(result.is_ok(), "my_endpoint() failed: {:?}", result.err());

    let data = result.unwrap();
    // Add structure validation
    assert!(data.some_field.is_some(), "missing expected field");
}
```

Key principles:
- Always gate on `common::test_client()`
- Use `common::skip_message()` for consistent messaging
- Test the happy path at minimum
- Keep tests independent (no ordering dependencies)
- Limit API calls to ≤5 per file
- Add meaningful assertion messages
