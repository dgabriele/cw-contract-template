use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::Something;

/// Initial contract state.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
  pub value: Option<String>,
}

/// Executable contract endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
  DoSomething {
    value: Option<String>,
  },
}

/// Custom contract query endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  GetSomething {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetSomethingResponse {
  pub something: Something,
}