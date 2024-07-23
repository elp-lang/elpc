#[cfg(test)]
mod tests {
    use elp_parser::{
        lexer::token_stream::TokenStream,
        span::Span,
        tokens::{Source, Token, TokenType},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_empty_state() {
        let stream = TokenStream::new(vec![]);

        assert_eq!(stream.len(), 0);
        assert_eq!(stream.is_empty(), true);
        assert_eq!(stream.next(), None);
    }

    #[test]
    fn test_token_stream() {
        let soi = Token {
            token_type: TokenType::SOI,
            source: Source::default(),
        };
        let eof = Token {
            token_type: TokenType::EOF,
            source: Source::default(),
        };
        let mut stream = TokenStream::new(vec![soi.clone(), eof.clone()]);

        assert_eq!(stream.len(), 2);
        assert_eq!(stream.is_empty(), false);
        assert_eq!(stream.next(), Some(&soi));
        stream.consume();
        assert_eq!(stream.next(), Some(&eof));
        stream.consume();
        assert_eq!(stream.next(), None);
    }
}
