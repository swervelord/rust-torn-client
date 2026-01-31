//! Torn API endpoints.
//!
//! This module provides typed methods for all endpoints under the `/torn` API tag.
//! These endpoints provide general Torn game information, items, crimes, honors, and more.

use crate::pagination::PaginatedResponse;
use crate::{Error, TornClient};
use torn_models::generated::common::TimestampResponse;
use torn_models::generated::torn::*;

/// Torn API endpoints (no ID required).
pub struct TornEndpoint<'a> {
    client: &'a TornClient,
}

impl<'a> TornEndpoint<'a> {
    pub(crate) fn new(client: &'a TornClient) -> Self {
        Self { client }
    }

    /// Get any Torn selection.
    ///
    /// This is the generic torn endpoint that can return various data based on selections.
    pub async fn selections(&self) -> Result<PaginatedResponse<TornSubcrimesResponse>, Error> {
        self.client.request_paginated("/torn", &[]).await
    }

    /// Get attack log details.
    ///
    /// Requires a log code to retrieve the attack log.
    pub async fn attacklog(&self, log_code: &str) -> Result<PaginatedResponse<AttackLogResponse>, Error> {
        self.client
            .request_paginated("/torn/attacklog", &[("log", log_code.to_string())])
            .await
    }

    /// Get bounties.
    ///
    /// Returns a list of active bounties in the game.
    pub async fn bounties(&self) -> Result<PaginatedResponse<TornBountiesResponse>, Error> {
        self.client.request_paginated("/torn/bounties", &[]).await
    }

    /// Get calendar information.
    ///
    /// Returns information about game events and schedules.
    pub async fn calendar(&self) -> Result<PaginatedResponse<TornCalendarResponse>, Error> {
        self.client.request_paginated("/torn/calendar", &[]).await
    }

    /// Get crimes information.
    ///
    /// Returns information about crimes in the game.
    pub async fn crimes(&self) -> Result<PaginatedResponse<TornCrimesResponse>, Error> {
        self.client.request_paginated("/torn/crimes", &[]).await
    }

    /// Get education information.
    ///
    /// Returns information about available education courses.
    pub async fn education(&self) -> Result<PaginatedResponse<TornEducationResponse>, Error> {
        self.client.request_paginated("/torn/education", &[]).await
    }

    /// Get current standings for all elimination teams.
    ///
    /// Returns the current elimination competition standings.
    pub async fn elimination(&self) -> Result<PaginatedResponse<TornEliminationTeamsResponse>, Error> {
        self.client.request_paginated("/torn/elimination", &[]).await
    }

    /// Get faction hall of fame positions for a specific category.
    ///
    /// Returns faction hall of fame rankings.
    pub async fn factionhof(&self) -> Result<PaginatedResponse<TornFactionHofResponse>, Error> {
        self.client.request_paginated("/torn/factionhof", &[]).await
    }

    /// Get full faction tree.
    ///
    /// Returns the complete faction upgrade tree.
    pub async fn factiontree(&self) -> Result<PaginatedResponse<TornFactionTreeResponse>, Error> {
        self.client.request_paginated("/torn/factiontree", &[]).await
    }

    /// Get player hall of fame positions for a specific category.
    ///
    /// Returns player hall of fame rankings.
    pub async fn hof(&self) -> Result<PaginatedResponse<TornHofResponse>, Error> {
        self.client.request_paginated("/torn/hof", &[]).await
    }

    /// Get all honors.
    ///
    /// Returns information about all honors in the game.
    pub async fn honors(&self) -> Result<PaginatedResponse<TornHonorsResponse>, Error> {
        self.client.request_paginated("/torn/honors", &[]).await
    }

    /// Get information about ammo.
    ///
    /// Returns details about ammunition types.
    pub async fn itemammo(&self) -> Result<PaginatedResponse<TornItemAmmoResponse>, Error> {
        self.client.request_paginated("/torn/itemammo", &[]).await
    }

