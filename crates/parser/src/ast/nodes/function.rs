use crate::{ast::ASTNodeMember, lexer::token_stream::TokenStream};

pub struct FunctionDeclaration<'a> {
    token_stream: &'a TokenStream,
}

impl<'a> ASTNodeMember<'a> for FunctionDeclaration<'a> {
    fn new(token_stream: &'a TokenStream) -> Self
    where
        Self: Sized,
    {
        Self { token_stream }
    }
}
