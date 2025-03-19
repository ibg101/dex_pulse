use crate::{
    rpc::client::RpcClient,
    types::{
        error::Error,
        custom::{Dex, TokenMeta, SharedTokenMeta},
        rpc::{CommitmentLevel, GetTransaction}
    }
};

use std::sync::Arc;
use tokio::sync::mpsc;


/// ### This non-blocking function performs 3 atomic stages:
/// 
/// 1. `get_transaction()` — Fetches the transaction data based on the given signature.
/// 2. `get_account_info()` — Retrieves account info relevant to the transaction.
/// 3. **Process & filter available data** — Filters the data and emits the filtered token metadata in case of success.
///
/// If any of these stages fail, an error will be printed out in log::error!() and tx skipped.
pub async fn emit_filtered_token_meta(
    arc_rpc_client: Arc<RpcClient>,
    mut sig_rx: mpsc::Receiver<(String, Dex)>,
    tm_tx: mpsc::Sender<TokenMeta>
) -> () {
    tokio::task::spawn(async move {
        while let Some((signature, dex)) = sig_rx.recv().await {
            log::info!("{dex:?} Recv: {}", signature);  // todo remove
            let arc_rpc_client: Arc<RpcClient> = Arc::clone(&arc_rpc_client);
            
            let handler= tokio::task::spawn(async move {
                let tx: GetTransaction = arc_rpc_client.get_transaction(signature, CommitmentLevel::Confirmed).await?;
                let token_meta_raw: TokenMeta = dex.process_transaction(tx).await?;
                log::info!("{dex:?}\n{:#?}\n", token_meta_raw);  // todo remove !
                
                
                Ok(()) as Result<(), Box<dyn std::error::Error + Send + Sync>>
            });

            if let Ok(Err(e)) = handler.await { 
                log::error!("{e}");
            }
        }
    });
}

impl Dex {
    /// Will return TokenMeta with already known fields, leaving unknown - default
    pub async fn process_transaction(&self, tx: GetTransaction) -> Result<TokenMeta, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            Dex::Raydium => self.raydium_process_transaction(tx).await,
            Dex::Meteora => self.meteora_process_transaction(tx).await,
        }
    }
}

/// Returns *base/quote* `&mut SharedTokenMeta` based on `is_base` condition.
pub fn get_mut_shared_token_meta(is_base: bool, token_meta: &mut TokenMeta) -> &mut SharedTokenMeta {
    if is_base {
        &mut token_meta.base
    } else {
        &mut token_meta.quote
    }
}