#![allow(dead_code)]

use server::Server;

mod http;
mod server;

#[tokio::main]
async fn main() {
    let server = Server::new("127.0.0.1:8080");
    server.run().await;
}
