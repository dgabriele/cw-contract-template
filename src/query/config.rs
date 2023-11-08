use crate::{error::ContractError, msg::ConfigResponse, state::storage::CONFIG};

use super::ReadonlyContext;

pub fn query_config(ctx: ReadonlyContext) -> Result<ConfigResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    Ok(CONFIG
        .load(deps.storage)
        .and_then(|config| Ok(ConfigResponse(config)))?)
}
