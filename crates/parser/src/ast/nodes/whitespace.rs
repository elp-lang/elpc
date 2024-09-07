use std::marker;

use crate::ast::ASTNodeMember;
use crate::parsing_error::ParsingError;
use crate::token_stream::TokenStream;
use crate::tokens::{Token, TokenType, WhiteSpace};

#[derive(PartialEq, Debug)]
pub struct WhiteSpaceASTNode<'a> {
    r#type: WhiteSpace,
    _marker: marker::PhantomData<&'a ()>,
}

impl<'a> WhiteSpaceASTNode<'a> {}

impl<'a> ASTNodeMember<'a> for WhiteSpaceASTNode<'a> {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            r#type: WhiteSpace::Space,
            _marker: marker::PhantomData::default(),
        }
    }

    fn accepts(&self, token: &Token) -> bool {
        matches!(token.token_type, TokenType::WhiteSpace(..))
    }

    fn produce(token_stream: &mut TokenStream) -> Result<Self, Box<ParsingError>>
    where
        Self: Sized,
    {
        let mut out = Self::new();

        for token in token_stream.iter() {
            match &token.token_type {
                TokenType::WhiteSpace(ws) => {
                    out.r#type = ws.clone();
                }
                _ => break,
            }
        }

        Ok(out)
    }
}
