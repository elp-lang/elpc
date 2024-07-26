use crate::{ast::ASTNodeMember, lexer::token_stream::TokenStream};

pub struct ObjectDeclaration<'a> {
    token_stream: &'a TokenStream,
}

impl<'a> ASTNodeMember<'a> for ObjectDeclaration<'a> {
    fn new(token_stream: &'a TokenStream) -> Self
    where
        Self: Sized,
    {
        Self { token_stream }
    }
}
