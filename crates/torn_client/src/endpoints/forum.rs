//! Forum API endpoints.
//!
//! This module provides methods for accessing the Torn forum API,
//! including forum categories, threads, posts, and lookups.

use crate::pagination::PaginatedResponse;
use crate::{Error, TornClient};
use torn_models::generated::common::TimestampResponse;
use torn_models::generated::forum::*;

/// Forum API endpoints (self-scoped).
pub struct ForumEndpoint<'a> {
    client: &'a TornClient,
}

impl<'a> ForumEndpoint<'a> {
    pub(crate) fn new(client: &'a TornClient) -> Self {
        Self { client }
    }

    /// Get publicly available forum categories.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Optional timestamp to bypass cache
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn categories(&self, timestamp: Option<String>) -> Result<PaginatedResponse<ForumCategoriesResponse>, Error> {
        let mut query = Vec::new();

        if let Some(ts) = timestamp {
            query.push(("timestamp", ts));
        }

        self.client.request_paginated("/forum/categories", &query).await
    }

    /// Get all available forum selections.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Optional timestamp to bypass cache
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn lookup(&self, timestamp: Option<String>) -> Result<PaginatedResponse<ForumLookupResponse>, Error> {
        let mut query = Vec::new();

        if let Some(ts) = timestamp {
            query.push(("timestamp", ts));
        }

        self.client.request_paginated("/forum/lookup", &query).await
    }

    /// Get threads across all forum categories.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering threads
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn threads(&self, params: ForumThreadsParams) -> Result<PaginatedResponse<ForumThreadsResponse>, Error> {
        let mut query = Vec::new();

        if let Some(limit) = params.limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(sort) = params.sort {
            query.push(("sort", sort));
        }

        if let Some(from) = params.from {
            query.push(("from", from.to_string()));
        }

        if let Some(to) = params.to {
            query.push(("to", to.to_string()));
        }

        if let Some(offset) = params.offset {
            query.push(("offset", offset.to_string()));
        }

        if let Some(timestamp) = params.timestamp {
            query.push(("timestamp", timestamp));
        }

        self.client.request_paginated("/forum/threads", &query).await
    }

    /// Get current server time.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Optional timestamp to bypass cache
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn timestamp(&self, timestamp: Option<String>) -> Result<PaginatedResponse<TimestampResponse>, Error> {
        let mut query = Vec::new();

        if let Some(ts) = timestamp {
            query.push(("timestamp", ts));
        }

        self.client.request_paginated("/forum/timestamp", &query).await
    }

    /// Access endpoints for a specific thread by ID.
    pub fn with_thread_id(&self, id: ForumThreadId) -> ForumThreadIdContext<'a> {
        ForumThreadIdContext {
            client: self.client,
            id,
        }
    }

    /// Access endpoints for specific category or categories by IDs.
    pub fn with_category_ids(&self, ids: Vec<ForumId>) -> ForumCategoryIdsContext<'a> {
        ForumCategoryIdsContext {
            client: self.client,
            ids,
        }
    }
}

/// Forum API endpoints scoped to a specific thread ID.
pub struct ForumThreadIdContext<'a> {
    client: &'a TornClient,
    id: ForumThreadId,
}

impl<'a> ForumThreadIdContext<'a> {
    /// Get specific forum thread posts.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering posts
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn posts(&self, params: ForumPostsParams) -> Result<PaginatedResponse<ForumPostsResponse>, Error> {
        let path = format!("/forum/{}/posts", self.id);
        let mut query = Vec::new();

        if let Some(striptags) = params.striptags {
            query.push(("striptags", striptags));
        }

        if let Some(limit) = params.limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(sort) = params.sort {
            query.push(("sort", sort));
        }

        if let Some(from) = params.from {
            query.push(("from", from.to_string()));
        }

        if let Some(to) = params.to {
            query.push(("to", to.to_string()));
        }

        if let Some(offset) = params.offset {
            query.push(("offset", offset.to_string()));
        }

        if let Some(timestamp) = params.timestamp {
            query.push(("timestamp", timestamp));
        }

        self.client.request_paginated(&path, &query).await
    }

    /// Get specific thread details.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Optional timestamp to bypass cache
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn thread(&self, timestamp: Option<String>) -> Result<PaginatedResponse<ForumThreadResponse>, Error> {
        let path = format!("/forum/{}/thread", self.id);
        let mut query = Vec::new();

        if let Some(ts) = timestamp {
            query.push(("timestamp", ts));
        }

        self.client.request_paginated(&path, &query).await
    }
}

/// Forum API endpoints scoped to specific category IDs.
pub struct ForumCategoryIdsContext<'a> {
    client: &'a TornClient,
    ids: Vec<ForumId>,
}

impl<'a> ForumCategoryIdsContext<'a> {
    /// Get threads for specific public forum category or categories.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering threads
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn threads(&self, params: ForumThreadsParams) -> Result<PaginatedResponse<ForumThreadsResponse>, Error> {
        // Join category IDs with commas
        let category_ids_str = self.ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let path = format!("/forum/{}/threads", category_ids_str);
        let mut query = Vec::new();

        if let Some(limit) = params.limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(sort) = params.sort {
            query.push(("sort", sort));
        }

        if let Some(from) = params.from {
            query.push(("from", from.to_string()));
        }

        if let Some(to) = params.to {
            query.push(("to", to.to_string()));
        }

        if let Some(offset) = params.offset {
            query.push(("offset", offset.to_string()));
        }

        if let Some(timestamp) = params.timestamp {
            query.push(("timestamp", timestamp));
        }

        self.client.request_paginated(&path, &query).await
    }
}

/// Parameters for the forum threads endpoints.
#[derive(Debug, Default, Clone)]
pub struct ForumThreadsParams {
    /// Pagination limit (default: 20, max: 100)
    pub limit: Option<i32>,
    /// Sorted by the greatest timestamps (DESC or ASC)
    pub sort: Option<String>,
    /// Timestamp that sets the lower limit for the data returned
    pub from: Option<i32>,
    /// Timestamp that sets the upper limit for the data returned
    pub to: Option<i32>,
    /// Pagination offset
    pub offset: Option<i32>,
    /// Timestamp to bypass cache
    pub timestamp: Option<String>,
}

/// Parameters for the forum posts endpoint.
#[derive(Debug, Default, Clone)]
pub struct ForumPostsParams {
    /// Determines if fields include HTML or not
    pub striptags: Option<String>,
    /// Pagination limit (default: 20, max: 100)
    pub limit: Option<i32>,
    /// Sorted by the greatest timestamps (DESC or ASC)
    pub sort: Option<String>,
    /// Timestamp that sets the lower limit for the data returned
    pub from: Option<i32>,
    /// Timestamp that sets the upper limit for the data returned
    pub to: Option<i32>,
    /// Pagination offset
    pub offset: Option<i32>,
    /// Timestamp to bypass cache
    pub timestamp: Option<String>,
}
