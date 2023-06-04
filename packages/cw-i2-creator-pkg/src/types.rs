use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, HexBinary};

#[cw_serde]
pub struct Config {
    /// Some, most, or all methods check this address for privileged access
    pub owner: String,
    /// Provide a code ID to what's considered a "base contract"
    /// That has a migrate entry point where the admin can
    /// overwrite the contract after it's created
    pub base_contract_code_id: u64,
    /// The base contract's checksum
    pub base_contract_checksum: HexBinary
}
