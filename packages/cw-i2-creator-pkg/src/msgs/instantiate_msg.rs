use cosmwasm_schema::cw_serde;

/// The instantiate message with no params
#[cw_serde]
pub struct InstantiateMsg {
  /// The code ID to the base contract
  pub code_id: u64,
  /// The sha256sum of the base contract's stored binary
  pub checksum: String,
}
