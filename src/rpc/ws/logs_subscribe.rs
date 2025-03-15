use crate::{
    bot::config::Config,
    utils::ws,
    types::{
        custom::Dex,
        rpc::LogsSubscribe
    }
};

use std::sync::Arc;
use futures_util::{
    StreamExt, 
    SinkExt, 
    stream::SplitSink
};
use tokio::{
    net::TcpStream,
    sync::{mpsc, Mutex, MutexGuard}
};
use tokio_tungstenite::{
    connect_async,
    MaybeTlsStream,
    WebSocketStream,
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
    let (write, mut read) = ws.split();
    let arc_write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>> = Arc::new(Mutex::new(write));
    
    let mut write_guard: MutexGuard<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>> = arc_write.lock().await;
    if let Err(e) = write_guard.send(Message::text(request_json_rpc.to_string())).await {
        if matches!(e, tungstenite::Error::ConnectionClosed | tungstenite::Error::AlreadyClosed) {
            log::error!("Handshake closed before making a subscribtion!"); 
        } else {
            log::error!("An error occurred! Trying to send a Close Frame.."); 
            write_guard.send(Message::Close(None)).await?
        }

        return Err(e);
    }
    drop(write_guard);   

    loop {
        tokio::select! {
            Some(msg) = read.next() => {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Ok(logs_subscribe) = serde_json::from_str::<LogsSubscribe>(&text) {
                            dex.filter_creation_event(logs_subscribe, tx).await;
                        }
                    },
                    Ok(Message::Ping(v)) => ws::send_pong_mutex(&arc_write, v).await,
                    Ok(Message::Pong(_)) => log::info!("Received Pong Frame!"),
                    Ok(Message::Close(frame)) => {
                        log::info!("Received Close Frame! Trying to send a Close Frame as a response..");
                        return arc_write.lock().await.send(Message::Close(frame))
                            .await
                            .map(|_| log::info!("Connection is properly closed!"));
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
                ws::send_ping_mutex(&arc_write).await;
            }
        }
    }

    Ok(())
}