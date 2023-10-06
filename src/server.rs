use crate::request;
use request::Request;
use std::io::{Read, Write};
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: &str) -> Self {
        Self {
            addr: addr.to_owned(),
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).expect("Failed to bind address");

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    println!("Here is the request: {}", request);

                                    write!(stream, "HTTP/1.1 200 OK\r\n\r\n");
                                }
                                Err(e) => println!("Failed to parse a request: {}", e),
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
