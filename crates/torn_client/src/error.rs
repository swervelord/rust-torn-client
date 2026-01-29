//! Error types for the Torn client.

/// Top-level error type for all Torn client operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// HTTP transport error from reqwest.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// The Torn API returned an error response.
    #[error("Torn API error {code}: {message}")]
    Api {
        /// Torn error code (e.g. 2 = "Incorrect ID").
        code: u16,
        /// Human-readable error message from Torn.
        message: String,
    },

    /// Rate limit exceeded — the client should back off.
    #[error("Rate limit exceeded")]
    RateLimited,

    /// No API keys configured.
    #[error("No API keys configured")]
    NoKeys,

    /// Request failed with a custom message.
    #[error("Request failed: {0}")]
    Request(String),
}
