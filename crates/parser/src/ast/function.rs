use super::{ASTNode, ASTType};

pub struct FunctionDeclaration {}

impl ASTNode for FunctionDeclaration {
    fn get_type(&self) -> super::ASTType {
        ASTType::FunctionDeclaration
    }
}
