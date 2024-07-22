use super::{ASTNode, ASTType};

pub struct VariableDeclaration {}

impl ASTNode for VariableDeclaration {
    fn get_type(&self) -> super::ASTType {
        ASTType::VariableDeclaration
    }
}
