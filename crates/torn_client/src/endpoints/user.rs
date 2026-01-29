//! User API endpoints.
//!
//! Provides access to all user-related endpoints in the Torn API.
//! Endpoints are organized into:
//! - Self-user endpoints (no ID required) - accessed via `client.user()`
//! - ID-scoped endpoints - accessed via `client.user().with_id(user_id)`
//! - Crime-specific endpoints - accessed via `client.user().with_crime_id(crime_id)`

use crate::{Error, PaginatedResponse, TornClient};
use torn_models::generated::common::{
    AttacksFullResponse, AttacksResponse, ReportsResponse, RevivesFullResponse, RevivesResponse,
    TimestampResponse,
};
use torn_models::generated::user::*;

/// User API endpoints (self-user, no ID required).
pub struct UserEndpoint<'a> {
    client: &'a TornClient,
}

impl<'a> UserEndpoint<'a> {
    pub(crate) fn new(client: &'a TornClient) -> Self {
        Self { client }
    }

    /// Get basic user information.
    pub async fn basic(&self) -> Result<UserBasicResponse, Error> {
        self.client.request("/user/basic", &[]).await
    }

    /// Get user ammo inventory.
    pub async fn ammo(&self) -> Result<UserAmmoResponse, Error> {
        self.client.request("/user/ammo", &[]).await
    }

    /// Get user attacks history.
    pub async fn attacks(&self) -> Result<PaginatedResponse<AttacksResponse>, Error> {
        self.client.request_paginated("/user/attacks", &[]).await
    }

    /// Get full user attacks history with additional details.
    pub async fn attacks_full(&self) -> Result<PaginatedResponse<AttacksFullResponse>, Error> {
        self.client.request_paginated("/user/attacksfull", &[]).await
    }

    /// Get user bars (energy, nerve, happy, life, chain).
    pub async fn bars(&self) -> Result<UserBarsResponse, Error> {
        self.client.request("/user/bars", &[]).await
    }

    /// Get user battle stats.
    pub async fn battle_stats(&self) -> Result<UserBattleStatsResponse, Error> {
        self.client.request("/user/battlestats", &[]).await
    }

    /// Get bounties on the user.
    pub async fn bounties(&self) -> Result<UserBountiesResponse, Error> {
        self.client.request("/user/bounties", &[]).await
    }

    /// Get user calendar information.
    pub async fn calendar(&self) -> Result<UserCalendarResponse, Error> {
        self.client.request("/user/calendar", &[]).await
    }

    /// Get user competition status.
    pub async fn competition(&self) -> Result<UserCompetitionResponse, Error> {
        self.client.request("/user/competition", &[]).await
    }

    /// Get user cooldowns.
    pub async fn cooldowns(&self) -> Result<UserCooldownsResponse, Error> {
        self.client.request("/user/cooldowns", &[]).await
    }

    /// Get user Discord information.
    pub async fn discord(&self) -> Result<UserDiscordResponse, Error> {
        self.client.request("/user/discord", &[]).await
    }

    /// Get user education information.
    pub async fn education(&self) -> Result<UserEducationResponse, Error> {
        self.client.request("/user/education", &[]).await
    }

    /// Get user's enlisted race cars.
    pub async fn enlisted_cars(&self) -> Result<UserEnlistedCarsResponse, Error> {
        self.client.request("/user/enlistedcars", &[]).await
    }

    /// Get user equipment and clothing.
    pub async fn equipment(&self) -> Result<UserEquipmentResponse, Error> {
        self.client.request("/user/equipment", &[]).await
    }

    /// Get user events.
    pub async fn events(&self) -> Result<PaginatedResponse<UserEventsResponse>, Error> {
        self.client.request_paginated("/user/events", &[]).await
    }

    /// Get user faction information.
    pub async fn faction(&self) -> Result<UserFactionResponse, Error> {
        self.client.request("/user/faction", &[]).await
    }

