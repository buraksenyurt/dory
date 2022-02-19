use thiserror::Error;

#[derive(Debug, Error)]
pub enum MessageParseError {
    #[error("Invalid item count")]
    InvalidItemCount,
    Empty
}