use std::io::{self, Read};
use std::net::{TcpListener, TcpStream};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on address: {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        listener.set_nonblocking(true).unwrap(); // Set the listener to non-blocking mode

        loop {
            match listener.accept() {
                Ok((stream, _addr)) => {
                    self.handle_connection(stream);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // No connections are incoming
                    continue;
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                }
            }
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        stream.set_nonblocking(true).unwrap(); // Set the stream to non-blocking mode
        match self.read_data(&mut stream) {
            Ok(data) if !data.is_empty() => {
                println!("Received a request: {}", String::from_utf8_lossy(&data));
            }
            Ok(_) => {
                println!("Received an empty request or no more data.");
            }
            Err(e) => {
                println!("Error during connection handling: {}", e);
            }
        }
    }

    fn read_data(&self, stream: &mut TcpStream) -> Result<Vec<u8>, io::Error> {
        let mut buffer = Vec::new();
        let mut temp_buf = [0; 1024];
        let mut attempts = 0;

        loop {
            match stream.read(&mut temp_buf) {
                Ok(0) => {
                    if attempts > 5 {
                        // Arbitrary number of retries
                        break; // Assume the client has closed the connection or no more data
                    }
                    attempts += 1;
                    std::thread::sleep(std::time::Duration::from_millis(10)); // Give some time for data to arrive
                    continue;
                }
                Ok(n) => {
                    buffer.extend_from_slice(&temp_buf[..n]);
                    if n < 1024 {
                        break; // No more data available immediately
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    if attempts > 5 {
                        // Retry a few times if non-blocking read would block
                        break;
                    }
                    attempts += 1;
                    std::thread::sleep(std::time::Duration::from_millis(10)); // Avoid tight loop
                    continue;
                }
                Err(e) => return Err(e), // Some other error occurred
            }
        }
        Ok(buffer)
    }
}
