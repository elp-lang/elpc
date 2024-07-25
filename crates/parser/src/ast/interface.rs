use super::{ASTNode, ASTType};

pub struct InterfaceDeclaration<'a> {
    children: Vec<&'a dyn ASTNode<'a>>,
}

impl<'a> ASTNode<'a> for InterfaceDeclaration<'a> {
    fn get_type(&self) -> super::ASTType {
        ASTType::InterfaceDeclaration
    }

    fn get_children(&'a self) -> &'a Vec<&'a dyn ASTNode<'a>> {
        &self.children
    }
}
