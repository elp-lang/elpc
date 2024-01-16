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
                println!("{:#?}", token);
                match token.token_type {
                    TokenType::Symbol(Symbol::DoubleSpeechMark) => {
                        if escaped {
                            value += token.value.as_str();
                            println!("{}", value);
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
        Symbol::OpenParen => todo!(),
        Symbol::CloseParen => todo!(),
        Symbol::Colon => todo!(),
        Symbol::OpenBlock => todo!(),
        Symbol::CloseBlock => todo!(),
        Symbol::Period => todo!(),
        Symbol::Comma => todo!(),
        Symbol::Other(_) => todo!(),
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
        let input = "\"test\"".to_string();
        let mut lexer = Lexer::new(input.clone());
        let tokens = lexer.consume_all_tokens();
        let mut parser = Parser::new(tokens);
        assert_eq!(
            parse_literal(&mut parser, crate::ast::lexer::Symbol::DoubleSpeechMark).unwrap(),
            Literal::String("test".into())
        );
    }
}
