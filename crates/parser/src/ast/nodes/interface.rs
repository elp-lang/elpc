use serde_json::Map;

use crate::{
    ast::ASTNodeMember,
    lexer::token_stream::TokenStream,
    parsing_error::ParsingError,
    tokens::{Keyword, Symbol, TokenType},
};

pub struct InterfaceMemberASTNode<'a> {
    name: String,
    r#type: &'a dyn ASTNodeMember<'a>,
}

pub struct InterfaceASTNode<'a> {
    name: Option<String>,
    token_stream: &'a TokenStream,

    pub members: Vec<&'a InterfaceMemberASTNode<'a>>,
}

impl<'a> InterfaceASTNode<'a> {
    fn parse_member(&mut self) -> Result<Option<InterfaceMemberASTNode>, Box<ParsingError>> {
        let mut member = InterfaceMemberASTNode {
            name: "".to_string(),
            r#type: TokenType::Keyword(Keyword::Nil),
        };

        loop {
            let token_option = self.token_stream.next();

            match token_option {
                Some(token) => match &token.token_type {
                    TokenType::EOF => {
                        return Err(Box::new(ParsingError::UnexpectedToken(token.clone())))
                    }
                    _ => return Err(Box::new(ParsingError::UnexpectedToken(token.clone()))),
                },
                None => return Ok(None),
            }
        }
    }
}

impl<'a> ASTNodeMember<'a> for InterfaceASTNode<'a> {
    fn new(token_stream: &'a TokenStream) -> Self
    where
        Self: Sized,
    {
        Self {
            name: None,
            token_stream,
            members: vec![],
        }
    }

    fn accepts(&'a self, token: &crate::tokens::Token) -> bool {
        matches!(token.token_type, TokenType::Keyword(Keyword::Interface))
    }

    fn produce(&'a mut self) -> Result<&Self, Box<ParsingError>>
    where
        Self: Sized,
    {
        let mut is_open = false;

        loop {
            let token_option = self.token_stream.next();

            match token_option {
                Some(token) => match &token.token_type {
                    TokenType::Ident(ident) => {
                        if !is_open && self.name.is_none() {
                            self.name = Some(ident.to_string());
                            continue;
                        } else if is_open {
                        } else {
                            return Err(Box::new(ParsingError::UnexpectedToken(token.clone())));
                        }
                    }
                    TokenType::Symbol(Symbol::OpenBlock) => {
                        is_open = true;

                        continue;
                    }
                    TokenType::Symbol(Symbol::CloseBlock) => {
                        if !is_open {
                            return Err(Box::new(ParsingError::UnexpectedToken(token.clone())));
                        }

                        break;
                    }
                    TokenType::EOF => {
                        return Err(Box::new(ParsingError::UnexpectedToken(token.clone())))
                    }
                    _ => return Err(Box::new(ParsingError::UnexpectedToken(token.clone()))),
                },
                None => break,
            }
        }

        Ok(self)
    }
}
