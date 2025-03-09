use std::sync::Arc;
use tokio::{
    net::TcpStream,
    sync::{Mutex, MutexGuard}
};
use futures_util::{SinkExt, stream::SplitSink};
use tokio_tungstenite::{
    MaybeTlsStream,
    WebSocketStream,
    tungstenite::protocol::Message
};


pub async fn try_to_close_connection_arc(
    write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,  
) -> () {
    let mut write_guard: MutexGuard<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>> = write.lock().await;
    if let Err(e) = write_guard.close().await {
        log::error!("Failed to properly close WSS! Reconnecting anyway..\nError msg: {e}");
    }
}

pub async fn keep_connection_alive(
    write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    send_delay_in_secs: u64
) -> () {
    loop {
        let mut write_guard: MutexGuard<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>> = write.lock().await;
        if let Err(e) = write_guard.send(Message::Ping(vec![].into())).await {
            log::error!("Failed to send Ping Frame! {e}");
        } else {
            log::info!("Sent Ping!");
        }
        drop(write_guard);  // because it will sleep before releasing lock

        tokio::time::sleep(tokio::time::Duration::from_secs(send_delay_in_secs)).await;
    }
}