use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum MessageSendError {
    #[error("Message couldn't add to pack")]
    Add,
    #[error("Message couldn't get from pack")]
    Get,
    #[error("Message couldn't delete from pack")]
    Del,
}