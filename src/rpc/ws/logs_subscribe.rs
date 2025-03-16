use crate::{
    bot::config::Config,
    utils::ws,
    types::{
        custom::Dex,
        rpc::LogsSubscribe
    }
};

use futures_util::{
    StreamExt, 
    SinkExt, 
};
use tokio::sync::mpsc;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        self, 
        protocol::Message
    }
};


pub async fn logs_subscribe(
    config: &Config,
    request_json_rpc: &serde_json::Value, 
    tx: &mpsc::Sender<(String, Dex)>,
    dex: &Dex
) -> Result<(), tungstenite::Error> {
    let (ws, _) = connect_async(&config.ws_url_mainnet).await?;
    let (mut write, mut read) = ws.split();
    
    if let Err(e) = write.send(Message::text(request_json_rpc.to_string())).await {
        if matches!(e, tungstenite::Error::ConnectionClosed | tungstenite::Error::AlreadyClosed) {
            log::error!("Handshake closed before making a subscription!"); 
        } else {
            log::error!("An error occurred while sending subscription request!");
            let _ = ws::try_to_close_connection(&mut write, None).await;  // skipping ? in order to return subscription req error
        }

        return Err(e);
    }

    loop {
        tokio::select! {
            Some(msg) = read.next() => {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Ok(logs_subscribe) = serde_json::from_str::<LogsSubscribe>(&text) {
                            dex.filter_creation_event(logs_subscribe, tx).await;
                        }
                    },
                    Ok(Message::Ping(v)) => ws::send_pong_frame(&mut write, v).await,
                    Ok(Message::Pong(_)) => log::info!("Received Pong Frame!"),
                    Ok(Message::Close(frame)) => {
                        log::info!("Received Close Frame!");
                        ws::try_to_close_connection(&mut write, frame).await?;  // as a response trying to send close frame 
                    },
                    Ok(_) => {},
                    Err(e) => {
                        match e {
                            tungstenite::Error::ConnectionClosed => break log::info!("Websocket connection is closed normally!"),
                            _ => return Err(e)
                        }
                    }
                }
            },

            // if received no msgs in 10 secs => send ping
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(10)) => {
                ws::send_ping_frame(&mut write).await;
            }
        }
    }

    Ok(())
}