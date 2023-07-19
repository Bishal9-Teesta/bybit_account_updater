// Dependency
use ring::hmac;
use std::fs::read;
use std::io::{self, Read};
use std::{sync, thread};

use crate::structure::stream::{
    ExecutionChannel, Operation, OrderChannel, Ping, PositionChannel, Request, SuccessResponse,
    Topic, WalletChannel,
};

pub fn private() {
    let url = crate::config::base_url::get_base_url().socket_private;
    let api_key = crate::config::key::get_keys().api_key;
    let secret_key = crate::config::key::get_keys().secret_key;
    let future_expiry_timestamp =
        (chrono::Local::now() + chrono::Duration::hours(12)).timestamp_millis();

    // Signing Message
    let param = format!("GET/realtime{expires}", expires = future_expiry_timestamp);
    let signing_key = hmac::Key::new(hmac::HMAC_SHA256, secret_key.as_bytes());
    let signature = hmac::sign(&signing_key, &param.as_bytes());
    let signature = hex::encode(signature.as_ref());
    // println!("Signature: {:#?}", signature);

    // Generating Authentication Payload
    let auth_request: Request = Request {
        req_id: "Rust_Auth".to_string(),
        op: Operation::auth,
        args: Vec::from([
            api_key.to_string(),
            future_expiry_timestamp.to_string(),
            signature,
        ]),
    };
    let auth_text_request = serde_json::to_string(&auth_request).unwrap();
    // println!("Auth Text Request: {:#?}", text_request);

    // Generating Subscription Payload
    let subscription_request: Request = Request {
        req_id: "Rust_Subscribe".to_string(),
        op: Operation::subscribe,
        args: Vec::from([
            "position".to_string(),
            "execution".to_string(),
            "order".to_string(),
            "wallet".to_string(),
        ]),
    };
    let subscription_text_request = serde_json::to_string(&subscription_request).unwrap();
    // println!("Subscription Text Request: {:#?}", subscription_text_request);

    let (mut socket_stream, _socket_response) =
        tungstenite::connect(url).expect("Bybit Private Stream connection establishing error!");

    // Socket Connection established
    if socket_stream.can_read() && socket_stream.can_write() {
        println!(
            "Bybit Private Stream connection established successfully at {}",
            chrono::Local::now().format("%A, %d %B, %Y - %H:%M:%S")
        )
    }

    if socket_stream.can_write() {
        // Authenticate to Private Stream Socket
        socket_stream
            .write_message(tungstenite::Message::Text(auth_text_request))
            .unwrap();

        // Subscribe to channels
        socket_stream
            .write_message(tungstenite::Message::Text(subscription_text_request))
            .unwrap();
    }

    let mut socket_for_ping = sync::Arc::new(socket_stream);
    let mut socket_for_data = socket_for_ping.clone();
    // let mut socket = sync::Arc::new(socket_stream);

    // This thread will only perform ping
    let ping_thread = thread::spawn({
        let mut socket = sync::RwLock::new(socket_for_ping);
        move || unsafe {
            loop {
                println!("Socket Can Write: {}", socket.read().unwrap().can_write())
                // if socket.can_write() {
                // socket
                //     .write_message(tungstenite::Message::Text(
                //         r#"{ "req_id": "Rust_Subscribe", "op": "ping" }"#.to_string(),
                //     ))
                //     .expect(&*format!(
                //         "Error in sending ping request at {}.",
                //         chrono::Local::now().format("%A, %d %B, %Y - %H:%M:%S")
                //     ));
                // }
            }
        }
    });

    // This thread will handle all socket related performance
    let socket_handler_thread = thread::spawn({
        let mut socket = sync::RwLock::new(socket_for_data);
        move || unsafe {
            loop {
                println!("Socket Can Read: {}", socket.read().unwrap().can_read());
                // if socket_stream.can_read() {
                //     let Ok(received) = socket_stream.read_message() else {
                //              println!("Private function call");
                //              private();
                //              break;
                //          };
                //     println!("{:#?}", received);
                // }
            }
        }
    });

    ping_thread.join().unwrap();
    socket_handler_thread.join().unwrap();
}
