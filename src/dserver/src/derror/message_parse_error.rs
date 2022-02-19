use std::str::Utf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MessageParseError {
    #[error("Invalid item count")]
    InvalidItemCount,
    #[error("Message is empty")]
    Empty,
    #[error("Encoding problem")]
    InvalidEncoding,
}

impl From<Utf8Error> for MessageParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
