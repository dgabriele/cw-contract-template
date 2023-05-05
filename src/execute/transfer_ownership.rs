use crate::{
  error::ContractError,
  state::{is_owner, OWNER},
};
use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response};

pub fn transfer_ownership(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  new_owner: &Addr,
) -> Result<Response, ContractError> {
  if !is_owner(deps.storage, &info.sender)? {
    return Err(ContractError::NotAuthorized {});
  }
  OWNER.save(deps.storage, new_owner)?;
  Ok(Response::new().add_attributes(vec![attr("action", new_owner.to_string())]))
}
