mod args;
mod client;
mod server;

use args::Args;
use clap::Parser;

use client::Client;
use server::tcp_handler::TCPHandler;
use server::Server;

static IP: &'static str = "127.0.0.1:3030";

fn main() {
    let args = Args::parse();
    println!("le args: {:?}", args);

    if args.remaining {
        let client = Client::new(IP);
        client.run();
        return;
    }

    let server = Server::new(IP);
    server.run(TCPHandler);
}
