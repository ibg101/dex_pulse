use crate::{
    bot::config::Config,
    rpc::ws::logs_subscribe::logs_subscribe,
    utils::rpc::craft_logs_subscribe_json_rpc,
    types::{
        custom::Dex,
        rpc::LogsSubscribe
    },
    constants::{
        RAYDIUM_LP_V4_PROGRAM_ID,
        METEORA_DLMM_PROGRAM_ID
    }
};

use tokio::sync::mpsc;


const RECONNECT_DELAY: u64 = 5;
const RECONNECT_MAX_DELAY: u64 = 300;

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
        // The mentions field currently only supports one Pubkey string per method call. 
        // Specifying & Listing to additional addresses will result in an error. 
        tokio::task::spawn(async move {
            let mut exp_backoff: u32 = 1;
            
            loop {
                let start_subscription: tokio::time::Instant = tokio::time::Instant::now();

                if let Err(e) = logs_subscribe(
                    &config_clone, 
                    &req_json_rpc, 
                    &sig_tx_clone, 
                    &dex
                ).await {
                    log::error!("WSS error occurred! {e}");
                }

                if start_subscription.elapsed() >= tokio::time::Duration::from_secs(60) {
                    exp_backoff = 1;  // reset exponential if elapsed >= 60 seconds
                } else {
                    exp_backoff += 1;  // otherwise increase (this approach follows best practices)
                }

                handle_log_retry(RECONNECT_DELAY, exp_backoff).await;
            }
        });
    }
}

async fn handle_log_retry(delay: u64, exp_backoff: u32) -> () {    
    let exp_delay: u64 = std::cmp::min(delay * 2u64.pow(exp_backoff), RECONNECT_MAX_DELAY);
    log::warn!("Reconnecting in {} seconds..", exp_delay);
    tokio::time::sleep(tokio::time::Duration::from_secs(exp_delay)).await;
}

impl Dex {
    // using static dispatch instead of dynamic dispatch with BoxFuture closure inside logs_subscribe()
    pub async fn filter_creation_event(&self, logs_subscribe: LogsSubscribe<'_>, tx: &mpsc::Sender<(String, Dex)>) -> () {
        match self {
            Self::Raydium => self.raydium_lp_creation_event(logs_subscribe, tx).await,
            Self::Meteora => self.meteora_lp_creation_event(logs_subscribe, tx).await,
        }
    }
}