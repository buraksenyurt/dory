use std::str::Utf8Error;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum MessageParseError {
    #[error("Key name is too long")]
    KeyNameTooLong,
    #[error("Message is empty")]
    Empty,
    #[error("Encoding problem")]
    Encoding,
    #[error("Invalid command")]
    Command,
}

impl From<Utf8Error> for MessageParseError {
    fn from(_: Utf8Error) -> Self {
        Self::Encoding
    }
}
