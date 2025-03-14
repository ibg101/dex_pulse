use crate::{
    rpc::client::RpcClient,
    utils::rpc::rpc_request_with_retries,
    types::{
        rpc::GetTransaction,
        error::Error,
        rpc::CommitmentLevel
    }
};


impl RpcClient {
    /// CommitmentLevel::Processed is not supported, so i ignore self.commitment,
    /// and force you to specify the correct one explicitly in fn args.
    pub async fn get_transaction<S: AsRef<[u8]> + serde::Serialize>(
        &self,
        signature: S, 
        commitment: CommitmentLevel
    ) -> Result<GetTransaction, Box<dyn std::error::Error + Send + Sync>> {
        if commitment == CommitmentLevel::Processed {
            return Err(Error::UnsupportedCommitment.into());
        }

        let json_rpc: serde_json::Value = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getTransaction",
            "params": [
                signature,
                {
                    "encoding": "json",  // using json instead of jsonParsed in order to reach the max performance
                    "commitment": commitment,
                    "maxSupportedTransactionVersion": 0
                }
            ]
        });

        let res: reqwest::Response = rpc_request_with_retries(
            json_rpc, 
            self, 
            None, 
            None
        ).await?;
        let res_body: String = res.text().await?;

        serde_json::from_str::<GetTransaction>(&res_body).map_err(|e| e.into())
    }
}