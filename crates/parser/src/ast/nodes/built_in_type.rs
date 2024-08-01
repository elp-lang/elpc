use crate::ast::ASTNodeMember;
use crate::ast::nodes::r#type::IntrinsicTypes::Nil;
use super::r#type::{TypeReferenceASTNode, Types};
use crate::lexer::token_stream::TokenStream;
use crate::parsing_error::ParsingError;
use crate::tokens::{Token, TokenType};


#[derive(PartialEq, Debug)]
pub struct BuiltInTypeASTNode<'a> {
    r#type: &'a Types,
}

impl<'a> ASTNodeMember<'a> for BuiltInTypeASTNode<'a> {
    fn new() -> Self
    where
        Self: Sized
    {
        Self {
            r#type: &Types::Intrinsic(Nil),
        }
    }

    // accepts only really accepts Idents, and we'll parse it out as a type contextually.
    // An ident that comes after `var` or `const` is going to be a name of something, an ident
    // like `name: value` where name and value are both idents, the first is the name and
    // the second is the type. However, there's not many reasons I can think of right now
    // to call this directly as this kind of node (BuiltInTypeASTNode) and really, even the
    // super (TypeReferenceASTNode) will be called by other combinators.
    fn accepts(&self, token: &Token) -> bool {
        matches!(token.token_type, TokenType::Ident(..))
    }

    fn produce(&'a mut self, token_stream: &'a mut TokenStream) -> Result<&Self, Box<ParsingError>>

    where
        Self: Sized
    {
        todo!()
    }
}

impl<'a> TypeReferenceASTNode<'a> for BuiltInTypeASTNode<'a> {
    fn get_type_reference(&self) -> &'a Types {
        self.r#type
    }
}
