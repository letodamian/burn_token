use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

use crate::state::Config;

#[cw_serde]
pub struct InstantiateMsg {
    pub nois_sink: String,
    pub nois_com_pool_addr: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Pays by distributing the funds according to what has been instructed by the gateway
    Pay {
        burn: Uint128,
        community_pool: Uint128,
        relayer: (String, Uint128),
    },
}

#[cw_serde]
pub enum NoisSinkExecuteMsg {
    /// Burns the tokens that are sent as `funds` with this message
    Burn {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Get the config state
    #[returns(ConfigResponse)]
    Config {},
}

// We define a custom struct for each query response
pub type ConfigResponse = Config;
