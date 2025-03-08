#![allow(dead_code)]
use serde::Deserialize;


// ---- logs subscribe ----
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
    context: LogsSubscribeContext,
    pub value: LogsSubscribeValue<'a>,
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
    jsonrpc: &'a str,
    method: &'a str,
    pub params: LogsSubscribeParams<'a>,
}
// ---- logs subscribe ----