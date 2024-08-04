use crate::{
    ast::ASTNodeMember,
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
        token_stream: Vec<Token>,
    ) -> Result<Option<InterfaceMemberASTNode>, Box<ParsingError>> {
        let mut member = InterfaceMemberASTNode {
            name: None,
            r#type: &Types::Intrinsic(IntrinsicTypes::InvalidUnknown),
        };

        for token in token_stream.iter() {
            match &token.token_type {
                TokenType::Ident(name) => {
                    if name.is_empty() {
                        return Err(Box::new(ParsingError::ExpectedToken(Token {
                            token_type: TokenType::Ident("named member".into()),
                            ..Default::default()
                        })));
                    }

                    member.name = Some(name.into());
                }
                TokenType::Symbol(Symbol::Colon) => {
                    if member.name.is_none() {
                        return Err(Box::new(ParsingError::UnexpectedToken(token.clone())));
                    }

                    continue;
                }
                TokenType::SOI => {
                    continue;
                }
                TokenType::WhiteSpace(..) => continue,
                _ => {
                    return Err(Box::new(ParsingError::UnexpectedToken(
                        token.clone().to_owned(),
                    )))
                }
            }
        }

        Ok(Some(member))
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

    fn produce(token_stream: Vec<Token>) -> Result<Self, Box<ParsingError>>
    where
        Self: Sized,
    {
        let mut out = Self::new();
        let mut is_open = false;

        for token in token_stream {
            match &token.token_type {
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
                    continue;
                }
                _ => return Err(Box::new(ParsingError::UnexpectedToken(token.clone()))),
            }
        }

        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::ASTNodeMember, lexer::Lexer};
    use crate::ast::nodes::interface::IntrinsicTypes::InvalidUnknown;

    use super::{InterfaceASTNode, InterfaceMemberASTNode, Types};

    #[test]
    fn test_interface_member_parsing() {
        let mut lexer = Lexer::new_str("name: String");
        let tokens = lexer.consume_all_tokens().unwrap();
        let mut interface_parser = InterfaceASTNode::new();

        let member = interface_parser.parse_member(tokens);

        assert_eq!(
            member.unwrap().unwrap(),
            InterfaceMemberASTNode {
                name: Some("name".into()),
                r#type: &Types::Intrinsic(InvalidUnknown)
            }
        )
    }
}
