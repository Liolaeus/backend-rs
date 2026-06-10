mod config;
mod controllers;
mod domain;
mod models;
mod schema;
mod server;
mod tests;

use crate::server::serve;

#[tokio::main]
async fn main() {
    serve().await
}
