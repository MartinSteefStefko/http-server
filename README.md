# http-server

A simple asynchronous HTTP server implemented in Rust using the Tokio runtime.

## Overview

This project is an asynchronous HTTP server using the Tokio framework in Rust. The server listens for incoming TCP connections, reads data from clients, and handles each connection asynchronously.

## Features

- Asynchronous handling of TCP connections.
- Dynamic resizing of the receive buffer to accommodate variable data sizes.
- Error handling for connection setup and data reading.
- Multi-threaded execution using Tokio's task spawning mechanism.

## Requirements

- Rust (stable) - The project is built using Rust programming language. You can install Rust by following the instructions [here](https://www.rust-lang.org/tools/install).
- Tokio - Tokio is a runtime for writing reliable asynchronous applications with Rust. It's added as a dependency in the `Cargo.toml` file and will be automatically installed when building the project.

## Usage

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/http-server.git
   ```

2. Navigate to the project directory:

   ```bash
   cd http-server
   ```

3. Build the project:

   ```bash
   cargo build
   ```

4. Run the server:

   ```bash
   cargo run -- <ADDRESS>
   ```

   Replace `<ADDRESS>` with the IP address and port number where you want the server to listen for incoming connections (e.g., `127.0.0.1:8080`).

## Example

```rust
use async_tcp_server::Server;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Create a new instance of the server
    let server = Server::new("127.0.0.1:8080".to_string());

    // Start the server asynchronously
    server.run().await;
}
```
