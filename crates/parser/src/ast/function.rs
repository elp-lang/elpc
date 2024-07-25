use super::{ASTNode, ASTType};

pub struct FunctionDeclaration<'a> {
    children: Vec<&'a dyn ASTNode<'a>>,
}

impl<'a> ASTNode<'a> for FunctionDeclaration<'a> {
    fn get_type(&self) -> super::ASTType {
        ASTType::FunctionDeclaration
    }

    fn get_children(&'a self) -> &'a Vec<&'a dyn ASTNode<'a>> {
        &self.children
    }
}
