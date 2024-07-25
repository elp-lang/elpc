use super::{ASTNode, ASTType};

pub struct ObjectDeclaration<'a> {
    children: Vec<&'a dyn ASTNode<'a>>,
}

impl<'a> ASTNode<'a> for ObjectDeclaration<'a> {
    fn get_type(&self) -> super::ASTType {
        ASTType::ObjectDeclaration
    }

    fn get_children(&'a self) -> &'a Vec<&'a dyn ASTNode<'a>> {
        &self.children
    }
}
