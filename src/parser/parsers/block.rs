use crate::parser::lexer::{Symbol, TokenType};
use crate::parser::lexer_parser::{Block, Parser};
use crate::parser::syntax_error::SyntaxError;
use super::expression::parse_expression;

pub fn parse_block(parser: &mut Parser) -> Result<Block, SyntaxError> {
    let mut block_declaration = Block {
        expressions: vec![],
    };

    while let Some(token) = parser.consume() {
        match &token.token_type {
            // When the block closes, we're done.
            TokenType::Symbol(Symbol::CloseBlock) => break,
            // Try to parse everything as an expression.
            _ => match parse_expression(parser) {
                Ok(expr) => block_declaration.expressions.push(expr),
                Err(err) => return Err(err),
            },
        }
    }

    Ok(block_declaration)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use crate::parser::lexer::{AccessModifier, Lexer};
    use crate::parser::lexer_parser::{Block, Expression, Identifier, Literal, Parser, Type, VariableDeclaration};
    use crate::parser::testing::Test;

    use super::parse_block;

    #[test]
    fn test_block_parser() {
        let tests: Vec<Test<&'static str, Block>> = vec![
            Test {
                name: "empty block has no expressions",
                input: "{}",
                expected: Block {
                    expressions: vec![],
                },
            },
            Test {
                name: "parses some expressions that print hello world",
                input: "{
                    const target = \"world\"
                    print(\"hello {target}\")
                }",
                expected: Block {
                    expressions: vec![Expression::VariableDeclaration(Box::new(
                        VariableDeclaration {
                            ident: Identifier {
                                immutable: true,
                                access_modifier: AccessModifier::Pub,
                                name: "target".into(),
                            },
                            r#type: Type::Void,
                            value: Some(Expression::Literal(Literal::String("world".into()))),
                        },
                    ))],
                },
            },
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.input.to_string());
            let tokens = lexer.consume_all_tokens();
            let mut parser = Parser::new(tokens);
            // It's assumed we have already consumed the { to reach this point so consume the first
            // character of the test input which should be the opening { of the block.
            parser.consume();

            assert_eq!(parse_block(&mut parser).unwrap(), test.expected);
        }
    }
}
