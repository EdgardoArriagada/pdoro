use super::request::Request;
use super::response::Response;
use super::status_code::StatusCode;
use super::Handler;

pub struct TCPHandler;

impl Handler for TCPHandler {
    fn handle_request(&self, request: &Request) -> Response {
        match request.path().trim() {
            "healthcheck" => Response::new(StatusCode::Ok, Some("I'm alive".to_string())),
            "remaining" => Response::new(StatusCode::Ok, Some("10 minutes".to_string())),
            _ => Response::new(StatusCode::NotFound, Some("Path not found".to_string())),
        }
    }
}
