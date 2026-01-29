//! Property API endpoints.

use crate::{Error, TornClient};
use torn_models::generated::common::{TimestampResponse, UserPropertyResponse};
use torn_models::generated::property::{PropertyId, PropertyLookupResponse, PropertyPropertyResponse, PropertySelectionName};

/// Property API endpoints (self-scoped, no ID required).
pub struct PropertyEndpoint<'a> {
    client: &'a TornClient,
}

impl<'a> PropertyEndpoint<'a> {
    pub(crate) fn new(client: &'a TornClient) -> Self {
        Self { client }
    }

    /// Get any property selection.
    ///
    /// This endpoint allows you to retrieve property information with optional selections.
    pub async fn get(&self, params: PropertyParams) -> Result<UserPropertyResponse, Error> {
        let mut query = Vec::new();
        
        if let Some(selections) = &params.selections {
            for selection in selections {
                // Serialize each selection as a separate selections[] parameter
                let value = match selection {
                    PropertySelectionName::Variant0(s) | PropertySelectionName::Variant1(s) => s.clone(),
                };
                query.push(("selections", value));
            }
        }
        
        if let Some(id) = params.id {
            query.push(("id", id.to_string()));
        }
        
        if let Some(timestamp) = &params.timestamp {
            query.push(("timestamp", timestamp.clone()));
        }
        
        self.client.request("/property", &query).await
    }

    /// Get all available property selections.
    ///
    /// Returns a list of all valid selection names for the property endpoint.
    pub async fn lookup(&self) -> Result<PropertyLookupResponse, Error> {
        self.client.request("/property/lookup", &[]).await
    }

    /// Get current server time.
    ///
    /// Returns the current Torn server timestamp.
    pub async fn timestamp(&self) -> Result<TimestampResponse, Error> {
        self.client.request("/property/timestamp", &[]).await
    }

    /// Access endpoints for a specific property by ID.
    pub fn with_id(&self, id: PropertyId) -> PropertyIdContext<'a> {
        PropertyIdContext {
            client: self.client,
            id,
        }
    }
}

/// Parameters for the property get endpoint.
#[derive(Debug, Default, Clone)]
pub struct PropertyParams {
    /// Selection names to retrieve specific data.
    pub selections: Option<Vec<PropertySelectionName>>,
    /// Property ID to query.
    pub id: Option<PropertyId>,
    /// Timestamp to bypass cache.
    pub timestamp: Option<String>,
}

/// Property API endpoints scoped to a specific property ID.
pub struct PropertyIdContext<'a> {
    client: &'a TornClient,
    id: PropertyId,
}

impl<'a> PropertyIdContext<'a> {
    /// Get a specific property.
    ///
    /// Returns detailed information about the property with the given ID.
    pub async fn property(&self) -> Result<PropertyPropertyResponse, Error> {
        let path = format!("/property/{}/property", self.id);
        self.client.request(&path, &[]).await
    }
}
