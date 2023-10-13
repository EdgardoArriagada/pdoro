pub struct Response {
    status: u16,
    msg: String,
}

impl Response {
    pub fn new(raw_res: &str) -> Response {
        let mut parts = raw_res.splitn(2, ' ');

        let status = parts.next().unwrap_or("");
        let msg = parts.next().unwrap_or("");

        let status = status.parse::<u16>().unwrap_or(500);
        let msg = match msg.rfind(';') {
            Some(i) => &msg[..i],
            None => msg,
        };

        Response {
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
