#[cfg(not(feature = "library"))]
pub mod contract;
pub mod error;
#[cfg(not(feature = "library"))]
pub mod execute;
pub mod msg;
#[cfg(not(feature = "library"))]
pub mod query;
pub mod state;

#[cfg(not(target_arch = "wasm32"))]
mod interface;
#[cfg(not(target_arch = "wasm32"))]
pub use crate::interface::Contract;
#[cfg(not(target_arch = "wasm32"))]
pub use crate::msg::{ExecuteMsgFns as ContractExecuteMsgFns, QueryMsgFns as ContractQueryMsgFns};
