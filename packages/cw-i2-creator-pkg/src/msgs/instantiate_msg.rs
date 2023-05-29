use cosmwasm_schema::cw_serde;

/// The instantiate message with no params
/// We will only allow one contract to be instantiated
#[cw_serde]
pub struct InstantiateMsg {}
