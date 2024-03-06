use crate::ast::{
    lexer::{AccessModifier::Pub, Keyword, TokenType},
    lexer_parser::{Identifier, Parser, Type},
    syntax_error::SyntaxError,
};

use super::{enums::parse_enum_declaration, interface::parse_interface_declaration};

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
            TokenType::Keyword(Keyword::Enum) => match parse_enum_declaration(parser) {
                Ok(enum_declaration) => {
                    return Ok(Type::EnumType(enum_declaration));
                }
                Err(error) => {
                    return Err(error);
                }
            },
            TokenType::Whitespace(_) => continue,
            _ => {
                return Err(SyntaxError::ExpectedTokenButGotL(
                    vec![
                        TokenType::Keyword(Keyword::Interface),
                        TokenType::Ident("ident".into()),
                    ],
                    token.clone(),
                ));
            }
        }
    }

    Ok(Type::Void)
}
