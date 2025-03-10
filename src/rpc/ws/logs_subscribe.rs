use crate::{
    bot::config::Config,
    utils::ws,
    types::{
        enums::Dex, 
        structs::LogsSubscribe
    },
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
    tungstenite::{self, protocol::Message}
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
        let _ = write_guard.close().await;
        return Err(e);
    }
    drop(write_guard);   

    let keep_alive_handler: tokio::task::JoinHandle<()> = tokio::task::spawn(
        ws::keep_connection_alive(Arc::clone(&arc_write), 50)
    );

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(logs_subscribe) = serde_json::from_str::<LogsSubscribe>(&text) {
                    dex.filter_creation_event(logs_subscribe, tx).await;
                }
            },
            Ok(Message::Pong(_)) => log::info!("Received Pong Frame!"),
            Ok(Message::Close(_)) => {
                log::warn!("Received Close Frame!");
                keep_alive_handler.abort();
                ws::try_to_close_connection_arc(arc_write).await;
                break;
            },
            Ok(_) => {},
            Err(e) => {
                log::warn!("Received Error Frame! {e}");
                keep_alive_handler.abort();
                ws::try_to_close_connection_arc(arc_write).await;
                break;
            }
        }
    }

    Ok(())
}