use crate::ast::{
    lexer::{Symbol, TokenType},
    lexer_parser::{Literal, Parser},
    syntax_error::SyntaxError,
};

pub struct LiteralHints {
    pub starts: Symbol,
    pub ends: Symbol,
}

pub fn parse_literal(parser: &mut Parser, hint: LiteralHints) -> Result<Literal, SyntaxError> {
    let mut escaped = false;
    let mut value: String = "".into();

    while let Some(token) = parser.consume() {
        if escaped {
            value += token.value.as_str();
            escaped = false;
            continue;
        } else {
            break;
        }
    }

    Ok(Literal::String(value))
}

#[cfg(test)]
mod tests {
    use super::parse_literal;
    use crate::ast::{
        lexer::{Lexer, Symbol},
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
            parse_literal(&mut parser, Symbol::DoubleSpeechMark).unwrap(),
            Literal::String("test".into())
        );
    }
}
