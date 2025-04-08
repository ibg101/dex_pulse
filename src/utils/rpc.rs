use crate::{
    rpc::client::RpcClient,
    types::{
        error::Error,
        rpc::CommitmentLevel
    },
};


pub fn craft_logs_subscribe_json_rpc(program_id: &str, commitment: Option<CommitmentLevel>) -> serde_json::Value {
    serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "logsSubscribe",
        "params": [
            {
                "mentions": [ program_id ]
            },
            {
                "commitment": commitment.unwrap_or(CommitmentLevel::Processed)
            }
        ]
    })
}

pub async fn rpc_request_with_retries<T: serde::de::DeserializeOwned>(
    json_rpc: serde_json::Value, 
    rpc_client: &RpcClient,
    max_retries: Option<u8>,
    retry_delay_millis: Option<u64>,
) -> Result<T, Box<dyn std::error::Error + Send + Sync>> {
    let max_retries: u8 = max_retries.unwrap_or(5);
    let retry_delay: u64 = retry_delay_millis.unwrap_or(1000);

    let url: &str = &rpc_client.url;
    let http_client: &reqwest::Client = &rpc_client.http_client;

    for attempt in 0..max_retries {
        match http_client.post(url).json(&json_rpc).send().await {
            Ok(res) => {                
                if let Ok(r) = res.json::<T>().await {
                    return Ok(r);
                }
                log::warn!("{} - Failed to deserialize! Retrying in {} millis..", attempt + 1, retry_delay);
                tokio::time::sleep(tokio::time::Duration::from_millis(retry_delay)).await;
            },
            Err(e) if e.is_timeout() => return Err(e.into()),
            Err(e) => {
                log::warn!("{} - Failed to make a HTTP RPC request! Retrying in {} millis..\nError: {e}", attempt + 1, retry_delay);
                tokio::time::sleep(tokio::time::Duration::from_millis(retry_delay)).await;
            }
        } 
    }

    Err(Error::ReachedMaxRetries.into())
}