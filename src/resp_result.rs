use core::fmt;

#[derive(Debug)]
pub enum RESPError {
    OutOfBounds(usize),
}

impl fmt::Display for RESPError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RESPError::OutOfBounds(index) => write!(f, "Out of bounds at index {}", index),
        }
    }
}

pub type RESPResult<T> = Result<T, RESPError>;
