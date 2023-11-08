mod set_config;

use cosmwasm_std::{DepsMut, Env, MessageInfo};
pub use set_config::exec_set_config;

pub struct Context<'a> {
    pub deps: DepsMut<'a>,
    pub env: Env,
    pub info: MessageInfo,
}
