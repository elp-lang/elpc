use crate::ast::{
    lexer::{AccessModifier, Keyword, Symbol, TokenType, Whitespace},
    lexer_parser::{Identifier, Parser, VariableDeclaration},
    syntax_error::SyntaxError,
};

use super::{expression::parse_expression, type_expression::parse_type_expression};

pub fn parse_variable(parser: &mut Parser) -> Result<VariableDeclaration, SyntaxError> {
    let mut declaration = VariableDeclaration {
        ident: Identifier {
            name: "".into(),
            immutable: true,
            access_modifier: AccessModifier::Pub,
        },
        r#type: Type::Void,
        value: None,
    };

    while let Some(token) = parser.consume() {
        match &token.token_type {
            TokenType::Keyword(Keyword::Const) => {
                declaration.ident.immutable = true;
            }
            TokenType::Keyword(Keyword::Var) => {
                declaration.ident.immutable = false;
            }
            TokenType::Ident(val) => {
                declaration.ident.name = val.to_string();
            }
            TokenType::Symbol(Symbol::Colon) => match parse_type_expression(parser) {
                Ok(type_info_token) => {
                    declaration.r#type = type_info_token;
                }
                Err(err) => {
                    return Err(err);
                }
            },
            TokenType::Symbol(Symbol::SingleEqual) => match parse_expression(parser) {
                Ok(expression) => {
                    declaration.value = Some(expression);
                }
                Err(err) => {
                    return Err(err);
                }
            },
            TokenType::Whitespace(Whitespace::NewLine) => break,
            TokenType::Whitespace(..) => continue,
            TokenType::EOF => continue,
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
        lexer_parser::{Expression, Identifier, Literal, Parser, Type, VariableDeclaration},
        testing::Test,
    };
    use pretty_assertions::assert_eq;

    use super::parse_variable;

    #[test]
    fn test_variable_declarations() {
        let tests: Vec<Test<&'static str, VariableDeclaration>> = vec![
            Test {
                name: "const declaration",
                input: "const constant: string",
                expected: VariableDeclaration {
                    ident: Identifier {
                        immutable: true,
                        access_modifier: AccessModifier::Pub,
                        name: "constant".to_string(),
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
                name: "mutable declaration",
                input: "var mutable: string",
                expected: VariableDeclaration {
                    ident: Identifier {
                        immutable: false,
                        access_modifier: AccessModifier::Pub,
                        name: "mutable".to_string(),
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
                name: "constant binding",
                input: "const immutable_binding = \"hello world\"",
                expected: VariableDeclaration {
                    ident: Identifier {
                        immutable: true,
                        access_modifier: AccessModifier::Pub,
                        name: "immutable_binding".to_string(),
                    },
                    // We won't know the type until the next stage of analysis.
                    r#type: Type::Void,
                    value: Some(Expression::Literal(Literal::String(
                        "hello world".to_string(),
                    ))),
                },
            },
            Test {
                name: "mutable binding",
                input: "var mutable_binding = \"hello world\"",
                expected: VariableDeclaration {
                    ident: Identifier {
                        immutable: false,
                        access_modifier: AccessModifier::Pub,
                        name: "mutable_binding".to_string(),
                    },
                    // We won't know the type until the next stage of analysis.
                    r#type: Type::Void,
                    value: Some(Expression::Literal(Literal::String(
                        "hello world".to_string(),
                    ))),
                },
            },
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.input.to_string());
            let tokens = lexer.consume_all_tokens();
            let mut parser = Parser::new(tokens);

            assert_eq!(parse_variable(&mut parser).unwrap(), test.expected);
        }
    }
}
