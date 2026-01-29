//! Faction API endpoints.
//!
//! Provides typed methods for all faction-related API endpoints,
//! including self-scoped (your faction), ID-scoped (any faction),
//! and special report endpoints.

use crate::pagination::PaginatedResponse;
use crate::{Error, TornClient};
use torn_models::generated::common::{
    AttacksFullResponse, AttacksResponse, ReportsResponse, RevivesFullResponse, RevivesResponse,
    TimestampResponse,
};
use torn_models::generated::faction::*;

/// Faction API endpoints (self-scoped, no ID required).
///
/// Access your own faction's data or search for factions.
pub struct FactionEndpoint<'a> {
    client: &'a TornClient,
}

impl<'a> FactionEndpoint<'a> {
    pub(crate) fn new(client: &'a TornClient) -> Self {
        Self { client }
    }

    // =========================================================================
    // Self-scoped endpoints (your faction)
    // =========================================================================

    /// Get any Faction selection.
    ///
    /// # Endpoint
    /// `GET /faction`
    pub async fn get(&self) -> Result<PaginatedResponse<FactionHofResponse>, Error> {
        self.client.request_paginated("/faction", &[]).await
    }

    /// Get your faction's applications.
    ///
    /// # Endpoint
    /// `GET /faction/applications`
    pub async fn applications(&self) -> Result<PaginatedResponse<FactionApplicationsResponse>, Error> {
        self.client
            .request_paginated("/faction/applications", &[])
            .await
    }

    /// Get your faction's detailed attacks.
    ///
    /// # Endpoint
    /// `GET /faction/attacks`
    pub async fn attacks(&self) -> Result<PaginatedResponse<AttacksResponse>, Error> {
        self.client
            .request_paginated("/faction/attacks", &[])
            .await
    }

    /// Get your faction's simplified attacks.
    ///
    /// # Endpoint
    /// `GET /faction/attacksfull`
    pub async fn attacks_full(&self) -> Result<PaginatedResponse<AttacksFullResponse>, Error> {
        self.client
            .request_paginated("/faction/attacksfull", &[])
            .await
    }

    /// Get your faction's & member's balance details.
    ///
    /// # Endpoint
    /// `GET /faction/balance`
    pub async fn balance(&self) -> Result<PaginatedResponse<FactionBalanceResponse>, Error> {
        self.client
            .request_paginated("/faction/balance", &[])
            .await
    }

    /// Get your faction's basic details.
    ///
    /// # Endpoint
    /// `GET /faction/basic`
    pub async fn basic(&self) -> Result<PaginatedResponse<FactionBasicResponse>, Error> {
        self.client
            .request_paginated("/faction/basic", &[])
            .await
    }

    /// Get your faction's current chain.
    ///
    /// # Endpoint
    /// `GET /faction/chain`
    pub async fn chain(&self) -> Result<PaginatedResponse<FactionOngoingChainResponse>, Error> {
        self.client
            .request_paginated("/faction/chain", &[])
            .await
    }

    /// Get your faction's latest chain report.
    ///
    /// # Endpoint
    /// `GET /faction/chainreport`
    pub async fn chain_report(&self) -> Result<PaginatedResponse<FactionChainReportResponse>, Error> {
        self.client
            .request_paginated("/faction/chainreport", &[])
            .await
    }

    /// Get a list of your faction's completed chains.
    ///
    /// # Endpoint
    /// `GET /faction/chains`
    pub async fn chains(&self) -> Result<PaginatedResponse<FactionChainsResponse>, Error> {
        self.client
            .request_paginated("/faction/chains", &[])
            .await
    }

    /// Get your faction's challenge contributors.
    ///
    /// # Endpoint
    /// `GET /faction/contributors`
    pub async fn contributors(&self) -> Result<PaginatedResponse<FactionContributorsResponse>, Error> {
        self.client
            .request_paginated("/faction/contributors", &[])
            .await
    }

