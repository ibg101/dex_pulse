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

pub async fn rpc_request_with_retries(
    json_rpc: serde_json::Value, 
    rpc_client: &RpcClient,
    max_retries: Option<u8>,
    retry_delay_secs: Option<u64>,
) -> Result<reqwest::Response, Box<dyn std::error::Error + Send + Sync>> {
    let mut attempt: u8 = 0;
    let max_retries: u8 = max_retries.unwrap_or(5);
    let retry_delay: u64 = retry_delay_secs.unwrap_or(1);

    let url: &str = &rpc_client.url;
    let http_client: &reqwest::Client = &rpc_client.http_client;

    let res: reqwest::Response = 
        loop {
            if attempt >= max_retries {
                return Err(Error::ReachedMaxRetries.into());
            }

            match http_client.post(url).json(&json_rpc).send().await {
                Ok(r) => break r,
                Err(e) => {
                    if e.is_timeout() {
                        return Err(e.into());
                    }

                    log::trace!("{e}");
                    attempt += 1;
                    tokio::time::sleep(tokio::time::Duration::from_secs(retry_delay)).await;
                }
            }
        };
    
    Ok(res)
}