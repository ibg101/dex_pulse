use super::core::{
    get_mut_shared_token_meta,
    check_necessary_fields_filled
};
use crate::types::{
    error::Error,
    custom::{
        Dex, 
        PairMeta
    },
    rpc::GetTransaction
};


impl Dex {
    pub async fn pumpswap_process_transaction(&self, tx: GetTransaction) -> Result<PairMeta, Box<dyn std::error::Error + Send + Sync>> {
        println!("CAUGHT PUMPSWAP: {tx:#?}");
        todo!()
    }
}