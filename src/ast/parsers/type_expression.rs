use crate::ast::{
    lexer::{AccessModifier::Pub, Keyword, TokenType},
    lexer_parser::{Identifier, Parser, Type},
    syntax_error::SyntaxError,
};

use super::interface::parse_interface_declaration;

pub fn parse_type_expression(parser: &mut Parser) -> Result<Type, SyntaxError> {
    while let Some(token) = parser.consume() {
        match &token.token_type {
            TokenType::Keyword(Keyword::Interface) => match parse_interface_declaration(parser) {
                Ok(interface) => {
                    return Ok(Type::InterfaceType(interface));
                }
                Err(error) => {
                    return Err(error);
                }
            },
            TokenType::ReturnType => continue,
            TokenType::Ident(value) => {
                return Ok(Type::TypeName(Identifier {
                    name: value.to_string(),
                    immutable: true,
                    access_modifier: Pub,
                }))
            }
            TokenType::Whitespace(_) => continue,
            _ => {
                return Err(SyntaxError::UnexpectedTokenButGotL(
                    vec![
                        TokenType::Keyword(Keyword::Interface),
                        TokenType::Ident("ident".into()),
                    ],
                    token.clone(),
                ));
            }
        }
    }

    return Ok(Type::Void);
}
