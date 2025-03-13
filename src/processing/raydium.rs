use crate::types::{
    rpc::GetTransaction,
    custom::{TokenMeta, Dex}
};


impl Dex {
    pub async fn raydium_process_transaction(&self, tx: GetTransaction) -> TokenMeta {
        TokenMeta {
            ..Default::default()
        }
    }
}