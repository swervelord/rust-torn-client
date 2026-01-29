//! Racing API endpoints.
//!
//! Provides access to all racing-related endpoints in the Torn API.
//! Endpoints are organized into:
//! - Base racing endpoints (no ID required) - accessed via `client.racing()`
//! - Race ID-scoped endpoints - accessed via `client.racing().with_race_id(race_id)`
//! - Track ID-scoped endpoints - accessed via `client.racing().with_track_id(track_id)`

use crate::{Error, PaginatedResponse, TornClient};
use torn_models::generated::common::TimestampResponse;
use torn_models::generated::racing::*;

/// Racing API endpoints (base endpoints, no ID required).
pub struct RacingEndpoint<'a> {
    client: &'a TornClient,
}

impl<'a> RacingEndpoint<'a> {
    pub(crate) fn new(client: &'a TornClient) -> Self {
        Self { client }
    }

    /// Get any Racing selection.
    ///
    /// Endpoint: `/racing`
    pub async fn get(&self) -> Result<PaginatedResponse<RacingRacesResponse>, Error> {
        self.client.request_paginated("/racing", &[]).await
    }

    /// Get cars and their racing stats.
    ///
    /// Endpoint: `/racing/cars`
    pub async fn cars(&self) -> Result<PaginatedResponse<RacingCarsResponse>, Error> {
        self.client.request_paginated("/racing/cars", &[]).await
    }

    /// Get car upgrades available in racing.
    ///
    /// Endpoint: `/racing/carupgrades`
    pub async fn car_upgrades(&self) -> Result<PaginatedResponse<RacingCarUpgradesResponse>, Error> {
        self.client.request_paginated("/racing/carupgrades", &[]).await
    }

    /// Get available racing selections for lookup.
    ///
    /// Endpoint: `/racing/lookup`
    pub async fn lookup(&self) -> Result<PaginatedResponse<RacingLookupResponse>, Error> {
        self.client.request_paginated("/racing/lookup", &[]).await
    }

    /// Get racing races.
    ///
    /// Endpoint: `/racing/races`
    pub async fn races(&self) -> Result<PaginatedResponse<RacingRacesResponse>, Error> {
        self.client.request_paginated("/racing/races", &[]).await
    }

    /// Get current server timestamp.
    ///
    /// Endpoint: `/racing/timestamp`
    pub async fn timestamp(&self) -> Result<PaginatedResponse<TimestampResponse>, Error> {
        self.client.request_paginated("/racing/timestamp", &[]).await
    }

    /// Get racing tracks.
    ///
    /// Endpoint: `/racing/tracks`
    pub async fn tracks(&self) -> Result<PaginatedResponse<RacingTracksResponse>, Error> {
        self.client.request_paginated("/racing/tracks", &[]).await
    }

    /// Access endpoints for a specific race by ID.
    pub fn with_race_id(&self, race_id: u64) -> RacingRaceIdContext<'a> {
        RacingRaceIdContext {
            client: self.client,
            race_id,
        }
    }

    /// Access endpoints for a specific track by ID.
    pub fn with_track_id(&self, track_id: u64) -> RacingTrackIdContext<'a> {
        RacingTrackIdContext {
            client: self.client,
            track_id,
        }
    }
}

/// Racing API endpoints scoped to a specific race ID.
pub struct RacingRaceIdContext<'a> {
    client: &'a TornClient,
    race_id: u64,
}

impl<'a> RacingRaceIdContext<'a> {
    /// Get race details for this specific race.
    ///
    /// Endpoint: `/racing/{raceId}/race`
    pub async fn race(&self) -> Result<PaginatedResponse<RacingRaceDetailsResponse>, Error> {
        let path = format!("/racing/{}/race", self.race_id);
        self.client.request_paginated(&path, &[]).await
    }
}

/// Racing API endpoints scoped to a specific track ID.
pub struct RacingTrackIdContext<'a> {
    client: &'a TornClient,
    track_id: u64,
}

impl<'a> RacingTrackIdContext<'a> {
    /// Get records for this specific track.
    ///
    /// Endpoint: `/racing/{trackId}/records`
    pub async fn records(&self) -> Result<PaginatedResponse<RacingTrackRecordsResponse>, Error> {
        let path = format!("/racing/{}/records", self.track_id);
        self.client.request_paginated(&path, &[]).await
    }
}
