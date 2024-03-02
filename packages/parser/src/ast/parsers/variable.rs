use crate::ast::{
    lexer::{AccessModifier, TokenType},
    lexer_parser::{Identifier, Parser, VariableDeclaration},
    syntax_error::SyntaxError,
};

pub fn parse_variable(parser: &mut Parser) -> Result<VariableDeclaration, SyntaxError> {
    let declaration = VariableDeclaration {
        ident: Identifier {
            name: "".into(),
            immutable: true,
            access_modifier: AccessModifier::Pub,
        },
        r#type: crate::ast::lexer_parser::Type::Void,
        value: None,
    };

    while let Some(token) = parser.consume() {
        match &token.token_type {
            TokenType::Keyword(Keyword::Const) => {}
            _ => {
                return Err(SyntaxError::UnexpectedToken(token));
            }
        }
    }

    Ok(declaration)
}

#[cfg(test)]
mod tests {
    use crate::ast::{
        lexer::{AccessModifier, Lexer},
        lexer_parser::{Identifier, Parser, Type, VariableDeclaration},
        testing::Test,
    };
    use pretty_assertions::assert_eq;

    use super::parse_variable;

    #[test]
    fn test_variable_declarations() {
        let tests: Vec<Test<&'static str, VariableDeclaration>> = vec![
            Test {
                name: "basic",
                input: "{
                const NAME: string
            }",
                expected: VariableDeclaration {
                    ident: Identifier {
                        immutable: true,
                        access_modifier: AccessModifier::Pub,
                        name: "NAME".to_string(),
                    },
                    r#type: Type::TypeName(Identifier {
                        immutable: true,
                        access_modifier: AccessModifier::Pub,
                        name: "string".to_string(),
                    }),
                    value: None,
                },
            },
            Test {
                name: "basic",
                input: "{
                let NAME: string
            }",
                expected: VariableDeclaration {
                    ident: Identifier {
                        immutable: false,
                        access_modifier: AccessModifier::Pub,
                        name: "NAME".to_string(),
                    },
                    r#type: Type::TypeName(Identifier {
                        immutable: true,
                        access_modifier: AccessModifier::Pub,
                        name: "string".to_string(),
                    }),
                    value: None,
                },
            },
            Test {
                name: "basic",
                input: "{
                let NAME = \"hello world\"
            }",
                expected: VariableDeclaration {
                    ident: Identifier {
                        immutable: false,
                        access_modifier: AccessModifier::Pub,
                        name: "NAME".to_string(),
                    },
                    r#type: Type::TypeName(Identifier {
                        immutable: true,
                        access_modifier: AccessModifier::Pub,
                        name: "string".to_string(),
                    }),
                    value: None,
                },
            },
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.input.to_string());
            let tokens = lexer.consume_all_tokens();
            let mut parser = Parser::new(tokens);
            parser.consume();

            assert_eq!(parse_variable(&mut parser).unwrap(), test.expected);
        }
    }
}