    /// Get user's forum feed.
    pub async fn forum_feed(&self) -> Result<UserForumFeedResponse, Error> {
        self.client.request("/user/forumfeed", &[]).await
    }

    /// Get forum activity from user's friends.
    pub async fn forum_friends(&self) -> Result<UserForumFriendsResponse, Error> {
        self.client.request("/user/forumfriends", &[]).await
    }

    /// Get user's forum posts.
    pub async fn forum_posts(&self) -> Result<PaginatedResponse<UserForumPostsResponse>, Error> {
        self.client.request_paginated("/user/forumposts", &[]).await
    }

    /// Get user's subscribed forum threads.
    pub async fn forum_subscribed_threads(&self) -> Result<UserForumSubscribedThreadsResponse, Error> {
        self.client.request("/user/forumsubscribedthreads", &[]).await
    }

    /// Get user's forum threads.
    pub async fn forum_threads(&self) -> Result<PaginatedResponse<UserForumThreadsResponse>, Error> {
        self.client.request_paginated("/user/forumthreads", &[]).await
    }

    /// Get user's hall of fame stats.
    pub async fn hof(&self) -> Result<UserHofResponse, Error> {
        self.client.request("/user/hof", &[]).await
    }

    /// Get user's honors.
    pub async fn honors(&self) -> Result<UserHonorsResponse, Error> {
        self.client.request("/user/honors", &[]).await
    }

    /// Get user's icons.
    pub async fn icons(&self) -> Result<UserIconsResponse, Error> {
        self.client.request("/user/icons", &[]).await
    }

    /// Get user's item market listings.
    pub async fn item_market(&self) -> Result<PaginatedResponse<UserItemMarketResponse>, Error> {
        self.client.request_paginated("/user/itemmarket", &[]).await
    }

    /// Get user's current job information.
    pub async fn job(&self) -> Result<UserJobResponse, Error> {
        self.client.request("/user/job", &[]).await
    }

    /// Get user's job points.
    pub async fn job_points(&self) -> Result<UserJobPointsResponse, Error> {
        self.client.request("/user/jobpoints", &[]).await
    }

    /// Get user's ranks in all job types.
    pub async fn job_ranks(&self) -> Result<UserJobRanksResponse, Error> {
        self.client.request("/user/jobranks", &[]).await
    }

    /// Get a list of users (requires specific query parameters).
    pub async fn list(&self) -> Result<PaginatedResponse<UserListResponse>, Error> {
        self.client.request_paginated("/user/list", &[]).await
    }

    /// Get user activity log.
    pub async fn log(&self) -> Result<PaginatedResponse<UserLogsResponse>, Error> {
        self.client.request_paginated("/user/log", &[]).await
    }

    /// Get available user selections for lookup.
    pub async fn lookup(&self) -> Result<UserLookupResponse, Error> {
        self.client.request("/user/lookup", &[]).await
    }

    /// Get user's medals.
    pub async fn medals(&self) -> Result<UserMedalsResponse, Error> {
        self.client.request("/user/medals", &[]).await
    }

    /// Get user's merits.
    pub async fn merits(&self) -> Result<UserMeritsResponse, Error> {
        self.client.request("/user/merits", &[]).await
    }

    /// Get user's messages.
    pub async fn messages(&self) -> Result<PaginatedResponse<UserMessagesResponse>, Error> {
        self.client.request_paginated("/user/messages", &[]).await
    }

    /// Get user's missions.
    pub async fn missions(&self) -> Result<UserMissionsResponse, Error> {
        self.client.request("/user/missions", &[]).await
    }

    /// Get user's money information.
    pub async fn money(&self) -> Result<UserMoneyResponse, Error> {
        self.client.request("/user/money", &[]).await
    }

    /// Get new events since last check.
    pub async fn new_events(&self) -> Result<UserNewEventsResponse, Error> {
        self.client.request("/user/newevents", &[]).await
    }

