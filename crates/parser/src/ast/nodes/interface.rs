use crate::{
    ast::ASTNodeMember,
    lexer::token_stream::TokenStream,
    parsing_error::ParsingError,
    tokens::{Keyword, Symbol, Token, TokenType},
};
use crate::ast::nodes::r#type::{IntrinsicTypes, Types};

#[derive(PartialEq, Debug)]
pub struct InterfaceMemberASTNode<'a> {
    name: Option<String>,
    r#type: &'a Types,
}

#[derive(PartialEq, Debug)]
pub struct InterfaceASTNode<'a> {
    name: Option<String>,
    pub members: Vec<&'a InterfaceMemberASTNode<'a>>,
}

impl<'a> InterfaceASTNode<'a> {
    fn parse_member(
        &mut self,
        token_stream: &'a mut TokenStream,
    ) -> Result<Option<InterfaceMemberASTNode>, Box<ParsingError>> {
        let mut member = InterfaceMemberASTNode {
            name: None,
            r#type: &Types::Intrinsic(IntrinsicTypes::InvalidUnknown),
        };

        loop {
            let token_option = token_stream.next();

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
                        token_stream.consume();
                    }
                    TokenType::Symbol(Symbol::Colon) => {
                        if member.name.is_none() {
                            return Err(Box::new(ParsingError::UnexpectedToken(token.clone())));
                        }

                        token_stream.consume();
                        continue;
                    }
                    TokenType::SOI => {
                        token_stream.consume();
                        continue;
                    }
                    TokenType::WhiteSpace(..) => continue,
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

    fn produce(token_stream: &'a mut TokenStream) -> Result<Self, Box<ParsingError>>
    where
        Self: Sized,
    {
        let mut out = Self::new();
        let mut is_open = false;

        loop {
            let token_option = token_stream.next();

            match token_option {
                Some(token) => match &token.token_type {
                    TokenType::Ident(ident) => {
                        if !is_open && out.name.is_none() {
                            out.name = Some(ident.to_string());
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
                    TokenType::WhiteSpace(..) | TokenType::SOI => {
                        token_stream.consume();
                        continue;
                    }
                    _ => return Err(Box::new(ParsingError::UnexpectedToken(token.clone()))),
                },
                None => break,
            }
        }

        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::ASTNodeMember,
        lexer::{Lexer, token_stream::TokenStream},
    };
    use crate::ast::nodes::interface::IntrinsicTypes::InvalidUnknown;

    use super::{InterfaceASTNode, InterfaceMemberASTNode, Types};

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
                r#type: &Types::Intrinsic(InvalidUnknown)
            }
        )
    }
}
