use std::fs::File;

mod args;
mod client;
mod server;
mod utils;

use args::Args;
use clap::Parser;
use daemonize::Daemonize;

use client::Client;
use server::tcp_handler::TCPHandler;
use server::Server;
use utils::{stderr, stdout};

use crate::client::ClientError;

static IP: &'static str = "127.0.0.1:3030";

fn main() {
    let args = Args::parse();
    println!("le args: {:?}", args);
    let client = Client::new(IP);

    if args.remaining {
        return match client.run("remaining;") {
            Ok(v) => stdout(&v),
            Err(ClientError::ServerNotStarted) => stderr("No pomodoro timer is running."),
            Err(e) => stderr(format!("Error: {:?}", e).as_str()),
        };
    }

    try_run_cmd(&client, &args);
}

fn try_run_cmd(client: &Client, args: &Args) {
    match client.clone().run("healthcheck;") {
        Ok(_) => {
            match client.clone().run("start;") {
                Ok(v) => stdout(&v),
                Err(e) => stderr(format!("Error: {:?}", e).as_str()),
            };
        }
        Err(ClientError::ServerNotStarted) => {
            println!("Server not started, starting...");
            start_daemon_server()
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

    let new_server = Server::new(IP);
    new_server.run(TCPHandler);
}
