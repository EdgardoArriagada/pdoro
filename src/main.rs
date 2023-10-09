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
use utils::{get_seconds_from_fromat, get_time_format, stderr, stdout};

use crate::{client::ClientError, utils::get_clock_from_seconds};

static IP: &'static str = "127.0.0.1:3030";

fn main() {
    let args = Args::parse();
    let client = Client::new(IP);

    if args.remaining {
        return match client.run("remaining;") {
            Ok(v) => {
                let digits = &v[4..];
                let digits = match digits.rfind(";") {
                    Some(i) => &digits[..i],
                    None => digits,
                };

                let seconds = digits.parse::<u32>().unwrap();
                let clock = get_clock_from_seconds(&seconds);
                stdout(&clock)
            }
            Err(ClientError::ServerNotStarted) => stderr("No pomodoro timer is running."),
            Err(e) => stderr(format!("Error: {:?}", e).as_str()),
        };
    }

    try_run_cmd(&client, &args);
}

fn get_start_request(time_arg: &str) -> String {
    let seconds = get_seconds_from_fromat(get_time_format(time_arg));

    if seconds == 0 {
        stderr("Invalid time format.");
    }

    format!("start {};", seconds)
}

fn try_run_cmd(client: &Client, args: &Args) {
    match client.run("healthcheck;") {
        Ok(_) => match args.time.to_owned() {
            Some(time_arg) => {
                let start_request = get_start_request(&time_arg);
                match client.run(start_request.as_str()) {
                    Ok(_) => stdout("Pomodoro timer started."),
                    Err(e) => stderr(format!("Error: {:?}", e).as_str()),
                }
            }
            _ => stderr("No time specified."),
        },
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
