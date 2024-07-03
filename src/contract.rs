#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_json, to_json_binary, to_json_vec, Binary, Coin, CosmosMsg, Deps, DepsMut, Env,
    IbcTimeout, MessageInfo, Response, StdResult, Timestamp, Uint128,
};
// use cw2::set_contract_version;
use cosmos_sdk_proto::{
    cosmos::base::v1beta1::Coin as ProtoCoin,
    cosmwasm::wasm::v1::{MsgExecuteContract, MsgSwapExactAmountIn, SwapAmountInRoute},
    ibc::applications::interchain_accounts::v1::{MsgRegisterAccount, MsgSubmitTx},
    traits::{MessageExt, TypeUrl},
    Any,
};
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};
use injective_math::FPDecimal;

use crate::{error::ContractError, interfaces::MitoExecuteMsg, msg::MigrateMsg};
use crate::{
    interfaces::{AssetInfo, ConverterExecuteMsg},
    msg::{Cw20HookMsg, ExecuteMsg, InstantiateMsg, QueryMsg},
};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:ibc-test";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RegisterInterchainAccount {
            connection_id,
            version,
        } => execute_register_interchain_account(env, connection_id, version),
        ExecuteMsg::Receive(msg) => receive_cw20(deps, env, info, msg),
        ExecuteMsg::MitoStake { amount } => execute_mito_stake(env, amount),
        ExecuteMsg::SendAndSwapOsmosis { amount } => {
            execute_send_and_swap_osmosis(env, info, amount)
        }
    }
}

fn execute_send_and_swap_osmosis(
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    {
        let osmosis_denom =
            "ibc/9C4DCD21B48231D0BC2AC3D1B74A864746B37E4292694C93C617324250D002FC".to_string();
        if !(info.funds.contains(&Coin {
            denom: osmosis_denom.clone(),
            amount,
        })) {
            return Err(ContractError::Unauthorized {});
        }

        let mut messages: Vec<CosmosMsg> = vec![];

        messages.push(CosmosMsg::Ibc(cosmwasm_std::IbcMsg::Transfer {
            channel_id: "channel-13".to_string(),
            to_address: "osmo15xgvz869tht64lavk93q5jzj6t3zfvy9w29gmrffnarnd28ywxfsa7g6a6"
                .to_string(),
            amount: Coin {
                denom: osmosis_denom,
                amount,
            },
            timeout: IbcTimeout::with_timestamp(Timestamp::from_seconds(
                env.block.time.seconds() + 86400,
            )),
        }));

        let osmosis_swap_msg = MsgSwapExactAmountIn {
            sender: "osmo15xgvz869tht64lavk93q5jzj6t3zfvy9w29gmrffnarnd28ywxfsa7g6a6".to_string(),
            routes: vec![SwapAmountInRoute {
                pool_id: 1464,
                token_out_denom:
                    "ibc/498A0751C798A0D9A389AA3691123DADA57DAA4FE165D5C75894505B876BA6E4"
                        .to_string(),
            }],
            token_in: Some(ProtoCoin {
                denom: "uosmo".to_string(),
                amount: amount.to_string(),
            }),
            token_out_min_amount: "26931".to_string(),
        };
        let msg = Any {
            type_url: "/osmosis.gamm.v1beta1.MsgSwapExactAmountIn".to_string(),
            value: osmosis_swap_msg.to_bytes().unwrap().into(),
        };

        let tx = MsgSubmitTx {
            owner: env.contract.address.to_string(),
            connection_id: "connection-21".to_string(),
            msg: Some(msg),
        };

        messages.push(CosmosMsg::Stargate {
            type_url: MsgSubmitTx::TYPE_URL.to_string(),
            value: tx.to_bytes().unwrap().into(),
        });

        Ok(Response::default().add_messages(messages))
    }
}

fn execute_mito_stake(env: Env, amount: String) -> Result<Response, ContractError> {
    let mito_swap_msg = MsgExecuteContract {
        sender: "inj1wlkhnrhy2u90tkg7xfgmxu0yyvd8zdx9cpuwjfxzkxrgazm9sf6schgmqr".to_string(),
        contract: "inj1j5mr2hmv7y2z7trazganj75u8km8jvdfuxncsp".to_string(),
        msg: to_json_vec(&MitoExecuteMsg::SwapMinOutput {
            min_output_quantity: FPDecimal::from(22000u128),
            target_denom: "peggy0xdAC17F958D2ee523a2206206994597C13D831ec7".to_string(),
        })?,
        funds: vec![ProtoCoin {
            amount,
            denom: "inj".to_string(),
        }],
    };
    // "/injective.wasmx.v1.MsgExecuteContractCompat"
    let msg = Any {
        type_url: MsgExecuteContract::TYPE_URL.to_string(),
        value: mito_swap_msg.to_bytes().unwrap().into(),
    };

    let tx = MsgSubmitTx {
        owner: env.contract.address.to_string(),
        connection_id: "connection-82".to_string(),
        msg: Some(msg),
    };

    Ok(Response::new().add_message(CosmosMsg::Stargate {
        type_url: MsgSubmitTx::TYPE_URL.to_string(),
        value: tx.to_bytes().unwrap().into(),
    }))
}

