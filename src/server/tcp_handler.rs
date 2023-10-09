use super::request::Request;
use super::response::Response;
use super::status_code::StatusCode;
use super::Handler;

use std::sync::RwLock;

use std::thread;
use std::time::Duration;

pub struct TCPHandler;

impl Handler for TCPHandler {
    fn handle_request(&self, request: &Request) -> Response {
        match request.path().trim() {
            "healthcheck" => Response::new(StatusCode::Ok, Some("I'm alive".to_string())),
            "start" => start_pomodoro(request),
            "remaining" => remaining_pomodoro(request),
            _ => Response::new(StatusCode::NotFound, Some("Path not found".to_string())),
        }
    }
}

fn sleep(secs: u64) {
    thread::sleep(Duration::from_secs(secs))
}

static REMAINING_TIME: RwLock<i32> = RwLock::new(0);

fn start_pomodoro(request: &Request) -> Response {
    {
        let mut rt = REMAINING_TIME.write().unwrap();
        *rt = 10;
    }

 thread::spawn(|| {
        for i in (0..10).rev() {
            sleep(1);
            {
                let mut rt = REMAINING_TIME.write().unwrap();
                *rt = i;
            }
        }
    });

    return Response::new(StatusCode::Ok, Some("Pomodoro started".to_string()));
}

fn remaining_pomodoro(request: &Request) -> Response {
    let remaining = REMAINING_TIME.read().unwrap();
    return Response::new(StatusCode::Ok, Some(remaining.to_string()));
}
