use core::fmt;
use std::{num, string::FromUtf8Error};

#[derive(Debug, PartialEq)]
pub enum RESPError {
    FromUtf8,
    IncorrectLength(RESPLength),
    OutOfBounds(usize),
    ParseInt,
    Unknown,
    WrongType,
}

impl fmt::Display for RESPError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RESPError::FromUtf8 => write!(f, "Cannot convert from UTF-8"),
            RESPError::IncorrectLength(length) => write!(f, "Incorrect length {}", length),
            RESPError::OutOfBounds(index) => write!(f, "Out of bounds at index {}", index),
            RESPError::ParseInt => write!(f, "Cannot parse string into integer"),
            RESPError::Unknown => write!(f, "Unknown format for RESP String"),
            RESPError::WrongType => write!(f, "Wrong prefix for REST type"),
        }
    }
}

impl From<num::ParseIntError> for RESPError {
    fn from(_err: num::ParseIntError) -> Self {
        Self::ParseInt
    }
}

impl From<FromUtf8Error> for RESPError {
    fn from(_value: FromUtf8Error) -> Self {
        Self::FromUtf8
    }
}

pub type RESPResult<T> = Result<T, RESPError>;
pub type RESPLength = i32;

#[derive(Debug, PartialEq)]
pub enum RESP {
    BulkString(String),
    Null,
    SimpleString(String),
}

impl fmt::Display for RESP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = match self {
            Self::BulkString(data) => format!("${}\r\n{}\r\n", data.len(), data),
            Self::Null => "$-1\r\n".to_string(),
            Self::SimpleString(data) => format!("+{}\r\n", data),
        };

        write!(f, "{}", data)
    }
}
