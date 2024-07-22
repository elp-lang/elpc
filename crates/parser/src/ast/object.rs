use super::{ASTNode, ASTType};

pub struct ObjectDeclaration {}

impl ASTNode for ObjectDeclaration {
    fn get_type(&self) -> super::ASTType {
        ASTType::ObjectDeclaration
    }
}
