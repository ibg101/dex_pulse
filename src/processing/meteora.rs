use crate::types::{
    rpc::GetTransaction,
    custom::{TokenMeta, Dex}
};


impl Dex {
    /// 1. Try to decode AddLiquidity instruction
    /// 2. Check whether it's a first AddLiquidity tx (if before_amount.pool_base_token == 0)
    /// 3. 
    pub async fn meteora_process_transaction(&self, tx: GetTransaction) -> TokenMeta {

        
        // ... todo
        TokenMeta {
            ..Default::default()
        }
    } 
}