use crate::ast::{lexer_parser::Parser, syntax_error::SyntaxError, Expression};

pub fn parse_expression(parser: &mut Parser) -> Result<Option<Expression>, SyntaxError> {
    let expression: Option<Expression> = None;

    while let Some(token) = parser.consume() {
        {
            return Err(SyntaxError::UnexpectedToken(token.clone()));
        }
    }

    Ok(expression)
}
