mod app_io;
mod args;
mod client;
mod server;

use args::Args;
use clap::Parser;

use app_io::{stderr, stdout};
use client::Client;
use server::tcp_handler::TCPHandler;
use server::Server;

static IP: &'static str = "127.0.0.1:3030";

fn main() {
    let args = Args::parse();
    println!("le args: {:?}", args);

    if args.remaining {
        let client = Client::new(IP);

        return match client.run("remaining;") {
            Ok(v) => stdout(&v),
            Err(e) => stderr(format!("Error: {:?}", e).as_str()),
        };
    }

    let server = Server::new(IP);
    server.run(TCPHandler);
}
