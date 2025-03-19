use crate::{
    rpc::client::RpcClient,
    types::rpc::GetAccountInfo,
    utils::rpc::rpc_request_with_retries
};


impl RpcClient {
    pub async fn get_account_info<P: AsRef<[u8]> + serde::Serialize>(
        &self, 
        pubkey: P
    ) -> Result<GetAccountInfo, Box<dyn std::error::Error + Send + Sync>> {
        let json_rpc: serde_json::Value = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getAccountInfo",
            "params": [
                pubkey,
                {
                    "encoding": "base58"
                }
            ]
        });
        
        rpc_request_with_retries::<GetAccountInfo>(
            json_rpc, 
            self, 
            None, 
            None
        ).await
    }
}