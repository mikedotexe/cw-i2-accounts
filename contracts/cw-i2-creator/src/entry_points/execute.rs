pub mod create;

use crate::errors::ContractError;
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw_i2_creator_pkg::msgs::execute_msg::ExecuteMsg;

/// Execute entry point.
/// You may see a list of the execute variants (methods) in [ExecuteMsg](ExecuteMsg)
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Create {
            target_contract,
            access_control_identifier,
            access_control,
            skip_code_id_check,
            fund_limits,
            extra,
            label,
        } => create::execute(
            env,
            deps,
            info,
            target_contract,
            access_control_identifier,
            access_control,
            skip_code_id_check,
            fund_limits,
            extra,
            label
        ),
    }
}