    /// Get information about weapon upgrades.
    ///
    /// Returns details about weapon modification items.
    pub async fn itemmods(&self) -> Result<PaginatedResponse<TornItemModsResponse>, Error> {
        self.client.request_paginated("/torn/itemmods", &[]).await
    }

    /// Get information about items.
    ///
    /// Returns information about all items in the game.
    pub async fn items(&self) -> Result<PaginatedResponse<TornItemsResponse>, Error> {
        self.client.request_paginated("/torn/items", &[]).await
    }

    /// Get available log categories.
    ///
    /// Returns a list of log categories that can be queried.
    pub async fn logcategories(&self) -> Result<PaginatedResponse<TornLogCategoriesResponse>, Error> {
        self.client.request_paginated("/torn/logcategories", &[]).await
    }

    /// Get all available log ids.
    ///
    /// Returns a list of all log type identifiers.
    pub async fn logtypes(&self) -> Result<PaginatedResponse<TornLogTypesResponse>, Error> {
        self.client.request_paginated("/torn/logtypes", &[]).await
    }

    /// Get all available torn selections.
    ///
    /// Returns metadata about available API selections for the torn endpoint.
    pub async fn lookup(&self) -> Result<PaginatedResponse<TornLookupResponse>, Error> {
        self.client.request_paginated("/torn/lookup", &[]).await
    }

    /// Get all medals.
    ///
    /// Returns information about all medals in the game.
    pub async fn medals(&self) -> Result<PaginatedResponse<TornMedalsResponse>, Error> {
        self.client.request_paginated("/torn/medals", &[]).await
    }

    /// Get all merits.
    ///
    /// Returns information about all merits in the game.
    pub async fn merits(&self) -> Result<PaginatedResponse<TornMeritsResponse>, Error> {
        self.client.request_paginated("/torn/merits", &[]).await
    }

    /// Get organized crimes information.
    ///
    /// Returns information about organized crimes.
    pub async fn organizedcrimes(&self) -> Result<PaginatedResponse<TornOrganizedCrimeResponse>, Error> {
        self.client.request_paginated("/torn/organizedcrimes", &[]).await
    }

    /// Get properties details.
    ///
    /// Returns information about properties in the game.
    pub async fn properties(&self) -> Result<PaginatedResponse<TornProperties>, Error> {
        self.client.request_paginated("/torn/properties", &[]).await
    }

    /// Get territory details.
    ///
    /// Returns information about faction territories.
    pub async fn territory(&self) -> Result<PaginatedResponse<TornTerritoriesResponse>, Error> {
        self.client.request_paginated("/torn/territory", &[]).await
    }

    /// Get current server time.
    ///
    /// Returns the current Torn server timestamp.
    pub async fn timestamp(&self) -> Result<PaginatedResponse<TimestampResponse>, Error> {
        self.client.request_paginated("/torn/timestamp", &[]).await
    }

    // Context methods for ID-scoped endpoints

    /// Access endpoints for a specific crime ID.
    pub fn crime(&self, crime_id: u32) -> TornCrimeContext<'a> {
        TornCrimeContext {
            client: self.client,
            crime_id,
        }
    }

    /// Access endpoints for specific honor IDs.
    pub fn honors_by_ids(&self, ids: Vec<u32>) -> TornHonorsContext<'a> {
        TornHonorsContext {
            client: self.client,
            ids,
        }
    }

    /// Access endpoints for specific item IDs.
    pub fn items_by_ids(&self, ids: Vec<u32>) -> TornItemsContext<'a> {
        TornItemsContext {
            client: self.client,
            ids,
        }
    }

    /// Access endpoints for specific medal IDs.
    pub fn medals_by_ids(&self, ids: Vec<u32>) -> TornMedalsContext<'a> {
        TornMedalsContext {
            client: self.client,
            ids,
        }
    }

    /// Access endpoints for a specific elimination team ID.
    pub fn elimination_team(&self, team_id: u32) -> TornEliminationTeamContext<'a> {
        TornEliminationTeamContext {
            client: self.client,
            team_id,
        }
    }

    /// Access endpoints for a specific item details by UID.
    pub fn item_details(&self, item_uid: u64) -> TornItemDetailsContext<'a> {
        TornItemDetailsContext {
            client: self.client,
            item_uid,
        }
    }

    /// Access endpoints for a specific log category ID.
    pub fn log_category(&self, category_id: u32) -> TornLogCategoryContext<'a> {
        TornLogCategoryContext {
            client: self.client,
            category_id,
        }
    }
}

