use crate::{
    rpc::client::RpcClient,
    utils::rpc::rpc_request_with_retries,
    types::enums::{Error, CommitmentLevel}
};


/// CommitmentLevel::Processed is not supported, so i ignore rpc_client.commitment,
/// and force you to specify the correct one explicitly in fn args.
pub async fn get_transaction(
    rpc_client: &RpcClient,
    signature: &str, 
    commitment: CommitmentLevel
) -> Result<(), Box<dyn std::error::Error>> {
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
                "encoding": "json",
                "commitment": commitment,
                "maxSupportedTransactionVersion": 0
            }
        ]
    });

    let res: reqwest::Response = rpc_request_with_retries(
        json_rpc, 
        &rpc_client, 
        None, 
        None
    ).await?;

    println!("{:#?}", res);  // todo!() remove

    Ok(())
}