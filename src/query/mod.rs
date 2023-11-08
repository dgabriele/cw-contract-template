mod config;

pub use config::query_config;
use cosmwasm_std::{Deps, Env};

pub struct ReadonlyContext<'a> {
    pub deps: Deps<'a>,
    pub env: Env,
}
