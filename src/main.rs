mod http;
mod srv;

use http::Method;
use http::Request;
use srv::Server;

fn main() {
    let server = Server::new("localhost:8080".to_string());
    server.run();
}
