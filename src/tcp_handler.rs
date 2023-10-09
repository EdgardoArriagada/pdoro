use crate::{request::Request, response::Response, server::Handler, status_code::StatusCode};

pub struct TCPHandler;

impl Handler for TCPHandler {
    fn handle_request(&self, request: &Request) -> Response {
        match request.path().trim() {
            "healthcheck" => Response::new(StatusCode::Ok, Some("I'm alive".to_string())),
            _ => Response::new(StatusCode::NotFound, Some("Path not found".to_string())),
        }
    }
}
