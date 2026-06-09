mod config;
mod domain;
mod http;
mod server;
mod tests;

use crate::server::serve;

#[tokio::main]
async fn main() {
    serve().await
}
