use crate::ast::{
    lexer::{self, AccessModifier, TokenType},
    lexer_parser::Parser,
    syntax_error::SyntaxError,
    Identifier, ImportStatement, ImportStatementMember,
};

fn parse_import_ident(
    parser: &mut Parser,
    import_name: String,
) -> Result<ImportStatementMember, SyntaxError> {
    let mut member = ImportStatementMember {
        alias: None,
        name: Identifier {
            immutable: true,
            access_modifier: AccessModifier::Pub,
            name: import_name,
        },
    };

    let mut dangling_alias = false;

    while parser.consume().is_some() {
        if let Some(token) = &parser.current_token {
            match &token.token_type {
                TokenType::Symbol(lexer::Symbol::Comma) => {
                    if dangling_alias {
                        return Err(SyntaxError::MissingToken("ident"));
                    }
                    break;
                }
                TokenType::Keyword(lexer::Keyword::As) => {
                    dangling_alias = true;
                    continue;
                }
                TokenType::Whitespace(_) => continue,
                TokenType::Symbol(lexer::Symbol::CloseBlock) => {
                    if dangling_alias {
                        return Err(SyntaxError::MissingToken("ident"));
                    }
                    parser.unconsume();
                    break;
                }
                TokenType::Ident(value) => {
                    member.alias = Some(value.clone());
                    dangling_alias = false;
                }
                _ => {
                    return Err(SyntaxError::UnexpectedToken(
                        parser.current_token.clone().unwrap(),
                    ));
                }
            }
        }
    }

    Ok(member)
}

pub fn parse_import(parser: &mut Parser) -> Result<ImportStatement, SyntaxError> {
    match parser.current_token.clone() {
        None => Err(SyntaxError::MissingToken("import")),
        Some(token) => {
            if token.token_type != lexer::TokenType::Keyword(lexer::Keyword::Import) {
                Err(SyntaxError::UnexpectedTokenButGot(
                    lexer::TokenType::Keyword(lexer::Keyword::Import),
                    token.clone(),
                ))
            } else {
                let mut import_statement = ImportStatement {
                    members: vec![],
                    source_path: "".to_string(),
                };

                // Collect all tokens until we get to lexer::TokenType::CloseBlock
                // this will include Ident, Comma and Whitespace and quotation marks.
                // We skip most of these characters.
                let mut found_opening_brace = false;
                let mut found_closing_brace = false;
                let mut imports: Vec<ImportStatementMember> = vec![];
                while parser.consume().is_some() {
                    if let Some(token) = &parser.current_token {
                        match &token.token_type {
                            // Skip whitespace and the opening brace but mark it as found.
                            lexer::TokenType::Symbol(lexer::Symbol::OpenBlock) => {
                                if found_opening_brace {
                                    return Err(SyntaxError::UnexpectedToken(token.clone()));
                                } else {
                                    found_opening_brace = true;
                                }
                            }
                            lexer::TokenType::Symbol(lexer::Symbol::CloseBlock) => {
                                if !found_opening_brace {
                                    return Err(SyntaxError::UnexpectedTokenButGot(
                                        TokenType::Symbol(lexer::Symbol::OpenBlock),
                                        token.clone(),
                                    ));
                                } else {
                                    found_closing_brace = true;
                                }
                            }
                            lexer::TokenType::Keyword(lexer::Keyword::Import) => {
                                continue;
                            }
                            lexer::TokenType::Keyword(lexer::Keyword::From) => {
                                if !found_closing_brace {
                                    let ast_node = Err(SyntaxError::UnexpectedTokenButGot(
                                        TokenType::Symbol(lexer::Symbol::CloseBlock),
                                        token.clone(),
                                    ));
                                    return ast_node;
                                }
                            }
                            lexer::TokenType::Symbol(..) => continue,
                            lexer::TokenType::Whitespace(..) => continue,
                            lexer::TokenType::Ident(value) => {
                                if found_closing_brace {
                                    import_statement.source_path = token.value.clone();
                                } else {
                                    match parse_import_ident(parser, value.clone()) {
                                        Ok(member) => imports.push(member),
                                        Err(err) => {
                                            return Err(err);
                                        }
                                    }
                                }
                            }
                            lexer::TokenType::EOF => break,
                            _ => {
                                return Err(SyntaxError::UnexpectedToken(
                                    parser.current_token.clone().unwrap(),
                                ));
                            }
                        };
                    }
                }

                if !found_closing_brace {
                    return Err(SyntaxError::MissingToken("}"));
                }

                import_statement.members = imports;

                Ok(import_statement)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use tests::lexer::Lexer;

    use crate::ast::testing::Test;

    use self::lexer::AccessModifier;

    use super::*;

    #[test]
    fn test_import() {
        let tests: Vec<Test<&'static str, ImportStatement>> = vec![
            Test {
                name: "basic import",
                input: "import { Thing } from \"elp\"",
                expected: ImportStatement {
                    source_path: "elp".into(),
                    members: vec![ImportStatementMember {
                        name: Identifier {
                            name: "Thing".into(),
                            access_modifier: AccessModifier::Pub,
                            immutable: true,
                        },
                        alias: None,
                    }],
                },
            },
            Test {
                name: "import as alias",
                input: "import { Thing as Other } from \"elp\"",
                expected: ImportStatement {
                    source_path: "elp".into(),
                    members: vec![ImportStatementMember {
                        alias: Some("Other".into()),
                        name: Identifier {
                            name: "Thing".into(),
                            access_modifier: AccessModifier::Pub,
                            immutable: true,
                        },
                    }],
                },
            },
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.input.to_string());
            let tokens = lexer.consume_all_tokens();
            let mut parser = Parser::new(tokens);
            // We assume you've consumed the import token already to know you need to parse an
            // import expression so consume the "import" keyword and call the parser..
            parser.consume();

            assert_eq!(parse_import(&mut parser).unwrap(), test.expected);
        }
    }
}
