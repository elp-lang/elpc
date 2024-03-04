use std::fmt;

use super::lexer::{Token, TokenType};

#[derive(Debug)]
pub enum SyntaxError {
    UnexpectedToken(Token),
    UnexpectedTokenType(TokenType),
    UnexpectedTokenButGot(TokenType, Token),
    UnexpectedTokenButGotL(Vec<TokenType>, Token),
    MissingToken(&'static str),
    WrappedWithContextMessage(String, Box<SyntaxError>),
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxError::UnexpectedToken(token) => write!(f, "Unexpected token: {:#?}", token),
            SyntaxError::UnexpectedTokenType(token) => {
                write!(f, "Unexpected token type: {:#?}", token)
            }
            SyntaxError::MissingToken(token) => write!(f, "Missing token: {:#?}", token),
            SyntaxError::UnexpectedTokenButGot(expected, got) => {
                write!(f, "Expected {:#?} but got {:#?}", expected, got)
            }
            SyntaxError::UnexpectedTokenButGotL(expected, got) => {
                let joined: String = expected
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(" or ");
                write!(f, "Expected {} but got {:#?}", joined, got)
            }
            SyntaxError::WrappedWithContextMessage(ctx_message, err) => {
                write!(f, "syntax error: {}\n{:#?}", ctx_message, err)
            }
        }
    }
}

impl std::error::Error for SyntaxError {}
