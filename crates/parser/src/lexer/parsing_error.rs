use core::fmt;
use std::num::{ParseFloatError, ParseIntError};

use crate::tokens::Source;

#[derive(Debug)]
pub enum ParsingError {
    InvalidInt(ParseIntError, Source),
    InvalidFloat(ParseFloatError, Source),
    UnknownChar(char, Source),
    Unknown(&'static str, Source),
    SyntaxError(String, Source),
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsingError::InvalidInt(err, source) => {
                write!(
                    f,
                    "{}\n{}\n {:?}:{}-{}",
                    err, source.path, source.span.lines, source.span.start, source.span.end
                )
            }
            ParsingError::InvalidFloat(err, source) => {
                write!(
                    f,
                    "{}\n{}\n {:?}:{}-{}",
                    err, source.path, source.span.lines, source.span.start, source.span.end
                )
            }
            ParsingError::UnknownChar(ch, source) => {
                write!(
                    f,
                    "unknown char, can't parse '{}'\n{}\n {:?}:{}-{}",
                    ch, source.path, source.span.lines, source.span.start, source.span.end
                )
            }
            ParsingError::Unknown(str, source) => {
                write!(
                    f,
                    "Something unexpected happened '{}'\n{}\n {:?}:{}-{}",
                    str, source.path, source.span.lines, source.span.start, source.span.end
                )
            }
            ParsingError::SyntaxError(str, source) => {
                write!(
                    f,
                    "syntax error: {}\n{}\n {:?}:{}-{}",
                    str, source.path, source.span.lines, source.span.start, source.span.end
                )
            }
        }
    }
}

impl std::error::Error for ParsingError {}
