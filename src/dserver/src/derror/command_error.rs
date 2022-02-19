use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Unknown command")]
    Unknown,
}