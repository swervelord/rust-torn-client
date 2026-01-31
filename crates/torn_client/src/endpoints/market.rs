//! Market API endpoints.
//!
//! This module provides methods for accessing the Torn market API,
//! including bazaar listings, auction house, item market, and property listings.

use crate::pagination::PaginatedResponse;
use crate::{Error, TornClient};
use torn_models::generated::common::{MarketSpecializedBazaarCategoryEnum, WeaponBonusEnum};
use torn_models::generated::market::*;
use torn_models::generated::torn::ItemId;

/// Market API endpoints (self-scoped).
pub struct MarketEndpoint<'a> {
    client: &'a TornClient,
}

impl<'a> MarketEndpoint<'a> {
    pub(crate) fn new(client: &'a TornClient) -> Self {
        Self { client }
    }

    /// Get any Market selection.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering market selections
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get(&self, params: MarketGetParams) -> Result<PaginatedResponse<BazaarResponse>, Error> {
        let mut query = Vec::new();

        if let Some(selections) = params.selections {
            for selection in selections {
                query.push(("selections", format!("{:?}", selection)));
            }
        }

        if let Some(id) = params.id {
            query.push(("id", id.to_string()));
        }

        if let Some(legacy) = params.legacy {
            for leg in legacy {
                query.push(("legacy", format!("{:?}", leg)));
            }
        }

        if let Some(cat) = params.cat {
            query.push(("cat", format!("{:?}", cat)));
        }

        if let Some(bonus) = params.bonus {
            query.push(("bonus", format!("{:?}", bonus)));
        }

        if let Some(sort) = params.sort {
            query.push(("sort", sort.to_string()));
        }

        if let Some(offset) = params.offset {
            query.push(("offset", offset.to_string()));
        }

        if let Some(limit) = params.limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(timestamp) = params.timestamp {
            query.push(("timestamp", timestamp));
        }

        self.client.request_paginated("/market", &query).await
    }

    /// Get auction house listings.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering auction listings
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn auction_house(&self, params: AuctionHouseParams) -> Result<PaginatedResponse<AuctionHouseResponse>, Error> {
        let mut query = Vec::new();

        if let Some(limit) = params.limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(sort) = params.sort {
            query.push(("sort", sort.to_string()));
        }

        if let Some(from) = params.from {
            query.push(("from", from.to_string()));
        }

        if let Some(to) = params.to {
            query.push(("to", to.to_string()));
        }

        if let Some(timestamp) = params.timestamp {
            query.push(("timestamp", timestamp));
        }

        self.client.request_paginated("/market/auctionhouse", &query).await
    }

    /// Get bazaar directory.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering bazaar listings
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn bazaar(&self, params: BazaarParams) -> Result<PaginatedResponse<BazaarResponse>, Error> {
        let mut query = Vec::new();

        if let Some(cat) = params.cat {
            query.push(("cat", format!("{:?}", cat)));
        }

        if let Some(timestamp) = params.timestamp {
            query.push(("timestamp", timestamp));
        }

        self.client.request_paginated("/market/bazaar", &query).await
    }

    /// Get all available market selections.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Optional timestamp to bypass cache
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn lookup(&self, timestamp: Option<String>) -> Result<PaginatedResponse<MarketLookupResponse>, Error> {
        let mut query = Vec::new();

        if let Some(ts) = timestamp {
            query.push(("timestamp", ts));
        }

        self.client.request_paginated("/market/lookup", &query).await
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
    pub async fn timestamp(&self, timestamp: Option<String>) -> Result<PaginatedResponse<torn_models::generated::common::TimestampResponse>, Error> {
        let mut query = Vec::new();

        if let Some(ts) = timestamp {
            query.push(("timestamp", ts));
        }

        self.client.request_paginated("/market/timestamp", &query).await
    }

    /// Access endpoints for a specific item by ID.
    pub fn with_item_id(&self, id: ItemId) -> MarketItemIdContext<'a> {
        MarketItemIdContext {
            client: self.client,
            id,
        }
    }

    /// Access endpoints for a specific auction listing by ID.
    pub fn with_auction_id(&self, id: AuctionListingId) -> MarketAuctionIdContext<'a> {
        MarketAuctionIdContext {
            client: self.client,
            id,
        }
    }

    /// Access endpoints for a specific property type by ID.
    pub fn with_property_type_id(&self, id: PropertyTypeId) -> MarketPropertyTypeIdContext<'a> {
        MarketPropertyTypeIdContext {
            client: self.client,
            id,
        }
    }
}

/// Market API endpoints scoped to a specific item ID.
pub struct MarketItemIdContext<'a> {
    client: &'a TornClient,
    id: ItemId,
}

impl<'a> MarketItemIdContext<'a> {
    /// Get auction house listings for this specific item.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering auction listings
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn auction_house(&self, params: AuctionHouseParams) -> Result<PaginatedResponse<AuctionHouseResponse>, Error> {
        let path = format!("/market/{}/auctionhouse", self.id);
        let mut query = Vec::new();

        if let Some(limit) = params.limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(sort) = params.sort {
            query.push(("sort", sort.to_string()));
        }

        if let Some(from) = params.from {
            query.push(("from", from.to_string()));
        }

        if let Some(to) = params.to {
            query.push(("to", to.to_string()));
        }

        if let Some(timestamp) = params.timestamp {
            query.push(("timestamp", timestamp));
        }

        self.client.request_paginated(&path, &query).await
    }

