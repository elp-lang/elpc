use async_trait::async_trait;

#[async_trait]
pub trait AstNode {
    async fn get_children(&self) -> Vec<&dyn AstNode>;
}
