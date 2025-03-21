use super::config::Config;
use crate::{
    observations, 
    processing, 
    rpc::client::RpcClient, 
    types::{
        rpc::CommitmentLevel,
        custom::{Dex, PairMeta}
    }
};

use std::sync::Arc;
use teloxide::{
    payloads::SendMessageSetters, prelude::Requester, types::ParseMode::MarkdownV2, Bot
};
use tokio::sync::mpsc;


pub async fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let bot: Bot = Bot::from_env();

    // ---- observation ----
    // centralized channel with all signatures, that must be processed.
    // !!! it doesn't store all signatures, parsed from the logs, but only filtered. !!!
    let (sig_tx, sig_rx) = mpsc::channel::<(String, Dex)>(100);
    observations::core::handle_all_logs_subscriptions(sig_tx, &config).await;
    // ---- observation ----

    // ---- processing tx & emitting pair meta ----
    let (pm_tx, mut pm_rx) = mpsc::channel::<PairMeta>(100);
    let rpc_client: RpcClient = RpcClient::new_with_commitment(
        config.http_url_mainnet.clone(), 
        CommitmentLevel::Processed
    )?;
    let arc_rpc_client: Arc<RpcClient> = Arc::from(rpc_client);
    processing::core::emit_processed_pair_meta(Arc::clone(&arc_rpc_client), sig_rx, pm_tx).await;

    while let Some(pair_meta) = pm_rx.recv().await {
        log::info!("Received finalized: {:#?}", pair_meta);  // todo remove
        let msg: String = processing::tg::build_post(pair_meta);

        if let Err(e) = bot.send_message(
            config.channel_username.clone(), 
            msg
        ).parse_mode(MarkdownV2).await {
            log::error!("Failed to make a TG post!\nError: {e}");
        }
    }

    Ok(())
}