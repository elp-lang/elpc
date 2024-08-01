use super::r#type::TypeReferenceASTNode;
use crate::lexer::token_stream::TokenStream;

pub struct BuiltInTypeASTNode<'a> {
    token_stream: &'a mut TokenStream,
}

impl<'a> TypeReferenceASTNode<'a> for BuiltInTypeASTNode<'a> {
    fn get_type_reference(&self) -> &dyn TypeReferenceASTNode<'a> {
        self
    }
}
