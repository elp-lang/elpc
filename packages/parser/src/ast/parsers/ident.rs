use crate::ast::{
    lexer::{Symbol, TokenType},
    lexer_parser::{Expression, Parser},
    syntax_error::SyntaxError,
};

use super::funcs::parse_fn_call;

pub fn parse_ident(parser: &mut Parser) -> Result<Expression, SyntaxError> {
    while let Some(token) = parser.consume() {
        match &token.token_type {
            TokenType::Symbol(Symbol::OpenParen) => match parse_fn_call(parser) {
                Ok(fun) => {
                    return Ok(Expression::Function(fun));
                }
                Err(err) => {
                    return Err(SyntaxError::WrappedWithContextMessage(
                        "interface parsing".into(),
                        Box::new(err),
                    ));
                }
            },
            _ => {
                return Err(SyntaxError::UnexpectedToken(token));
            }
        }
    }

    Err(SyntaxError::MissingToken("unknown"))
}
