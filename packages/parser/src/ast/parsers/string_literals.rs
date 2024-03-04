use crate::ast::{
    lexer::{Symbol, TokenType},
    lexer_parser::{Literal, Parser},
    syntax_error::SyntaxError,
};

pub fn parse_string_literal(parser: &mut Parser, hint: Symbol) -> Result<Literal, SyntaxError> {
    let mut escaped = false;
    let mut value: String = "".into();

    while let Some(token) = parser.consume() {
        if token.token_type == TokenType::Symbol(Symbol::BackSlash) {
            if escaped {
                value += token.value.as_str();
                escaped = false;
            } else {
                escaped = true;
            }
            continue;
        }

        if token.token_type == TokenType::Symbol(hint.clone()) {
            if escaped {
                value += token.value.as_str();
                escaped = false;
                continue;
            }

            break;
        }

        value += token.value.as_str();
    }

    Ok(Literal::String(value))
}

#[cfg(test)]
mod tests {
    use super::parse_string_literal;
    use crate::ast::{
        lexer::{Lexer, Symbol},
        lexer_parser::{Literal, Parser},
        testing::Test,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_string_literals() {
        let tests: Vec<Test<&'static str, Literal>> = vec![Test {
            name: "escaped string",
            input: "\\\"test\\\"",
            expected: Literal::String("\"test\"".into()),
        }];

        for test in tests {
            let mut lexer = Lexer::new(test.input.to_string());
            let tokens = lexer.consume_all_tokens();
            let mut parser = Parser::new(tokens);
            assert_eq!(
                parse_string_literal(&mut parser, Symbol::DoubleSpeechMark).unwrap(),
                test.expected,
            );
        }
    }
}
