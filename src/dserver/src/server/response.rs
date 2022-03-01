use std::fmt::{Display, Formatter};
use std::io::Write;
use std::net::TcpStream;

pub struct Response {
    code: Code,
}

impl Response {
    pub fn new(code: Code) -> Self {
        Response { code }
    }

    pub fn write(&self, stream: &mut TcpStream) {
        write!(stream, "{}|\r\n", self.code.to_string());
    }
}

#[derive(Copy, Clone)]
pub enum Code {
    Success = 200,
    Error = 400,
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = *self as u16;
        match self {
            Self::Success => write!(f, "{} Success", c),
            Self::Error => write!(f, "{} Error", c),
        }
    }
}
