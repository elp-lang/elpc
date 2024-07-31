use crate::{
    ast::ASTNodeMember,
    lexer::token_stream::TokenStream,
    parsing_error::ParsingError,
    tokens::{Token, TokenType},
};

pub struct NilASTNode<'a> {
    token_stream: &'a mut TokenStream,
}

impl<'a> Default for NilASTNode<'a> {
    fn default() -> Self {
        let mut token_stream = TokenStream::new(vec![]);
        Self {
            token_stream: &token_stream,
        }
    }
}

impl<'a> ASTNodeMember<'a> for NilASTNode<'a> {
    fn new(token_stream: &'a mut TokenStream) -> Self
    where
        Self: Sized,
    {
        Self { token_stream }
    }

    fn accepts(&'a self, token: &crate::tokens::Token) -> bool {
        matches!(token.token_type, TokenType::Nil)
    }

    fn produce(&'a mut self) -> Result<&Self, Box<crate::parsing_error::ParsingError>>
    where
        Self: Sized,
    {
        match self.token_stream.next() {
            Some(token) => {
                if !matches!(token.token_type, TokenType::Nil) {
                    Err(Box::new(ParsingError::UnexpectedToken(token.clone())))
                } else {
                    self.token_stream.consume();
                    Ok(self)
                }
            }
            None => Err(Box::new(ParsingError::ExpectedToken(Token {
                token_type: TokenType::Nil,
                ..Default::default()
            }))),
        }
    }
}
