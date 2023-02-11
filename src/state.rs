use crate::error::ContractError;
use crate::msg::InstantiateMsg;
use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, StdResult, Storage};
use cw_storage_plus::Item;

pub const OWNER: Item<Addr> = Item::new("owner");

/// Initialize contract state data.
pub fn initialize(
  deps: DepsMut,
  _env: &Env,
  info: &MessageInfo,
  _msg: &InstantiateMsg,
) -> Result<(), ContractError> {
  OWNER.save(deps.storage, &info.sender)?;
  Ok(())
}

pub fn is_owner(
  storage: &dyn Storage,
  addr: &Addr,
) -> StdResult<bool> {
  return Ok(OWNER.load(storage)? == *addr);
}
