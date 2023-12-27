use crate::ast::{
    lexer::{self, Symbol, TokenType},
    lexer_parser::{Identifier, InterfaceDeclaration, InterfaceProperty, Parser, Type},
    syntax_error::SyntaxError,
};

fn parse_interface_property(parser: &mut Parser) -> Result<InterfaceProperty, SyntaxError> {
    match &parser.current_token.clone() {
        None => return Err(SyntaxError::MissingToken("property")),
        Some(_token) => {
            let mut property = InterfaceProperty {
                name: Identifier {
                    immutable: true,
                    access_modifier: lexer::AccessModifier::Pub,
                    name: "anonymous".to_string(),
                },
                r#type: Type::TypeName(Identifier {
                    immutable: true,
                    access_modifier: lexer::AccessModifier::Pub,
                    name: "unknown".to_string(),
                }),
            };

            let tokens = parser.consume_n(3);

            match tokens {
                Ok(tokens) => {
                    // first token must be an ident,
                    // second will be a colon
                    // third will be a type expression
                    if let Some(ident) = tokens.get(0) {
                        property.name.name = ident.value.clone();
                    } else {
                        return Err(SyntaxError::MissingToken("ident"));
                    }

                    let colon = tokens.get(1).unwrap();
                    if colon.token_type != TokenType::Symbol(Symbol::Colon) {
                        return Err(SyntaxError::MissingToken(":"));
                    }

                    let type_hint = tokens.get(2).unwrap();

                    match &type_hint.token_type {
                        TokenType::Keyword(lexer::Keyword::Interface) => {
                            let interface = parse_interface_declaration(parser);

                            match interface {
                                Ok(interface) => {
                                    property.r#type = Type::InterfaceType(interface);
                                }
                                Err(error) => {
                                    return Err(error);
                                }
                            }
                        }
                        TokenType::Ident(ident) => {
                            property.r#type = Type::TypeName(Identifier {
                                immutable: true,
                                access_modifier: lexer::AccessModifier::Pub,
                                name: ident.clone(),
                            })
                        }
                        _ => {
                            return Err(SyntaxError::UnexpectedTokenButGot(
                                TokenType::Keyword(lexer::Keyword::Interface),
                                type_hint.clone(),
                            ))
                        }
                    };

                    return Ok(property);
                }
                Err(_) => Err(SyntaxError::MissingToken("ident")),
            }
        }
    }
}

pub fn parse_interface_declaration(
    parser: &mut Parser,
) -> Result<InterfaceDeclaration, SyntaxError> {
    let mut interface = InterfaceDeclaration {
        name: Identifier {
            immutable: true,
            access_modifier: lexer::AccessModifier::Pub,
            name: "".to_string(),
        },
        members: vec![],
    };

    let mut found_interface_name = false;
    let mut found_opening_brace = false;

    while let Some(token) = parser.consume() {
        match &token.token_type {
            TokenType::Keyword(lexer::Keyword::Interface) => continue,
            TokenType::Whitespace(..) => continue,
            lexer::TokenType::Symbol(lexer::Symbol::OpenBlock) => {
                found_opening_brace = true;
            }
            lexer::TokenType::Symbol(lexer::Symbol::CloseBlock) => {
                break;
            }
            lexer::TokenType::Symbol(Symbol::Period) => match parse_interface_property(parser) {
                Ok(interface_property) => interface.members.push(interface_property),
                Err(error) => return Err(error),
            },
            lexer::TokenType::EOF => break,
            TokenType::Symbol(Symbol::Colon) => {
                if !found_opening_brace {
                    return Err(SyntaxError::UnexpectedTokenButGot(
                        TokenType::Symbol(Symbol::OpenBlock),
                        token.clone(),
                    ));
                }

                if !found_interface_name {
                    return Err(SyntaxError::UnexpectedTokenButGot(
                        TokenType::Ident("interface".to_string()),
                        token.clone(),
                    ));
                }

                continue;
            }
            TokenType::Ident(name) => {
                if !found_interface_name {
                    found_interface_name = true;
                    interface.name.name = name.to_string();
                    continue;
                }
            }
            _ => {
                return Err(SyntaxError::UnexpectedToken(token.clone()));
            }
        }
    }

    Ok(interface)
}
