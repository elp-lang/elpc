use crate::ast::{
    lexer_parser::{Literal, Parser},
    syntax_error::SyntaxError,
};

pub fn parse_number_literal(parser: &mut Parser) -> Result<Literal, SyntaxError> {}
