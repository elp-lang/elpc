use crate::parser::{
    lexer::{Symbol, TokenType},
    lexer_parser::{Expression, Parser},
    syntax_error::SyntaxError,
};

use super::func_calls::parse_fn_call;

pub fn parse_ident(parser: &mut Parser) -> Result<Expression, SyntaxError> {
    while let Some(token) = parser.consume() {
        return match &token.token_type {
            TokenType::Symbol(Symbol::OpenParen) => {
                // Spit out the paren and ident first.
                parser.unconsume();
                parser.unconsume();
                match parse_fn_call(parser) {
                    Ok(fun) => {
                        Ok(Expression::Function(fun))
                    }
                    Err(err) => {
                        Err(SyntaxError::WrappedWithContextMessage(
                            "function call parsing".into(),
                            Box::new(err),
                        ))
                    }
                }
            }
            _ => {
                Err(SyntaxError::WrappedWithContextMessage(
                    "ident parsing".into(),
                    Box::new(SyntaxError::UnexpectedToken(token)),
                ))
            }
        }
    }

    Err(SyntaxError::MissingToken("unknown"))
}
