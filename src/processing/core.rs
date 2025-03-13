use crate::{
    rpc::client::RpcClient,
    types::{
        custom::{Dex, TokenMeta},
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
    tm_tx: mpsc::Sender<TokenMeta>  // todo! create tokenmeta struct and use instead of ()
) -> () {
    tokio::task::spawn(async move {
        while let Some((signature, dex)) = sig_rx.recv().await {
            let arc_rpc_client: Arc<RpcClient> = Arc::clone(&arc_rpc_client);
            
            let handler= tokio::task::spawn(async move {
                let tx: GetTransaction = arc_rpc_client.get_transaction(signature, CommitmentLevel::Confirmed).await?;
                let token_meta_raw: TokenMeta = dex.process_transaction(tx).await;
                
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
    pub async fn process_transaction(&self, tx: GetTransaction) -> TokenMeta {
        match self {
            Dex::Raydium => self.raydium_process_transaction(tx).await,
            Dex::Meteora => self.meteora_process_transaction(tx).await,
        }
    }
}