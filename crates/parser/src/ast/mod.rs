use crate::{lexer::token_stream::TokenStream, parsing_error::ParsingError, tokens::Token};

use self::nodes::interface::InterfaceASTNode;

pub mod nodes;

// ASTNode is an expression parsed from the source file.
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

    // accepts will receive a token and it should decide whether or not
    // it will continue to consume the token stream and parse or skip.
    // returning true will invoke the produce function to produce a new ASTNodeMember.
    fn accepts(&'a self, token: &Token) -> bool;

    // produce should return a new ASTNodeMember that represents the current
    // user's intention and advance the token_stream to the next token for
    // the next ASTNodeMember to consume.
    // TODO work out a better parsing error structure as having to store on the heap might lead to OOM error if the token is huge (a large interface, recursive type, etc.)
    fn produce(&'a mut self) -> Result<&Self, Box<ParsingError>>
    where
        Self: Sized;
}
