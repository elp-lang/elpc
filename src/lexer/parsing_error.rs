use core::fmt;
use std::num::{ParseFloatError, ParseIntError};
use std::sync::Arc;

use crate::lexer::lexer::SourceFile;

#[derive(Debug)]
pub enum ParsingError {
    InvalidInt(ParseIntError, Arc<SourceFile>),
    InvalidFloat(ParseFloatError, Arc<SourceFile>),
    UnknownChar(char, Arc<SourceFile>),
    Unknown(&'static str, Arc<SourceFile>),
    SyntaxError(String, Arc<SourceFile>),
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsingError::InvalidInt(err, source) => {
                write!(
                    f,
                    "{}\n{}\n {}:{}-{}",
                    err, source.path, source.line, source.span.0, source.span.1
                )
            }
            ParsingError::InvalidFloat(err, source) => {
                write!(
                    f,
                    "{}\n{}\n {}:{}-{}",
                    err, source.path, source.line, source.span.0, source.span.1
                )
            }
            ParsingError::UnknownChar(ch, source) => {
                write!(
                    f,
                    "unknown char, can't parse '{}'\n{}\n {}:{}-{}",
                    ch, source.path, source.line, source.span.0, source.span.1
                )
            }
            ParsingError::Unknown(str, source) => {
                write!(
                    f,
                    "Something unexpected happened '{}'\n{}\n {}:{}-{}",
                    str, source.path, source.line, source.span.0, source.span.1
                )
            }
            ParsingError::SyntaxError(str, source) => {
                write!(
                    f,
                    "syntax error: {}\n{}\n {}:{}-{}",
                    str, source.path, source.line, source.span.0, source.span.1
                )
            }
        }
    }
}

impl std::error::Error for ParsingError {}
