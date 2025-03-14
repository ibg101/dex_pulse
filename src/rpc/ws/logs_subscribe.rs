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
        Utf8Bytes, 
        protocol::{
            Message, 
            frame::{CloseFrame, coding::CloseCode}
        }
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
        let close_frame: CloseFrame = CloseFrame { code: CloseCode::Unsupported, reason: Utf8Bytes::from_static("") };
        let _ = write_guard.send(Message::Close(Some(close_frame))).await;
        let _ = write_guard.flush().await;
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
                    Ok(Message::Close(_)) => {
                        log::warn!("Received Close Frame!");
                        ws::try_to_close_connection_arc(arc_write, CloseCode::Normal).await;
                        break;
                    },
                    Ok(_) => {},
                    Err(e) => {
                        log::warn!("Received Error Frame! {e}");
                        ws::try_to_close_connection_arc(arc_write, CloseCode::Error).await;
                        break;
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