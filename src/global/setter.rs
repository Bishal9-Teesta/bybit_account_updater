use crate::global::data::PRIVATE_SOCKET;
use std::collections::HashMap;
use std::net::TcpStream;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::WebSocket;

pub fn set_private_socket(new_private_socket: &WebSocket<MaybeTlsStream<TcpStream>>) {
    println!("{:?}", new_private_socket);
}
