use crate::ast::ASTNodeMember;
use crate::ast::nodes::r#type::IntrinsicTypes::Nil;
use crate::lexer::token_stream::TokenStream;
use crate::parsing_error::ParsingError;
use crate::tokens::{Keyword, Token, TokenType};

use super::r#type::Types;

#[derive(PartialEq, Debug)]
pub struct VariableASTNode<'a> {
    r#type: &'a Types,
}

impl<'a> ASTNodeMember<'a> for VariableASTNode<'a> {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            r#type: &Types::Intrinsic(Nil),
        }
    }

    fn accepts(&self, token: &Token) -> bool {
        matches!(
            token.token_type,
            TokenType::Keyword(Keyword::Const) | TokenType::Keyword(Keyword::Var)
        )
    }

    fn produce(_token_stream: &'a mut TokenStream) -> Result<Self, Box<ParsingError>>
    where
        Self: Sized,
    {
        todo!()
    }
}
