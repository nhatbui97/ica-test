use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use injective_math::FPDecimal;

#[cw_serde]
pub enum ConverterExecuteMsg {
    ConvertReverse { from: AssetInfo },
}
#[cw_serde]
pub enum AssetInfo {
    Token { contract_addr: Addr },
    NativeToken { denom: String },
}

#[cw_serde]
pub enum MitoExecuteMsg {
    SwapMinOutput {
        target_denom: String,
        min_output_quantity: FPDecimal,
    },
}
