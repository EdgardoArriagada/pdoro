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

                match clock.as_str() {
                    "00" => stdout("No pomodoro timer is running."),
                    _ => stdout(&clock),
                }
            }
            Err(ClientError::ServerNotStarted) => stderr("No pomodoro timer is running."),
            Err(e) => stderr(format!("Error: {:?}", e).as_str()),
        };
    }

    if args.start {
        match client.run("healthcheck;") {
            Ok(_) => return stderr("Pomodoro server already running."),
            Err(ClientError::ServerNotStarted) => {
                println!("starting...");
                start_daemon_server()
            }
            Err(e) => return stderr(format!("Error: {:?}", e).as_str()),
        }
    }

    if let Some(time) = args.time {
        let start_request = get_start_request(&time);
        match client.run(start_request.as_str()) {
            Ok(_) => return stdout("Pomodoro timer started."),
            Err(ClientError::ServerNotStarted) => stderr("Server not started yet."),
            Err(e) => return stderr(format!("Error: {:?}", e).as_str()),
        }
    }

    stderr("No arguments provided.");
}

fn get_start_request(time_arg: &str) -> String {
    let seconds = get_seconds_from_fromat(get_time_format(time_arg));

    if seconds == 0 {
        stderr("Invalid time format.");
    }

    format!("start {};", seconds)
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
