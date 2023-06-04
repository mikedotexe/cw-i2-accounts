use cosmwasm_schema::write_api;

use cw_i2_base::msgs::ExecuteMsg;
use cw_i2_base::msgs::InstantiateMsg;
use cw_i2_base::msgs::QueryMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg,
    }
}
