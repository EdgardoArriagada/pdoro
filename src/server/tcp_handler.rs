use crate::utils::sleep;

use super::request::Request;
use super::response::Response;
use super::status_code::StatusCode;
use super::Handler;

use std::process::Command;
use std::sync::RwLock;
use std::thread;

pub struct TCPHandler;

impl Handler for TCPHandler {
    fn handle_request(&self, request: &Request) -> Response {
        match request.path() {
            "healthcheck" => Response::new(StatusCode::Ok, Some("I'm alive".to_string())),
            "start" => start_pomodoro(request),
            "halt-counter" => halt_counter(),
            "remaining" => remaining_pomodoro(),
            "is-counter-running" => is_counter_running(),
            "pause-resume-counter" => pause_resume_counter(),
            _ => Response::new(StatusCode::NotFound, Some("Path not found".to_string())),
        }
    }
}

enum CounterState {
    Pristine,
    Running,
    Halting,
    Paused,
}

static REMAINING_TIME: RwLock<u32> = RwLock::new(0);
static COUNTER_STATE: RwLock<CounterState> = RwLock::new(CounterState::Pristine);

fn start_pomodoro(request: &Request) -> Response {
    match COUNTER_STATE.try_read() {
        Ok(cs) => match *cs {
            CounterState::Pristine => {}
            _ => {
                return Response::new(
                    StatusCode::Conflict,
                    Some("Pomodoro already running.".to_string()),
                )
            }
        },
        Err(_) => {
            return Response::new(
                StatusCode::InternalServerError,
                Some("Failed to read counter state.".to_string()),
            )
        }
    }

    let (arg1, arg2) = match (request.arg1(), request.arg2()) {
        (Some(a), Some(b)) => (a, b),
        _ => return Response::new(StatusCode::BadRequest, Some("Missing args.".to_string())),
    };

    let seconds = match arg1.parse::<u32>() {
        Ok(s) => s,
        Err(_) => {
            return Response::new(
                StatusCode::BadRequest,
                Some("Invalid time format.".to_string()),
            )
        }
    };

    let callback_with_args = arg2.to_string();

    {
        let mut rt = REMAINING_TIME.write().unwrap();
        *rt = seconds;

        let mut cs = COUNTER_STATE.write().unwrap();
        *cs = CounterState::Running;
    }

    thread::spawn(move || {
        // prevent index from going below 0
        let mut i = match seconds {
            1 => 1,
            _ => seconds - 1,
        };

        let mut stored_state = CounterState::Running;

        loop {
            sleep(1);
            i -= 1;

            // lock 1
            {
                let mut rt = REMAINING_TIME.write().unwrap();
                let cs = COUNTER_STATE.read().unwrap(); // read for better performance

                match *cs {
                    CounterState::Paused => {
                        i += 1;
                        continue;
                    }
                    CounterState::Halting => {
                        *rt = 0;
                    }
                    _ => {
                        *rt = i;
                    }
                }

                if *rt <= 0 {
                    stored_state = CounterState::Pristine;
                }
            } // unlock lock 1

            match stored_state {
                // match all states asigned to `stored_state` during lock 1
                CounterState::Pristine => {
                    {
                        let mut cs = COUNTER_STATE.write().unwrap();
                        *cs = CounterState::Pristine;
                    }
                    break;
                }
                _ => {}
            }
        }

        run_callback(&callback_with_args);
    });

    return Response::new(StatusCode::Created, Some("Pomodoro started.".to_string()));
}

fn run_callback(callback_with_args: &str) {
    let (callback, args) = parse_callback_with_args(callback_with_args);

    Command::new(callback)
        .args(args)
        .spawn()
        .expect("Failed to run callback.");
}

fn parse_callback_with_args(callback_with_args: &str) -> (String, Vec<String>) {
    let mut split = callback_with_args.split(" ");
    let callback = split.next().unwrap().to_string();
    let args = split.map(|s| s.to_string()).collect();

    return (callback, args);
}

fn remaining_pomodoro() -> Response {
    let remaining = REMAINING_TIME.read().unwrap();
    let state = COUNTER_STATE.read().unwrap();

    let status_code = match *state {
        CounterState::Paused => StatusCode::NotModified,
        _ => StatusCode::Ok,
    };

    return Response::new(status_code, Some(remaining.to_string()));
}

fn halt_counter() -> Response {
    let mut cs = COUNTER_STATE.write().unwrap();
    match *cs {
        CounterState::Halting => {
            return Response::new(
                StatusCode::Conflict,
                Some("Pomodoro counter already halting...".to_string()),
            )
        }
        CounterState::Pristine => {
            return Response::new(StatusCode::Conflict, Some("Nothing to halt.".to_string()))
        }
        _ => {
            *cs = CounterState::Halting;
            return Response::new(
                StatusCode::Ok,
                Some("Pomodoro counter halting...".to_string()),
            );
        }
    }
}

fn pause_resume_counter() -> Response {
    let mut cs = COUNTER_STATE.write().unwrap();
    match *cs {
        CounterState::Running => {
            *cs = CounterState::Paused;
            return Response::new(StatusCode::Ok, Some("Pomodoro counter paused.".to_string()));
        }
        CounterState::Paused => {
            *cs = CounterState::Running;
            return Response::new(
                StatusCode::Ok,
                Some("Pomodoro counter resumed.".to_string()),
            );
        }
        _ => {
            return Response::new(
                StatusCode::Conflict,
                Some("nothing to pause/resume.".to_string()),
            )
        }
    }
}

fn is_counter_running() -> Response {
    let state = COUNTER_STATE.read().unwrap();

    match *state {
        CounterState::Pristine => Response::new(StatusCode::Continue, Some("false".to_string())),
        _ => Response::new(StatusCode::Processing, Some("true".to_string())),
    }
}
