use http::Method;
use http::Request;
use server::Server;

mod http;
mod server;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());

    // dbg!(string_slice);

    // NOTES
    // EVERY 2 NETCAT ECHOING ENDS UP IN PRINTING THE THE VALUE - INVESTIGATE WHY
    server.run();
}
