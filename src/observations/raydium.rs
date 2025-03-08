use crate::{
    bot::config::Config,
    utils::wss,
    types::structs::LogsSubscribe,
    constants::{
        RAYDIUM_INSTRUCTION_CREATE_NEW_LP,
        RAYDIUM_INSTRUCTION_SUCCESSFUL_CREATION_NEW_LP
    }
};

use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{self, protocol::Message}
};


pub async fn logs_subscribe(config: &Config, tx: mpsc::Sender<String>) -> Result<(), tungstenite::Error> {
    let (ws, _) = connect_async(&config.wss_url_mainnet).await?;
    let (mut write, mut read) = ws.split();

    let request_json_rpc: serde_json::Value = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "logsSubscribe",
        "params": [
            {
                "mentions": [ crate::constants::RAYDIUM_LP_V4_PROGRAM_ID ]
            },
            {
                "commitment": "confirmed"
            }
        ]
    });

    if let Err(e) = write.send(Message::text(request_json_rpc.to_string())).await {
        log::error!("Failed to make a subscription! {e}");
    }

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(logs_subscribe) = serde_json::from_str::<LogsSubscribe>(&text) {
                    let logs: Vec<&str> = logs_subscribe.params.result.value.logs;
                    let mut is_creation_instruction: bool = false;
                    let mut is_successfully_created: bool = false; 

                    for log in logs.into_iter() {
                        if is_creation_instruction && is_successfully_created { break; }

                        if log.contains(RAYDIUM_INSTRUCTION_CREATE_NEW_LP) {
                            is_creation_instruction = true;
                        }

                        if log.contains(RAYDIUM_INSTRUCTION_SUCCESSFUL_CREATION_NEW_LP) {
                            is_successfully_created = true;
                        }
                    }

                    if is_creation_instruction && is_successfully_created {
                        let signature: String = logs_subscribe.params.result.value.signature.to_owned();
                        if let Err(e) = tx.send(signature).await {
                            log::error!("Failed to extend signatures channel! {e}");
                        }
                    }
                }
            },
            Ok(Message::Pong(_)) => log::info!("Received Pong Frame!"),
            Ok(Message::Close(_)) => {
                log::warn!("Received Close Frame!");
                wss::try_to_close_connection(&mut write).await;
                break;
            },
            Ok(_) => {},
            Err(e) => {
                log::warn!("Received Error Frame! {e}");
                wss::try_to_close_connection(&mut write).await;
                break;
            }
        }
    }

    Ok(())
}