mod server;

use server::Server;
use server::tcp_handler::TCPHandler;

fn main() {
    let server = Server::new("127.0.0.1:3030");
    server.run(TCPHandler);
}
