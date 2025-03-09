use super::config::Config;
use crate::{
    observations,
    types::enums::Dex
};

use teloxide::{
    Bot,
    prelude::Requester,
};
use tokio::sync::mpsc;


pub async fn run(config: Config) -> () {
    let bot: Bot = Bot::from_env();

    // ---- observation ----
    // centralized channel with all signatures, that must be processed.
    let (sig_tx, mut sig_rx) = mpsc::channel::<(String, Dex)>(100);
    observations::core::handle_all_logs_subscriptions(sig_tx, &config).await;
    // ---- observation ----

    // ---- processing tx ----
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
}