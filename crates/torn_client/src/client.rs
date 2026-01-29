//! Core TornClient and builder pattern.

use crate::config::{ApiKeyBalancing, RateLimitMode, TornClientConfig};
use crate::endpoints::{
    faction::FactionEndpoint, forum::ForumEndpoint, key::KeyEndpoint, market::MarketEndpoint,
    property::PropertyEndpoint, racing::RacingEndpoint, torn::TornEndpoint, UserEndpoint,
};
use crate::key_pool::KeyPool;
use crate::rate_limit::RateLimiter;
use crate::Error;

/// The main Torn API client.
///
/// Holds one or more API keys and manages rate limiting, key rotation,
/// and HTTP transport.
#[derive(Debug)]
pub struct TornClient {
    pub(crate) config: TornClientConfig,
    pub(crate) http: reqwest::Client,
    pub(crate) key_pool: KeyPool,
    pub(crate) rate_limiter: RateLimiter,
}

impl TornClient {
    /// Create a new client with a single API key.
    ///
    /// # Example
    ///
    /// ```
    /// use torn_client::TornClient;
    ///
    /// let client = TornClient::new("YOUR_API_KEY");
    /// ```
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_keys(vec![api_key.into()])
    }

    /// Create a new client with multiple API keys for round-robin balancing.
    ///
    /// # Example
    ///
    /// ```
    /// use torn_client::TornClient;
    ///
    /// let client = TornClient::with_keys(vec![
    ///     "KEY_1".to_string(),
    ///     "KEY_2".to_string(),
    /// ]);
    /// ```
    pub fn with_keys(keys: Vec<String>) -> Self {
        let mut config = TornClientConfig::default();
        config.api_keys = keys;
        Self::with_config(config)
    }

    /// Create a new client with custom configuration.
    ///
    /// # Example
    ///
    /// ```
    /// use torn_client::{TornClient, TornClientConfig};
    ///
    /// let mut config = TornClientConfig::default();
    /// config.api_keys = vec!["YOUR_API_KEY".to_string()];
    /// config.comment = Some("my-app".to_string());
    ///
    /// let client = TornClient::with_config(config);
    /// ```
    pub fn with_config(config: TornClientConfig) -> Self {
        let version = env!("CARGO_PKG_VERSION");
        let user_agent = format!("rs-torn-client/{}", version);

        let http = reqwest::Client::builder()
            .user_agent(user_agent)
            .build()
            .expect("failed to build HTTP client");

        let key_pool = KeyPool::new(config.api_keys.clone(), config.api_key_balancing)
            .expect("failed to create key pool");

        let rate_limiter = RateLimiter::new(config.rate_limit_mode);

        Self {
            config,
            http,
            key_pool,
            rate_limiter,
        }
    }

    /// Create a builder for constructing a client with custom options.
    ///
    /// # Example
    ///
    /// ```
    /// use torn_client::TornClient;
    ///
    /// let client = TornClient::builder()
    ///     .api_key("YOUR_API_KEY")
    ///     .comment("my-app")
    ///     .verbose(true)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn builder() -> TornClientBuilder {
        TornClientBuilder::default()
    }

    /// Returns the number of API keys configured.
    pub fn key_count(&self) -> usize {
        self.config.api_keys.len()
    }

    /// Returns a reference to the underlying reqwest client.
    pub fn http_client(&self) -> &reqwest::Client {
        &self.http
    }

    /// Log a message if verbose mode is enabled.
    pub(crate) fn log(&self, msg: &str) {
        if self.config.verbose {
            eprintln!("[TornClient] {}", msg);
        }
    }

    /// Access key endpoints.
    pub fn key(&self) -> KeyEndpoint<'_> {
        KeyEndpoint::new(self)
    }

    /// Access user endpoints.
    pub fn user(&self) -> UserEndpoint<'_> {
        UserEndpoint::new(self)
    }

    /// Access faction endpoints.
    pub fn faction(&self) -> FactionEndpoint<'_> {
        FactionEndpoint::new(self)
    }

    /// Access property endpoints.
    pub fn property(&self) -> PropertyEndpoint<'_> {
        PropertyEndpoint::new(self)
    }

    /// Access Torn API endpoints.
    pub fn torn(&self) -> TornEndpoint<'_> {
        TornEndpoint::new(self)
    }

    /// Access market endpoints.
    pub fn market(&self) -> MarketEndpoint<'_> {
        MarketEndpoint::new(self)
    }

    /// Access racing endpoints.
    pub fn racing(&self) -> RacingEndpoint<'_> {
        RacingEndpoint::new(self)
    }

    /// Access forum endpoints.
    pub fn forum(&self) -> ForumEndpoint<'_> {
        ForumEndpoint::new(self)
    }

}

