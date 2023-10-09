mod request;
mod response;
mod server;
mod status_code;
mod tcp_handler;
use server::Server;
use tcp_handler::TCPHandler;

fn main() {
    let server = Server::new("127.0.0.1:3030");
    server.run(TCPHandler);
}
