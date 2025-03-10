use super::config::Config;
use crate::{
    observations,
    rpc::client::RpcClient,
    types::enums::{Dex, CommitmentLevel},
};

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

    // ---- processing tx ----
    let rpc_client: RpcClient = RpcClient::new_with_commitment(
        config.http_url_mainnet.clone(), 
        CommitmentLevel::Processed
    )?;
    // todo!();
    // ---- processing tx ----
    
    // ---- processing meta & filtering ----
    // todo!();
    // ---- processing meta & filtering ----

    while let Some((signature, dex)) = sig_rx.recv().await {
        let msg: String = format!("{dex:#?}\nLP creation signature: {}", signature);

        if let Err(e) = bot.send_message(config.channel_username.clone(), msg).await {
            log::error!("{:?}", e);
        }
    }

    Ok(())
}