use core::fmt;
use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug)]
pub enum ParsingError {
    InvalidInt(ParseIntError),
    InvalidFloat(ParseFloatError),
    UnknownChar(char),
    Unknown(&'static str),
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsingError::UnknownChar(ch) => write!(f, "unknown char, can't parse '{}'", ch),
            ParsingError::InvalidInt(err) => write!(f, "failed to parse int because {}", err),
            ParsingError::InvalidFloat(err) => write!(f, "failed to parse float because {}", err),
            ParsingError::Unknown(str) => write!(f, "Something unexpected happened '{}'", str),
        }
    }
}

impl std::error::Error for ParsingError {}
