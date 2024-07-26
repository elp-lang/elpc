use crate::lexer::token_stream::TokenStream;

use self::nodes::interface::InterfaceASTNode;

pub mod nodes;

pub enum ASTNode<'a> {
    // Represents the root of the AST tree.
    Root,

    // Type system.
    InterfaceDeclaration(InterfaceASTNode<'a>),
    TypeAlias,

    // Statements.
    VariableDeclaration,
    FunctionDeclaration,
    ObjectDeclaration,
    ComponentDeclaration,
}

pub trait ASTNodeMember<'a> {
    fn new(token_stream: &'a TokenStream) -> Self
    where
        Self: Sized;
}
