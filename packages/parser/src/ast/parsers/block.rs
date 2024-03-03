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
        testing::Test,
    };
    use pretty_assertions::assert_eq;

    use super::parse_block;

    #[test]
    fn test_block_parser() {
        let tests: Vec<Test<&'static str, Block>> = vec![];

        for test in tests {
            let mut lexer = Lexer::new(test.input.to_string());
            let tokens = lexer.consume_all_tokens();
            let mut parser = Parser::new(tokens);

            assert_eq!(parse_block(&mut parser).unwrap(), test.expected);
        }
    }
}
