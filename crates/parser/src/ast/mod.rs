use std::{cell::Ref, ops::Deref};

use crate::{
    lexer::token_stream::TokenStream,
    parsing_error::ParsingError,
    tokens::{Token, TokenType},
};

pub mod function;
pub mod interface;
pub mod object;
pub mod variable;

pub enum ASTType {
    // Represents the root of the AST tree.
    Root,

    // Type system.
    InterfaceDeclaration,
    TypeAlias,

    // Statements.
    VariableDeclaration,
    FunctionDeclaration,
    ObjectDeclaration,
    ComponentDeclaration,
}

pub trait ASTNode<'a> {
    fn get_type(&self) -> ASTType;
    fn get_children(&'a self) -> &'a Vec<&'a dyn ASTNode<'a>>;
    fn new(token_stream: &'a TokenStream) -> Self
    where
        Self: Sized;
}

#[derive(Clone)]
pub struct ASTTree<'a> {
    token_stream: &'a TokenStream,
    children: Vec<&'a dyn ASTNode<'a>>,
}

impl<'a> ASTNode<'a> for ASTTree<'a> {
    fn get_type(&self) -> ASTType {
        ASTType::Root
    }

    fn get_children(&'a self) -> &'a Vec<&'a dyn ASTNode<'a>> {
        &self.children
    }

    fn new(token_stream: &'a TokenStream) -> Self
    where
        Self: Sized,
    {
        ASTTree {
            token_stream,
            children: vec![],
        }
    }
}

impl<'a> ASTTree<'a> {
    pub fn parse_tokens(&mut self) -> Result<Self, ParsingError> {
        while let Some(node) = self.token_stream.next() {
            match node.token_type {
                TokenType::SOI => continue,
                TokenType::EOF => break,
                _ => {
                    return Err(ParsingError::SyntaxError(
                        "Unexpected token".into(),
                        node.source.clone(),
                    ));
                }
            };
        }

        Ok(self.clone())
    }
}
