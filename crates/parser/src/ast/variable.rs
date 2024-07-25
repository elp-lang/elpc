use super::{ASTNode, ASTType};

pub struct VariableDeclaration<'a> {
    children: Vec<&'a dyn ASTNode<'a>>,
}

impl<'a> ASTNode<'a> for VariableDeclaration<'a> {
    fn get_type(&self) -> super::ASTType {
        ASTType::VariableDeclaration
    }

    fn get_children(&'a self) -> &'a Vec<&'a dyn ASTNode<'a>> {
        &self.children
    }
}
