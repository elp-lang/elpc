use crate::ast::ASTNodeMember;

pub trait TypeReferenceASTNode<'a>: ASTNodeMember<'a> {
    fn get_type_reference(&self) -> &dyn TypeReferenceASTNode<'a>;
}
