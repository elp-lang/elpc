use crate::lexer::token_stream::TokenStream;

use super::{ASTNode, ASTType};

pub struct ObjectDeclaration<'a> {
    token_stream: &'a TokenStream,
    children: Vec<&'a dyn ASTNode<'a>>,
}

impl<'a> ASTNode<'a> for ObjectDeclaration<'a> {
    fn get_type(&self) -> super::ASTType {
        ASTType::ObjectDeclaration
    }

    fn get_children(&'a self) -> &'a Vec<&'a dyn ASTNode<'a>> {
        &self.children
    }

    fn new(token_stream: &'a TokenStream) -> Self
    where
        Self: Sized,
    {
        Self {
            token_stream,
            children: vec![],
        }
    }
}
