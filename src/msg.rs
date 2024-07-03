use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
use cw20::Cw20ReceiveMsg;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    RegisterInterchainAccount {
        connection_id: String,
        version: String,
    },
    Receive(Cw20ReceiveMsg),
    MitoStake {
        amount: String,
    },
    SendAndSwapOsmosis {
        amount: Uint128,
    },
}

#[cw_serde]
pub enum Cw20HookMsg {
    IbcSend {},
    SendAndSwapInjective {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

#[cw_serde]
pub struct MigrateMsg {}
