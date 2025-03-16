use std::fmt::Display;
use futures_util::{Sink, SinkExt};
use tokio_tungstenite::tungstenite::{
    self,
    Bytes, 
    protocol::{
        Message, 
        CloseFrame
    }
};


pub async fn try_to_close_connection<T>(write: &mut T, close_frame: Option<CloseFrame>) -> Result<(), tungstenite::Error>
where
    T: SinkExt<Message, Error = tungstenite::Error> + Unpin,
{
    log::info!("Trying to send a Close Frame!");
    if let Err(e) = write.send(Message::Close(close_frame)).await {
        log::error!("Failed to send a Close Frame!\nError: {e}");
        return Err(e);
    }
    log::info!("Connection is properly closed!");
    Ok(())
}

pub async fn send_pong_frame<T>(write: &mut T, data: Bytes) -> () 
where
    T: SinkExt<Message> + Unpin,
    <T as Sink<Message>>::Error: Display
{
    if let Err(e) = write.send(Message::Pong(data)).await {
        log::error!("Failed to send Pong Frame! {e}");
    }
    log::info!("Sent Pong Frame!");
}

pub async fn send_ping_frame<T>(write: &mut T) -> () 
where
    T: SinkExt<Message> + Unpin,
    <T as Sink<Message>>::Error: Display
{
    if let Err(e) = write.send(Message::Ping(vec![].into())).await {
        log::error!("Failed to send Ping Frame! {e}");
    }
    log::info!("Sent Ping Frame!");
}