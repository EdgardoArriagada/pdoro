use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    arg: Option<&'buf str>,
}

impl<'buf> Display for Request<'buf> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.display())
    }
}

impl<'buf> Request<'buf> {
    pub fn display(&self) -> String {
        match self.arg {
            Some(arg) => format!("path: {}, arg: {}", self.path, arg),
            None => format!("path: {}", self.path),
        }
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = String;

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buf).map_err(|_| "Invalid request")?;

        let (path, arg) = get_next_word(&request).ok_or("Invalid request")?;

        match arg {
            Some(arg) => Ok(Self {
                path,
                arg: Some(arg),
            }),
            None => Ok(Self { path, arg: None }),
        }
    }
}

fn get_next_word(request: &str) -> Option<(&str, Option<&str>)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], Some(&request[i + 1..])));
        }
    }

    Some((&request[..], None))
}
