use super::config::Config;
use crate::{
    observations, 
    processing, 
    rpc::client::RpcClient, 
    types::{
        rpc::CommitmentLevel,
        custom::{Dex, TokenMeta}
    }
};

use std::sync::Arc;
use teloxide::{
    Bot,
    prelude::Requester,
};
use tokio::sync::mpsc;


pub async fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let bot: Bot = Bot::from_env();

    // ---- observation ----
    // centralized channel with all signatures, that must be processed.
    let (sig_tx, mut sig_rx) = mpsc::channel::<(String, Dex)>(100);
    observations::core::handle_all_logs_subscriptions(sig_tx, &config).await;
    // ---- observation ----

    // ---- processing & filtering tx => emitting token meta ----
    // let (tm_tx, tm_rx) = mpsc::channel::<TokenMeta>(100);
    // let rpc_client: RpcClient = RpcClient::new_with_commitment(
    //     config.http_url_mainnet.clone(), 
    //     CommitmentLevel::Processed
    // )?;
    // let arc_rpc_client: Arc<RpcClient> = Arc::from(rpc_client);
    // processing::core::emit_filtered_token_meta(Arc::clone(&arc_rpc_client), sig_rx, tm_tx).await;

    while let Some((signature, dex)) = sig_rx.recv().await {
        let msg: String = format!("{dex:#?}\nLP creation signature: {}", signature);

        if let Err(e) = bot.send_message(config.channel_username.clone(), msg).await {
            log::error!("{:?}", e);
        }
    }

    Ok(())
}