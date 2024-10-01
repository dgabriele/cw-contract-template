use crate::{
    error::ContractError,
    responses::ConfigResponse,
    state::{QueryContext, CONFIG},
};

pub fn query_config(ctx: QueryContext) -> Result<ConfigResponse, ContractError> {
    let QueryContext { deps, .. } = ctx;
    Ok(CONFIG
        .load(deps.storage)
        .and_then(|config| Ok(ConfigResponse(config)))?)
}
