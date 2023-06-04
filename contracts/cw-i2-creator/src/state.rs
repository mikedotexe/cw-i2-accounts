use cw_storage_plus::Item;
// Remember the "pub" here, since it's used elsewhere.
pub use cw_i2_creator_pkg::types::Config;

pub const CONFIG: Item<Config> = cw_i2_creator_pkg::state::CONFIG;
