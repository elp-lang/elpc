use crate::ast::{
    lexer::{AccessModifier::Pub, Symbol, TokenType},
    lexer_parser::{Fn, Identifier, Parameter, Parser, Type},
    syntax_error::SyntaxError,
};

pub fn parse_fn(parser: &mut Parser) -> Result<Fn, SyntaxError> {
    let mut fn_declaration = Fn {
        name: Identifier {
            name: "".into(),
            immutable: true,
            access_modifier: Pub,
        },
        params: vec![],
        returns: Identifier {
            name: "".into(),
            immutable: true,
            access_modifier: Pub,
        },
    };

    while parser.consume().is_some() {
        if let Some(token) = &parser.current_token {
            match &token.token_type {
                TokenType::Symbol(Symbol::OpenParen) => continue,
                TokenType::Ident(ident) => match parser.peek() {
                    Ok(next_token) => {
                        match next_token.token_type {
                            TokenType::Symbol(Symbol::Colon) => {
                                // Peek doesn't consume the token so we grab the next 2 to check the
                                // second is an ident.
                                let tokens = parser.consume_n(2);

                                match tokens {
                                    Ok(tokens) => {}
                                    Err(_) => return Err(SyntaxError::MissingToken("ident")),
                                }
                            }
                            _ => {
                                return Err(SyntaxError::UnexpectedToken(
                                    parser.current_token.clone().unwrap(),
                                ));
                            }
                        }
                    }
                    Err(_) => {
                        return Err(SyntaxError::MissingToken("any"));
                    }
                },
                _ => {
                    return Err(SyntaxError::UnexpectedToken(
                        parser.current_token.clone().unwrap(),
                    ));
                }
            }
        }
    }

    Ok(fn_declaration)
}