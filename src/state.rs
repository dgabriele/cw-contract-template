use crate::error::ContractError;
use crate::msg::{InstantiateMsg};
use cosmwasm_std::{DepsMut, Env, MessageInfo};
use cw_storage_plus::{Item};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Something {
  pub value: Option<String>,
}

pub const SOMETHING: Item<Something> = Item::new("something");

/// Initialize contract state data.
pub fn initialize(
  deps: DepsMut,
  _env: &Env,
  _info: &MessageInfo,
  msg: &InstantiateMsg,
) -> Result<Something, ContractError> {
  let something = Something {
    value: msg.value.clone()
  };
  SOMETHING.save(deps.storage, &something)?;
  Ok(something)
}
