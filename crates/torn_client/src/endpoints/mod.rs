//! Typed endpoint methods grouped by API tag.

pub mod faction;
pub mod forum;
pub mod key;
pub mod market;
pub mod property;
pub mod racing;
pub mod torn;
pub mod user;
pub use faction::{
    FactionChainReportContext, FactionCrimeContext, FactionEndpoint, FactionIdContext,
    FactionRaidReportContext, FactionRankedWarReportContext, FactionTerritoryWarReportContext,
};
pub use forum::{ForumCategoryIdsContext, ForumEndpoint, ForumThreadIdContext};
pub use key::KeyEndpoint;
pub use market::{MarketAuctionIdContext, MarketEndpoint, MarketItemIdContext, MarketPropertyTypeIdContext};
pub use property::{PropertyEndpoint, PropertyIdContext, PropertyParams};
pub use racing::{RacingEndpoint, RacingRaceIdContext, RacingTrackIdContext};
pub use torn::{
    TornCrimeContext, TornEliminationTeamContext, TornEndpoint, TornHonorsContext,
    TornItemDetailsContext, TornItemsContext, TornLogCategoryContext, TornMedalsContext,
};
pub use user::{UserCrimeIdContext, UserEndpoint, UserIdContext};
