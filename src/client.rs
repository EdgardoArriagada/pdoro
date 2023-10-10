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

pub struct Res {
    status: u16,
    msg: String,
}

impl Res {
    pub fn new(raw_res: &str) -> Res {
        let mut parts = raw_res.splitn(2, ' ');

        let status = parts.next().unwrap_or("");
        let msg = parts.next().unwrap_or("");

        let status = status.parse::<u16>().unwrap_or(500);
        let msg = match msg.rfind(';') {
            Some(i) => &msg[..i],
            None => msg,
        };

        Res {
            status,
            msg: msg.to_string(),
        }
    }

    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn msg(&self) -> &str {
        self.msg.as_str()
    }
}

impl Client {
    pub fn new(addr: &str) -> Self {
        Self {
            addr: addr.to_owned(),
        }
    }

    pub fn run(&self, request_line: &str) -> Result<Res, ClientError> {
        match TcpStream::connect(&self.addr) {
            Ok(mut stream) => {
                if let Err(_) = stream.write(request_line.as_bytes()) {
                    return Err(ClientError::WriteError);
                }

                let mut data = Vec::new();
                match stream.read_to_end(&mut data) {
                    Ok(_) => match from_utf8(&data) {
                        Ok(raw_res) => Ok(Res::new(raw_res)),
                        Err(_) => return Err(ClientError::DecodeError),
                    },
                    Err(_) => Err(ClientError::ReadError),
                }
            }
            Err(_) => Err(ClientError::ServerNotStarted),
        }
    }
}
