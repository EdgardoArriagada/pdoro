use crate::utils::sleep;

use super::request::Request;
use super::response::Response;
use super::status_code::StatusCode;
use super::Handler;

use std::sync::RwLock;
use std::thread;

pub struct TCPHandler;

impl Handler for TCPHandler {
    fn handle_request(&self, request: &Request) -> Response {
        match request.path() {
            "healthcheck" => Response::new(StatusCode::Ok, Some("I'm alive".to_string())),
            "start" => start_pomodoro(request),
            "halt-counter" => halt_counter(),
            "pause-counter" => pause_counter(),
            "remaining" => remaining_pomodoro(),
            "resume-counter" => resume_counter(),
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

static REMAINING_TIME: RwLock<i32> = RwLock::new(0);
static COUNTER_STATE: RwLock<CounterState> = RwLock::new(CounterState::Pristine);

fn start_pomodoro(request: &Request) -> Response {
    let raw_seconds = match request.arg1() {
        Some(s) => s,
        None => {
            return Response::new(
                StatusCode::BadRequest,
                Some("No time specified".to_string()),
            )
        }
    };

    let seconds = match raw_seconds.parse::<i32>() {
        Ok(s) => s,
        Err(_) => {
            return Response::new(
                StatusCode::BadRequest,
                Some("Invalid time format".to_string()),
            )
        }
    };

    match COUNTER_STATE.try_read() {
        Ok(cs) => match *cs {
            CounterState::Running => {
                return Response::new(
                    StatusCode::Conflict,
                    Some("Pomodoro already running".to_string()),
                )
            }
            _ => {}
        },
        Err(_) => {
            return Response::new(
                StatusCode::InternalServerError,
                Some("Failed to read counter state".to_string()),
            )
        }
    }

    {
        let mut rt = REMAINING_TIME.write().unwrap();
        *rt = seconds;

        let mut cs = COUNTER_STATE.write().unwrap();
        *cs = CounterState::Running;
    }

    thread::spawn(move || {
        let mut i = seconds;

        loop {
            sleep(1);
            i -= 1;

            {
                let mut rt = REMAINING_TIME.write().unwrap();
                let mut cs = COUNTER_STATE.write().unwrap();

                match *cs {
                    CounterState::Halting => {
                        *rt = 0;
                        *cs = CounterState::Pristine;
                        return;
                    }
                    CounterState::Paused => {
                        i += 1;
                        continue;
                    }
                    _ => {
                        *rt = i;
                    }
                }
            }

            if i <= 0 {
                break;
            }
        }
    });

    return Response::new(StatusCode::Created, Some("Pomodoro started".to_string()));
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
    {
        let mut cs = COUNTER_STATE.write().unwrap();
        *cs = CounterState::Halting;
    }

    return Response::new(StatusCode::Ok, Some("Pomodoro counter halting".to_string()));
}

fn pause_counter() -> Response {
    {
        let mut cs = COUNTER_STATE.write().unwrap();
        match *cs {
            CounterState::Running => {
                *cs = CounterState::Paused;
                return Response::new(StatusCode::Ok, Some("Pomodoro counter paused".to_string()));
            }
            _ => return Response::new(StatusCode::Conflict, Some("nothing to pause.".to_string())),
        }
    }
}

fn resume_counter() -> Response {
    {
        let mut cs = COUNTER_STATE.write().unwrap();
        match *cs {
            CounterState::Paused => {
                *cs = CounterState::Running;
                return Response::new(StatusCode::Ok, Some("Pomodoro counter resumed".to_string()));
            }
            _ => {
                return Response::new(StatusCode::Conflict, Some("nothing to resume.".to_string()))
            }
        }
    }
}

fn pause_resume_counter() -> Response {
    {
        let mut cs = COUNTER_STATE.write().unwrap();
        match *cs {
            CounterState::Running => {
                *cs = CounterState::Paused;
                return Response::new(StatusCode::Ok, Some("Pomodoro counter paused".to_string()));
            }
            CounterState::Paused => {
                *cs = CounterState::Running;
                return Response::new(StatusCode::Ok, Some("Pomodoro counter resumed".to_string()));
            }
            _ => {
                return Response::new(
                    StatusCode::Conflict,
                    Some("nothing to pause/resume.".to_string()),
                )
            }
        }
    }
}
