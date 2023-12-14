use crate::ast::{
    lexer::{self, Symbol, TokenType},
    lexer_parser::{AstNode, Identifier, InterfaceDeclaration, InterfaceProperty, Parser, Type},
    syntax_error::SyntaxError,
};

fn parse_interface_property<'a>(parser: &'a mut &Parser) -> Result<InterfaceProperty, SyntaxError> {
    println!("parsing interface property");
    match &parser.current_token.clone() {
        None => return Err(SyntaxError::MissingToken("property")),
        Some(_token) => {
            let mut property = InterfaceProperty {
                name: Identifier {
                    immutable: true,
                    access_modifier: lexer::AccessModifier::Pub,
                    name: "".to_string(),
                },
                r#type: Type::TypeName(Identifier {
                    immutable: true,
                    access_modifier: lexer::AccessModifier::Pub,
                    name: "unknown".to_string(),
                }),
            };

            while parser.consume().is_some() {
                if let Some(token) = &parser.current_token {
                    match &token.token_type {
                        // Skip whitespace.
                        TokenType::Whitespace(..) => continue,
                        TokenType::Symbol(Symbol::Colon) => continue,
                        TokenType::Ident(name) => {
                            if property.name.name != "".to_string() {
                                property.name.name = name.to_string();
                                continue;
                            } else {
                                property.r#type = Type::TypeName(Identifier {
                                    name: name.to_string(),
                                    access_modifier: lexer::AccessModifier::Pub,
                                    immutable: true,
                                });
                            }
                        }
                        _ => {
                            return Err(SyntaxError::UnexpectedToken(
                                parser.current_token.clone().unwrap(),
                            ));
                        }
                    }
                }
            }

            return Ok(property);
        }
    }
}

pub fn parse_interface_declaration(parser: &mut Parser) -> Result<AstNode, SyntaxError> {
    println!("parsing interface");
    match &parser.current_token.clone() {
        None => return Err(SyntaxError::MissingToken("interface")),
        Some(_token) => {
            let mut interface: InterfaceDeclaration = InterfaceDeclaration {
                name: Identifier {
                    immutable: true,
                    access_modifier: lexer::AccessModifier::Pub,
                    name: "".to_string(),
                },
                members: vec![],
            };

            let mut found_interface_name = false;
            let mut found_opening_brace = false;
            let mut found_closing_brace = false;
            while parser.consume().is_some() {
                let Some(token) = &parser.current_token else {
                    continue;
                };
                match &token.token_type {
                    // @TODO check this keyword is in the right place and possibly recurse.
                    TokenType::Keyword(lexer::Keyword::Interface) => continue,
                    // Skip whitespace.
                    TokenType::Whitespace(..) => continue,
                    lexer::TokenType::Symbol(lexer::Symbol::OpenBlock) => {
                        found_opening_brace = true;
                    }
                    lexer::TokenType::Symbol(lexer::Symbol::CloseBlock) => {
                        found_closing_brace = true;
                    }
                    lexer::TokenType::Symbol(Symbol::Period) => {
                        let result = parse_interface_property(parser);
                        if let Ok(property) = result {
                            interface.members.push(property);
                            continue;
                        }

                        return Err(result.err().unwrap());
                    }
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
                    // The first ident will be the name of the interface.
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

            return Ok(AstNode::InterfaceDeclaration(interface));
        }
    };
}