/// Builder for constructing a TornClient with custom options.
#[derive(Debug, Default)]
pub struct TornClientBuilder {
    config: TornClientConfig,
}

impl TornClientBuilder {
    /// Add a single API key to the client.
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.config.api_keys.push(key.into());
        self
    }

    /// Set multiple API keys at once.
    pub fn api_keys(mut self, keys: Vec<String>) -> Self {
        self.config.api_keys = keys;
        self
    }

    /// Set the rate limit behavior mode.
    pub fn rate_limit_mode(mut self, mode: RateLimitMode) -> Self {
        self.config.rate_limit_mode = mode;
        self
    }

    /// Set the API key balancing strategy.
    pub fn api_key_balancing(mut self, balancing: ApiKeyBalancing) -> Self {
        self.config.api_key_balancing = balancing;
        self
    }

    /// Set an optional comment to append to all requests.
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.config.comment = Some(comment.into());
        self
    }

    /// Add a custom HTTP header to all requests.
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.config.headers.insert(key.into(), value.into());
        self
    }

    /// Set a custom base URL (default: `https://api.torn.com/v2`).
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.config.base_url = url.into();
        self
    }

    /// Enable verbose debug logging to stderr.
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.config.verbose = verbose;
        self
    }

    /// Build the TornClient.
    ///
    /// Returns an error if no API keys were provided.
    pub fn build(self) -> Result<TornClient, Error> {
        if self.config.api_keys.is_empty() {
            return Err(Error::NoKeys);
        }
        Ok(TornClient::with_config(self.config))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_new() {
        let client = TornClient::new("test-key");
        assert_eq!(client.key_count(), 1);
    }

    #[test]
    fn test_client_with_keys() {
        let client = TornClient::with_keys(vec!["key1".into(), "key2".into()]);
        assert_eq!(client.key_count(), 2);
    }

    #[test]
    fn test_builder_single_key() {
        let client = TornClient::builder()
            .api_key("test-key")
            .build()
            .unwrap();
        assert_eq!(client.key_count(), 1);
    }

    #[test]
    fn test_builder_multiple_keys() {
        let client = TornClient::builder()
            .api_key("key1")
            .api_key("key2")
            .api_key("key3")
            .build()
            .unwrap();
        assert_eq!(client.key_count(), 3);
    }

    #[test]
    fn test_builder_no_keys_error() {
        let result = TornClient::builder().build();
        assert!(matches!(result, Err(Error::NoKeys)));
    }

    #[test]
    fn test_builder_with_config() {
        let client = TornClient::builder()
            .api_key("test-key")
            .comment("my-app")
            .base_url("https://example.com/api")
            .verbose(true)
            .header("X-Custom", "value")
            .build()
            .unwrap();

        assert_eq!(client.config.comment, Some("my-app".to_string()));
        assert_eq!(client.config.base_url, "https://example.com/api");
        assert!(client.config.verbose);
        assert_eq!(
            client.config.headers.get("X-Custom"),
            Some(&"value".to_string())
        );
    }
}
