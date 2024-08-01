use crate::ast::ASTNodeMember;

#[derive(PartialEq, Debug)]
pub enum IntrinsicTypes {
    Bool,
    String,
    Int,
    Float,
    Char,
    Byte,
    Function,
    Interface,
    Nil,
}

#[derive(PartialEq, Debug)]
pub enum Types {
    Intrinsic(IntrinsicTypes),
    User(String)
}
pub trait TypeReferenceASTNode<'a>: ASTNodeMember<'a> {
    fn get_type_reference(&self) -> &'a Types;
}