    /// Get your faction's organized crimes.
    ///
    /// # Endpoint
    /// `GET /faction/crimes`
    pub async fn crimes(&self) -> Result<PaginatedResponse<FactionCrimesResponse>, Error> {
        self.client
            .request_paginated("/faction/crimes", &[])
            .await
    }

    /// Get your faction's hall of fame rankings.
    ///
    /// # Endpoint
    /// `GET /faction/hof`
    pub async fn hof(&self) -> Result<PaginatedResponse<FactionHofResponse>, Error> {
        self.client.request_paginated("/faction/hof", &[]).await
    }

    /// Get faction lookup data.
    ///
    /// # Endpoint
    /// `GET /faction/lookup`
    pub async fn lookup(&self) -> Result<PaginatedResponse<FactionLookupResponse>, Error> {
        self.client
            .request_paginated("/faction/lookup", &[])
            .await
    }

    /// Get a list of your faction's members.
    ///
    /// # Endpoint
    /// `GET /faction/members`
    pub async fn members(&self) -> Result<PaginatedResponse<FactionMembersResponse>, Error> {
        self.client
            .request_paginated("/faction/members", &[])
            .await
    }

    /// Get your faction's news details.
    ///
    /// # Endpoint
    /// `GET /faction/news`
    pub async fn news(&self) -> Result<PaginatedResponse<FactionNewsResponse>, Error> {
        self.client.request_paginated("/faction/news", &[]).await
    }

    /// Get your faction's positions details.
    ///
    /// # Endpoint
    /// `GET /faction/positions`
    pub async fn positions(&self) -> Result<PaginatedResponse<FactionPositionsResponse>, Error> {
        self.client
            .request_paginated("/faction/positions", &[])
            .await
    }

    /// Get a list of current rackets.
    ///
    /// # Endpoint
    /// `GET /faction/rackets`
    pub async fn rackets(&self) -> Result<PaginatedResponse<FactionRacketsResponse>, Error> {
        self.client
            .request_paginated("/faction/rackets", &[])
            .await
    }

    /// Get raids history for your faction.
    ///
    /// # Endpoint
    /// `GET /faction/raids`
    pub async fn raids(&self) -> Result<PaginatedResponse<FactionRaidsResponse>, Error> {
        self.client.request_paginated("/faction/raids", &[]).await
    }

    /// Get ranked wars history for your faction.
    ///
    /// # Endpoint
    /// `GET /faction/rankedwars`
    pub async fn ranked_wars(&self) -> Result<PaginatedResponse<FactionRankedWarResponse>, Error> {
        self.client
            .request_paginated("/faction/rankedwars", &[])
            .await
    }

    /// Get faction reports.
    ///
    /// # Endpoint
    /// `GET /faction/reports`
    pub async fn reports(&self) -> Result<PaginatedResponse<ReportsResponse>, Error> {
        self.client
            .request_paginated("/faction/reports", &[])
            .await
    }

    /// Get your faction's detailed revives.
    ///
    /// # Endpoint
    /// `GET /faction/revives`
    pub async fn revives(&self) -> Result<PaginatedResponse<RevivesResponse>, Error> {
        self.client
            .request_paginated("/faction/revives", &[])
            .await
    }

    /// Get your faction's simplified revives.
    ///
    /// # Endpoint
    /// `GET /faction/revivesFull`
    pub async fn revives_full(&self) -> Result<PaginatedResponse<RevivesFullResponse>, Error> {
        self.client
            .request_paginated("/faction/revivesFull", &[])
            .await
    }

    /// Search factions by name or other criteria.
    ///
    /// # Endpoint
    /// `GET /faction/search`
    pub async fn search(&self) -> Result<PaginatedResponse<FactionSearchResponse>, Error> {
        self.client
            .request_paginated("/faction/search", &[])
            .await
    }

    /// Get your faction's challenges stats.
    ///
    /// # Endpoint
    /// `GET /faction/stats`
    pub async fn stats(&self) -> Result<PaginatedResponse<FactionStatsResponse>, Error> {
        self.client.request_paginated("/faction/stats", &[]).await
    }

