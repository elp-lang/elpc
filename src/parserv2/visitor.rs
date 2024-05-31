use std::sync::Arc;

use async_trait::async_trait;

use crate::lexer::lexer::Token;
use crate::lexer::parsing_error::ParsingError;
use crate::parserv2::node::AstNode;

// A TokenVisitor is an object that receives tokens and returns
// an AstNode representing what this expression is equal to.
#[async_trait]
pub trait TokenVisitor: Send + Sync + 'static {
    // Do we accept this token? If so we do not process it here,
    // and simply return true. Processing should be done in the
    // `visit` method.
    // The way this works is that the `accept` method returns
    // true and the token is pushed into a vec of tokens. Once
    // this function returns false, if it ever returns true,
    // we then call the parse method and give it all the tokens
    // it previously accepted to produce an AstNode.
    fn accept(&mut self, token: &Token) -> bool;

    // Take the tokens we previously accepted and return an AstNode.
    async fn parse(&mut self, tokens: Vec<Token>) -> Result<Option<Arc<AstNode>>, ParsingError>;
}
