use std::{
    sync::Arc,
    fmt::Display
};
use tokio::sync::{Mutex, MutexGuard};
use futures_util::{Sink, SinkExt};
use tokio_tungstenite::tungstenite::{
    Bytes,
    Utf8Bytes,
    protocol::{
        Message, 
        frame::{
            CloseFrame,
            coding::CloseCode
        }
    }
};


pub async fn try_to_close_connection_arc<T>(write: Arc<Mutex<T>>, close_code: CloseCode) -> () 
where
    T: SinkExt<Message> + Unpin,
    <T as Sink<Message>>::Error: Display
{
    let close_frame: CloseFrame = CloseFrame { code: close_code, reason: Utf8Bytes::from_static("") };
    let mut write_guard: MutexGuard<T> = write.lock().await;
    if let Err(e) = write_guard.send(Message::Close(Some(close_frame))).await {
        log::error!("Failed to properly close WSS!\nError: {e}");
    }
    if let Err(e) = write_guard.flush().await {
        log::error!("Failed to flush WebSocket messages! Possible data loss.\nError: {e}");
    }
}

pub async fn send_pong_mutex<T>(write: &Mutex<T>, data: Bytes) -> () 
where
    T: SinkExt<Message> + Unpin,
    <T as Sink<Message>>::Error: Display
{
    let mut write_guard: MutexGuard<T> = write.lock().await;
    if let Err(e) = write_guard.send(Message::Pong(data)).await {
        log::error!("Failed to send Pong Frame! {e}");
    }
    log::info!("Sent Pong Frame!");  // todo remove
}

pub async fn send_ping_mutex<T>(write: &Mutex<T>) -> () 
where
    T: SinkExt<Message> + Unpin,
    <T as Sink<Message>>::Error: Display
{
    let mut write_guard: MutexGuard<T> = write.lock().await;
    if let Err(e) = write_guard.send(Message::Ping(vec![].into())).await {
        log::error!("Failed to send Ping Frame! {e}");
    }
    log::info!("Sent Ping Frame!");  // todo remove
}