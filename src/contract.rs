use cosmwasm_std::{entry_point, Addr, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use crate::state::{Recipient, RECIPIENTS};
use crate::msg::{InstantiateMsg, ExecuteMsg};
use crate::error::ContractError::{self, ExceedShareSum};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let mut share_sum = 0u128;

    for recipient in msg.initial_recipients {
        RECIPIENTS.save(deps.storage, &recipient.addr, &Recipient {
            addr: recipient.addr.clone(),
            share: recipient.share,
        })?;
        share_sum += recipient.share.u128();
    }
    
    if share_sum > 100u128 {
        return Err(ExceedShareSum {share_sum: Uint128::from(share_sum)}) ;
    }

    Ok(Response::new().add_attribute("method", "instantiate"))
}

use cosmwasm_std::{BankMsg, coins};

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Distribute {} => handle_distribute(deps, env, info),
        ExecuteMsg::UpdateRecipient { recipient, share } => handle_update_recipient(deps, recipient, share),
    }
}

fn handle_distribute(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let total_amount = info.funds.iter().find(|c| c.denom == "usei").map_or(0u128, |c| c.amount.u128()); 
    let mut messages = vec![];
    
    let recipients = RECIPIENTS.range(deps.storage, None, None, cosmwasm_std::Order::Ascending).collect::<StdResult<Vec<_>>>()?;
    
    for (addr, recipient) in recipients {
        let recipient_share = total_amount * recipient.share.u128() / 100u128;
        messages.push(BankMsg::Send {
            to_address: addr.into(),
            amount: coins(recipient_share, "usei"),
        });
    }
    
    Ok(Response::new().add_messages(messages).add_attribute("action", "distribute"))
}

fn handle_update_recipient(deps: DepsMut, recipient_addr: Addr, share: Uint128) -> Result<Response, ContractError> {
    RECIPIENTS.save(deps.storage, &recipient_addr, &Recipient { addr: recipient_addr.clone(), share })?;

    let mut share_sum = 0u128;
    for addr_recip in RECIPIENTS.keys(deps.storage, None, None, cosmwasm_std::Order::Ascending) {
        let value_recip =RECIPIENTS.load(deps.storage, &addr_recip.unwrap()).unwrap();
        share_sum += value_recip.share.u128();
    }
    
    if share_sum > 100u128 {
        return Err(ExceedShareSum {share_sum: Uint128::from(share_sum)}) ;
    }
    Ok(Response::new().add_attribute("action", "update_recipient"))
}
