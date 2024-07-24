pub mod models;
pub mod storage;

use cosmwasm_std::Response;

use crate::{error::ContractError, execute::Context, msg::InstantiateMsg};

/// Top-level initialization of contract state
pub fn init(
    _ctx: Context,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "instantiate"))
}
