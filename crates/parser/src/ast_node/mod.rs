use async_trait::async_trait;

use crate::tokens::Token;

#[async_trait]
pub trait AstNode {
    async fn get_children(&self) -> Vec<&dyn AstNode>;
    async fn parse_token(&self, token: &Token) -> Vec<&dyn AstNode>;
}
