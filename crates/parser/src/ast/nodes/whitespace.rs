use crate::ast::ASTNodeMember;
use crate::lexer::token_stream::TokenStream;
use crate::parsing_error::ParsingError;
use crate::tokens::{Source, Token, TokenType, WhiteSpace};

#[derive(PartialEq, Debug)]
pub struct WhiteSpaceASTNode<'a> {
    r#type: &'a WhiteSpace,
}

impl<'a> ASTNodeMember<'a> for WhiteSpaceASTNode<'a> {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            r#type: &WhiteSpace::Space,
        }
    }

    fn accepts(&self, token: &Token) -> bool {
        matches!(token.token_type, TokenType::WhiteSpace(..))
    }

    fn produce(token_stream: &'a mut TokenStream) -> Result<Self, Box<ParsingError>>
    where
        Self: Sized,
    {
        let mut out = Self::new();

        loop {
            match token_stream.next() {
                Some(token) => match &token.token_type {
                    TokenType::WhiteSpace(ws) => {
                        out.r#type = ws;
                        token_stream.consume();
                        continue;
                    }
                    _ => break,
                },
                None => {
                    return Err(Box::new(ParsingError::ExpectedToken(Token {
                        token_type: TokenType::Ident("any".into()),
                        source: Source::default(),
                    })))
                }
            }
        }

        Ok(out)
    }
}