    /// Get new messages since last check.
    pub async fn new_messages(&self) -> Result<UserNewMessagesResponse, Error> {
        self.client.request("/user/newmessages", &[]).await
    }

    /// Get user's notification settings.
    pub async fn notifications(&self) -> Result<UserNotificationsResponse, Error> {
        self.client.request("/user/notifications", &[]).await
    }

    /// Get user's organized crime information.
    pub async fn organized_crime(&self) -> Result<UserOrganizedCrimeResponse, Error> {
        self.client.request("/user/organizedcrime", &[]).await
    }

    /// Get user's personal stats.
    pub async fn personal_stats(&self) -> Result<UserPersonalStatsResponse, Error> {
        self.client.request("/user/personalstats", &[]).await
    }

    /// Get user's profile information.
    pub async fn profile(&self) -> Result<UserProfileResponse, Error> {
        self.client.request("/user/profile", &[]).await
    }

    /// Get user's properties (paginated list).
    pub async fn properties(&self) -> Result<PaginatedResponse<UserPropertiesResponse>, Error> {
        self.client.request_paginated("/user/properties", &[]).await
    }

    /// Get user's property details.
    pub async fn property(&self) -> Result<PaginatedResponse<UserPropertiesResponse>, Error> {
        self.client.request_paginated("/user/property", &[]).await
    }

    /// Get user's race history.
    pub async fn races(&self) -> Result<PaginatedResponse<UserRacesResponse>, Error> {
        self.client.request_paginated("/user/races", &[]).await
    }

    /// Get user's racing records.
    pub async fn racing_records(&self) -> Result<UserRacingRecordsResponse, Error> {
        self.client.request("/user/racingrecords", &[]).await
    }

    /// Get user's refills information.
    pub async fn refills(&self) -> Result<UserRefillsResponse, Error> {
        self.client.request("/user/refills", &[]).await
    }

    /// Get user's reports (requires specific permissions).
    pub async fn reports(&self) -> Result<PaginatedResponse<ReportsResponse>, Error> {
        self.client.request_paginated("/user/reports", &[]).await
    }

    /// Get user's revives history.
    pub async fn revives(&self) -> Result<PaginatedResponse<RevivesResponse>, Error> {
        self.client.request_paginated("/user/revives", &[]).await
    }

    /// Get full user's revives history with additional details.
    pub async fn revives_full(&self) -> Result<PaginatedResponse<RevivesFullResponse>, Error> {
        self.client.request_paginated("/user/revivesFull", &[]).await
    }

    /// Get user's skills.
    pub async fn skills(&self) -> Result<UserSkillsResponse, Error> {
        self.client.request("/user/skills", &[]).await
    }

    /// Get current server timestamp.
    pub async fn timestamp(&self) -> Result<TimestampResponse, Error> {
        self.client.request("/user/timestamp", &[]).await
    }

    /// Get user's travel information.
    pub async fn travel(&self) -> Result<UserTravelResponse, Error> {
        self.client.request("/user/travel", &[]).await
    }

    /// Get user's virus programming status.
    pub async fn virus(&self) -> Result<UserVirusResponse, Error> {
        self.client.request("/user/virus", &[]).await
    }

    /// Get user's weapon experience.
    pub async fn weapon_exp(&self) -> Result<UserWeaponExpResponse, Error> {
        self.client.request("/user/weaponexp", &[]).await
    }

    /// Get user's work stats.
    pub async fn work_stats(&self) -> Result<UserWorkStatsResponse, Error> {
        self.client.request("/user/workstats", &[]).await
    }

    /// Access endpoints for a specific user by ID.
    pub fn with_id(&self, id: u64) -> UserIdContext<'a> {
        UserIdContext {
            client: self.client,
            id,
        }
    }

    /// Access crime-specific endpoints by crime ID.
    pub fn with_crime_id(&self, crime_id: u64) -> UserCrimeIdContext<'a> {
        UserCrimeIdContext {
            client: self.client,
            crime_id,
        }
    }
}