/// Torn API endpoints scoped to a specific crime ID.
pub struct TornCrimeContext<'a> {
    client: &'a TornClient,
    crime_id: u32,
}

impl<'a> TornCrimeContext<'a> {
    /// Get subcrimes information for this crime.
    pub async fn subcrimes(&self) -> Result<PaginatedResponse<TornSubcrimesResponse>, Error> {
        let path = format!("/torn/{}/subcrimes", self.crime_id);
        self.client.request_paginated(&path, &[]).await
    }
}

/// Torn API endpoints scoped to specific honor IDs.
pub struct TornHonorsContext<'a> {
    client: &'a TornClient,
    ids: Vec<u32>,
}

impl<'a> TornHonorsContext<'a> {
    /// Get specific honors by IDs.
    pub async fn get(&self) -> Result<PaginatedResponse<TornHonorsResponse>, Error> {
        let ids_str = self
            .ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let path = format!("/torn/{}/honors", ids_str);
        self.client.request_paginated(&path, &[]).await
    }
}

/// Torn API endpoints scoped to specific item IDs.
pub struct TornItemsContext<'a> {
    client: &'a TornClient,
    ids: Vec<u32>,
}

impl<'a> TornItemsContext<'a> {
    /// Get information about specific items by IDs.
    pub async fn get(&self) -> Result<PaginatedResponse<TornItemsResponse>, Error> {
        let ids_str = self
            .ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let path = format!("/torn/{}/items", ids_str);
        self.client.request_paginated(&path, &[]).await
    }
}

/// Torn API endpoints scoped to specific medal IDs.
pub struct TornMedalsContext<'a> {
    client: &'a TornClient,
    ids: Vec<u32>,
}

impl<'a> TornMedalsContext<'a> {
    /// Get specific medals by IDs.
    pub async fn get(&self) -> Result<PaginatedResponse<TornMedalsResponse>, Error> {
        let ids_str = self
            .ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let path = format!("/torn/{}/medals", ids_str);
        self.client.request_paginated(&path, &[]).await
    }
}

/// Torn API endpoints scoped to a specific elimination team ID.
pub struct TornEliminationTeamContext<'a> {
    client: &'a TornClient,
    team_id: u32,
}

impl<'a> TornEliminationTeamContext<'a> {
    /// Get players in this elimination team.
    pub async fn players(&self) -> Result<PaginatedResponse<TornEliminationTeamPlayersResponse>, Error> {
        let path = format!("/torn/{}/eliminationteam", self.team_id);
        self.client.request_paginated(&path, &[]).await
    }
}

/// Torn API endpoints scoped to a specific item UID.
pub struct TornItemDetailsContext<'a> {
    client: &'a TornClient,
    item_uid: u64,
}

impl<'a> TornItemDetailsContext<'a> {
    /// Get detailed information about this specific item instance.
    pub async fn get(&self) -> Result<PaginatedResponse<TornItemDetailsResponse>, Error> {
        let path = format!("/torn/{}/itemdetails", self.item_uid);
        self.client.request_paginated(&path, &[]).await
    }
}

/// Torn API endpoints scoped to a specific log category ID.
pub struct TornLogCategoryContext<'a> {
    client: &'a TornClient,
    category_id: u32,
}

impl<'a> TornLogCategoryContext<'a> {
    /// Get available log IDs for this log category.
    pub async fn logtypes(&self) -> Result<PaginatedResponse<TornLogTypesResponse>, Error> {
        let path = format!("/torn/{}/logtypes", self.category_id);
        self.client.request_paginated(&path, &[]).await
    }
}
