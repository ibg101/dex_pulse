#![allow(dead_code)]
use serde::Deserialize;


#[derive(serde::Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CommitmentLevel {
    Processed,
    Confirmed,
    Finalized
}

// ---- rpc logs subscribe ----
// ! implementing custom LogsSubscribe instead of using one from solana_client, in order to minimize Heap Allocation 
//    in my impl (String is changed to &'a str) 
#[derive(Deserialize, Debug)]
pub struct LogsSubscribeValue<'a> {
    pub err: Option<serde_json::Value>,
    pub logs: Vec<&'a str>,
    pub signature: &'a str,
}

#[derive(Deserialize, Debug)]
struct LogsSubscribeContext {
    slot: u64,
}

#[derive(Deserialize, Debug)]
pub struct LogsSubscribeResult<'a> {
    pub value: LogsSubscribeValue<'a>,
    context: LogsSubscribeContext,
    _dummy: Option<&'a str>  // is used in order to pass lifetime to the children (without dummy field it won't compile)
}

#[derive(Deserialize, Debug)]
pub struct LogsSubscribeParams<'a> {
    pub result: LogsSubscribeResult<'a>,
    subscription: u64,
    _dummy: Option<&'a str>  // is used in order to pass lifetime to the children (without dummy field it won't compile)
}

#[derive(Deserialize, Debug)]
pub struct LogsSubscribe<'a> {
    pub params: LogsSubscribeParams<'a>,
    jsonrpc: &'a str,
    method: &'a str,
}
// ---- rpc logs subscribe ----


// ---- rpc tx instruction ----
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Instruction {
    stack_height: Option<serde_json::Value>,
    pub accounts: Vec<usize>,  // list of ordered indices into message.account_keys
    pub program_id_index: usize,
    pub data: String,  // bs58 encoded data
}

#[derive(Deserialize, Debug)]
pub struct InnerInstruction {
    pub instructions: Vec<Instruction>,
    index: u8  // unused field for indicing, so u8::MAX (255) is enough
}
// ---- rpc tx instruction ----


// ---- rpc token ----
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenAmount {
    pub amount: String,
    pub decimals: u8,
    pub ui_amount: Option<f64>,
    pub ui_amount_string: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalance {
    pub ui_token_amount: TokenAmount,
    pub mint: String,
    pub owner: String,
    pub program_id: String,
    pub account_index: usize,
}
// ---- rpc token ----


// ---- rpc transaction ----
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionHeader {
    pub num_readonly_signed_accounts: u64,
    pub num_readonly_unsigned_accounts: u64,
    pub num_required_signatures: u64
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionMessage {
    address_table_lookups: Option<serde_json::Value>,
    pub account_keys: Vec<String>,
    pub instructions: Vec<Instruction>,
    pub header: TransactionHeader,
    pub recent_blockhash: String,
}

#[derive(Deserialize, Debug)]
pub struct TransactionData {
    pub message: TransactionMessage,
    pub signatures: Vec<String>
}
// ---- rpc transaction ----


// ---- rpc get transaction ----
// !!! Some fields that contain serde_json::Value either arent important,
//   or not implemented yet.
// Please refer to https://solana.com/docs/rpc/http/gettransaction in order to access necessary fields.
#[derive(Deserialize, Debug)]
pub struct LoadedAddresses {
    pub writable: Vec<String>,
    pub readonly: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionMeta {
    err: Option<serde_json::Value>,
    post_balances: Vec<u64>,
    pub post_token_balances: Option<Vec<TokenBalance>>,
    pre_balances: Vec<u64>,
    pub pre_token_balances: Option<Vec<TokenBalance>>,
    fee: u64,
    pub inner_instructions: Option<Vec<InnerInstruction>>,
    rewards: Option<Vec<serde_json::Value>>,
    pub loaded_addresses: Option<LoadedAddresses>,
    return_data: Option<serde_json::Value>,
    compute_units_consumed: Option<serde_json::Value>,
    /// ## !!! Deprecated field !!!
    status: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct TransactionResult {
    pub meta: TransactionMeta,
    pub transaction: TransactionData,
    pub slot: u64,
    #[serde(rename = "version")]
    version: Option<serde_json::Value> 
}

#[derive(Deserialize, Debug)]
pub struct GetTransaction {
    pub result: TransactionResult,
    jsonrpc: String,
    #[serde(rename = "blockTime")]
    block_time: Option<u64>,
    id: u8
}
// ---- rpc get transaction ----


// ---- rpc get account info ----
#[derive(Deserialize, Debug)]
/// specifically for MarketID
pub struct GetAccountInfo {
    pub result: AccountInfoResult,
    jsonrpc: String,
    id: u8,
}

#[derive(Deserialize, Debug)]
pub struct AccountInfoResult {
    pub value: Option<AccountInfoValue>,
    context: AccountInfoContext,
}

#[derive(Deserialize, Debug)]
pub struct AccountInfoContext {
    #[serde(rename = "apiVersion")]
    api_version: String,
    slot: u64
}

#[derive(Deserialize, Debug)]
pub struct AccountInfoValue {
    // pub data: [String; 2],  // can be an object depending on encoding parameter (consider using serde_json::Value)
    pub data: serde_json::Value,
    executable: bool,
    lamports: usize,
    owner: String,
    #[serde(rename = "rentEpoch")]
    rent_epoch: usize,
    space: u64
}
// ---- rpc get account info ----