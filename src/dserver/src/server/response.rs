use std::net::TcpStream;

pub struct Response {
    code: Code,
    body: String,
}

impl Response {
    pub fn new(code: Code, body: String) -> Self {
        Response { code, body }
    }

    pub fn write(&self, stream: &mut TcpStream) {
        write!(stream, "{}|{}", self.code.to_string(), body);
    }
}

pub enum Code{
    Success=200,
    Error=400
}
