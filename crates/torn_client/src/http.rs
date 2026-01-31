//! HTTP request building and response handling.

use crate::client::TornClient;
use crate::pagination::{PaginatedResponse, PaginationMetadata};
use crate::Error;
use serde::Deserialize;
use std::sync::Arc;

/// Torn API error response shape.
#[derive(Debug, Deserialize)]
struct TornApiErrorResponse {
    error: TornApiError,
}

#[derive(Debug, Deserialize)]
struct TornApiError {
    code: u16,
    error: String,
}

impl TornClient {
    /// Make an authenticated GET request to the Torn API.
    ///
    /// This is the core method all endpoint methods call.
    ///
    /// # Arguments
    ///
    /// * `path` - The API path (e.g., "/user")
    /// * `params` - Query parameters as key-value pairs
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Rate limit is exceeded
    /// - HTTP request fails
    /// - Response cannot be parsed
    /// - Torn API returns an error response
    pub(crate) async fn request<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, String)],
    ) -> Result<T, Error> {
        // 1. Get an available API key, respecting rate limits
        let api_key = self.rate_limiter.wait_for_available_key(&self.key_pool).await?;

        // Log the request if verbose mode is enabled
        self.log(&format!(
            "Request: {} (key: {}...)",
            path,
            &api_key.chars().take(5).collect::<String>()
        ));

        // 3. Build the URL
        let url = self.build_url(path, params)?;

        self.log(&format!("URL: {}", url));

        // 4. Build the request with headers
        let mut request = self
            .http
            .get(&url)
            .header("Authorization", format!("ApiKey {}", api_key))
            .header("Accept", "application/json");

        // Add custom headers from config
        for (key, value) in &self.config.headers {
            request = request.header(key, value);
        }

        // 5. Execute the request
        let response = request.send().await?;

        // Log response status
        self.log(&format!("Response status: {}", response.status()));

        // 6. Check HTTP status
        let status = response.status();
        if !status.is_success() {
            return Err(Error::Request(format!(
                "HTTP {} from API",
                status.as_u16()
            )));
        }

        // 7. Get response text for parsing
        let response_text = response.text().await?;

        // 8. Check for Torn API error shape first
        if let Ok(error_response) = serde_json::from_str::<TornApiErrorResponse>(&response_text) {
            self.log(&format!(
                "API error: {} (code {})",
                error_response.error.error, error_response.error.code
            ));
            return Err(Error::Api {
                code: error_response.error.code,
                message: error_response.error.error,
            });
        }

        // 9. Deserialize into the target type
        let data: T = serde_json::from_str(&response_text)?;

        // 10. Record the request for rate limiting
        self.rate_limiter.record_request(&api_key);

        Ok(data)
    }

    /// Make a request and return a PaginatedResponse if metadata is present.
    ///
    /// This method checks for `_metadata.links` in the response and wraps
    /// the result in a PaginatedResponse.
    ///
    /// # Arguments
    ///
    /// * `path` - The API path (e.g., "/user")
    /// * `params` - Query parameters as key-value pairs
    ///
    /// # Errors
    ///
    /// Returns the same errors as `request()`.
    pub(crate) async fn request_paginated<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, String)],
    ) -> Result<PaginatedResponse<T>, Error> {
        // Get an available API key, respecting rate limits
        let api_key = self.rate_limiter.wait_for_available_key(&self.key_pool).await?;

        self.log(&format!(
            "Paginated request: {} (key: {}...)",
            path,
            &api_key.chars().take(5).collect::<String>()
        ));

        // Build the URL
        let url = self.build_url(path, params)?;

        self.log(&format!("URL: {}", url));

        // Build the request with headers
        let mut request = self
            .http
            .get(&url)
            .header("Authorization", format!("ApiKey {}", api_key))
            .header("Accept", "application/json");

        // Add custom headers from config
        for (key, value) in &self.config.headers {
            request = request.header(key, value);
        }

        // Execute the request
        let response = request.send().await?;

        self.log(&format!("Response status: {}", response.status()));

        // Check HTTP status
        let status = response.status();
        if !status.is_success() {
            return Err(Error::Request(format!(
                "HTTP {} from API",
                status.as_u16()
            )));
        }

        // Get response text for parsing
        let response_text = response.text().await?;

        // Check for Torn API error shape first
        if let Ok(error_response) = serde_json::from_str::<TornApiErrorResponse>(&response_text) {
            self.log(&format!(
                "API error: {} (code {})",
                error_response.error.error, error_response.error.code
            ));
            return Err(Error::Api {
                code: error_response.error.code,
                message: error_response.error.error,
            });
        }

        // Parse response as a generic JSON value first to extract metadata
        let json_value: serde_json::Value = serde_json::from_str(&response_text)?;

        // Extract metadata if present
        let metadata: Option<PaginationMetadata> = json_value
            .get("_metadata")
            .and_then(|m| serde_json::from_value(m.clone()).ok());

        // Deserialize the main data (which includes the flattened fields)
        let data: T = serde_json::from_str(&response_text)?;

        // Record the request for rate limiting
        self.rate_limiter.record_request(&api_key);

        // Create an Arc<TornClient> for the paginated response
        // Note: This is a bit of a workaround since we can't easily convert &self to Arc<Self>
        // In a real implementation, TornClient would internally use Arc
        // For now, we'll create a new client with the same config (which is not ideal)
        // TODO: Refactor TornClient to use Arc internally
        let client_arc = Arc::new(TornClient::with_config(self.config.clone()));

        Ok(PaginatedResponse::new(data, metadata, client_arc))
    }

    /// Build a complete URL from path and query parameters.
    ///
    /// Filters out "key" and "comment" from user-supplied params,
    /// and appends the configured comment if present.
    fn build_url(&self, path: &str, params: &[(&str, String)]) -> Result<String, Error> {
        let mut url = format!("{}{}", self.config.base_url, path);

        // Filter out "key" and "comment" from params
        let filtered_params: Vec<_> = params
            .iter()
            .filter(|(k, _)| *k != "key" && *k != "comment")
            .collect();

        // Build query string
        let mut query_parts = Vec::new();

        for (key, value) in filtered_params {
            query_parts.push(format!("{}={}", key, urlencoding::encode(value)));
        }

        // Add comment if configured
        if let Some(ref comment) = self.config.comment {
            query_parts.push(format!("comment={}", urlencoding::encode(comment)));
        }

        // Append query string if we have any parameters
        if !query_parts.is_empty() {
            url.push('?');
            url.push_str(&query_parts.join("&"));
        }

        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use crate::TornClient;

    #[test]
    fn test_build_url_no_params() {
        let client = TornClient::new("test-key");
        let url = client.build_url("/user", &[]).unwrap();
        assert_eq!(url, "https://api.torn.com/v2/user");
    }

    #[test]
    fn test_build_url_with_params() {
        let client = TornClient::new("test-key");
        let url = client
            .build_url("/user", &[("id", "123".to_string())])
            .unwrap();
        assert_eq!(url, "https://api.torn.com/v2/user?id=123");
    }

    #[test]
    fn test_build_url_filters_key_and_comment() {
        let client = TornClient::new("test-key");
        let url = client
            .build_url(
                "/user",
                &[
                    ("id", "123".to_string()),
                    ("key", "should-be-filtered".to_string()),
                    ("comment", "should-be-filtered".to_string()),
                ],
            )
            .unwrap();
        assert_eq!(url, "https://api.torn.com/v2/user?id=123");
    }

    #[test]
    fn test_build_url_appends_comment() {
        let client = TornClient::builder()
            .api_key("test-key")
            .comment("my-app")
            .build()
            .unwrap();

        let url = client
            .build_url("/user", &[("id", "123".to_string())])
            .unwrap();
        assert_eq!(url, "https://api.torn.com/v2/user?id=123&comment=my-app");
    }

    #[test]
    fn test_build_url_custom_base_url() {
        let client = TornClient::builder()
            .api_key("test-key")
            .base_url("https://example.com/api")
            .build()
            .unwrap();

        let url = client.build_url("/user", &[]).unwrap();
        assert_eq!(url, "https://example.com/api/user");
    }

    #[test]
    fn test_build_url_encodes_special_chars() {
        let client = TornClient::new("test-key");
        let url = client
            .build_url("/user", &[("name", "test user".to_string())])
            .unwrap();
        assert_eq!(url, "https://api.torn.com/v2/user?name=test%20user");
    }
}
