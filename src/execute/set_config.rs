use crate::{
    error::ContractError,
    state::{Config, ExecuteContext, CONFIG},
};
use cosmwasm_std::{attr, Response};

pub fn exec_set_config(
    ctx: ExecuteContext,
    config: Config,
) -> Result<Response, ContractError> {
    let ExecuteContext { deps, .. } = ctx;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_attributes(vec![attr("action", "set_config")]))
}
