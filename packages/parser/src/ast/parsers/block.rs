use crate::ast::{
    lexer_parser::{Block, Parser},
    syntax_error::SyntaxError,
};

pub fn parse_block(parser: &mut Parser) -> Result<Block, SyntaxError> {
    let mut block_declaration = Block {
        expressions: vec![],
    };

    while let Some(token) = parser.consume() {
        match &token.token_type {
            _ => {
                return Err(SyntaxError::UnexpectedToken(token));
            }
        }
    }

    Ok(block_declaration)
}

#[cfg(test)]
mod tests {
    use crate::ast::{
        lexer::{AccessModifier, Lexer},
        lexer_parser::{Block, Expression, Identifier, Parser, Type, VariableDeclaration},
        parsers::funcs::parse_fn,
        testing::Test,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_block_parser() {
        let tests: Vec<Test<&'static str, Block>> = vec![Test {
            name: "basic",
            input: "{
                const NAME: string
            }",
            expected: Block {
                expressions: vec![Expression::VariableDeclaration(Box::new(
                    VariableDeclaration {
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
                ))],
            },
        }];

        for test in tests {
            let mut lexer = Lexer::new(test.input.to_string());
            let tokens = lexer.consume_all_tokens();
            let mut parser = Parser::new(tokens);
            parser.consume();

            assert_eq!(parse_fn(&mut parser, false).unwrap(), test.expected);
        }
    }
}