fn execute_register_interchain_account(
    env: Env,
    connection_id: String,
    version: String,
) -> Result<Response, ContractError> {
    let register_msg = MsgRegisterAccount {
        owner: env.contract.address.to_string(),
        connection_id,
        version,
    };
    Ok(Response::new().add_message(CosmosMsg::Stargate {
        type_url: MsgRegisterAccount::TYPE_URL.to_string(),
        value: register_msg.to_bytes().unwrap().into(),
    }))
}

fn receive_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    match from_json(&cw20_msg.msg)? {
        Cw20HookMsg::IbcSend {} => {
            let inj_contract_address = deps
                .api
                .addr_validate("orai19rtmkk6sn4tppvjmp5d5zj6gfsdykrl5rw2euu5gwur3luheuuusesqn49")?;
            if info.sender != inj_contract_address {
                return Err(ContractError::Unauthorized {});
            }

            let mut messages: Vec<CosmosMsg> = vec![];
            messages.push(CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
                contract_addr: inj_contract_address.to_string(),
                msg: to_json_binary(&Cw20ExecuteMsg::Send {
                    contract: "orai14wy8xndhnvjmx6zl2866xqvs7fqwv2arhhrqq9".to_string(),
                    amount: cw20_msg.amount,
                    msg: to_json_binary(&ConverterExecuteMsg::ConvertReverse { from: AssetInfo::NativeToken { denom: "ibc/49D820DFDE9F885D7081725A58202ABA2F465CAEE4AFBC683DFB79A8E013E83E".to_string() }})?,
                })?,
                funds: vec![],
            }));

            messages.push(CosmosMsg::Ibc(cosmwasm_std::IbcMsg::Transfer {
                channel_id: "channel-146".to_string(),
                to_address: "inj1wlkhnrhy2u90tkg7xfgmxu0yyvd8zdx9cpuwjfxzkxrgazm9sf6schgmqr"
                    .to_string(),
                amount: Coin {
                    denom: "ibc/49D820DFDE9F885D7081725A58202ABA2F465CAEE4AFBC683DFB79A8E013E83E"
                        .to_string(),
                    amount: cw20_msg.amount,
                },
                timeout: IbcTimeout::with_timestamp(Timestamp::from_seconds(
                    env.block.time.seconds() + 86400,
                )),
            }));

            Ok(Response::default().add_messages(messages))
        }
        Cw20HookMsg::SendAndSwapInjective {} => {
            let inj_contract_address = deps
                .api
                .addr_validate("orai19rtmkk6sn4tppvjmp5d5zj6gfsdykrl5rw2euu5gwur3luheuuusesqn49")?;
            if info.sender != inj_contract_address {
                return Err(ContractError::Unauthorized {});
            }

            let mut messages: Vec<CosmosMsg> = vec![];
            messages.push(CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: inj_contract_address.to_string(),
            msg: to_json_binary(&Cw20ExecuteMsg::Send {
                contract: "orai14wy8xndhnvjmx6zl2866xqvs7fqwv2arhhrqq9".to_string(),
                amount: cw20_msg.amount,
                msg: to_json_binary(&ConverterExecuteMsg::ConvertReverse { from: AssetInfo::NativeToken { denom: "ibc/49D820DFDE9F885D7081725A58202ABA2F465CAEE4AFBC683DFB79A8E013E83E".to_string() }})?,
            })?,
            funds: vec![],
            }));

            let native_decimal_amount = cw20_msg.amount * Uint128::from(1_000_000_000_000u128);

            messages.push(CosmosMsg::Ibc(cosmwasm_std::IbcMsg::Transfer {
                channel_id: "channel-146".to_string(),
                to_address: "inj1wlkhnrhy2u90tkg7xfgmxu0yyvd8zdx9cpuwjfxzkxrgazm9sf6schgmqr"
                    .to_string(),
                amount: Coin {
                    denom: "ibc/49D820DFDE9F885D7081725A58202ABA2F465CAEE4AFBC683DFB79A8E013E83E"
                        .to_string(),
                    amount: native_decimal_amount,
                },
                timeout: IbcTimeout::with_timestamp(Timestamp::from_seconds(
                    env.block.time.seconds() + 86400,
                )),
            }));

            let mito_swap_msg = MsgExecuteContract {
                sender: "inj1wlkhnrhy2u90tkg7xfgmxu0yyvd8zdx9cpuwjfxzkxrgazm9sf6schgmqr"
                    .to_string(),
                contract: "inj1j5mr2hmv7y2z7trazganj75u8km8jvdfuxncsp".to_string(),
                msg: to_json_vec(&MitoExecuteMsg::SwapMinOutput {
                    min_output_quantity: FPDecimal::from(22000u128),
                    target_denom: "peggy0xdAC17F958D2ee523a2206206994597C13D831ec7".to_string(),
                })?,
                funds: vec![ProtoCoin {
                    amount: native_decimal_amount.to_string(),
                    denom: "inj".to_string(),
                }],
            };
            // "/injective.wasmx.v1.MsgExecuteContractCompat"
            let msg = Any {
                type_url: MsgExecuteContract::TYPE_URL.to_string(),
                value: mito_swap_msg.to_bytes().unwrap().into(),
            };

            let tx = MsgSubmitTx {
                owner: env.contract.address.to_string(),
                connection_id: "connection-82".to_string(),
                msg: Some(msg),
            };

            messages.push(CosmosMsg::Stargate {
                type_url: MsgSubmitTx::TYPE_URL.to_string(),
                value: tx.to_bytes().unwrap().into(),
            });

            Ok(Response::default().add_messages(messages))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}
