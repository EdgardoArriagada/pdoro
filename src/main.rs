mod args;
mod server;

use args::Args;
use clap::Parser;

// use server::tcp_handler::TCPHandler;
// use server::Server;

fn main() {
    let args = Args::parse();
    println!("le args: {:?}", args);

    // let server = Server::new("127.0.0.1:3030");
    // server.run(TCPHandler);
}