    /// Get a list of your faction's territories.
    ///
    /// # Endpoint
    /// `GET /faction/territory`
    pub async fn territory(&self) -> Result<PaginatedResponse<FactionTerritoriesResponse>, Error> {
        self.client
            .request_paginated("/faction/territory", &[])
            .await
    }

    /// Get a list territory ownership.
    ///
    /// # Endpoint
    /// `GET /faction/territoryownership`
    pub async fn territory_ownership(&self) -> Result<PaginatedResponse<FactionTerritoriesOwnershipResponse>, Error> {
        self.client
            .request_paginated("/faction/territoryownership", &[])
            .await
    }

    /// Get territory wars history for your faction.
    ///
    /// # Endpoint
    /// `GET /faction/territorywars`
    pub async fn territory_wars(&self) -> Result<PaginatedResponse<FactionTerritoryWarsHistoryResponse>, Error> {
        self.client
            .request_paginated("/faction/territorywars", &[])
            .await
    }

    /// Get current server time.
    ///
    /// # Endpoint
    /// `GET /faction/timestamp`
    pub async fn timestamp(&self) -> Result<PaginatedResponse<TimestampResponse>, Error> {
        self.client
            .request_paginated("/faction/timestamp", &[])
            .await
    }

    /// Get your faction's upgrades.
    ///
    /// # Endpoint
    /// `GET /faction/upgrades`
    pub async fn upgrades(&self) -> Result<PaginatedResponse<FactionUpgradesResponse>, Error> {
        self.client
            .request_paginated("/faction/upgrades", &[])
            .await
    }

    /// Get faction warfare.
    ///
    /// # Endpoint
    /// `GET /faction/warfare`
    pub async fn warfare(&self) -> Result<PaginatedResponse<FactionWarfareResponse>, Error> {
        self.client
            .request_paginated("/faction/warfare", &[])
            .await
    }

    /// Get your faction's wars & pacts details.
    ///
    /// # Endpoint
    /// `GET /faction/wars`
    pub async fn wars(&self) -> Result<PaginatedResponse<FactionWarsResponse>, Error> {
        self.client.request_paginated("/faction/wars", &[]).await
    }

    // =========================================================================
    // Context builders
    // =========================================================================

    /// Access endpoints for a specific faction by ID.
    ///
    /// # Example
    /// ```no_run
    /// # use torn_client::TornClient;
    /// # async fn example(client: TornClient) -> Result<(), torn_client::Error> {
    /// let basic = client.faction().with_id(12345).basic().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_id(&self, id: u64) -> FactionIdContext<'a> {
        FactionIdContext {
            client: self.client,
            id,
        }
    }

    /// Access a specific chain report by chain ID.
    ///
    /// # Example
    /// ```no_run
    /// # use torn_client::TornClient;
    /// # async fn example(client: TornClient) -> Result<(), torn_client::Error> {
    /// let report = client.faction().chain_report_with_id(123).get().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn chain_report_with_id(&self, chain_id: u64) -> FactionChainReportContext<'a> {
        FactionChainReportContext {
            client: self.client,
            chain_id,
        }
    }

    /// Access a specific organized crime by crime ID.
    ///
    /// # Example
    /// ```no_run
    /// # use torn_client::TornClient;
    /// # async fn example(client: TornClient) -> Result<(), torn_client::Error> {
    /// let crime = client.faction().crime_with_id(456).get().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn crime_with_id(&self, crime_id: u64) -> FactionCrimeContext<'a> {
        FactionCrimeContext {
            client: self.client,
            crime_id,
        }
    }

    /// Access a specific raid war report by raid war ID.
    ///
    /// # Example
    /// ```no_run
    /// # use torn_client::TornClient;
    /// # async fn example(client: TornClient) -> Result<(), torn_client::Error> {
    /// let report = client.faction().raid_report_with_id(789).get().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn raid_report_with_id(&self, raid_war_id: u64) -> FactionRaidReportContext<'a> {
        FactionRaidReportContext {
            client: self.client,
            raid_war_id,
        }
    }

    /// Access a specific ranked war report by ranked war ID.
    ///
    /// # Example
    /// ```no_run
    /// # use torn_client::TornClient;
    /// # async fn example(client: TornClient) -> Result<(), torn_client::Error> {
    /// let report = client.faction().ranked_war_report_with_id(101).get().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn ranked_war_report_with_id(&self, ranked_war_id: u64) -> FactionRankedWarReportContext<'a> {
        FactionRankedWarReportContext {
            client: self.client,
            ranked_war_id,
        }
    }

    /// Access a specific territory war report by territory war ID.
    ///
    /// # Example
    /// ```no_run
    /// # use torn_client::TornClient;
    /// # async fn example(client: TornClient) -> Result<(), torn_client::Error> {
    /// let report = client.faction().territory_war_report_with_id(202).get().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn territory_war_report_with_id(&self, territory_war_id: u64) -> FactionTerritoryWarReportContext<'a> {
        FactionTerritoryWarReportContext {
            client: self.client,
            territory_war_id,
        }
    }
}

