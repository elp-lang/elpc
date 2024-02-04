use crate::ast::{
    lexer::{AccessModifier, Symbol, TokenType},
    lexer_parser::Parser,
    syntax_error::SyntaxError,
    EnumDeclaration, EnumVariant, EnumVariantType, Identifier, Parameter,
};

use super::type_expression::parse_type_expression;

fn parse_enum_variant_action_parameters(
    parser: &mut Parser,
) -> Result<Vec<Parameter>, SyntaxError> {
    let mut params: Vec<Parameter> = vec![];

    while let Some(token) = parser.consume() {
        match &token.token_type {
            TokenType::Ident(_) => {
                let mut param = Parameter {
                    ..Default::default()
                };
                parser.unconsume();
                match parse_type_expression(parser) {
                    Ok(val) => {
                        param.r#type = val;
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
                params.push(param);
            }
            TokenType::Symbol(Symbol::Comma) => continue,
            TokenType::Symbol(Symbol::CloseParen) => break,
            TokenType::Whitespace(_) => continue,
            _ => {
                return Err(SyntaxError::UnexpectedToken(token));
            }
        }
    }

    Ok(params)
}

fn parse_enum_variant(parser: &mut Parser) -> Result<EnumVariant, SyntaxError> {
    let mut variation = EnumVariant {
        r#type: Some(EnumVariantType::Option),
        ..Default::default()
    };

    while let Some(token) = parser.consume() {
        match &token.token_type {
            TokenType::Ident(value) => {
                variation.name = Identifier {
                    immutable: true,
                    access_modifier: AccessModifier::Pub,
                    name: value.clone(),
                }
            }
            TokenType::Symbol(Symbol::OpenParen) => {
                match parse_enum_variant_action_parameters(parser) {
                    Ok(params) => {
                        variation.r#type = Some(EnumVariantType::Action(params));
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            TokenType::Symbol(Symbol::CloseParen) => break,
            TokenType::Symbol(Symbol::CloseBlock) => break,
            TokenType::Symbol(Symbol::Period) => {
                parser.unconsume();
                break;
            }
            TokenType::Whitespace(_) => continue,
            _ => {
                return Err(SyntaxError::UnexpectedToken(token));
            }
        }
    }

    Ok(variation)
}

pub fn parse_enum_declaration(parser: &mut Parser) -> Result<EnumDeclaration, SyntaxError> {
    let mut declaration = EnumDeclaration {
        name: None,
        variants: vec![],
    };

    let mut found_open_brace = false;

    while let Some(token) = parser.consume() {
        match &token.token_type {
            TokenType::Ident(val) => {
                if found_open_brace {
                    return Err(SyntaxError::UnexpectedTokenButGot(
                        TokenType::Symbol(Symbol::Period),
                        token,
                    ));
                } else {
                    declaration.name = Some(Identifier {
                        name: val.clone(),
                        immutable: true,
                        access_modifier: AccessModifier::Pub,
                    });
                    continue;
                }
            }
            TokenType::Symbol(Symbol::CloseBlock) => break,
            TokenType::EOF => break,
            TokenType::Symbol(Symbol::OpenBlock) => {
                found_open_brace = true;
                continue;
            }
            TokenType::Symbol(Symbol::Period) => match parse_enum_variant(parser) {
                Ok(variant) => {
                    declaration.variants.push(variant);
                }
                Err(err) => {
                    return Err(err);
                }
            },
            TokenType::Whitespace(_) => continue,
            _ => {
                return Err(SyntaxError::UnexpectedToken(token));
            }
        }
    }

    Ok(declaration)
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use crate::ast::{
        lexer::{AccessModifier, Lexer},
        lexer_parser::Parser,
        parsers::enums::parse_enum_declaration,
        testing::Test,
        EnumDeclaration, EnumVariant, EnumVariantType, Identifier, Parameter, Type,
    };

    #[test]
    fn test_enum_parser() {
        let tests: Vec<Test<&'static str, EnumDeclaration>> = vec![Test {
            name: "enums",
            input: "enum MyEnum {
                    .VARIANT
                    .ACTION(String)
                    .ACTION2(String, Int, Float)
                }",
            expected: EnumDeclaration {
                name: Some(Identifier {
                    immutable: true,
                    access_modifier: AccessModifier::Pub,
                    name: "MyEnum".into(),
                }),
                variants: vec![
                    EnumVariant {
                        name: Identifier {
                            immutable: true,
                            access_modifier: AccessModifier::Pub,
                            name: "VARIANT".into(),
                        },
                        r#type: Some(EnumVariantType::Option),
                    },
                    EnumVariant {
                        name: Identifier {
                            immutable: true,
                            access_modifier: AccessModifier::Pub,
                            name: "ACTION".into(),
                        },
                        r#type: Some(EnumVariantType::Action(vec![Parameter {
                            name: None,
                            r#type: Type::TypeName(Identifier {
                                name: "String".into(),
                                immutable: true,
                                access_modifier: AccessModifier::Pub,
                            }),
                        }])),
                    },
                    EnumVariant {
                        name: Identifier {
                            immutable: true,
                            access_modifier: AccessModifier::Pub,
                            name: "ACTION2".into(),
                        },
                        r#type: Some(EnumVariantType::Action(vec![
                            Parameter {
                                name: None,
                                r#type: Type::TypeName(Identifier {
                                    name: "String".into(),
                                    immutable: true,
                                    access_modifier: AccessModifier::Pub,
                                }),
                            },
                            Parameter {
                                name: None,
                                r#type: Type::TypeName(Identifier {
                                    name: "Int".into(),
                                    immutable: true,
                                    access_modifier: AccessModifier::Pub,
                                }),
                            },
                            Parameter {
                                name: None,
                                r#type: Type::TypeName(Identifier {
                                    name: "Float".into(),
                                    immutable: true,
                                    access_modifier: AccessModifier::Pub,
                                }),
                            },
                        ])),
                    },
                ],
            },
        }];

        for test in tests {
            let mut lexer = Lexer::new(test.input.to_string());
            let tokens = lexer.consume_all_tokens();
            let mut parser = Parser::new(tokens);
            parser.consume();

            assert_eq!(parse_enum_declaration(&mut parser).unwrap(), test.expected);
        }
    }
}
