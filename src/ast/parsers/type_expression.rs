use crate::ast::{
    lexer::{AccessModifier::Pub, Keyword, TokenType},
    lexer_parser::{Identifier, Parser, Type},
    syntax_error::SyntaxError,
};

use super::interface::parse_interface_declaration;

pub fn parse_type_expression(parser: &mut Parser) -> Result<Type, SyntaxError> {
    if let Some(token) = &parser.current_token {
        match &token.token_type {
            TokenType::Keyword(Keyword::Interface) => match parse_interface_declaration(parser) {
                Ok(interface) => {
                    return Ok(Type::InterfaceType(interface));
                }
                Err(error) => {
                    return Err(error);
                }
            },
            TokenType::Ident(value) => {
                return Ok(Type::TypeName(Identifier {
                    name: value.to_string(),
                    immutable: true,
                    access_modifier: Pub,
                }))
            }
            _ => {
                return Err(SyntaxError::UnexpectedTokenButGot(
                    TokenType::Keyword(Keyword::Interface),
                    token.clone(),
                ));
            }
        }
    } else {
        Err(SyntaxError::MissingToken("Type"))
    }
}
