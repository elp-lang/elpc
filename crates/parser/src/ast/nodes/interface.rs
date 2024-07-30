use crate::{
    ast::ASTNodeMember,
    lexer::token_stream::TokenStream,
    parsing_error::ParsingError,
    tokens::{Keyword, TokenType},
};

pub struct InterfaceASTNode<'a> {
    name: Option<String>,
    token_stream: &'a TokenStream,
}

impl<'a> ASTNodeMember<'a> for InterfaceASTNode<'a> {
    fn new(token_stream: &'a TokenStream) -> Self
    where
        Self: Sized,
    {
        Self {
            name: None,
            token_stream,
        }
    }

    fn accepts(&'a self, token: &crate::tokens::Token) -> bool {
        matches!(token.token_type, TokenType::Keyword(Keyword::Interface))
    }

    fn produce(&'a mut self) -> Result<&Self, Box<ParsingError>>
    where
        Self: Sized,
    {
        loop {
            let token_option = self.token_stream.next();

            match token_option {
                Some(token) => match &token.token_type {
                    TokenType::Ident(ident) => {
                        if self.name.is_none() {
                            self.name = Some(ident.to_string());
                            continue;
                        } else {
                            return Err(Box::new(ParsingError::UnexpectedToken(
                                token.clone(),
                                token.source.clone(),
                            )));
                        }
                    }
                    TokenType::EOF => {
                        return Err(Box::new(ParsingError::UnexpectedToken(
                            token.clone(),
                            token.source.clone(),
                        )))
                    }
                    _ => {
                        return Err(Box::new(ParsingError::UnexpectedToken(
                            token.clone(),
                            token.source.clone(),
                        )))
                    }
                },
                None => break,
            }
        }

        Ok(self)
    }
}
