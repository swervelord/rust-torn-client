//! Pagination support for paginated API responses.
//!
//! This module provides types and methods for navigating paginated responses
//! from the Torn API, including `.next()` and `.prev()` methods plus an
//! async stream adapter for lazy iteration.

use crate::{Error, TornClient};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// A paginated API response with navigation methods.
///
/// Contains the response data plus pagination metadata from the Torn API.
/// Use `.next()` and `.prev()` to fetch adjacent pages, or `.pages()` to
/// create an async stream that yields all pages automatically.
///
/// # Example
///
/// ```rust,ignore
/// // Fetch a paginated endpoint
/// let response = client.user().attacks().await?;
///
/// // Check if there are more pages
/// if response.has_next() {
///     let next_page = response.next().await?.unwrap();
///     println!("Next page has {} attacks", next_page.data.attacks.len());
/// }
///
/// // Or use the stream adapter
/// let mut pages = response.pages();
/// while let Some(page_result) = pages.next_page().await {
///     let page = page_result?;
///     process_attacks(&page.data.attacks);
/// }
/// ```
///
/// # Implementation Note
///
/// Currently, each `PaginatedResponse` creates its own `TornClient` instance for
/// fetching subsequent pages. This means rate limiting state is not shared between
/// the original client and pagination navigation. In practice, this should not cause
/// issues for typical pagination use cases, but heavy concurrent pagination may not
/// benefit from optimal rate limit sharing.
///
/// Future improvement: Refactor `TornClient` to use `Arc` internally so that
/// `PaginatedResponse` can share the same client instance.
#[derive(Debug, Clone)]
pub struct PaginatedResponse<T> {
    /// The response data for this page.
    pub data: T,

    /// Pagination metadata (if present).
    metadata: Option<PaginationMetadata>,

    /// URL of the next page, if any.
    next_url: Option<String>,

    /// URL of the previous page, if any.
    prev_url: Option<String>,

    /// Reference to the client for fetching pages.
    client: Arc<TornClient>,
}

/// Pagination metadata from the Torn API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMetadata {
    /// Pagination links.
    #[serde(default)]
    pub links: PaginationLinks,
}

/// Pagination navigation links.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaginationLinks {
    /// URL to the next page (if any).
    pub next: Option<String>,

    /// URL to the previous page (if any).
    #[serde(rename = "prev")]
    pub previous: Option<String>,
}

impl<T> PaginatedResponse<T> {
    /// Create a new PaginatedResponse from raw response data.
    ///
    /// This is called internally by `request_paginated()`.
    pub(crate) fn new(
        data: T,
        metadata: Option<PaginationMetadata>,
        client: Arc<TornClient>,
    ) -> Self {
        let next_url = metadata.as_ref().and_then(|m| m.links.next.clone());
        let prev_url = metadata.as_ref().and_then(|m| m.links.previous.clone());

        Self {
            data,
            metadata,
            next_url,
            prev_url,
            client,
        }
    }

    /// Returns true if there is a next page available.
    pub fn has_next(&self) -> bool {
        self.next_url.is_some()
    }

    /// Returns true if there is a previous page available.
    pub fn has_prev(&self) -> bool {
        self.prev_url.is_some()
    }

    /// Fetch the next page of results.
    ///
    /// Returns `None` if there is no next page.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails or the response cannot be parsed.
    pub async fn next(&self) -> Result<Option<PaginatedResponse<T>>, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        match &self.next_url {
            Some(url) => {
                let (path, params) = parse_pagination_url(url)?;
                // Convert Vec<(String, String)> to Vec<(&str, String)>
                let params_refs: Vec<(&str, String)> = params
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.clone()))
                    .collect();
                let response = self.client.request_paginated(&path, &params_refs).await?;
                Ok(Some(response))
            }
            None => Ok(None),
        }
    }

    /// Fetch the previous page of results.
    ///
    /// Returns `None` if there is no previous page.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails or the response cannot be parsed.
    pub async fn prev(&self) -> Result<Option<PaginatedResponse<T>>, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        match &self.prev_url {
            Some(url) => {
                let (path, params) = parse_pagination_url(url)?;
                // Convert Vec<(String, String)> to Vec<(&str, String)>
                let params_refs: Vec<(&str, String)> = params
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.clone()))
                    .collect();
                let response = self.client.request_paginated(&path, &params_refs).await?;
                Ok(Some(response))
            }
            None => Ok(None),
        }
    }

    /// Returns an async stream that yields pages starting from this one.
    ///
    /// This provides a convenient way to iterate through all pages without
    /// manually calling `.next()` in a loop.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let mut pages = response.pages();
    /// while let Some(page_result) = pages.next_page().await {
    ///     let page = page_result?;
    ///     process_data(&page.data);
    /// }
    /// ```
    pub fn pages(self) -> PageStream<T> {
        PageStream {
            current: Some(self),
            done: false,
        }
    }

    /// Returns true if there is a next page available.
    ///
    /// This is an alias for `has_next()` to match the existing API.
    pub fn has_next_page(&self) -> bool {
        self.has_next()
    }

    /// Returns true if there is a previous page available.
    ///
    /// This is an alias for `has_prev()` to match the existing API.
    pub fn has_previous_page(&self) -> bool {
        self.has_prev()
    }

    /// Get the next page URL if available.
    pub fn next_url(&self) -> Option<&str> {
        self.next_url.as_deref()
    }

    /// Get the previous page URL if available.
    pub fn prev_url(&self) -> Option<&str> {
        self.prev_url.as_deref()
    }
}

