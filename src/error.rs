use crate::consts::msg;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("{}", msg::IO_ERROR)]
    IoError(String),
}

impl From<std::io::Error> for Error {
    fn from(io_err: std::io::Error) -> Self {
        Self::IoError(io_err.to_string())
    }
}
