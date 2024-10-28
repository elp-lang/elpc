use crate::ast::nodes::r#type::{IntrinsicTypes, Types};
use crate::token_stream::TokenStream;
use crate::{
    parsing_error::ParsingError,
    tokens::{Keyword, Symbol, Token, TokenType},
};
use crate::ast::{ASTNodeProducer, ElpASTContext};

#[derive(PartialEq, Debug)]
pub struct InterfaceMemberASTNode {
    name: Option<String>,
    r#type: Types,
}

#[derive(PartialEq, Debug)]
pub struct InterfaceASTNode {
    name: Option<String>,
    pub members: Vec<InterfaceMemberASTNode>,
}

fn parse_member(
    token_stream: &mut TokenStream,
) -> Result<InterfaceMemberASTNode, Box<ParsingError>> {
    let mut member = InterfaceMemberASTNode {
        name: None,
        r#type: Types::Intrinsic(IntrinsicTypes::InvalidUnknown),
    };

    while let Some(token) = token_stream.token() {
        match &token.token_type {
            TokenType::Ident(ident) => {
                if ident.is_empty() {
                    return Err(Box::new(ParsingError::ExpectedToken(Token {
                        token_type: TokenType::Ident("any".into()),
                        ..Default::default()
                    })));
                }

                if member.name.is_none() {
                    member.name = Some(ident.into());
                } else {
                    member.r#type = Types::User(ident.into());
                }
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
            TokenType::EOF | TokenType::Symbol(Symbol::CloseBlock) => {
                token_stream.consume();
                break;
            }
            TokenType::WhiteSpace(..) => {
                token_stream.consume();
                continue;
            }
            _ => {
                return Err(Box::new(ParsingError::UnexpectedToken(
                    token.clone().to_owned(),
                )))
            }
        }
    }

    Ok(member)
}

impl ASTNodeProducer for InterfaceASTNode {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            name: None,
            members: vec![],
        }
    }

    fn produce(&self, elp_astcontext: &ElpASTContext) -> Result<Self, Box<ParsingError>>
    where
        Self: Sized,
    {
        let mut out = Self::new();
        let mut is_open = false;

        while let Some(token) = elp_astcontext.token_stream.token() {
            match &token.token_type {
                TokenType::Ident(ident) => {
                    if !is_open && out.name.is_none() {
                        out.name = Some(ident.to_string());
                        elp_astcontext.token_stream.consume();
                        continue;
                    } else if is_open {
                        match parse_member(elp_astcontext.token_stream) {
                            Ok(member) => out.members.push(member),
                            Err(err) => return Err(err),
                        };
                        elp_astcontext.token_stream.consume();
                    } else {
                        return Err(Box::new(ParsingError::UnexpectedToken(token.clone())));
                    }
                }
                TokenType::Symbol(Symbol::OpenBlock) => {
                    is_open = true;

                    elp_astcontext.token_stream.consume();
                    continue;
                }
                TokenType::Symbol(Symbol::CloseBlock) => {
                    if !is_open {
                        return Err(Box::new(ParsingError::UnexpectedToken(token.clone())));
                    }

                    is_open = false;

                    elp_astcontext.token_stream.consume();
                    continue;
                }
                TokenType::WhiteSpace(..)
                | TokenType::Keyword(Keyword::Interface)
                | TokenType::SOI => {
                    elp_astcontext.token_stream.consume();
                    continue;
                }
                TokenType::EOF => {
                    if is_open {
                        return Err(Box::new(ParsingError::UnexpectedToken(token.clone())));
                    }
                    elp_astcontext.token_stream.consume();
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
    use super::{parse_member, InterfaceASTNode, InterfaceMemberASTNode, Types};
    use crate::token_stream::TokenStream;
    use crate::{ lexer::Lexer};

    #[test]
    fn test_interface_parsing() {
        let mut lexer = Lexer::new_str(
            "interface NameInterface {
    name: String
}",
        );
        let tokens = lexer.consume_all_tokens().unwrap();
        let mut token_stream = TokenStream::new(tokens);
        let interface = InterfaceASTNode::produce(&mut token_stream);

        assert_eq!(
            interface.unwrap(),
            InterfaceASTNode {
                name: Some("NameInterface".into()),
                members: vec![InterfaceMemberASTNode {
                    name: Some("name".into()),
                    r#type: Types::User("String".into())
                }],
            }
        )
    }

    #[test]
    fn test_interface_member_parsing() {
        let mut lexer = Lexer::new_str("name: String");
        let tokens = lexer.consume_all_tokens().unwrap();
        let mut token_stream = TokenStream::new(tokens);
        let member = parse_member(&mut token_stream);

        assert_eq!(
            member.unwrap(),
            InterfaceMemberASTNode {
                name: Some("name".into()),
                r#type: Types::User("String".into())
            }
        )
    }
}
