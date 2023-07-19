use async_lock::Mutex;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::Arc;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::WebSocket;

lazy_static! {
    pub static ref PRIVATE_SOCKET: Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>> =
        Arc::new(Mutex::new(WebSocket::from_raw_socket(
            MaybeTlsStream::Plain(TcpStream::connect("").unwrap()),
            tungstenite::protocol::Role::Client,
            None
        )));
}
