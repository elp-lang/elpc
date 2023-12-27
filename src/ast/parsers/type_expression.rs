use crate::ast::{
    lexer::{Keyword, TokenType},
    lexer_parser::{Parser, Type},
    syntax_error::SyntaxError,
};

use super::{import::parse_import, interface::parse_interface_declaration};

pub fn parse_unknown_type(parser: &mut Parser) -> Result<Type, SyntaxError> {
    match &parser.current_token {
        Some(token) => match token.token_type {
            TokenType::Keyword(Keyword::Interface) => {
                return Ok(Type::InterfaceType(
                    parse_interface_declaration(parser).unwrap(),
                ))
            }
            TokenType::Keyword(Keyword::Import) => return Ok(parse_import(parser).unwrap()),
            _ => return Err(SyntaxError::UnexpectedToken(token.clone())),
        },
        None => return Err(SyntaxError::MissingToken("any")),
    }
}
