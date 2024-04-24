use crate::http::Request;
use std::sync::Arc;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub struct Server {
    pub addr: String,
}

impl Server {
    pub fn new(addr: String) -> Arc<Self> {
        Arc::new(Self { addr })
    }

    pub async fn run(self: Arc<Self>) {
        let listener = TcpListener::bind(&self.addr).await.unwrap();
        println!("Listening on address: {}", self.addr);

        loop {
            let (socket, _addr) = match listener.accept().await {
                Ok(connection) => connection,
                Err(e) => {
                    eprintln!("Failed to accept connection: {:?}", e);
                    continue;
                }
            };
            let server_clone = self.clone();
            tokio::spawn(async move {
                if let Err(e) = server_clone.handle_connection(socket).await {
                    eprintln!("Error handling connection: {:?}", e);
                }
            });
        }
    }

    async fn handle_connection(self: Arc<Self>, mut socket: TcpStream) -> io::Result<()> {
        let mut buffer = Vec::new();
        loop {
            let mut tmp_buffer = vec![0; 1024];
            let n = socket.read(&mut tmp_buffer).await?;
            if n == 0 {
                break; // Connection closed by the client
            }
            buffer.extend_from_slice(&tmp_buffer[..n]);

            if buffer.ends_with(b"\r\n\r\n") {
                break; // End of the HTTP header section
            }
        }

        let response = match Request::try_from(&buffer[..]) {
            Ok(request) => {
                println!("Received request: {:?}", request);
                self.handle_request(&request)
            }
            Err(e) => {
                println!("Failed to parse request: {:?}", e);
                self.handle_bad_request(&e)
            }
        };

        socket.write_all(response.as_bytes()).await?;
        socket.flush().await?;
        Ok(())
    }

    fn handle_request(&self, request: &Request) -> String {
        // Example response for demonstration
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, world!".to_string()
    }

    fn handle_bad_request(&self, _e: &dyn std::error::Error) -> String {
        // Basic bad request response
        "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\n\r\nBad request".to_string()
    }
}
