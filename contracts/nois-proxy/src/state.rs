use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {}

pub const CONFIG: Item<Config> = Item::new("config");

pub const TERRAND_CHANNEL: Item<String> = Item::new("terrand_channel");
