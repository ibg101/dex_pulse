use super::config::Config;
use crate::observations::raydium;

use teloxide::{
    Bot,
    prelude::Requester,
};
use tokio::sync::mpsc;


pub async fn run(config: Config) -> () {
    let bot: Bot = Bot::from_env();

    let (sig_tx, mut sig_rx) = mpsc::channel::<String>(100);
    let config_clone: Config = config.clone();  // this is cheap, Arc is overkill

    tokio::task::spawn(async move {
        if let Err(e) = raydium::logs_subscribe(&config_clone, sig_tx).await {
            log::error!("CRITICAL ERROR: {e}");  // must reconnect 
        }
    });

    while let Some(signature) = sig_rx.recv().await {
        let msg: String = format!("LP creation signature: {}", signature);

        if let Err(e) = bot.send_message(config.channel_username.clone(), msg).await {
            log::error!("{:?}", e);
        }
    }
}