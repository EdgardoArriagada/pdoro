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
use utils::{get_seconds_from_fromat, get_time_format, stderr, stdout, Time};

use crate::{
    client::ClientError,
    utils::{get_clock_from_seconds, TimeFormat},
};

static IP: &'static str = "127.0.0.1:3030";

fn main() {
    let args = Args::parse();

    if let Some(time) = args.is_valid_time {
        match get_time_format(&time) {
            Time {
                format: TimeFormat::Invalid,
                ..
            } => return stdout("false"),
            _ => return stdout("true"),
        }
    }

    if args.remaining {
        return match Client::new(IP).run("remaining;") {
            Ok(res) => {
                let digits = res.msg();

                if digits.is_empty() {
                    return stdout("No pomodoro timer is running.");
                }

                let seconds = digits.parse::<u32>().unwrap();
                let clock = get_clock_from_seconds(&seconds);

                match (clock.as_str(), res.status()) {
                    ("00", _) => stdout("No pomodoro timer is running."),
                    (_, 304) => stdout(format!("{} (paused)", &clock).as_str()),
                    _ => stdout(&clock),
                }
            }
            Err(ClientError::ServerNotStarted) => stderr("No pomodoro timer is running."),
            Err(e) => stderr(format!("Error: {:?}", e).as_str()),
        };
    }

    if args.start_server {
        match Client::new(IP).run("healthcheck;") {
            Ok(_) => return stderr("Pomodoro server already running."),
            Err(ClientError::ServerNotStarted) => {
                println!("starting...");
                start_daemon_server()
            }
            Err(e) => return stderr(format!("Error: {:?}", e).as_str()),
        }
    }

    match (args.time, args.callback_with_args) {
        (Some(time), Some(callback_with_args)) => {
            let start_request = get_start_request(&time, &callback_with_args);

            match Client::new(IP).run(start_request.as_str()) {
                Ok(res) => match res.status() {
                    201 => return stdout(res.msg()),
                    _ => return stderr(res.msg()),
                },
                Err(ClientError::ServerNotStarted) => stderr("Server not started yet."),
                Err(e) => return stderr(format!("Error: {:?}", e).as_str()),
            }
        }
        (Some(_), None) | (None, Some(_)) => {
            return stderr("Both time and callback_with_args must be provided.")
        }
        _ => {}
    }

    if args.halt_counter {
        match Client::new(IP).run("halt-counter;") {
            Ok(res) => match res.status() {
                200 => return stdout(res.msg()),
                _ => return stderr(res.msg()),
            },
            Err(ClientError::ServerNotStarted) => stderr("No pomodoro timer is running."),
            Err(e) => return stderr(format!("Error: {:?}", e).as_str()),
        }
    }

    if args.pause_resume_counter {
        match Client::new(IP).run("pause-resume-counter;") {
            Ok(res) => match res.status() {
                200 => return stdout(res.msg()),
                _ => return stderr(res.msg()),
            },
            Err(ClientError::ServerNotStarted) => stderr("No pomodoro timer is running."),
            Err(e) => return stderr(format!("Error: {:?}", e).as_str()),
        }
    }

    if args.is_counter_running {
        match Client::new(IP).run("is-counter-running;") {
            Ok(res) => match res.status() {
                100 | 102 => return stdout(res.msg()),
                _ => return stderr(res.msg()),
            },
            Err(ClientError::ServerNotStarted) => stderr("No pomodoro timer is running."),
            Err(e) => return stderr(format!("Error: {:?}", e).as_str()),
        }
    }

    stderr("No arguments provided.");
}

fn get_start_request(time_arg: &str, callback_with_args: &str) -> String {
    let seconds = get_seconds_from_fromat(get_time_format(time_arg));

    if seconds == 0 {
        stderr("Invalid time format.");
    }

    format!("start {} {};", seconds, callback_with_args)
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

    Server::new(IP).run(TCPHandler);
}
