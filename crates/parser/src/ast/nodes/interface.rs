use crate::{
    ast::ASTNodeMember,
    lexer::token_stream::TokenStream,
    parsing_error::ParsingError,
    tokens::{Keyword, Symbol, Token, TokenType},
};

use super::nil::NilASTNode;

pub struct InterfaceMemberASTNode<'a> {
    name: String,
    r#type: &'a dyn ASTNodeMember<'a>,
}

pub struct InterfaceASTNode<'a> {
    name: Option<String>,
    token_stream: &'a mut TokenStream,

    pub members: Vec<&'a InterfaceMemberASTNode<'a>>,
}

impl<'a> InterfaceASTNode<'a> {
    fn parse_member(&mut self) -> Result<Option<InterfaceMemberASTNode>, Box<ParsingError>> {
        let mut member = InterfaceMemberASTNode {
            name: "".to_string(),
            r#type: &NilASTNode::new(self.token_stream),
        };

        loop {
            let token_option = &self.token_stream.next();

            match token_option {
                Some(token) => match &token.token_type {
                    TokenType::Ident(name) => {
                        if name.is_empty() {
                            return Err(Box::new(ParsingError::ExpectedToken(Token {
                                token_type: TokenType::Ident("named member".into()),
                                ..Default::default()
                            })));
                        }

                        member.name = name.into();
                    }
                    TokenType::EOF => {
                        return Err(Box::new(ParsingError::UnexpectedToken(
                            token.clone().to_owned(),
                        )))
                    }
                    _ => {
                        return Err(Box::new(ParsingError::UnexpectedToken(
                            token.clone().to_owned(),
                        )))
                    }
                },
                None => return Ok(None),
            }
        }
    }
}

impl<'a> ASTNodeMember<'a> for InterfaceASTNode<'a> {
    fn new(token_stream: &'a mut TokenStream) -> Self
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

#[cfg(test)]
mod tests {
    use crate::{
        ast::ASTNodeMember,
        lexer::{token_stream::TokenStream, Lexer},
    };

    use super::InterfaceASTNode;

    #[test]
    fn test_interface_member_parsing() {
        let mut lexer = Lexer::new_str("name: String");
        let tokens = lexer.consume_all_tokens().unwrap();
        let mut token_stream = TokenStream::new(tokens);

        let mut interface_parser = InterfaceASTNode::new(&mut token_stream);

        let member = interface_parser.parse_member();

        assert_eq!(
            member,
            Ok(Some(InterfaceMemberASTNode {
            name: "name".into(),
            r#type: 
        }))
    }
}
