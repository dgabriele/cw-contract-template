use crate::{msg::SelectResponse, state::OWNER};
use crate::{error::ContractError};
use cosmwasm_std::{Addr, Deps};

pub fn select(
  deps: Deps,
  fields: Option<Vec<String>>,
  _account: Option<Addr>
) -> Result<SelectResponse, ContractError> {
  if let Some(fields) = fields {
    Ok(SelectResponse {
      owner: if fields.contains(&"owner".to_owned()) {
        OWNER.may_load(deps.storage)?
      } else {
        None
      },
    })
  } else {
    Ok(SelectResponse {
      owner: OWNER.may_load(deps.storage)?,
    })
  }
}
