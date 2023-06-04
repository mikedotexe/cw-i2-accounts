use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Coin};

/// Lists the available execute messages for this contract
#[cw_serde]
pub enum ExecuteMsg {
    Create {
        /// The contract the signer wishes to call, through this contract
        /// The authz execute will only happen if the key is allowed to
        /// call the given method, as defined by access control
        target_contract: String,
        /// Whatever access control we decide is best, we name it
        access_control_identifier: String,
        /// The binary of the privileges / access control meant to be
        /// deserialized and applied to guard logic, so that authz exec
        /// is only called when the criteria is met.
        /// ex: the sender is allowed to call function X on the target contract
        access_control: Binary,
        /// When the contract is instantiated, controls the preference
        /// toward confirming the code_id hasn't changed
        /// This can be overriden on the created contract
        skip_code_id_check: bool,
        /// Blanket rule that applies to how much funds can be sent
        fund_limits: Vec<Coin>,
        /// To future-proof, let's allow people to chuck in more info
        extra: Option<Binary>,
        /// Label for the contract (doesn't affect the salt)
        label: Option<String>,
    },
}
