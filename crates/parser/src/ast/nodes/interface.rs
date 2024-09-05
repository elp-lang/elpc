use crate::ast::nodes::r#type::{IntrinsicTypes, Types};
use crate::{
    ast::ASTNodeMember,
    parsing_error::ParsingError,
    tokens::{Keyword, Symbol, Token, TokenType},
};

#[derive(PartialEq, Debug)]
pub struct InterfaceMemberASTNode<'a> {
    name: Option<String>,
    r#type: &'a Types,
}

#[derive(PartialEq, Debug)]
pub struct InterfaceASTNode<'a> {
    name: Option<String>,
    pub members: Vec<InterfaceMemberASTNode<'a>>,
}

fn parse_member(token_stream: &Vec<Token>) -> Result<InterfaceMemberASTNode, Box<ParsingError>> {
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
            TokenType::EOF => {
                break;
            }
            TokenType::WhiteSpace(..) => continue,
            _ => {
                return Err(Box::new(ParsingError::UnexpectedToken(
                    token.clone().to_owned(),
                )))
            }
        }
    }

    Ok(member)
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

    fn produce(token_stream: &'a Vec<Token>) -> Result<Self, Box<ParsingError>>
    where
        Self: Sized,
    {
        let mut out = Self::new();
        let mut is_open = false;

        for token in token_stream.into_iter() {
            match &token.token_type {
                TokenType::Ident(ident) => {
                    if !is_open && out.name.is_none() {
                        out.name = Some(ident.to_string());
                        continue;
                    } else if is_open {
                        match parse_member(&token_stream) {
                            Ok(member) => out.members.push(member),
                            Err(err) => return Err(err),
                        }
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

                    is_open = false;

                    continue;
                }
                TokenType::WhiteSpace(..)
                | TokenType::Keyword(Keyword::Interface)
                | TokenType::SOI => {
                    continue;
                }
                TokenType::EOF => {
                    if is_open {
                        return Err(Box::new(ParsingError::UnexpectedToken(token.clone())));
                    }
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
    use crate::ast::nodes::r#type::IntrinsicTypes;
    use crate::{ast::ASTNodeMember, lexer::Lexer};

    use super::{parse_member, InterfaceASTNode, InterfaceMemberASTNode, Types};

    #[test]
    fn test_interface_member_parsing() {
        let mut lexer = Lexer::new_str("name: String");
        let tokens = lexer.consume_all_tokens().unwrap();

        let member = parse_member(&tokens);

        assert_eq!(
            member.unwrap(),
            InterfaceMemberASTNode {
                name: Some("name".into()),
                r#type: &Types::Intrinsic(IntrinsicTypes::String)
            }
        )
    }

    #[test]
    fn test_interface_parsing() {
        let mut lexer = Lexer::new_str(
            "interface NameInterface {
    name: String
}",
        );
        let tokens = lexer.consume_all_tokens().unwrap();
        let interface = InterfaceASTNode::produce(&tokens);

        assert_eq!(
            interface.unwrap(),
            InterfaceASTNode {
                name: Some("NameInterface".into()),
                members: vec![InterfaceMemberASTNode {
                    name: Some("name".into()),
                    r#type: &Types::Intrinsic(IntrinsicTypes::String)
                }],
            }
        )
    }
}
