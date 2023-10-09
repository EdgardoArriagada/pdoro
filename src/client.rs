use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

#[derive(Debug)]
pub enum ClientError {
    ServerNotStarted,
    ReadError,
    WriteError,
    DecodeError,
}

#[derive(Clone)]
pub struct Client {
    addr: String,
}

impl Client {
    pub fn new(addr: &str) -> Self {
        Self {
            addr: addr.to_owned(),
        }
    }

    pub fn run(&self, request_line: &str) -> Result<String, ClientError> {
        match TcpStream::connect(&self.addr) {
            Ok(mut stream) => {
                if let Err(_) = stream.write(request_line.as_bytes()) {
                    return Err(ClientError::WriteError);
                }

                let mut data = Vec::new();
                match stream.read_to_end(&mut data) {
                    Ok(_) => match from_utf8(&data) {
                        Ok(v) => Ok(v.to_string()),
                        Err(_) => return Err(ClientError::DecodeError),
                    },
                    Err(e) => {
                        println!("le e: {:?}", e);
                        Err(ClientError::ReadError)
                    }
                }
            }
            Err(_) => Err(ClientError::ServerNotStarted),
        }
    }
}
