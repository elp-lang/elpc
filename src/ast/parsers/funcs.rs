use crate::ast::{
    lexer::{AccessModifier::Pub, Symbol, TokenType},
    lexer_parser::{Fn, Identifier, Parameter, Parser, Type},
    syntax_error::SyntaxError,
};

use super::type_expression::parse_type_expression;

pub fn parse_fn_parameter(parser: &mut Parser) -> Result<Parameter, SyntaxError> {
    let mut param = Parameter {
        name: None,
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
            TokenType::Symbol(Symbol::Comma) => break,
            _ => {
                return Err(SyntaxError::UnexpectedToken(token.clone()));
            }
        }
    }

    Ok(param)
}

pub fn parse_fn_parameters(parser: &mut Parser) -> Result<Vec<Parameter>, SyntaxError> {
    let mut params: Vec<Parameter> = vec![];

    while let Ok(param) = parse_fn_parameter(parser) {
        params.push(param);
    }

    Ok(params)
}

pub fn parse_fn(parser: &mut Parser, with_body: bool) -> Result<Fn, SyntaxError> {
    let mut fn_declaration = Fn {
        name: Some(Identifier {
            name: "".into(),
            immutable: true,
            access_modifier: Pub,
        }),
        params: vec![],
        returns: Box::new(Type::Undefined),
    };

    // the next 3 tokens could/should be an ident, opening parenthesis and parameter.
    if let Ok(next_tokens) = parser.consume_n(3) {
        let name = next_tokens.get(0).unwrap();

        match &name.token_type {
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
                    _ => {
                        return Err(SyntaxError::MissingToken(") or param"));
                    }
                },
                Err(_) => {
                    return Err(SyntaxError::MissingToken(") or param"));
                }
            },
            TokenType::Symbol(Symbol::OpenBlock) => {
                if !with_body {
                    return Err(SyntaxError::UnexpectedToken(name.clone()));
                }
                todo!("body parsing")
            }
            _ => {
                return Err(SyntaxError::UnexpectedTokenButGot(
                    TokenType::Ident("ident or (".into()),
                    name.clone(),
                ));
            }
        }
    }

    Ok(fn_declaration)
}

#[cfg(test)]
mod tests {
    use crate::ast::{
        lexer::{AccessModifier, Lexer},
        lexer_parser::{Fn, Identifier, Parser, Type},
        parsers::funcs::parse_fn,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_fn_signature_parser() {
        let input = "fn MyFunction() -> thing".to_string();
        let mut lexer = Lexer::new(input.clone());
        let tokens = lexer.consume_all_tokens();
        let mut parser = Parser::new(tokens);
        parser.consume();

        assert_eq!(
            parse_fn(&mut parser, false).unwrap(),
            Fn {
                name: Some(Identifier {
                    name: "MyFunction".into(),
                    immutable: true,
                    access_modifier: AccessModifier::Pub,
                }),
                params: vec!(),
                returns: Box::new(Type::TypeName(Identifier {
                    immutable: true,
                    access_modifier: AccessModifier::Pub,
                    name: "thing".into()
                }))
            }
        );
    }
}
