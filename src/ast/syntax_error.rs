use std::fmt;

use super::lexer::{Token, TokenType};

#[derive(Debug)]
pub enum SyntaxError<'a> {
    UnexpectedToken(Token),
    UnexpectedTokenButGot(TokenType, Token),
    MissingToken(&'a str),
}

impl fmt::Display for SyntaxError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxError::UnexpectedToken(token) => write!(f, "Unexpected token: {:#?}", token),
            SyntaxError::MissingToken(token) => write!(f, "Missing token: {:#?}", token),
            SyntaxError::UnexpectedTokenButGot(expected, got) => {
                write!(f, "Expected {:#?} but got {:#?}", expected, got)
            }
        }
    }
}

impl std::error::Error for SyntaxError<'_> {}