// =============================================================================
// ID-scoped context
// =============================================================================

/// Faction API endpoints scoped to a specific faction ID.
pub struct FactionIdContext<'a> {
    client: &'a TornClient,
    id: u64,
}

impl<'a> FactionIdContext<'a> {
    /// Get a faction's basic details.
    ///
    /// # Endpoint
    /// `GET /faction/{id}/basic`
    pub async fn basic(&self) -> Result<PaginatedResponse<FactionBasicResponse>, Error> {
        let path = format!("/faction/{}/basic", self.id);
        self.client.request_paginated(&path, &[]).await
    }

    /// Get a faction's current chain.
    ///
    /// # Endpoint
    /// `GET /faction/{id}/chain`
    pub async fn chain(&self) -> Result<PaginatedResponse<FactionOngoingChainResponse>, Error> {
        let path = format!("/faction/{}/chain", self.id);
        self.client.request_paginated(&path, &[]).await
    }

    /// Get a list of a faction's completed chains.
    ///
    /// # Endpoint
    /// `GET /faction/{id}/chains`
    pub async fn chains(&self) -> Result<PaginatedResponse<FactionChainsResponse>, Error> {
        let path = format!("/faction/{}/chains", self.id);
        self.client.request_paginated(&path, &[]).await
    }

    /// Get a faction's hall of fame rankings.
    ///
    /// # Endpoint
    /// `GET /faction/{id}/hof`
    pub async fn hof(&self) -> Result<PaginatedResponse<FactionHofResponse>, Error> {
        let path = format!("/faction/{}/hof", self.id);
        self.client.request_paginated(&path, &[]).await
    }

    /// Get a list of a faction's members.
    ///
    /// # Endpoint
    /// `GET /faction/{id}/members`
    pub async fn members(&self) -> Result<PaginatedResponse<FactionMembersResponse>, Error> {
        let path = format!("/faction/{}/members", self.id);
        self.client.request_paginated(&path, &[]).await
    }

    /// Get a faction's raids history.
    ///
    /// # Endpoint
    /// `GET /faction/{id}/raids`
    pub async fn raids(&self) -> Result<PaginatedResponse<FactionRaidsResponse>, Error> {
        let path = format!("/faction/{}/raids", self.id);
        self.client.request_paginated(&path, &[]).await
    }

    /// Get a faction's ranked wars history.
    ///
    /// # Endpoint
    /// `GET /faction/{id}/rankedwars`
    pub async fn ranked_wars(&self) -> Result<PaginatedResponse<FactionRankedWarResponse>, Error> {
        let path = format!("/faction/{}/rankedwars", self.id);
        self.client.request_paginated(&path, &[]).await
    }

