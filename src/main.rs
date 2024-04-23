mod server;

use server::Server;

#[tokio::main]
async fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run().await;
}
