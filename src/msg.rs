use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
  TransferOwnership { new_owner: Addr },
}

#[cw_serde]
pub enum QueryMsg {
  Select { fields: Option<Vec<String>> },
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct SelectResponse {
  pub owner: Option<Addr>,
}