/// User API endpoints scoped to a specific user ID.
pub struct UserIdContext<'a> {
    client: &'a TornClient,
    id: u64,
}

impl<'a> UserIdContext<'a> {
    /// Get basic info for this user.
    pub async fn basic(&self) -> Result<UserBasicResponse, Error> {
        let path = format!("/user/{}/basic", self.id);
        self.client.request(&path, &[]).await
    }

    /// Get bounties on this user.
    pub async fn bounties(&self) -> Result<UserBountiesResponse, Error> {
        let path = format!("/user/{}/bounties", self.id);
        self.client.request(&path, &[]).await
    }

    /// Get competition status for this user.
    pub async fn competition(&self) -> Result<UserCompetitionResponse, Error> {
        let path = format!("/user/{}/competition", self.id);
        self.client.request(&path, &[]).await
    }

    /// Get Discord information for this user.
    pub async fn discord(&self) -> Result<UserDiscordResponse, Error> {
        let path = format!("/user/{}/discord", self.id);
        self.client.request(&path, &[]).await
    }

    /// Get faction information for this user.
    pub async fn faction(&self) -> Result<UserFactionResponse, Error> {
        let path = format!("/user/{}/faction", self.id);
        self.client.request(&path, &[]).await
    }

    /// Get forum posts by this user.
    pub async fn forum_posts(&self) -> Result<PaginatedResponse<UserForumPostsResponse>, Error> {
        let path = format!("/user/{}/forumposts", self.id);
        self.client.request_paginated(&path, &[]).await
    }

    /// Get forum threads by this user.
    pub async fn forum_threads(&self) -> Result<PaginatedResponse<UserForumThreadsResponse>, Error> {
        let path = format!("/user/{}/forumthreads", self.id);
        self.client.request_paginated(&path, &[]).await
    }

    /// Get hall of fame stats for this user.
    pub async fn hof(&self) -> Result<UserHofResponse, Error> {
        let path = format!("/user/{}/hof", self.id);
        self.client.request(&path, &[]).await
    }

    /// Get icons for this user.
    pub async fn icons(&self) -> Result<UserIconsResponse, Error> {
        let path = format!("/user/{}/icons", self.id);
        self.client.request(&path, &[]).await
    }

    /// Get job information for this user.
    pub async fn job(&self) -> Result<UserJobResponse, Error> {
        let path = format!("/user/{}/job", self.id);
        self.client.request(&path, &[]).await
    }

    /// Get personal stats for this user.
    pub async fn personal_stats(&self) -> Result<UserPersonalStatsResponse, Error> {
        let path = format!("/user/{}/personalstats", self.id);
        self.client.request(&path, &[]).await
    }

    /// Get profile information for this user.
    pub async fn profile(&self) -> Result<UserProfileResponse, Error> {
        let path = format!("/user/{}/profile", self.id);
        self.client.request(&path, &[]).await
    }

    /// Get properties owned by this user (paginated list).
    pub async fn properties(&self) -> Result<PaginatedResponse<UserPropertiesResponse>, Error> {
        let path = format!("/user/{}/properties", self.id);
        self.client.request_paginated(&path, &[]).await
    }

    /// Get property details for this user.
    pub async fn property(&self) -> Result<PaginatedResponse<UserPropertiesResponse>, Error> {
        let path = format!("/user/{}/property", self.id);
        self.client.request_paginated(&path, &[]).await
    }
}

/// User API endpoints scoped to a specific crime ID.
pub struct UserCrimeIdContext<'a> {
    client: &'a TornClient,
    crime_id: u64,
}

impl<'a> UserCrimeIdContext<'a> {
    /// Get crime statistics for this specific crime.
    pub async fn crimes(&self) -> Result<UserCrimesResponse, Error> {
        let path = format!("/user/{}/crimes", self.crime_id);
        self.client.request(&path, &[]).await
    }
}