    /// Get item specialized bazaar directory for this item.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Optional timestamp to bypass cache
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn bazaar(&self, timestamp: Option<String>) -> Result<PaginatedResponse<BazaarResponseSpecialized>, Error> {
        let path = format!("/market/{}/bazaar", self.id);
        let mut query = Vec::new();

        if let Some(ts) = timestamp {
            query.push(("timestamp", ts));
        }

        self.client.request_paginated(&path, &query).await
    }

    /// Get item market listings for this item.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering item market listings
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn item_market(&self, params: ItemMarketParams) -> Result<PaginatedResponse<MarketItemMarketResponse>, Error> {
        let path = format!("/market/{}/itemmarket", self.id);
        let mut query = Vec::new();

        if let Some(bonus) = params.bonus {
            query.push(("bonus", format!("{:?}", bonus)));
        }

        if let Some(limit) = params.limit {
            query.push(("limit", limit.to_string()));
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

/// Market API endpoints scoped to a specific auction listing ID.
pub struct MarketAuctionIdContext<'a> {
    client: &'a TornClient,
    id: AuctionListingId,
}

impl<'a> MarketAuctionIdContext<'a> {
    /// Get specific auction house listing details.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Optional timestamp to bypass cache
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get(&self, timestamp: Option<String>) -> Result<PaginatedResponse<AuctionHouseListing>, Error> {
        let path = format!("/market/{}/auctionhouselisting", self.id);
        let mut query = Vec::new();

        if let Some(ts) = timestamp {
            query.push(("timestamp", ts));
        }

        self.client.request_paginated(&path, &query).await
    }
}

/// Market API endpoints scoped to a specific property type ID.
pub struct MarketPropertyTypeIdContext<'a> {
    client: &'a TornClient,
    id: PropertyTypeId,
}

impl<'a> MarketPropertyTypeIdContext<'a> {
    /// Get properties market listings for this property type.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering property listings
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn properties(&self, params: PropertyParams) -> Result<PaginatedResponse<MarketPropertiesResponse>, Error> {
        let path = format!("/market/{}/properties", self.id);
        let mut query = Vec::new();

        if let Some(offset) = params.offset {
            query.push(("offset", offset.to_string()));
        }

        if let Some(limit) = params.limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(sort) = params.sort {
            query.push(("sort", sort.to_string()));
        }

        if let Some(timestamp) = params.timestamp {
            query.push(("timestamp", timestamp));
        }

        self.client.request_paginated(&path, &query).await
    }

    /// Get properties rental listings for this property type.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering rental listings
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn rentals(&self, params: PropertyParams) -> Result<PaginatedResponse<MarketRentalsResponse>, Error> {
        let path = format!("/market/{}/rentals", self.id);
        let mut query = Vec::new();

        if let Some(offset) = params.offset {
            query.push(("offset", offset.to_string()));
        }

        if let Some(limit) = params.limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(sort) = params.sort {
            query.push(("sort", sort.to_string()));
        }

        if let Some(timestamp) = params.timestamp {
            query.push(("timestamp", timestamp));
        }

        self.client.request_paginated(&path, &query).await
    }
}

/// Parameters for the market get endpoint.
#[derive(Debug, Default, Clone)]
pub struct MarketGetParams {
    /// Selection names
    pub selections: Option<Vec<String>>,
    /// Selection id (can be ItemId, AuctionListingId, or PropertyTypeId)
    pub id: Option<i64>,
    /// Legacy selection names for which you want or expect API v1 response
    pub legacy: Option<Vec<String>>,
    /// Category of specialized bazaars returned
    pub cat: Option<MarketSpecializedBazaarCategoryEnum>,
    /// Used to filter weapons with a specific bonus
    pub bonus: Option<WeaponBonusEnum>,
    /// Direction to sort rows in (DESC or ASC)
    pub sort: Option<String>,
    /// Pagination offset
    pub offset: Option<i32>,
    /// Pagination limit (default: 20, max: 100)
    pub limit: Option<i32>,
    /// Timestamp to bypass cache
    pub timestamp: Option<String>,
}

/// Parameters for the auction house endpoint.
#[derive(Debug, Default, Clone)]
pub struct AuctionHouseParams {
    /// Pagination limit (default: 20, max: 100)
    pub limit: Option<i32>,
    /// Sorted by the greatest timestamps (DESC or ASC)
    pub sort: Option<String>,
    /// Timestamp that sets the lower limit for the data returned
    pub from: Option<i32>,
    /// Timestamp that sets the upper limit for the data returned
    pub to: Option<i32>,
    /// Timestamp to bypass cache
    pub timestamp: Option<String>,
}

/// Parameters for the bazaar endpoint.
#[derive(Debug, Default, Clone)]
pub struct BazaarParams {
    /// Category of specialized bazaars returned
    pub cat: Option<MarketSpecializedBazaarCategoryEnum>,
    /// Timestamp to bypass cache
    pub timestamp: Option<String>,
}

/// Parameters for the item market endpoint.
#[derive(Debug, Default, Clone)]
pub struct ItemMarketParams {
    /// Used to filter weapons with a specific bonus
    pub bonus: Option<WeaponBonusEnum>,
    /// Pagination limit (default: 20, max: 100)
    pub limit: Option<i32>,
    /// Pagination offset (default: 0)
    pub offset: Option<i32>,
    /// Timestamp to bypass cache
    pub timestamp: Option<String>,
}

/// Parameters for property endpoints.
#[derive(Debug, Default, Clone)]
pub struct PropertyParams {
    /// Pagination offset (default: 0)
    pub offset: Option<i32>,
    /// Pagination limit (default: 20, max: 100)
    pub limit: Option<i32>,
    /// Sorted by the greatest timestamps (DESC or ASC)
    pub sort: Option<String>,
    /// Timestamp to bypass cache
    pub timestamp: Option<String>,
}
