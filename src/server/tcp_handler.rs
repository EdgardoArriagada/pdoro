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
        match request.path().trim() {
            "healthcheck" => Response::new(StatusCode::Ok, Some("I'm alive".to_string())),
            "start" => start_pomodoro(request),
            "remaining" => remaining_pomodoro(),
            _ => Response::new(StatusCode::NotFound, Some("Path not found".to_string())),
        }
    }
}

static REMAINING_TIME: RwLock<i32> = RwLock::new(0);

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

    {
        let mut rt = REMAINING_TIME.write().unwrap();
        *rt = seconds;
    }

    thread::spawn(move || {
        for i in (0..seconds).rev() {
            sleep(1);
            {
                let mut rt = REMAINING_TIME.write().unwrap();
                *rt = i;
            }
        }
    });

    return Response::new(StatusCode::Ok, Some("Pomodoro started".to_string()));
}

fn remaining_pomodoro() -> Response {
    loop {
        match REMAINING_TIME.try_read() {
            Ok(rt) => {
                let rt = *rt;

                return Response::new(StatusCode::Ok, Some(rt.to_string()));
            }
            Err(_) => sleep(1),
        };
    }
}
