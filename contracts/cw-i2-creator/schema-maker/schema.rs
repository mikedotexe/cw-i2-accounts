use cosmwasm_schema::write_api;

use cw_i2_creator::msgs::ExecuteMsg;
use cw_i2_creator::msgs::InstantiateMsg;
use cw_i2_creator::msgs::QueryMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg,
    }
}
