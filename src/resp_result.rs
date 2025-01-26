use core::fmt;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum RESPError {
    FromUtf8,
    OutOfBounds(usize),
}

impl fmt::Display for RESPError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RESPError::FromUtf8 => write!(f, "Cannot convert from UTF-8"),
            RESPError::OutOfBounds(index) => write!(f, "Out of bounds at index {}", index),
        }
    }
}

impl From<FromUtf8Error> for RESPError {
    fn from(_value: FromUtf8Error) -> Self {
        Self::FromUtf8
    }
}

pub type RESPResult<T> = Result<T, RESPError>;
