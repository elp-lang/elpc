use std::slice::Iter;

use crate::tokens::Token;

pub struct TokenStream<'a> {
    tokens: &'a Vec<Token>,
    position: usize,
}

impl<'a> TokenStream<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn token(&self) -> Option<&Token> {
        if self.position < self.tokens.len() {
            self.tokens.get(self.position)
        } else {
            None
        }
    }

    pub fn consume(&mut self) -> usize {
        self.position += 1;
        self.position
    }

    pub fn iter(&self) -> Iter<Token> {
        self.tokens[self.position..].iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        token_stream::TokenStream,
        tokens::{Source, Symbol, Token, TokenType},
    };

    #[test]
    fn test_token_stream() {
        let tokens = vec![
            Token {
                token_type: TokenType::Ident("public".into()),
                ..Default::default()
            },
            Token {
                token_type: TokenType::Ident("var".into()),
                ..Default::default()
            },
            Token {
                token_type: TokenType::Ident("myVar".into()),
                ..Default::default()
            },
            Token {
                token_type: TokenType::Symbol(Symbol::SingleEqual),
                ..Default::default()
            },
            Token {
                token_type: TokenType::FloatLiteral(1.0),
                ..Default::default()
            },
            Token {
                token_type: TokenType::Symbol(Symbol::Semicolon),
                ..Default::default()
            },
        ];

        let mut token_stream = TokenStream::new(&tokens);

        assert_eq!(
            token_stream.token(),
            Some(&Token {
                token_type: TokenType::Ident("public".into()),
                source: Source::default(),
            })
        );

        assert_eq!(token_stream.consume(), 1);
    }
}
