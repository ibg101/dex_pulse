use crate::{
    rpc::client::RpcClient,
    constants::NATIVE_MINT,
    utils::parser::account::AccountType,
    types::{
        error::Error,
        custom::{Dex, PairMeta, Unpack, SharedTokenMeta},
        rpc::{CommitmentLevel, GetTransaction, GetAccountInfo}
    }
};

use std::sync::Arc;
use tokio::sync::mpsc;


/// ### Steps 2 & 3 are encapsulated to the PairMeta::try_to_parse_mint_accounts() method.
/// ### This non-blocking function performs 4 atomic stages:
/// 
/// 1. `get_transaction()` — Fetches the transaction data based on the given signature.
/// 2. `get_account_info()` — Retrieves account info relevant to the transaction.
/// 3. **Process encoded base58 data** from the step 2 and try to finalize `PairMeta`.
/// 4. Send the finalized `PairMeta` via the provided `tm_tx`.
///
/// If any of these stages fail, an error will be printed out in log::error!() and tx skipped.
pub async fn emit_processed_pair_meta(
    arc_rpc_client: Arc<RpcClient>,
    mut sig_rx: mpsc::Receiver<(String, Dex)>,
    pm_tx: mpsc::Sender<PairMeta>
) -> () {
    tokio::task::spawn(async move {
        while let Some((signature, dex)) = sig_rx.recv().await {
            log::info!("{dex:?} Recv: {}", signature);  // todo remove
            let arc_rpc_client: Arc<RpcClient> = Arc::clone(&arc_rpc_client);
            let pm_tx: mpsc::Sender<PairMeta> = pm_tx.clone();

            let handler= tokio::task::spawn(async move {
                let tx: GetTransaction = arc_rpc_client.get_transaction(signature, CommitmentLevel::Confirmed).await?;
                let mut pair_meta: PairMeta = dex.process_transaction(tx).await?;
                pair_meta.try_to_parse_mint_accounts(arc_rpc_client).await?;
                pair_meta.parse_provided_liq_ratio(); 
                pm_tx.send(pair_meta).await?;
                
                Ok(()) as Result<(), Box<dyn std::error::Error + Send + Sync>>
            });

            if let Ok(Err(e)) = handler.await { 
                log::error!("{e}");
            }
        }
    });
}

impl Dex {
    /// Will return PairMeta with already known fields, leaving unknown - default
    pub async fn process_transaction(&self, tx: GetTransaction) -> Result<PairMeta, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            Dex::Raydium => self.raydium_process_transaction(tx).await,
            Dex::Meteora => self.meteora_process_transaction(tx).await,
        }
    }
}

impl PairMeta {
    /// 1. Invokes `rpc_client.get_account_info()`
    /// 2. Tries to decode base58 encoded data
    /// 3. Tries to unpack `AccountType` from the decoded bytes
    /// 4. Replaces `None` to unpacked `AccountType` in `pair_meta.base|quote.mint_account`
    async fn try_to_parse_mint_accounts(
        &mut self,
        rpc_client: Arc<RpcClient>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let base_mint: &str = &self.base.mint;
        let quote_mint: &str = &self.quote.mint;

        let parse_decoded_data_as_account_type = |acc_info: GetAccountInfo| { 
            let encoded_data: &String = &acc_info.result.value.ok_or(Error::ProcessTransaction)?.data[0];
            let bytes: Vec<u8> = bs58::decode(encoded_data).into_vec()?;
            Ok(AccountType::unpack(&bytes)?) as Result<AccountType, Box<dyn std::error::Error + Send + Sync>> 
        };

        match NATIVE_MINT {
            native if base_mint == native => {
                let quote_info: GetAccountInfo = rpc_client.get_account_info(quote_mint).await?;
                let account_type: AccountType = parse_decoded_data_as_account_type(quote_info)?;
                self.quote.mint_account.replace(account_type);
            },
            native if quote_mint == native => {
                let base_info: GetAccountInfo = rpc_client.get_account_info(base_mint).await?;
                let account_type: AccountType = parse_decoded_data_as_account_type(base_info)?;
                self.base.mint_account.replace(account_type);
            },
            _ => {
                // for example PEPE/BONK , where base & quote == spl tokens
                let (base_info, quote_info) = tokio::join!(
                    rpc_client.get_account_info(base_mint),
                    rpc_client.get_account_info(quote_mint)
                );
                let base_account_type: AccountType = parse_decoded_data_as_account_type(base_info?)?;
                let quote_account_type: AccountType = parse_decoded_data_as_account_type(quote_info?)?;
                self.base.mint_account.replace(base_account_type);
                self.quote.mint_account.replace(quote_account_type);
            }
        };

        Ok(())
    }

    /// Calculates ratio based on formula `added/supply * 100`.
    fn parse_provided_liq_ratio(&mut self) -> () {
        for shared_meta in [&mut self.base, &mut self.quote] {
            if shared_meta.provided_liq_amount == 0 {  // this often happens in meteora pools   
                shared_meta.provided_liq_ratio = Some(0f64);
                continue; 
            }
            if let Some(AccountType::Mint { supply, .. }) = shared_meta.mint_account {
                shared_meta.provided_liq_ratio = Some((shared_meta.provided_liq_amount as f64 / supply as f64) * 100f64);
            } 
        }
    }  
}

/// Returns *base/quote* `&mut SharedTokenMeta` based on `is_base` condition.
pub fn get_mut_shared_token_meta(is_base: bool, pair_meta: &mut PairMeta) -> &mut SharedTokenMeta {
    if is_base {
        &mut pair_meta.base
    } else {
        &mut pair_meta.quote
    }
}