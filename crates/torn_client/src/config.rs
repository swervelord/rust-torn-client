//! Configuration types for the Torn API client.

use std::collections::HashMap;

/// Configuration for the Torn API client.
#[derive(Debug, Clone)]
pub struct TornClientConfig {
    /// One or more API keys (required, non-empty).
    pub api_keys: Vec<String>,
    /// Rate limit behavior. Default: `AutoDelay`.
    pub rate_limit_mode: RateLimitMode,
    /// Key balancing strategy. Default: `RoundRobin`.
    pub api_key_balancing: ApiKeyBalancing,
    /// Optional comment appended to all requests.
    pub comment: Option<String>,
    /// Custom HTTP headers added to every request.
    pub headers: HashMap<String, String>,
    /// Base URL. Default: `https://api.torn.com/v2`.
    pub base_url: String,
    /// Enable verbose debug logging. Default: false.
    pub verbose: bool,
}

impl Default for TornClientConfig {
    fn default() -> Self {
        Self {
            api_keys: Vec::new(),
            rate_limit_mode: RateLimitMode::default(),
            api_key_balancing: ApiKeyBalancing::default(),
            comment: None,
            headers: HashMap::new(),
            base_url: "https://api.torn.com/v2".to_string(),
            verbose: false,
        }
    }
}

/// Rate limiting behavior mode.
#[derive(Debug, Clone, Copy, Default)]
pub enum RateLimitMode {
    /// Automatically delay requests to stay within rate limits.
    #[default]
    AutoDelay,
    /// Throw an error when rate limit would be exceeded.
    ThrowOnLimit,
    /// Ignore rate limits entirely.
    Ignore,
}

/// API key balancing strategy for multi-key clients.
#[derive(Debug, Clone, Copy, Default)]
pub enum ApiKeyBalancing {
    /// Use keys in round-robin order.
    #[default]
    RoundRobin,
    /// Select keys randomly.
    Random,
}
