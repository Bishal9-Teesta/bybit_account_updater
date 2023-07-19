
fn() {
if chrono::Local::now().second() == 15 && sent_ping == false {
println!("Sending ping at {} second.", chrono::Local::now().second());

if socket_stream.can_write() {
socket_stream
.write_message(Message::Text(
r#"{ "req_id": "Rust_Subscribe", "op": "ping" }"#.to_string(),
))
.expect(&*format!(
"Error in sending ping request at {}.",
chrono::Local::now().format("%A, %d %B, %Y - %H:%M:%S")
));
}

sent_ping = true
} else if chrono::Local::now().second() == 16 {
sent_ping = false
} else if chrono::Local::now().second() == 30 && sent_ping == false {
println!("Sending ping at {} second.", chrono::Local::now().second());

if socket_stream.can_write() {
socket_stream
.write_message(Message::Text(
r#"{ "req_id": "Rust_Subscribe", "op": "ping" }"#.to_string(),
))
.expect(&*format!(
"Error in sending ping request at {}.",
chrono::Local::now().format("%A, %d %B, %Y - %H:%M:%S")
));
}

sent_ping = true
} else if chrono::Local::now().second() == 31 {
sent_ping = false
} else if chrono::Local::now().second() == 45 && sent_ping == false {
println!("Sending ping at {} second.", chrono::Local::now().second());

if socket_stream.can_write() {
socket_stream
.write_message(Message::Text(
r#"{ "req_id": "Rust_Subscribe", "op": "ping" }"#.to_string(),
))
.expect(&*format!(
"Error in sending ping request at {}.",
chrono::Local::now().format("%A, %d %B, %Y - %H:%M:%S")
));
}

sent_ping = true
} else if chrono::Local::now().second() == 46 {
sent_ping = false
} else if chrono::Local::now().second() == 0 && sent_ping == false {
println!("Sending ping at {} second.", chrono::Local::now().second());

if socket_stream.can_write() {
socket_stream
.write_message(Message::Text(
r#"{ "req_id": "Rust_Subscribe", "op": "ping" }"#.to_string(),
))
.expect(&*format!(
"Error in sending ping request at {}.",
chrono::Local::now().format("%A, %d %B, %Y - %H:%M:%S")
));
}

sent_ping = true
} else if chrono::Local::now().second() == 1 {
sent_ping = false
}
}