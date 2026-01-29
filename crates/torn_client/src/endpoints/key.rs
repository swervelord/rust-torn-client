//! Key API endpoints.
//!
//! Provides access to all key-related endpoints in the Torn API.
//! These endpoints allow you to query information about the API key being used.

use crate::{Error, TornClient};
use torn_models::generated::key::*;

/// Key API endpoints (self-key, no ID required).
pub struct KeyEndpoint<'a> {
    client: &'a TornClient,
}

impl<'a> KeyEndpoint<'a> {
    pub(crate) fn new(client: &'a TornClient) -> Self {
        Self { client }
    }

    /// Get any Key selection.
    ///
    /// Retrieves information based on the provided selections parameter.
    /// Supports limit and offset for pagination.
    pub async fn selections(&self, params: KeySelectionsParams) -> Result<KeyInfoResponse, Error> {
        let mut query = Vec::new();

        if let Some(selections) = params.selections {
            for selection in selections {
                match selection {
                    KeySelectionName::Variant0(s) | KeySelectionName::Variant1(s) => {
                        query.push(("selections", s));
                    }
                }
            }
        }

        if let Some(limit) = params.limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(offset) = params.offset {
            query.push(("offset", offset.to_string()));
        }

        self.client.request("/key", &query).await
    }

    /// Get current key info.
    ///
    /// Returns information about the API key being used, including access level,
    /// permissions, and available selections.
    pub async fn info(&self) -> Result<KeyInfoResponse, Error> {
        self.client.request("/key/info", &[]).await
    }

    /// Get current key log history.
    ///
    /// Returns the request history for the current API key.
    /// Supports limit and offset for pagination.
    pub async fn log(&self, params: KeyLogParams) -> Result<KeyLogResponse, Error> {
        let mut query = Vec::new();

        if let Some(limit) = params.limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(offset) = params.offset {
            query.push(("offset", offset.to_string()));
        }

        self.client.request("/key/log", &query).await
    }
}

/// Parameters for the key selections endpoint.
#[derive(Debug, Default, Clone)]
pub struct KeySelectionsParams {
    /// Selection names to retrieve.
    pub selections: Option<Vec<KeySelectionName>>,
    /// Maximum number of results to return (1-100, default: 100).
    pub limit: Option<u32>,
    /// Offset for pagination.
    pub offset: Option<u32>,
}

/// Parameters for the key log endpoint.
#[derive(Debug, Default, Clone)]
pub struct KeyLogParams {
    /// Maximum number of results to return (1-100, default: 100).
    pub limit: Option<u32>,
    /// Offset for pagination.
    pub offset: Option<u32>,
}
