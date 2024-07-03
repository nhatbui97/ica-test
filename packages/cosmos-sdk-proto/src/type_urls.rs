use crate::{cosmos, cosmwasm, ibc, traits::TypeUrl};

#[cfg(feature = "cosmwasm")]
use crate::cosmwasm;

impl TypeUrl for cosmos::bank::v1beta1::MsgSend {
    const TYPE_URL: &'static str = "/cosmos.bank.v1beta1.MsgSend";
}

impl TypeUrl for ibc::applications::interchain_accounts::v1::MsgRegisterAccount {
    const TYPE_URL: &'static str = "/intertx.MsgRegisterAccount";
}

impl TypeUrl for ibc::applications::interchain_accounts::v1::MsgSubmitTx {
    const TYPE_URL: &'static str = "/intertx.MsgSubmitTx";
}

impl TypeUrl for ibc::applications::transfer::v1::MsgTransfer {
    const TYPE_URL: &'static str = "/ibc.applications.transfer.v1.MsgTransfer";
}

impl TypeUrl for cosmos::staking::v1beta1::MsgDelegate {
    const TYPE_URL: &'static str = "/cosmos.staking.v1beta1.MsgDelegate";
}

impl TypeUrl for cosmos::staking::v1beta1::MsgUndelegate {
    const TYPE_URL: &'static str = "/cosmos.staking.v1beta1.MsgUndelegate";
}

impl TypeUrl for cosmos::distribution::v1beta1::MsgSetWithdrawAddress {
    const TYPE_URL: &'static str = "/cosmos.distribution.v1beta1.MsgSetWithdrawAddress";
}

impl TypeUrl for cosmos::staking::v1beta1::MsgBeginRedelegate {
    const TYPE_URL: &'static str = "/cosmos.staking.v1beta1.MsgBeginRedelegate";
}

impl TypeUrl for cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward {
    const TYPE_URL: &'static str = "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward";
}

impl TypeUrl for cosmwasm::wasm::v1::MsgExecuteContract {
    const TYPE_URL: &'static str = "/cosmwasm.wasm.v1.MsgExecuteContract";
}
