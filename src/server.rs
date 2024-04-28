use crate::http::{Request, StatusCode};
use std::sync::Arc;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub struct Server {
    pub addr: String,
}

pub trait Handler {
    fn handle_request(&self, request: &Request, status_code: StatusCode) -> String;
}

impl Server {
    pub fn new(addr: &str) -> Arc<Self> {
        Arc::new(Self {
            addr: addr.to_string(),
        })
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
                // Assuming a way to determine the correct status code from the request
                let status_code = StatusCode::Ok; // This would be dynamic based on actual request analysis
                println!("Received request: {:?}", request);
                self.handle_request(&request, status_code)
            }
            Err(e) => {
                println!("Failed to parse request: {:?}", e);
                self.handle_request(&Request::default(), StatusCode::BadRequest)
                // Assuming Request has a default impl
            }
        };

        socket.write_all(response.as_bytes()).await?;
        socket.flush().await?;
        Ok(())
    }
}

// Implement the Handler trait on the Server or a specific handler struct
impl Handler for Server {
    fn handle_request(&self, request: &Request, status_code: StatusCode) -> String {
        match status_code {
            StatusCode::Ok => format!("HTTP/1.1 {} {}\r\nContent-Type: text/plain\r\n\r\nHello, world!", StatusCode::Ok as u16, StatusCode::Ok.reason_phrase()),
            StatusCode::Created => format!("HTTP/1.1 {} {}\r\nContent-Type: text/plain\r\n\r\nResource created successfully!", StatusCode::Created as u16, StatusCode::Created.reason_phrase()),
            StatusCode::NoContent => format!("HTTP/1.1 {} {}", StatusCode::NoContent as u16, StatusCode::NoContent.reason_phrase()),
            StatusCode::BadRequest => format!("HTTP/1.1 {} {}\r\nContent-Type: text/plain\r\n\r\nBad request: Error in request format.", StatusCode::BadRequest as u16, StatusCode::BadRequest.reason_phrase()),
            StatusCode::Unauthorized => format!("HTTP/1.1 {} {}\r\nContent-Type: text/plain\r\n\r\nUnauthorized: Authentication required.", StatusCode::Unauthorized as u16, StatusCode::Unauthorized.reason_phrase()),
            StatusCode::Forbidden => format!("HTTP/1.1 {} {}\r\nContent-Type: text/plain\r\n\r\nForbidden: You do not have access to this resource.", StatusCode::Forbidden as u16, StatusCode::Forbidden.reason_phrase()),
            StatusCode::NotFound => format!("HTTP/1.1 {} {}\r\nContent-Type: text/plain\r\n\r\nNot Found: The requested resource was not found on this server.", StatusCode::NotFound as u16, StatusCode::NotFound.reason_phrase()),
            StatusCode::InternalServerError => format!("HTTP/1.1 {} {}\r\nContent-Type: text/plain\r\n\r\nInternal Server Error: An unexpected error occurred.", StatusCode::InternalServerError as u16, StatusCode::InternalServerError.reason_phrase()),
            _ => format!("HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/plain\r\n\r\nAn unexpected server error occurred."),
        }
    }
}
