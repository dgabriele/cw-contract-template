use cosmwasm_schema::cw_serde;

use crate::state::Config;

#[cw_serde]
pub struct ConfigResponse(pub Config);
