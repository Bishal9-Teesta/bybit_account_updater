mod config;
pub mod global;
mod stream;
mod structure;
// mod global;

use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    // stream::private::private();
    stream::private::private();
}
