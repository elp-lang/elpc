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
    fn get_children(&self) -> &'a dyn ASTNode;
}

pub struct ASTTree<'a> {
    token_stream: &'a TokenStream,
    children: Vec<&'a dyn ASTNode<'a>>,
}

impl<'a> ASTNode<'a> for ASTTree<'a> {
    fn get_type(&self) -> ASTType {
        ASTType::Root
    }

    fn get_children(&self) -> &'a dyn ASTNode {
        todo!()
    }
}

impl<'a> ASTTree<'a> {
    pub fn new(token_stream: &'a TokenStream) -> Self {
        ASTTree {
            token_stream,
            children: vec![],
        }
    }

    pub fn parse_tokens(&mut self) -> Result<Vec<&'a dyn ASTNode>, ParsingError> {
        let nodes: Vec<&'a dyn ASTNode<'a>> = vec![];

        while let Some(node) = self.token_stream.next() {
            match node.token_type {
                TokenType::SOI => continue,
                TokenType::AccessModifier(_) => todo!(),
                TokenType::BooleanLiteral(_) => todo!(),
                TokenType::CommentBlock(_) => todo!(),
                TokenType::CommentLine(_) => todo!(),
                TokenType::Component(_) => todo!(),
                TokenType::FloatLiteral(_) => todo!(),
                TokenType::Ident(_) => todo!(),
                TokenType::Identifier(_) => todo!(),
                TokenType::IntegerLiteral(_) => todo!(),
                TokenType::Keyword(_) => todo!(),
                TokenType::MacroCall(_) => todo!(),
                TokenType::Nil => todo!(),
                TokenType::StringLiteral(_) => todo!(),
                TokenType::Symbol(_) => todo!(),
                TokenType::WhiteSpace(_) => todo!(),
                TokenType::EOF => break,
            };
        }

        Ok(nodes)
    }
}
