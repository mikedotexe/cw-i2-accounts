use crate::errors::ContractError;
use crate::state::{Config, CONFIG};
use crate::{CONTRACT_NAME, CONTRACT_VERSION};
use cosmwasm_schema::schemars::_serde_json::to_vec;
use cosmwasm_std::{entry_point, from_binary, ContractInfoResponse, Empty, QueryRequest, StdError, WasmQuery, to_binary, instantiate2_address, CodeInfoResponse, CanonicalAddr, CosmosMsg, WasmMsg, HexBinary};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use cw_i2_creator_pkg::msgs::instantiate_msg::InstantiateMsg;

/// Instantiate entry point
/// See the instantiate message and fields in [InstantiateMsg](InstantiateMsg)
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let checksum = HexBinary::from_hex(&msg.checksum)?;

    let config = Config {
        owner: info.sender.to_string(),
        base_contract_code_id: msg.code_id,
        base_contract_checksum: checksum,
    };
    CONFIG.save(deps.storage, &config)?;

    // Sets the version via cw2, just a normal thing to do
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}
