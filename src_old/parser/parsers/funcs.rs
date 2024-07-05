use crate::parser::{
    lexer::{AccessModifier::Pub, Symbol, TokenType},
    lexer_parser::{Fn, Identifier, Parameter, Parser, Type},
    syntax_error::SyntaxError,
};

use super::{
    block::parse_block, expression::parse_expression, type_expression::parse_type_expression,
};

pub fn parse_fn_parameter(parser: &mut Parser, position: i32) -> Result<Parameter, SyntaxError> {
    let mut param = Parameter {
        position,
        name: None,
        value: None,
        r#type: Type::TypeName(Identifier {
            ..Default::default()
        }),
    };

    // yikes.
    let mut found_colon = false;

    while let Some(token) = parser.consume() {
        match &token.token_type {
            TokenType::Ident(value) => {
                if !found_colon {
                    param.name = Some(Identifier {
                        name: value.into(),
                        immutable: true,
                        access_modifier: Pub,
                    })
                } else {
                    param.r#type = Type::TypeName(Identifier {
                        name: value.into(),
                        immutable: true,
                        access_modifier: Pub,
                    })
                }
            }
            TokenType::Symbol(Symbol::Colon) => {
                found_colon = true;
                continue;
            }
            TokenType::Symbol(Symbol::SingleEqual) => match parse_expression(parser) {
                Ok(expr) => {
                    print!("VALUE {:#?}", expr);
                    param.value = Some(expr);
                }
                Err(err) => {
                    return Err(SyntaxError::WrappedWithContextMessage(
                        "parsing parameter".into(),
                        Box::new(err),
                    ));
                }
            },
            TokenType::Symbol(Symbol::Comma) => break,
            _ => {
                return Err(SyntaxError::ExpectedTokenButGot(
                    TokenType::Ident("param name".into()),
                    token.clone(),
                ));
            }
        }
    }

    Ok(param)
}

pub fn parse_fn_parameters(parser: &mut Parser) -> Result<Vec<Parameter>, SyntaxError> {
    let mut params: Vec<Parameter> = vec![];
    let mut position = 0;

    while let Ok(param) = parse_fn_parameter(parser, position) {
        params.push(param);
        position += 1;
    }

    Ok(params)
}

pub fn parse_fn(parser: &mut Parser) -> Result<Fn, SyntaxError> {
    let mut fn_declaration = Fn {
        name: Some(Identifier {
            name: "".into(),
            immutable: true,
            access_modifier: Pub,
        }),
        is_call: false,
        is_callable: true,
        block: None,
        params: vec![],
        returns: Box::new(Type::Undefined),
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
            TokenType::ReturnType => match parse_type_expression(parser) {
                Ok(expr) => {
                    fn_declaration.returns = Box::new(expr);
                }
                Err(error) => {
                    return Err(error);
                }
            },
            TokenType::Symbol(Symbol::OpenParen) => match parser.peek() {
                Ok(next) => match next.token_type {
                    TokenType::Ident(_) => {
                        let params = parse_fn_parameters(parser);
                        match params {
                            Ok(params) => fn_declaration.params = params,
                            Err(error) => {
                                return Err(error);
                            }
                        }
                    }
                    TokenType::Symbol(Symbol::CloseParen) => continue,
                    _ => {
                        return Err(SyntaxError::MissingToken(") or param"));
                    }
                },
                Err(_) => {
                    return Err(SyntaxError::MissingToken(") or param"));
                }
            },
            TokenType::Symbol(Symbol::CloseParen) => continue,
            TokenType::Symbol(Symbol::OpenBlock) => match parse_block(parser) {
                Ok(block) => {
                    fn_declaration.block = Some(Box::new(block));
                }
                Err(err) => return Err(err),
            },
            TokenType::Whitespace(_) => continue,
            TokenType::EOF => break,
            _ => {
                return Err(SyntaxError::UnexpectedToken(token.clone()));
            }
        }
    }

    Ok(fn_declaration)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::parser::{
        lexer::{AccessModifier, Lexer},
        lexer_parser::{
            Fn, Identifier, InterfaceProperty, Parser, Type,InterfaceDeclaration
        },
        parsers::funcs::parse_fn,
        testing::Test,
    };

    #[test]
    fn test_fn_signature_parser() {
        let tests: Vec<Test<&'static str, Fn>> = vec![
            Test {
                name: "basic",
                input: "fn MyFunction() -> thing",
                expected: Fn {
                    name: Some(Identifier {
                        name: "MyFunction".into(),
                        immutable: true,
                        access_modifier: AccessModifier::Pub,
                    }),
                    block: None,
                    is_call: false,
                    is_callable: true,
                    params: vec![],
                    returns: Box::new(Type::TypeName(Identifier {
                        immutable: true,
                        access_modifier: AccessModifier::Pub,
                        name: "thing".into(),
                    })),
                },
            },
            Test {
                name: "return inline interface",
                input: "fn MyFunction() -> interface { .first: thing }",
                expected: Fn {
                    name: Some(Identifier {
                        name: "MyFunction".into(),
                        immutable: true,
                        access_modifier: AccessModifier::Pub,
                    }),
                    params: vec![],
                    block: None,
                    is_call: false,
                    is_callable: true,
                    returns: Box::new(Type::InterfaceType(
                        InterfaceDeclaration {
                            name: Identifier {
                                name: "".into(),
                                immutable: true,
                                access_modifier: AccessModifier::Pub,
                            },
                            members: vec![InterfaceProperty {
                                name: Identifier {
                                    name: "first".into(),
                                    immutable: true,
                                    access_modifier: AccessModifier::Pub,
                                },
                                r#type: Type::TypeName(Identifier {
                                    immutable: true,
                                    access_modifier: AccessModifier::Pub,
                                    name: "thing".into(),
                                }),
                            }],
                        },
                    )),
                },
            },
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.input.to_string());
            let tokens = lexer.consume_all_tokens();
            let mut parser = Parser::new(tokens);
            parser.consume();

            assert_eq!(parse_fn(&mut parser).unwrap(), test.expected);
        }
    }
}
