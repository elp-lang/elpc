use std::sync::Arc;

use async_trait::async_trait;

use crate::lexer::lexer::{Symbol, Token, TokenType};
use crate::lexer::parsing_error::ParsingError;
use crate::parserv2::node::{AstNode, Literal};
use crate::parserv2::visitor::TokenVisitor;

// Examples of string literals are:
// - "Hello, world!"
// - ""
// - "Hello, ${world}!"
// - """Hello,
// world!"""
#[derive(Clone, Default, Debug)]
pub struct StringLiteralsVisitor {
    string_has_started: bool,
    string_has_ended: bool,
    next_char_is_escaped: bool,
    string_content: String,
}

#[async_trait]
impl TokenVisitor for StringLiteralsVisitor {
    fn accept(&mut self, token: &Token) -> bool {
        !matches!(token.token_type, TokenType::SOI | TokenType::EOF)
    }

    async fn parse(&mut self, tokens: Vec<Token>) -> Result<Option<Arc<AstNode>>, ParsingError> {
        for token in tokens {
            match token.token_type {
                TokenType::Symbol(Symbol::BackSlash) => {
                    if self.next_char_is_escaped {
                        self.string_content += token.value.as_str();
                        self.next_char_is_escaped = false;
                    }
                }
                TokenType::Symbol(Symbol::DoubleSpeechMark) => {
                    if !self.string_has_started && self.next_char_is_escaped {
                        return Err(ParsingError::SyntaxError(
                            "Unterminated string literal".into(),
                            token.source.clone(),
                        ));
                    }
                    if self.string_has_started && !self.next_char_is_escaped {
                        self.string_content += token.value.as_str();
                        self.string_has_ended = true;
                        break;
                    }
                }
                _ => {
                    self.string_content += token.value.as_str();
                }
            }
        }

        Ok(Some(Arc::new(AstNode::LiteralString(Literal::String(
            self.string_content.clone(),
        )))))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::lexer::lexer::{SourceFile, Symbol, Token, TokenType, Whitespace};
    use crate::parserv2::node::{AstNode, Literal};
    use crate::parserv2::visitor::TokenVisitor;

    #[tokio::test]
    async fn parse_token_accepts() {
        let input = vec![
            Token {
                token_type: TokenType::Symbol(Symbol::DoubleSpeechMark),
                value: "\"".into(),
                source: Arc::new(SourceFile {
                    name: "".to_string(),
                    path: "".to_string(),
                    line: 0,
                    span: (0, 0),
                }),
            },
            Token {
                token_type: TokenType::Ident("hello".into()),
                value: "hello".into(),
                source: Arc::new(SourceFile {
                    name: "".to_string(),
                    path: "".to_string(),
                    line: 0,
                    span: (1, 5),
                }),
            },
            Token {
                token_type: TokenType::Whitespace(Whitespace::Other(" ".into())),
                value: " ".into(),
                source: Arc::new(SourceFile {
                    name: "".to_string(),
                    path: "".to_string(),
                    line: 0,
                    span: (6, 6),
                }),
            },
            Token {
                token_type: TokenType::Ident("world".into()),
                value: "world".into(),
                source: Arc::new(SourceFile {
                    name: "".to_string(),
                    path: "".to_string(),
                    line: 0,
                    span: (7, 11),
                }),
            },
            Token {
                token_type: TokenType::Symbol(Symbol::DoubleSpeechMark),
                value: "\"".into(),
                source: Arc::new(SourceFile {
                    name: "".to_string(),
                    path: "".to_string(),
                    line: 0,
                    span: (12, 12),
                }),
            },
        ];
        let expected = AstNode::LiteralString(Literal::String("hello world".into()));

        let mut visitor = super::StringLiteralsVisitor {
            string_has_started: false,
            string_has_ended: false,
            next_char_is_escaped: false,
            string_content: "".to_string(),
        };

        for token in &input {
            assert!(visitor.accept(token));
        }

        assert_eq!(
            visitor.parse(input).await.unwrap(),
            Some(Arc::new(expected))
        );
    }
}
