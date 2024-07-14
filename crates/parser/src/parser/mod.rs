use async_trait::async_trait;

use crate::{ast_node::AstNode, tokens::Token};

#[async_trait]
pub trait Parser {
    fn new(tokens: Vec<Token>) -> Self;
    async fn parse(&mut self) -> Vec<&dyn AstNode>;
}

pub struct ElpParser {
    tokens: Vec<Token>,
}

#[async_trait]
impl Parser for ElpParser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    async fn parse(&mut self) -> Vec<&dyn AstNode> {
        let mut ast_nodes: Vec<&dyn AstNode> = [].into();

        for t in self.tokens.clone().into_iter() {
            match t.token_type {
                crate::tokens::TokenType::SOI => todo!(),
                crate::tokens::TokenType::AccessModifier(_) => todo!(),
                crate::tokens::TokenType::BooleanLiteral(_) => todo!(),
                crate::tokens::TokenType::FloatLiteral(_) => todo!(),
                crate::tokens::TokenType::Ident(_) => todo!(),
                crate::tokens::TokenType::Identifier(_) => todo!(),
                crate::tokens::TokenType::IntegerLiteral(_) => todo!(),
                crate::tokens::TokenType::Keyword(_) => todo!(),
                crate::tokens::TokenType::MacroCall(_) => todo!(),
                crate::tokens::TokenType::Nil => todo!(),
                crate::tokens::TokenType::StringLiteral(_) => todo!(),
                crate::tokens::TokenType::Symbol(_) => todo!(),
                crate::tokens::TokenType::WhiteSpace(_) => todo!(),
                crate::tokens::TokenType::EOF => todo!(),
            }
        }

        ast_nodes
    }
}
