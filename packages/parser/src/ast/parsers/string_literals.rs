use crate::ast::{
    lexer::{Symbol, TokenType},
    lexer_parser::Parser,
    syntax_error::SyntaxError,
    Literal,
};

pub fn parse_string_literal(parser: &mut Parser, hint: Symbol) -> Result<Literal, SyntaxError> {
    match hint {
        Symbol::DoubleSpeechMark => {
            let mut escaped = false;
            let mut value: String = "".into();

            while let Some(token) = parser.consume() {
                match token.token_type {
                    TokenType::Symbol(Symbol::BackSlash) => {
                        if escaped {
                            value += token.value.as_str();
                            escaped = false;
                        } else {
                            escaped = true;
                        }
                        continue;
                    }
                    TokenType::Symbol(Symbol::DoubleSpeechMark) => {
                        if escaped {
                            value += token.value.as_str();
                            escaped = false;
                            continue;
                        } else {
                            break;
                        }
                    }
                    _ => {
                        value += token.value.as_str();
                    }
                }
            }

            Ok(Literal::String(value))
        }
        Symbol::SingleSpeechMark => todo!(),
        _ => Err(SyntaxError::UnexpectedTokenType(TokenType::Symbol(hint))),
    }
}

#[cfg(test)]
mod tests {
    use super::parse_string_literal;
    use crate::ast::{lexer::Lexer, lexer_parser::Parser, testing::Test, Literal};
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
                parse_string_literal(&mut parser, crate::ast::lexer::Symbol::DoubleSpeechMark)
                    .unwrap(),
                test.expected,
            );
        }
    }
}
