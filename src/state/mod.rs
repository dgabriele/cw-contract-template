pub mod models;
pub mod storage;

use crate::{error::ContractError, execute::Context, msg::InstantiateMsg};

/// Initialize contract state data.
pub fn init(
    _ctx: Context,
    _msg: &InstantiateMsg,
) -> Result<(), ContractError> {
    Ok(())
}
