use crate::ast::{
    lexer_parser::{Parser, Type},
    syntax_error::SyntaxError,
};

pub fn parse_type_expression(parser: &mut Parser) -> Result<Type, SyntaxError> {
    todo!("implement")
}
