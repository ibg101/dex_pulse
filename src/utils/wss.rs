use tokio::net::TcpStream;
use futures_util::{SinkExt, stream::SplitSink};
use tokio_tungstenite::{
    MaybeTlsStream,
    WebSocketStream,
    tungstenite::protocol::Message
};


pub async fn try_to_close_connection(
    write: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,  
) -> () {
    if let Err(e) = write.close().await {
        log::error!("Failed to properly close WSS! Reconnecting anyway..\nError msg: {e}");
    }
}