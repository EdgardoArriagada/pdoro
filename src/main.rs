use std::fs::File;

mod app_io;
mod args;
mod client;
mod server;

use args::Args;
use clap::Parser;
use daemonize::Daemonize;

use app_io::{stderr, stdout};
use client::Client;
use server::tcp_handler::TCPHandler;
use server::Server;

use crate::client::ClientError;

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

    let client = Client::new(IP);

    match client.run("healthcheck;") {
        Ok(v) => stdout(&v),
        Err(ClientError::ServerNotStarted) => {
            println!("Server not started, starting...");
            start_daemon_server();
        }
        Err(e) => stderr(format!("Error: {:?}", e).as_str()),
    };
}

fn start_daemon_server() {
    let stdout_file = File::create("/tmp/pdoro.out").unwrap();
    let stderr_file = File::create("/tmp/pdoro.err").unwrap();

    let daemonize = Daemonize::new()
        .working_directory("/tmp")
        .user("nobody")
        .group("pdoro_daemon")
        .umask(0o777)
        .stdout(stdout_file)
        .stderr(stderr_file)
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => println!("Success, daemonized"),
        Err(e) => eprintln!("Error, {}", e),
    }

    let server = Server::new(IP);
    server.run(TCPHandler);
}
