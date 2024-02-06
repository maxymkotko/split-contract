use cosmwasm_std::{Addr, Uint128};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
    pub initial_recipients: Vec<RecipientInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RecipientInfo {
    pub addr: Addr,
    pub share: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Distribute {},
    UpdateRecipient{ recipient: Addr, share: Uint128 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    
}