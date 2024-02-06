use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Share Sum exceeds 100 - ShareSum: {share_sum}")]
    ExceedShareSum { share_sum: Uint128 },
}