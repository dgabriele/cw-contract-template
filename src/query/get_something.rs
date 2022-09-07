use crate::msg::GetSomethingResponse;
use crate::state::{SOMETHING, Something};
use cosmwasm_std::{Deps, StdResult};

pub fn get_something(deps: Deps) -> StdResult<GetSomethingResponse> {
  let something: Something = SOMETHING.load(deps.storage)?;
  Ok(GetSomethingResponse{ something })
}
