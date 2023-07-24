mod config;
pub mod global;
mod stream;
mod structure;

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    // stream::private::private();
    stream::private::private();
}
