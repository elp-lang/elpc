use crate::{
    ast::ASTNodeMember,
    lexer::token_stream::TokenStream,
    parsing_error::ParsingError,
    tokens::{Keyword, Symbol, Token, TokenType},
};
use crate::ast::nodes::built_in_type::BuiltInTypeASTNode;
use crate::ast::nodes::r#type::TypeReferenceASTNode;

#[derive(PartialEq, Debug)]
pub struct InterfaceMemberASTNode<'a> {
    name: Option<String>,
    r#type: &'a dyn TypeReferenceASTNode<'a>,
}

#[derive(PartialEq, Debug)]
pub struct InterfaceASTNode<'a> {
    name: Option<String>,
    pub members: Vec<&'a InterfaceMemberASTNode<'a>>,
}

impl<'a> InterfaceASTNode<'a> {
    fn parse_member(&mut self, token_stream: &'a mut TokenStream) -> Result<Option<InterfaceMemberASTNode>, Box<ParsingError>> {
        let mut member = InterfaceMemberASTNode {
            name: None,
            r#type: &BuiltInTypeASTNode::new()
        };

        loop {
            let token_option = &token_stream.next();

            match token_option {
                Some(token) => match &token.token_type {
                    TokenType::Ident(name) => {
                        if name.is_empty() {
                            return Err(Box::new(ParsingError::ExpectedToken(Token {
                                token_type: TokenType::Ident("named member".into()),
                                ..Default::default()
                            })));
                        }

                        member.name = Some(name.into());
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
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            name: None,
            members: vec![],
        }
    }

    fn accepts(&'a self, token: &Token) -> bool {
        matches!(token.token_type, TokenType::Keyword(Keyword::Interface))
    }

    fn produce(&'a mut self, token_stream: &'a mut TokenStream) -> Result<&Self, Box<ParsingError>>
    where
        Self: Sized,
    {
        let mut is_open = false;

        loop {
            let token_option = token_stream.next();

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
    use crate::ast::nodes::built_in_type::BuiltInTypeASTNode;
    use super::{InterfaceASTNode, InterfaceMemberASTNode};

    #[test]
    fn test_interface_member_parsing() {
        let mut lexer = Lexer::new_str("name: String");
        let tokens = lexer.consume_all_tokens().unwrap();
        let mut token_stream = TokenStream::new(tokens);

        let mut interface_parser = InterfaceASTNode::new();

        let member = interface_parser.parse_member(&mut token_stream);

        assert_eq!(
            member.unwrap().unwrap(),
            InterfaceMemberASTNode {
            name: Some("name".into()),
            r#type: &BuiltInTypeASTNode::new()
        })
    }
}
