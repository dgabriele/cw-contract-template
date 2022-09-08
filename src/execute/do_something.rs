use crate::error::ContractError;
use crate::state::{
  SOMETHING
};
use cosmwasm_std::{
  attr,  DepsMut, Env, MessageInfo, Response,
};

pub fn do_something(
  deps: DepsMut,
  _env: Env,
  _info: MessageInfo,
  value: &Option<String>,
) -> Result<Response, ContractError> {
  SOMETHING.update(deps.storage, |mut something| -> Result<_, ContractError> {
    something.value = value.clone();
    Ok(something)
  })?;

  // build JSON response attributes 
  let mut attrs = vec![
      attr("action", "do_something"),
  ];
  if let Some(value_str) = value {
    attrs.push(attr("value", value_str.clone()));
  }

  Ok(Response::new().add_attributes(attrs))
}
