use crate::ast::{
    lexer::{Symbol, TokenType},
    lexer_parser::{Expression, Parser},
    syntax_error::SyntaxError,
};

use super::func_calls::parse_fn_call;

pub fn parse_ident(parser: &mut Parser) -> Result<Expression, SyntaxError> {
    while let Some(token) = parser.consume() {
        match &token.token_type {
            TokenType::Symbol(Symbol::OpenParen) => {
                // Spit out the paren and ident first.
                parser.unconsume();
                parser.unconsume();
                match parse_fn_call(parser) {
                    Ok(fun) => {
                        return Ok(Expression::Function(fun));
                    }
                    Err(err) => {
                        return Err(SyntaxError::WrappedWithContextMessage(
                            "function call parsing".into(),
                            Box::new(err),
                        ));
                    }
                }
            }
            _ => {
                return Err(SyntaxError::WrappedWithContextMessage(
                    "ident parsing".into(),
                    Box::new(SyntaxError::UnexpectedToken(token)),
                ));
            }
        }
    }

    Err(SyntaxError::MissingToken("unknown"))
}
