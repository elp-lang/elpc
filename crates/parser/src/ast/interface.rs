use super::{ASTNode, ASTType};

pub struct InterfaceDeclaration {}

impl ASTNode for InterfaceDeclaration {
    fn get_type(&self) -> super::ASTType {
        ASTType::InterfaceDeclaration
    }
}
