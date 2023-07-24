// Dependency
use ring::hmac;
use std::fs::read;
use std::io::{self, Read};
use std::{sync, thread};
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message, WebSocket};
// use std::ops::Deref;

use crate::structure::stream::{
    ExecutionChannel, Operation, OrderChannel, Ping, Pong, PositionChannel, Request,
    SuccessResponse, SuccessSocketData, Topic, WalletChannel,
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

    // let (mut socket_stream, _socket_response) =
    //     tungstenite::connect(url).expect("Bybit Private Stream connection establishing error!");
    //
    // // Socket Connection established
    // if socket_stream.can_read() && socket_stream.can_write() {
    //     println!(
    //         "Bybit Private Stream connection established successfully at {}",
    //         chrono::Local::now().format("%A, %d %B, %Y - %H:%M:%S")
    //     )
    // }
    //
    // if socket_stream.can_write() {
    //     // Authenticate to Private Stream Socket
    //     socket_stream
    //         .write_message(tungstenite::Message::Text(auth_text_request))
    //         .unwrap();
    //
    //     // Subscribe to channels
    //     socket_stream
    //         .write_message(tungstenite::Message::Text(subscription_text_request))
    //         .unwrap();
    // }
    //
    // let mut socket_for_ping = sync::Arc::new(sync::RwLock::new(socket_stream));
    // let socket_for_data = socket_for_ping.clone();
    // // let mut socket = sync::Arc::new(sync::RwLock::new(socket_stream));
    //
    // // This thread will only perform ping
    // let ping_thread = thread::spawn({
    //     // let mut socket = sync::Arc::new(socket_for_ping);
    //     move || {
    //         loop {
    //             // thread::sleep(std::time::Duration::from_secs(15));
    //             // println!("Socket Can Write: {}", socket_for_ping.write().unwrap().can_write());
    //             if socket_for_ping.clone().write().unwrap().can_write() {
    //                 // socket_for_ping
    //                 //     .clone()
    //                 //     .write()
    //                 //     .unwrap()
    //                 //     .write_message(tungstenite::Message::Text(
    //                 //         r#"{ "req_id": "Rust_Subscribe", "op": "ping" }"#.to_string(),
    //                 //     ))
    //                 //     .expect(&*format!(
    //                 //         "Error in sending ping request at {}.",
    //                 //         chrono::Local::now().format("%A, %d %B, %Y - %H:%M:%S")
    //                 //     ))
    //                 println!("Ping sent at {}", chrono::Local::now().format("%A, %d %B, %Y - %H:%M:%S"));
    //             }
    //         }
    //     }
    // });
    //
    // // This thread will handle all socket related performance
    // let socket_handler_thread = thread::spawn({
    //     // let mut socket = sync::Arc::new(socket_for_data);
    //     move || {
    //         let mut last_ping_sent = chrono::Local::now().timestamp();
    //         loop {
    //             if socket_for_data.clone().write().unwrap().can_read() {
    //
    //
    //                 // let current_timestamp = chrono::Local::now().timestamp();
    //                 // if last_ping_sent + 5 == current_timestamp {
    //                 //     println!("Send ping at {}", chrono::Local::now().format("%A, %d %B, %Y - %H:%M:%S"));
    //                 //     last_ping_sent = chrono::Local::now().timestamp();
    //                 // }
    //
    //
    //                 // let Ok(received) = socket_for_data.write().unwrap().read_message() else {
    //                 //          println!("Private function call");
    //                 //          private();
    //                 //          break;
    //                 //      };
    //                 // let received = socket_for_data.clone().write().unwrap().read_message().unwrap();
    //                 // println!("{:#?}", received);
    //
    //                 match socket_for_data.clone().write().unwrap().read_message() {
    //                     Ok(received) => {
    //
    //                         if received.is_binary() {
    //                             println!("Socket Data is Binary!")
    //                         }
    //                         if received.is_ping() {
    //                             println!("Socket Data is Ping!")
    //                         }
    //                         if received.is_pong() {
    //                             println!("Socket Data is Pong!")
    //                         }
    //                         if received.is_close() {
    //                             println!(
    //                                 "Socket Data is Closed at {}",
    //                                 chrono::Local::now().format("%A, %d %B, %Y - %H:%M:%S")
    //                             );
    //                             private();
    //                         }
    //
    //                         if received.is_text() {
    //                             println!("Socket Data is Text!!");
    //
    //                             // let raw_data = received.to_text().unwrap();
    //                             // if raw_data.contains("Rust_Auth") {
    //                             //     let data: SuccessResponse = serde_json::from_str(raw_data).unwrap();
    //                             //     println!("Successful Authentication Response: {:#?}", data);
    //                             // } else if raw_data.contains("Rust_Subscribe") && raw_data.contains("pong") {
    //                             //     // if raw_data.contains("pong") {
    //                             //     let data: Pong = serde_json::from_str(raw_data).unwrap();
    //                             //     println!("Successful Pong Response: {:#?}", data);
    //                             //     // }
    //                             // } else {
    //                             //
    //                             //     if raw_data.contains("position") {
    //                             //         let position_data: PositionChannel =
    //                             //             serde_json::from_str(raw_data.to_string().as_str()).unwrap();
    //                             //         println!("Position Channel Data: {:#?}", position_data);
    //                             //     } else if raw_data.contains("execution") {
    //                             //         let execution_data: ExecutionChannel =
    //                             //             serde_json::from_str(raw_data.to_string().as_str()).unwrap();
    //                             //         println!("Execution Channel Data: {:#?}", execution_data);
    //                             //     } else if raw_data.contains("order") {
    //                             //         let order_data: OrderChannel =
    //                             //             serde_json::from_str(raw_data.to_string().as_str()).unwrap();
    //                             //         println!("Order Channel Data: {:#?}", order_data);
    //                             //     } else if raw_data.contains("wallet") {
    //                             //         let wallet_data: WalletChannel =
    //                             //             serde_json::from_str(raw_data.to_string().as_str()).unwrap();
    //                             //         println!("Wallet Channel Data: {:#?}", wallet_data);
    //                             //     } else {
    //                             //         println!("Got None")
    //                             //     }
    //                             // }
    //                         }
    //                     },
    //                     Err(e) => {
    //
    //                     }
    //                 }
    //
    //                 // if received.is_binary() {
    //                 //     println!("Socket Data is Binary!")
    //                 // }
    //                 // if received.is_ping() {
    //                 //     println!("Socket Data is Ping!")
    //                 // }
    //                 // if received.is_pong() {
    //                 //     println!("Socket Data is Pong!")
    //                 // }
    //                 // if received.is_close() {
    //                 //     println!(
    //                 //         "Socket Data is Closed at {}",
    //                 //         chrono::Local::now().format("%A, %d %B, %Y - %H:%M:%S")
    //                 //     );
    //                 //     private();
    //                 // }
    //                 //
    //                 // if received.is_text() {
    //                 //     println!("Socket Data is Text!!");
    //                 //
    //                 //     // let raw_data = received.to_text().unwrap();
    //                 //     // if raw_data.contains("Rust_Auth") {
    //                 //     //     let data: SuccessResponse = serde_json::from_str(raw_data).unwrap();
    //                 //     //     println!("Successful Authentication Response: {:#?}", data);
    //                 //     // } else if raw_data.contains("Rust_Subscribe") && raw_data.contains("pong") {
    //                 //     //     // if raw_data.contains("pong") {
    //                 //     //     let data: Pong = serde_json::from_str(raw_data).unwrap();
    //                 //     //     println!("Successful Pong Response: {:#?}", data);
    //                 //     //     // }
    //                 //     // } else {
    //                 //     //
    //                 //     //     if raw_data.contains("position") {
    //                 //     //         let position_data: PositionChannel =
    //                 //     //             serde_json::from_str(raw_data.to_string().as_str()).unwrap();
    //                 //     //         println!("Position Channel Data: {:#?}", position_data);
    //                 //     //     } else if raw_data.contains("execution") {
    //                 //     //         let execution_data: ExecutionChannel =
    //                 //     //             serde_json::from_str(raw_data.to_string().as_str()).unwrap();
    //                 //     //         println!("Execution Channel Data: {:#?}", execution_data);
    //                 //     //     } else if raw_data.contains("order") {
    //                 //     //         let order_data: OrderChannel =
    //                 //     //             serde_json::from_str(raw_data.to_string().as_str()).unwrap();
    //                 //     //         println!("Order Channel Data: {:#?}", order_data);
    //                 //     //     } else if raw_data.contains("wallet") {
    //                 //     //         let wallet_data: WalletChannel =
    //                 //     //             serde_json::from_str(raw_data.to_string().as_str()).unwrap();
    //                 //     //         println!("Wallet Channel Data: {:#?}", wallet_data);
    //                 //     //     } else {
    //                 //     //         println!("Got None")
    //                 //     //     }
    //                 //     // }
    //                 // }
    //             }
    //         }
    //     }
    // });
    //
    // ping_thread.join().unwrap();
    // socket_handler_thread.join().unwrap();









    let (mut ws, _) = connect(url).unwrap();

    // Set read timeout to the underlying TCP stream.
    //
    // Read and write are both in the main thread loop. A blocking read call
    // will starve writing that causes ping op message can't be sent on time.
    // Read timeout mitigate this situation.
    set_read_timeout(&ws);

    // Authenticate
    // if let Some(credentials) = credentials {
    //     let req = auth_req(credentials);
        ws.write_message(Message::Text(auth_text_request));
    // }

    // Subscribe
    ws.write_message(Message::Text(subscription_text_request));

    let rx = ping();

    loop {
        // Ping
        if let Ok(ping) = rx.try_recv() {
            ws.write_message(Message::Text(ping.into()));
        };

        match ws.read_message() {
            Ok(msg) => match msg {
                Message::Text(content) => {
                    println!("Received: {}", content);
                    // match serde_json::from_str(&content) {
                    //     Ok(res) => println!("{:#?}", res),
                    //     Err(e) => println!("Error: {}", e),
                    // }
                }
                _ => {}
            },
            Err(e) => match e {
                tungstenite::Error::Io(ref ee) => {
                    if ee.kind() != std::io::ErrorKind::WouldBlock
                        && ee.kind() != std::io::ErrorKind::TimedOut
                    {
                        Err(e).unwrap()
                    }
                }
                _ => Err(e).unwrap(),
            },
        }
    }
}

fn set_read_timeout(ws: &WebSocket<MaybeTlsStream<TcpStream>>) {
    match ws.get_ref() {
        MaybeTlsStream::Plain(s) => {
            s.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
        }
        MaybeTlsStream::Rustls(t) => {
            t.get_ref()
                .set_read_timeout(Some(Duration::from_secs(10)))
                .unwrap();
        }
        _ => unreachable!(),
    };
}

fn ping() -> Receiver<&'static str> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || loop {
        if let Err(_) = tx.send("{\"op\":\"ping\"}") {
            break;
        };
        thread::sleep(Duration::from_secs(20));
    });
    rx
}