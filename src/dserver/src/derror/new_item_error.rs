use thiserror::Error;

#[derive(Debug, Error)]
pub enum NewItemError {
    #[error("Key name is too long.")]
    InvalidKeyLen,
    #[error("Value is too long.")]
    InvalidValueLen,
}