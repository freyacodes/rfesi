#![allow(unused)]

use crate::prelude::*;

/// Endpoints for Loyalty
pub struct LoyaltyGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct LoyaltyOffer {
    pub type_id: i32
}

impl LoyaltyGroup<'_> {
    api_get!(
        /// Get loyalty offers for an NPC corporation
        get_corporation_offers,
        "get_loyalty_stores_corporation_id_offers",
        RequestType::Public,
        Vec<LoyaltyOffer>,
        (corporation_id: i32) => "{corporation_id}"
    );
}