/// Parse a Torn API pagination URL into path + query params.
///
/// Extracts the path and query parameters from a pagination URL,
/// stripping the `/v2` prefix to match the client's internal path format.
///
/// # Example
///
/// ```rust,ignore
/// let (path, params) = parse_pagination_url(
///     "https://api.torn.com/v2/user/attacks?limit=25&cursor=abc123"
/// )?;
/// assert_eq!(path, "/user/attacks");
/// assert_eq!(params, vec![
///     ("limit".to_string(), "25".to_string()),
///     ("cursor".to_string(), "abc123".to_string())
/// ]);
/// ```
///
/// # Errors
///
/// Returns an error if the URL cannot be parsed.
fn parse_pagination_url(url: &str) -> Result<(String, Vec<(String, String)>), Error> {
    // Parse the URL using reqwest's Url type (which is url::Url)
    let parsed = reqwest::Url::parse(url)
        .map_err(|e| Error::Request(format!("Invalid pagination URL: {}", e)))?;

    // Extract path, stripping the /v2 prefix (matching TS client behavior)
    let path = parsed.path();
    let path = if path.starts_with("/v2") {
        &path[3..]
    } else {
        path
    };

    // Extract query parameters
    let params: Vec<(String, String)> = parsed
        .query_pairs()
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect();

    Ok((path.to_string(), params))
}

/// Async stream that yields pages one at a time.
///
/// Created by calling `.pages()` on a `PaginatedResponse`.
/// Automatically fetches the next page when you call `.next_page()`.
pub struct PageStream<T> {
    current: Option<PaginatedResponse<T>>,
    done: bool,
}

