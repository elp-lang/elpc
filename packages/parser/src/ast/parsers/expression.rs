use crate::ast::{
    lexer_parser::{Expression, Parser},
    syntax_error::SyntaxError,
};

pub fn parse_expression(parser: &mut Parser) -> Result<Option<Expression>, SyntaxError> {
    let mut expression: Option<Expression> = None;

    while let Some(token) = parser.consume() {
        match &token.token_type {
            _ => {
                return Err(SyntaxError::UnexpectedToken(token.clone()));
            }
        }
    }

    Ok(expression)
}
