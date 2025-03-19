use crate::{
    rpc::client::RpcClient,
    constants::NATIVE_MINT,
    utils::parser::account::AccountType,
    types::{
        error::Error,
        custom::{Dex, TokenMeta, SharedTokenMeta},
        rpc::{CommitmentLevel, GetTransaction, GetAccountInfo}
    }
};

use std::sync::Arc;
use tokio::sync::mpsc;


/// ### Steps 2 & 3 are encapsulated to the TokenMeta::try_to_parse_mint_accounts() method.
/// ### This non-blocking function performs 4 atomic stages:
/// 
/// 1. `get_transaction()` — Fetches the transaction data based on the given signature.
/// 2. `get_account_info()` — Retrieves account info relevant to the transaction.
/// 3. **Process encoded base58 data** from the step 2 and try to finalize `TokenMeta`.
/// 4. Send the finalized `TokenMeta` via the provided `tm_tx`.
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
            let tm_tx: mpsc::Sender<TokenMeta> = tm_tx.clone();

            let handler= tokio::task::spawn(async move {
                let tx: GetTransaction = arc_rpc_client.get_transaction(signature, CommitmentLevel::Confirmed).await?;
                let token_meta_raw: TokenMeta = dex.process_transaction(tx).await?;
                let token_meta: TokenMeta = token_meta_raw.try_to_parse_mint_accounts(arc_rpc_client).await?; 
                tm_tx.send(token_meta).await?;
                
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

impl TokenMeta {
    /// 1. Invokes `rpc_client.get_account_info()`
    /// 2. Tries to decode base58 encoded data
    /// 3. Tries to unpack `AccountType` from the decoded bytes
    /// 4. Replaces `None` to unpacked `AccountType` in `token_meta.base|quote.mint_account`
    async fn try_to_parse_mint_accounts(
        mut self,
        rpc_client: Arc<RpcClient>,
    ) -> Result<TokenMeta, Box<dyn std::error::Error + Send + Sync>> {
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

        Ok(self)
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