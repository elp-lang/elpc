use crate::ast::{
    lexer::{Symbol, TokenType},
    lexer_parser::{Literal, Parser},
    syntax_error::SyntaxError,
};

pub fn parse_literal(parser: &mut Parser, hint: Symbol) -> Result<Literal, SyntaxError> {
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

            return Ok(Literal::String(value));
        }
        Symbol::SingleSpeechMark => todo!(),
        _ => return Err(SyntaxError::UnexpectedTokenType(TokenType::Symbol(hint))),
    };
}

#[cfg(test)]
mod tests {
    use super::parse_literal;
    use crate::ast::{
        lexer::Lexer,
        lexer_parser::{Literal, Parser},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_string_literals() {
        let input = "\\\"test\\\"".to_string();
        let mut lexer = Lexer::new(input.clone());
        let tokens = lexer.consume_all_tokens();
        let mut parser = Parser::new(tokens);
        assert_eq!(
            parse_literal(&mut parser, crate::ast::lexer::Symbol::DoubleSpeechMark).unwrap(),
            Literal::String("\"test\"".into())
        );
    }
}
