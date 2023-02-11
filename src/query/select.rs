use crate::{msg::SelectResponse, state::OWNER};
use cosmwasm_std::{Deps, StdResult};

pub fn select(
  deps: Deps,
  fields: Option<Vec<String>>,
) -> StdResult<SelectResponse> {
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
