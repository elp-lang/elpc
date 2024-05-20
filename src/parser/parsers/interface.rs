use crate::parser::{
    lexer::{self, Symbol, TokenType},
    lexer_parser::{Identifier, InterfaceDeclaration, InterfaceProperty, Parser, Type},
    syntax_error::SyntaxError,
};

pub fn parse_interface_property(parser: &mut Parser) -> Result<InterfaceProperty, SyntaxError> {
    match &parser.current_token.clone() {
        None => Err(SyntaxError::MissingToken("property")),
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
                    if let Some(ident) = tokens.first() {
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
                            return Err(SyntaxError::ExpectedTokenButGot(
                                TokenType::Keyword(lexer::Keyword::Interface),
                                type_hint.clone(),
                            ))
                        }
                    };

                    Ok(property)
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
            TokenType::Symbol(Symbol::OpenBlock) => {
                found_opening_brace = true;
            }
            TokenType::Symbol(Symbol::CloseBlock) => {
                break;
            }
            TokenType::Symbol(Symbol::Period) => match parse_interface_property(parser) {
                Ok(interface_property) => interface.members.push(interface_property),
                Err(error) => return Err(error),
            },
            TokenType::EOF => break,
            TokenType::Symbol(Symbol::Colon) => {
                if !found_opening_brace {
                    return Err(SyntaxError::ExpectedTokenButGot(
                        TokenType::Symbol(Symbol::OpenBlock),
                        token.clone(),
                    ));
                }

                if !found_interface_name {
                    return Err(SyntaxError::ExpectedTokenButGot(
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
                return Err(SyntaxError::WrappedWithContextMessage(
                    "interface parsing".into(),
                    Box::new(SyntaxError::UnexpectedToken(token.clone())),
                ));
            }
        }
    }

    Ok(interface)
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        lexer::{AccessModifier, Lexer},
        lexer_parser::{
            AstNode, Identifier, InterfaceDeclaration, InterfaceProperty, Parser, Trie, Type,
        },
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_basic_interface() {
        let input = "interface MyInterface {
            .property: string
        }"
        .to_string();
        let mut lexer = Lexer::new(input.clone());
        let tokens = lexer.consume_all_tokens();
        let mut parser = Parser::new(tokens);

        assert_eq!(
            parser.parse(),
            Trie {
                nodes: vec!(AstNode::InterfaceDeclaration(InterfaceDeclaration {
                    name: Identifier {
                        immutable: true,
                        access_modifier: AccessModifier::Pub,
                        name: "MyInterface".to_string(),
                    },
                    members: vec!(InterfaceProperty {
                        name: Identifier {
                            immutable: true,
                            access_modifier: AccessModifier::Pub,
                            name: "property".to_string()
                        },
                        r#type: Type::TypeName(Identifier {
                            immutable: true,
                            access_modifier: AccessModifier::Pub,
                            name: "string".to_string()
                        })
                    }),
                }))
            }
        );
    }

    #[test]
    fn test_parse_interface() {
        let input = "interface MyInterface {
            .property: string
            .property: interface {
                .property: int32
                .property : number
            }
        }"
        .to_string();
        let mut lexer = Lexer::new(input.clone());
        let tokens = lexer.consume_all_tokens();
        let mut parser = Parser::new(tokens);

        assert_eq!(
            parser.parse(),
            Trie {
                nodes: vec!(AstNode::InterfaceDeclaration(InterfaceDeclaration {
                    name: Identifier {
                        immutable: true,
                        access_modifier: AccessModifier::Pub,
                        name: "MyInterface".to_string(),
                    },
                    members: vec!(
                        InterfaceProperty {
                            name: Identifier {
                                immutable: true,
                                access_modifier: AccessModifier::Pub,
                                name: "property".to_string()
                            },
                            r#type: Type::TypeName(Identifier {
                                immutable: true,
                                access_modifier: AccessModifier::Pub,
                                name: "string".to_string()
                            })
                        },
                        InterfaceProperty {
                            name: Identifier {
                                immutable: true,
                                access_modifier: AccessModifier::Pub,
                                name: "property".to_string()
                            },
                            r#type: Type::InterfaceType(InterfaceDeclaration {
                                name: Identifier {
                                    name: "".into(),
                                    immutable: true,
                                    access_modifier: AccessModifier::Pub,
                                },
                                members: vec!(
                                    InterfaceProperty {
                                        name: Identifier {
                                            immutable: true,
                                            access_modifier: AccessModifier::Pub,
                                            name: "property".to_string()
                                        },
                                        r#type: Type::TypeName(Identifier {
                                            immutable: true,
                                            access_modifier: AccessModifier::Pub,
                                            name: "int32".to_string()
                                        })
                                    },
                                    InterfaceProperty {
                                        name: Identifier {
                                            immutable: true,
                                            access_modifier: AccessModifier::Pub,
                                            name: "property".to_string()
                                        },
                                        r#type: Type::TypeName(Identifier {
                                            immutable: true,
                                            access_modifier: AccessModifier::Pub,
                                            name: "number".to_string()
                                        })
                                    },
                                )
                            })
                        }
                    ),
                }))
            }
        );
    }
}
