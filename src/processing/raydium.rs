use crate::types::{
    error::Error,
    rpc::GetTransaction,
    custom::{TokenMeta, Dex}
};


impl Dex {
    pub async fn raydium_process_transaction(&self, tx: GetTransaction) -> Result<TokenMeta, Box<dyn std::error::Error + Send + Sync>> {
        Ok(TokenMeta {
            ..Default::default()
        })
    }
}