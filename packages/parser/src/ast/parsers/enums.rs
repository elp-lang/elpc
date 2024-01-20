
use crate::ast::{
    lexer::{self, Symbol, TokenType},
    lexer_parser::{Identifier, enumDeclaration, enumProperty, Parser, Type, EnumVariant},
    syntax_error::SyntaxError,
};

pub fn parse_enum_property(parser: &mut Parser) -> Result<enumProperty, SyntaxError> {
    match &parser.current_token.clone() {
        None => return Err(SyntaxError::MissingToken("property")),
        Some(_token) => {
            let mut property = EnumVariant {
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
                        TokenType::Keyword(lexer::Keyword::enum) => {
                            let enum = parse_enum_declaration(parser);

                            match enum {
                                Ok(enum) => {
                                    property.r#type = Type::enumType(enum);
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
                                TokenType::Keyword(lexer::Keyword::enum),
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

pub fn parse_enum_declaration(
    parser: &mut Parser,
) -> Result<enumDeclaration, SyntaxError> {
    let mut enum = EnumDeclaration {
        name: Identifier {
            immutable: true,
            access_modifier: lexer::AccessModifier::Pub,
            name: "".to_string(),
        },
        members: vec![],
    };

    let mut found_enum_name = false;
    let mut found_opening_brace = false;

    while let Some(token) = parser.consume() {
        match &token.token_type {
            TokenType::Keyword(lexer::Keyword::enum) => continue,
            TokenType::Whitespace(..) => continue,
            lexer::TokenType::Symbol(lexer::Symbol::OpenBlock) => {
                found_opening_brace = true;
            }
            lexer::TokenType::Symbol(lexer::Symbol::CloseBlock) => {
                break;
            }
            lexer::TokenType::Symbol(Symbol::Period) => match parse_enum_property(parser) {
                Ok(enum_property) => enum.members.push(enum_property),
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

                if !found_enum_name {
                    return Err(SyntaxError::UnexpectedTokenButGot(
                        TokenType::Ident("enum".to_string()),
                        token.clone(),
                    ));
                }

                continue;
            }
            TokenType::Ident(name) => {
                if !found_enum_name {
                    found_enum_name = true;
                    enum.name.name = name.to_string();
                    continue;
                }
            }
            _ => {
                return Err(SyntaxError::UnexpectedToken(token.clone()));
            }
        }
    }

    Ok(enum)
}

#[cfg(test)]
mod tests {
    use crate::ast::{
        lexer::{AccessModifier, Lexer},
        lexer_parser::{
            AstNode, Identifier, ImportStatement, Parser,
            Trie, Type,
        },
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_import() {
        let input = "import { Thing } from \"elp\"".to_string();
        let mut lexer = Lexer::new(input.clone());
        let tokens = lexer.consume_all_tokens();
        let mut parser = Parser::new(tokens);

        assert_eq!(
            parser.parse(),
            Trie {
                nodes: vec!(AstNode::Import(ImportStatement {
                    members: vec!(Identifier {
                        name: "Thing".to_string(),
                        immutable: true,
                        access_modifier: AccessModifier::Pub,
                    }),
                    source_path: "elp".to_string(),
                }))
            }
        );
    }

    #[test]
    fn test_parse_enum() {
        let input = "enum MyEnum {
            .property
            .action(string)
        }"
        .to_string();
        let mut lexer = Lexer::new(input.clone());
        let tokens = lexer.consume_all_tokens();
        let mut parser = Parser::new(tokens);

        assert_eq!(
            parser.parse(),
            Trie {
                nodes: vec!()
            }
        );
    }
}
