use crate::types::enums::CommitmentLevel;


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