    /// Get a list of a faction's territories.
    ///
    /// # Endpoint
    /// `GET /faction/{id}/territory`
    pub async fn territory(&self) -> Result<PaginatedResponse<FactionTerritoriesResponse>, Error> {
        let path = format!("/faction/{}/territory", self.id);
        self.client.request_paginated(&path, &[]).await
    }

    /// Get a faction's territory wars history.
    ///
    /// # Endpoint
    /// `GET /faction/{id}/territorywars`
    pub async fn territory_wars(&self) -> Result<PaginatedResponse<FactionTerritoryWarsHistoryResponse>, Error> {
        let path = format!("/faction/{}/territorywars", self.id);
        self.client.request_paginated(&path, &[]).await
    }

    /// Get a faction's wars & pacts details.
    ///
    /// # Endpoint
    /// `GET /faction/{id}/wars`
    pub async fn wars(&self) -> Result<PaginatedResponse<FactionWarsResponse>, Error> {
        let path = format!("/faction/{}/wars", self.id);
        self.client.request_paginated(&path, &[]).await
    }
}

// =============================================================================
// Special context structs for report endpoints
// =============================================================================

/// Context for chain report endpoints.
pub struct FactionChainReportContext<'a> {
    client: &'a TornClient,
    chain_id: u64,
}

impl<'a> FactionChainReportContext<'a> {
    /// Get a chain report.
    ///
    /// # Endpoint
    /// `GET /faction/{chainId}/chainreport`
    pub async fn get(&self) -> Result<PaginatedResponse<FactionChainReportResponse>, Error> {
        let path = format!("/faction/{}/chainreport", self.chain_id);
        self.client.request_paginated(&path, &[]).await
    }
}

/// Context for organized crime endpoints.
pub struct FactionCrimeContext<'a> {
    client: &'a TornClient,
    crime_id: u64,
}

impl<'a> FactionCrimeContext<'a> {
    /// Get a specific organized crime.
    ///
    /// # Endpoint
    /// `GET /faction/{crimeId}/crime`
    pub async fn get(&self) -> Result<PaginatedResponse<FactionCrimeResponse>, Error> {
        let path = format!("/faction/{}/crime", self.crime_id);
        self.client.request_paginated(&path, &[]).await
    }
}

/// Context for raid war report endpoints.
pub struct FactionRaidReportContext<'a> {
    client: &'a TornClient,
    raid_war_id: u64,
}

impl<'a> FactionRaidReportContext<'a> {
    /// Get raid war details.
    ///
    /// # Endpoint
    /// `GET /faction/{raidWarId}/raidreport`
    pub async fn get(&self) -> Result<PaginatedResponse<FactionRaidWarReportResponse>, Error> {
        let path = format!("/faction/{}/raidreport", self.raid_war_id);
        self.client.request_paginated(&path, &[]).await
    }
}

/// Context for ranked war report endpoints.
pub struct FactionRankedWarReportContext<'a> {
    client: &'a TornClient,
    ranked_war_id: u64,
}

impl<'a> FactionRankedWarReportContext<'a> {
    /// Get ranked war details.
    ///
    /// # Endpoint
    /// `GET /faction/{rankedWarId}/rankedwarreport`
    pub async fn get(&self) -> Result<PaginatedResponse<FactionRankedWarReportResponse>, Error> {
        let path = format!("/faction/{}/rankedwarreport", self.ranked_war_id);
        self.client.request_paginated(&path, &[]).await
    }
}

/// Context for territory war report endpoints.
pub struct FactionTerritoryWarReportContext<'a> {
    client: &'a TornClient,
    territory_war_id: u64,
}

impl<'a> FactionTerritoryWarReportContext<'a> {
    /// Get territory war details.
    ///
    /// # Endpoint
    /// `GET /faction/{territoryWarId}/territorywarreport`
    pub async fn get(&self) -> Result<PaginatedResponse<FactionTerritoryWarReportResponse>, Error> {
        let path = format!("/faction/{}/territorywarreport", self.territory_war_id);
        self.client.request_paginated(&path, &[]).await
    }
}
