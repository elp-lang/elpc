use core::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ParsingError {
    InvalidInt(ParseIntError),
    UnknownChar(char),
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsingError::UnknownChar(ch) => write!(f, "unknown char, can't parse '{}'", ch),
            ParsingError::InvalidInt(err) => write!(f, "failed to parse int becase {}", err),
        }
    }
}

impl std::error::Error for ParsingError {}
