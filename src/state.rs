use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Recipient {
    pub addr: Addr,
    pub share: Uint128,  // Represents the percentage share of each wallet using a Uint128 between 0 to 100
}

// Map each recipient to their details for easy lookup
pub const RECIPIENTS: Map<&Addr, Recipient> = Map::new("recipients");