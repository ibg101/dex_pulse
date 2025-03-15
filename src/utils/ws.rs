use std::fmt::Display;
use tokio::sync::{Mutex, MutexGuard};
use futures_util::{Sink, SinkExt};
use tokio_tungstenite::tungstenite::{
    Bytes, 
    protocol::Message
};


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