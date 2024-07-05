use crate::parser::{
    lexer::{AccessModifier::Pub, Symbol, TokenType},
    lexer_parser::{Expression, Fn, Identifier, Literal, Parameter, Parser, Type},
    syntax_error::SyntaxError,
};

use super::expression::parse_expression;

pub fn parse_fn_call_argument(
    parser: &mut Parser,
    position: i32,
) -> Result<Option<Parameter>, SyntaxError> {
    let mut param = Parameter {
        position,
        name: None,
        value: None,
        r#type: Type::TypeName(Identifier {
            ..Default::default()
        }),
    };

    while let Some(token) = parser.consume() {
        match &token.token_type {
            TokenType::Symbol(Symbol::Comma) => break,
            TokenType::Symbol(Symbol::CloseParen) => break,
            TokenType::Ident(name) => {
                param.name = Some(Identifier {
                    immutable: true,
                    access_modifier: Pub,
                    name: name.into(),
                });

                match parser.peek() {
                    Ok(next) => match &next.token_type {
                        TokenType::Symbol(Symbol::SingleEqual) => match parse_expression(parser) {
                            Ok(expr) => param.value = Some(expr),
                            Err(err) => {
                                return Err(SyntaxError::WrappedWithContextMessage(
                                    "parsing named fn call argument".into(),
                                    Box::new(err),
                                ));
                            }
                        },
                        _ => {
                            return Err(SyntaxError::WrappedWithContextMessage(
                                "func call -> ident -> peek".into(),
                                Box::new(SyntaxError::UnexpectedToken(next)),
                            ));
                        }
                    },
                    Err(err) => {
                        return Err(SyntaxError::WrappedWithContextMessage(
                            "parsing named fn call argument".into(),
                            Box::new(SyntaxError::Unknown(err)),
                        ));
                    }
                }
            }
            TokenType::BooleanLiteral(b) => {
                param.r#type = Type::Literal(Literal::Boolean(*b));
                param.value = Some(Expression::Literal(Literal::Boolean(*b)));
            }
            TokenType::FloatLiteral(f) => {
                param.r#type = Type::Literal(Literal::Float(*f));
                param.value = Some(Expression::Literal(Literal::Float(*f)));
            }
            TokenType::IntegerLiteral(i) => {
                param.r#type = Type::Literal(Literal::Number(*i));
                param.value = Some(Expression::Literal(Literal::Number(*i)));
            }
            TokenType::StringLiteral(str) => {
                param.r#type = Type::Literal(Literal::String(str.to_string()));
                param.value = Some(Expression::Literal(Literal::String(str.to_string())));
            }
            _ => match parse_expression(parser) {
                Ok(expr) => {
                    param.value = Some(expr);
                }
                Err(err) => {
                    return Err(SyntaxError::WrappedWithContextMessage(
                        "parsing fn call argument".into(),
                        Box::new(err),
                    ));
                }
            },
        }
    }

    if param.value != None {
        Ok(Some(param))
    } else {
        Ok(None)
    }
}

pub fn parse_fn_call_arguments(parser: &mut Parser) -> Result<Vec<Parameter>, SyntaxError> {
    let mut params: Vec<Parameter> = vec![];
    let mut position = 0;

    while let Ok(result) = parse_fn_call_argument(parser, position) {
        if let Some(param) = result {
            params.push(param);
            position += 1;
        }
    }

    Ok(params)
}

pub fn parse_fn_call(parser: &mut Parser) -> Result<Fn, SyntaxError> {
    let mut fn_declaration = Fn {
        name: Some(Identifier {
            name: "".into(),
            immutable: true,
            access_modifier: Pub,
        }),
        is_call: true,
        is_callable: false,
        block: None,
        params: vec![],
        returns: Box::new(Type::Void),
    };

    while let Some(token) = parser.consume() {
        match &token.token_type {
            TokenType::Ident(name) => {
                fn_declaration.name = Some(Identifier {
                    name: name.into(),
                    access_modifier: Pub,
                    immutable: true,
                })
            }
            TokenType::Symbol(Symbol::OpenParen) => match parse_fn_call_arguments(parser) {
                Ok(param) => {
                    fn_declaration.params = param;
                }
                Err(err) => {
                    return Err(SyntaxError::WrappedWithContextMessage(
                        "parsing call params".into(),
                        Box::new(err),
                    ));
                }
            },
            TokenType::Symbol(Symbol::CloseParen) => break,
            TokenType::EOF => break,
            _ => {
                return Err(SyntaxError::WrappedWithContextMessage(
                    "parse fn call".into(),
                    Box::new(SyntaxError::UnexpectedToken(token.clone())),
                ));
            }
        }
    }

    Ok(fn_declaration)
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        lexer::{AccessModifier, Lexer},
        lexer_parser::{Expression, Fn, Identifier, Literal, Parameter, Parser, Type},
        parsers::func_calls::parse_fn_call,
        testing::Test,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_fn_call_parser() {
        let tests: Vec<Test<&'static str, Fn>> = vec![
            Test {
                name: "parse simple call",
                input: "increment()",
                expected: Fn {
                    name: Some(Identifier {
                        name: "increment".into(),
                        immutable: true,
                        access_modifier: AccessModifier::Pub,
                    }),
                    params: vec![],
                    block: None,
                    is_call: true,
                    is_callable: false,
                    returns: Box::new(Type::Void),
                },
            },
            Test {
                name: "single expression argument",
                input: "print(234)",
                expected: Fn {
                    name: Some(Identifier {
                        name: "print".into(),
                        immutable: true,
                        access_modifier: AccessModifier::Pub,
                    }),
                    params: vec![Parameter {
                        name: None,
                        position: 0,
                        value: Some(Expression::Literal(Literal::Number(234.into()))),
                        r#type: Type::Literal(Literal::Number(234.into())),
                    }],
                    block: None,
                    is_call: true,
                    is_callable: false,
                    returns: Box::new(Type::Void),
                },
            },
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.input.to_string());
            let tokens = lexer.consume_all_tokens();
            let mut parser = Parser::new(tokens);

            assert_eq!(parse_fn_call(&mut parser).unwrap(), test.expected);
        }
    }
}
