//! Execute my logic

use cosmwasm_schema::schemars::_serde_json::json;
use crate::errors::ContractError;
use crate::state::CONFIG;
use cosmwasm_std::{Addr, Binary, CanonicalAddr, CodeInfoResponse, Coin, ContractInfoResponse, DepsMut, Env, instantiate2_address, MessageInfo, Response, to_binary, WasmMsg, WasmQuery};
use cw_i2_creator_pkg::msgs::instantiate_msg::InstantiateMsg;

/// Logic for the [MyExecute](cw_i2_creator_pkg::msgs::execute_msg::ExecuteMsg::MyExecute) (`my_execute`) method
pub fn execute(
    env: Env,
    deps: DepsMut,
    info: MessageInfo,
    target_contract: String,
    access_control_identifier: String,
    access_control: Binary,
    skip_code_id_check: bool,
    fund_limits: Vec<Coin>,
    extra: Option<Binary>,
    label: Option<String>,
) -> Result<Response, ContractError> {
    let salt_json = json!({
        "target_contract": target_contract,
        "access_control_identifier": access_control_identifier,
        "access_control": access_control,
        "skip_code_id_check": skip_code_id_check,
        "fund_limits": fund_limits,
        "extra": extra,
    });
    let salt_string = salt_json.to_string();
    let salt = salt_string.as_bytes();

    let contract_addr = env.contract.address;
    // Grab our owner
    let config = CONFIG.load(deps.storage)?;
    // This was saved during instantiate from info.sender
    let owner = Addr::unchecked(config.owner);
    let checksum = config.base_contract_checksum.as_slice();
    let code_id = config.base_contract_code_id;

    // Access control, so only owner can proceed
    if info.sender != owner {
        return Err(ContractError::Unauthorized { owner });
    }

    // arg i think i gotta remove this
    // Get your own code ID
    // let rewards_res: ContractInfoResponse = deps.querier.query(
    //     &WasmQuery::ContractInfo {
    //         contract_addr: contract_addr.clone().to_string(),
    //     }.into(),
    // )?;
    // let code_id = rewards_res.code_id;

    // // Now get our code info
    // let ContractInfoResponse { code_id, .. } = deps
    //   .querier
    //   .query_wasm_contract_info(contract_addr.clone())?;
    // Grab the checksum of ourselves
    // let CodeInfoResponse { checksum, .. } = deps.querier.query_wasm_code_info(code_id)?;
    // // The creator will be this contract
    // let creator = deps.api.addr_canonicalize(contract_addr.as_str())?;
    let creator = deps.api.addr_canonicalize(info.sender.as_str())?;

    let done_address = deps
      .api
      .addr_humanize(&instantiate2_address(&checksum, &creator, &salt)?)?;

    // Check to see if that address exists
    if deps.querier.query_wasm_contract_info(done_address).is_ok() {
        // Fail. This contract has already been instantiated once
        return Err(ContractError::AlreadyInstantiated)
    }

    // At this point we know this is the first instantiation
    // CosmosMsg::Wasm(WasmMsg::Instantiate2 {
    // })
    let instantiate_done_contract_msg = WasmMsg::Instantiate2 {
        admin: Some(info.sender.to_string()),
        code_id,
        label: "Confirming the first instantiation".to_string(),
        msg: to_binary(&cw_i2_base_pkg::msgs::instantiate_msg::InstantiateMsg {})?,
        funds: vec![],
        salt: to_binary(salt)?,
    };

    Ok(Response::new().add_attribute("action", "my_execute"))
}
