use crate::global::data::PRIVATE_SOCKET;
use async_lock::MutexGuard;
use std::collections::HashMap;
use std::net::TcpStream;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::WebSocket;

pub async fn get_private_socket() -> MutexGuard<'static, WebSocket<MaybeTlsStream<TcpStream>>> {
    return PRIVATE_SOCKET.lock().await;
}
