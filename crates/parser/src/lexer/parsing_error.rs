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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{lexer::parsing_error::ParsingError, tokens::Source};

    #[test]
    fn test_parsing_error_display() {
        let mock_int_err: Result<i32, _> = "not_a_number".parse();
        assert_eq!(
            "invalid digit found in string\nnone\n []:0-0",
            format!(
                "{}",
                ParsingError::InvalidInt(
                    mock_int_err.err().unwrap(),
                    Source {
                        path: "none".into(),
                        ..Default::default()
                    }
                )
            )
        );

        let mock_float_err: Result<f32, _> = "not_a_number".parse();
        assert_eq!(
            "invalid float literal\nnone\n []:0-0",
            format!(
                "{}",
                ParsingError::InvalidFloat(
                    mock_float_err.err().unwrap(),
                    Source {
                        path: "none".into(),
                        ..Default::default()
                    }
                )
            )
        );

        assert_eq!(
            "Something unexpected happened 'test'\nnone\n []:0-0",
            format!(
                "{}",
                ParsingError::Unknown(
                    "test",
                    Source {
                        path: "none".into(),
                        ..Default::default()
                    }
                )
            )
        );

        assert_eq!(
            "unknown char, can't parse 'a'\nnone\n []:0-0",
            format!(
                "{}",
                ParsingError::UnknownChar(
                    'a',
                    Source {
                        path: "none".into(),
                        ..Default::default()
                    }
                )
            )
        );

        assert_eq!(
            "syntax error: test\nnone\n []:0-0",
            format!(
                "{}",
                ParsingError::SyntaxError(
                    "test".into(),
                    Source {
                        path: "none".into(),
                        ..Default::default()
                    }
                )
            )
        );
    }
}