impl<T> PageStream<T>
where
    T: serde::de::DeserializeOwned,
{
    /// Get the next page from the stream.
    ///
    /// Returns `None` when there are no more pages.
    ///
    /// # Errors
    ///
    /// Returns an error if fetching the next page fails.
    pub async fn next_page(&mut self) -> Option<Result<PaginatedResponse<T>, Error>> {
        if self.done {
            return None;
        }

        let current = self.current.take()?;

        // Try to fetch next page
        match current.next().await {
            Ok(Some(next_page)) => {
                // We have a next page, so yield current and prepare next
                self.current = Some(next_page);
                Some(Ok(current))
            }
            Ok(None) => {
                // No more pages, yield current and mark done
                self.done = true;
                Some(Ok(current))
            }
            Err(e) => {
                // Error fetching next page, yield error and stop
                self.done = true;
                Some(Err(e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pagination_url_basic() {
        let url = "https://api.torn.com/v2/user/attacks?limit=25&cursor=abc123";
        let (path, params) = parse_pagination_url(url).unwrap();

        assert_eq!(path, "/user/attacks");
        assert_eq!(params.len(), 2);
        assert!(params.iter().any(|(k, v)| k == "limit" && v == "25"));
        assert!(params.iter().any(|(k, v)| k == "cursor" && v == "abc123"));
    }

    #[test]
    fn test_parse_pagination_url_strips_v2() {
        let url = "https://api.torn.com/v2/faction/123/members?sort=DESC";
        let (path, params) = parse_pagination_url(url).unwrap();

        assert_eq!(path, "/faction/123/members");
        assert_eq!(params.len(), 1);
        assert!(params.iter().any(|(k, v)| k == "sort" && v == "DESC"));
    }

    #[test]
    fn test_parse_pagination_url_no_query() {
        let url = "https://api.torn.com/v2/user";
        let (path, params) = parse_pagination_url(url).unwrap();

        assert_eq!(path, "/user");
        assert!(params.is_empty());
    }

    #[test]
    fn test_parse_pagination_url_no_v2_prefix() {
        let url = "https://api.torn.com/user/attacks?limit=25";
        let (path, params) = parse_pagination_url(url).unwrap();

        assert_eq!(path, "/user/attacks");
        assert_eq!(params.len(), 1);
        assert!(params.iter().any(|(k, v)| k == "limit" && v == "25"));
    }

    #[test]
    fn test_parse_pagination_url_encoded_params() {
        let url = "https://api.torn.com/v2/user?name=test%20user&value=100%25";
        let (path, params) = parse_pagination_url(url).unwrap();

        assert_eq!(path, "/user");
        assert_eq!(params.len(), 2);
        assert!(params.iter().any(|(k, v)| k == "name" && v == "test user"));
        assert!(params.iter().any(|(k, v)| k == "value" && v == "100%"));
    }

    #[test]
    fn test_parse_pagination_url_invalid() {
        let url = "not a valid url";
        let result = parse_pagination_url(url);

        assert!(result.is_err());
        match result {
            Err(Error::Request(msg)) => {
                assert!(msg.contains("Invalid pagination URL"));
            }
            _ => panic!("Expected Request error"),
        }
    }

    #[test]
    fn test_pagination_metadata_has_next() {
        let metadata = PaginationMetadata {
            links: PaginationLinks {
                next: Some("https://api.torn.com/v2/user/attacks?cursor=abc".to_string()),
                previous: None,
            },
        };

        assert!(metadata.links.next.is_some());
        assert!(metadata.links.previous.is_none());
    }

    #[test]
    fn test_pagination_metadata_has_prev() {
        let metadata = PaginationMetadata {
            links: PaginationLinks {
                next: None,
                previous: Some("https://api.torn.com/v2/user/attacks?cursor=xyz".to_string()),
            },
        };

        assert!(metadata.links.next.is_none());
        assert!(metadata.links.previous.is_some());
    }

    #[test]
    fn test_pagination_metadata_has_both() {
        let metadata = PaginationMetadata {
            links: PaginationLinks {
                next: Some("https://api.torn.com/v2/user/attacks?cursor=next".to_string()),
                previous: Some("https://api.torn.com/v2/user/attacks?cursor=prev".to_string()),
            },
        };

        assert!(metadata.links.next.is_some());
        assert!(metadata.links.previous.is_some());
    }

    #[test]
    fn test_pagination_metadata_has_neither() {
        let metadata = PaginationMetadata {
            links: PaginationLinks {
                next: None,
                previous: None,
            },
        };

        assert!(metadata.links.next.is_none());
        assert!(metadata.links.previous.is_none());
    }

    #[test]
    fn test_has_next_with_url() {
        let client = Arc::new(TornClient::new("test-key"));
        let metadata = Some(PaginationMetadata {
            links: PaginationLinks {
                next: Some("https://api.torn.com/v2/user/attacks?cursor=abc".to_string()),
                previous: None,
            },
        });

        let response: PaginatedResponse<()> = PaginatedResponse::new((), metadata, client);

        assert!(response.has_next());
        assert!(!response.has_prev());
        assert!(response.has_next_page());
        assert!(!response.has_previous_page());
    }

    #[test]
    fn test_has_prev_with_url() {
        let client = Arc::new(TornClient::new("test-key"));
        let metadata = Some(PaginationMetadata {
            links: PaginationLinks {
                next: None,
                previous: Some("https://api.torn.com/v2/user/attacks?cursor=xyz".to_string()),
            },
        });

        let response: PaginatedResponse<()> = PaginatedResponse::new((), metadata, client);

        assert!(!response.has_next());
        assert!(response.has_prev());
        assert!(!response.has_next_page());
        assert!(response.has_previous_page());
    }

    #[test]
    fn test_has_both_urls() {
        let client = Arc::new(TornClient::new("test-key"));
        let metadata = Some(PaginationMetadata {
            links: PaginationLinks {
                next: Some("https://api.torn.com/v2/user/attacks?cursor=next".to_string()),
                previous: Some("https://api.torn.com/v2/user/attacks?cursor=prev".to_string()),
            },
        });

        let response: PaginatedResponse<()> = PaginatedResponse::new((), metadata, client);

        assert!(response.has_next());
        assert!(response.has_prev());
    }

    #[test]
    fn test_has_neither_url() {
        let client = Arc::new(TornClient::new("test-key"));
        let metadata = Some(PaginationMetadata {
            links: PaginationLinks {
                next: None,
                previous: None,
            },
        });

        let response: PaginatedResponse<()> = PaginatedResponse::new((), metadata, client);

        assert!(!response.has_next());
        assert!(!response.has_prev());
    }

    #[test]
    fn test_no_metadata() {
        let client = Arc::new(TornClient::new("test-key"));
        let response: PaginatedResponse<()> = PaginatedResponse::new((), None, client);

        assert!(!response.has_next());
        assert!(!response.has_prev());
    }

    #[test]
    fn test_next_url() {
        let client = Arc::new(TornClient::new("test-key"));
        let metadata = Some(PaginationMetadata {
            links: PaginationLinks {
                next: Some("https://api.torn.com/v2/user/attacks?cursor=abc".to_string()),
                previous: None,
            },
        });

        let response: PaginatedResponse<()> = PaginatedResponse::new((), metadata, client);

        assert_eq!(
            response.next_url(),
            Some("https://api.torn.com/v2/user/attacks?cursor=abc")
        );
        assert_eq!(response.prev_url(), None);
    }

    #[test]
    fn test_prev_url() {
        let client = Arc::new(TornClient::new("test-key"));
        let metadata = Some(PaginationMetadata {
            links: PaginationLinks {
                next: None,
                previous: Some("https://api.torn.com/v2/user/attacks?cursor=xyz".to_string()),
            },
        });

        let response: PaginatedResponse<()> = PaginatedResponse::new((), metadata, client);

        assert_eq!(response.next_url(), None);
        assert_eq!(
            response.prev_url(),
            Some("https://api.torn.com/v2/user/attacks?cursor=xyz")
        );
    }
}
