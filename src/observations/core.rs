use crate::{
    bot::config::Config,
    utils::json_rpc::craft_logs_subscribe_json_rpc,
    types::{
        enums::Dex,
        structs::LogsSubscribe
    },
    constants::{
        RAYDIUM_LP_V4_PROGRAM_ID,
        METEORA_DLMM_PROGRAM_ID
    }
};

use tokio::sync::mpsc;


pub async fn handle_all_logs_subscriptions(
    sig_tx: mpsc::Sender<(String, Dex)>,
    config: &Config
) -> () {
    let raydium_req_json_rpc: serde_json::Value = craft_logs_subscribe_json_rpc(RAYDIUM_LP_V4_PROGRAM_ID, None);
    let meteora_req_json_rpc: serde_json::Value = craft_logs_subscribe_json_rpc(METEORA_DLMM_PROGRAM_ID, None);
    let subscriptions_arr: [(serde_json::Value, Dex); 2] = [
        (raydium_req_json_rpc, Dex::Raydium),
        (meteora_req_json_rpc, Dex::Meteora)
    ];

    for (req_json_rpc, dex) in subscriptions_arr {
        let sig_tx_clone: mpsc::Sender<(String, Dex)> = sig_tx.clone();
        let config_clone: Config = config.clone();

        // Creating separated WSS for each DEX due to:
        // The mentions field currently only supports one Pubkey string per method call. Listing additional addresses will result in an error. 
        tokio::task::spawn(async move {
            const DELAY: u64 = 30;
            const DELAY_HANDSHAKE_ERROR: u64 = DELAY * 2;

            loop {
                if let Err(e) = super::logs::logs_subscribe(
                    &config_clone, 
                    &req_json_rpc, 
                    &sig_tx_clone, 
                    &dex
                ).await {
                    log::error!("WSS error occurred! {e}");
                    log::warn!("Reconnecting in {} seconds..", DELAY_HANDSHAKE_ERROR);
                    tokio::time::sleep(tokio::time::Duration::from_secs(DELAY_HANDSHAKE_ERROR)).await;
                }
                log::warn!("Reconnecting in {} seconds..", DELAY);
                tokio::time::sleep(tokio::time::Duration::from_secs(DELAY)).await;
            }
        });
    }
}


impl Dex {
    // using static dispatch instead of dynamic dispatch with BoxFuture closure inside logs_subscribe()
    pub async fn filter_creation_event(&self, logs_subscribe: LogsSubscribe<'_>, tx: &mpsc::Sender<(String, Dex)>) -> () {
        match self {
            Self::Raydium => self.raydium_creation_event(logs_subscribe, tx).await,
            Self::Meteora => self.meteora_creation_event(logs_subscribe, tx).await,
        }
    }
}